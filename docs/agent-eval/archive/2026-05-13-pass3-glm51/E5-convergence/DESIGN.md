---
name: "Aevum Editorial"
version: "0.1"
description: "Calm scholarly editorial presence for a longevity research clinic, warm stone grounds with desaturated celadon accents and classic serif typography."
colors:
  primary:
    "50": "#eef4f1"
    "100": "#c5d9ce"
    "200": "#9dbfab"
    "300": "#74a58e"
    "400": "#5c9379"
    "500": "#448264"
    "600": "#3a7060"
    "700": "#2e5a4d"
    "800": "#23443b"
    "900": "#172e28"
  neutral:
    "0": "#ffffff"
    "50": "#faf6f1"
    "100": "#f0ebe4"
    "200": "#ddd7ce"
    "300": "#c4bcae"
    "400": "#a89e8f"
    "500": "#8a8071"
    "600": "#6b6357"
    "700": "#4e4940"
    "800": "#332f29"
    "900": "#1a1714"
    "1000": "#000000"
typography:
  display:
    family: "Newsreader"
    weight: 700
  body:
    family: "Source Serif 4"
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
  lg: "6px"
metadata:
  variants:
    light:
      description: "Default warm light mode — parchment ground with deep carbon text"
      tokens:
        colors.background.primary: "#faf6f1"
        colors.text.primary: "#1a1714"
    dark:
      description: "Deep espresso dark mode — near-black stone ground with warm ivory text"
      tokens:
        colors.background.primary: "#1a1714"
        colors.text.primary: "#f0ebe4"
    brand-a:
      description: "Warmer ochre variant — sunned linen ground with umber text"
      tokens:
        colors.background.primary: "#f5f0e8"
        colors.text.primary: "#241d10"
    brand-b:
      description: "Sage-tinted variant — pale celadon ground with deep olive text"
      tokens:
        colors.background.primary: "#f2f5f0"
        colors.text.primary: "#141a12"
---

# Overview

Aevum Editorial is designed for a longevity research clinic that wants to communicate rigor, calm authority, and quiet optimism. The system avoids clinical coldness in favor of warmth — the feeling of a well-appointed reading room rather than a sterile corridor. Every surface and type choice is measured against the goal of sustained, comfortable reading.

The palette is built around desaturated celadon greens — not sage, not teal, but a midway hue that reads as botanical without feeling decorative. These greens are restrained enough to serve as structural accents on a predominantly warm-neutral canvas of stone, parchment, and ivory.

Typography leads with Newsreader for headlines, a high-contrast optical-size serif with enough personality to feel editorial without crossing into novelty. Body text uses Source Serif 4, a workhorse text face with excellent legibility at reading sizes and a quiet warmth that matches the palette.

# Colors

The primary celadon family runs from a misty pale tint at shade 50 to a deep forest at shade 900. The mid tones (300–500) carry most of the accent work — links, active states, subtle highlights — while the darkest shades anchor hover states and focus rings. The lightest shade tints backgrounds where a faint green wash is needed without competing with the warm neutrals.

Neutral colors are intentionally warm. Even the near-white tones carry a faint cream undertone, and the dark tones lean slightly warm — closer to charcoal or espresso than pure black. This prevents the jarring coldness that pure grays can introduce in an otherwise warm system.

The interaction between primary and neutral is the system's key dynamic. Primary accents should feel like a single living plant in a room of natural wood and linen — present, alive, but never dominant. The ratio of neutral to primary on any given screen should be approximately ten to one.

# Typography

Newsreader at weight 700 provides headlines with the gravitas of a journal masthead. Its optical sizing means it remains crisp and authoritative at large display sizes without feeling heavy or overwrought. Headlines should be set with generous letter-spacing at very large sizes, tightening to default at subhead scales.

Source Serif 4 at weight 400 handles body text, navigation labels, captions, and metadata. Its x-height is well-suited to reading sizes between 15 and 18 pixels. For editorial long-form, 17 pixels with 1.65 line-height provides the comfortable sustained reading experience the content demands.

A single italic variant of Source Serif 4 may be used for pull quotes, publication titles, or light emphasis. Bold should be used sparingly — the typographic hierarchy relies on size and weight contrast between display and body faces, not within the body scale.

# Layout

The layout follows a centered single-column editorial spine, 680 pixels wide for body text, expanding to 1120 pixels for full-width compositions and image essays. Generous margins — 64 pixels at large breakpoints — give the content room to breathe and reinforce the unhurried tone.

Vertical rhythm is built on an 8-pixel baseline grid. Headlines, body text, rules, and image heights all snap to this grid. Spacing between sections defaults to 48 pixels (spacing-12), creating clear editorial separation without fragmenting the reading flow.

Responsive behavior is deliberately simple. Below 768 pixels, the column shrinks to fill available width with 24 pixels of horizontal padding. Navigation collapses to a minimal state. Images go full-bleed. The reading experience should feel identical in character, just tighter.

# Elevation & Depth

Elevation in this system is subtle — almost invisible. There are only two levels: resting (no shadow) and lifted (a single soft shadow offset 2 pixels vertically with 8 pixels of blur at 8 percent opacity in a warm gray). This is sufficient to distinguish cards or modals from the background without creating a layered app aesthetic.

Depth comes primarily from background value shifts rather than shadows. A section with a slightly darker ground (neutral-100 on neutral-50) reads as recessed. A card with a slightly lighter ground reads as raised. This approach reinforces the material, physical quality of the surfaces.

Shadows should never be colored — even with the primary accent. They remain warm neutral, consistent with the system's preference for light and surface over artifice.

# Shapes

Border radii are minimal and consistent. The small radius (2 pixels) takes the sharp edge off buttons and inputs without being perceptibly round. The medium radius (4 pixels) softens cards and containers. The large radius (6 pixels) is reserved for modal overlays and featured image containers.

Roundness beyond 6 pixels is avoided. The editorial character demands rectilinear precision — the feeling of set type in a fixed grid. Circular or pill-shaped elements would undermine this tone.

Icon containers, avatars, and similar small elements use the small radius. They should feel like tokens set into the page rather than floating shapes.

# Components

Buttons follow a clear hierarchy: primary buttons use the primary-500 background with neutral-0 text, secondary buttons use a transparent background with primary-700 text and a 1-pixel primary-200 border, and ghost buttons use transparent background with neutral-700 text and no border. All buttons have 8 pixels of vertical padding and 24 pixels of horizontal padding, with the small radius.

Cards are simple bordered rectangles (1 pixel, neutral-200) with neutral-0 backgrounds, 24 pixels of internal padding, and the medium radius. Card elevation on hover is optional — a subtle shift to the lifted shadow level is acceptable if the interaction calls for it.

Navigation is horizontal, left-aligned, using body typography at weight 400 with neutral-600 text. Active or hovered items shift to neutral-900 with a 2-pixel primary-400 underline offset 4 pixels below the baseline. The nav should feel like a table of contents, not a toolbar.

# Do's and Don'ts

Do maintain high contrast between text and background — the warm neutrals must never compromise readability. Do use primary accents sparingly and intentionally — a single link color, a focused input border, a section rule. Do let whitespace do the work of separating ideas. Do treat every page as a published piece with clear hierarchy.

Don't introduce additional hue families — no blue, no red, no orange. If warmth is needed, shift within the neutral scale. Don't use primary-500 or darker as a background behind light text at large scale — it is an accent, not a surface. Don't animate shadows or border radii. Don't use decorative type treatments — no all-caps display, no stretched tracking on body text, no gradient fills. Don't add illustrations or icons where a well-set word would serve.