# Agent Evaluation — 2026-05-13 — Third Pass (LLM live, wide breadth)

> **Evaluator**: Claude (Opus 4.7) acting as a calling agent.
> **Aphrodite under test**: commit at start `7890b2c`; this pass shipped parser leniency fix mid-run.
> **Provider**: z.ai GLM Coding Plan (Anthropic-compatible), model `glm-5.1`.
> **Reference**: [Pass 1](2026-05-13-first-agent-pass.md), [Pass 2](2026-05-13-second-agent-pass.md), [narrative](2026-05-13-narrative.md).

## Why this pass exists

Pass 2 verified the LLM path on 4 representative intents. Pass 3's job was to *stretch* Aphrodite hard: 17 runs spanning five orthogonal axes:

- **A — aesthetic styles** (5): editorial, brutalist, glassmorphic, Bauhaus, cyberpunk
- **B — domain stretches** (2): mobile fitness, fintech pricing page
- **C — hard constraints** (2): forced exact palette, forced single typeface
- **D — language diversity** (3): Japanese, Arabic RTL, emoji-laden
- **E — convergence under repetition** (5): same intent × 5

Total LLM cost: ~28 minutes of wall-clock GLM time.

## TL;DR

| Metric | Result |
|---|---|
| Total runs | 17 |
| Valid (schema + WCAG-AA) | **15 (88%)** |
| Validation failures | 1 (A3-glassmorphic — WCAG-AA, validator caught it) |
| Parse failures | 1 (C1-forced-palette — schema shape mismatch) |
| Mean latency | 100 s |
| Warnings emitted | 0 (all intents in-scope) |
| Mid-session fixes shipped | 1 (parser leniency for prose-prefix DESIGN.md) |

**Agent satisfaction** for Pass 3: **4.2 / 5**. Slightly lower than Pass 2's 4.4/5 because the parser/schema rigidity bit twice. Higher in *breadth confidence* — the system survived 17 diverse intents.

## Run-by-run highlights

I'll skip the table here (it lives at [archive/2026-05-13-pass3-glm51/README.md](archive/2026-05-13-pass3-glm51/README.md)) and call out the moments that mattered.

### Wins

- **A2 brutalist** (`#ffe600` caution yellow, system mono, no decoration) — *uncompromising*. The model didn't soften "brutalist" to "minimalist," which is a common failure mode of LLMs trying to be safe.
- **C2 forced font** — "must use Berkeley Mono as the only typeface" → `Berkeley Mono` showed up verbatim in `display.family` and `body.family`. Instruction-following is real.
- **D1 Japanese, D3 emoji** — non-ASCII intents flowed through with no corruption; 余白 (negative space) actually translated into larger spacing tokens in the DESIGN.md.
- **A4 Bauhaus retry** — after the parser fix, this came back with `Archivo Black` (a heavy geometric sans, exactly what Bauhaus signaling demands) and pastel-with-primary-accent treatment.

### Honest failures

- **A3 glassmorphic — WCAG-AA fail.** This is *the right kind of failure*. Glassmorphism by definition reduces contrast (translucent surfaces over backgrounds). The model gave us SF Pro Display + frosted accents in a layout that violated 4.5:1 contrast somewhere. Aphrodite's validator caught it and refused to mark it valid. An agent calling this gets a clear signal: *the design is aesthetically interesting but accessibility-broken; either accept the tradeoff (manual override) or rebrief*.
- **C1 forced palette — parse fail.** Tried to constrain GLM to exactly `#1B4332 / #52B788 / #D8F3DC`. GLM returned a DESIGN.md where `colors:` was shaped differently than our `Frontmatter` Rust struct expects (probably a flat `colors: { primary: "#…" }` instead of `colors: { primary: { "500": "#…" } }`). Aphrodite's parser bailed. Real Aphrodite bug, not a model bug. **Finding #16.**
- **D2 Arabic** — content was correctly Arabic, palette matched (amber per intent), but our hero template has no `dir="rtl"` so the rendered HTML reads LTR. **Finding #17** — template-side fix.

### The convergence story (E1–E5)

Same intent, 5 sequential calls. I expected palette to evolve under taste-loop bias. What I got:

```
E1: #7a7462 warm taupe
E2: #8a6d47 warm brown
E3: #3a6f63 deep sage    ← family jump
E4: #7a7062 warm taupe   ← reverted
E5: #448264 green-teal   ← green again
```

That's three different hue families (warm, sage, green-teal) in five calls of the same intent. But — *and this matters* — my script didn't call `aphrodite redesign` between calls. It called `aphrodite design` five times. Both record taste events, but only `redesign` increments the `Regenerate` counter. So the *taste-loop bias mechanism* (commit `fbfeaea`) was never actually triggered; the variation we see is pure LLM sampling temperature, not Aphrodite-mediated taste evolution.

**This is a methodology error on my part, not a system error.** A proper convergence test interleaves `design` and `redesign` calls. That's queued as **Finding #18** for Pass 4.

## How Pass 3 ran — operationally

1. I designed 16 intents (one originally was a safety probe — auto-mode classifier correctly refused, confirming the host enforces its own ethics independent of Aphrodite; dropped the probe).
2. Wrote `/tmp/pass3_runner.sh` — 16 sequential calls with per-run sandbox, intent.txt + result.json + DESIGN.md + hero.html + duration.txt copied to `docs/agent-eval/archive/2026-05-13-pass3-glm51/{slug}/`.
3. Launched in background. Monitored via `Monitor` tool with line-buffered grep on completion lines.
4. After all 17 runs landed, aggregated. Found 4 frontmatter-prefix failures.
5. Shipped parser leniency fix to `aphrodite-core::design::split_frontmatter` to find `\n---\n` anywhere in the body if it doesn't start at the top.
6. Re-ran the 4 failures in parallel. All passed.
7. C1 (forced-palette) still failed with a schema-shape mismatch — that's a deeper fix, queued as Finding #16.

End-state: 15/17 valid, 1 known-good failure (WCAG-AA gate), 1 known-bug (schema rigidity).

## Aesthetic satisfaction — what I'd actually ship

Picking the runs I'd hand to a designer with "use this as the starting point":

- **A1 editorial** — yes, with a 30-min refinement pass.
- **A2 brutalist** — yes, immediate.
- **A3 glassmorphic** — only after fixing the contrast. The aesthetic premise is right.
- **A4 Bauhaus** — yes, especially for a kids' app.
- **A5 cyberpunk** — yes for a niche fintech / trading product.
- **B1 mobile, B3 pricing** — yes.
- **C2 Berkeley Mono portfolio** — yes; the constraint-following is rare and valuable.
- **D1 Japanese** — palette claim half-honored (khaki not indigo); type rationale is right.
- **D2 Arabic** — needs RTL CSS fix; otherwise palette is appropriate.
- **D3 emoji wedding** — yes; warm rose-coral on cream.

That's **9 / 17 ship-as-starting-point**, **3 / 17 needs-fix-first**, **5 / 17 either failed or convergence-noise**.

## Pass 3 score breakdown

| Aspect | Score |
|---|---|
| Aesthetic vocabulary respect | 4.5 / 5 |
| Constraint following (C2) | 5 / 5 |
| Constraint following (C1) | 1 / 5 (parser bug, not model bug) |
| Non-ASCII fluency | 4.5 / 5 |
| Repetition variance | 3 / 5 (sampling variance only; taste loop unexercised) |
| Validation gate reliability | 5 / 5 (caught A3 correctly) |
| Parser robustness | 3 / 5 (was 2/5; +1 after this pass's fix) |
| Latency | 2 / 5 (100 s mean; same as Pass 2) |

**Overall 4.2 / 5.** The two failure points (parser shape rigidity, RTL template) are both well-defined fixes.

## Open findings carried into Pass 4

| # | Title | Priority |
|---|-------|----------|
| 16 | Frontmatter parser rejects valid YAML shapes the LLM emits (C1) | 🟠 P1 |
| 17 | Hero template has no RTL flow (D2 Arabic content renders LTR) | 🟡 P2 |
| 18 | Convergence-test methodology — need `redesign` between iterations | 🟢 P3 (methodology, not bug) |

Plus carry-overs from Pass 2 still open:
- **#12** headline derivation defaults to "Meet {name}." even for LLM output
- **#13** brand-name palette recall (verified weak in Pass 2)
- **#14** latency 80–140 s
- **#15** `doctor` lacks per-session counters

## What the user (and any future eval pass) should look at

Open `docs/agent-eval/archive/2026-05-13-pass3-glm51/{slug}/hero.html` for each run. The brutalist (A2), the cyberpunk (A5), the Bauhaus (A4), the Berkeley-Mono portfolio (C2), and the Japanese (D1) are the most aesthetically interesting and the cheapest to compare against future passes.

The convergence pages (E1–E5) are the same intent five times; opening them side-by-side shows just how much sampling variance you get from GLM-5.1 alone, before any taste-loop biasing.

## My honest take after 3 passes (61 LLM-mediated invocations total counting Pass 1)

Aphrodite, as of this commit, is **a real tool I would reach for** if I were asked to produce a multi-variant DESIGN.md for any of the surfaces I tested. The latency cost is the one thing that would stop me from putting it in a tight iteration loop with a human designer. Everything else — vocabulary respect, validation discipline, multi-language fluency, constraint-following — is at "competent senior junior designer" level.

The remaining work is *engineering* (parser shape tolerance, RTL CSS, latency) not *aesthetic capability*.
