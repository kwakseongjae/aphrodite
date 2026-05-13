---
name: "brutal-tutorials"
version: "0.1"
description: "Raw brutalist coding tutorial site — monochrome violence with a single screaming yellow accent"
colors:
  primary:
    "50":  "#fffff5"
    "100": "#fffce6"
    "200": "#fff9cc"
    "300": "#fff4a3"
    "400": "#ffee7a"
    "500": "#ffe600"
    "600": "#ccb800"
    "700": "#998a00"
    "800": "#665c00"
    "900": "#332e00"
  neutral:
    "0":    "#ffffff"
    "50":   "#f5f5f5"
    "100":  "#d9d9d9"
    "200":  "#b3b3b3"
    "300":  "#8c8c8c"
    "400":  "#666666"
    "500":  "#404040"
    "600":  "#2a2a2a"
    "700":  "#1a1a1a"
    "800":  "#111111"
    "900":  "#0a0a0a"
    "1000": "#000000"
typography:
  display:
    family: "\"IBM Plex Mono\", monospace"
    weight: 700
  body:
    family: "\"IBM Plex Mono\", monospace"
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
  sm: "0px"
  md: "0px"
  lg: "0px"
metadata:
  variants:
    light:
      description: "Default light mode — white void with black type"
      tokens:
        colors.background.primary: "#ffffff"
        colors.text.primary: "#0a0a0a"
    dark:
      description: "Dark mode — absolute void with bleached text"
      tokens:
        colors.background.primary: "#0a0a0a"
        colors.text.primary: "#f0f0f0"
    brand-a:
      description: "Terminal green — CRT phosphor nostalgia"
      tokens:
        colors.background.primary: "#050a05"
        colors.text.primary: "#33ff33"
    brand-b:
      description: "Amber monochrome — original brutalist warmth"
      tokens:
        colors.background.primary: "#1a1200"
        colors.text.primary: "#ffcc66"

---

# Overview

This system rejects comfort. Every decision serves legibility through confrontation — a coding tutorial environment where nothing is softened, nothing is decorative, and nothing asks permission. IBM Plex Mono at a single weight handles every typographic need from page titles to inline code, eliminating hierarchy games and forcing structure to emerge from spacing alone.

The chromatic world collapses to two poles: black and white in dialogue, interrupted only by #ffe600 yellow as a single screaming accent. Yellow marks interactive elements, active states, and critical callouts. Nothing else earns color. This restraint is the point — a brutalist site should feel engineered, not designed.

Shapes have no radius. Borders are either absent or violently present at 2px or heavier. Elevation is a lie; depth comes from borders and contrast, never shadows. Components stack like concrete blocks.

# Colors

The neutral scale runs from pure white (#ffffff) through true black (#000000) in even increments, avoiding warmth or coolness. This indifference is intentional — the palette refuses to flatter. Primary yellow (#ffe600) sits at maximum chroma, chosen for its industrial signage associations and its ability to vibrate against both black and white.

In light mode, black type (#0a0a0a) on white yields a contrast ratio above 19:1. Dark mode flips to #f0f0f0 on #0a0a0a at approximately 18:1. Both exceed WCAG-AAA requirements. Yellow accent passes AA for large text on black (4.6:1) but is never used for body copy — only highlights, borders, and labels above 18px.

Brand-a pushes into CRT green (#33ff33 on #050a05), delivering approximately 14:1 contrast while evoking terminal environments. Brand-b uses amber (#ffcc66 on #1a1200) at roughly 12:1, referencing the original monochrome monitors that preceded design as a discipline.

# Typography

IBM Plex Mono carries the entire typographic load at two weights: 400 for body, 700 for headings. No italic. No variable width fallback. The font's technical DNA — drawn for developers, not designers — aligns with the system's refusal to pretend it was made by a traditional design process.

Line height stays rigid at 1.5 for body text and 1.15 for display headings. Letter-spacing is untouched at default. Code blocks inherit the same family but drop to 13px at 1.65 line-height, maximizing density without sacrificing scan-ability. Paragraph spacing is a consistent 24px between blocks. Headings gain presence through size jumps (32px → 20px → 16px) rather than weight variation.

The typographic scale is modular based on 16px: 11, 13, 16, 20, 24, 32, 48. No intermediate sizes. No optical adjustments. The grid is the grid.

# Layout

A single column, 720px maximum width, centered. Margins are gutters — 32px on desktop, 16px on mobile. There is no sidebar, no aside, no floating element. Content earns its position by being the only thing on the page.

Vertical rhythm follows the spacing scale strictly: 8px increments. Sections stack at 48px gaps. Articles separate at 64px. Lists compact to 4px between items. Code blocks sit inside 16px padding with a 2px top border in the accent color, marking their territory without subtlety.

Navigation is a horizontal list of monospace links at the top, underlined in yellow on hover. No icons. No logos. The site name is the first link, left-aligned. Breadcrumbs are slash-delimited paths rendered at 13px.

# Elevation & Depth

There are no shadows. Depth is communicated through border weight and background contrast. A 2px solid border in neutral-700 separates sections. A 4px left border in yellow marks the active navigation item or a highlighted code line. Cards do not exist; content blocks stack with borders between them.

On dark backgrounds, subtle differentiation uses a single step up the neutral scale: #111111 panels on #0a0a0a backgrounds. This is the only form of layering. Nothing floats. Nothing overlaps unless it is positioned absolutely, which is reserved for tooltips and toast notifications only.

The z-index scale is integer-simple: 0 for content, 10 for sticky headers, 20 for modals, 30 for toasts. There is no z-index conflict because there is almost nothing to stack.

# Shapes

All corners are 0px. Buttons are rectangles. Inputs are rectangles. Images are rectangles. The only shape is the rectangle and its aggressive honesty. Borders, when present, are 2px solid. No dashes, no dots, no decorative patterns.

Focus rings are 3px solid yellow, inset, with a 2px offset from the element edge. This visible accessibility marker doubles as a design element — brutalism should not hide its infrastructure. Form elements are bare: no rounded corners, no gradients, no inner shadows. A text input is a white rectangle with a black border and monospace text.

Horizontal rules are 2px solid black (light mode) or 2px solid #404040 (dark mode). They span the full content width and serve as the primary section divider. No ornamentation, no centered dots, no embossing.

# Components

**Navigation bar:** Full-width, sticky, 48px tall. Background matches the page background. Bottom border 2px solid in accent yellow. Links are 13px uppercase with 4px letter-spacing. Active link has a yellow underline.

**Code block:** Background #111111 in light mode, #000000 in dark. Monospace at 13px. Padding 16px. Top border 2px solid yellow. Line numbers omitted — the code is the code. Copy button is a yellow-outlined rectangle, top-right, labeled "COPY" in 11px uppercase.

**Tutorial step:** Numbered with yellow digits at 32px bold. Step body at 16px regular. Code examples inline use a neutral-100 background with no border, styled as `<code>` elements with 2px padding vertical and 4px horizontal.

**Button:** Rectangle, no radius. Two variants: filled (black bg, white text) and outlined (transparent bg, 2px black border, black text). Hover inverts. Active state adds the yellow accent as background. All transitions are instant — no animation, no easing.

# Do's and Don'ts

**Do** use the accent yellow sparingly. It marks the interactive, the active, the critical. If everything is yellow, nothing is. Reserve it for focus states, active navigation, primary actions, and code-block markers.

**Don't** introduce a second accent color. The system's authority comes from its constraint. A red for errors, a green for success — these are crutches. Use text labels and border weight instead. "ERROR:" in bold monospace on a bordered block communicates without chromatic hand-holding.

**Do** maintain the spacing grid. In a system without decorative elements, rhythm is the only design tool. A 3px gap or a 12px margin breaks the logic. Use the scale or break the system deliberately and consistently.

**Don't** add illustrations, icons, or decorative imagery. If a concept needs visual explanation, use a diagram built from the same monochrome geometry — rectangles, lines, and text. Stock photography, gradient meshes, and SVG illustrations belong to a softer world.