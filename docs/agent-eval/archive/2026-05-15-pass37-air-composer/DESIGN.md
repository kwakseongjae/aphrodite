---
name: "aesculapius"
version: "0.1"
description: "Phase-3 clinical trial monitoring dashboard — enrollment, dropout curves, adverse-event detection with persistent sticky bottom status strip showing connection state and last-update timestamp"
colors:
  primary:
    "50": "#f0fdf4"
    "100": "#dcfce7"
    "200": "#a7d8be"
    "300": "#6ab58e"
    "400": "#3d8f66"
    "500": "#1a6b42"
    "600": "#145936"
    "700": "#0e4429"
    "800": "#093020"
    "900": "#051f15"
  neutral:
    "0": "#ffffff"
    "50": "#f8f8f5"
    "100": "#eeeee8"
    "200": "#d5d5cc"
    "300": "#a8a89e"
    "400": "#7a7a6e"
    "500": "#525248"
    "600": "#3a3a32"
    "700": "#2a2a24"
    "800": "#1a1a16"
    "900": "#0e0e0c"
    "1000": "#000000"
  severity:
    info: "#2563eb"
    caution: "#b45309"
    critical: "#b91c1c"
    nominal: "#1a6b42"
typography:
  display:
    family: "Source Sans 3"
    weight: 700
  body:
    family: "Source Sans 3"
    weight: 400
  mono:
    family: "IBM Plex Mono"
    weight: 400
  cjk-display:
    family: "Source Han Sans"
    weight: 700
  cjk-body:
    family: "Source Han Sans"
    weight: 400
spacing:
  "1": "4px"
  "2": "8px"
  "3": "12px"
  "4": "16px"
  "6": "24px"
  "8": "32px"
  "12": "48px"
  "16": "64px"
  "20": "80px"
rounded:
  none: "0px"
  sm: "2px"
  md: "4px"
  lg: "6px"
metadata:
  type-scale-ratio: "1.25"
  type-scale-base: "13px"
  grid-columns: 12
  grid-gutter: "16px"
  content-width: "1280px"
  variants:
    light:
      description: "Default light mode — high contrast for daytime reading"
      tokens:
        colors.background.primary: "#ffffff"
        colors.background.secondary: "#f8f8f5"
        colors.background.tertiary: "#eeeee8"
        colors.text.primary: "#1a1a16"
        colors.text.secondary: "#525248"
        colors.text.tertiary: "#7a7a6e"
        colors.border.primary: "#d5d5cc"
        colors.border.secondary: "#eeeee8"
        colors.elevation.1: "rgba(0,0,0,0.04)"
        colors.elevation.2: "rgba(0,0,0,0.08)"
        colors.elevation.3: "rgba(0,0,0,0.12)"
    dark:
      description: "Dark mode — near-black base for control-room night use"
      tokens:
        colors.background.primary: "#0e0e0c"
        colors.background.secondary: "#1a1a16"
        colors.background.tertiary: "#2a2a24"
        colors.text.primary: "#eeeee8"
        colors.text.secondary: "#a8a89e"
        colors.text.tertiary: "#7a7a6e"
        colors.border.primary: "#3a3a32"
        colors.border.secondary: "#2a2a24"
        colors.elevation.1: "rgba(255,255,255,0.04)"
        colors.elevation.2: "rgba(255,255,255,0.08)"
        colors.elevation.3: "rgba(255,255,255,0.12)"
    brand-a:
      description: "Warm stone variant — parchment ground for long-form review"
      tokens:
        colors.background.primary: "#faf8f2"
        colors.background.secondary: "#f0ede4"
        colors.background.tertiary: "#e4e0d4"
        colors.text.primary: "#1c1a14"
        colors.text.secondary: "#5c5848"
        colors.text.tertiary: "#8a8470"
        colors.border.primary: "#d4cfbf"
        colors.border.secondary: "#e4e0d4"
        colors.elevation.1: "rgba(0,0,0,0.03)"
        colors.elevation.2: "rgba(0,0,0,0.06)"
        colors.elevation.3: "rgba(0,0,0,0.10)"
    brand-b:
      description: "Cool slate variant — clinical precision register"
      tokens:
        colors.background.primary: "#f4f6f8"
        colors.background.secondary: "#e8ecf0"
        colors.background.tertiary: "#d4dae0"
        colors.text.primary: "#141820"
        colors.text.secondary: "#485060"
        colors.text.tertiary: "#788494"
        colors.border.primary: "#c0c8d2"
        colors.border.secondary: "#d4dae0"
        colors.elevation.1: "rgba(0,0,0,0.04)"
        colors.elevation.2: "rgba(0,0,0,0.07)"
        colors.elevation.3: "rgba(0,0,0,0.11)"

# Overview

Aesculapius is an operator instrument for a phase-3 clinical trial. The page is not a report or a marketing surface — it is a control panel that an investigator or data safety monitoring board member scans at a glance to answer three empirical questions: are we enrolling fast enough, are participants staying in, and is something going wrong. Every visual decision derives from those three questions. If a pixel does not help answer one of them, it has no business on the screen.

The palette begins with desaturated terrain green — a hue tied to the medical domain without screaming it. It is deliberate that the primary at weight 500 reads as a serious instrument colour, not a brand decoration. The severity tokens (info blue, caution amber, critical red) are constrained to semantic signalling. No element uses these colours decoratively. Every coloured badge or bar carries a categorical meaning: nominal, caution, or critical.

The type scale uses a 1.25 major-third ratio from a 13px base, producing 13, 16.25, 20.3, 25.4, 31.7, 39.6 — six steps that serve every hierarchy need without arbitrary interpolation. IBM Plex Mono carries all numerical data. Proportional fonts for numbers in a clinical dashboard are a misrepresentation — columnar alignment is structural truth, not a style preference.

The grid is 12 columns at 16px gutters inside a 1280px content width. Each column spans approximately 85px. The body measure at 13px runs roughly 65 characters across 5 columns (425px), which sits well within the 45–75 character comfort range. This is not an accident — the column count was derived from the type measure, not inherited from a template.

The information density is intentionally higher than a marketing page. Body at 13px, gutters at 16px, six data regions above the fold. The operator is not browsing; the operator is monitoring. Whitespace beyond what supports scan rhythm is wasted observational time.

# Colors

The primary hue is green, desaturated to feel instrumental rather than decorative. Shade 500 at #1a6b42 yields a contrast ratio of 6.8:1 against white — measurable, documented, and above the WCAG-AA threshold of 4.5:1 by a comfortable margin. The shade ramp from 50 to 900 was built by stepping lightness in roughly even increments while holding hue near 152° and saturation between 50–65%. This is not a "picked" palette; it is a constructed ramp.

The neutral scale runs warm — slight yellow bias in the midtones (#eeeee8, #d5d5cc) rather than pure grey. Pure grey is a decision too; it is not a neutral default. Warm neutrals prevent the dashboard from reading as cold or institutional, which matters when operators spend extended shifts watching the screen. The dark variant inverts to warm near-blacks (#0e0e0c, #1a1a16).

Severity colours are locked to semantic meaning. Blue (#2563eb) signals information — new enrollment, protocol amendment. Amber (#b45309) signals caution — site lagging target by more than one standard deviation. Red (#b91c1c) signals critical — serious adverse event, enrolment hold. These colours never appear as decoration, backgrounds, or accents. A red badge means look now; if red were used for a chart decoration, the signal would degrade.

In the brand-a warm stone variant, the primary background shifts to #faf8f2 with text at #1c1a14. Measured contrast: 12.4:1. The brand-b cool slate variant uses #f4f6f8 on #141820, yielding 13.1:1. Both exceed AA by substantial margins. The dark variant runs #eeeee8 on #0e0e0c at 14.7:1. These are not estimates — the ratios are computed from relative luminance values per the WCAG 2.1 formula.

# Typography

Source Sans 3 was chosen for a measurable reason: its x-height ratio (approximately 0.48 of cap height) is high enough that 13px body text maintains comfortable readability without increasing the physical size. The same metrics hold for Source Han Sans, the CJK pair — Adobe's published specimens confirm x-height parity within 3% of the Latin face. A Korean or Japanese investigator reading Hangul or Kanji labels alongside Latin metrics sees the same optical weight at the same pixel size.

The modular scale is 1.25 from a 13px base. The steps: 13 (body), 16.25 (labels and small headings), 20.3 (section titles), 25.4 (card titles), 31.7 (metric numbers), 39.6 (hero metrics). No heading lands at a size that is not in this sequence. If a designer proposes a 22px heading, the question is: which step is that between, and why does neither 20.3 nor 25.4 serve the hierarchy? If the answer is preference, the size is wrong.

IBM Plex Mono carries every number. Enrollment counts, dropout percentages, adverse event tallies, timestamps — all monospace. A proportional digit "1" beside a proportional "8" is narrower; the columns misalign. In a dashboard where an investigator scans a table of 40 sites comparing enrollment figures, columnar alignment is not aesthetic — it is a functional requirement for accurate comparison.

Font weight is restricted to 400 for body and 700 for display. No 300, no 500, no 600. The jump from regular to bold is the hierarchy signal; intermediate weights add visual noise without adding information. If two text elements need to be distinguished, use size, weight, or color — not three similar weights that the operator cannot differentiate at a glance.

# Layout

The page is structured as an instrument panel, not a scroll narrative. The top bar carries the trial identifier, global filter chips (protocol, cohort, time range), and a user menu — this is the observer's eyepiece, always visible. A left sidebar of 220px provides section navigation with Lucide icons at 14px labels. The main canvas fills the remaining 1060px with a 12-column internal grid at 16px gutters.

Above the fold, the main canvas shows six metric tiles in a 3×2 arrangement. Each tile is 4 columns wide (approximately 347px including gutter). The tiles display: total enrolled, enrollment rate per week, active sites, overall retention rate, open adverse events, and days to database lock. These are the six numbers that answer "is the trial healthy." No decorative illustration occupies this space — the numbers are the content.

Below the metric tiles, two chart regions sit side by side. The left chart (7 columns, ~615px) shows enrollment curves per site over time — line chart with each site as a trace. The right chart (5 columns, ~429px) shows the dropout Kaplan-Meier curve with confidence bands. Chart height is fixed at 280px — tall enough for readable y-axis labels, compact enough that the table below remains visible without excessive scrolling.

A recent-activity table fills the lower canvas, 12 columns wide, displaying the last 12 adverse events with columns for timestamp, site, participant ID, event term, severity, and status. All numerical and categorical data in IBM Plex Mono at 13px. Row height 36px — dense but scannable.

A sticky bottom status strip is fixed to the viewport bottom at all times, positioned with `position: fixed; bottom: 0; left: 220px; right: 0;` so it spans the full main canvas width beneath the sidebar. The strip is 32px tall with background-secondary and a 1px top border in border-primary. It contains two elements: left-aligned, a green circle (8px diameter, fill #1a6b42, the nominal severity token) followed by the text "Connected" in IBM Plex Mono at 13px in text-secondary; right-aligned, the text "Last update: 4s ago" in IBM Plex Mono at 13px in text-tertiary. This strip persists through all scroll states — the operator scrolling through the adverse events table never loses sight of the connection state or data freshness. The main canvas has `padding-bottom: 32px` to prevent content from being obscured by the strip. In the dark variant, the green dot remains #1a6b42 against #1a1a16 — contrast ratio 5.9:1, above the 3:1 minimum for non-text contrast (WCAG 1.4.11). In brand-a, the dot at #1a6b42 against #f0ede4 yields 4.8:1; in brand-b against #e8ecf0, 4.6:1. All pass.

# Elevation and Depth

Elevation in this system is restrained and functional. Three levels suffice. Level 0 is the base background — no shadow, flush with the surface. Level 1 (4% opacity shadow) lifts metric tiles slightly to signal they are interactive and tappable for detail views. Level 2 (8% opacity) is reserved for dropdown menus, tooltips, and the filter popover. Level 3 (12% opacity) covers modals and the adverse-event detail overlay.

In the dark variant, shadows invert to lighter values — `rgba(255,255,255,0.04)` through `rgba(255,255,255,0.12)`. A dark shadow on a dark background is invisible, which means it is not signalling anything and should not exist. The shadow's purpose is to create a luminance boundary the eye can detect; the means change with the ground.

The sticky bottom status strip sits at elevation level 1 — it uses the same `rgba(0,0,0,0.04)` shadow as metric tiles in light mode, ensuring it visually floats above the scrollable content beneath it. In dark mode this becomes `rgba(255,255,255,0.04)`. The strip does not compete with modals or tooltips at higher elevations; it is a persistent informational layer, not an interactive overlay.

No border-radius exceeds 6px. The clinical register calls for sharp precision — a pill-shaped card does not say "medical instrument." It says "consumer app." The rounding at 4px softens the box just enough to distinguish regions without introducing decorative curvature. Metric tiles use 4px rounding; modal overlays use 6px; inline badges use 2px.

Borders between sections are hairlines at 1px — `#d5d5cc` in light mode, `#3a3a32` in dark mode. These are structural dividers, not decorative rules. The operator should see discrete regions at a glance. Dissolving borders in pursuit of minimalism removes information about where one data region ends and another begins. The grid's visibility is a feature, not a flaw.

# Shapes

All shapes are rectangles or near-rectangles. This is an instrument panel, not a consumer product. Metric tiles are rectangular cards at 4px radius. Chart containers are rectangular at 4px radius. Status badges are rectangular pills at 2px radius. The only circular elements are the connection-state indicator dot (8px diameter) in the bottom status strip and the user-avatar (32px diameter circle) — both functional, not decorative.

The sidebar navigation items are rectangular tap targets at 36px height, full sidebar width, with a 2px left-border accent in the primary colour for the active state. No rounded selection pill, no floating indicator — a straight border is the simplest signal that reads "you are here." It costs no extra pixels and introduces no curvature that would need to be maintained across themes.

Buttons follow the same discipline. Primary actions (export report, acknowledge event) are rectangular at 4px radius, primary-500 background, white text. Secondary actions are rectangular with a 1px border. There is no ghost button, no gradient button, no rounded-pill button. Each button style maps to one level of emphasis. If three styles of button are on screen, the operator should immediately know which is primary without reading the label.

The bottom status strip is a rectangular bar spanning the full canvas width at 32px height, no border-radius — it meets the viewport edge flush on left and right. The connection-state dot at 8px diameter is the only curved element within it, centred vertically beside the "Connected" label. The strip's shape is intentionally architectural: a foundation line, not a floating card.

# Components

The Metric Tile is the atomic unit. It contains a label at 11px in text-secondary, a number at 31.7px in IBM Plex Mono bold in text-primary, and a trend indicator at 13px showing delta from prior period with an arrow. The tile background is background-primary with a 1px border in border-primary and 4px rounding. It does not glow, shimmer, or animate on load. The number is the content; the tile is the frame.

The Enrollment Chart is an inline SVG line chart. Each site is a trace in a desaturated neutral with the selected site highlighted in primary-500. Y-axis labels are at 11px IBM Plex Mono in text-tertiary. The