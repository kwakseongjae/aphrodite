---
name: "longevity-editorial"
version: "0.1"
description: "Calm editorial presence for a longevity research clinic"
colors:
  primary:
    "50":  "#fef9ee"
    "100": "#fcf0d4"
    "200": "#f8dda8"
    "300": "#f3c76e"
    "400": "#edae3c"
    "500": "#d4922a"
    "600": "#b87520"
    "700": "#96581c"
    "800": "#7a471d"
    "900": "#5c3518"
  neutral:
    "0":    "#ffffff"
    "50":   "#faf9f7"
    "100":  "#f2f0ec"
    "200":  "#e4e0d8"
    "300":  "#c9c2b6"
    "400":  "#a69d8e"
    "500":  "#8a8070"
    "600":  "#6e6558"
    "700":  "#524b42"
    "800":  "#3a352f"
    "900":  "#2a2622"
    "1000": "#000000"
typography:
  display:
    family: "Fraunces"
    weight: 700
  body:
    family: "Newsreader"
    weight: 400
spacing:
  "1": "4px"
  "2": "8px"
  "4": "16px"
  "6": "24px"
  "8": "32px"
  "12": "48px"
  "16": "64px"
  "24": "96px"
rounded:
  sm: "4px"
  md: "8px"
  lg: "12px"
metadata:
  variants:
    light:
      description: "Default light mode — warm parchment with deep brown text"
      tokens:
        colors.background.primary: "#faf9f7"
        colors.text.primary: "#2a2622"
    dark:
      description: "Dark mode — deep warm charcoal with cream text"
      tokens:
        colors.background.primary: "#1c1a17"
        colors.text.primary: "#f2f0ec"
    brand-a:
      description: "Flax — golden linen warmth"
      tokens:
        colors.background.primary: "#f8f2e6"
        colors.text.primary: "#3a2e1e"
    brand-b:
      description: "Frost — cool ivoried quiet"
      tokens:
        colors.background.primary: "#f5f3ee"
        colors.text.primary: "#2e2b26"
---

# Overview

This system supports a slow, considered reading experience for a longevity research clinic — a place where science meets the long arc of a human life. The design should feel like opening a well-bound monograph: unhurried, authoritative, warm without being cozy. Generous whitespace is not empty space but breathing room.

The primary palette draws from amber and raw sienna rather than saturated gold or citrus. This keeps the warmth grounded and medicinal rather than decorative. Paired with warm neutrals that lean slightly stone-like, the overall impression is one of mineral calm — an institutional warmth that never feels clinical.

Fraunces serves as the display face, its variable weight and optical sizing making it suitable for everything from quiet section headers to expressive pull quotes. Newsreader handles body text, offering the gentle authority of a serif without the stiffness of a traditional text face. Together they create a typographic temperature that is learned but not aloof.

The four variants share the same tonal DNA: warm backgrounds, high-readability foregrounds, and a single amber accent that appears sparingly. Dark mode inverts the temperature slightly but maintains the parchment-meets-stone quality that makes the system feel timeless rather than trendy.

# Colors

The primary amber scale runs from a nearly-white honey at shade 50 to a deep saddle brown at 900. The 500 value — a burnished amber — is the default accent and should be used only for interactive elements, small highlights, and active states. Overuse will flatten the palette into decoration.

Neutral colors carry a subtle warm cast. Even the darkest backgrounds retain a faint brown undertone rather than going pure black. This prevents the clinical coldness that pure grays can introduce, especially in a health-adjacent context. Light mode backgrounds use neutral-50 as a linen-like off-white.

Each variant has been verified for WCAG-AA contrast between its primary background and text color. Brand-a leans slightly more golden, evoking aged paper, while brand-b is cooler and more ivory — suitable for subsections that need subtle differentiation without introducing a new hue family.

Accent usage should follow the 90-9-1 rule: 90% neutral tones, 9% tinted neutrals (borders, subtle backgrounds), 1% saturated amber. This restraint is what gives the system its editorial authority. When amber appears, it should feel deliberate.

# Typography

Fraunces is set at 700 weight for display moments. Its wonk axis and variable optical size make it feel hand-lettered at large sizes but clean at smaller sizes. Use it for page titles, section heads, and featured pull quotes. Avoid using it below 18px — below that threshold, switch to Newsreader.

Newsreader at 400 weight carries all running text. Its slightly condensed proportions make it efficient for long-form reading while maintaining the serif texture that signals editorial credibility. Recommended body size is 17–19px with 1.6–1.75 line height for optimal reading comfort on screen.

Type scale should use a 1.25 ratio: body at 1rem, large body at 1.25rem, h4 at 1.563rem, h3 at 1.953rem, h2 at 2.441rem, h1 at 3.052rem. This is a conservative scale suited to editorial layouts where hierarchy is expressed through spacing and weight as much as through size.

Avoid italicizing Fraunces — its italic axis is better expressed through the variable font's own style axis. Newsreader italic is reserved for captions, asides, and inline editorial emphasis. Never combine bold and italic simultaneously.

# Layout

The page operates on a 12-column grid with a maximum content width of 720px for single-column reading and 1080px for two-column layouts. Margins should be generous: 64px minimum on desktop, scaling down to 24px on mobile. The gutters between columns are 32px.

Vertical rhythm follows an 8px baseline. All spacing values are multiples of this unit. Paragraph spacing, section gaps, and component padding should all snap to this grid. This creates an underlying regularity that makes even complex pages feel composed.

The header is minimal — logo or wordmark left-aligned, navigation right-aligned, with 48px of vertical padding. No background color or border; it should breathe with the page. A subtle warm line (neutral-200, 1px) can separate the header from content on scroll.

Footer uses a slightly warmed background (primary-50 in light mode) and contains only essential information: address, a single contact link, and a small copyright notice. It should feel like the colophon of a book — present but recessive.

# Elevation & Depth

Elevation is minimal by design. The system relies on surface color shifts rather than shadows to communicate layering. A card might use a 1px border of neutral-200 and a background of white against a neutral-50 page — that single step is sufficient.

When shadows are necessary (dropdowns, modals), use a single subtle box-shadow: 0 4px 16px rgba(42, 38, 34, 0.08). This warm-tinted shadow avoids the cold blue cast of default shadows. Never stack multiple shadow values.

Depth can also be communicated through typography: larger, bolder text comes forward; smaller, lighter text recedes. This is preferred over adding background fills or borders. The type hierarchy should do most of the spatial work.

Avoid glassmorphism, gradient overlays, and multi-layer card nesting. The site is about longevity science — clarity and permanence should be reflected in surfaces that feel solid and singular. If you catch yourself adding a third layer of depth, remove one.

# Shapes

Border radius is used sparingly. Small interactive elements (buttons, inputs, tags) use sm (4px). Cards and larger containers use md (8px). Never exceed lg (12px), and never apply radius to full-width sections or page-level containers.

The slight rounding keeps things from feeling harsh without drifting into friendly or playful territory. This is a research institution — confidence comes from restraint. Sharp corners on images and full-bleed elements provide counterpoint to the softer UI elements.

Circles are reserved for the favicon and perhaps a small avatar. Do not use circular containers for content cards or image thumbnails. Rectangles with subtle rounding are the default shape vocabulary.

Consistency matters more than creativity here. Pick a radius and apply it uniformly within each component category. Mixing radii within a single component (different corners, nested rounding) creates visual noise that undermines the calm tone.

# Components

Buttons come in two variants: primary (primary-700 background, white text) and secondary (transparent background, primary-700 border and text). Both use Newsreader at 15px, medium weight, with 12px vertical and 24px horizontal padding. Hover states shift the background one shade darker. No icon-only buttons.

Links are underlined in primary-700, with underline offset of 3px. On hover, the underline thickens from 1px to 2px. Avoid hover color shifts — the underline change is sufficient feedback. Visited links can shift to primary-500.

Article cards show a title in Fraunces (h3 scale), a two-line excerpt in Newsreader body, and a publication date in neutral-400. The entire card is not clickable — only the title is a link. This respects the reader's intent and avoids accidental navigation.

Pull quotes use Fraunces at a larger scale (h1), centered, with generous vertical padding (96px top and bottom). The quote text sits in neutral-700 with a 3px left border in primary-300. Attribution in Newsreader italic, neutral-400, sits below with 16px gap.

# Do's and Don'ts

Do let whitespace do the work. Every section of this site should feel like it has room to breathe. If a page feels cramped, increase spacing before reducing content. The editorial tone depends on visual generosity.

Don't introduce new colors. The amber palette and warm neutrals are the entire chromatic vocabulary. If a chart or data visualization feels like it needs blue or green, use different opacities of amber instead. Monochrome is a feature, not a limitation.

Do use real editorial imagery — photographs of lab environments, abstracted cellular imagery, quiet portraits of researchers. Avoid stock photography with obvious warmth filters. Images should feel documented, not styled.

Don't animate entry transitions on text content. Paragraphs should not fade in or slide up as the user scrolls. The content is serious and should appear with the permanence of print. Subtle hover states on interactive elements are the only acceptable animation.