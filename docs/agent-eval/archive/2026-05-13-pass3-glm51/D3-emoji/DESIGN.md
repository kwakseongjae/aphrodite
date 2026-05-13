---
name: "wedding-celebration"
version: "0.1"
description: "An elegant wedding celebration landing page with warm terracotta and antique copper accents"
colors:
  primary:
    "50":  "#fef6f0"
    "100": "#fce8d8"
    "200": "#f5c9a8"
    "300": "#e8a474"
    "400": "#d4834e"
    "500": "#b5643a"
    "600": "#8f4e2e"
    "700": "#6b3a22"
    "800": "#4a2818"
    "900": "#2d1a0f"
  neutral:
    "0":    "#ffffff"
    "50":   "#faf8f5"
    "100":  "#f0ece6"
    "200":  "#ddd6cb"
    "300":  "#c4b9a8"
    "400":  "#a89a86"
    "500":  "#8a7d6d"
    "600":  "#6d6357"
    "700":  "#524a41"
    "800":  "#3a342e"
    "900":  "#25211d"
    "1000": "#000000"
typography:
  display:
    family: "'Cormorant Garamond', 'Georgia', serif"
    weight: 700
  body:
    family: "'Libre Baskerville', 'Georgia', serif"
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
  lg: "12px"
  xl: "20px"
  pill: "9999px"
metadata:
  variants:
    light:
      description: "Warm parchment with deep espresso text"
      tokens:
        colors.background.primary: "#faf8f5"
        colors.text.primary: "#25211d"
    dark:
      description: "Deep espresso background with parchment text"
      tokens:
        colors.background.primary: "#1a1714"
        colors.text.primary: "#f0ece6"
    brand-a:
      description: "Rose-tinged ivory with copper text"
      tokens:
        colors.background.primary: "#fef6f0"
        colors.text.primary: "#3a2818"
    brand-b:
      description: "Soft sage-cream with espresso text"
      tokens:
        colors.background.primary: "#f2f5f0"
        colors.text.primary: "#25211d"

---

# Overview

This design system captures the warmth and ceremony of a wedding celebration through a terracotta-copper palette that feels both seasonal and timeless. The system avoids predictable blush-and-gold conventions in favor of earthy, sun-baked tones drawn from Italian countryside estates — warm clays, antique coppers, and aged parchment. The result is a landing page that reads as intimate and considered rather than generically festive.

Every element aims for restrained elegance. The spacing is generous, the type hierarchy is steep, and decorative accents appear sparingly — a thin copper rule, a quietly animated botanical motif — so each ornamental moment carries weight. Celebration does not require visual noise; joy can be conveyed through warmth and intention alone.

The four variants allow the same foundational system to shift atmosphere significantly. Light and brand-a offer daytime luminosity with different warm undertones, dark evokes an evening candlelit reception, and brand-b introduces a subtle herbal freshness that complements outdoor or garden ceremonies.

# Colors

The primary palette centers on terracotta (#b5643a), a hue that bridges the warmth users expect from wedding materials with a grounded, artisanal quality that distinguishes this design from typical celebrations. The 50–200 range provides whisper-soft backgrounds for cards and sections, while 500 and above anchor headlines, buttons, and interactive elements. Terracotta reads as warm without being saccharine, as celebratory without being garish.

Neutral tones lean distinctly warm, with brown undertones rather than cool gray. This prevents the jarring contrast that pure grays would introduce against the terracotta system. Even the darkest text (#25211d) carries a faint warmth that keeps the page feeling cohesive, as though everything were printed on the same material.

Each variant shifts the background undertone while preserving the primary accent family. Brand-a pulls the background warmer toward peach-ivory, brand-b introduces the faintest sage cast for outdoor ceremonies, and dark mode inverts the parchment relationship for evening use. All four maintain WCAG-AA contrast between their respective text and background colors.

# Typography

Cormorant Garamond serves as the display face, chosen for its tall ascenders and refined hairlines that bring a calligraphic quality without crossing into casual territory. Its italic variants are particularly effective for pull quotes, couple names, or ceremonial phrasing. The weight 700 provides sufficient presence for hero text without requiring oversized font sizes, keeping the layout proportionate and breathable.

Libre Baskerville handles body text and functional UI elements. It shares Cormorant's serif heritage but is more restrained and highly legible at smaller sizes, making it ideal for event details, RSVP instructions, and navigation. The pairing is harmonious rather than contrasting — both faces belong to the same typographic family tree, which reinforces the cohesive, curated feel.

Line height is set generously at 1.7 for body text and 1.2 for display headings, creating the open, airy quality associated with formal stationery. Paragraphs are kept short, and vertical rhythm follows the spacing scale strictly to maintain visual calm.

# Layout

The page follows a single-column scroll narrative well suited to storytelling: hero with couple names, ceremony details, reception information, gallery teaser, and RSVP call-to-action. Each section occupies generous vertical space with ample padding, letting content breathe as it would on a beautifully typeset invitation card.

Content width is capped at 640 pixels for text-heavy sections, expanding to 960 for image galleries and the hero band. This narrower measure reinforces the intimate, personal quality of the occasion and prevents lines of text from stretching uncomfortably wide on larger screens. Sections alternate between full-bleed backgrounds and contained content blocks to create visual rhythm without complexity.

Responsive behavior is intentionally simple. The design degrades gracefully by reducing spacing values and collapsing the gallery grid from three columns to two to one, rather than reorganizing the page. The narrative order remains unchanged, preserving the ceremonial cadence regardless of viewport.

# Elevation & Depth

Elevation is used sparingly and organically. Cards for venue details, accommodation suggestions, and RSVP forms carry a single subtle shadow that suggests they are resting on the parchment surface — not floating above it. Shadows use warm gray tones rather than cool blacks, preserving the thermal coherence of the palette.

Background section transitions provide the primary sense of depth without explicit shadows. Alternating between the primary background and slightly tinted variants creates gentle layering, as though the page were assembled from different sheets of handmade paper. This is more effective than dramatic shadows for a design that needs to feel tactile and grounded.

Interactive elements such as buttons gain a slight shadow increase on hover and press, confirming user actions with physical feedback. The motion is subtle — perhaps 2 pixels of shadow expansion — because the page should feel responsive without becoming animated in a way that undermines its stateliness.

# Shapes

Rounded corners sit at the intersection of softness and structure. Cards and input fields use the md radius (8px), which removes visual harshness without appearing blob-like. Buttons use the same radius for consistency, though pill-shaped badges for date and location tags offer a gentle counterpoint.

Image containers use the lg radius (12px) to soften photographs while keeping them firmly rectangular, avoiding the trendy but distracting circle and blob crops. The slight rounding feels like a matte board edge — finished and intentional without calling attention to itself.

Decorative dividers use straight lines with a 2px stroke in the primary-200 color, sometimes terminating in a small botanical SVG flourish. These horizontal rules are the primary ornamental device and are used between major sections to mark transitions in the ceremony narrative.

# Components

The hero module displays the couple's names in Cormorant Garamond at large scale, centered, with a thin copper rule above and a ceremony date below in Libre Baskerville small caps. The background is either a soft gradient from primary-50 to neutral-0 or a subtly desaturated photograph with a warm overlay. No animations play here; the stillness is the statement.

RSVP cards are contained modules with a light border, generous internal padding, and clear input labels in the body font. The submit button uses primary-500 with white text, gaining a slightly deeper shade on hover. Error states use a muted brick red that harmonizes with the terracotta family rather than clashing with it.

Image galleries use a masonry-lite grid with consistent gap spacing. Each image has a subtle hover state — perhaps a slow scale to 1.02 — suggesting interactivity without visual drama. Lightbox overlays use a warm dark background at 90% opacity rather than pure black, keeping the experience tonally consistent.

# Do's and Don'ts

**Do** let whitespace do the decorative work. A wide margin around the couple's names communicates significance more effectively than ornamental borders or flourishes ever could. Trust that restraint reads as confidence and elegance.

**Do** keep animation minimal and slow. If any elements animate — a fade-in on scroll, a gentle parallax on the hero — the duration should be 600ms or longer. Quick movements break the contemplative atmosphere that the palette and typography have established.

**Don't** introduce new hue families. No cool blues, no bright greens, no purples. Even link text and focus indicators should use primary-600 or primary-700 rather than defaulting to blue. Color discipline is what separates a designed system from an assembled page.

**Don't** use pure black (#000000) for text against warm backgrounds. The neutral-900 (#25211d) carries enough warmth to feel integrated while still providing strong readability. Pure black creates tiny but perceptible visual holes in an otherwise warm composition.

**Don't** overcrowd the hero section with every detail at once. The hero holds names, date, and location at most. Everything else lives downstream in the scroll narrative, revealed at the viewer's pace.