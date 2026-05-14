---
name: "longevity-editorial"
version: "0.1"
description: "Calm editorial design system for a longevity research clinic"
colors:
  primary:
    "50": "#edf5f4"
    "100": "#d1e8e5"
    "200": "#a3d1cb"
    "300": "#75bab2"
    "400": "#47a398"
    "500": "#298c80"
    "600": "#217067"
    "700": "#19554e"
    "800": "#113935"
    "900": "#091e1c"
  accent:
    "50": "#eef4f8"
    "100": "#d0e3ee"
    "200": "#a1c7dd"
    "300": "#72abcc"
    "400": "#438fbb"
    "500": "#2d7da8"
    "600": "#246486"
    "700": "#1b4b65"
    "800": "#123243"
    "900": "#091922"
  neutral:
    "0": "#ffffff"
    "50": "#f7f8f6"
    "100": "#ebede8"
    "200": "#d6d9d2"
    "300": "#b8bcb2"
    "400": "#94988b"
    "500": "#747868"
    "600": "#5c6052"
    "700": "#46493d"
    "800": "#30322a"
    "900": "#1c1e18"
    "1000": "#000000"
typography:
  display:
    family: "DM Sans"
    weight: 700
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
metadata:
  variants:
    light:
      description: "Default light mode — warm parchment over deep forest"
      tokens:
        colors.background.primary: "#f7f8f6"
        colors.text.primary: "#1c1e18"
    dark:
      description: "Dark mode — deep botanical night"
      tokens:
        colors.background.primary: "#0e110d"
        colors.text.primary: "#ebede8"
    brand-a:
      description: "Ocean morning variant — cool luminous calm"
      tokens:
        colors.background.primary: "#eef4f8"
        colors.text.primary: "#091922"
    brand-b:
      description: "Forest floor variant — dense canopy serenity"
      tokens:
        colors.background.primary: "#edf5f4"
        colors.text.primary: "#091e1c"
---

# Overview

This design system serves a longevity research clinic with an editorial voice that must feel authoritative yet approachable. The clinical subject matter demands visual calm — readers may be anxious about aging, mortality, or complex medical decisions. Every design choice prioritizes legibility, spatial generosity, and a sense of measured confidence. The palette draws from deep oceanic teal rather than the warmer greens or blues one might expect, avoiding any association with urgency or sterility.

The typographic strategy uses DM Sans exclusively, exploiting its optical sizing range to create hierarchy through weight and scale alone. This single-family approach reinforces the editorial seriousness — a research publication should feel like a unified document, not a collage of decorative choices. Dense information layouts are supported through strict vertical rhythm and generous line-height.

The system is built for long-form reading. Research summaries, clinical trial descriptions, and explanatory articles form the core content. Components are designed to break text into scannable, digestible units without fragmenting the narrative flow. Interactive elements remain subtle; the emphasis stays on content.

Four variants accommodate different contexts and reader preferences. Light serves daytime browsing. Dark reduces eye strain for extended reading sessions. The two brand variants offer tonal flexibility — one cooler and more clinical, one warmer and more organic — without departing from the core system.

# Colors

The primary palette is anchored in a deep teal that reads as neither blue nor green, occupying an ambiguous middle ground that feels scientific without being cold. The 500-level shade sits at a comfortably saturated midpoint, while the 900-level approaches black for use in text and critical UI elements. The lightest shades provide cool-toned neutral backgrounds that subtly reinforce the brand without overwhelming content.

The accent palette shifts slightly toward blue, offering a complementary counterpoint for interactive elements, data visualizations, and secondary emphasis. This prevents the teal from becoming monotonous while maintaining chromatic harmony. Both palettes share similar saturation curves, ensuring neither dominates inappropriately.

Neutral tones are deliberately warm — a faint green undertone runs through the grays, connecting them to the primary palette. Pure grays would feel disconnected and clinical. This warmth is subtle enough to read as neutral at a glance but creates cohesion across the full interface. The darkest neutral shades carry enough warmth to maintain the organic sensibility even at maximum contrast.

Color application follows a content-first principle. Backgrounds stay quiet. Text stays dark. Interactive elements earn their saturation through user attention. No surface should compete with the information it carries.

# Typography

DM Sans handles every typographic role from page titles to caption text. Its geometric structure provides the precision expected in scientific contexts, while its humanist touches — slightly rounded terminals, open apertures — keep the tone approachable. The 700-weight display variant creates strong hierarchy without shouting, and the 400-weight body text maintains excellent readability at standard paragraph sizes.

Headline sizes follow a modular scale with a 1.25 ratio, creating clear visual steps between hierarchy levels without excessive size jumps. Body text defaults to 17px with 1.65 line-height, optimized for the sustained reading sessions typical of research-oriented audiences. Line length is constrained to 660px maximum.

Spacing between typographic elements follows a consistent baseline grid of 8px. Paragraphs receive generous bottom margin. Lists, blockquotes, and asides maintain the same rhythm, preventing the visual fragmentation that plagues many editorial layouts. This discipline is especially important in dense information displays where multiple content types coexist.

The typographic mood is deliberately restrained — confident, not decorative. This is a reading interface, not a marketing page. Every typographic choice serves comprehension and retention.

# Layout

The layout system operates on a 12-column grid with generous gutters, supporting both long-form editorial content and structured data displays. The default content column spans 8 columns at desktop width, with the remaining space providing breathing room. This asymmetry creates visual tension that keeps the page from feeling static.

Vertical spacing follows the defined spacing tokens strictly. Sections receive 64px separation. Subsections receive 32px. Related elements sit within 16px or 24px clusters. This creates a clear visual hierarchy that readers parse instinctively — the spacing itself communicates structure before any content is read.

For dense information displays — research summaries, protocol descriptions, data tables — the grid supports multi-column layouts that maximize horizontal space without sacrificing readability. Cards and data cells maintain consistent internal padding and alignment, creating a regular visual cadence.

Mobile layouts collapse to a single column with maintained vertical rhythm. Nothing is lost — only reordered. The editorial reading experience must remain intact at every viewport.

# Elevation & Depth

Elevation in this system is minimal and purposeful. The content must feel grounded and trustworthy — excessive shadow and layering undermine the clinical authority the site needs. Three elevation levels suffice: flush surfaces for primary content, a subtle raised level for cards and interactive elements, and a modal level for overlays.

Shadows are cool-toned, using the blue-accent palette at very low opacity. This prevents the warm visual noise that default black shadows create. Even at the modal level, shadows remain tight and diffuse — never dramatic. Depth should be perceived, not noticed.

Background color shifts handle most visual separation. Slightly different neutral tones distinguish sections more quietly than borders or shadows. When borders are necessary, they use the 200-level neutral at one pixel width.

The overall effect is restrained physicality — surfaces exist in a calm, readable space without competing for attention. The page is a container for knowledge, not a demonstration of visual technique.

# Shapes

Corner radii are intentionally tight: 4px for small elements like buttons and tags, 8px for cards and containers, 12px for modal surfaces and hero elements. This precision reinforces the clinical, research-oriented identity. Softer radii would introduce informality; sharper corners would feel aggressive.

Buttons and interactive elements use the small radius consistently, creating a compact, efficient visual language. The slight rounding prevents harshness without softening the overall tone. Input fields match button radius for visual consistency.

Card containers use the medium radius, providing enough curve to feel modern without becoming decorative. Cards are tools for organizing information, not visual features themselves. The radius serves usability — distinguishing card boundaries — rather than aesthetic expression.

Shapes throughout the system are functional. Every curve serves perception or interaction. Decorative shapes are avoided entirely.

# Components

Core components are designed for an editorial context: article headers, text blocks, pull quotes, data callouts, citation cards, research summaries, author bios, and navigation elements. Each component maintains strict internal spacing and typographic consistency, ensuring they compose predictably in any arrangement.

Interactive components — buttons, links, form controls, toggles — use the accent palette to signal affordance without shouting. Hover states deepen slightly. Active states provide clear feedback. Focus states meet accessibility requirements with visible outlines using the primary 500 shade. All interactive elements maintain a minimum 44px touch target.

Data display components — statistics, metrics, comparison tables — prioritize density and scannability. Numbers are set in the display weight at generous sizes. Labels sit above values in body weight. Supporting context appears in muted tones. This hierarchy guides the eye from headline to detail without requiring conscious effort.

Every component includes specifications for all four variants. The system has no light-only components.

# Do's and Don'ts

Do maintain generous whitespace around text blocks. Longevity research demands sustained attention — visual crowding increases cognitive load and reduces comprehension. Let the content breathe.

Do use the full tonal range of the primary palette for data visualization and status indicators. The ten shades provide sufficient variety for most information displays without introducing new hue families.

Don't introduce warm accent colors — no coral, amber, or gold highlights. The user's taste profile explicitly rejects warm-orange tones, and they would clash with the cool teal foundation. Use the blue accent palette for emphasis.

Don't use Fraunces or any serif display face. The editorial identity is built on DM Sans's geometric clarity. Serif alternatives would fracture the typographic system and introduce decorative noise that contradicts the research-focused tone.

Don't exceed three levels of visual hierarchy on any single screen. If more distinction is needed, reconsider the content structure rather than adding typographic variations. Dense information requires disciplined restraint, not more formatting.

Don't sacrifice readability for visual interest. Every background color, spacing decision, and typographic choice must justify itself against the primary goal: making complex medical information accessible to a general audience.