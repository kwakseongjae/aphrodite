---
name: "fitflow-hero"
version: "0.1"
description: "Thumb-zone mobile fitness hero with warm saffron palette and oversized touch targets"
colors:
  primary:
    "50": "#fef7ed"
    "100": "#fde9c6"
    "200": "#f9cf84"
    "300": "#f5b04a"
    "400": "#f09324"
    "500": "#e2780e"
    "600": "#b8570b"
    "700": "#8b3e0d"
    "800": "#6b3112"
    "900": "#4a2210"
  secondary:
    "50": "#f0fdf6"
    "500": "#34d37a"
    "900": "#0a3d1e"
  neutral:
    "0": "#ffffff"
    "50": "#faf9f7"
    "100": "#f2f0ec"
    "200": "#e4e1da"
    "300": "#c9c3b8"
    "400": "#a69d8e"
    "500": "#8a8070"
    "600": "#6e6558"
    "700": "#534c42"
    "800": "#3a352e"
    "900": "#1e1b17"
    "1000": "#000000"
  success:
    "500": "#2d9e5a"
  warning:
    "500": "#d4850a"
  error:
    "500": "#c4362b"
typography:
  display:
    family: "Space Grotesk"
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
  lg: "16px"
  xl: "24px"
  pill: "9999px"
metadata:
  variants:
    light:
      description: "Warm cream light mode with deep brown text"
      tokens:
        colors.background.primary: "#faf9f7"
        colors.text.primary: "#1e1b17"
    dark:
      description: "Rich dark mode with warm undertones"
      tokens:
        colors.background.primary: "#1e1b17"
        colors.text.primary: "#f2f0ec"
    brand-a:
      description: "Terracotta warm variant"
      tokens:
        colors.background.primary: "#fef7ed"
        colors.text.primary: "#3a2210"
    brand-b:
      description: "Sage-meets-saffron cool-warm variant"
      tokens:
        colors.background.primary: "#f2f4ee"
        colors.text.primary: "#1a2316"

---

# Overview

FitFlow Hero is a mobile-first fitness tracker landing experience engineered around natural thumb biomechanics. The primary CTA sits firmly in the bottom thumb arc, the zone most users reach effortlessly during one-handed phone operation. Every interactive element measures at least 48 by 48 pixels, exceeding platform accessibility guidelines without feeling bloated.

The warm saffron-amber primary palette evokes dawn energy and post-workout warmth rather than the clinical greens or electric neons typical of fitness apps. This is a motivational interface that feels like golden-hour sunlight, not a sterile gym dashboard. Paired with Space Grotesk for headlines, the typographic mood leans confident and geometric without coldness.

Information density stays deliberately low in the hero viewport. A bold stat or motivational phrase occupies the upper third, a secondary metric cluster fills the middle, and the bottom third belongs entirely to action. This vertical rhythm mirrors the natural scan pattern of a distracted mobile user glancing down at their screen mid-stride.

Four theme variants ensure the design performs across lighting conditions and brand contexts. Each variant maintains WCAG-AA contrast between text and background at all tiers. The dark variant uses warm charcoal rather than pure black to preserve the palette's inviting character in low-light environments.

# Colors

The saffron primary family anchors the entire system. At shade 500 the hue reads as a confident warm amber, distinct from generic orange or muted gold. Lighter shades carry the motivational warmth into backgrounds and subtle highlights, while darker shades provide rich contrast for text and iconography over pale surfaces.

Neutral tones are warm-tinted rather than pure gray. This prevents the sterility that cool grays introduce into a fitness context. The warm neutrals harmonize with the saffron primary so that UI chrome feels like part of the same energetic system rather than a separate structural layer.

The secondary green exists for success states and achievement indicators. It is cool enough to contrast clearly with the warm primary but desaturated enough to sit quietly alongside it. Error and warning tones follow the same restrained saturation philosophy to avoid visual anxiety in an already high-energy context.

Surface layering uses opacity-tinted neutrals rather than colored overlays. This keeps the background clean while still distinguishing cards, sheets, and elevated components from the base layer. In dark mode, elevated surfaces shift toward slightly lighter warm neutrals to simulate ambient light bounce.

# Typography

Space Grotesk provides the display voice with its geometric warmth and slightly rounded terminals. It carries the motivational tone of the hero headline without the aggressive hardness of condensed sans-serifs or the casualness of rounded display faces. At large sizes the letterforms feel determined and modern.

DM Sans handles all body text, labels, and stat values. Its round geometry echoes Space Grotesk's construction but at a quieter weight and narrower proportion. The pairing maintains visual family resemblance while clearly separating headline hierarchy from reading content.

Stat values in the fitness context demand large type. Calorie counts, step totals, and duration metrics render at display scale with generous letter-spacing so users can read them at arm's length during a run. Supporting labels sit directly beneath in smaller body type with muted color to maintain hierarchy.

Line height stays loose at 1.5 for body text and tight at 1.1 for display headlines. This contrast reinforces the visual weight difference between motivational copy and instructional text. Paragraph spacing is handled through spacing tokens rather than margin hacks.

# Layout

The hero viewport divides into three vertical zones mapped to thumb-reach ergonomics. The top zone sits outside comfortable thumb range and holds read-only content: greeting, date, motivational headline. The middle zone falls within moderate thumb reach and houses secondary stats or progress indicators. The bottom zone occupies the natural thumb arc and contains all primary actions.

Horizontal margins are generous at 24 pixels per side, leaving a narrow but comfortable content column. This constraint forces disciplined content editing and ensures text never crowds the screen edge on devices with curved display corners. Touch targets extend fully edge-to-edge within the action zone regardless of visual button width.

The CTA button itself spans the full content width and measures 56 pixels tall, exceeding the 48-pixel minimum. Its pill shape follows the natural curve of the thumb pad, making the tap target feel anatomically correct rather than arbitrarily rectangular. Secondary actions stack above with identical width but reduced visual prominence.

A bottom safe area of 32 pixels accounts for home indicators on modern devices. The CTA group floats above this zone in a fixed position so it remains accessible during scroll. Content above scrolls freely underneath with a subtle frosted-glass fade to maintain depth hierarchy.

# Elevation & Depth

Three elevation tiers define the system. Base level sits flat on the background. Cards and stat clusters elevate one tier with a soft warm shadow. The CTA group and any sheet or modal content occupy the highest tier with stronger shadow and slight scale emphasis.

Shadows use warm-tinted opacity values rather than pure black. In light mode a warm neutral at 12 percent opacity replaces the typical cold shadow. In dark mode shadows shift to slightly warmer tones to maintain the cohesive atmosphere. This prevents the floating-element look that cold shadows create on warm backgrounds.

Progress rings and achievement badges use a single-pixel inset shadow to simulate physical depth without adding visual weight. This micro-elevation differentiates interactive elements from flat illustrations. The effect is subtle enough to feel premium rather than skeuomorphic.

Background blur applies to the CTA container when content scrolls beneath it. An 8-pixel blur with a semi-transparent warm neutral fill creates a frosted-glass effect that keeps the action zone visually distinct from scrolling content. This layering technique preserves the thumb-zone's spatial identity even during aggressive scrolling.

# Shapes

Pill shapes dominate the action layer. The primary CTA, secondary buttons, and tag-style filters all use full pill rounding. This shape language references physical exercise equipment grips and creates a friendly, approachable interaction surface that aligns with thumb-pad geometry.

Cards and stat containers use a 16-pixel border radius, large enough to feel modern but not so round that content feels trapped. Inner elements like small stat blocks use 8-pixel rounding for nested hierarchy. The progression from sharp-edged backgrounds to softly-rounded cards to fully-rounded buttons establishes clear elevation-to-shape correspondence.

Icon containers follow the 24-pixel extra-large radius to form softened squares that suggest buttons without explicit borders. This shape bridges the gap between purely decorative icons and tappable elements, a useful ambiguity in a fitness interface where heart-rate or calorie icons might be informational or interactive depending on context.

# Components

The primary CTA button renders at 56 pixels tall with full pill rounding and the saffron 500 background. Its label uses Space Grotesk at semibold weight in the surface background color for maximum contrast. A subtle 4-pixel inner glow in saffron 300 creates dimensional warmth without affecting tap-target geometry.

Stat cards stack vertically in the middle zone, each measuring at least 48 pixels in minimum dimension for accessibility. Values render in Space Grotesk at 32 pixels. Labels sit 4 pixels below in DM Sans regular at 14 pixels. A thin saffron 200 border separates cards without heavy lines that would feel anxious.

Progress rings use a 4-pixel stroke with saffron 200 as the track and saffron 500 as the fill. The ring diameter measures 48 pixels minimum to stay readable at arm's length. Percentage values center inside the ring in body type at semibold weight.

The motivational headline occupies the upper third in Space Grotesk bold at 28 to 32 pixels depending on copy length. It sits on the primary background with no card container, keeping the top zone breathable. A subtle background gradient shifts from neutral 50 to primary 50 across the viewport to introduce warmth without competing with the content.

# Do's and Don'ts

Do place every interactive control in the bottom third of the viewport. The thumb arc is the only zone users can reach comfortably during movement. Even secondary actions like sharing or settings should live above the fold in the middle zone rather than in a top toolbar.

Don't use small or tightly spaced touch targets. Fitness interfaces are used in motion, often with sweaty hands or partial attention. Every tappable element must exceed 48 pixels in both dimensions with at least 8 pixels of clear space between adjacent targets. Visual compactness never justifies interaction difficulty.

Do leverage the warm palette to create emotional momentum. The saffron family should dominate the action layer so users associate the motivational color with forward progress. Reserve neutral tones for structural elements and the secondary green exclusively for achievement confirmations.

Don't mix warm and cool tones in the same elevation tier. A cool blue button on a warm amber background creates visual dissonance that undermines the cohesive energy the palette is designed to build. If a contrasting accent is needed, draw from the warm neutral range or use the secondary green sparingly.

Do test the interface one-handed while walking. Thumb-zone theory is only useful if validated in realistic usage conditions. If any primary action requires grip adjustment or hand repositioning, the layout has failed its core ergonomic promise. Adjust placement until the entire action flow completes with the thumb alone.