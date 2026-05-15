---
name: clinical-dashboard
description: Clinical / scientific / analytics dashboards — data density, monospace tables, semantic colour coding, no decorative chrome. The Galileo register.
version: 1.0.0
tags:
  - dashboard
  - analytics
  - clinical
  - data-dense
  - monitoring
  - scientific
related_skills:
  - asset-standards
agent_created: false
default: false
---

# Clinical / scientific dashboard

## When this skill applies

Intent mentions: dashboard, analytics, clinical trial, monitoring, observability, telemetry, scientific instrument, lab result, time-series, KPI surface, control room. The user is an operator scanning the page at-a-glance for state — not a marketing visitor.

## Workflow

1. **Information density.** Operator dashboards run at higher density than marketing pages. Body 13-14 px (not 16-18). Grid gutters 12-16 px (not 24-32). Multiple data regions per fold.

2. **Monospace for numbers.** Every number is in a monospace family — JetBrains Mono, IBM Plex Mono, Söhne Mono, ui-monospace. Column alignment is structural. Reading a 4-digit count next to a 6-digit count in a proportional font is *wrong*.

3. **Semantic colour with redundant encoding.** Status colour ≠ the only signal. Green / amber / red carries information BUT must be paired with text or an icon (`check`, `triangle-alert`, `x`). Don't rely on colour alone — operators include colourblind users and night-shift operators with desaturated monitors.

4. **Modular type scale from a measurable ratio.** 13 × 1.25 = 16. 16 × 1.25 = 20. 20 × 1.25 = 25. 25 × 1.25 = 31. Pick from this sequence. Big metrics 36-48 px (single number); section headings 16-20 px; body 13-14 px; labels 11 px.

5. **Grid layout — visible structure.** 12-column with explicit dividers (hairline `rgba(0,0,0,0.10)` light, `rgba(255,255,255,0.10)` dark). The operator should see the regions as discrete; don't dissolve dividers for aesthetic minimalism.

6. **Layout pattern:**
   - Top bar: title + global filter chips (cohort / time range / site) + user menu
   - Left sidebar: navigation (≥ 5 items, Lucide icons + labels at 14 px)
   - Main canvas: 4-6 metric tiles in a 2 × 3 or 3 × 2 grid
   - Below: 2 inline-SVG charts (line / bar, never pie — pies don't scale)
   - Below: recent-activity table (≥ 8 rows, monospace columns)
   - Status: a sticky bottom strip with "last update Xs ago" + connection state

7. **Chart conventions.** Inline `<svg>` paths, no JS libs. Use the design system's neutral as the baseline grid, primary as the line/bar fill, accent only for highlights. Y-axis labels in 11 px monospace. Legends inline with the chart, not floating.

8. **Anti-AI-slop in dashboard register.** No "AI-powered insights" or "intelligent recommendations" as feature tags — they're vapor. Name the actual metric ("Recruitment lag", "Time-to-first-event", "Adverse-event rate").

## Do / Don't

- DO emit Lucide icons in sidebar (`layout-dashboard`, `users`, `flask-conical`, `chart-line`, `triangle-alert`, `settings`).
- DO use `triangle-alert` (not `circle-alert` or `info`) for adverse-event indicators.
- DO use real-range numbers ("12 years" not "10+ years"; "87 commissions" not "100+ commissions").
- DO show last-update timestamp explicitly.
- DON'T use pie charts.
- DON'T use 3D chart effects, gradients in chart bars, or rounded chart corners.
- DON'T centre numbers in their cells — right-align (numerical alignment).
- DON'T use emoji in any data cell.

## Reference fragments worth lifting

- Body: 13-14 px monospace for numbers, 13-14 px sans (Inter / IBM Plex Sans) for labels
- Display metric: 36-48 px in display sans (Inter Display Semi-Bold)
- Sidebar width: 240-280 px on desktop, collapses to icon-only at < 1024 vw
- Table row height: 36-40 px, hover state shifts background by 4%
- Status colours (with semantic icons):
    - ok      = `--colors-accent-500` greenish + `lucide-check-circle`
    - warning = amber `--colors-warning-500` + `lucide-triangle-alert`
    - error   = `--colors-danger-500` red + `lucide-x-circle`
- Chart axis labels: 11 px monospace, 60% text opacity

## Cross-references

- Persona: `galileo-galilei` is the natural pair (measurable principles, modular scale, Fibonacci spacing).
- Wiki: search future entries for "Grafana", "Datadog", "Vercel Observability" — none seeded yet; consider adding when available.
