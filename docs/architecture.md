# Aphrodite — Architecture

> Status: **re-grounded against `seed_20260513T073417Z.yaml`** (2026-05-13). Decisions below now reflect the seed; tentative-pre-seed defaults that the interview overrode are marked **[revised]**.

## 1. The two surfaces

Aphrodite exposes itself in exactly two ways:

1. **`aphrodite` CLI** — for humans and for sub-shelling agents.
2. **MCP server** (`aphrodite-mcp`) — for any agent runtime (Claude Code, Codex, opencode, Hermes, Gemini, Kiro). Tools exposed: `design`, `compose`, `evaluate`, `evolve`, `tokens`, `route`.

Anything else (Slack/Telegram bot, web dashboard) is downstream packaging, not core.

## 2. Stack decision **[revised — Rust core]**

| Layer | Choice | Rationale |
|---|---|---|
| Core | **Rust** (`crates/aphrodite-core`) | Seed-mandated. One source of truth for DESIGN.md model, validator, taste store, policy engine. Predictable performance for the multi-mode emitter and contrast-validator inner loops. |
| CLI | Rust (`crates/aphrodite-cli`) | Single static binary; first-run setup, `aphrodite design`, `aphrodite setup`. Renders a JSON projection through a thin pretty-print layer (agent-first contract). |
| MCP server | Rust (`crates/aphrodite-mcp`) | Exposes the same operations as JSON tools. This is the **primary surface** under the seed's agent-first decision. |
| TS binding | `@aphrodite/sdk` (napi-rs or wasm) | For TS-side hosts that want to embed without spawning the binary; emits the same JSON contract. |
| Python binding | `aphrodite` (pyo3) | For ML / Blender / Ouroboros-adjacent hosts. |
| Distribution | Homebrew tap + `cargo install` + `npx @aphrodite/cli` + `pipx install aphrodite` | Single Rust binary inside each. |
| Spec format | `DESIGN.md` (Google Labs alpha) + `seed.yaml` (from Ouroboros) | Unchanged. |

**Why Rust over the prior TS-primary plan:** the seed loads the v0.1 with multi-mode DESIGN.md emit + WCAG validator + adaptive taste store + policy engine + MCP server + CLI. A Rust core lets all bindings share one validator and one taste-store implementation. The earlier "TS-primary because UI artifacts are JS" objection still holds for the *emitted artifacts* (HTML/CSS/React/three.js), but those are output strings, not core types — Rust emits them just as well via templating. The earlier argument did not consider that the taste store + DESIGN.md model are the *shared invariants* that benefit most from a single-language home.

## 3. Subsystems **[revised — v0.1 scope tightened]**

```
┌─────────────────────────────────────────────────────────────┐
│             Aphrodite Harness  (Rust core)                  │
├─────────────────────────────────────────────────────────────┤
│  Engine            ── invocation loop, LLM provider routing  │ v0.1
│  Auth Broker       ── API-key only (keychain). OAuth: v0.2.  │ v0.1
│  DESIGN.md Core    ── parse, validate, diff, emit Tailwind   │ v0.1
│                       multi-mode resolver (light/dark/brand) │
│  Generator         ── prompt + skill + DESIGN.md → hero HTML │ v0.1
│  Skills            ── markdown packs ("editorial", …)        │ v0.1 (≥1)
│  Policy / Deny     ── fs / shell / net deny-list             │ v0.1
│  Taste Store       ── global ⊕ project; implicit signals     │ v0.1
│  Repo Writer       ── git-add / commit / PR; --no-write flag │ v0.1
│  Validator         ── DESIGN.md schema + WCAG-AA contrast    │ v0.1
│  Event store       ── SQLite; replayable lineage             │ v0.1
│  ──────────────────────────────────────────────────────────  │
│  Aesthetic Jury    ── LLM/heuristic explicit scoring         │ v0.2
│  OAuth flows       ── OpenAI/Kimi/Gemini browser callback    │ v0.2
│  Figma adapter     ── round-trip with Figma Tokens           │ v0.2
│  Motion (Hyper-)   ── HyperFrames composition + render       │ v0.2
│  3D adapter        ── three.js scene + Blender sidecar       │ v0.3
│  Coordinator       ── color/layout/motion sub-experts        │ v0.3
└─────────────────────────────────────────────────────────────┘
```

Stitch adapter is **deleted from the architecture entirely**, not deferred (see ADR-0002 revision).

## 4. Auth broker **[revised — v0.1 = API-key only]**

See ADR-0003 (revised). v0.1 ships API-key flow only; OS keychain storage from day 1. Subscription-reuse detection is a READ-ONLY hint, never a silent slurp.

```
aphrodite setup
  → list configured providers from keyring
  → for each unconfigured provider: interactive prompt OR `--from-keyring <entry>` OR APHRODITE_PROVIDER_KEY_* env
  → write only to OS keychain
  → surface a hint if opencode/claude-code/codex-cli configs are present (read-only)
```

Provider matrix (v0.1, May 2026):

| Provider | v0.1 (API key) | v0.2 (OAuth) | Notes |
|---|---|---|---|
| OpenAI (GPT) | ✅ | ✅ ChatGPT browser flow | Pro/Plus quota reusable via OAuth in v0.2 |
| Moonshot / Kimi (z.ai) | ✅ | ✅ | OAuth in v0.2 |
| Anthropic (Claude) | ✅ | ❌ (no third-party OAuth exists) | Permanent API-key path |
| Google (Gemini) | ✅ | ✅ | OAuth in v0.2 |
| Codex CLI / opencode reuse | hint only | hint only | Never auto-import; always confirm |

## 5. The DESIGN.md core

Aphrodite ships:
- **`aphrodite design validate`** — spec compliance, broken token refs, WCAG AA contrast.
- **`aphrodite design diff v1 v2`** — semantic diff between two DESIGN.md versions.
- **`aphrodite design emit --target tailwind|css-vars|figma-tokens|swift|compose`** — single source → all platforms.
- **`aphrodite design from-stitch <url>`** — pull a DESIGN.md from a Stitch project.
- **`aphrodite design to-figma`** — push tokens to a Figma file via plugin.

DESIGN.md is **the** contract. Every other tool either reads from or writes to it. No private intermediate formats.

## 6. Skills (the "looks")

Markdown packs under `~/.aphrodite/skills/` or per-project `aphrodite-skills/`. Each pack is a directory:

```
brutalist/
├── SKILL.md           # frontmatter: triggers, tags, license
├── DESIGN.md          # baseline tokens for this look
├── prompts/           # prompt fragments for design/compose phases
├── references/        # image references (license-tracked)
└── components/        # optional, pre-baked component snippets
```

Skills are the community contribution surface. Aphrodite ships ~6 first-party skills; everything else is community.

## 7. Permissions — the "full trust" stance

Default: Aphrodite operates with full filesystem + network access *within the working directory and the user's auth-scoped APIs*. No per-tool-call confirmation. This is the **authentic agency** anchor.

Opt-in *deny* rules (not allow-list):
```yaml
# .aphrodite/policy.yaml
deny:
  fs:
    - "../**"          # never escape project root
    - "~/.ssh/**"
  shell:
    - "rm -rf /"
    - "git push --force-with-lease origin main"
  net:
    - "*.suspicious.tld"
```

## 8. Resolved-by-seed questions (was: open)
- ~~Single-binary CLI vs CLI+daemon~~ → **single Rust binary** for v0.1; daemon only when 3D/render servers land (v0.3).
- ~~Should `aphrodite` invoke Ouroboros internally?~~ → **No**, `seed.yaml` is the contract; Ouroboros is a separate CLI users run beforehand. (ADR-0001 unchanged.)
- ~~Visual-regression storage~~ → **Out of v0.1.** Visual-regression is part of the v0.2 explicit aesthetic jury.
- ~~Aesthetic jury aggressiveness~~ → **Advisory only** at v0.1 (implicit signals captured to taste store); becomes a configurable gate at v0.2+.
- ~~Multi-tenancy~~ → **Global default ⊕ project override** for taste; `~/.aphrodite/` holds user-level defaults, `<repo>/.aphrodite/` overrides per project. Seed-mandated.

## 9. Still-open questions (post-seed)
- Rust binding distribution: napi-rs vs wasm for TS; tradeoffs on bundle size vs platform fanout.
- Concrete deny-list defaults: what does `policy.yaml` ship as on day 1 to make "full write" safe? (Project-root jail and `~/.ssh` block are obvious; what else?)
- Hero HTML stack: plain HTML + Tailwind v4 only, or hand a small Astro/Eleventy skeleton? Seed says "no external network calls at render time" — favors plain HTML + inlined Tailwind.
- Skill format: copy Anthropic's `SKILL.md` shape verbatim or extend? Probably copy + extend in `metadata`.
