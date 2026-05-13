---
name: "Meridian"
version: "0.1"
description: "Frosted glassmorphic landing for a macOS-style productivity app with warm terracotta accents and translucent depth"
colors:
  primary:
    "50":  "#fef4ee"
    "100": "#fde4d4"
    "200": "#f9c5a4"
    "300": "#f5a06e"
    "400": "#ef7a3e"
    "500": "#e35a1a"
    "600": "#c74514"
    "700": "#a43614"
    "800": "#842d17"
    "900": "#5c1f0e"
  neutral:
    "0":    "#ffffff"
    "50":   "#faf9f7"
    "100":  "#f2f0ec"
    "200":  "#e4e1da"
    "300":  "#cec9bf"
    "400":  "#aea899"
    "500":  "#8f8877"
    "600":  "#746d5f"
    "700":  "#5c564b"
    "800":  "#49453d"
    "900":  "#3a3731"
    "950":  "#211f1b"
    "1000": "#000000"
typography:
  display:
    family: "SF Pro Display, -apple-system, Helvetica Neue, sans-serif"
    weight: 700
  body:
    family: "SF Pro Text, -apple-system, Lucida Grande, sans-serif"
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
  sm: "6px"
  md: "10px"
  lg: "14px"
  xl: "20px"
  pill: "9999px"
metadata:
  variants:
    light:
      description: "Warm light with soft cream backgrounds and terracotta highlights"
      tokens:
        colors.background.primary: "#ffffff"
        colors.background.secondary: "#faf9f7"
        colors.background.glass: "rgba(255, 255, 255, 0.45)"
        colors.text.primary: "#1c1914"
        colors.text.secondary: "#6b6459"
        colors.border.glass: "rgba(255, 255, 255, 0.5)"
        colors.glow.primary: "#e35a1a"
    dark:
      description: "Deep warm charcoal with ember-like accents"
      tokens:
        colors.background.primary: "#1a1816"
        colors.background.secondary: "#12110f"
        colors.background.glass: "rgba(26, 24, 22, 0.55)"
        colors.text.primary: "#f5f2ed"
        colors.text.secondary: "#a09888"
        colors.border.glass: "rgba(255, 255, 255, 0.08)"
        colors.glow.primary: "#ef7a3e"
    brand-a:
      description: "Muted sage with terracotta warmth for an organic feel"
      tokens:
        colors.background.primary: "#f6f3ee"
        colors.background.secondary: "#edeae3"
        colors.background.glass: "rgba(246, 243, 238, 0.5)"
        colors.text.primary: "#2d2923"
        colors.text.secondary: "#7d756a"
        colors.border.glass: "rgba(255, 255, 255, 0.45)"
        colors.glow.primary: "#c74514"
    brand-b:
      description: "Cool parchment with subtle blue-grey undertone"
      tokens:
        colors.background.primary: "#f3f4f6"
        colors.background.secondary: "#e8eaed"
        colors.background.glass: "rgba(243, 244, 246, 0.48)"
        colors.text.primary: "#1a1d24"
        colors.text.secondary: "#64697a"
        colors.border.glass: "rgba(255, 255, 255, 0.5)"
        colors.glow.primary: "#a43614"

---

# Overview

Meridian draws from the soft, layered luminosity of macOS native windows at dusk — translucent panels floating over warm gradient canvases. The system captures that moment when desktop light dims and interface surfaces glow with gentle internal warmth rather than cold sharpness.

The primary hue family sits in terracotta and burnt sienna rather than the expected blue or violet, pulling the entire palette toward an earthy, grounded warmth. This choice differentiates Meridian immediately: it feels like a tool crafted from natural materials, not stamped from synthetic ones. The accent reads as confident but never aggressive.

Frosted glass surfaces use low-opacity white (or dark) fills with generous 20–40px blur radii. The effect is atmospheric — content layers recede and advance naturally, guided by translucency rather than hard shadows. Every surface should feel like it could be touched and would yield slightly, like waxed paper held against light.

Four variants cover the range from bright warm-neutral light to deep molten dark, plus two brand directions that shift background warmth without altering the core accent. All maintain WCAG-AA contrast on critical text-over-background pairings.

# Colors

The terracotta primary spans from a pale blush at 50 to a deep roasted brown at 900, with the 500 level hitting a saturated rust that anchors CTAs, active states, and focus rings. Intermediate stops allow smooth gradients across glass surfaces — a common technique for the aurora-style backgrounds that sit beneath frosted panes.

Neutral ramps lean warm across all values. Even pure white backgrounds carry a hint of cream in the secondary surface. Dark mode avoids true black; instead it settles into a warm charcoal that echoes oxidized metal, making bright text feel comfortable rather than piercing.

Glass border tokens are semi-transparent white in light modes and near-invisible white in dark mode. The border's job is optical — to define the fuzzy edge where frosted content meets blurred background — not to draw attention to itself. A 1px border at the defined opacity is sufficient.

The glow token provides a warm radial gradient placed behind focused or featured elements. It simulates light bleeding through the interface from an unseen warm source, reinforcing the terracotta family's organic warmth.

# Typography

SF Pro Display handles headlines at weights 600–700, set in tight tracking for large sizes (-0.02em below 24px, -0.03em above). The goal is density with clarity — letterforms that feel engineered and precise, the unmistakable signature of Apple's type system, without custom font licensing overhead.

Body text uses SF Pro Text, designed for smaller sizes with slightly more generous spacing and optical sizing. At 15–17px for paragraph copy and 13px for captions and metadata, the weight stays at 400 regular. Italic at 400 for secondary captions adds hierarchy without demanding a second color.

Type scale follows a 1.25 ratio: 13 / 16 / 20 / 25 / 31 / 39 / 49 / 61. Display headlines (49px+) should use the glass surface as a background — white or dark fills with the appropriate translucency — so the type itself feels luminous rather than flat.

Line heights are generous: 1.55 for body text, 1.15 for display. The airiness prevents the translucent layers from feeling cluttered and gives each text block room to breathe against the gradient backgrounds.

# Layout

The page is organized as a vertical stack of full-width sections, each potentially holding its own gradient canvas and frosted content cards. Sections use 80–120px vertical padding to create generous rhythm. Content max-width is 1120px, centered, with 24px horizontal padding on smaller screens.

Glass cards float within sections using CSS Grid or Flexbox, with 24px gaps. Cards should not touch viewport edges; the minimum margin is 20px on mobile, 40px on desktop. This ensures the background gradient is always visible around content, maintaining the layered-depth illusion.

A persistent top navigation bar uses the highest translucency — a thin frosted strip pinned to the viewport top. Its height is 56px with content vertically centered. Logo left, navigation links center or right, a single CTA button in the primary color. The bar's blur should be strong enough that scrolling content visibly softens behind it.

Below-the-fold sections alternate between lighter and slightly warmer background tokens to create natural section breaks without hard dividers. Each section transition should feel like turning a page in a well-made book — noticeable but not jarring.

# Elevation & Depth

Elevation in Meridian is primarily expressed through background blur intensity, not shadow. Three levels: low (12px blur, subtle separation), medium (24px blur, standard card), and high (40px blur, modal or featured panel). Shadow is used sparingly — a single soft drop shadow at most, offset 0 4px with 16px spread at 8% opacity, never stacked.

Glass surfaces carry three visual layers: the translucent fill, a 1px semi-transparent border on the light-facing edge, and an optional inner highlight (a second border or inset box-shadow at 6% white opacity) simulating light catching the top edge. Together these three signals create convincing frosted material.

The background gradient canvas sits beneath everything and uses large radial gradients — 400–600px circles of warm color placed art-directed positions. These create the color variation visible through frosted panels. In dark mode, gradients are more saturated but lower luminosity; in light mode, they are pale washes.

Hover and focus states increase blur by 4–8px and brighten the fill by 3–5% opacity, creating a subtle "lifting" sensation. Active states reverse this, pressing the surface down by reducing blur slightly. The tactile metaphor is consistent: hover raises, active presses.

# Shapes

Border radius follows Apple's philosophy of moderate rounding that feels soft but not playful. Small elements (tags, badges) use sm (6px). Standard cards and inputs use md (10px). Hero cards and feature panels use lg (14px). Large overlay containers use xl (20px). Pill-shaped elements (CTA buttons, toggles) use the pill token.

Shapes should never have sharp corners — even the most minimal rounding (sm) is sufficient. However, avoid over-rounding that shrinks usable content area or makes rectangular images look awkward inside containers. Image containers should match their parent card's radius.

Icon containers are circles at 40–48px diameter, filled with the glass token and bordered with the glass border token. Icons themselves use the primary color at 500 for standard emphasis, 300 for subtle decorative use.

Consistency matters: if sibling cards share a row, their radius, padding, and internal spacing must be identical. Variation should come from content and background gradient positioning, not from shape inconsistencies.

# Components

The primary CTA button uses the primary-500 fill with white text, set in SF Pro Text at 600 weight, 15px size. Padding is 12px vertical and 28px horizontal with a pill radius. On hover, it shifts to primary-600; on active, primary-700. A secondary button uses transparent fill with a 1px primary-400 border and primary-600 text.

Glass cards combine the background-glass token fill with backdrop-filter blur at the medium level (24px), the glass-border token as a 1px top/left border, and a gentle box-shadow for depth. Internal padding is 24–32px depending on content density. Cards may stack vertically with 16px gaps between them.

Input fields use a slightly more opaque glass fill (rgba bump of +10%) to ensure text legibility. Placeholder text uses the text-secondary token. Focus state adds a 2px outline in primary-500 with 2px offset, ensuring keyboard accessibility without disrupting the frosted aesthetic.

Modals and overlays use the highest blur level (40px) and xl border radius. A semi-transparent scrim (rgba(0,0,0,0.3) in light modes, rgba(0,0,0,0.5) in dark) sits between the page and the modal. The modal itself uses the glass token with slightly elevated opacity for readability.

# Do's and Don'ts

Do maintain generous spacing between glass elements — the translucency effect degrades when surfaces crowd each other. Give each panel at least 24px of breathing room so the blurred background is clearly visible.

Do test blur intensities across browsers and operating systems. Safari's backdrop-filter rendering differs subtly from Chrome's; ensure the frosted effect reads as intentional, not broken, on both. Consider providing a semi-opaque fallback for unsupported browsers.

Don't stack more than three translucent layers simultaneously. Beyond three, the visual noise overwhelms content and the blur performance cost spikes on lower-end devices. If complex layouts require more layers, make the deepest ones fully opaque.

Don't use pure white (#ffffff) or pure black (#000000) for text on translucent surfaces without first verifying contrast against the worst-case background scenario. Because translucent surfaces let background gradients through, check contrast against both the lightest and darkest gradient positions. Use a minimum 4.5:1 ratio against all possible backgrounds.

Don't animate blur radius or opacity continuously (as in looping animations). These properties are GPU-intensive and the performance ceiling varies widely. Prefer discrete state transitions (hover, focus) with 150–200ms durations using ease-out curves.