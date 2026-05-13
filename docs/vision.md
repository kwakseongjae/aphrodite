# Aphrodite — Vision

## The pitch in one sentence
A standalone, open-source UI harness that any AI (or human) can summon to produce *undeniably beautiful* interfaces — by orchestrating Stitch, Figma, Blender, three.js, HyperFrames, and friends under a single `DESIGN.md` contract.

## Why this exists
Today, "AI-generated UI" splits into two unsatisfying camps:
- **One-shot tools** (Stitch, v0, Lovable) — gorgeous first drafts, but isolated; no continuity, no design-system memory, no way for *another* agent to pick up the work.
- **Generalist coding agents** (Claude Code, Codex, OpenCode) — full agency, but UI is an afterthought; outputs are functional, not beautiful.

Aphrodite is the missing middle: **a harness that exists only to make UI work, but does it with full agency.** It is invoked, not embedded. A user types `aphrodite` in their terminal, or another agent calls it over MCP — either way, Aphrodite gets the keys.

## What "authentic" means here
Coined by the user: every requested operation runs end-to-end with no permission-prompt theatre, no sandboxed sub-scope. If the caller (human or AI) says "rebuild the marketing site's hero with a parallax three.js scene," Aphrodite plans, designs (`DESIGN.md` update), generates, renders, validates, and lands the change. Trust is granted up-front by the caller; Aphrodite's job is to deserve it through quality, not gate-keeping.

## The five anchors (from initial briefing)
1. **Open-source by default** — every artifact is either git-tracked (designs, decisions, code) or git-ignored (secrets, caches). Per-file policy is decided when the file is born, not retrofitted.
2. **Ouroboros upstream** — vague user ideas get crystallized through Ouroboros's Socratic interview *before* Aphrodite executes. Aphrodite consumes `seed.yaml`; it does not re-implement the interview.
3. **Learn from existing harnesses, do not become one** — borrow architecture (OpenHarness 10-subsystem shape, opencode OAuth flow, omo skill marketplace), reject scope creep into general coding.
4. **OAuth-first multi-provider auth** — Kimi/z.ai, GPT/ChatGPT, Gemini via OAuth; Claude via API key. Reuse the user's existing subscriptions; never demand new billing.
5. **DESIGN.md is the spine** — every project Aphrodite touches gets a Google-Labs-compatible `DESIGN.md`. It is the contract Stitch, Figma, code, and 3D all agree on.

## The five-loop lifecycle (mirrors Ouroboros, specialized for UI)

| Phase | Input | Output | Aphrodite's specialization |
|---|---|---|---|
| **Interview** | User intent | `seed.yaml` (clarity ≥ 0.8) | Delegated to Ouroboros |
| **Design** | `seed.yaml` | `DESIGN.md` v1 | Routes to Stitch for first-draft, lifts tokens |
| **Compose** | `DESIGN.md` | Concrete UI artifacts (HTML/React, three.js scene, HyperFrames composition) | Tool routing + multi-tool merge |
| **Evaluate** | Artifacts | Pass/fail + critique | DESIGN.md validator + a11y + visual-regression + LLM aesthetic jury |
| **Evolve** | Critique | New `DESIGN.md` v2, regen | Tight loop until aesthetic + a11y gates pass |

## What Aphrodite is *not*
- Not a Claude Code plugin or skill (though Claude Code can invoke it).
- Not a hosted service.
- Not a general coding agent.
- Not opinionated about your framework — emits React, Vue, Svelte, plain HTML, or Figma file, whichever the project wants.
