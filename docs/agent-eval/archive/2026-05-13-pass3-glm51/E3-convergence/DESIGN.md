---
name: "Chronos Editorial"
version: "0.1"
description: "A measured editorial presence for a longevity research clinic — warm parchment, desaturated teal-green, scholarly serif"
colors:
  primary:
    "50":  "#eef4f2"
    "100": "#d5e4df"
    "200": "#a8c9be"
    "300": "#7baa9d"
    "400": "#548b7c"
    "500": "#3a6f63"
    "600": "#2d5a50"
    "700": "#21463e"
    "800": "#17312b"
    "900": "#0e1f1b"
  neutral:
    "0":    "#ffffff"
    "50":   "#f9f6f0"
    "100":  "#f0ece3"
    "200":  "#ddd8cb"
    "300":  "#c4bdaa"
    "400":  "#9e9580"
    "500":  "#7a7060"
    "600":  "#5c5447"
    "700":  "#443e34"
    "800":  "#2e2a23"
    "900":  "#1a1712"
    "1000": "#000000"
typography:
  display:
    family: "Libre Caslon Display"
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
  sm: "4px"
  md: "8px"
  lg: "12px"
metadata:
  variants:
    light:
      description: "Warm parchment with dark espresso text — the default editorial calm"
      tokens:
        colors.background.primary: "#f9f6f0"
        colors.text.primary: "#1a1712"
    dark:
      description: "Deep charcoal-linen with warm ivory text — low-glare nocturnal reading"
      tokens:
        colors.background.primary: "#1a1712"
        colors.text.primary: "#f0ece3"
    brand-a:
      description: "Warm cream with deep teal-carbon text — branded editorial variant"
      tokens:
        colors.background.primary: "#f3efe6"
        colors.text.primary: "#0e1f1b"
    brand-b:
      description: "Light sage tint with warm charcoal text — botanical research accent"
      tokens:
        colors.background.primary: "#eef4f2"
        colors.text.primary: "#2e2a23"
---

# Overview

Chronos Editorial is built around the quiet authority of a well-set research monograph. The palette draws from aged parchment and desaturated teal-green rather than clinical blue or bright sage, evoking institutional depth without sterility. Every surface breathes; every letterform carries weight.

Longevity research demands credibility without coldness. Libre Caslon Display brings an Old More scholarly gravity to headlines — its bracketed serifs and modest contrast suggest permanence. Source Serif 4 handles body text with even color and comfortable proportions suited to extended reading about complex protocols.

The system intentionally resists decoration. Borders are rare, shadows almost absent, color applied with the restraint of a journal masthead rather than a consumer health app. Visual hierarchy comes from scale, weight, and the generous whitespace that signals seriousness.

Four variants accommodate different presentation contexts — from the default warm parchment through dark-mode reading to two branded tints — each maintaining WCAG-AA contrast and tonal consistency.

# Colors

The primary palette is a desaturated teal-green (hue ~160°, very low chroma) rather than the more obvious sage or forest. At shade 500 it reads as a muted green-gray with enough distinction to anchor links, buttons, and accents without competing with content text. Lighter shades whisper; darker shades recede into near-black.

Neutrals run warm throughout — even the darkest values carry a faint brown undertone that prevents the clinical cast of pure gray. This warmth echoes the parchment sensibility and keeps dark mode from feeling like an IDE.

Accent usage follows a strict economy. The primary 500 shade appears on interactive elements, small data visualizations, and section dividers. It never floods large surfaces. Supporting accents — a muted amber or soft ochre — can be introduced at the component level sparingly, for callouts or status indicators, but the default system remains monochromatic.

Background and text pairs across all four variants exceed 12:1 contrast ratio, well beyond the WCAG-AA minimum of 4.5:1, ensuring comfortable reading at all sizes.

# Typography

Libre Caslon Display serves as the display face — its high contrast and generous x-height give headlines a sculptural quality that rewards the reader scanning article titles and research summaries. It is used exclusively for headings at sizes 24px and above, set in bold only.

Source Serif 4 carries all other text. Its slightly larger x-height and moderate stroke contrast make it legible at body sizes (16–18px) while maintaining the scholarly atmosphere. It ships with a genuine italic, not a slanted roman, which preserves editorial credibility in citations and marginal notes.

Type scale follows a 1.25 ratio: 16, 20, 24, 30, 38, 48px. Line height for body sits at 1.65 to accommodate the dense informational prose common to research abstracts. Headings tighten to 1.2. Paragraph spacing equals one line height, creating vertical rhythm without explicit rules.

A single monospace face — a system stack — handles data tables, genetic sequences, and measurement readouts, appearing only within scoped components.

# Layout

The foundational grid is an 8px baseline with a 12-column content area maxing at 720px for prose — the proven measure for sustained reading. Wider layouts (research grids, team pages) expand to 1080px with the same column math. Margins scale with viewport: 24px on mobile, 48px on tablet, 64px on desktop.

Vertical rhythm is paramount. Every element snaps to the 8px baseline. Spacing tokens (4, 8, 16, 24, 32, 48, 64px) are the only legal vertical distances, ensuring mathematical consistency across sections authored by different contributors.

Sections within an article are separated by 64px — a deliberate pause that mimics the whitespace between journal sections. Inline elements — figures, pull quotes, asides — break the text column but respect its alignment, floating within the margin space rather than overlaying content.

Navigation is minimal: a fixed masthead with the clinic wordmark left and three links right, set in the body face at small size. No mega-menus, no dropdowns. The hierarchy is flat, reflecting the editorial monograph structure.

# Elevation & Depth

Elevation in Chronos is almost nonexistent by design. The system uses flat surfaces with border delineation rather than shadow, reinforcing the printed-page metaphor. The single elevation level — a 1px shadow offset by 2px with 6% opacity — is reserved for floating elements like tooltips and image captions.

Depth instead comes from background value shifts. Sections alternate between primary background and a 2% darker variant (e.g., #f9f6f0 to #f0ece3 in light mode), creating visual separation without competing with content. These shifts are subtle enough to feel structural rather than decorative.

Card components use a 1px border in neutral 200 rather than a shadow, keeping them flush with the surface plane. This flatness signals that content lives on a single reading surface — pages in a book, not cards on a table.

When temporary overlays are necessary (modal confirmations, lightbox images), they use the primary shadow plus a scrim at 40% opacity in the dominant neutral dark value, ensuring focus snaps to the overlay content.

# Shapes

Border radius is restrained: 4px for small elements (inputs, tags), 8px for medium elements (cards, images), 12px for large elements (modals, feature panels). No fully rounded elements exist in the system — even avatars are 8px rounded squares, prioritizing the typographic grid over decorative circles.

Buttons follow a pill-adjacent shape with 8px radius and generous horizontal padding (24px), but their height is determined by the type baseline (40px) rather than arbitrary sizing. This keeps button geometry tethered to the vertical rhythm.

Images within the content column are inset with 8px rounding on all corners, maintaining visual softness without the frame feeling like a separate container. Full-bleed images in wider layouts maintain the same rounding, applied via overflow clipping on the parent.

Decorative shapes — circles, blobs, gradients — are explicitly excluded from the system. The only non-rectilinear element is the clinic wordmark itself, which is treated as a fixed asset outside the shape system.

# Components

Buttons exist in two weights: primary (teal-green background, warm light text) and secondary (transparent background, teal-green text, 1px border). Both use Source Serif 4 at 14px with 24px horizontal padding. States — hover (darken 10%), active (darken 20%), disabled (50% opacity) — modify the existing token rather than introducing new colors.

Article headers combine the display face at 38–48px with a thin rule in neutral 300 below, followed by metadata (author, date, category) in the body face at 14px in neutral 500. This triad — title, rule, metadata — is the signature composition of the system.

Pull quotes break the text column, set in Libre Caslon Display at 24px in the primary 600 shade, with a 3px left border in primary 200. They occupy the left margin on desktop and indent 32px on mobile.

Data tables — used for clinical results and protocol comparisons — are bare: no zebra striping, no heavy borders. Header row uses neutral 100 background with 600 text; body rows alternate invisibly, separated only by a 1px bottom border in neutral 200.

# Do's and Don'ts

Do maintain generous whitespace around text blocks — the reading experience should feel unhurried, reflecting the subject matter's long time horizons.

Do use the primary teal-green sparingly. A single accent line, a small highlighted term, or a hover state is more powerful than a saturated page.

Do set all prose text in Source Serif 4. If a piece of text is meant to be read, it earns the body face — even in sidebars and footers.

Don't introduce bright or saturated colors as accents. The system's credibility relies on chromatic restraint; a vivid chart or badge would rupture the scholarly tone.

Don't use display type below 24px. Libre Caslon Display is a headline face; at smaller sizes its high contrast becomes a liability.

Don't apply shadows gratuitously. If two surfaces need differentiation, use a border or a background value shift instead. Shadows break the printed-page metaphor and introduce visual noise.