---
name: "longevity-editorial"
version: "0.1"
description: "Calm editorial presence for a longevity research clinic"
colors:
  primary:
    "50": "#f0fdf6"
    "100": "#dcfce9"
    "200": "#bbf7d4"
    "300": "#86efad"
    "400": "#4ade7f"
    "500": "#16a34a"
    "600": "#15803d"
    "700": "#166534"
    "800": "#14532d"
    "900": "#0a3d1e"
  neutral:
    "0": "#ffffff"
    "50": "#fafaf9"
    "100": "#f5f5f4"
    "200": "#e7e5e4"
    "300": "#d6d3d1"
    "400": "#a8a29e"
    "500": "#78716c"
    "600": "#57534e"
    "700": "#44403c"
    "800": "#292524"
    "900": "#1c1917"
    "1000": "#000000"
typography:
  display:
    family: "Instrument Sans"
    weight: 700
  body:
    family: "Instrument Sans"
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
  sm: "4px"
  md: "8px"
  lg: "12px"
  xl: "16px"
metadata:
  variants:
    light:
      description: "Default light mode — clean, airy editorial"
      tokens:
        colors.background.primary: "#ffffff"
        colors.text.primary: "#1c1917"
    dark:
      description: "Dark mode — deep and restful"
      tokens:
        colors.background.primary: "#0f0f0e"
        colors.text.primary: "#f5f5f4"
    brand-a:
      description: "Warm sage — grounded and organic"
      tokens:
        colors.background.primary: "#fafbf8"
        colors.text.primary: "#1a2e1a"
    brand-b:
      description: "Ivory cream — heritage editorial warmth"
      tokens:
        colors.background.primary: "#fdfcfa"
        colors.text.primary: "#2c1810"

---

# Overview

This system serves a longevity research clinic whose audience expects authority without clinical coldness. The design language draws from high-end editorial publishing — generous margins, restrained ornamentation, and a quiet confidence that lets content breathe. Every decision prioritizes calm readability over visual excitement, reflecting the measured, evidence-based ethos of longevity science itself.

Green was chosen deliberately as the primary hue, evoking growth, renewal, and organic vitality. The saturated 500 shade provides enough presence for interactive elements and wayfinding, while the deeper 800–900 tones ground headlines in seriousness. This is not the green of novelty — it is the green of endurance.

Instrument Sans was selected for its geometric clarity and contemporary editorial feel. It lacks the theatricality the user has consistently rejected in serif display faces, instead offering a quiet authority at large scales. Its uniform stroke width and open apertures support the clinical precision the subject demands without feeling sterile.

The spacing scale is intentionally generous. Longevity content often involves complex scientific concepts — readers need visual room to absorb and reflect. Tight layouts create cognitive pressure that runs counter to the contemplative tone the clinic requires.

# Colors

The green primary palette spans ten shades, giving designers fine-grained control over hierarchy and emphasis. The 500 shade at #16a34a sits at the intersection of vitality and restraint — saturated enough to read as intentional, muted enough to avoid feeling recreational. Lighter tints work beautifully for subtle backgrounds on cards and callouts, while the deepest shades anchor headlines with weight.

Neutral tones lean slightly warm through the stone spectrum rather than pure gray. This warmth prevents the palette from feeling medical or institutional. In editorial contexts, warm neutrals create a sense of approachable expertise — the difference between a welcoming consultation room and a sterile laboratory.

All four variants maintain WCAG-AA contrast ratios between primary text and background. The dark mode variant uses #0f0f0e rather than pure black, which reduces eye strain during extended reading sessions — appropriate for an audience likely to consume longer-form research content. The warm (#fafbf8) and ivory (#fdfcfa) variants offer subtle tonal alternatives that shift the emotional register without changing the underlying system.

# Typography

Instrument Sans serves as both display and body typeface, creating a unified typographic voice. At display scale, its geometric construction provides visual weight without ornamentation — headlines feel contemporary and assured. The weight 700 for display use provides sufficient contrast against body text without requiring a separate typeface.

Body text at weight 400 maintains excellent readability at standard sizes. The typeface's generous x-height and open counters ensure legibility across the extended reading sessions common to editorial content. Line heights should be set between 1.5 and 1.7 for body copy, tightening to 1.1–1.2 for display headlines.

Paragraph spacing should use the 8px base unit, with 24px between paragraphs in long-form content. This creates a rhythm that guides the eye without fragmenting the reading experience. Pull quotes and featured findings can use the display weight at 1.3× the body size to create emphasis without introducing a new typeface.

# Layout

Content width should be constrained to 680–720px for reading comfort, with generous padding on either side. The longevity audience includes older readers who benefit from consistent, predictable layouts. Asymmetric grids can introduce visual interest in feature layouts, but the reading column should remain centered and stable.

Vertical rhythm follows the spacing scale rigidly. Sections breathe at 96px intervals, subsections at 48px. This creates a calm, predictable scrolling experience that reduces cognitive load. Inline elements like buttons and tags use 32px height minimums to ensure comfortable touch targets across all age groups.

The layout should feel like a well-edited journal — each element has clear purpose and sufficient room to be understood independently. Negative space is not emptiness; it is the visual silence between notes that makes the composition coherent.

# Elevation & Depth

Depth in this system is subtle and functional rather than decorative. A single shadow level — achieved through a soft, wide spread at low opacity — lifts interactive elements like cards and buttons off the surface. This shadow should feel like afternoon light rather than harsh spotlight.

Dark mode inverts the elevation logic: lighter surfaces appear to sit above darker ones. This maintains the perceptual model while adapting to reduced ambient light conditions. The key principle is that elevation should communicate interactivity and hierarchy, never decoration.

Borders can supplement or replace shadows for definition, particularly in light mode. A 1px border at neutral-200 provides sufficient separation without the visual weight of a shadow. This approach aligns with the editorial print tradition where structure comes from rules and spacing rather than dimensional effects.

# Shapes

Border radii are kept deliberately small. The sm (4px) and md (8px) values provide just enough softening to feel modern without approaching the friendliness of consumer product design. This restraint matches the clinical authority the brand requires — approachable but not casual.

Larger radii (lg, xl) are reserved for feature elements like hero images and featured research cards. Even at these sizes, the rounding remains subtle. The goal is to soften the clinical edge of hard rectangles without introducing playfulness. Every rounding decision should feel inevitable rather than decorative.

Interactive elements like buttons and inputs use md rounding consistently. This creates a predictable interaction grammar — if something is clickable, its shape signals that affordance uniformly across the interface.

# Components

Buttons follow a clear hierarchy: primary actions use the green-500 fill with white text; secondary actions use a transparent background with green-700 text and a 1px border; tertiary actions use green-700 text alone. All variants maintain generous horizontal padding (24px minimum) to ensure readability and comfortable activation.

Cards for research summaries and team profiles use minimal visual treatment — white or near-white backgrounds with subtle neutral-200 borders, md rounding, and consistent 24px internal padding. The content within cards carries the visual weight; the card itself is merely a container that organizes space.

Navigation should be sparse and typographic. A simple horizontal bar with text links in body weight, with the active state indicated by green-700 color rather than background fills. This editorial approach to navigation prioritizes content hierarchy and reduces visual competition with page content.

# Do's and Don'ts

**Do** use the full green palette to create hierarchy within a single page. A research finding headline in green-900, a supporting label in green-600, and an accent line in green-500 creates depth without introducing competing hues. **Don't** introduce additional accent colors — the system's authority comes from chromatic restraint.

**Do** allow generous whitespace around key content blocks, especially research findings and data visualizations. **Don't** fill space for its own sake — if content doesn't serve the reader's understanding, remove it rather than style around it.

**Do** use Instrument Sans consistently across all touchpoints to build recognition and trust. **Don't** introduce decorative or display typefaces for emphasis — use scale, weight, and color instead.

**Do** test all color combinations against WCAG-AA, particularly in data visualization contexts where contrast failures commonly occur. **Don't** sacrifice accessibility for aesthetic preferences — the longevity audience includes older adults who may experience age-related vision changes.