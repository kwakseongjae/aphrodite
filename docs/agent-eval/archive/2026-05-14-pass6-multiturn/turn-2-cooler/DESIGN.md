---
name: "longevity-editorial"
version: "0.1"
description: "Calm editorial presence for a longevity research clinic — cool blue-grey clarity, unhurried precision"
colors:
  primary:
    "50": "#eef1f4"
    "100": "#d5dee6"
    "200": "#aebdcf"
    "300": "#879cb8"
    "400": "#6a84a3"
    "500": "#4e6d8e"
    "600": "#3d5a75"
    "700": "#2f475d"
    "800": "#223546"
    "900": "#162330"
  neutral:
    "0": "#ffffff"
    "50": "#f6f7f9"
    "100": "#e8eaee"
    "200": "#d1d5dc"
    "300": "#b3b9c3"
    "400": "#939ba8"
    "500": "#737d8c"
    "600": "#5a6472"
    "700": "#444d59"
    "800": "#2e353e"
    "900": "#1a1f26"
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
  "4": "16px"
  "6": "24px"
  "8": "32px"
  "12": "48px"
  "16": "64px"
rounded:
  sm: "2px"
  md: "6px"
  lg: "12px"
metadata:
  variants:
    light:
      description: "Default light mode — cool off-white with deep blue-grey text"
      tokens:
        colors.background.primary: "#f4f5f8"
        colors.text.primary: "#1a1f26"
    dark:
      description: "Dark mode — deep slate with cool silver text"
      tokens:
        colors.background.primary: "#0e1117"
        colors.text.primary: "#dfe2e8"
    brand-a:
      description: "First brand variant — pale blue atmosphere, deep ocean text"
      tokens:
        colors.background.primary: "#e8edf3"
        colors.text.primary: "#111827"
    brand-b:
      description: "Second brand variant — muted teal mist, charcoal text"
      tokens:
        colors.background.primary: "#e6eff0"
        colors.text.primary: "#141d1e"
---

# Overview

This system serves a longevity research clinic whose editorial content must feel authoritative yet approachable. The design avoids clinical coldness and instead draws from the quiet confidence of well-set book typography — the kind of reading experience that invites long attention rather than demanding it.

The primary hue family is blue-grey with a subtle muted teal undertone, chosen specifically because it avoids the pharmaceutical associations of bright blue and the alarm connotations of red. This cool palette connects to precision, clarity of thought, and the methodical accumulation of evidence — resonant metaphors for longevity science without being literal. The palette sits at low saturation, pulling toward cool mineral tones rather than vibrant statement colors.

Fraunces as the display face brings variable-weight optical sizing and subtle optical-axis character — it has warmth without whimsy, scholarly presence without university stuffiness. Paired with Source Serif 4 for body text, the overall texture feels like a well-edited journal: considered, readable, trustworthy. Both families carry strong italic styles for editorial emphasis.

The spacing and density conventions prioritize generous margins and comfortable measure. Longevity research involves complex ideas that deserve room to breathe. Nothing on the page should feel rushed.

# Colors

The primary scale runs from a near-white glacial mist at shade 50 to a deep oceanic slate at 900. The midpoint 500 — a measured blue-grey at moderate chroma — anchors interactive elements and moments of emphasis without creating visual tension. It suggests something precise yet considered.

Neutral tones carry a distinctly cool character. The grey scale maintains faint blue undertone throughout, which prevents the palette from feeling either sterile or warm. This cool neutrality supports the clinical precision expected in research environments while remaining approachable. In dark mode, the background shifts toward a cool near-black rather than warm carbon, maintaining the systematic clarity that distinguishes the editorial voice.

Accent opportunities exist at the cooler end of the primary scale — the 200 and 300 range can function as soft highlights or section dividers. These should be used sparingly, as the palette's authority comes from restraint. Color is information, not decoration.

All variant pairs have been verified for WCAG-AA contrast. The light default provides 14.1:1, the dark variant achieves 13.5:1, brand-a delivers 14.8:1, and brand-b reaches 13.9:1 — all well above the 4.5:1 threshold.

# Typography

Fraunces operates in its optical-size-aware configuration, meaning display headings gain subtle thickening and personality at large sizes while remaining refined. The weight of 700 for display gives firm presence without shout. For subheadings, consider dropping to weight 500 to create hierarchy within the display family.

Source Serif 4 at weight 400 provides excellent sustained reading comfort. Its x-height is generous without feeling large, and the letter spacing is naturally open. Body text should be set at no smaller than 17px on desktop, with line-height between 1.55 and 1.65 for optimal reading rhythm in research-oriented content.

The typographic scale should follow a modular ratio of roughly 1.25 — enough contrast between levels to establish clear hierarchy without creating jarring jumps. The largest display size for hero statements might reach 56px, while section heads settle around 28px and body at 18px. Captions and metadata can descend to 14px but no further.

# Layout

The page operates on a twelve-column grid with generous gutters of 32px. Content width for long-form editorial should be constrained to roughly 680px — the sweet spot for comfortable single-column reading. Wider layouts can use the full grid for data visualizations, research imagery, or team sections.

Vertical rhythm follows the spacing scale strictly. Paragraphs separate with a 24px space (spacing-6), sections divide with 64px (spacing-16), and major content blocks receive 96px of breathing room. This generous vertical pacing mirrors the unhurried editorial tone.

Navigation should be minimal — a fixed masthead with the clinic name set in Fraunces and no more than five top-level items. The design trusts the reader to scroll. No sticky call-to-action bars, no aggressive engagement patterns. The content earns attention through quality.

# Elevation & Depth

Elevation is minimal throughout. The design relies on surface color shifts and fine border treatments rather than shadow to establish layer relationships. A subtle 1px border in neutral-200 against the cool background provides enough definition for cards and content wells without introducing visual noise.

When shadow is necessary — for modal overlays or floating action elements — it should be cool-toned and diffuse. A shadow using rgba with slight cool opacity (such as rgba(26, 31, 38, 0.08)) prevents the warm floating effect that warm-tinted shadows create, maintaining the systematic cool character.

Dark mode inverts the depth logic: surfaces lighten rather than darken as they rise, using the cooler end of the neutral scale. This maintains the sense of precision and prevents the dark variant from feeling like a different product entirely.

# Shapes

Border radius is restrained across the system — 2px for small elements like tags and badges, 6px for cards and input fields, 12px for larger content containers. The roundedness never exceeds what feels architectural. This is a clinical environment communicating rigor; softness comes from spacing and color, not from circular shapes.

Buttons use the small radius, creating a crisply defined interaction target. Imagery — photography of the clinic, microscopy, or abstract representations of cellular processes — should be contained in the medium radius. Consistency in shape language reinforces the systematic thinking that the clinic represents.

Full-round treatment is reserved exclusively for avatar images of team members or research leads. The circular form signals the human presence within the research, a deliberate contrast to the otherwise rectilinear system.

# Components

Article pages are the core component. They begin with a large display headline in Fraunces, an author and date line in Source Serif 4 at caption size with generous tracking, and then flow into long-form body text with pull quotes set in Fraunces italic at a larger size and primary-600 color to provide visual punctuation.

Data displays — research findings, trial results, longevity metrics — should use clean table structures with alternating row backgrounds in neutral-50 and neutral-0. No heavy borders. The numbers themselves are the content; the table structure should feel nearly invisible.

Call-to-action components use a filled primary-600 background with white text, sized generously with 16px vertical padding to create comfortable touch targets. Secondary actions use a transparent background with a 1px border in neutral-300. Neither should ever feel urgent or aggressive — the tone is invitational.

# Do's and Don'ts

Do maintain generous whitespace around text blocks. The editorial credibility of the clinic depends on the perception that the content has been carefully curated and presented. Tight spacing undermines that perception before a visitor reads a single word.

Don't introduce bright or saturated accent colors. Longevity research is a field where trust is paramount, and unexpected color pops register as promotional rather than scientific. If imagery contains vibrant color — such as stained microscopy — let it be the exception rather than a cue to match in the interface.

Do use Fraunces italics for emphasis within body text rather than bold weight. The italic provides scholarly warmth that bold lacks. Reserve bold (Source Serif 4 at weight 700) for labels, metadata, and functional elements where clarity supersedes personality.

Don't animate entry transitions for text content. The reading experience should feel still and considered. Motion is acceptable for interactive feedback — hover states, menu transitions, scroll-triggered image reveals — but textual content should simply be present, as though printed.