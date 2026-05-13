---
name: "folio"
version: "0.1"
description: "A literary magazine landing page built for immersive long-form reading with generous serif typography and restrained warmth."
colors:
  primary:
    "50": "#faf9f7"
    "100": "#f0ede6"
    "200": "#ddd9cf"
    "300": "#c4bfb3"
    "400": "#a99f8e"
    "500": "#7d7363"
    "600": "#635b4e"
    "700": "#4e483f"
    "800": "#3a3630"
    "900": "#2a2621"
  accent:
    "50": "#f2fbf5"
    "100": "#daf2e2"
    "200": "#b8e3c6"
    "300": "#8ccb9f"
    "400": "#66b17e"
    "500": "#4d9465"
    "600": "#3b7a50"
    "700": "#2f6341"
    "800": "#285137"
    "900": "#234530"
  neutral:
    "0": "#ffffff"
    "50": "#f8f7f5"
    "100": "#eae8e4"
    "200": "#d5d1cb"
    "300": "#b5afa6"
    "400": "#8f877c"
    "500": "#6d655a"
    "600": "#565048"
    "700": "#3f3b35"
    "800": "#2a2621"
    "900": "#181614"
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
  "20": "80px"
  "24": "96px"
rounded:
  sm: "2px"
  md: "4px"
  lg: "6px"
metadata:
  variants:
    light:
      description: "Default warm paper light mode for long-form reading"
      tokens:
        colors.background.primary: "#faf9f7"
        colors.text.primary: "#2a2621"
    dark:
      description: "Inverted ink-on-night dark mode for evening reading"
      tokens:
        colors.background.primary: "#1c1a17"
        colors.text.primary: "#e6e2db"
    brand-a:
      description: "Sun-bleached vellum variant with parchment warmth"
      tokens:
        colors.background.primary: "#f5f0e8"
        colors.text.primary: "#352d22"
    brand-b:
      description: "Verdant journal variant with cool botanical undertone"
      tokens:
        colors.background.primary: "#f0f4f1"
        colors.text.primary: "#1e2a1f"

---

# Overview

Folio is a literary magazine landing page designed around a single conviction: reading deserves architecture. Every decision — from the warm stone-and-ink palette to the optical sizing of the serif faces — serves the reader's sustained attention. The page breathes at the pace of a turned page.

The primary palette draws from the taupe-clay spectrum, deliberately avoiding both the expected blue and the trendy terracotta. This is the color of old paper, of linen bookcloth, of quiet afternoon light. A single accent — a muted sage green — marks interactive elements and editorial callouts without competing with the text.

Fraunces, with its variable weight and optical size axes, anchors headlines with warmth and sculptural presence. Source Serif 4 handles body text with the even color and generous spacing that long essays demand. The two pair through shared old-style sensibilities rather than matching geometry.

Four variants offer different reading environments: warm paper (light), inverted ink (dark), sun-bleached vellum (brand-a), and cool botanical (brand-b). Each maintains WCAG-AA contrast and identical structural rhythm.

# Colors

The primary palette lives in the desaturated warm-brown range, specifically the region between raw umber and warm grey. These hues recede. They create a sense of material — paper, cloth, binding board — without asserting chromatic identity. Shades 50 through 100 serve as background tones; 500 through 700 handle secondary text and rules; 800 and 900 carry primary text.

The accent palette is a single, quiet sage green (#4d9465 at 500). It appears only on interactive elements — links, buttons, selected states — and editorial metadata like category tags. It is never used decoratively or as a large-area fill. Its coolness against the warm neutrals creates just enough tension to guide the eye without breaking concentration.

Neutrals follow the same warm bias as the primary family. Pure white (#fff) is reserved for isolation moments; the default light background sits at 50 (#f8f7f5), which reads as warm white. Pure black never appears in text — even the darkest shade (#181614) carries a faint warmth that prevents the harshness of true black on screen.

The dark variant inverts the relationship thoughtfully. Background shifts to a warm near-black (#1c1a17) and text lifts to a parchment white (#e6e2db). The accent green brightens slightly to maintain visibility against the dark ground.

# Typography

Fraunces at display weight (700) with optical sizing creates headlines that feel carved rather than typeset. Its soft bracketed serifs and gentle stroke modulation reference Renaissance lettering without descending into novelty. Headlines should be set large — 48px minimum on desktop — with generous line-height (1.15–1.25) and tight tracking (−0.01em) to maintain color density at scale.

Source Serif 4 at body weight (400) prioritizes even texture and comfortable reading geometry. Its x-height is moderate, its serifs unobtrusive. Body text should be set between 18 and 21px with line-height of 1.6 to 1.75. Measure (line length) must stay between 55 and 75 characters, enforced with max-width containers.

A strict two-tier hierarchy governs all type: display for headlines, pull quotes, and the masthead; body for everything else. Subheadings use body-family at semibold weight (600) rather than switching to the display family. This creates a family resemblance rather than a contrast between headline and text.

Line spacing, paragraph spacing, and vertical rhythm are more important than font size for readability. Paragraphs should be separated by exactly one line-height of spacing. Indented first lines may replace paragraph spacing for dense literary text.

# Layout

The page follows a single-column reading layout with a centered content area maxing at 680px. This is a book-width measure. Sidebars, multi-column grids, and modular arrangements are avoided entirely. The reading experience is linear, contemplative, and undistracted.

Above the fold, the masthead sits centered with generous vertical padding (96px top on desktop). The issue number or seasonal tag appears in accent-colored small caps beneath. A single featured essay or story excerpt occupies the remainder of the viewport, set in display typography with a "Continue reading" link marked by an understated arrow.

Below the fold, recent pieces are listed chronologically — not in a grid of cards but in a sequential list with title, author, and a two-line excerpt. Each item is separated by a thin rule in neutral-200. This newspaper-stock-list approach respects the editorial hierarchy without visual competition.

The footer contains only essential information: masthead credits, submission guidelines link, and an email newsletter signup. It uses the same max-width container and disappears gracefully with muted text sizing.

# Elevation & Depth

Elevation is minimal and purely atmospheric. The light variant uses a single shadow level — a soft 2px offset with 8px blur in neutral-900 at 8% opacity — reserved for interactive cards or modals that appear above the reading surface.

The dark variant relies on luminance steps rather than shadows. Elevated surfaces are slightly lighter than the base background (#242119 vs #1c1a17), creating a subtle glow rather than a shadow. This preserves the ink-and-paper metaphor in reverse.

No element receives more than one elevation level above the base. The reading surface itself should feel flat and material — like a sheet of paper on a desk. Depth is reserved for things that float above the page: a sticky navigation header, an expanded menu, a modal reader.

Borders are preferred over shadows for defining edges. A 1px rule in neutral-200 provides separation without the visual weight of a drop shadow. This keeps the page feeling typographic rather than interface-driven.

# Shapes

Rounded corners are minimal across all elements. The default radius is 2px (sm), applied to buttons, inputs, and accent-tag badges. This is just enough to soften the intersection without creating a rounded-rectangle aesthetic. The maximum radius is 6px (lg), used only on featured image containers.

The overall page geometry is rectilinear. Images, pull quotes, and content containers align to hard edges. The reading column itself has no visible border — it is defined by the placement of text, not by a box. Margins are generous and symmetric.

Buttons follow an understated form: text-label-only with a thin border in neutral-300, or a ghost style with no border and accent-colored text. Active and hover states shift the text to accent-600 and add a subtle underline. There are no filled-background buttons on the default variant.

Pull quotes are set apart by a wide left border in accent-500 (3px solid), left-aligned with the body text. They do not receive a background fill or different container shape.

# Components

The masthead component consists of the publication name in Fraunces 700 at 32–48px, centered, with the issue designation in Source Serif 4 at 12px uppercase tracking 0.12em in accent-500 directly below. Vertical spacing between name and issue is 8px.

Navigation is a simple horizontal list of text links: Fiction, Poetry, Essays, About. Links are Source Serif 4 at 14px in neutral-500, with hover state shifting to accent-600 and an underline appearing 2px below the baseline. The nav sits left-aligned on desktop below the masthead, collapsing to a hamburger on mobile.

Article list items are the core repeating component. Each contains a category tag (Source Serif 4, 11px uppercase, accent-600), a headline (Fraunces 700, 24px), an author byline (Source Serif 4, 14px, neutral-500), and a two-line excerpt (Source Serif 4, 16px, neutral-700). Vertical spacing between items is 48px with a 1px neutral-200 rule.

The newsletter signup is a text field with a border-only button, both at rounded sm. The input uses body typography at 14px with a placeholder in neutral-400. The submit button reads "Subscribe" in accent-600 text. No background fills, no icons.

# Do's and Don'ts

Do maintain generous whitespace around all text elements. Literary content needs air. Margins below 32px between major sections feel cramped. When in doubt, add more space.

Don't introduce a second accent color. The entire system relies on a single chromatic note against warm neutrals. Additional colors fracture the reading atmosphere and compete with content for attention.

Do use real typographic hierarchy — weight, size, and spacing — rather than color to distinguish elements. A headline at 48px 700 is obviously distinct from body at 18px 400. You do not need a different color to make this clear.

Don't set body text below 18px or above 75 characters per line. Literary content demands comfortable reading geometry. If the measure exceeds 75 characters, narrow the container.

Do let images breathe. If a photograph or illustration appears, give it full column width with 48px vertical padding above and below. No floating, no wrapping text around images, no multi-column image grids.

Don't use Fraunces for body text or Source Serif 4 for display headlines. Each face has a specific role. Cross-contamination weakens the hierarchy and makes the page feel either monotonous or chaotic.

Do keep the dark variant truly dark. Background values above #252520 start to read as grey rather than night, which breaks the immersion for evening readers.

Don't add decorative elements — dividers, ornaments, patterns, or background textures. The typography and spacing are the design. Ornamentation signals a lack of confidence in the type itself.