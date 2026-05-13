---
name: "synth-ledger"
version: "0.1"
description: "High-density cyberpunk trading dashboard with neon accents on deep dark surfaces"
colors:
  primary:
    "50":  "#e6fff6"
    "100": "#b3ffe3"
    "200": "#66ffc9"
    "300": "#1affb0"
    "400": "#00e696"
    "500": "#00d4aa"
    "600": "#00b08c"
    "700": "#008d6f"
    "800": "#006a52"
    "900": "#004736"
  accent:
    "50":  "#fff0fb"
    "100": "#ffd6f0"
    "200": "#ffadd6"
    "300": "#ff85bf"
    "400": "#ff5ca7"
    "500": "#ff3390"
    "600": "#e6005c"
    "700": "#b30047"
    "800": "#800033"
    "900": "#4d001f"
  neon-cyan:
    "50":  "#e6ffff"
    "200": "#80ffee"
    "400": "#00ffd5"
    "500": "#00e6bf"
    "700": "#00997d"
  neon-magenta:
    "50":  "#ffe6ff"
    "200": "#ff80e8"
    "400": "#ff00c8"
    "500": "#e600b4"
    "700": "#99007a"
  neutral:
    "0":    "#ffffff"
    "50":   "#e8e8ec"
    "100":  "#c4c4cc"
    "200":  "#9d9daa"
    "300":  "#767688"
    "400":  "#56566a"
    "500":  "#38384a"
    "600":  "#2a2a3a"
    "700":  "#1e1e2c"
    "800":  "#141422"
    "900":  "#0c0c16"
    "1000": "#000000"
  surface:
    "raised": "#1a1a2e"
    "overlay": "#12122088"
    "glass":  "#ffffff0a"
typography:
  display:
    family: "IBM Plex Mono"
    weight: 600
  body:
    family: "IBM Plex Mono"
    weight: 400
  data:
    family: "JetBrains Mono"
    weight: 400
  mono:
    family: "JetBrains Mono"
    weight: 500
spacing:
  "0": "0px"
  "1": "4px"
  "2": "8px"
  "3": "12px"
  "4": "16px"
  "5": "20px"
  "6": "24px"
  "8": "32px"
  "10": "40px"
  "12": "48px"
  "16": "64px"
rounded:
  none: "0px"
  xs: "2px"
  sm: "4px"
  md: "6px"
  lg: "8px"
metadata:
  variants:
    light:
      description: "Daylight terminal with muted neon signals on pale surfaces"
      tokens:
        colors.background.primary: "#f2f2f5"
        colors.background.secondary: "#ffffff"
        colors.text.primary: "#1a1a2e"
        colors.text.secondary: "#56566a"
        colors.border.default: "#c4c4cc"
    dark:
      description: "Deep void primary theme — neon glows emerge from darkness"
      tokens:
        colors.background.primary: "#0c0c16"
        colors.background.secondary: "#141422"
        colors.text.primary: "#e8e8ec"
        colors.text.secondary: "#9d9daa"
        colors.border.default: "#2a2a3a"
    brand-a:
      description: "Toxic green variant — mutant market energy"
      tokens:
        colors.background.primary: "#0a100c"
        colors.background.secondary: "#121a14"
        colors.text.primary: "#d4ffe3"
        colors.text.secondary: "#7aaa8c"
        colors.border.default: "#1e2e22"
    brand-b:
      description: "Plasma pink variant — high-voltage trading floor"
      tokens:
        colors.background.primary: "#100a0e"
        colors.background.secondary: "#1a1218"
        colors.text.primary: "#ffd6f0"
        colors.text.secondary: "#aa7a99"
        colors.border.default: "#2e1e28"

---

# Overview

Synth Ledger is a high-density trading dashboard that channels the visual language of cyberpunk neon signage and terminal interfaces into a functional data environment. The system treats information density as a feature, not a liability — every pixel earns its place on screen.

The design avoids the typical blue-purple cyberpunk cliché. Instead, the primary palette orbits a sharp mint-teal green, paired with a hot coral-pink accent. Neon cyan and magenta exist as signal colors for alerts, gains, and losses — they glow, but they never dominate the structural palette.

Typography is monospace-first. IBM Plex Mono carries the brand voice and navigation, while JetBrains Mono handles the raw data cells where numeric alignment is critical. The result feels like a machine that respects its operator.

# Colors

The primary mint-teal (#00d4aa) provides a cool, clinical anchor that reads as financial without falling into banking blue. Its darker shades recede into surfaces; its brighter shades highlight interactive elements and active states. The accent coral-pink (#ff3390) marks calls to action, critical thresholds, and entry points.

Neon cyan (#00ffd5) signals positive movement — gains, executions, confirmations. Neon magenta (#ff00c8) signals negative movement — losses, cancellations, warnings. These two colors are reserved exclusively for data semantics and never appear on structural UI elements.

The neutral scale runs cold with a subtle purple undertone, ensuring that grays never feel muddy against the neon accents. Surface colors include raised, overlay, and glass tokens for the layered panel system the dashboard requires.

# Typography

IBM Plex Mono at weight 600 serves as the display typeface for section headers, navigation labels, and brand moments. Its geometric clarity reads as technical authority without the self-conscious quirk of more decorative monospaces. Body text at weight 400 handles descriptions, tooltips, and secondary information.

JetBrains Mono is the data workhorse. Every number, price, percentage, and timestamp uses this face. Its consistent character width and distinctive slashed zeros prevent misreading across rapid scanning. The weight stays at 400 for data cells and steps up to 500 for totals and summaries.

Type scale is tight and modular. The dashboard uses a 12px base for dense table cells, 14px for body content, and scales up through 18px, 24px, and 32px for headings. Line height stays at 1.4 across all sizes to maintain the compressed vertical rhythm.

# Layout

The dashboard uses a CSS Grid backbone with named regions: nav-rail, ticker-strip, main-chart, order-book, trade-feed, and position-summary. The grid collapses intelligently across breakpoints, prioritizing the chart and order book on smaller viewports.

Spacing tokens follow a 4px base unit. Dense data regions use the 1–2 range (4–8px). Section gaps use the 4–6 range (16–24px). Generous breathing room appears only at the outer margins and between major dashboard zones, using the 8–12 range (32–48px).

Panels within the grid use a consistent internal layout pattern: a header bar with label and action icons, a content area that fills remaining space, and an optional footer for pagination or totals. This predictability lets users build muscle memory for where information lives.

# Elevation & Depth

The elevation system has five levels, each expressed through a combination of background lightening and border treatment. Level 0 is the base canvas. Level 1 is the default panel. Level 2 is a raised panel. Level 3 is a popover or dropdown. Level 4 is a modal dialog.

Borders do double duty as both edges and glow channels. A default panel uses a 1px border in the neutral-600 range. Hovered or focused panels shift the border to a dim neon tint, creating the impression of a surface charging with energy without the performance cost of box-shadow glow effects.

Glass surfaces use a semi-transparent background token combined with a subtle backdrop blur. These appear exclusively on overlay panels and tooltips that float above the primary data layer. They maintain readability while preserving the sense of layered depth.

# Shapes

Border radius tokens are intentionally restrained. The default panel radius is 6px — enough to feel modern without sacrificing the hard-edged terminal aesthetic. The small radius (4px) applies to buttons, inputs, and badges. The large radius (8px) is reserved for modals and major overlay surfaces.

There is no circle token in this system. Status indicators use 2px rounded squares instead of dots. Avatar-like elements use the sm radius. This decision reinforces the manufactured, precise quality of the interface and avoids organic softness.

The sharp edges also serve a practical purpose: in a dense grid of panels, minimal rounding maximizes usable space at panel corners where data cells push to their boundaries. Every rounded pixel is a pixel of lost content area.

# Components

The Button component comes in four variants: primary (mint fill), accent (coral fill), ghost (transparent with border), and signal (neon cyan or magenta for trade execution). All buttons use the sm radius and horizontal padding of 12px, with a minimum touch target of 32px height.

DataCell is the atomic unit of the dashboard. It accepts a value, a change indicator (positive, negative, neutral), and an optional sparkline. The cell renders the numeric value in JetBrains Mono right-aligned, with the change indicator as a small colored bar on the left edge.

Panel is the primary container. It accepts a title, a variant (default, raised, glass), and a status (idle, active, alert). The panel header includes a colored dot indicator and optional action icons. The body is a flex column that accepts any content. Panels compose together to form the dashboard grid.

# Do's and Don'ts

Do maintain strict semantic use of neon cyan and magenta. These colors carry specific meaning in trading contexts — cyan for gains and buy signals, magenta for losses and sell signals. Using them decoratively breaks the information scent trust that operators rely on.

Do use the monospace typefaces at their intended roles. IBM Plex Mono for labels and navigation, JetBrains Mono for data. Mixing them reversely creates cognitive dissonance and slows scanning speed during high-pressure trading moments.

Don't add glow effects via box-shadows on every element. Selective glow on the currently active or hovered panel is effective. Ubiquitous glow is noise. The dashboard should feel like a precision instrument, not a disco.

Don't sacrifice contrast for atmosphere. Every variant has been verified at WCAG-AA contrast ratios. If a design exploration dips below 4.5:1 between text and background, it is rejected regardless of how well it photographs. Functional readability always wins over aesthetic drama.