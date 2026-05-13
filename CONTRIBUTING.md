# Contributing to Aphrodite

Aphrodite is a UI-generation harness designed to be called by humans **and** by AI agents — both are first-class contributors here. The notes below set the rules of the road for both.

## What Aphrodite is, briefly
A standalone (non-plugin) harness with two surfaces: the `aphrodite` CLI and the `aphrodite-mcp` MCP server. Both produce a `DESIGN.md` (Google Labs alpha) + a hero HTML page from a user intent, and learn from implicit aesthetic feedback over time. The full direction lives in [`docs/vision.md`](docs/vision.md); the immutable v0.1 spec lives in [`.ouroboros/seeds/seed_20260513T073417Z.yaml`](.ouroboros/seeds/seed_20260513T073417Z.yaml).

## Ground rules
1. **The seed is immutable.** It is the contract; changes to v0.1 scope require a new seed + a written ADR. Run `ooo interview` and `ooo seed` (Ouroboros, ADR-0001) if you genuinely need to renegotiate.
2. **DESIGN.md is the design contract.** No private intermediate formats. Every emitter or importer must go through `aphrodite-core::DesignDocument`.
3. **No external network calls at hero render time.** Hero HTML is always self-contained.
4. **Secrets live in the OS keychain.** Never in `.env`, never in committed files. The only code path that touches credentials is `aphrodite-keyring`.
5. **Match the v0.1 evaluation principles.** PRs that drop a `design_md_validity` or `multi_mode_completeness` score get rejected.

## Workflow
- Fork → branch → PR against `main`.
- A PR must compile (`cargo build --workspace`) and pass tests (`cargo test --workspace`). Both run in CI.
- Every PR that changes generated output must include a screenshot or short clip of the new hero page on at least the light + dark variants.

## Agent contributors
You may contribute through the same flow. Use `aphrodite-mcp` to propose changes; sign the commit trailer with your model + version (`Co-Authored-By: <model> <ts>`). The Restate gate from Ouroboros applies to PR descriptions — open with a one-line statement of what changes.

## ADRs
Architectural changes land via ADRs in `docs/adr/`. Numbering is monotonic. Format: Context → Decision → Consequences. Status is one of `Proposed`, `Accepted`, `Superseded`.

## Skills
First-party skills live under `skills/`. The format mirrors Anthropic's `SKILL.md` so any harness can adopt our packs and vice versa.

## Code of conduct
Be respectful. Critique designs, not designers.
