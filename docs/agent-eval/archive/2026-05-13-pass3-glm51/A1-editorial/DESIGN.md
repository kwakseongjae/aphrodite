---
name: "meridian-literary"
version: "0.1"
description: "A literary magazine landing page with generous serif scale, muted terracotta warmth, and single-accent restraint for long-form reading"
colors:
  primary:
    "50": "#f5ece3"
    "100": "#e8d5c0"
    "200": "#d4b08e"
    "300": "#c08b5c"
    "400": "#b07038"
    "500": "#a66430"
    "600": "#8f5228"
    "700": "#75411f"
    "800": "#5c3218"
    "900": "#3d200f"
  neutral:
    "0": "#ffffff"
    "50": "#faf9f7"
    "100": "#f2efe9"
    "200": "#e2ddd3"
    "300": "#c9c1b3"
    "400": "#a89d8c"
    "500": "#8a7e6c"
    "600": "#6b6153"
    "700": "#4a4238"
    "800": "#2d2821"
    "900": "#1a1611"
    "1000": "#000000"
typography:
  display:
    family: "Fraunces"
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
  "24": "96px"
rounded:
  sm: "2px"
  md: "4px"
  lg: "6px"
metadata:
  variants:
    light:
      description: "Warm parchment light mode with deep ink text"
      tokens:
        colors.background.primary: "#faf9f7"
        colors.text.primary: "#1a1611"
    dark:
      description: "Rich dark umber mode with warm light text"
      tokens:
        colors.background.primary: "#1a1611"
        colors.text.primary: "#f2efe9"
    brand-a:
      description: "Cream-linen variant with sepia undertone"
      tokens:
        colors.background.primary: "#f5ece3"
        colors.text.primary: "#2d2821"
    brand-b:
      description: "Cool stone variant with muted warmth"
      tokens:
        colors.background.primary: "#e2ddd3"
        colors.text.primary: "#1a1611"
---

# Overview

Meridian is a literary magazine landing page built around a singular conviction: reading deserves room. The design privileges whitespace, typographic scale, and visual calm over decorative complexity. Every element exists to escort the reader toward the text and keep them there.

The palette draws from a single muted terracotta family — baked earth rather than burnt orange — grounded in warm neutrals that recall ink on aged paper. One accent only, used with extreme restraint for interactive moments and editorial highlights. The warmth is ambient, never loud.

Fraunces serves as the display face, chosen for its optical-size responsiveness and the way its softer weighted strokes feel approachable at large scale without losing authority. Source Serif 4 handles body text, offering the crisp serifs and open apertures that sustain comfortable reading across long-form essays. Neither font is a default; both reward sustained attention.

The system supports four variants — light, dark, and two branded linen tones — each maintaining WCAG-AA contrast. Dark mode inverts the parchment relationship into dark umber and warm cream, preserving the editorial warmth without resorting to cold grays.

# Colors

The primary palette lives within a narrow terracotta range, centered on #a66430. This is not coral, not rust, not camel — it is specifically the tone of clay after firing, carrying warmth without saturation. The 50 and 100 shades serve as tinted backgrounds; the 800 and 900 shades anchor dark-mode text.

Neutrals are warm by default, pulling faint yellow-brown undertone rather than pure gray. This keeps the system cohesive: even "neutral" areas feel like they belong to the same tonal world as the accent. Pure white and pure black exist at the extremes but are rarely used at full strength.

The single-accent rule is strict. Terracotta-500 marks links, active states, and editorial callouts. Nothing else competes for chromatic attention. This discipline gives the accent meaning — when a reader sees it, they know it signals something actionable or noteworthy.

Dark mode does not simply invert. The background shifts to the deepest neutral brown rather than black, and text settles into the warmest off-white. This prevents the jarring coldness that pure dark modes introduce, maintaining the literary atmosphere across viewing conditions.

# Typography

Fraunces operates at display scale: hero headlines at 72–96px, section headings at 36–48px, pull quotes at 28–32px. Its variable weight range allows for dramatic weight shifts without font changes. At large sizes, its ink traps and optical sizing create a presence that feels carved rather than rendered.

Source Serif 4 handles everything else. Body text sits at 18–20px with 1.65 line-height — generous by web standards, necessary for literary content. Paragraphs breathe with 24px bottom margins. Measure holds at 65–75 characters per line, the upper end of comfortable reading range for serif faces.

Type hierarchy relies on weight contrast rather than color contrast. A 700-weight Fraunces headline against 400-weight Source Serif body creates clear visual separation without shouting. Italics in Source Serif 4 serve for editorial asides, citations, and subtitle treatments.

Letter-spacing tightens slightly on display text (-0.01em at large sizes) and loosens on small caps and labels (+0.04em). No tracking is applied to body text. The typographic system trusts the fonts' built-in spacing, intervening only where scale demands optical adjustment.

# Layout

The page operates on a 12-column grid, but the primary content lane spans only 7 columns, centered. This asymmetric content well creates natural breathing room and discourages the wide, scanning-unfriendly measure that full-width layouts encourage. Feature essays may expand to 9 columns for special emphasis.

Vertical rhythm follows a 4px base unit scaled through the spacing tokens. Major section breaks receive 96px of space; subsection breaks receive 48px. This creates a cadence that mirrors the pacing of a well-edited manuscript — pauses between movements, not between every breath.

The masthead and navigation are minimal. The magazine title in Fraunces 700 at 24px sits top-left, with three navigation items in Source Serif 4 at 14px tracking wide. A single terracotta underline marks the current section. No hamburger menus, no mega-navs, no distraction from the content below.

Mobile layout collapses the grid to a single column with maintained typographic scale. Display text reduces to 48px maximum; body stays at 18px. The generous spacing compresses but does not vanish — literary pacing survives the smaller viewport.

# Elevation & Depth

Elevation is nearly absent by design. This is a reading surface, not a material assembly. Cards do not float; they sit flush with their background, distinguished by subtle border or background tone shifts. The deepest shadow in the system is 0 2px 8px rgba(0,0,0,0.06) — a whisper, not a statement.

Interactive elevation is limited to one element: the current feature's card receives a barely perceptible lift on hover (0 4px 16px rgba(0,0,0,0.08)). This single moment of depth signals interactivity without breaking the restrained atmosphere.

Dark mode reduces even this minimal shadow system. Shadows become warmer (pulling from the brown neutral range) and softer, maintaining perception without the starkness of cool shadows against warm surfaces. The goal is atmospheric consistency, not technical parity.

Layering instead relies on tonal separation. A featured essay section might use the primary-50 background against the neutral-0 page, creating a content zone that feels distinct without casting shadows. This tonal architecture scales cleanly across all four variants.

# Shapes

Border radius stays small: 2px for buttons and inputs, 4px for cards and containers, 6px for modals and overlays. The literary temperament is rectilinear — rounded corners suggest friendliness that the editorial voice does not require. Sharp edges feel more like a book's geometry.

The one softening allowance is that photographs and illustrations receive the same md radius as cards. This creates a cohesive family of shapes while acknowledging that imagery benefits from slight warmth at its edges. The treatment is consistent, not arbitrary.

Interactive shapes maintain their rounding across states. Focus rings use the sm radius and the terracotta accent color, ensuring keyboard navigation is visible without introducing new form language. The shape system is deliberately simple — two radii cover 95% of needs.

Full-width dividers are the primary structural shape, rendered in neutral-200 (light) or neutral-800 (dark) at 1px. These horizontal rules serve as section punctuation, replacing the need for varied background colors or elevation to separate content zones.

# Components

The Article Card is the workhorse component: a flush rectangle containing a photograph (16:10 ratio, md radius), a category label in Source Serif 4 at 11px uppercase with wide tracking, a Fraunces 700 headline at 24px, a two-line excerpt in Source Serif 4 at 16px, and the author name in primary-500. No shadow, no hover animation beyond the subtle lift.

The Pull Quote component breaks the content well wider, spanning 9 columns against the standard 7. Fraunces italic at 32px, left-aligned with a 3px terracotta left border. Attribution sits below in Source Serif 4 regular at 14px. This component exists to create visual rhythm within long-form text.

The Subscribe Block is the sole moment of chromatic intensity. A primary-100 background with Fraunces 700 headline at 28px, a single sentence of body copy, and an email input with a terracotta-500 submit button. It appears once per page, mid-scroll, and does not follow the reader.

Navigation is rendered as a simple horizontal list with text labels. No icons, no dropdowns. The mobile view collapses to a slide-in panel with the same text labels, now stacked vertically with generous tap targets. Terracotta-500 marks the active state via an underline offset 2px below the text baseline.

# Do's and Don'ts

Do maintain the single-accent discipline. Terracotta marks meaning — links, active states, editorial emphasis. If a second color feels necessary, the solution is almost always to reduce visual noise rather than add chromatic variety.

Do set body text at 18px minimum with 1.6+ line-height. Literary content fails at standard web typography sizes. The audience reads here; the type must reward that commitment. Test with real essays, not placeholder paragraphs.

Don't introduce decorative illustrations, patterns, or background textures. The visual identity is typographic and tonal. Ornamentation dilutes the editorial authority the design is built to project. If the page feels empty, add better whitespace cadence, not decoration.

Don't use the terracotta accent as a background for white text at small sizes. The 500 shade does not maintain AA contrast against white below 24px bold. For small interactive elements, use terracotta as a border or icon color instead, pairing it with the appropriate neutral text color.

Do test all four variants against real content with varied image tonality. A photograph with strong warm tones may require a neutral-0 background behind the caption to maintain text legibility. The system's restraint provides room for these contextual adjustments without breaking consistency.