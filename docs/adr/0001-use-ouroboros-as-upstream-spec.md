# ADR 0001 — Use Ouroboros as the upstream spec crystallizer

Date: 2026-05-13
Status: Proposed (pre-interview)

## Context
Aphrodite needs a way to convert vague user intent ("make my landing page gorgeous") into an immutable, machine-readable spec before any expensive UI generation happens. We could:
- (a) Build our own Socratic interview engine.
- (b) Use Ouroboros (`Q00/ouroboros`) and consume its `seed.yaml`.
- (c) Use GitHub Spec-Kit.

## Decision
Adopt (b). Ouroboros is already open source, already battle-tested, already integrates with Claude Code / Codex / Hermes / opencode / Gemini / Kiro, and already produces `seed.yaml` with an ambiguity threshold gate (≤ 0.2). Building (a) would duplicate this work and split the ecosystem.

## Consequences
- **+** Zero spec-engine code in Aphrodite. Faster to first vertical slice.
- **+** Users who already know Ouroboros transfer instantly.
- **+** Aphrodite gets `evaluate`/`evolve`/`ralph` loops "for free" via Ouroboros MCP.
- **−** Hard dependency on Ouroboros's API stability (mitigate: pin version, contribute upstream).
- **−** Users new to *both* tools face a two-CLI ramp. Mitigate with `aphrodite init` that internally runs `ooo setup && ooo interview`.

## Open
Should Aphrodite *vendor* a minimal interview fallback for offline / Ouroboros-down cases? Defer until we see real failure modes.
