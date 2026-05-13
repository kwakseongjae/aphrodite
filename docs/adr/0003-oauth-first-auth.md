# ADR 0003 — Auth: API-key-only at v0.1, OAuth-first roadmap from v0.2

Date: 2026-05-13 (revised 2026-05-13 post-seed)
Status: Accepted (revised)

## Context
First-run UX is dominated by auth. The user (and most likely users) already pays for ChatGPT / Kimi / Gemini subscriptions and does not want a second billing relationship. Anthropic does not offer OAuth for third-party CLI use, so Claude is API-key-only.

Original ADR (2026-05-13 morning) targeted OAuth-first across the board from v0.1. The seed interview (afternoon) reduced v0.1 scope: with Rust core + multi-mode DESIGN.md + adaptive taste loop + full write authority all already on the v0.1 plate, the OAuth flow implementation slid to v0.2 to keep the v0.1 timeline (~4 months) realistic.

## Decision

### v0.1 (now)
- **API-key-only** for every supported provider, including OpenAI / Moonshot-Kimi / Gemini / Anthropic.
- **Storage:** OS keychain (macOS Keychain / libsecret / Windows Credential Manager) via a Rust keyring crate. Never plaintext files. Never `.env`.
- **First-run UX:** `aphrodite setup` walks the user through provider selection and key entry. Target: < 90s to a working `aphrodite design` command for a user who already has an Anthropic API key (seed acceptance criterion).
- **Subscription-reuse detection:** READ-ONLY hint only. When `opencode`, `claude-code`, or `codex-cli` configs are present, surface them as "we noticed you have X — paste the key from there if you want" copy. Never silently slurp keys.

### v0.2 (next)
- **OAuth-first** for OpenAI, Moonshot-Kimi (z.ai), Google Gemini. Browser-flow via `aphrodite setup --oauth`.
- API-key remains as fallback for all providers and as the only path for Anthropic.
- Subscription-reuse stays at "hint, not contract."

### v1.0+
- Hardware-key / passkey auth (WebAuthn) for any future dashboard surface.

## Consequences
- **+** v0.1 ships sooner; OAuth callback servers, dynamic client registration, and provider-specific token-refresh quirks all deferred.
- **+** API-key-only is the *worst* UX, so v0.2's OAuth release is an obvious quality jump and a natural marketing beat.
- **+** Keychain-only storage from day 1 prevents the worst class of mistake (committed `.env`).
- **−** v0.1 users face manual key paste from each provider's dashboard. Mitigate with a `--from-clipboard` flow and clear "where to find this" doc links.
- **−** Anthropic-only users have no upgrade path even in v0.2 (no Anthropic OAuth exists). Accept; surface clearly in docs.

## Required behavior (v0.1)
- `aphrodite setup` must accept keys via interactive prompt, env-var read (`APHRODITE_PROVIDER_KEY_<PROVIDER>`), and `--from-keyring <existing-entry>` (reuse a key already in keychain).
- All key writes go through the keyring abstraction; no other code path touches credentials.
- `aphrodite auth status` lists configured providers without revealing key material.
- Deny-list policy applies to net: `aphrodite-mcp` server must refuse to call any provider not explicitly listed in `~/.aphrodite/auth/providers.yaml`.
