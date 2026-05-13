---
name: "synth-ledger"
version: "0.1"
description: "Neon cyberpunk trading dashboard with hot magenta and electric cyan accents on deep dark surfaces"
colors:
  primary:
    "50": "#fff0fa"
    "100": "#ffd6f0"
    "200": "#ff8ed4"
    "300": "#ff47b8"
    "400": "#ff0f9c"
    "500": "#e60073"
    "600": "#b8005c"
    "700": "#8a0044"
    "800": "#5c002d"
    "900": "#2e0017"
  secondary:
    "50": "#edfffe"
    "100": "#c8fffe"
    "200": "#6effff"
    "300": "#00f5ff"
    "400": "#00c9d9"
    "500": "#009db3"
    "600": "#007a8c"
    "700": "#005766"
    "800": "#003440"
    "900": "#001a20"
  neutral:
    "0": "#ffffff"
    "50": "#f0f0f5"
    "100": "#c8c8d4"
    "200": "#9898aa"
    "300": "#6a6a7e"
    "400": "#3e3e52"
    "500": "#2a2a3c"
    "600": "#1c1c2e"
    "700": "#141424"
    "800": "#0e0e1a"
    "900": "#08080f"
    "1000": "#000000"
  success:
    "500": "#00ff88"
  danger:
    "500": "#ff3366"
  warning:
    "500": "#ffaa00"
typography:
  display:
    family: "'JetBrains Mono', 'Fira Code', 'SF Mono', monospace"
    weight: 700
  body:
    family: "'Space Mono', 'Roboto Mono', 'Courier New', monospace"
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
rounded:
  sm: "2px"
  md: "4px"
  lg: "8px"
metadata:
  variants:
    light:
      description: "Inverted light mode with muted neon accents"
      tokens:
        colors.background.primary: "#f0f0f5"
        colors.text.primary: "#141424"
    dark:
      description: "Primary dark cyberpunk mode"
      tokens:
        colors.background.primary: "#08080f"
        colors.text.primary: "#e8e8f0"
    brand-a:
      description: "Warm neon variant with amber-shifted accents"
      tokens:
        colors.background.primary: "#0f0a08"
        colors.text.primary: "#f5ece0"
    brand-b:
      description: "Cool teal variant with green-shifted accents"
      tokens:
        colors.background.primary: "#060f0d"
        colors.text.primary: "#dff5ef"
---

# Overview

Synth Ledger is a dense, information-rich trading dashboard rooted in cyberpunk aesthetics. The design balances aggressive neon energy with the readability demands of real-time financial data. Hot magenta serves as the primary accent for critical actions, alerts, and active states, while electric cyan marks secondary interactions, informational highlights, and navigational elements.

The visual language draws from terminal interfaces, electronic signage, and retro-futuristic HUDs without falling into parody. Every glow effect serves a purpose: drawing the eye to changing data, signaling urgency, or establishing spatial hierarchy in a compact layout. The monospace typography reinforces the terminal identity while ensuring perfect alignment of numerical columns and tabular data.

Four variants provide flexibility. The dark variant is the canonical experience. The light variant inverts the palette for high-ambience environments. Brand-a warms the dark surface toward amber undertones. Brand-b cools toward teal-green for an alternative cyberpunk identity.

# Colors

The color system orbits two neon poles. Magenta (e60073 at 500) carries emotional weight: it marks danger, confirmation, and primary actions. Cyan (009db3 at 500) provides analytical clarity: it marks data, navigation, and secondary elements. These two accents vibrate against the near-black neutral backgrounds without competing, because magenta always dominates as the action color and cyan always recedes as the information color.

Neutral tones are slightly cool-leaning, tinted toward blue-violet at the dark end to avoid the flatness of pure gray. This subtle chromatic bias makes the dark backgrounds feel deeper and the neon accents feel more electric by contrast. The success green (00ff88) and danger red (ff3366) are chosen to be distinguishable from both accents and from each other, even under color-vision deficiency.

On light backgrounds, neon accents are desaturated and darkened significantly to maintain readability. A full-brightness magenta on white fails WCAG and assaults the eye; the light variant uses deeper, more subdued versions of the same hue family while preserving the palette identity.

# Typography

JetBrains Mono at bold weight handles all display moments: dashboard headers, section titles, large numerical callouts, and status indicators. Its ligature support and clear digit forms make it ideal for financial data where 0/O and 1/l/I distinctions matter. Space Mono at regular weight handles body content: labels, descriptions, secondary data, and metadata.

Both fonts are true monospaces, ensuring perfect vertical alignment across columns of prices, volumes, and percentages. This is not merely an aesthetic choice but a functional requirement for a trading dashboard where misaligned decimals cause cognitive friction. The type scale is tight and modular, using the spacing grid to create a dense but scannable information hierarchy.

Line height is kept compact at 1.4 for body text and 1.1 for display numerals. In a dense dashboard, vertical space is precious. The reduced leading allows more data rows per viewport without sacrificing readability, because the monospace letterforms already provide generous inherent spacing between lines.

# Layout

The dashboard uses a twelve-column grid with a 16px gutter. The default viewport is divided into three zones: a narrow left sidebar for navigation and watchlists, a wide center area for the main chart and order book, and a right panel for positions and trade history. On smaller screens, the right panel collapses into a tabbed overlay.

Data density is a feature, not a flaw. Components are packed tightly with 8px internal padding and 4px gaps between related elements. Section dividers are achieved through background color shifts rather than whitespace, preserving pixel budget for content. The layout respects a strict 4px baseline grid, ensuring that all text, icons, and dividers align to the same invisible rhythm.

Responsive behavior is minimal by design. This is a professional tool expected to run on desktop monitors. Below 1024px width, the layout stacks vertically with a tab navigation replacing the sidebar. There is no mobile-first philosophy here; the dashboard is desktop-first and makes no apologies for it.

# Elevation & Depth

Elevation in Synth Ledger is expressed through two mechanisms: subtle background lightening and neon border glow. The base layer sits at neutral-900. Raised panels shift to neutral-800. Overlay modals and dropdowns use neutral-700. Each step up is only one token lighter than the last, maintaining the dark immersive feel while providing enough contrast to parse spatial relationships.

Active or focused elements receive a 1px border in the relevant accent color with a 4px outer glow at 20% opacity. This glow is the primary depth signal and replaces traditional drop shadows entirely. A magenta glow marks primary actions and danger states. A cyan glow marks selected items and informational focus. The glow radius never exceeds 8px to prevent visual noise in dense layouts.

The deepest elevation layer is reserved for notifications and critical alerts. These use both an accent border and a subtle animated pulse on the glow, drawing attention without demanding dismissal. The animation is a slow 2-second ease-in-out cycle to avoid triggering photosensitivity concerns.

# Shapes

All components use minimal border radius: 2px for small elements like tags and badges, 4px for cards and buttons, 8px for modals and large panels. The sharp corners reinforce the terminal aesthetic. Roundness would soften the identity and undermine the precise, engineered feeling the dashboard requires.

Icon containers and status indicators are the exception: they use perfect circles or full rounding to create visual punctuation marks in the rectilinear grid. A circular glowing dot next to a price is instantly readable as a live-status indicator precisely because it breaks the square motif.

Dividers are 1px solid lines in neutral-700, never dashed or dotted. Horizontal dividers separate table rows. Vertical dividers split paned views. The consistency of 1px solid lines creates a grid-like texture that reinforces the data-dense identity without adding visual complexity.

# Components

Buttons exist in three tiers. Primary buttons use a magenta fill with white text. Secondary buttons use a transparent background with a cyan border and cyan text. Ghost buttons use transparent background with neutral-200 text and no border. All buttons have a monospace label in uppercase at 11px tracking at 0.1em, maintaining the terminal voice.

Data tables use the body font at 12px with rows at 32px height. Alternating row backgrounds shift between transparent and neutral-800 at 50% opacity. The active or hovered row gains a left border in cyan and a background shift to neutral-700 at 30% opacity. Sortable column headers display a small caret icon in neutral-300 that changes to cyan when active.

The chart component is the visual centerpiece. It renders on a transparent background with a subtle neutral-800 grid. Price lines use cyan by default, with magenta for comparison overlays. Crosshair tooltips use a 1px dashed neutral-200 line with a floating label in neutral-700 background and neutral-50 text. Volume bars use neutral-600 with a subtle gradient toward neutral-500 at the top edge.

# Do's and Don'ts

Do use magenta sparingly for primary actions and critical alerts. Its visual weight increases with every instance, so reserve it for moments that genuinely require immediate attention. Do use cyan liberally for data highlights, active states, and navigational cues. Do maintain strict alignment across all numerical columns. Do test glow effects at 50% and 200% zoom to ensure they scale gracefully.

Don't use full-brightness neon colors on light backgrounds. The light variant requires desaturated, darkened versions of every accent. Don't combine magenta and cyan on the same element unless it is a deliberate gradient treatment on a hero moment. Don't use rounded corners beyond 8px. Don't introduce non-monospace fonts for any data element. Don't animate glow effects faster than a 1-second cycle. Don't use the success green for anything that is not a positive financial outcome, and don't use the danger red for anything that is not a loss, error, or destructive action.