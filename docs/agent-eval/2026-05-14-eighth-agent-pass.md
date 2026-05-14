# Agent Evaluation — 2026-05-14 — Eighth Pass (autonomous `aphrodite create`)

> Pass 7 verified that **reactive** multi-turn produces 5/5 results when an external agent reads each turn's output and authors the next delta. ADR 0004 reframes Aphrodite as an autonomous creation harness — the loop moves *inside*. Pass 8 is the first dogfood of `aphrodite create`, the verb that internalises Pass 7's reactive logic via the new `critic` module.

## Test design

Same intent Pass 7 used, same fresh-prefs setup:

```bash
mkdir -p /tmp/pass8-create && cd /tmp/pass8-create
git init && rm -f ~/.aphrodite/taste/preferences.toml
aphrodite create "a portfolio site for an independent furniture maker working in solid walnut, based in Seoul" --max-turns 3
```

Internal loop: **design → (critique → refine)\* → final**, capped at 3 turns, stop on satisfaction ≥ 0.85.

Pass 7 took **4 turns and 1036 s** with an external human reading output and authoring deltas. Pass 8 measures whether the internalised loop reaches a comparable result with **zero external input**.

## What happened — two runs, two failure modes, both informative

### Run A — critic parser bug (475 s, 1 useful turn, then crash)

- **Phase 3 (design)** in 247 s. Generator emitted Space Grotesk + Sora. Tech-forward geometric sans. Wrong register for a walnut craftsperson.
- **Turn 1 critique** — 0.72 satisfaction, axis = `typography`. Rationale: *"Space Grotesk and Sora are both tech-forward geometric sans-serifs that clash with the quiet, material-honest, craft register the prose promises for a Seoul-based walnut furniture maker — they read more like a fintech startup than a woodworker's studio."* This is **exactly the register-mismatch reasoning Pass 7 ran reactively**. ✓
- **Turn 1 refine** landed: Cormorant Garamond + Source Sans 3. (Renaissance serif — direction is *correct away from tech-geo*, but lands in a different bad bucket Pass 7 explicitly avoided. The seeded `editorial-portfolio` skill encodes this lesson, but Phase 8 skillify is still a stub, so the orchestrator can't load it.)
- **Turn 2 critique** crashed: critic emitted `{satisfaction: 0.88, weakest_axis: null, rationale: null, proposed_delta: null}` — a clean "satisfied, nothing more to say" verdict. My parser declared `rationale: String` rather than `Option<String>`, so the loop reported `critic_error` and stopped *one keystroke short* of recording a successful internalised refinement.

### Bug fix

`SelfCritique.rationale` is now `String` with a serde deserialiser that maps both missing fields and explicit JSON null to `""`. `weakest_axis` and `proposed_delta` are `Option<String>` with explicit nullable handling (empty string and the literal "null" also collapse to None). Added regression test `parses_all_null_rationale_and_axis` that uses the exact JSON Run A choked on.

### Run B — critic over-satisfied at turn 1 (319 s, 0 turns)

After the parser fix, same command, fresh sandbox:

- **Phase 3 (design)** in 312 s. Generator emitted Instrument Serif + Inter. **Modern display serif + clean grotesque sans.** This is meaningfully closer to Pass 7's *final* state than Pass 7's *initial* state was.
- **Turn 1 critique** — 0.92 satisfaction, axis = null. Stop.

Comparing Run B's accepted design to what Pass 7 reactively shaped over 4 turns:

| Axis | Pass 7 final (after 4 reactive turns) | Pass 8 Run B (0 turns) |
|---|---|---|
| display family | Source Serif 4 | **Instrument Serif** |
| body family | Outfit | Inter |
| primary 500 | #9a7550 walnut | walnut equivalent |
| max spacing | **224 px** | 96 px |
| brand-a bg | #faf9f6 gallery white | #e9e4db (hanji paper — culturally anchored, not parchment-bug) |

Run B is *partially* on parity (typography register correct, palette anchored, structure right) and *partially* short (spacing scale collapsed at 96 px — the exact "prose breathes but tokens don't" mismatch Pass 7 reactively caught on its turn 3).

**The critic missed it.** Spacing is one of the eight axes the system prompt names, with `prose_token_mismatch` as priority 1. Yet it gave 0.92 with no refines. Three plausible explanations:

1. **Sycophantic critic.** LLMs trained on RLHF tend toward agreement; once the typography register is right, the critic stops digging.
2. **The prose is less assertive about breathing room in Run B's DESIGN.md.** Pass 7's "pages breathe" language was a stronger hook for a prose-vs-tokens mismatch verdict.
3. **No anchor for "good."** With prefs reset and Phase 8 skillify still stubbed, the critic has no `editorial-portfolio` skill telling it "max spacing ≥ 160 px is the standard for this genre." It judges in a vacuum and the vacuum biases toward acceptance.

## What this tells us

**The internalised loop works at the contract level.**
- `aphrodite create` runs design → critic → refine → re-compose → re-render → validate → write end-to-end with no external interaction.
- The critic's register-mismatch reasoning (Run A turn 1) is *the same kind of reasoning* I was doing externally in Pass 7. Internalising it does produce correct judgments.
- Run B reached an arguably-better-than-Pass-7-turn-1 design with zero refines and **three quarters the wall time** of one Pass 7 turn.

**The internalised loop is currently under-tuned in two specific ways:**
- **Parser was brittle.** Fixed by Run A's surfacing. Regression test in place.
- **Critic is too easily satisfied.** Run B's 0.92 with no axis pick is the failure pattern to fight in v0.3.x. The fix is *not* "lower the threshold" — that just makes the critic invent work. The fix is **load skills as scaffolds** (ADR 0004 Phase 8). With `editorial-portfolio` in the system prompt, the critic has a concrete checklist ("verify spacing reaches ≥160 px for editorial-pace portfolios") and the prose-token mismatch becomes hard to miss.

**The Cormorant Garamond landing in Run A is real evidence for skills mattering.** The critic correctly moved away from Space Grotesk (tech-geo register) but landed on Cormorant Garamond (Renaissance register), which Pass 7 explicitly rejected. The seeded skill encodes *exactly that lesson*. Run A is the strongest argument for prioritising Phase 8 skillify next.

## Cost / time comparison

| Pass | Turns | Wall clock | Cognitive load on caller | Final result |
|---|---|---|---|---|
| 7 (reactive) | 4 | 1036 s | High — read DESIGN.md + composition each turn, author delta | 5 / 5 |
| 8 Run A | 1 (+ crash) | 475 s | Zero | Stopped mid-loop; Cormorant landing |
| 8 Run B | 0 | 319 s | Zero | Better baseline than Pass 7 turn 1; missed spacing |

Run B is the *interesting* baseline. It cost 31% of Pass 7's wall clock and required no caller attention. The output isn't Pass 7's final, but it's also not Pass 7's starting point — the **generator alone** has improved (or simply rolled well this time) since Pass 7. Critic's failure to refine is the gap.

## Honest agent satisfaction

| Aspect | Pass 7 | Pass 8 Run A | Pass 8 Run B |
|---|---|---|---|
| Output is the page asked for | 5 / 5 | 3 / 5 (mid-loop) | 4 / 5 (no spacing) |
| Multi-turn fidelity (each turn surgical) | 5 / 5 | 5 / 5 on the one completed turn | n/a (zero turns) |
| Loop ran without external input | n/a | 4 / 5 (until crash) | **5 / 5** |
| Critic identified the right weakest axis | n/a (external) | 5 / 5 (typography register) | 1 / 5 (missed spacing) |
| Wall clock | 1 / 5 | 2 / 5 | **4 / 5** |
| Parser robustness | n/a | 1 / 5 (now fixed) | 5 / 5 |

**Overall: 3.5 / 5** for Pass 8 as a system test. The internalisation succeeds at the architecture level; the *judgment* layer needs skill-loading to match Pass 7's specificity.

## Findings opened by Pass 8

- **#27** Critic over-satisfied without skill scaffolds. Same intent that Pass 7 needed 4 reactive turns for, Pass 8 Run B refines zero times. Fix: ship Phase 8 skillify + Phase 1 skill loading from `~/.aphrodite/skills/` into the critic's system prompt.
- **#28** Cormorant Garamond is a known-bad landing for "Seoul craftsperson workshop" register; seeded `editorial-portfolio` skill encodes this. Skill-loading would have steered Run A's turn 1 toward Source Serif 4 / Newsreader instead.
- **#29** Critic latency (~50 s) + refine latency (~100 s) + surface re-compose (~80 s) → one critic→refine round trips ~230 s. Three turns = ~700 s on top of the initial ~250 s design. Budget pressure for v0.3 dogfood patterns.

## Open from Pass 7 (carried)

- **#24** Font @import injection (harmonize phase)
- **#25** Composition regeneration on every refine
- **#26** Hero ignores typography tokens

## How to reproduce

```bash
cargo install --path crates/aphrodite-cli
mkdir -p /tmp/pass8-create && cd /tmp/pass8-create
git init && rm -f ~/.aphrodite/taste/preferences.toml
aphrodite create "a portfolio site for an independent furniture maker working in solid walnut, based in Seoul" --json
```

Run A and Run B were both run from a clean sandbox with no taste preferences. Run A was the original (parser-bug) run; Run B is post-fix. Both archived at `docs/agent-eval/archive/2026-05-14-pass8-autonomous/`.

## Next move

Pass 8 makes the Phase 8 case empirically: **skill loading is the single biggest unlock for autonomous quality**. Without it, the critic's judgment is generic and the loop terminates early on plausible-but-suboptimal results. With it, the seeded `editorial-portfolio` skill (commit `6a4f5fd`) gives the critic a checklist Pass 7 already validated.

Phase 8 / skillify implementation:
1. Orchestrator calls `aphrodite_core::skills::rank_for_intent(tags_from_intent, top_k=3)` before phase 3
2. Loaded skills' bodies prepend to the design AND critic system prompts as scaffold context
3. Post-loop: if `prior_deltas.len() >= 3` AND `final_satisfaction >= 0.85`, propose a new skill from the trajectory (don't auto-write — surface a `proposed_skill` field in the JSON for now)

Pass 9 will dogfood this with skills loaded.
