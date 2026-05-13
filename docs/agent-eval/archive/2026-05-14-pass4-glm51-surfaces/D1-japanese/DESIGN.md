---
name: "wa-blanc"
version: "0.1"
description: "和風ランディングページ — 白と藍、明朝見出し、ゴシック本文、たっぷり余白"
colors:
  primary:
    "50":  "#eef3f8"
    "100": "#d0dfec"
    "200": "#a8c4dc"
    "300": "#7aa5c8"
    "400": "#568db5"
    "500": "#3d7196"
    "600": "#315c7c"
    "700": "#274963"
    "800": "#1b3549"
    "900": "#112435"
  neutral:
    "0":    "#ffffff"
    "50":   "#f8f8f6"
    "100":  "#ededea"
    "200":  "#d6d6d0"
    "300":  "#b0afa7"
    "400":  "#8a8980"
    "500":  "#6b6a62"
    "600":  "#52514b"
    "700":  "#3c3b37"
    "800":  "#2a2a27"
    "900":  "#1a1a18"
    "1000": "#000000"
typography:
  display:
    family: "'Shippori Mincho', 'Noto Serif JP', 'Hiragino Mincho ProN', serif"
    weight: 700
  body:
    family: "'Noto Sans JP', 'Hiragino Kaku Gothic ProN', 'Yu Gothic', sans-serif"
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
  "32": "128px"
rounded:
  sm: "2px"
  md: "4px"
  lg: "8px"
metadata:
  variants:
    light:
      description: "Default light mode — warm white with deep slate-blue"
      tokens:
        colors.background.primary: "#ffffff"
        colors.text.primary: "#1a1a18"
    dark:
      description: "Dark mode — deep ink with cool grey"
      tokens:
        colors.background.primary: "#0e1218"
        colors.text.primary: "#f0efe8"
    brand-a:
      description: "Sumi ink variant — parchment with charcoal"
      tokens:
        colors.background.primary: "#f5f2eb"
        colors.text.primary: "#2a2520"
    brand-b:
      description: "Moonlit variant — pale blue-grey with deep navy"
      tokens:
        colors.background.primary: "#f0f2f5"
        colors.text.primary: "#1c1f26"
---

# Overview

This design system captures the quiet restraint of Japanese aesthetic tradition — *shibui* — through careful proportion, a restrained blue-slate palette, and the deliberate tension between Minchō display headings and Gothic body text. Rather than the obvious navy-and-white pairing, the primary hue is shifted toward a cooler, more muted slate-blue that evokes weathered *noroshi* signal smoke against a winter sky.

Generous whitespace is not empty space but an active compositional element. Every margin and padding value is scaled to create rhythmic breathing room, echoing the *ma* (間) concept — the meaningful pause between elements. The system prioritizes legibility and calm over visual density.

Four variants are provided to adapt the core language across contexts: a clean light default, a deep ink-dark mode, a warm parchment variant that recalls washi paper, and a cool moonlit grey-blue variant. Each maintains WCAG-AA contrast and the same typographic hierarchy.

The design avoids ornamental decoration. Texture comes from typographic contrast, spatial rhythm, and the subtle warmth or coolness of neutral backgrounds — never from added embellishment.

# Colors

The primary palette is a desaturated slate-blue family, rooted in a muted teal-blue that reads as *ai* (藍, indigo) without the saturation fatigue of a pure blue. The 900 shade sits at a deep charcoal-navy, providing strong heading color against light backgrounds while remaining softer than true black.

Neutrals carry a faint warm undertone in lighter shades and cool undertone in darker shades, preventing the palette from feeling sterile. This mimics the subtle color shift of natural materials — silk, handmade paper, unglazed clay — under different light.

Color usage follows a 90-8-2 rule: 90% neutral, 8% primary, 2% accent. Primary blue appears in links, section rules, and key interactive elements. It should never dominate a surface. Borders and dividers use primary-200 at 1px width, evoking a single brushstroke.

# Typography

Display headings use Shippori Mincho at weight 700, providing strong serif character without the visual weight of traditional bold Minchō. Letter-spacing is tightened by -0.02em for Japanese characters to create compact, elegant line forms. Heading sizes follow a 1.333 modular scale (minor third), from 12rem down through the hierarchy.

Body text uses Noto Sans JP at weight 400 for optimal screen legibility. Line-height is set at 1.85 for Japanese body text to accommodate the visual density of CJK characters and provide comfortable reading rhythm. Paragraph spacing equals one line-height unit.

The typographic system deliberately avoids the common pairing of ultra-light weights with tight tracking. Instead, it uses medium weights with generous spacing, ensuring text reads clearly at all sizes and maintains its composure across viewport widths.

# Layout

A centered single-column layout with maximum content width of 680px reflects the proportions of a traditional scroll. Wider sections use a 12-column grid with 32px gutters, but most content sits within the narrow central column to maintain focus.

Vertical rhythm follows the spacing scale, with sections separated by 96px or 128px of whitespace. Within sections, elements are spaced at 24px or 32px. This creates a clear hierarchy of pauses — the long silence between sections, the shorter breath between elements within a section.

The page is divided into horizontally banded sections, each with full-width backgrounds and contained content. This mirrors the spatial composition of a folding screen (*byōbu*), where each panel holds its own scene while contributing to a unified whole.

# Elevation & Depth

Elevation is minimal. The system uses only two levels: flush surfaces and a single subtle shadow for floating elements. Shadows are cast in primary-900 at 6% opacity, offset 2px down with 8px blur, creating a sense of depth without material weight.

Cards and interactive surfaces may elevate on hover by shifting the shadow to 4px offset and 16px blur. This is the only elevation transition. No element uses more than one shadow layer. The aesthetic draws from paper layered on a flat surface rather than objects floating in space.

Dark mode inverts the logic: surfaces lighten rather than darken as they elevate, using a 4% white overlay on the base background. This maintains the paper metaphor while respecting the inverted light relationship.

# Shapes

Border radii are minimal — 2px for small elements like buttons and tags, 4px for cards and containers. This creates softness without rounding, suggesting the slight cushion of handmade paper edges rather than machine-cut curves.

Full-circle elements are reserved for single-icon buttons or avatar containers. All other shapes maintain their rectangular character. The restraint in rounding reinforces the architectural, gridded quality of the layout.

Container edges are sharp at 0px radius for full-width sections. This creates clean horizontal boundaries between content bands. The only rounded shapes within these bands are interactive elements, providing subtle affordance without disrupting the overall rectilinear rhythm.

# Components

Key components include: full-bleed hero with centered Minchō headline and sparse subtext; horizontal rule dividers using primary-200 at 1px with 80px width; text links underlined with primary-500 at 1px; navigation bar with logo-left and minimal link set; CTA buttons in primary-500 with white text at 8px radius; and content sections with centered headings above narrow text columns.

Each component is designed to function within generous whitespace. Buttons have 16px vertical and 32px horizontal padding. Navigation links are spaced at 32px. Form inputs use the body font at 16px minimum with 12px padding and a 1px border in neutral-200.

No component includes decorative elements, gradients, or multiple background layers. Visual interest comes from scale contrast, typographic weight shifts, and the intentional use of the primary accent color at key interaction points.

# Do's and Don'ts

**Do** maintain the high whitespace ratio — never reduce section spacing below 64px. **Do** use Minchō exclusively for headings and decorative text; never for body copy or UI labels. **Do** keep borders at 1px; thicker lines break the delicate aesthetic. **Do** use the primary blue sparingly; it should feel precious and intentional.

**Don't** combine primary-500 with warm-toned neutrals; keep cool tones together. **Don't** use drop shadows on text or apply gradients to backgrounds. **Don't** introduce decorative illustration or pattern without clear functional purpose. **Don't** reduce body line-height below 1.75 for Japanese text.

**Do** test dark mode contrast at all text sizes. **Don't** use primary-200 as text color on any background; it fails contrast at all sizes. **Do** let the typography and space carry the design — the system's power is in what it omits.