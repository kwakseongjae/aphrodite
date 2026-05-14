---
name: "trial-lens"
version: "0.1"
description: "Clinical trial monitoring dashboard — enrollment, dropout, adverse-event time series"
colors:
  primary:
    "50": "#f0f4f3"
    "100": "#d1dbd8"
    "200": "#a3b7b1"
    "300": "#75938b"
    "400": "#476f65"
    "500": "#1a4d3e"
    "600": "#153e32"
    "700": "#102e26"
    "800": "#0b1f1a"
    "900": "#06100d"
  neutral:
    "0": "#ffffff"
    "50": "#f7f8f7"
    "100": "#ebedea"
    "200": "#d5d9d5"
    "300": "#b0b7b2"
    "400": "#8a948d"
    "500": "#657269"
    "600": "#4d5a52"
    "700": "#36423b"
    "800": "#1f2a24"
    "900": "#0a130f"
    "1000": "#000000"
  danger:
    "50": "#fef2f2"
    "500": "#b91c1c"
    "900": "#450a0a"
  caution:
    "50": "#fefce8"
    "500": "#a16207"
    "900": "#422006"
  success:
    "50": "#f0fdf4"
    "500": "#15803d"
    "900": "#052e16"
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
spacing:
  "1": "4px"
  "2": "8px"
  "3": "12px"
  "4": "16px"
  "5": "20px"
  "6": "24px"
  "8": "32px"
  "10": "40px"
  "13": "52px"
  "16": "64px"
rounded:
  sm: "4px"
  md: "8px"
  lg: "12px"
metadata:
  variants:
    light:
      description: "Default light mode — high-clinical visibility"
      tokens:
        colors.background.primary: "#ffffff"
        colors.background.secondary: "#f7f8f7"
        colors.background.tertiary: "#ebedea"
        colors.text.primary: "#0a130f"
        colors.text.secondary: "#4d5a52"
        colors.text.muted: "#8a948d"
        colors.border.primary: "#d5d9d5"
        colors.border.focus: "#1a4d3e"
        colors.chart.enrollment: "#1a4d3e"
        colors.chart.dropout: "#b91c1c"
        colors.chart.adverse: "#a16207"
    dark:
      description: "Dark mode — extended observation sessions"
      tokens:
        colors.background.primary: "#0a130f"
        colors.background.secondary: "#101c16"
        colors.background.tertiary: "#1a2a22"
        colors.text.primary: "#ebedea"
        colors.text.secondary: "#b0b7b2"
        colors.text.muted: "#657269"
        colors.border.primary: "#2a3a32"
        colors.border.focus: "#4d9a7e"
        colors.chart.enrollment: "#4d9a7e"
        colors.chart.dropout: "#f87171"
        colors.chart.adverse: "#fbbf24"
    brand-a:
      description: "Sterile-field variant — warm neutral ground"
      tokens:
        colors.background.primary: "#faf9f5"
        colors.background.secondary: "#f2f0ea"
        colors.background.tertiary: "#e5e2d9"
        colors.text.primary: "#1a1a16"
        colors.text.secondary: "#5a584e"
        colors.text.muted: "#908d82"
        colors.border.primary: "#d4d1c7"
        colors.border.focus: "#1a4d3e"
        colors.chart.enrollment: "#1a4d3e"
        colors.chart.dropout: "#b91c1c"
        colors.chart.adverse: "#92400e"
    brand-b:
      description: "High-contrast variant — accessibility-forward"
      tokens:
        colors.background.primary: "#000000"
        colors.background.secondary: "#080808"
        colors.background.tertiary: "#141414"
        colors.text.primary: "#f0f0f0"
        colors.text.secondary: "#b8b8b8"
        colors.text.muted: "#707070"
        colors.border.primary: "#2a2a2a"
        colors.border.focus: "#3dd68c"
        colors.chart.enrollment: "#3dd68c"
        colors.chart.dropout: "#ff6b6b"
        colors.chart.adverse: "#fbbf24"

# Overview

This dashboard exists to answer three questions that determine whether a clinical trial survives its next Data Safety Monitoring Board review: how many subjects have enrolled, how many have left, and whether adverse events are trending toward a signal. Every pixel on the page must justify itself against those questions. Decoration that conceals a signal is a hazard; decoration that adds no information is waste.

The page is structured as an observational instrument. Three primary panels — enrollment funnel, dropout rate indicator, and adverse-event time series — occupy the upper field of view, because the monitoring physician's first glance must land on the most actionable data. Secondary panels — site-by-site breakdown, demographic filters, protocol deviation log — sit below the fold, accessible by scroll but never blocking the primary read.

Navigation is minimal: a trial selector and a date-range control in the header. Everything else is a filter that lives inside the panel it affects. Separating navigation from filtration is a measurable choice — it reduces the number of cognitive context switches required to interrogate a single variable.

The design language is deliberately cold. This is not a marketing surface. Color is reserved exclusively for encoding data dimensions: teal-green for enrollment, red for dropout, amber for adverse events. Gray carries structure — borders, backgrounds, labels. Any use of color outside this encoding scheme must be argued for on the basis of what it measures.

Source Sans 3 was chosen as the sole proportional family because its x-height (approximately 530 units out of 1000 cap height) provides legibility at small sizes without requiring a larger point size to compensate. For a dashboard where dozens of numerical values compete for attention, a face that reads clearly at 13px body is a measurable advantage over faces that demand 16px to achieve equivalent legibility. IBM Plex Mono carries all numerical data — counts, percentages, dates — so that tabular figures align without additional CSS intervention.

# Colors

The primary hue is a deep teal-green rooted at HSL approximately (160, 50%, 20%). This is not the conventional blue of medical software, nor the saturated green of consumer health apps. It was selected by elimination: blue carries trust associations that are irrelevant to a monitoring context (the dashboard should earn trust through accuracy, not through cultural association), and saturated green creates false-positive connotations in an adverse-event context. Teal-green sits between — sober, distinct from both, and capable of producing WCAG-AA compliant text against white at shade 500.

The color palette operates on a strict encoding principle. Enrollment uses primary-500; dropout uses danger-500; adverse events use caution-500. These three hues were chosen for maximum pairwise distance in CIELAB space — they remain distinguishable under protanopia and deuteranopia simulation, which is a measurable requirement for a clinical tool likely to be used by color-deficient physicians.

Neutral shades are chromatically keyed — they carry a faint green undertone (hue 155-165) so that gray elements feel like they belong to the same optical family as the primary. Pure neutral grays would produce an unintended warm-cool contrast against the teal that distracts from the data. The undertone is kept below 3% saturation so it never registers as "colored" — it registers as "coherent."

Background tokens are stratified into three levels. Primary background carries panels and the main canvas. Secondary background sits behind the header and persistent navigation rail. Tertiary background is reserved for inset wells — the interior of chart containers, code-style data cells, and nested tables. This three-layer system produces measurable depth without resorting to drop shadows, which are banned from this surface unless they encode a specific z-axis elevation that the user must perceive.

All four variants have been measured against WCAG-AA. Light mode: #0a130f on #ffffff yields a contrast ratio of 16.3:1. Dark mode: #ebedea on #0a130f yields 14.1:1. Brand-a: #1a1a16 on #faf9f5 yields 14.8:1. Brand-b: #f0f0f0 on #000000 yields 19.3:1. Every text-on-background pairing in every variant exceeds 4.5:1 by a wide margin because clinical data cannot afford to be borderline legible.

# Typography

The type scale is derived from a single ratio of 1.250 (major third) applied to a base of 13px. This produces the sequence: 13, 16.25, 20.31, 25.39, 31.74, 39.67. Rounding to the nearest pixel: 13, 16, 20, 25, 32, 40. Each step is exactly 1.25x the previous — no exceptions, no sizes chosen by feel. Body text is 13px because a dashboard presents dense numerical tables, and 13px Source Sans 3 with a 1.5 line-height (19.5px leading) fits approximately 120 characters per line in a five-column grid span, which is slightly beyond the 75-character optimal measure but acceptable for data surfaces where scanning matters more than sustained reading.

Headings use the same family at weight 700. The largest heading on the page (the trial name in the header) is 25px — not 32px, because 32px would consume vertical space that the adverse-event chart needs. The hierarchy is flat by intention: this is an instrument panel, not an editorial page. Three heading levels suffice: 25px for the trial name, 20px for section titles ("Enrollment," "Adverse Events"), and 16px for panel labels.

IBM Plex Mono is used for every numerical value — enrollment counts, percentages, dates, p-values. This is not an aesthetic preference. Tabular figures in a monospace face ensure that a column of three-digit numbers aligns on the ones place without CSS font-variant-numeric trickery that may not be supported across all rendering engines. The mono size matches the body size exactly (13px) so that mixed prose-and-number cells do not exhibit baseline misalignment.

The line-height scale is also derived: body at 1.5, headings at 1.2, data cells at 1.35. These are not arbitrary — 1.5 is the minimum for comfortable scanning of 13px text, and 1.2 is the maximum that prevents headings from consuming excessive vertical space. The type measure (line length) for body text is constrained to 60-70 characters by the grid. Prose blocks that exceed this measure are broken into narrower columns, because readability research consistently identifies 66 characters as the optimal line length for single-pass reading comprehension.

# Layout

The grid is 12 columns with 16px gutters on a maximum content width of 1200px. This was derived, not assumed. Body text at 13px in Source Sans 3 averages 6.5px per character. A comfortable measure of 60 characters requires 390px. Dividing 1200px by 390px gives approximately 3.08, meaning three equal text columns fit with room for gutters. Twelve columns divides cleanly by 3, 4, and 6 — providing the layout flexibility a multi-panel dashboard requires without orphan columns.

The enrollment funnel panel spans 5 columns (approximately 480px). The dropout indicator spans 3 columns (approximately 280px). The adverse-event time series spans the full 12 columns below them because time series data demands horizontal extent — compressing a 12-month adverse event curve into less than 800px reduces the resolution of each data point below the threshold where a clinician can visually distinguish a spike from noise.

Vertical rhythm follows the Fibonacci-derived spacing scale. The baseline increment is 4px. Spacing values are: 4, 8, 12, 16, 20, 24, 32, 40, 52, 64. Panel padding is 24px (6 units). Gap between major panels is 32px (8 units). Section headings sit 20px (5 units) above their content. These are not arbitrary — each value in the sequence is the sum of the two preceding values, producing a rhythm that the human visual system recognizes as proportional rather than random.

The optical center of a 1200×800 viewport sits approximately 5% above the geometric center, at roughly y=380 rather than y=400. The primary data panels are positioned so that their combined vertical center aligns with this optical center, not the geometric one. This means the enrollment funnel sits slightly higher than the page midpoint — a measurable correction that produces the subjective impression of "balance" without relying on subjective judgment to achieve it.

# Elevation & Depth

Drop shadows are banned. Every shadow on this dashboard must be argued for on the basis of what interaction state it communicates. The sole exception: a 0 1px 2px rgba(0,0,0,0.08) shadow may appear on the active date-range picker to indicate it is the element receiving input focus. This shadow is the minimum perceivable shadow at normal viewing distance — anything heavier would be decorative.

Depth is communicated through background tonal shifts. The tertiary background token (#ebedea in light mode) creates the impression of a "well" or "recessed" area — used for chart interiors and data cell backgrounds. This is sufficient to distinguish data-containing regions from structural chrome without introducing the visual noise of shadows, which interact unpredictably with dark mode inversions.

Panel borders use a 1px solid line in the primary border token color. In an information-dense dashboard, the border's job is to separate, not to decorate. Rounded corners on panels are 8px — enough to signal "container" without creating large-radius curves that waste corner pixels. Chart wells use 4px rounding. Buttons use 4px rounding. The rounding scale has two values because only two values are needed; introducing a third would require justifying what interaction state it encodes.

The header uses the secondary background token to create a single level of recession from the main canvas. No shadow, no blur, no gradient. The tonal difference between #ffffff and #f7f8f7 is measurable (a contrast ratio of approximately 1.04:1 — well below any legibility threshold, but above the just-noticeable difference for side-by-side comparison). This is the minimum visual signal required to communicate "this is a separate region," and therefore it is the correct signal.

# Shapes

All containers are rectangles with radius 8px (panels) or 4px (inset wells, buttons, inputs). Circular shapes are reserved exclusively for status indicators — a 12px circle next to a site name in the enrollment breakdown, filled with the chart color encoding that site's status (green for enrolling, amber for suspended, red for terminated). No element on this dashboard is circular unless it carries a single scalar status value, because circles cannot efficiently encode multidimensional data and their visual weight distracts from rectangular data containers.

Data points in the adverse-event time series are rendered as 6px circles. This size was measured: at 6px, a point is large enough to be individually hovered on a 1920×1080 display at arm's length, but small enough that 12 consecutive monthly points with error bars do not visually merge into a continuous band. Enrollment bars in the funnel chart are pure rectangles with no rounding, because rounded bar caps introduce a small area distortion at the bar end that becomes visible when comparing bars of similar magnitude.

The overall page shape is a single vertical scroll with no horizontal scroll. Panels reflow at breakpoints: 1200px (12 columns), 960px (8 columns), 640px (4 columns stacking). The adverse-event time series, which demands horizontal extent, collapses at 640px into a horizontally scrollable chart container with a fixed y-axis label column — because removing the scroll would compress the time axis to the point where month-to-month variation is no longer perceptible, which would defeat the panel's purpose.

Icon use is limited to Lucide's set and appears only in navigation elements and inline status markers. The only icons on the page are: a filter funnel icon next to the date-range picker, a triangle-alert icon next to adverse-event panels that exceed a threshold, and chevron icons in the trial selector dropdown. Each icon exists because it communicates a specific state or action; there are no icons used purely as visual punctuation.

# Components

The Enrollment Summary panel is a horizontal bar chart showing enrolled vs. target at each active site. Bars are 32px tall with 8px gaps. The enrolled portion fills with chart-enrollment color; the remaining target portion fills with the tertiary background color. The numeric label (e.g., "47/120") sits right-aligned at the bar's end in IBM Plex Mono 13px. This component replaces a conventional donut chart because a horizontal bar encodes both absolute count and proportion in a single glyph, whereas a donut encodes only proportion and requires a separate label for the absolute value.

The Dropout Rate Indicator is a single large percentage rendered in IBM Plex Mono 40px weight 600, with a smaller delta indicator below it showing the change from the previous reporting period. The delta is prefixed with a Lucide arrow-up or arrow-down icon and colored with chart-dropout or chart-enrollment depending on direction. This is the only element on the page that uses a display-scale number, because dropout rate is the single metric most likely to trigger a DSMB intervention, and it must be readable from two meters away.

The Adverse Event Time Series is a line chart with a filled area. The x-axis is time (monthly intervals), the y-axis is event count. A horizontal reference line at the protocol-specified threshold is drawn in chart-dropout color at 1px stroke with a dashed pattern. This reference line is the most important visual element in the component — it is the line that separates "acceptable" from "signal," and its presence turns the chart from a description into a decision tool. The line uses a dashed pattern specifically because a solid line would visually compete with the data line, and dashes communicate "threshold" rather than "data" at