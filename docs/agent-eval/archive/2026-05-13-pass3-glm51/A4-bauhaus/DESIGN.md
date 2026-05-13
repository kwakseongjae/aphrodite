---
name: "bauhaus-kinder"
version: "0.1"
description: "Bauhaus-inspired pastel learning app for children with geometric discipline and warm coral geometry"
colors:
  primary:
    "50": "#fff5f0"
    "100": "#ffe8dd"
    "200": "#ffc4a8"
    "300": "#ff9e75"
    "400": "#ff6b3d"
    "500": "#e85d30"
    "600": "#c44a22"
    "700": "#9c3818"
    "800": "#752a12"
    "900": "#4e1c0c"
  secondary:
    "50": "#f0f6f4"
    "100": "#d6ebe4"
    "200": "#add7c9"
    "300": "#80c2ab"
    "400": "#58b08f"
    "500": "#3d9a76"
    "600": "#2f7a5e"
    "700": "#245c46"
    "800": "#1a3e30"
    "900": "#11221a"
  accent:
    "50": "#f6f3ff"
    "100": "#ebe5ff"
    "200": "#d6caff"
    "300": "#b9a0ff"
    "400": "#9b73ff"
    "500": "#804dff"
    "600": "#6b30e6"
    "700": "#5720b8"
    "800": "#43168a"
    "900": "#2e0c5c"
  neutral:
    "0": "#ffffff"
    "50": "#faf9f7"
    "100": "#f0eeea"
    "200": "#ddd9d3"
    "300": "#c4bfb6"
    "400": "#a69f93"
    "500": "#8a8278"
    "600": "#6d665c"
    "700": "#524c44"
    "800": "#38342e"
    "900": "#1f1d19"
    "1000": "#000000"
  pastel-yellow:
    "100": "#fff9e6"
    "200": "#ffefb3"
    "300": "#ffe580"
  pastel-rose:
    "100": "#fff0f0"
    "200": "#ffd9d9"
    "300": "#ffc2c2"
  pastel-mint:
    "100": "#edfaf4"
    "200": "#c8f0de"
    "300": "#a3e6c8"
typography:
  display:
    family: "Archivo Black"
    weight: 900
  body:
    family: "Nunito"
    weight: 400
spacing:
  "1": "4px"
  "2": "8px"
  "3": "12px"
  "4": "16px"
  "5": "20px"
  "6": "24px"
  "8": "32px"
  "10": "40px"
  "12": "48px"
  "16": "64px"
rounded:
  none: "0px"
  sm: "6px"
  md: "12px"
  lg: "20px"
  xl: "28px"
  full: "9999px"
metadata:
  variants:
    light:
      description: "Default light mode — warm cream with coral geometry"
      tokens:
        colors.background.primary: "#faf9f7"
        colors.background.secondary: "#f0eeea"
        colors.text.primary: "#1f1d19"
        colors.text.secondary: "#524c44"
        colors.surface: "#ffffff"
    dark:
      description: "Dark mode — deep charcoal with glowing pastels"
      tokens:
        colors.background.primary: "#141210"
        colors.background.secondary: "#1f1d19"
        colors.text.primary: "#f0eeea"
        colors.text.secondary: "#a69f93"
        colors.surface: "#252320"
    brand-a:
      description: "Sunlit studio variant — golden warmth for morning sessions"
      tokens:
        colors.background.primary: "#fffbf2"
        colors.background.secondary: "#fff5e6"
        colors.text.primary: "#2a2010"
        colors.text.secondary: "#6b5a3a"
        colors.surface: "#fffdf8"
    brand-b:
      description: "Forest workshop variant — sage greens and earthy depth"
      tokens:
        colors.background.primary: "#f4f7f5"
        colors.background.secondary: "#e4ebe6"
        colors.text.primary: "#121f18"
        colors.text.secondary: "#3a5245"
        colors.surface: "#f9fbfa"

---

# Overview

Bauhaus-Kinder draws from the Bauhaus school's radical clarity — where every shape earns its place and color serves communication, not decoration. The system translates this philosophy into a children's learning context, replacing austerity with warmth while preserving geometric intentionality. Every element should feel like it belongs in a carefully composed classroom poster from 1923 Dessau, yet function intuitively for a five-year-old today.

The palette centers on a muted coral-terracotta primary rather than the expected toy-bright primaries. This choice grounds the experience in earthy warmth and avoids the visual noise of saturated competing colors. Pastels — butter yellow, faded rose, seafoam mint — appear as supporting geometric accents, never as background fields. The neutral scale runs warm, with faint amber undertones that prevent the sterility of pure gray.

Typography pairs Archivo Black's slab-serif confidence with Nunito's soft rounded terminals. The contrast creates a productive tension: headlines feel architectural and authoritative (Bauhaus), while body text feels friendly and approachable (kindergarten). Neither font is the obvious first choice for a children's app, which is precisely the point — legibility through structure, not through infantile roundness.

The system defines four variants. Light and dark serve standard mode preferences. Brand-a introduces golden warmth for morning learning sessions. Brand-b shifts to forest sage for nature-focused modules. All maintain WCAG-AA contrast for text readability.

# Colors

The primary coral-terracotta family (e85d30 at 500) anchors the palette. It reads as warm and energetic without the aggressive stimulation of pure orange or the gendered associations of pink. At shade 50, it becomes a whisper of peach suitable for large background surfaces. At 900, it provides near-black depth for text on pastel grounds.

Secondary sage green (3d9a76) and accent violet (804dff) complete a triadic relationship inspired by classic Bauhaus color theory exercises. These appear sparingly — as geometric accents, interactive highlights, and category indicators. The discipline is critical: each shape or interactive element may use one accent color, never all three simultaneously.

Pastel supporting tones (yellow, rose, mint) exist for decorative geometric shapes that float behind content areas. These shapes are large, low-contrast, and non-interactive. They provide atmospheric playfulness without competing with functional elements. Their saturation is deliberately restrained — more Scandinavian kindergarten than American candy store.

Neutral tones carry consistent warmth across the scale. Even the darkest neutral (1f1d19) leans slightly warm compared to pure black, preventing the clinical feeling that cool neutrals introduce in educational contexts. This warmth is subtle but cumulative across an entire interface.

# Typography

Archivo Black at display weight (900) handles all headings, navigation labels, and key numerical content. Its geometric construction and consistent stroke width reference Bauhaus typographic principles without the austerity of purely sans-serif systems. The heavy weight ensures headlines land with confidence — important when young readers scan for section markers.

Nunito at weight 400 serves body text, button labels, and instructional content. Its rounded terminals feel approachable without sacrificing legibility. The moderate x-height and open apertures support early readers who are still developing letter recognition. Weight 600 is available for emphasis within body contexts, reserving the display font for structural hierarchy.

Type scale follows a 1.25 ratio. Body text defaults to 16px. Small text at 12.8px serves captions and metadata only — never instructional content. Large display sizes cap at 40px, beyond which geometric shape elements take over visual hierarchy duties rather than endlessly scaling type.

Line height runs generous at 1.6 for body text and 1.2 for display. Children's reading comprehension benefits from generous vertical spacing, and the Bauhaus grid discipline accommodates this through consistent spacing multiples rather than arbitrary leading values.

# Layout

The grid system operates on an 8px base unit. All spacing, sizing, and positioning snap to this grid or its 4px half-step. This creates the rhythmic regularity that defines Bauhaus composition — elements align with mathematical clarity, even when the content feels playful and organic.

Content areas maintain generous padding (24px minimum) to prevent the cramped feeling that overwhelms young users. Geometric accent shapes occupy designated zones at grid intersections, never overlapping interactive elements. The layout treats negative space as an active compositional element, not an afterthought to fill.

Cards and content modules follow a consistent width rhythm: 280px, 360px, and 480px for small, medium, and large contexts. These widths ensure comfortable reading measures and predictable layout behavior across screen sizes. Full-bleed geometric backgrounds may extend beyond content boundaries, but interactive elements always respect the content grid.

Navigation uses a persistent side rail on tablet and desktop, collapsing to a bottom bar on mobile. The rail itself incorporates geometric cutouts — semicircles or angled corners — that reference Bauhaus architectural motifs without interfering with tap targets. Minimum touch target size is 44px in all directions.

# Elevation & Depth

The system employs five elevation levels. Level zero sits flush with the background — standard content areas, text blocks, illustrations. Level one (2px offset, 4px blur) lifts cards slightly for subtle separation. Level two (4px offset, 8px blur) designates interactive surfaces like buttons and input fields. Level three (8px offset, 16px blur) elevates modals and popovers. Level four (16px offset, 32px blur) reserves for system-level alerts.

Shadow colors derive from the neutral palette with increased opacity, never pure black. This maintains the warm tonal consistency across all variants. In dark mode, shadows shift to lower-opacity dark tones that blend with the background rather than creating harsh boundaries.

Geometric shapes can overlap to create implied depth without shadows — a circle partially behind a rectangle creates natural layering that references Bauhaus collage techniques. These compositional overlaps are decorative only and never interfere with interactive z-indexing.

Border treatments serve as an alternative elevation signal. A 2px border in neutral-200 provides equivalent visual separation to a level-one shadow, useful in contexts where shadows feel too heavy or where the design calls for flat geometric clarity. The border approach aligns with the Bauhaus preference for defined edges over atmospheric blur.

# Shapes

Three primary geometric forms drive the visual language: circles, equilateral triangles, and rectangles (including squares). These appear as decorative backgrounds, container shapes, button morphologies, and iconographic elements. Every rounded corner references the circle. Every angled cut references the triangle. Every straight edge references the rectangle.

Border radius values are deliberate. The small radius (6px) softens rectangles without making them read as pills. The medium radius (12px) creates clearly rounded cards. The large radius (20px) produces friendly button and badge shapes. The extra-large radius (28px) approaches circle territory for hero elements and feature cards. Full radius creates pills and circular avatars.

No arbitrary curves. If a shape is rounded, it uses a defined radius token. If a shape is angled, it snaps to 30-degree increments (0, 30, 60, 90). Freeform curves and organic blobs are excluded from the system — the discipline of geometry is what separates this from generic playful interfaces.

Geometric accent shapes maintain minimum dimensions of 48px to ensure they read as intentional forms rather than visual noise. Maximum dimensions cap at 200px to prevent single shapes from dominating compositions. Overlap between accent shapes is permitted within a dedicated layer, but accent shapes never overlap functional content.

# Components

Buttons follow clear geometric typology. Primary buttons use the medium radius with solid primary-500 fill and white text. Secondary buttons use a 2px primary-500 border with primary-500 text on transparent ground. Icon-only buttons are perfect circles at 44px minimum. Button text always uses Nunito 600, never the display font — buttons communicate action, not hierarchy.

Cards combine rectangular containers with optional geometric accent corners — a semicircular cutout or triangular clip at one corner that signals the Bauhaus influence without reducing content area. Card backgrounds use surface color with level-one elevation. Card headers may use pastel accent fills for category coding, but the fill area is limited to 64px height maximum.

Input fields use the large radius to distinguish them from buttons (medium radius). Labels sit above inputs in Nunito 600 at 12.8px. Focus states introduce a 3px primary-500 outline offset by 2px — visible and clear for young users developing motor control. Error states use a muted red (#c43e3e) with an accompanying icon, never relying on color alone.

Navigation items combine text labels with geometric shape indicators — a circle for the current section, a triangle for notifications, a square for settings. These shapes provide non-textual wayfinding that supports pre-literate users. Active states fill the shape with primary-500; inactive states use neutral-300 outlines.

Progress indicators use segmented geometric paths rather than continuous bars. A series of circles or squares that fill progressively creates a visual rhythm that young learners can count and track. Completed segments use primary-500 fill; the current segment pulses with a subtle opacity animation; future segments remain in neutral-200.

# Do's and Don'ts

Do maintain geometric consistency within a single view. If circles represent navigation items, all navigation items use circles. Mixing shape meanings within a context confuses the visual grammar. Each functional category claims one shape and keeps it across the entire application.

Don't use more than three colors in a single compositional group. The Bauhaus discipline requires restraint. A card may use one background color, one text color, and one accent color. If a fourth color seems necessary, the composition needs simplification, not more color.

Do ensure all interactive elements have both a geometric shape indicator and a text label. Young users may recognize shapes before they can read, but accessible design requires redundant encoding. The shape accelerates recognition; the text confirms meaning.

Don't apply decorative geometric shapes to interactive surfaces. Background shapes and functional elements occupy separate layers that never intersect. A triangle behind a card is decorative. A triangle on a button is functional. These must never overlap, as the visual confusion undermines both aesthetics and usability.

Do test all compositions in the dark variant early and often. The warm pastel palette that sings on cream backgrounds can become muddy on dark surfaces. Verify that accent colors maintain sufficient brightness against dark-mode backgrounds and that geometric shapes read clearly without excessive elevation.

Don't sacrifice spacing density for content density. Children's interfaces need breathing room. If a screen feels cramped, remove content rather than reducing spacing. The grid's generosity is a feature, not wasted space. Every spacing token exists to create rhythm and readability, not to be minimized.