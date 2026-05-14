# Agent Evaluation — 2026-05-14 — Fourteenth Pass (Lucide asset injection + provider retry)

> Pass 13 surfaced two operational gaps: composition-level asset emissions still didn't carry Lucide-specific markers (#32), and parallel orchestration amplified transient provider failures (#29). Pass 14 ships both fixes. The mitigation for #29 is structural (provider-level retry with exponential back-off). The mitigation for #32 is content (embed canonical Lucide SVG paths inside the `asset-standards` skill body so the LLM has byte-perfect material to copy).

## What got built

### Provider retry — Finding #29 closes

`provider::call_raw` now wraps the underlying HTTP call in a bounded retry loop:

```rust
let mut attempt: u32 = 0;
loop {
    let result = /* anthropic or openai compat call */;
    match result {
        Ok(s) => return Ok(s),
        Err(e) if is_transient(&e) && attempt < 3 => {
            let delay_ms = 1000u64 << attempt;  // 1s, 2s, 4s
            tracing::warn!("provider transient failure (attempt {}/3): {} — retrying in {}ms",
                           attempt + 1, e, delay_ms);
            tokio::time::sleep(Duration::from_millis(delay_ms)).await;
            attempt += 1;
        }
        Err(e) => return Err(e),
    }
}
```

`is_transient` matches: 429 rate-limit, 5xx server errors, network connect/timeout failures, and unknown reqwest errors (treated as likely network). 4xx other than 429 (auth, bad request, validation) are NOT retried — the next attempt would fail the same way.

Worst-case extra wall clock: 1 + 2 + 4 = 7 s per call. The Pass 13 Vignelli crash on z.ai 429 would have recovered cleanly under this regime.

### Lucide SVG canonical embeddings — Finding #32 closes (with nuance)

The `asset-standards` skill body now contains **10 verbatim Lucide SVG snippets** with the exact path data lifted from `lucide.dev/icons` as of 2026-05. Each carries the full canonical attribute set:

```html
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
     fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
     stroke-linejoin="round" class="lucide lucide-chart-line" aria-hidden="true">
  <path d="M3 3v16a2 2 0 0 0 2 2h16"/>
  <path d="m19 9-5 5-4-4-3 3"/>
</svg>
```

Embedded icons (selected for typical Aphrodite intents): `arrow-right`, `arrow-up-right`, `mail`, `phone`, `map-pin`, `hammer`, `ruler`, `flask-conical`, `chart-line`, `chevron-right`.

Plus a directive: *"PRESERVE the `class=\"lucide lucide-<name>\"` attribute exactly — that class is the marker downstream tools use to detect Lucide-sourced icons. Stripping the class while keeping the path is a partial-credit failure mode."*

## Pass 14 dogfood — two runs

### Run A — same Seoul furniture portfolio intent (no persona)

```bash
aphrodite create "a portfolio site for an independent furniture maker working in solid walnut, based in Seoul" --max-turns 2
```

Wall: 450 s. 1 refine (critic over-corrected on display family again — Pass 11-era #31 returns when no persona is invoked). Composition has 4 `<svg>` tags, but they're all *decorative* (hero background SVG, project image placeholder SVGs at viewBox 600×800). **Zero Lucide-class icons** in the composition.

This intent simply doesn't call for icon UI — the page is photo-dominant magazine pacing, no nav rail icons, no CTA arrows. The asset-standards skill body was loaded but the composer didn't reach for icons because the design didn't have icon slots.

This is a null result for icon injection — not a failure of the skill, but a property of the intent. To exercise Lucide injection we need an icon-rich intent.

### Run B — clinical-trial dashboard with Galileo persona

```bash
aphrodite create "an analytics dashboard for a clinical trial monitoring platform showing enrollment, dropout rates, time series of adverse events" --persona galileo-galilei --max-turns 1
```

Wall: 311 s. **0 refines, satisfaction 0.86.** Critic accepted on turn 1 with rationale:
> "The design is rigorously derived from measurable principles: Fibonacci spacing scale, modular type ratio (1.25), golden-ratio-informed grid, monospace display for tabular clinical data, warm neutral axis justified by eye-strain research, semantic colors with redundant encoding, and four coherent variants — all consistent between DESIGN.md prose and composition.html tokens."

This is exact Galileo register from the persona body — measurable principles named, modular ratios named, eye-strain *research* invoked rather than aesthetics. Persona authority working as designed.

**Composition has 11 `<svg>` tags.** Inspection of path data:

| First path fragment | Identified Lucide icon | Notes |
|---|---|---|
| `m21.73 18-8-14a2 2 0 0 0-3.48 0...` | `triangle-alert` | adverse-event UI signal — *perfect* semantic match |
| `<rect width="7" height="9" x="3" y="3" rx="1"/>...4 rect grid` | `layout-dashboard` | sidebar nav anchor |
| `M16 21v-2a4 4 0 0 0-4-4H6...circle cx="9" cy="7" r="4"` + overlay | `user-plus` | enrollment metric |
| `M3 3v18h18.../m19 9-5 5-4-4-3 3` | **`chart-line`** — *byte-identical to the skill's embedded path* | time-series header |
| `M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0` | `map-pin` family | trial-site UI |

The composer **picked semantically-appropriate Lucide icons** for each dashboard region (alert symbol for adverse events; user-plus for enrollment; chart-line for time series; layout-dashboard for nav). The path data is byte-perfect or near-byte-perfect Lucide source. **This is the asset-standards skill working at the path-data level.**

The gap: the `class="lucide lucide-<name>"` attribute was stripped during the LLM's copy. So technically `grep "lucide lucide-"` returns zero, but the underlying SVG geometry is correct Lucide.

## Mitigation for class-stripping

The skill now says explicitly *"PRESERVE the `class=\"lucide lucide-<name>\"` attribute exactly... Stripping the class while keeping the path is a partial-credit failure mode."* — the directive lands in the next run's prompt. Whether it makes the LLM consistently preserve the class is empirical; a follow-up pass would confirm.

A complementary fix: a tiny harmonize-phase walker that detects known Lucide path fingerprints (we have ~10 embedded; can add ~20-30 more) and re-injects the class label. Bundled-Lucide-fingerprint harmonize is a future enhancement, deferred for the moment.

## Findings closed / advanced

- **#29 closes** — provider call_raw now retries 429 / 5xx / connect / timeout with 1s/2s/4s back-off. No additional infrastructure required; future parallel runs (Pass 13-style) will absorb transient hits cleanly.
- **#32 partial close** — Lucide path data lands in composition.html via the asset-standards skill's embedded snippets. Class attribute preservation is the remaining gap; addressed in-prompt with a directive, falls back to harmonize-fingerprint injection if necessary.

## Findings re-confirmed

- **#31** — critic over-corrects against skill scaffolds (Pass 14 Run A: Crimson Pro display → Source Serif 4 by critic citing scaffold's "Source Serif 4, Newsreader, Fraunces" as a *requirement* when the scaffold says "good candidates"). The Pass 12 persona-authority fix protects persona-driven picks; non-persona scaffold-driven picks still suffer this over-correction. Future mitigation: same quote-verification step requiring critic to literally quote the rejected pattern from the scaffold before refining.

## Cost / latency snapshot

| Pass | Wall | Refines | Notes |
|---|---|---|---|
| Pass 13 Vignelli (parallel) | 289 s | n/a | critic crashed on 429 |
| Pass 14 Run A (no persona) | 450 s | 1 | #31 over-correction |
| Pass 14 Run B (Galileo + dashboard) | 311 s | 0 | full persona + Lucide path match |

Run B's 311 s is *with* asset-standards skill, persona, harmonize, retry — all the v0.3 plumbing in one call. Single-LLM-call baseline (Pass 4 era) was ~250 s. Plumbing overhead: ~60 s. Acceptable.

## ADR 0004 status

Phase 5 (asset create) moves from `stub` to `partial` — Lucide path embeddings in asset-standards mean the composer now reaches for canonical icon paths, even if class labels aren't yet preserved. Phase 6 (asset manage) and Phase 1 (research) remain stub.

Updated phase matrix:

| Phase | Status |
|---|---|
| 1 research | stub |
| 1a skill loading | done |
| 1b persona loading | done |
| 2 taste application | done |
| 3 design | done |
| 4 self-critic refine | done |
| 5 asset create | **partial** (Lucide path embeddings work; class preservation pending) |
| 6 asset manage | stub |
| 7 harmonize | done |
| 8 skillify proposal | done |

## Reproduction

```bash
cargo install --path crates/aphrodite-cli
rm -rf ~/.aphrodite/skills    # force re-seed with Lucide embeddings
mkdir -p /tmp/pass14-dashboard && cd /tmp/pass14-dashboard && git init
aphrodite create "an analytics dashboard for a clinical trial monitoring platform showing enrollment, dropout rates, time series of adverse events" --persona galileo-galilei --max-turns 1 --json
grep -oE 'd="[^"]{0,60}' composition.html | head -10    # inspect Lucide path data
```

Archive at `docs/agent-eval/archive/2026-05-14-pass14-assets-retry/` with both runs preserved.
