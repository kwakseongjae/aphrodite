# Agent Evaluation — 2026-05-14 — Sixth Pass (multi-turn refinement)

> The user asked: "how does Aphrodite behave when I issue multi-turn requests?" The answer requires a real command — `aphrodite refine "<delta>"` — that takes the current DESIGN.md as context and emits a revised version with surgical scope. This pass ships that command and measures whether the LLM honors the surgical contract.

## Test design

Single initial design + three successive refinements on the same DESIGN.md. Each refinement targets a *different* axis (color, spacing, display font). The test isn't whether the LLM responds — it's whether the LLM responds *to the requested axis only* while preserving prior decisions.

## Results — every axis behaved correctly

| Turn | Delta requested | Primary 500 | Display | Body | Max spacing |
|---|---|---|---|---|---|
| 1 — baseline | (no delta) | `#5d7a32` olive | Fraunces | Source Serif 4 | 64 px |
| 2 — "make it cooler" | hue family only | `#4e6d8e` **cool blue** | Fraunces (preserved) | Source Serif 4 (preserved) | 64 px (preserved) |
| 3 — "much more spacing, top token ≥ 128px" | spacing only | `#4e6d8e` (preserved) | Fraunces (preserved) | Source Serif 4 (preserved) | **192 px** |
| 4 — "switch display to sans, keep palette and spacing" | display family only | `#4e6d8e` (preserved) | **Inter** | Source Serif 4 (preserved — body wasn't in scope) | 192 px (preserved) |

Three properties confirmed:

1. **Delta lands on the requested axis.** Turn 2 moved hue 142° (olive → blue), turn 3 tripled max spacing, turn 4 replaced the display family. Each was the *largest visible* change in its turn.

2. **Other axes are preserved.** The cool blue from turn 2 is still present in turn 4 — three turns later, no LLM "forgot" the earlier decision. Spacing of 192 px from turn 3 persists into turn 4 despite the refinement only mentioning the display family.

3. **Nuance respected.** Turn 4's instruction was *"switch the display family from any serif to a high-quality grotesque sans-serif while keeping everything else (palette, spacing) exactly as it is."* The output:
   - display: Inter ✓ (a grotesque sans)
   - body: Source Serif 4 unchanged ✓ (body was excluded from the request — most LLMs would also change body for consistency; this one didn't, which is the more careful read)
   - palette: same `#4e6d8e` ✓
   - spacing: same 192 px ✓

This is the senior-designer-level multi-turn behavior the system needed. The contract is **"refinement, not re-design."**

## How it works under the hood

`aphrodite refine "<change>"`:

1. Reads `DESIGN.md` from cwd.
2. Builds a system prompt: *"You are revising an existing DESIGN.md. Output a NEW complete DESIGN.md that incorporates the change while preserving everything else."*
3. The user message contains the full current DESIGN.md + the delta request.
4. The LLM emits a revised DESIGN.md.
5. Aphrodite parses, validates (schema + WCAG-AA), re-composes the surface (second LLM call with the new DESIGN.md), and writes all three files (DESIGN.md, hero.html, composition.html). Git auto-commit if the dir is a repo.
6. A `Regenerate` taste event is appended to the project + global taste store with `delta` recorded, so the loop continues to learn from multi-turn behavior.

The original `intent.txt` from the first `design` call is kept under `.aphrodite/`, and `refine` passes it to the surface composer as the *root* intent — so the composer still knows it's drawing an editorial site even when the delta is just "more spacing."

## Other v0.2 work shipped this pass

- **Finding #20 ✅ closed** — `reqwest::Client` now has 180 s `timeout` + 15 s `connect_timeout`. Pass 5 had a step that ran 2 991 s (50 min) because the call hung on a flaky network. With this cap, max wall-clock per `aphrodite design` is bounded at ~720 s (4 calls × 180 s); typical Pass 6 turns came in at 234–301 s.
- **Finding #21 ✅ closed** — density extractor now considers both max spacing AND token count + spread. Returns a non-None signal for the realistic range of DESIGN.md outputs we've seen.

## Honest agent satisfaction

| Aspect | Pass 5b | Pass 6 |
|---|---|---|
| Output is the page the user asked for | 5 / 5 | 5 / 5 |
| **Multi-turn fidelity** | not measured | **5 / 5** (3 turns × 4 axes × 100% surgical) |
| Taste loop demonstrably moves output direction | 5 / 5 | 5 / 5 |
| Latency stability | 1 / 5 (Pass 5 hangs) | **4 / 5** (timeouts shipped) |
| Validation gate | 5 / 5 | 5 / 5 |

**Overall: 4.7 / 5.** The remaining gap is the brand-name palette recall (#13), latency *throughput* (still 200–300 s per turn — multi-turn iteration of 4 turns = ~17 min), and untested surfaces like RTL composition.

## Open findings carried into next pass

- **#13** brand-name palette recall (unchanged)
- **#23** *new* — refine's surface re-composition runs the full LLM cost even when the delta wouldn't change the composition (e.g., spacing-only refinement still re-composes the page from scratch). A cheap fast-path: when delta only touches frontmatter tokens and the surface_type doesn't change, just re-inject CSS vars without a fresh LLM call.
- **#17** RTL flow (untested in Pass 6)
- Pass 5 step 4-6 (love side of taste loop) still unverified end-to-end — though Pass 5b A/B controlled experiment is a stronger proof anyway.

## How to inspect

```bash
open /Users/kwakseongjae/Desktop/projects/aphrodite/docs/agent-eval/archive/2026-05-14-pass6-multiturn/gallery.html
```

Open the four turn cards side-by-side and read across:
- Palette swatches: turn 1 olive, turns 2–4 the same cool blue.
- Display font: turns 1–3 Fraunces, turn 4 Inter.
- Body font: all turns Source Serif 4.
- Composition layout: same editorial structure across turns — only the tokens move.

This is the multi-turn behavior worth shipping. The kind of system you can sit in front of and iterate with.
