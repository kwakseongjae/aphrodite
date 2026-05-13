---
name: "dev-portfolio-mono"
version: "0.1"
description: "A developer portfolio using Berkeley Mono as the sole typeface with a warm ochre accent palette"
colors:
  primary:
    "50": "#fefbf4"
    "100": "#fdf6e3"
    "200": "#f5e6c0"
    "300": "#e8cc8a"
    "400": "#d4a84b"
    "500": "#b8860b"
    "600": "#9a6f09"
    "700": "#7a5807"
    "800": "#5c4205"
    "900": "#3d2c03"
  neutral:
    "0": "#ffffff"
    "50": "#fafaf8"
    "100": "#f0efe8"
    "200": "#d9d8d0"
    "300": "#b5b3a8"
    "400": "#8a887c"
    "500": "#6b695f"
    "600": "#55534a"
    "700": "#3f3e38"
    "800": "#2a2a25"
    "900": "#1a1a16"
    "1000": "#0d0d0a"
typography:
  display:
    family: "Berkeley Mono"
    weight: 700
  body:
    family: "Berkeley Mono"
    weight: 400
spacing:
  "1": "4px"
  "2": "8px"
  "3": "12px"
  "4": "16px"
  "5": "20px"
  "6": "24px"
  "8": "32px"
  "12": "48px"
  "16": "64px"
rounded:
  sm: "2px"
  md: "4px"
  lg: "6px"
metadata:
  variants:
    light:
      description: "Default light mode with warm paper background"
      tokens:
        colors.background.primary: "#fafaf8"
        colors.text.primary: "#1a1a16"
    dark:
      description: "Dark mode with warm black background"
      tokens:
        colors.background.primary: "#0d0d0a"
        colors.text.primary: "#f0efe8"
    brand-a:
      description: "Warm cream variant with deep sepia text"
      tokens:
        colors.background.primary: "#fefbf4"
        colors.text.primary: "#2a2a25"
    brand-b:
      description: "Sage-tinted variant with warm charcoal text"
      tokens:
        colors.background.primary: "#f4f6f0"
        colors.text.primary: "#1e2018"
---

# Overview

This design system treats the developer portfolio as a typographic monospace canvas — every word shares the same rhythm and mechanical DNA. Berkeley Mono carries the entire experience, from hero headlines down to footer captions. The constraint is the identity.

The palette avoids typical developer blues and purples in favor of a warm ochre-gold accent that references aged brass, manuscript marginalia, and the amber glow of a CRT terminal. Warm neutrals ensure the page never feels cold or clinical, even in dark mode.

Four variants cover the essential presentation contexts. Light and dark modes serve default browsing. Brand-a leans into a parchment warmth for editorial or long-form portfolio layouts. Brand-b introduces a subtle sage tint for a more botanical, restrained mood.

# Colors

The primary accent palette centers on dark goldenrod (#b8860b at 500), spanning from pale cream at shade 50 to deep umber at shade 900. This hue family reads as authoritative without screaming — it complements code-heavy content rather than competing with it.

Neutral tones carry a deliberate warm bias. The near-black (#0d0d0a) and near-white (#fafaf8) both contain traces of yellow and green that prevent them from feeling sterile. Mid-tone grays maintain this warmth, ensuring subtle UI elements like borders and disabled states belong to the same tonal world.

Text-to-background contrast ratios all exceed WCAG AA requirements. Light mode pairs #1a1a16 on #fafaf8 at approximately 13.4:1. Dark mode pairs #f0efe8 on #0d0d0a at approximately 14.7:1. Brand variants maintain similar ratios, ensuring readability across every context.

# Typography

Berkeley Mono at weight 700 for display creates bold, architectural headlines with a monospaced cadence. Headlines feel like terminal prompts or code comments — inherently technical without needing decoration. Size hierarchy does the heavy lifting since weight variation is minimal.

Body text at weight 400 sets at 14px with 1.7 line-height to counteract monospace density. The generous leading prevents the uniform character widths from creating visual walls. Paragraph spacing uses the 8-step (32px) to give each block room to breathe.

Code captions, metadata labels, nav links, and button text all share the same family. Instead of switching typefaces, differentiation comes from size, letter-spacing, and color. This radical consistency becomes the portfolio's signature — a single voice speaking at different volumes.

# Layout

A 12-column grid with a maximum content width of 1080px centers the portfolio content. Wide margins (minimum 64px on desktop) frame the monospace text like a manuscript, giving the dense typographic texture space to exist without crowding.

Vertical rhythm follows a strict 8px baseline. The monospace nature of Berkeley Mono makes consistent alignment especially important — even small deviations become visible when every character shares the same width. Sections stack with 64px (spacing-16) gaps; components within sections use 32px (spacing-8) or 16px (spacing-4).

The layout avoids sidebar patterns. Single-column flows suit the typographic constraint best, letting the eye track naturally down the page without the complexity of multi-column monospace alignment.

# Elevation & Depth

Elevation is minimal by intent. A single shadow level (0 1px 3px rgba(13,13,10,0.08)) provides subtle lift for cards and interactive surfaces. In dark mode, the shadow shifts to a slightly warmer tone (rgba(0,0,0,0.25)) against the near-black background.

Border-based depth replaces heavy shadows. A 1px solid border in neutral-200 (light) or neutral-800 (dark) defines card edges and content boundaries. This approach feels more mechanical and honest — appropriate for a typeface that itself is transparent about its grid.

Interactive elevation changes are subtle. Hover states might shift from no shadow to the single shadow level, or brighten a border color. The goal is perceptible feedback without theatrical animation that would undermine the portfolio's restrained character.

# Shapes

Corner radii stay tight: 2px for small elements like tags and code blocks, 4px for cards and buttons, 6px for larger containers. These small radii soften edges just enough to feel modern without abandoning the geometric honesty of the monospace aesthetic.

Buttons follow a rectangular proportion with the 4px radius and generous horizontal padding (20px or spacing-5). The shape language deliberately avoids pill shapes or fully rounded elements — the portfolio's voice is precise, not playful.

Icon containers and avatar frames use the same 4px radius. Circular shapes are reserved for status indicators and decorative dots only, maintaining the angular, engineered character throughout.

# Components

Navigation sits as a horizontal row of Berkeley Mono text links with 24px (spacing-6) gaps. Active states use the primary-500 color; inactive states use neutral-500. No underlines, no backgrounds — just typographic weight and color shifting.

Project cards display as bordered rectangles with a 4px radius. Each card contains a title (700 weight, primary-500 color), a one-line description (400 weight, neutral-600), and a row of technology tags. Tags use a neutral-100 background with neutral-700 text at 12px size.

The hero section uses Berkeley Mono at 48px / weight 700 for the name, with a subtitle at 18px / weight 400 in neutral-500. A single horizontal rule in primary-200 separates the hero from content below. Contact links repeat the same monospace treatment, differentiated only by color.

# Do's and Don'ts

Do embrace the single-typeface constraint as a creative advantage. Use size jumps (48px to 18px to 14px) and color shifts to create hierarchy. Let letter-spacing adjustments (-0.02em for display, 0 for body) fine-tune the monospace rhythm at different scales.

Don't introduce a second typeface for any reason — not for quotes, not for legal text, not for decorative elements. If Berkeley Mono cannot serve the purpose, reconsider whether the element belongs in this portfolio at all.

Don't use the primary accent color for large background areas. The ochre-gold works as a highlight, a border accent, or text color against dark surfaces. Large fields of primary-500 overwhelm and fight the warm neutral foundation that makes the palette work.

Do test all four variants against real content, especially code snippets and project descriptions. Monospace text at body sizes requires careful line-length management (aim for 60-70 characters) to prevent reader fatigue.