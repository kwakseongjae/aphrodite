---
name: "fintech-pricing-v2"
version: "0.1"
description: "Conversion-focused fintech pricing page with three tiers and trust-coded warm palette"
colors:
  primary:
    "50": "#faf8f5"
    "100": "#f0ebe3"
    "200": "#ddd3c4"
    "300": "#c4b49d"
    "400": "#ad9575"
    "500": "#8a6d3b"
    "600": "#6b5530"
    "700": "#544026"
    "800": "#3d2f1d"
    "900": "#2a1f14"
  neutral:
    "0": "#ffffff"
    "50": "#f8f7f5"
    "100": "#efeee9"
    "200": "#d8d5cc"
    "300": "#b5b0a3"
    "400": "#918a7a"
    "500": "#736c5e"
    "600": "#5c564a"
    "700": "#48433a"
    "800": "#3a3630"
    "900": "#1e1c18"
    "1000": "#0e0d0b"
  accent:
    "50": "#f2faf7"
    "100": "#d6f0e5"
    "200": "#a8ddb8"
    "300": "#6bc488"
    "400": "#3dab62"
    "500": "#1e7a3e"
    "600": "#166232"
    "700": "#104a26"
    "800": "#0b341a"
    "900": "#072312"
typography:
  display:
    family: "Fraunces"
    weight: 700
  body:
    family: "Outfit"
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
  sm: "4px"
  md: "8px"
  lg: "12px"
  xl: "16px"
  "2xl": "24px"
  full: "9999px"
metadata:
  variants:
    light:
      description: "Default light mode — warm parchment with deep bronze text"
      tokens:
        colors.background.primary: "#faf8f5"
        colors.text.primary: "#2a1f14"
    dark:
      description: "Dark mode — deep charcoal-brown with warm cream text"
      tokens:
        colors.background.primary: "#141210"
        colors.text.primary: "#efeee9"
    brand-a:
      description: "Warm ivory variant — heritage financial aesthetic"
      tokens:
        colors.background.primary: "#f5f0e8"
        colors.text.primary: "#1e1608"
    brand-b:
      description: "Cool stone variant — modern minimal fintech"
      tokens:
        colors.background.primary: "#f5f3ef"
        colors.text.primary: "#1a1815"
---

# Overview

This pricing page is designed around the psychology of trust in financial products. Instead of reaching for the typical fintech blue, the palette draws from warm amber-bronze tones historically associated with aged institutions, leather-bound ledgers, and physical currency — materials that quietly signal permanence without nostalgia.

The three-tier structure (Free, Pro, Enterprise) follows a deliberate visual weight escalation. Free is understated and neutral. Pro is elevated with a subtle border and the warm primary accent. Enterprise is the focal anchor — slightly larger, centered, with the accent green CTA breaking the warm monotone to draw the eye exactly where conversion matters most.

Fraunces as the display typeface introduces an optical-size-aware serif that feels editorial and authoritative, while Outfit handles all body and UI text with a clean geometric neutrality. This pairing creates a clear hierarchy: serif for decisions, sans-serif for details.

The feature comparison table uses alternating row tints and tight spacing to maintain scannability across potentially dozens of rows. Check marks in the accent green provide micro-moments of positive reinforcement as users scan toward their target tier.

# Colors

The primary palette centers on bronze-amber (500: #8a6d3b), a hue that reads as neither warm nor cool but distinctly "established." This is not a UI color — it is a trust signal. The primary appears only in the Pro tier card border, the display headings for pricing figures, and selected interface moments. Overuse would erode its authority.

Neutral tones are slightly warm-shifted across the entire scale. Even in dark mode, pure black is avoided in favor of deep warm browns. This prevents the clinical coldness that can undermine perceived warmth in financial interfaces. The warmest neutrals appear in backgrounds; the darkest in text and structural elements.

The accent green (#1e7a3e) is reserved almost exclusively for conversion actions — the primary CTA button on each pricing card, check marks in the comparison table, and success-state messaging. Its scarcity gives it urgency. When a user sees this green, their brain should register "action opportunity."

Feature comparison rows alternate between the primary background and a barely-there shift to neutral-50, providing rhythm without distraction. The Enterprise tier header uses a whisper of the accent green in its background tint to subconsciously associate it with the CTA color.

# Typography

Fraunces operates at two optical sizes: a large display weight for the hero pricing figures and card prices, and a slightly smaller size for section headings like "Choose your plan." Its variable weight axes allow subtle differentiation between the hero statement (weight 900) and tier names (weight 700) without changing typefaces.

Outfit carries the entire informational layer — feature list items, comparison table cells, body copy, and interface chrome. Its geometric structure pairs cleanly with Fraunces's more expressive forms. Body text runs at 16px with 1.6 line-height for comfortable reading of feature descriptions.

Pricing figures use tabular figures in both typefaces to ensure column alignment across the three tiers. Currency symbols are set at 60% of the figure size and optically aligned to the baseline, preventing the common mistake of visually oversized dollar signs that distort perceived value.

Label text for tier names uses letter-spacing of 0.04em in Outfit uppercase at 12px, creating a classification layer that sits above the price but below the page title in the visual hierarchy. Badge text for "Most popular" uses the same treatment but in the accent green.

# Layout

The page follows a five-section vertical flow: trust header with social proof, tier selector with billing toggle, three pricing cards, feature comparison table, and a closing FAQ accordion. Each section occupies full viewport width with content constrained to a 1200px centered container.

The three pricing cards are arranged in a responsive grid — three columns at desktop, stacked at mobile with Pro appearing first. The Pro card receives 8px of additional vertical padding and a 1px border in the primary-400 shade, creating subtle visual dominance without garish scaling or shadow theatrics.

The billing toggle (monthly/annual) sits above the cards with the annual option showing savings prominently. When toggled, prices animate with a vertical slide and cross-fade, making the value shift tangible and noticeable rather than a silent swap.

The comparison table uses sticky column headers and a sticky tier-name column on mobile, ensuring users never lose orientation while scrolling through features. Row grouping with labeled section dividers (Core Features, Advanced Analytics, Support, Security) breaks the table into digestible chunks.

# Elevation & Depth

Elevation is restrained and systematic. The Free card has no shadow and sits flush with the background — it is the baseline. The Pro card uses a single shadow layer (y: 2px, blur: 12px, spread: 0, opacity: 0.06) in a warm-tinted black. The Enterprise card adds a second shadow layer at a wider spread for a lifted, premium feel.

The CTA buttons invert this pattern. The Free tier button is outlined with no fill. The Pro tier button uses a solid fill in the primary-500 amber, and the Enterprise button uses the accent green. Each button has an identical border-radius (8px) and height (48px), differentiated only by fill treatment and color.

On hover, cards shift up 2px with a shadow increase of 4px on the primary shadow layer. This micro-interaction provides spatial feedback that rewards exploration. The active/selected billing toggle receives a subtle inner shadow to indicate a depressed state.

Modals and overlays (such as the Enterprise contact form) use a backdrop blur of 8px over a dark neutral scrim, with the modal surface elevated at the highest shadow tier. This prevents the pricing context from disappearing entirely during conversion flows.

# Shapes

Border-radius follows a consistent scale. Cards use 12px (lg), buttons use 8px (md), badges use full rounding (9999px), and table cells use 4px (sm). This creates a clear reading of container hierarchy: larger radii indicate primary containers, smaller radii indicate nested elements.

The billing toggle pill uses full rounding with a sliding indicator circle. The toggle track is neutral-200 in light mode with the active segment filled in primary-500. The sliding indicator is a white circle with a 2px shadow, providing a tangible, physical quality to the interaction.

Tier badges ("Most popular," "Best value") are pill-shaped with full rounding, positioned absolutely at the top-center of their respective cards, overlapping the top edge by half their height. This creates a tab-like visual that associates the badge with the card beneath it.

The comparison table cells have no border-radius — they are structural, not interactive. This contrast between the rounded cards and squared table reinforces the boundary between decision space (rounded, warm) and information space (square, neutral).

# Components

The pricing card is the primary component, containing a tier label, price display, feature list, and CTA button. Each card has a consistent internal layout: 32px padding all sides, tier label at top, price below with 24px gap, feature list with 12px row spacing, and CTA pinned to the bottom with 24px top margin.

The billing toggle is a compound component with a label group and a switch. The switch has smooth transitions on both the sliding indicator and the background fill. Annual pricing displays a savings badge ("Save 20%") positioned inline with the toggle, using the accent green text on a transparent background.

The comparison table component uses a fixed first column for feature names and scrollable subsequent columns for tier values. Check marks are 16px SVG icons in accent-500, dashes for excluded features are in neutral-300, and "Custom" text appears in primary-600 for Enterprise-specific features.

The FAQ accordion at page bottom uses Outfit at 15px for questions and 14px for answers, with a 2px primary-300 left border on the open state. Only one answer is open at a time, reducing cognitive load and preventing the page from growing unmanageably long.

# Do's and Don'ts

Do anchor the visual hierarchy on the Pro tier as the conversion target — it should feel like the obvious choice without making Free feel crippled or Enterprise feel exclusionary. The warm primary border and centered position achieve this through subtlety rather than spectacle.

Don't use the accent green for anything that is not a conversion action or a positive indicator. If green appears on decorative elements, informational text, or secondary UI, its pull on the CTA buttons dissipates. The same applies to the primary amber — it signals "important pricing information," not "click here."

Do ensure that the feature comparison table highlights differences rather than similarities. Features present in all three tiers should be listed but visually muted (neutral-400 text), while tier-exclusive features use full-contrast text. This helps users locate differentiators quickly.

Don't animate prices with horizontal slides, bounces, or scale transforms that could trigger distrust in financial contexts. Price changes should feel factual and stable — the vertical slide-and-fade on billing toggle is the most movement acceptable. Trust in fintech is built partly through visual sobriety.

Do test the comparison table with real feature data, not placeholder rows. The table's value is proportional to its accuracy. Don't ship with lorem features — users will question the product's maturity if the pricing page itself seems unfinished.