---
name: "longevity-editorial"
version: "0.1"
description: "Calm editorial site for a longevity research clinic with warm neutrals and scholarly serif typography"
colors:
  primary:
    "50":  "#f6f3ed"
    "100": "#e8e0d2"
    "200": "#d4c7b0"
    "300": "#bfa98d"
    "400": "#a98b6b"
    "500": "#8a6d47"
    "600": "#6e573a"
    "700": "#54412d"
    "800": "#3a2c20"
    "900": "#221a14"
  neutral:
    "0":    "#ffffff"
    "50":   "#faf8f4"
    "100":  "#f0ece4"
    "200":  "#ddd6c9"
    "300":  "#c4b9a7"
    "400":  "#a89a84"
    "500":  "#8b7d68"
    "600":  "#6d6152"
    "700":  "#504740"
    "800":  "#332e2a"
    "900":  "#1a1715"
    "1000": "#000000"
  accent:
    "50":  "#f0f5f1"
    "100": "#d9e5db"
    "200": "#b3cbb7"
    "300": "#8db093"
    "400": "#6b9472"
    "500": "#537a5a"
    "600": "#426248"
    "700": "#324a37"
    "800": "#233326"
    "900": "#142818"
typography:
  display:
    family: "'Newsreader', 'Georgia', serif"
    weight: 700
  body:
    family: "'Source Serif 4', 'Georgia', serif"
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
  sm: "3px"
  md: "6px"
  lg: "10px"
metadata:
  variants:
    light:
      description: "Default light mode — warm parchment with umber text"
      tokens:
        colors.background.primary: "#faf8f4"
        colors.background.secondary: "#f0ece4"
        colors.text.primary: "#1a1715"
        colors.text.secondary: "#504740"
        colors.border.default: "#ddd6c9"
        colors.accent.default: "#537a5a"
        colors.accent.muted: "#6b9472"
    dark:
      description: "Dark mode — deep espresso with cream text"
      tokens:
        colors.background.primary: "#1a1715"
        colors.background.secondary: "#262220"
        colors.text.primary: "#f0ece4"
        colors.text.secondary: "#c4b9a7"
        colors.border.default: "#3d3731"
        colors.accent.default: "#8db093"
        colors.accent.muted: "#6b9472"
    brand-a:
      description: "Parchment variant — warmer cream with bistre ink"
      tokens:
        colors.background.primary: "#f5f0e6"
        colors.background.secondary: "#ebe3d5"
        colors.text.primary: "#2a1f12"
        colors.text.secondary: "#5e4f3d"
        colors.border.default: "#d6cab5"
        colors.accent.default: "#4a6e50"
        colors.accent.muted: "#6b9472"
    brand-b:
      description: "Cool sage variant — muted green-gray with warm ink"
      tokens:
        colors.background.primary: "#f2f4f0"
        colors.background.secondary: "#e4e8e2"
        colors.text.primary: "#1c2018"
        colors.text.secondary: "#4a5044"
        colors.border.default: "#c8cfc4"
        colors.accent.default: "#8a6d47"
        colors.accent.muted: "#a98b6b"
---

# Overview

This design system serves a longevity research clinic with the gravity and quietude the subject demands. The visual language draws from academic journals and small-press monographs — publications that earn trust through restraint rather than spectacle. Every element exists to serve the reader's comprehension and comfort over extended sessions.

The palette centers on warm umbers and cream rather than clinical whites or sterile blues. This warmth subtly communicates care and human attention, distinguishing the clinic from cold pharmaceutical aesthetics. The restrained sage accent provides a single directional signal without competing for attention.

Typography carries the editorial identity. Two related but distinct serif faces create hierarchy through weight and size contrast rather than family switching. The system prioritizes generous line height, comfortable measure, and unhurried vertical rhythm suited to long-form research content.

# Colors

The primary palette is built on a warm umber spectrum — reminiscent of aged paper, leather bindings, and natural materials. The 500 value (#8a6d47) serves as the core accent for interactive elements and small highlights, while the extremes provide tonal range for backgrounds and deep text.

Sage greens appear only as secondary accents for success states, research category tags, and subtle directional cues. This limited use preserves the green's ability to signal growth and vitality without diluting it across the interface.

All four variants maintain WCAG-AA contrast ratios. The light and brand-a variants use dark warm tones on pale backgrounds. The dark variant inverts to cream on espresso. Brand-b introduces a cool sage atmosphere while retaining warm ink tones for text.

# Typography

Newsreader at 700 weight handles display moments — page titles, section openers, and pull quotes. Its high-contrast serifs and dignified proportions reference classical typographic traditions without feeling antiquarian. Source Serif 4 at 400 serves body text with a slightly more mechanical warmth that stays legible at small sizes over long reads.

The type scale follows a 1.25 ratio with deliberate size jumps between levels. Body text sits at 18px with 1.7 line-height — generous but not diffuse. Headings compress line-height proportionally as size increases.

Letter-spacing loosens slightly for display sizes and tightens for body, following optical rather than mathematical rules. No letter-spacing is applied to running body text to preserve natural reading rhythm.

# Layout

The grid uses a 12-column structure with generous gutters (32px at desktop, narrowing to 16px on mobile). Content maxes at 720px for optimal reading measure — wide enough for comfort, narrow enough to track. Wider layouts are reserved for data visualizations and research comparison tables.

Vertical spacing follows a baseline grid derived from 8px. Paragraphs receive 24px bottom margin. Sections breathe with 64px or 96px gaps. This consistent rhythm creates a sense of editorial pacing — the page feels composed, not assembled.

Navigation is minimal and fixed: clinic name, three to five primary links, and a single call-to-action. Interior pages use breadcrumb-style progression. The layout prioritizes content density and readability over decorative whitespace.

# Elevation & Depth

Depth in this system is restrained and functional. Two elevation levels suffice: flush (no shadow, bordered) and raised (subtle shadow, no border). Shadows use warm gray tones to maintain palette cohesion rather than neutral blacks.

Raised elevation applies only to interactive cards, modal overlays, and sticky navigation. All other elements remain flush, using borders and background contrast for separation. This limited elevation vocabulary keeps the interface grounded and scholarly.

On dark backgrounds, shadows are replaced by subtle lighter borders and background tinting. The depth system adapts to each variant's tonal direction rather than inverting shadow values mechanically.

# Shapes

Corner radii are intentionally tight — 3px for small elements, 6px for cards and containers, 10px for large panels. This near-square quality reinforces the editorial, typeset character. Fully rounded elements are reserved for avatars and status indicators only.

Interactive elements maintain consistent shape language. Buttons use the same radius as their container context. Input fields match adjacent card radius. This consistency creates subconscious grouping without explicit borders.

Aspect ratios for imagery follow classical editorial proportions: 3:2 for feature images, 4:3 for thumbnails, and 16:9 only for video content. Portraits use a consistent 5:6 ratio for research team headshots.

# Components

Buttons exist in two weights: primary (filled with umber) and secondary (outlined or ghost). Both use Source Serif 4 for labels, set in sentence case with comfortable horizontal padding. Hover states deepen or lighten predictably. Transitions are slow (200ms ease) to match the unhurried tone.

Cards for research articles display title, author byline, publication date, and a single category tag. No thumbnails on list views — the typography is the visual element. Featured articles may include a muted photographic header.

Form inputs use generous padding and clear labels positioned above fields. Validation states use the accent palette sparingly — a thin border and small inline message. No animated shaking or attention-grabbing error patterns.

# Do's and Don'ts

Do maintain generous whitespace around text blocks. Do use the warm neutral palette for all structural elements. Do let typography dominate the visual hierarchy. Do ensure images feel quiet and desaturated — documentary, not advertising. Do use the sage accent sparingly to preserve its signaling power.

Don't introduce blue tones, which invoke clinical pharmaceutical aesthetics. Don't use sans-serif typefaces, which undermine the editorial authority. Don't animate elements beyond subtle fades and gentle shifts. Don't crowd the navigation with more than seven items. Don't use rounded avatars for anything other than actual people. Don't place colored backgrounds behind body text — maintain the warm parchment field.

Don't mix the two serif families within the same text block. Don't bold body text for emphasis — use italic instead. Don't use the 900 primary shade for large filled areas; reserve it for text only.