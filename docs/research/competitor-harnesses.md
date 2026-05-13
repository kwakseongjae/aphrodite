# Competitor / Reference Harnesses — Field Notes

Goal: understand how existing open harnesses deliver "full authority to an agent" *without* being a plugin of one host CLI. We learn from their architecture; we do not copy scope (Aphrodite is UI-focused, they are general).

## Tier 1 — Direct architectural references

### 1. Ouroboros — `Q00/ouroboros` (Python, MIT)
- **What it is:** Local-first **Agent OS**. Workflow contract: `Interview → Seed → Execute → Evaluate → Evolve`. Refuses to start work until ambiguity ≤ 0.2 (Socratic). Outputs immutable `seed.yaml`.
- **Auth model:** Re-uses host agent's auth (Claude Code, Codex CLI, Hermes, Gemini, Copilot CLI, OpenCode, Kiro). Registers itself as an MCP server (`evaluate`, `evolve`, `unstuck`, `ralph`).
- **Take for Aphrodite:** We will *use* Ouroboros as the upstream spec-crystallizer rather than re-invent it. Aphrodite consumes a finalized `seed.yaml` and emits a `DESIGN.md` + UI.

### 2. OpenHarness / Ohmo — `HKUDS/OpenHarness` (Python, Apache-2.0)
- **What it is:** Lightweight agent infra — engine loop, 43+ tools, skills-as-markdown, permissions, memory, multi-agent coordinator. Ohmo is a packaged personal-agent on top, accessible via Feishu/Slack/Telegram/Discord.
- **Crucial design move:** "Runs on your existing Claude Code or Codex subscription — no extra API key needed." This is the **subscription-reuse pattern** Aphrodite must mimic.
- **Take:** Steal the 10-subsystem decomposition (Engine, Tools, Skills, Permissions, Memory, Coordinator…) as Aphrodite's internal module shape, but specialize each for UI.

### 3. opencode — `sst/opencode` (TS, MIT)
- **What it is:** Terminal-native, model-agnostic coding agent. Supports OAuth flows for providers + MCP OAuth (Dynamic Client Registration on 401). Has DigitalOcean-style "sign in and we auto-create the access key" UX.
- **Take:** Adopt their **OAuth-first / API-key-fallback** strategy verbatim. Their provider config schema is a strong starting point.

### 4. oh-my-openagent (omo) — agent harness collection
- **What it is:** Skills/agents marketplace pattern atop OpenCode.
- **Take:** Naming + community/skills-as-markdown distribution model. Aphrodite plugins ("looks", "motion-kits", "3D-kits") should follow this shape.

### 5. Hermes (Nous Hermes Agent CLI)
- **What it is:** Open agentic CLI; integrates HyperFrames natively for video. Demonstrates that an agent harness can *embed UI tooling as first-class skills* rather than as bolt-ons.
- **Take:** Validates the "HTML-native skill" thesis — agents are great at HTML/CSS/JS, so Aphrodite's renderable surface should be HTML-first (Stitch and HyperFrames both already align).

## Tier 2 — Patterns to borrow, not whole architectures

- **Aider / Cline / Kilo / Continue** — diff-first edit loops; useful for the "apply UI changes back to a real repo" step.
- **GitHub Spec-Kit** — alternative spec-first format; aligns with Ouroboros philosophy. Cross-reference both.
- **Claude Code skills** — markdown-as-skill is now the de-facto standard. Aphrodite skills ship the same shape so any harness can adopt them.

## Tier 3 — UI / asset toolchain (orchestrated, not competed with)

| Tool | Role in Aphrodite | License / Access | Risk |
|---|---|---|---|
| **Google Stitch** | First-draft UI from prompt; emits `DESIGN.md` natively | Hosted, Google account | Vendor lock; no public API yet — likely browser-automation bridge |
| **Figma + plugins** | Refinement, design-system curation | OAuth (Figma API) | Stable, well-documented |
| **Blender MCP** | 3D scenes, materials, render | Open, MCP server exists | Heavyweight; gate behind opt-in |
| **three.js** | Web 3D in final output | MIT | None |
| **HyperFrames** | Motion/video; HTML-native, agent-first | Apache-2.0 | None — strong fit |
| **DESIGN.md (Google Labs)** | Cross-tool design contract | Open spec | None — adopt as-is |

## Design-contract layer: DESIGN.md

Google open-sourced `DESIGN.md` from Stitch. Spec requires `name` in YAML frontmatter; ordered sections (Overview → Colors → Typography → Layout → Elevation & Depth → Shapes → Components → Do's & Don'ts); `{path.to.token}` references; at minimum a `primary` color palette. **Aphrodite ships a validator, a Tailwind v4 theme emitter, and a Stitch ↔ Figma ↔ code round-tripper around this format.**

## Open questions (deferred to Ouroboros interview)

1. **Language:** Python (matches Ouroboros, Blender, ML ecosystem) vs TS (matches opencode, web/Figma/three.js/HyperFrames). Likely **TS-primary with Python sidecars** — defer.
2. **Distribution:** Single `aphrodite` CLI (`npx aphrodite` + `pipx`) vs MCP-server-only. Probably **both** — CLI for humans, MCP for agents.
3. **Memory:** Per-project `.aphrodite/` dir (git-ignored auth, committed `DESIGN.md` + decisions) vs global home dir.
4. **Multi-agent:** Does Aphrodite spawn sub-agents for "color expert / layout expert / motion expert" (OpenHarness Coordinator pattern), or stay single-agent and route to tools?
5. **Stitch integration:** Browser-automation bridge (Playwright) vs wait-for-official-API. Browser-bridge unblocks now but is fragile.
