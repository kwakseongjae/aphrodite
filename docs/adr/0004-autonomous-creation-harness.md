# ADR 0004 — Aphrodite is an autonomous creation harness, not a one-shot generator

Date: 2026-05-14
Status: Accepted

## Context

Through v0.1 → v0.2 Aphrodite has been positioned as a *generator*: one CLI call, one DESIGN.md + composition.html out. Multi-turn refinement was added in Pass 6 (commit `bc11610`) and verified reactively in Pass 7 (commit `99a574c`), but the *refinement loop itself sits outside the harness* — it's the caller (a human, Claude Code, Codex) that observes each turn's output and authors the next delta.

Pass 7 made the cost of this externalization visible:

- Each reactive turn requires the caller to spend cognitive load reading DESIGN.md prose + composition.html structure + CSS tokens + WCAG report, then forming a judgment about "the single most unsatisfying axis," then writing a precise delta.
- This work is reusable knowledge — *how do you decide what to refine next?* is itself a skill that should accumulate.
- A casual caller (`claude_code: aphrodite design "<intent>"`) gets a one-shot output. The "5/5 result via 4 reactive turns" path is invisible to them.
- Research, reference gathering, asset creation, and harmonization (cross-file consistency, font-import injection, prose-vs-tokens audit) are all *also* externalized: the caller is expected to know they need to do these.

The Hermes Agent codebase (`NousResearch/hermes-agent`) demonstrates a working alternative architecture: the *agent* internalizes the loop, the *caller* hands over the task and receives the finished artifact. Hermes accomplishes this via five pillars: memory (declarative facts, decay-able), skills (procedural workflows, patchable), curator (skill lifecycle management), insights (usage telemetry feeding curator), and kanban (orchestrator pattern for multi-phase work).

## Decision

Aphrodite's product contract changes from *"one-shot generator"* to *"autonomous creation harness."*

A single invocation — whether from a human terminal, Claude Code, Codex, or `aphrodite-mcp` — triggers an internal orchestrator that runs the following phases until a satisfaction signal fires or a budget is exhausted:

1. **research** — gather references for the intent (web search, taste-store lookup)
2. **taste application** — filter references by accumulated user preferences
3. **design** — generate DESIGN.md + composition.html (today's `aphrodite design`)
4. **self-critic refinement** — internal multi-turn loop; harness reads its own output, identifies weakest axis, issues refine delta, repeats until threshold or N exhausted (this is Pass 7 *internalized*)
5. **asset creation** — generate or source imagery/icons when intent implies them
6. **asset management** — dedupe, name, organize under `<project>/.aphrodite/assets/`
7. **harmonization** — final pass for cross-file consistency, font import injection (Finding #24), WCAG re-check, prose-vs-tokens audit
8. **skillification** — if the trajectory was non-trivial (≥5 internal tool calls or ≥3 refine turns), propose a reusable skill capturing the workflow

The caller's contract becomes:

```
aphrodite create "<intent>"
  → returns: a finished DESIGN.md + composition.html + assets/ tree, with
    optional auto-proposed skill on disk for next time
```

`aphrodite design` (one-shot, no refinement) remains available as a power-user verb and as an internal building block used by phase 3 above. `aphrodite refine` likewise remains available for manual control.

## Architectural choices borrowed from Hermes (source-level, not surface)

- **Memory ≠ Skills.** Memory holds *declarative facts that may decay* (`user prefers warm hues — accumulated over 17 runs`). Skills hold *procedural workflows that get patched* (`for Korean editorial portfolios, prefer Source Serif 4 over EB Garamond and validate ≥3 refine turns before declaring satisfaction`). Aphrodite's existing `TastePreferences` already plays the memory role; this ADR adds the skill substrate.
- **Skills are on-disk markdown + a usage JSON.** Each skill is `~/.aphrodite/skills/<slug>/SKILL.md` with YAML frontmatter (`name`, `description`, `version`, `tags`, `related_skills`, `agent_created`) and a procedural body. A single `~/.aphrodite/skills/usage.json` tracks per-skill `view_count`, `use_count`, `patch_count`, `latest_activity_at`, `state` (active / stale / archived), `pinned`. Mirrors `tools/skill_usage.py` in Hermes.
- **Auto-creation trigger.** After a successful `aphrodite create` whose trajectory exceeded ≥5 internal tool calls or ≥3 refine turns, the orchestrator proposes a skill. The skill *captures the workflow*, not the artifact.
- **Curator deferred.** A `aphrodite curator` verb that handles stale/archive transitions of agent-created skills is on the roadmap. v0.3 ships skills without curator; users can hand-archive.
- **Orchestrator owns the loop.** Workers (research, design, refine, etc.) do their one job and hand off structured `{summary, metadata}`. They never scope-creep. Follow-up work that appears mid-task is *created as a new sub-task*, not silently performed.

## Non-goals (carried from v0.1)

- Aphrodite does not land code; artifact + commit is the contract.
- Aphrodite does not own a UI dashboard; the surfaces remain `aphrodite` CLI + `aphrodite-mcp`.
- The skill system is **not a plugin system for arbitrary code**. Skills are *prompts and workflows* — they instruct the LLM, they do not import Rust modules. (Future skills with custom code paths can be considered post-v1.0.)

## Consequences

- **+** Casual callers (`claude_code: aphrodite create "<intent>"`) get the same 5/5 result Pass 7 demanded 4 reactive turns to produce. The cost is internal; the surface stays simple.
- **+** Skills accumulate cross-task value. After 10 furniture-portfolio sessions, a `furniture-portfolio-seoul.md` skill encodes what worked — next call skips re-discovery.
- **+** Research, reference gathering, asset creation become *parts of the harness*, not externalized expectations of the caller. Matches how Hermes positions itself.
- **+** Same orchestrator scaffolding supports future surfaces (Blender MCP for 3D, HyperFrames for motion) — they become additional worker profiles, not new top-level commands.
- **−** v0.3 scope grows. The 7-phase orchestrator plus skill substrate plus harmonizer is a non-trivial slice. Refinement loop internalization alone is multi-week work because the self-critic logic must be robust enough not to *over-refine* (the kind of trap where each turn introduces a new "the most unsatisfying axis" forever).
- **−** Internal turns are spent compute. A 5/5 result that took the user 1 manual call now takes the harness ~7-12 LLM calls. Cost transparency required.
- **−** Skill bloat risk. Without curator, agent-created skills accumulate indefinitely. v0.3 ships with a hard cap (e.g., 100 agent-created skills) until curator lands.

## Required behavior (v0.3 entry)

- `aphrodite create "<intent>"` must exist as a verb and orchestrate at minimum phases 3 (design) + 4 (self-critic refinement, ≥3 internal turns) + 8 (skillification proposal). Phases 1, 2, 5, 6, 7 may be stubs in v0.3.0 and filled in across v0.3.x.
- `aphrodite skill list` / `aphrodite skill show <slug>` / `aphrodite skill archive <slug>` must work.
- `aphrodite skill propose <intent-summary>` must allow manual skill creation for users who want to seed their library.
- All skills carry `agent_created: true | false` in frontmatter. Curator (future) operates only on `true`.
- Skill lookup at orchestration time: orchestrator reads `~/.aphrodite/skills/usage.json`, ranks by `tags` overlap with the incoming intent, loads the top-K skills' bodies into the system prompt as scaffolds. No exact-match required — partial overlap is the signal.
- Cost transparency: every `aphrodite create` invocation returns the total LLM call count and elapsed wall-clock alongside the artifact paths.

## Validation criteria (Pass 8+)

This ADR is in force from v0.3 onward. Empirical validation:

- Pass 8: same intent run via `aphrodite create` should produce at least one of the four outputs Pass 7 produced reactively — without any external delta authoring.
- Pass 9: same intent run twice with `aphrodite create` should produce *different but coherent* outputs, with the second invocation showing measurable skill reuse (skill loaded into system prompt, taste preferences applied).
- Pass 10: introduce one deliberately-mistaken skill into `~/.aphrodite/skills/`. Orchestrator's harmonizer phase should either patch the skill (preferred) or down-rank it.

## References

- Pass 6 testimony: `docs/agent-eval/2026-05-14-sixth-agent-pass.md`
- Pass 7 testimony: `docs/agent-eval/2026-05-14-seventh-agent-pass.md` — establishes the case for internalization
- Hermes Agent source: `https://github.com/NousResearch/hermes-agent` — patterns borrowed from `agent/memory_manager.py`, `agent/curator.py`, `tools/skill_usage.py`, `agent/prompt_builder.py:SKILLS_GUIDANCE`
- ADR 0002 (DESIGN.md as design contract) — unchanged, the artifact contract still holds
- ADR 0003 (auth) — unchanged
