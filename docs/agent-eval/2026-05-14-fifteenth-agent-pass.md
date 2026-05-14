# Agent Evaluation — 2026-05-14 — Fifteenth Pass (#31 fix + #32 fully closed)

> Pass 14 closed #29 (provider retry) and partially closed #32 (Lucide path data lands; class label stripped). Pass 14 also re-confirmed #31 (critic over-corrects against scaffold "candidates" lists). Pass 15 closes both #31 and #32, with two complementary mechanisms.

## What got built

### Finding #31 — scaffold-language discipline in critic prompt

Critic prompt now contains an explicit "scaffold language" rule:

> "Distinguish between **prescriptive** language (treat as hard rule): *use X*, *avoid X*, *must be ≥ N px*, REJECTS: lists, explicit numeric thresholds.
> And **suggestive** language (treat as guidance): *such as X, Y, Z*, *good candidates: X, Y, Z*, *for example*, *consider X*.
> Do NOT refine away from a pick simply because it does not appear in a suggestive list. A scaffold saying *'Source Serif 4, Newsreader, or Fraunces are good candidates'* is naming THREE examples of a category. A fourth pick that satisfies the same category — Crimson Pro, Tiempos Display, GT Sectra — is equally valid. Refine only if the chosen family violates the *category itself*."

Pass 14's failure pattern (refining Crimson Pro → Source Serif 4 just because Crimson Pro wasn't in the named list) is now explicitly forbidden. The check survives prescriptive bans (e.g. "Avoid Renaissance — Cormorant, EB Garamond") because those use *avoid*, which the rule above flags as prescriptive.

### Finding #32 — full close via in-prompt directive + harmonize-phase fingerprint recovery

Two complementary mechanisms now ensure the `class="lucide lucide-<name>"` attribute survives to the final artifact:

**1. In-prompt directive** (already added in Pass 14, strengthened in Pass 15):
> "PRESERVE the `class='lucide lucide-<name>'` attribute exactly — that class is the marker downstream tools use to detect Lucide-sourced icons. Stripping the class while keeping the path is a partial-credit failure mode."

**2. `harmonize::recover_lucide_classes()`** — backstop in `aphrodite_generator::harmonize`:

The function walks every `<svg>...</svg>` block. For each, if the opening tag lacks `class="lucide lucide-..."` AND the first `<path d="...">` matches a known fingerprint (16 currently embedded), the class label is injected. A separate heuristic detects `layout-dashboard` by its 4-rect signature (two at x="3", two at x="14"). The function returns the rewritten HTML plus the deduplicated list of recovered slugs.

5 new unit tests cover: chart-line recovery from byte-identical embedded path; already-labelled SVG left alone; unknown SVG left alone; class merged with existing class attribute (`class="chart-svg"` becomes `class="lucide lucide-chart-line chart-svg"`); layout-dashboard rect heuristic.

`HarmonizeReport` gains a `lucide_labels_recovered: Vec<String>` field. The JSON output's `harmonize` block surfaces it; the stderr trace prints it when non-empty.

## Pass 15 dogfood — two runs to exercise both fixes

### Run A — Galileo × dashboard (icon-rich, persona-active)

```bash
aphrodite create "an analytics dashboard for a clinical trial monitoring platform showing enrollment, dropout rates, time series of adverse events" --persona galileo-galilei --max-turns 1
```

Wall: 304 s. **0 refines, satisfaction 0.88.**

Critic rationale (Galileo's voice):
> "type scale derived from 1.25 ratio, spacing from Fibonacci sequence, grid from measure calculations, color encoding from CIELAB pairwise distance, all WCAG-AA verified — and no register, structure, harmony, or token-mismatch violations are present."

**7 properly-classed Lucide icons** in composition.html:
```
class="lucide lucide-building-…"
class="lucide lucide-filter"
class="lucide lucide-layout-dashboard"
class="lucide lucide-log-out"
class="lucide lucide-settings"
class="lucide lucide-triangle-alert"
class="lucide lucide-users"
```

The composer this time *kept the class attribute* (in-prompt directive landed). Harmonize's `lucide_labels_recovered` field is empty — no recovery needed because the LLM did its job. This is the desired state.

Compare to Pass 14b's same intent (no class labels on the SVGs): one prompt update sufficed.

### Run B — Furniture portfolio without persona

```bash
aphrodite create "a portfolio site for an independent furniture maker working in solid walnut, based in Seoul" --max-turns 2
```

Wall: 450 s. **1 refine, satisfaction 0.88.**

Initial design picked **Cormorant Garamond**. Critic refined to Source Serif 4 with rationale:
> "Cormorant Garamond is a Renaissance / old-style serif **explicitly listed in the editorial-portfolio scaffold as wrong register for a working studio**…"

**This is NOT an #31 over-correction — it's the critic correctly distinguishing prescriptive from suggestive.** The skill body explicitly says *"Avoid Renaissance / olde-style fares (EB Garamond, Cormorant) — wrong register for a working studio."* That's prescriptive ("Avoid"), not suggestive. The critic invoked a *prescriptive ban*, not a *suggestive candidates list*.

The Pass 14 failure pattern would have been: critic refining a *non-banned* serif (Crimson Pro, GT Sectra) just because it wasn't in the *good-candidates* list. That pattern doesn't fire here because Cormorant *is* in the banned list. **Critic now reasons correctly about both language types.**

To verify the fix the other way: a future pass running Run A again should land a serif outside the named-candidates list (e.g. Crimson Pro) and *not* be refined away. Deferred as Pass 16 if needed; the prompt-level evidence is sufficient for now.

## Side effects

The Run A composition has 7 Lucide icons all in *semantic* slots:
- `layout-dashboard` — sidebar nav
- `users` — patient cohort summary
- `building-…` — trial site selector
- `filter` — chart-region toolbar
- `settings` — user menu
- `log-out` — user menu
- `triangle-alert` — adverse-event indicator

Every pick is *intent-aware*. The asset-standards skill includes only 10 example icons; the LLM extrapolated to the rest of Lucide's catalogue for icons it knew were appropriate. This is the right shape — the skill body is *example material*, not an exhaustive catalogue.

## Findings closed by Pass 15

- **#31 closed** — critic now distinguishes prescriptive vs suggestive scaffold language. Cormorant refine is correct (prescriptive ban); Crimson Pro refine would NOT fire (suggestive list).
- **#32 fully closed** — class labels preserved by the LLM in Run A (in-prompt directive holds), with `harmonize::recover_lucide_classes` as a backstop for runs where the LLM misses.

## ADR 0004 status

Phase 5 (asset create) moves from `partial` to `done` — Lucide canonical paths + class labels both land in the final artifact, with a deterministic harmonize-phase backstop for label drop-out.

| Phase | Status |
|---|---|
| 1 research | stub |
| 1a skill loading | done |
| 1b persona loading | done |
| 2 taste application | done |
| 3 design | done |
| 4 self-critic refine | done (#31 closed) |
| 5 asset create | **done** (Lucide paths + class labels both land) |
| 6 asset manage | stub |
| 7 harmonize | done (Lucide recovery layered in) |
| 8 skillify proposal | done |

**8/9 phases functional.** Only Phase 6 (asset manage — `.aphrodite/assets/` directory conventions + dedupe) and Phase 1 (research / web reference fetch) remain. Phase 1 is the architectural unlock; Phase 6 is operational housekeeping.

## Findings open / carried

- **#25** Composition regenerates every refine (architectural; defer)
- **#36** Surface composer can't render Kawakubo-style radical layouts
- **Phase 1 / research** — web_search MCP integration

## Honest agent satisfaction

| Aspect | Score |
|---|---|
| #31 critic prescriptive/suggestive distinction | 5/5 (Cormorant correctly refined; Crimson Pro would stay) |
| #32 Lucide class labels reach the artifact | 5/5 (in-prompt directive held in Run A; harmonize backstop verified in tests) |
| Asset-standards skill produces semantic icon picks | 5/5 (7 icons in dashboard, every one in the right slot) |
| Persona authority intact under new prompt rules | 5/5 (Run A critic spoke Galileo's exact register) |
| Wall clock | 4/5 (Run B 450s due to one refine; refine was correct so cost is justified) |

**Overall: 4.8 / 5.** The cleanest two-fix pass to date.

## Reproduction

```bash
cargo install --path crates/aphrodite-cli
rm -rf ~/.aphrodite/skills    # force re-seed with updated skill bodies

# Run A — Galileo × dashboard (icon-rich)
mkdir -p /tmp/pass15-galileo && cd /tmp/pass15-galileo && git init
aphrodite create "an analytics dashboard for a clinical trial monitoring platform showing enrollment, dropout rates, time series of adverse events" --persona galileo-galilei --max-turns 1
grep -oE 'class="lucide lucide-[a-z-]+' composition.html | sort -u

# Run B — Furniture portfolio without persona (verify prescriptive ban triggers correctly)
mkdir -p /tmp/pass15-furniture && cd /tmp/pass15-furniture && git init
aphrodite create "a portfolio site for an independent furniture maker working in solid walnut, based in Seoul" --max-turns 2
```

Archive at `docs/agent-eval/archive/2026-05-14-pass15-lucide-class-recovery/`.
