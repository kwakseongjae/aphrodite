---
name: "longevity-editorial"
version: "0.1"
description: "A calm editorial site for a longevity research clinic, balancing scientific authority with human warmth"
colors:
  primary:
    "50": "#f0f4f8"
    "100": "#d9e2ec"
    "200": "#bcccdc"
    "300": "#9fb3c8"
    "400": "#829ab1"
    "500": "#627d98"
    "600": "#486581"
    "700": "#334e68"
    "800": "#243b53"
    "900": "#102a43"
  neutral:
    "0": "#ffffff"
    "50": "#fafbfc"
    "100": "#f0f2f5"
    "200": "#e0e4ea"
    "300": "#c7cdd6"
    "400": "#a4adb8"
    "500": "#7b8794"
    "600": "#5a6672"
    "700": "#3d4852"
    "800": "#252c35"
    "900": "#141a21"
    "1000": "#0a0d12"
typography:
  display:
    family: "DM Serif Display"
    weight: 400
  body:
    family: "DM Sans"
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
  sm: "4px"
  md: "8px"
  lg: "12px"
  xl: "16px"
metadata:
  variants:
    light:
      description: "Default light mode — clean, airy editorial feel"
      tokens:
        colors.background.primary: "#ffffff"
        colors.text.primary: "#102a43"
    dark:
      description: "Dark mode — deep slate for focused reading"
      tokens:
        colors.background.primary: "#0a0e17"
        colors.text.primary: "#f0f2f5"
    brand-a:
      description: "Warm editorial — parchment undertones for approachability"
      tokens:
        colors.background.primary: "#faf6f0"
        colors.text.primary: "#1e1810"
    brand-b:
      description: "Cool medical — clinical confidence with warmth"
      tokens:
        colors.background.primary: "#f2f5f9"
        colors.text.primary: "#0d1b2a"
---

# Overview

This design system serves a longevity research clinic whose audience expects both scientific rigor and a sense of optimistic calm. The visual language draws from long-form editorial publishing — generous whitespace, considered typography, and a muted steel-blue palette that signals trust without coldness. Every element is deliberate, reflecting the precision of the research itself.

The primary hue family sits in the steel-slate range, deliberately avoiding greens (which skew too "wellness"), blues (too corporate), and warm neutrals (too lifestyle). This cool grey-blue carries institutional authority while remaining approachable. DM Serif Display provides editorial gravitas for headlines without the quirky personality the user has rejected in previous display fonts. DM Sans pairs as a clean geometric-humanist hybrid that reads beautifully at body scale.

The system prioritizes reading. Longevity research involves complex ideas that patients and families must understand, so typographic comfort, high contrast, and breathing room between elements are structural decisions, not aesthetic preferences. The design gets out of the way.

Four variants accommodate different contexts: standard light and dark for web browsing, a warm parchment variant for patient-facing print or newsletter contexts, and a cool clinical variant for research publication pages.

# Colors

The primary palette is built on a steel-slate axis ranging from near-white (#f0f4f8) to deep navy-black (#102a43). The midpoint (#627d98) is a muted blue-grey that reads as authoritative without feeling institutional. This family avoids the sterility of pure blue, the casualness of green, and the user's noted aversion to olive tones.

Neutral tones share the same cool undertone, ensuring harmony across the full range. Background surfaces use the lightest primaries rather than pure grey, creating subtle depth without introducing competing hues. Text colors pull from the deepest end of the primary scale, guaranteeing strong contrast ratios across all variants.

Accent usage is minimal. When needed for interactive elements or data visualization, a single warm copper tone (#b8734b) provides complementary contrast against the cool base. This accent is used sparingly — call-to-action buttons, selected research highlights, and data chart markers — so it retains its visual impact.

Dark mode inverts the logic but maintains the same hue family. The background uses a blue-black (#0a0e17) rather than pure black, which reduces eye strain during extended reading sessions — a key consideration for users researching health decisions late at night.

# Typography

DM Serif Display handles headlines and feature pull-quotes. Its relatively low contrast between thick and thin strokes reads as modern editorial rather than traditional newspaper, and its moderate x-height keeps it dignified at large sizes without becoming overwhelming. The regular weight (400) is sufficient for all display applications, maintaining the restraint appropriate for a medical-adjacent brand.

DM Sans serves all body text, navigation, and interface elements. It shares DNA with the display face through matched proportions and similar geometric tendencies, creating natural pairing without visual monotony. Its generous apertures and open counters aid legibility at small sizes — critical for an audience that may include older adults researching longevity treatments.

Type scale follows a 1.25 ratio: 14, 17, 21, 26, 33, 41, 51, 64px. Body text defaults to 17px on desktop (slightly above average) to accommodate the reading-intensive nature of the content. Line height sits at 1.65 for body and 1.15 for display — optimized for their respective reading patterns.

Letter-spacing remains at default for body text and tightens by -0.01em for display headlines. No tracked-out uppercase navigation labels — this system avoids that pattern entirely, as it undermines the editorial warmth we are building.

# Layout

The system uses an eight-point grid with comfortable multipliers. Content width caps at 720px for reading columns (optimal for single-column editorial) and 1200px for full-width layouts. Sidebar elements occupy 280px minimum, ensuring they don't compete with primary content.

Vertical rhythm prioritizes generous spacing between content blocks. Section gaps use 64px (spacing-16), creating clear visual separation that aids scanning. Paragraph spacing within articles uses 32px (spacing-8), enough to breathe without losing cohesion. This generous rhythm reflects the unhurried, reassuring tone the clinic wants to project.

Responsive breakpoints target three reading contexts: mobile (below 768px), tablet (768-1024px), and desktop (above 1024px). On mobile, display type scales down two steps, and spacing tokens halve proportionally. The reading column expands to full width with appropriate padding.

A persistent top navigation uses a slim 56px bar with the clinic's wordmark left-aligned and navigation right-aligned. On scroll, a subtle shadow appears rather than a background color change — the nav breathes with the page rather than declaring dominance.

# Elevation & Depth

This system deliberately minimizes shadow usage. The editorial metaphor suggests flatness, and the medical context demands clarity over decoration. When elevation is needed, shadows use a single soft layer with 12-16px blur, no spread, and the primary-900 color at 8-12% opacity. This creates gentle recession rather than dramatic floating.

Three elevation levels suffice: resting (no shadow), raised (one subtle shadow for cards and callouts), and overlay (slightly stronger shadow for modals and dropdowns). Each level increases blur by 8px and opacity by 4% — barely perceptible differences that the eye registers as spatial ordering without conscious attention.

Background color differentiation supplements shadows. Raised elements can shift one step darker (or lighter in dark mode) in the neutral scale rather than relying on shadows alone. This technique works particularly well for card groups and content sections, where subtle tonal variation creates hierarchy without visual noise.

Borders use a 1px line in primary-200 (light mode) or primary-700 (dark mode). They serve as separators, not decorative elements, and appear only where needed — between list items, around form inputs, and at section boundaries.

# Shapes

Border-radius stays conservative: 4px for small elements (buttons, inputs), 8px for medium elements (cards, images), 12px for large elements (feature containers), and 16px for hero overlays. This graduated scale prevents the rounding from feeling arbitrary while maintaining a sense of precision appropriate to a scientific brand.

Circular elements are reserved for specific semantic purposes: author avatars, status indicators, and icon containers. Arbitrary circular decoration is avoided. When circles appear, they are exactly 40px or 48px in diameter, maintaining alignment with the spacing grid.

Full-bleed images use the 12px radius on exposed corners only. Images embedded in content columns use no radius — they sit flush with the text grid. This distinction creates visual variety without introducing arbitrary shape decisions.

The overall impression should feel slightly softened but never soft. The tight radii maintain a crisp, professional quality that distinguishes this clinic from consumer wellness brands that lean into roundness as a friendliness signal.

# Components

Article cards present research summaries with a thumbnail image (no radius, aligned to grid), a category label in primary-600 small caps, headline in DM Serif Display at 26px, and a two-line excerpt in DM Sans at 15px. Metadata (date, reading time) sits below in primary-400. Hover state adds the subtle raised shadow and shifts the headline color to primary-600.

Buttons follow two tiers. Primary buttons use a solid primary-700 background with white text, 8px radius, and 12px vertical padding. Secondary buttons use a transparent background with a 1px primary-300 border and primary-800 text. Both use DM Sans at 15px with medium weight, maintaining the typographic system without introducing a separate button font.

Form inputs use 1px borders in primary-200, 8px radius, and generous padding (12px horizontal, 10px vertical). Focus state shifts the border to primary-500 with a 3px outer glow in primary-100. Error states introduce a muted terracotta (#c25a3e) — the only intentional warm color outside the copper accent.

Navigation links use an underline reveal pattern: a 1px line in primary-500 grows from center on hover. Active states persist the underline and shift text to primary-700. This pattern avoids background-color hover states, which would undercut the editorial calm.

# Do's and Don'ts

Do maintain generous whitespace around text blocks. The spacing is a feature, not waste — it communicates calm deliberation and respects the reader's attention. Every cramped element undermines the clinic's brand promise of thoughtful, unhurried care.

Don't introduce new color families beyond the primary steel-slate, the copper accent, and the terracotta error state. The palette's restraint is its strength. If a visualization requires more colors, extend into different values of the primary family rather than adding hues.

Do use the display serif for headlines and the sans-serif for everything else. This clear division of labor keeps the typographic hierarchy legible. Mixing the two weights within a single component (a card with serif headline and sans body) is correct; using the serif for body text or the sans for large headlines weakens the editorial voice.

Don't animate elements gratuitously. Transitions should be functional (hover states, page transitions) and use 200-300ms easing. Decorative entrance animations, parallax scrolling, and auto-playing carousels contradict the still, contemplative quality this system builds. Motion is permitted when it serves understanding; otherwise, stillness is the default.