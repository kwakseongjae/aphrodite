---
name: "seoul-walnut-studio"
version: "0.1"
description: "Editorial portfolio for an independent walnut furniture maker in Seoul — photo-dominant, calm, materially honest"
colors:
  primary:
    "50": "#faf7f2"
    "100": "#f0ebe0"
    "200": "#ddd5c4"
    "300": "#c4b89e"
    "400": "#a89876"
    "500": "#8a7a5f"
    "600": "#6d6049"
    "700": "#564a38"
    "800": "#3d3528"
    "900": "#2a2419"
  neutral:
    "0": "#ffffff"
    "50": "#fafafa"
    "100": "#f0f0ef"
    "200": "#d4d4d1"
    "300": "#a8a8a4"
    "400": "#787874"
    "500": "#525250"
    "600": "#3a3a38"
    "700": "#2a2a28"
    "800": "#1a1a18"
    "900": "#0e0e0c"
    "1000": "#000000"
typography:
  display:
    family: "Newsreader"
    weight: 700
  body:
    family: "Outfit"
    weight: 300
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
  "32": "128px"
  "40": "160px"
  "48": "192px"
  "56": "224px"
rounded:
  sm: "2px"
  md: "4px"
  lg: "6px"
metadata:
  variants:
    light:
      description: "Default light mode — warm gallery white"
      tokens:
        colors.background.primary: "#faf9f6"
        colors.text.primary: "#1a1710"
        colors.text.secondary: "#5c5647"
        colors.background.secondary: "#f0ede5"
        colors.border.default: "#ddd8cb"
    dark:
      description: "Dark studio mode — charred timber"
      tokens:
        colors.background.primary: "#0f0e0b"
        colors.text.primary: "#e8e4da"
        colors.text.secondary: "#9a9487"
        colors.background.secondary: "#1a1814"
        colors.border.default: "#33302a"
    brand-a:
      description: "Gallery exhibition — cool off-white"
      tokens:
        colors.background.primary: "#f6f5f2"
        colors.text.primary: "#221f18"
        colors.text.secondary: "#6b6558"
        colors.background.secondary: "#eceae4"
        colors.border.default: "#d8d5cc"
    brand-b:
      description: "Workshop palette — raw linen"
      tokens:
        colors.background.primary: "#f0ece3"
        colors.text.primary: "#0f0d08"
        colors.text.secondary: "#4a453a"
        colors.background.secondary: "#e4dfd3"
        colors.border.default: "#ccc6b8"

---

# Overview

This system serves a single furniture maker who works in solid walnut from a workshop in Seoul. The design treats every page as a gallery wall: photography leads, text follows, and spacing carries the rhythm. The palette is keyed to walnut's natural colour — a warm grey-brown ochre — but countered with cooler stone neutrals so the warmth doesn't become cloying.

The typographic register is editorial rather than artisanal. Newsreader at display weight provides a sharp contemporary serif presence, while Outfit at light weight keeps body text airy and legible. This avoids the heritage-craft cliché of Renaissance serif faces while maintaining the dignity appropriate to considered object-making.

The spacing scale extends to 224 px because magazine-paced portfolios require real breathing room between projects. Tight spacing collapses the visual rhythm and makes photography feel cramped. The rounded tokens stop at 6 px — joinery is precise, not soft, and the corners should reflect that material honesty.

Four variants cover the range of contexts: default light for general browsing, dark for evening or studio viewing, brand-a for a cooler gallery feel, and brand-b for a warmer workshop atmosphere. All four maintain WCAG-AA contrast between text and background.

# Colors

The primary palette runs from a near-white cream at shade 50 to a deep warm black at shade 900. The 500 value sits at a muted grey-brown that reads as walnut without being literal — it's the colour of a well-worn chisel handle, not a raw plank. This avoids the common trap of picking a medium wood tone that competes with the photography.

Neutral colours lean slightly warm at the light end and cool slightly at the dark end, mimicking how materials shift under different light conditions. This subtlety prevents the sterile feeling of pure grey while keeping the palette versatile enough for UI elements, borders, and secondary text.

Each variant adjusts the background warmth and text depth together. The light variant uses a near-white gallery tone at #faf9f6 against deep warm text at #1a1710, yielding a contrast ratio above 15:1. The dark variant inverts this with a charred-timber black at #0f0e0b and warm off-white text at #e8e4da, clearing 13:1. Both brand variants maintain similar ratios with slight temperature shifts.

# Typography

Newsreader is a contemporary optical-size serif designed for editorial use. Its heavier weights carry headlines with authority without resorting to decorative tricks. It reads as working-studio rather than heritage-artisan, which matches a maker who produces furniture — not curios. The display weight of 700 gives titles enough presence to anchor pages without competing with photography.

Outfit at weight 300 keeps body text light and generous. The thin weight creates an airy texture that recedes behind images, which is exactly what editorial portfolio prose should do. Readers see the work first and read the context second.

Line-height should sit at 1.5 for body and 1.15 for display. Letter-spacing for display headings can open slightly — +0.02em — to give titles room to breathe at large sizes. Body text remains at default tracking.

# Layout

The structure follows a simple editorial cadence: hero statement, selected work grid, brief about, plain contact. The hero is a single line of display text — the maker's name or a seasonal statement — with generous padding above and below. No tagline carousel, no animated intro.

The selected work grid uses image-dominant cards at roughly 3:2 aspect ratio. Each card shows a single piece with a narrow text rail beneath: piece name, material, year. No hover reveals, no overlay captions. The grid spacing uses the 40 or 48 token — 160 to 192 px between project rows — so each piece has room to exist on its own.

The about section is short: one photograph of the maker or workshop, two paragraphs of biographical context, then a clean break. Contact is plain text — email address, phone number, workshop address in Seoul. No form widget, no contact button that opens a modal. A person you call, not a brand you submit tickets to.

# Elevation & Depth

Elevation in this system is minimal. The work speaks through photography, not through shadow gymnastics. Cards sit flat on the background. Depth comes from tonal contrast between sections — a slightly darker or lighter background band — rather than from drop shadows.

If subtle separation is needed, a single-pixel border at the default border token provides enough distinction without visual noise. Shadows, when used at all, should be tight and diffuse — 0 2px 8px at 8% opacity — suggesting the object is resting on a surface rather than floating above it.

The dark variant can afford slightly more shadow depth because dark backgrounds absorb ambient light differently, but the principle holds: the furniture has dimension; the website does not need to compete.

# Shapes

Rounded tokens max out at 6 px. This is a deliberate rejection of the soft-cornered SaaS aesthetic. Furniture joinery is precise — mortise and tenon, dovetail, finger joint — and the interface corners should reflect that precision. A 2 px radius softens just enough to prevent visual harshness without rounding into friendliness.

Full-bleed images have no radius at all. They meet the viewport edge cleanly, the way a well-fitted panel meets a frame. Inline images within content can use the small radius for subtle softening.

Buttons and interactive elements use the 4 px token. Larger radii on interactive elements is acceptable if it serves touch-target comfort, but should never exceed 6 px. Circles are reserved for favicon or avatar contexts only.

# Components

The project card is the primary component: a full-width or half-width image with a text block containing the piece name in display serif, material in body light, and year in a smaller muted tone. Cards do not lift on hover. The image is the interaction — clicking it enters the project detail.

Navigation is minimal text links in body weight: Work, About, Contact. Left-aligned on desktop, centered on mobile. No sticky header — the page scrolls as a continuous editorial spread. A simple top anchor or back-to-top link serves orientation.

The project detail page is a vertical scroll of large images interspersed with short text passages. No lightbox, no carousel. Each image is as wide as the viewport allows, with consistent side margins. Captions sit below in body text at small size. The pacing is magazine-like: image, pause, text, image, pause — letting the reader absorb each piece before moving to the next.

# Do's and Don'ts

Do let photography dominate every page. Use full-bleed images wherever the layout permits. The furniture is the content; the website is the gallery. Do maintain generous spacing between sections — the 160 to 224 px range exists precisely for this pacing.

Do use the warm grey-brown primary as an accent for interactive states, small labels, and hover treatments. It connects the interface to the material without overwhelming the photography.

Don't add SaaS-style metrics rows listing years of experience, pieces completed, or client counts. The work is the metric. Don't use form widgets for contact. An email address, a phone number, and a workshop address are sufficient and more honest.

Don't introduce blue as an accent colour unless the maker's existing brand identity demands it. Don't use Renaissance or revival serif faces — they signal heritage craft in a way that underserves a working contemporary studio. Don't round corners beyond 6 px. Don't animate section entrances or use parallax scrolling effects. The stillness is the point.