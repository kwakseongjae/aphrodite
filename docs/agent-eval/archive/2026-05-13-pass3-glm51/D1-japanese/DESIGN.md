---
name: "wa-land"
version: "0.1"
description: "日本の美意識を反映したミニマル和風ランディングページ"
colors:
  primary:
    "50": "#f5f0eb"
    "100": "#e2d5c4"
    "200": "#d4c0a4"
    "300": "#c4a67e"
    "400": "#b08d5f"
    "500": "#8b6d47"
    "600": "#6d5436"
    "700": "#4f3c27"
    "800": "#33261a"
    "900": "#1a130d"
  neutral:
    "0": "#ffffff"
    "50": "#faf9f7"
    "100": "#f0eeea"
    "200": "#ddd9d2"
    "300": "#c4bfb5"
    "400": "#a69f92"
    "500": "#7a7368"
    "600": "#5c5650"
    "700": "#3d3935"
    "800": "#252320"
    "900": "#141210"
    "1000": "#000000"
  accent:
    "50": "#f0f4f7"
    "100": "#c8d5e2"
    "200": "#a1b9cf"
    "300": "#7a9dbb"
    "400": "#5e8aab"
    "500": "#3a6484"
    "600": "#2d4e67"
    "700": "#21394a"
    "800": "#16252f"
    "900": "#0b131a"
typography:
  display:
    family: "'Noto Serif JP', 'Shippori Mincho', 'Hiragino Mincho ProN', serif"
    weight: 700
  body:
    family: "'Noto Sans JP', 'Hiragino Kaku Gothic ProN', 'Yu Gothic', sans-serif"
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
  "32": "128px"
rounded:
  sm: "4px"
  md: "8px"
  lg: "12px"
metadata:
  variants:
    light:
      description: "Default — sumi-ink warmth on unbleached washi"
      tokens:
        colors.background.primary: "#faf9f7"
        colors.text.primary: "#1a130d"
    dark:
      description: "Night — charcoal stone under moonlight"
      tokens:
        colors.background.primary: "#141210"
        colors.text.primary: "#f0eeea"
    brand-a:
      description: "Kinari — warm unbleached paper with aged sumi"
      tokens:
        colors.background.primary: "#f5f0eb"
        colors.text.primary: "#252320"
    brand-b:
      description: "Asa — cool hemp cloth with iron-gall ink"
      tokens:
        colors.background.primary: "#f0f4f7"
        colors.text.primary: "#141210"
---

# Overview

This system captures the quiet restraint of Japanese aesthetic philosophy — *shibui* (渋い) and *ma* (間) — through generous whitespace, muted earth tones, and deliberate typographic contrast. The palette draws from sumi ink, unbleached washi paper, and aged bamboo rather than the expected indigo, creating warmth that feels centuries old yet imminently readable on screen.

The primary hue family is a warm umber-ochre (hue ~30°), chosen to evoke calligraphy ink diluted on handmade paper. This avoids the obvious indigo trope while maintaining deep Japanese roots through material association. Accent tones of muted steel blue reference *nando* (納戸) — the traditional Japanese closet color — providing cool counterpoint without competing.

Every spacing decision honors *ma*, the Japanese concept of meaningful emptiness. Sections breathe. Content floats in generous fields of negative space. Density is avoided except where intentional focus demands it.

# Colors

The primary palette ranges from near-white cream (#f5f0eb) through warm mid-tones to deep espresso (#1a130d). These shades carry implicit warmth without becoming overtly brown or orange. At 500, the tone sits at a comfortable reading weight for subtle graphic elements — decorative lines, borders, and rules.

Neutral tones run warm-gray, leaning slightly yellow to match the washi paper substrate. Pure white (#ffffff) is reserved for maximum contrast moments; most backgrounds use #faf9f7 or #f5f0eb to soften the digital edge and create a more tactile, paper-like feeling.

The accent palette is a desaturated steel blue drawn from traditional *aonibi* (青鈍) — a blue-gray used in Edo-period textiles. It provides enough chromatic difference to signal interactive elements without disrupting the overall calm. All text-background pairings meet WCAG-AA at 4.5:1 or above.

# Typography

Display headings use a Mincho (serif) face — specifically Noto Serif JP with Shippori Mincho as a character-rich fallback. The weight sits at 700, which reads as confident without shouting. Line height for display text is set generous at 1.8 to 1.9, honoring the vertical breathing room traditional to Japanese typesetting.

Body text employs a Gothic (sans-serif) stack led by Noto Sans JP. Weight 400 provides comfortable reading for longer Japanese passages. Character spacing is slightly loosened (+0.02em) to prevent the dense texture that Latin-oriented rendering defaults can create with CJK glyphs.

The interplay between Mincho headings and Gothic body creates a subtle material contrast — brush against stone, handwritten against carved. This pairing should feel inevitable rather than decorative.

# Layout

A single-column layout dominates, with content width constrained to 640–720px for optimal Japanese line length (20–30 characters per line). Wider hero sections may extend to 960px but maintain generous internal padding. The grid is intentionally simple; asymmetry comes from alignment choices rather than complex column structures.

Vertical rhythm follows a 24px baseline. Section spacing uses the 96px and 128px tokens to create unmistakable *ma* between content blocks. Horizontal margins are never less than 24px on mobile, scaling to 64px on desktop.

Elements should feel placed, not fitted. A single call-to-action button might sit alone in a vast field of cream. A short poem-like heading might anchor the upper third of a section with nothing else competing for attention.

# Elevation & Depth

Elevation is nearly absent by design. The aesthetic favors flat, material presence over layered depth. When distinction is needed between surfaces, a 1px rule in primary-100 or a subtle background shift (#faf9f7 to #f5f0eb) provides sufficient separation.

The one exception is a single shadow style — a soft, warm-toned box-shadow using rgba(26, 19, 13, 0.08) at 0 2px 16px — reserved for modal overlays and floating navigation elements. This shadow should feel like paper lifting slightly off a tatami surface, not a dramatic material elevation.

Focus states use the accent-500 tone (#3a6484) as a 2px outline offset by 2px, ensuring keyboard accessibility without introducing visual noise that contradicts the system's restraint.

# Shapes

Border radii are minimal — 4px for small elements like tags and inputs, 8px for cards and containers. The language is rounded but not circular; shapes should suggest the soft edges of handmade paper, not machine-perfect curves.

Full-width decorative elements may use no radius at all, maintaining the rectilinear discipline of traditional Japanese architecture. Buttons sit in the 4–8px range, wide and low, with generous horizontal padding (24px or more) reinforcing the horizontal emphasis of Japanese written language.

Circular elements are reserved for singular focal points — perhaps a mon (crest) or a single decorative motif. Overuse of circles breaks the rectilinear calm.

# Components

The primary button is a filled rectangle using primary-900 backgrounds with neutral-0 text. It carries no border-radius beyond sm (4px) and enough vertical padding to feel substantial without feeling Western. Hover states deepen slightly. The secondary button is ghost-style — transparent background with a 1px primary-500 border and primary-700 text.

Cards are simple bordered rectangles with 8px radius and no shadow. Content within cards aligns to a 24px internal grid. If a card contains an image, it bleeds to the card edge; text content sits within padded gutters.

The navigation bar is a minimal horizontal list of Gothic-weight links, possibly including a single Mincho wordmark on the left. No hamburger menus on desktop; on mobile, a simple slide-in panel with the same warm background and restrained typography.

# Do's and Don'ts

Do leave large areas of empty space intentionally. *Ma* is not waste — it is the silence between notes that makes music. Each section should feel like it has room to exist without touching its neighbors. Err on the side of too much space rather than too little.

Do maintain the warm undertone consistently. Even cool accent elements should feel warmed by their context. Avoid introducing pure saturated blues, reds, or greens that shatter the muted, earthy harmony.

Don't use indigo simply because the prompt mentioned 和風 (Japanese style). Indigo (*ai*) is one thread in a vast tapestry. The umber-ochre family chosen here connects to calligraphy, architecture, and tea ceremony materials with equal legitimacy and greater warmth.

Don't crowd the layout with multiple calls-to-action, dense card grids, or feature-heavy hero sections. One message per section. One primary action per viewport. Trust the user's attention by demanding less of it.