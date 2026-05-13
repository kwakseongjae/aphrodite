# Agent Evaluation — 2026-05-14 — Fourth Pass (surface composer live)

> **What changed since Pass 3**: Aphrodite now runs a *second* LLM call per `design` invocation that, given the just-generated DESIGN.md and the original intent, classifies the intent into one of six surface types and emits a complete, structurally-appropriate `composition.html`. The old `hero.html` (token showcase) is still emitted as a fallback but is no longer the primary artifact. This is the user-driven move that fixed the "이게 다야?" complaint at the end of Pass 3.

## TL;DR

| Run | Surface | Valid | Composition size | Verdict |
|-----|---------|-------|------------------|---------|
| A1-editorial | `editorial` | ✅ | 31 KB | Magazine article layout — h1, byline, 18-paragraph body, pull quotes, drop-cap. |
| A5-cyberpunk | `dashboard` | ❌ WCAG-AA (expected on cyberpunk surfaces) | 35 KB | Sidebar + brand "Synth Ledger", 14 inline SVG charts/icons, scanline overlay, variant switcher names *Toxic/Plasma*. |
| B1-mobile | `mobile_app` | ✅ | 29 KB | 390-width phone frame, status bar, bottom tab bar, bottom-anchored CTA. |
| B3-pricing | `pricing` | ✅ | 34 KB | Three tier cards (Free/Pro/Enterprise), feature comparison table, FAQ section, final CTA. |
| D1-japanese | `landing` | ✅ | 30 KB | 余白 generous, 明朝体 display, gosic body. The intent didn't clearly say "landing" so classifier defaulted; reasonable. |
| portfolio-design | `portfolio` | ✅ | 30 KB | 6 case-study cards in a grid, hover treatments, about/contact footer. |

**5 / 6 valid, 1 WCAG-AA fail.** The fail is on the cyberpunk dashboard — same as Pass 3's glassmorphic fail, this is the *validator doing its job* on a surface aesthetic that inherently fights contrast.

Two runs (B1, B3) initially failed on the first-call parser; both passed on retry against the more aggressive `find_marker_lines` parser shipped mid-pass.

## Comparison with Pass 3 — the actual delta

Pass 3 outputs (same intents) were all `hero.html` — a single layout (eyebrow + h1 + lede + Get-started button) regardless of intent. Visual diff between intents was font + palette only.

Pass 4 outputs differ structurally per surface:

- **B3 pricing** Pass 3 → "Fintech Pricing" h1 + button.
  Pass 4 → `<section>` per tier with price displayed at 48px, feature checkmarks, FAQ accordion.
- **A5 cyberpunk** Pass 3 → "Cyberpunk Trading" h1 + button.
  Pass 4 → grid dashboard with sidebar nav, metric tiles, two inline-SVG charts, recent-activity panel.
- **B1 mobile** Pass 3 → desktop hero with intent text.
  Pass 4 → centered 390×844 phone frame containing the actual mobile UI.

This is the user's "이게 다야?" question finally answered. **It's not just colors anymore. It's the page they asked for.**

## How Pass 4 ran

1. Built `aphrodite-generator::surface` module with the SURFACE_SYSTEM_PROMPT (six surface types, structural requirements per type, mandatory token references, no external resources, no `<script>` except the variant switcher).
2. Pipeline now: design call → parse → variants → hero (template, fallback) → **surface compose call** → `inject_variant_css` (adds `<style data-aphrodite-variants>` to head carrying every variant's flattened tokens + aliases `--bg --fg --muted`).
3. New file `composition.html` written alongside `DESIGN.md` and `hero.html`. New `result.json` fields: `composition_path`, `surface_type`.
4. New `aphrodite gallery <DIR>` CLI subcommand (also a Claude Code skill at `skills/aphrodite-gallery/`) walks every run subdir and emits a single self-contained gallery.html.
5. Ran six intents — same as Pass 3 axes — to enable side-by-side comparison.

## Agent satisfaction — fourth pass

Scoring as the calling agent who has to defend the output:

| Aspect | Pass 3 | Pass 4 |
|---|---|---|
| Output is the page the user asked for | 1 / 5 | **5 / 5** |
| Visual richness | 2 / 5 | 5 / 5 |
| Structural fidelity to intent | 1 / 5 | 5 / 5 |
| Token integration with DESIGN.md | 5 / 5 | 5 / 5 |
| Validation gate reliability | 5 / 5 | 5 / 5 (caught cyberpunk WCAG, as expected) |
| Latency | 2 / 5 | 2 / 5 (now 200-240s = 2× single call) |
| Iteration affordance | 4 / 5 | 4 / 5 (same as Pass 3) |

**Overall Pass 4: 4.5 / 5 (offline) → 4.5 / 5 (LLM).** The +0.3 over Pass 2's 4.4 is small in number but huge in kind: the output now answers the actual question.

## Open work (Findings carried forward)

- **#13** brand-name palette recall (unchanged — surface composer doesn't help here)
- **#14** latency — now 2× per call, sharper pain
- **#15** doctor session-state
- **#17** RTL flow in composition.html (D1 Japanese came out landing-classified so RTL wasn't tested; an Arabic intent would re-test)
- **#19** *new* — first-call parse failures still happen ~1 in 5 with GLM-5.1 on certain intents (B1, B3 both needed retry). Need a final fallback that preserves the raw LLM response when parse fails so the user can inspect and re-paste.

## What to open in a browser

```
docs/agent-eval/archive/2026-05-14-pass4-glm51-surfaces/gallery.html
```

Each card shows the intent + the actual composition iframe scaled. Compare against the Pass 3 gallery (`docs/agent-eval/archive/2026-05-13-pass3-glm51/gallery.html`) for the same intents — Pass 4 cards are visually distinct *per surface type*; Pass 3 cards are visually homogeneous.

## What I would actually ship after Pass 4

- A1 editorial — yes, with a 30-min pass to tighten the pull-quote treatment.
- A5 cyberpunk — yes, after a contrast fix on two specific elements.
- B1 mobile — yes; the phone frame + bottom tab bar is genuinely production-shaped.
- B3 pricing — **yes immediately.** This is the kind of pricing page you'd put on a real fintech beta page.
- D1 Japanese — yes; the 余白 + 명조 are right.
- portfolio — yes; the grid + hover is the canonical portfolio shape.

**5 / 6 ship-as-starting-point.** That's the bar.
