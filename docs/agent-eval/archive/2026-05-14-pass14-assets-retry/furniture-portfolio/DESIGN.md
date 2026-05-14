---
name: "heeyang-studio"
version: "0.1"
description: "Portfolio for an independent walnut furniture maker in Seoul — calm, photo-dominant, editorial, contemporary serif with optical sizing"
colors:
  primary:
    "50": "#f2f0ec"
    "100": "#e2ddd6"
    "200": "#c4b9a8"
    "400": "#8d7b64"
    "500": "#6d5e49"
    "600": "#574a38"
    "900": "#3a3025"
  neutral:
    "0": "#ffffff"
    "50": "#f6f6f5"
    "100": "#ececea"
    "200": "#d5d5d0"
    "400": "#8e8e86"
    "500": "#6d6d64"
    "700": "#3d3d36"
    "900": "#1a1a17"
    "1000": "#000000"
  accent:
    "50": "#f2f0ee"
    "500": "#8a7d6d"
    "900": "#4a4135"
typography:
  display:
    family: "Source Serif 4"
    weight: 300
  body:
    family: "Inter"
    weight: 300
spacing:
  "0": "0px"
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
  "56": "224px"
rounded:
  none: "0px"
  xs: "2px"
  sm: "4px"
  md: "6px"
metadata:
  variants:
    light:
      description: "Default light mode — cool gallery white"
      tokens:
        colors.background.primary: "#f6f6f5"
        colors.text.primary: "#1a1a17"
        colors.background.secondary: "#ececea"
        colors.text.secondary: "#6d6d64"
    dark:
      description: "Dark mode — graphite studio"
      tokens:
        colors.background.primary: "#141413"
        colors.text.primary: "#ececea"
        colors.background.secondary: "#1f1f1c"
        colors.text.secondary: "#8e8e86"
    brand-a:
      description: "Gallery wall — warm near-white"
      tokens:
        colors.background.primary: "#f9f8f5"
        colors.text.primary: "#2a2a24"
        colors.background.secondary: "#efede8"
        colors.text.secondary: "#7a7a6e"
    brand-b:
      description: "Workshop interior — dim warmth"
      tokens:
        colors.background.primary: "#242018"
        colors.text.primary: "#ddd9cf"
        colors.background.secondary: "#302b22"
        colors.text.secondary: "#9a9487"

# Overview

Heeyang Studio is the online portfolio of an independent furniture maker working exclusively in solid walnut, based in Seoul. The site exists to present completed work with the clarity and restraint of a gallery catalogue — no sales pipeline, no client logins, no craft-disruption rhetoric.

The design borrows its pacing from Korean and Scandinavian design journals: generous white space between projects, a narrow text rail, and photographs that fill the viewport edge to edge. Navigation is minimal — Work, About, Contact — because the furniture should do the persuading.

This is a site for someone who will visit on a Saturday morning with coffee, scroll slowly, and reach for the phone number when they are ready to commission a piece. Every decision — type weight, spacing scale, corner radius — serves that unhurried register.

# Colors

The palette is built on the grey-brown ochre of raw walnut heartwood, but the emphasis sits closer to neutral than to warm. Pure warmth would fight the photography; instead the primary range — running from #f2f0ec through #3a3025 — reads as a quiet, desaturated clay that recedes behind the material images.

Light variants use cool-toned backgrounds (#f6f6f5, #f9f8f5) rather than parchment yellows. The slight green-grey undertone keeps the page feeling like a gallery wall rather than a bakery. Dark variants invert into graphite with a faint olive cast, echoing a dimly lit workshop at dusk.

There is no accent colour in the conventional sense. Call-to-action elements inherit the primary 500 or 900 shade. If a single interactive element needs to signal "click here," it does so through underline or arrow icon, not through hue contrast.

# Typography

Source Serif 4 at weight 300 is the display face — a contemporary text serif with optical sizing that adjusts stroke contrast and spacing across the size range. It reads as honest and workmanlike rather than editorial-fashionable, fitting for a craftsperson's workshop. The optical-sizing axis means headlines at 48–72 px deploy the "Display" optical master with appropriate contrast, while smaller settings gracefully shift toward the "Text" master. Headlines sit at large sizes with generous letter-spacing, letting the strokes breathe against the neutral backgrounds.

Body text is Inter at weight 300 — noticeably lighter than the default 400. The combination of a clean contemporary serif and a thin sans body creates a consistent optical weight across the page, avoiding the common trap of heavy display + light body which fractures the vertical rhythm.

Korean text (the maker's name, section headings in Hangul) is set in the system default Gothic, which Inter pairs with cleanly. No attempt is made to force a Korean web font; the system stack is more reliable and loads instantly.

# Layout

The page follows a single-column editorial flow with a maximum content width of 1080 px. Photographs break out to full viewport width at select moments — the hero, and one or two mid-page interludes — while text and captions stay within the narrow rail.

Vertical spacing is the primary compositional tool. The scale tops out at 224 px, used between major project sections. This is deliberately excessive by web conventions but correct for magazine pacing: each piece of furniture deserves the visual silence of a page turn.

The work grid is a masonry-style layout with images at their natural aspect ratios, 4 px gaps, and no visible cards or borders. Captions sit below each image in small body text, left-aligned, with the piece name in primary 900 and the year in neutral 400.

# Elevation & Depth

There is almost no elevation. The portfolio avoids drop shadows, frosted glass, or layered card surfaces. The furniture itself provides all the material depth the page needs. The only shadow on the entire site is a subtle 0 1px 2px rgba(0,0,0,0.06) under the fixed navigation bar, just enough to separate it from scrolling content.

Interactive states — hover on a project image, focus on a text link — use a gentle opacity shift (0.7) or a 2 px underline in primary 500. There are no floating cards, no modals, no overlapping layers. Depth is achieved through spacing and photographic full-bleeds, not through z-index.

The dark variant does not add glow effects or neon accents. It simply inverts the value range, maintaining the same flatness and letting the walnut photographs carry all the visual warmth.

# Shapes

Corner radii are constrained to 0–6 px across the entire site. This mirrors the maker's aesthetic — sharp joinery, tight reveals, no rounded softening. Buttons and interactive elements use 4 px radius, enough to indicate clickability without softening the overall geometric register.

Image containers have no border-radius at all when they are full-bleed. In the work grid, images receive 2 px radius — barely perceptible, but enough to catch the eye on close inspection and signal intentional framing.

The contact section uses a simple horizontal rule (1 px, neutral 200) rather than a boxed container. Shapes here mean boundaries and divisions, never containers or cards.

# Components

Navigation is a fixed top bar: studio name on the left (Source Serif 4, 18 px), three links on the right (Inter 300, 14 px), all in primary 900 on light or neutral 200 on dark. The bar is 64 px tall with the content vertically centred. On scroll, a hairline border appears beneath it.

The hero section is a full-viewport photograph with no overlay text — just the image, bleeding to all edges. The first text element appears 128 px below the scroll fold, a single line introducing the maker.

The work grid presents 6–10 pieces, each as a large image with a two-line caption beneath. There is no filtering, no category tagging, no modal lightbox. Clicking an image scrolls to a dedicated project page with fuller photography and a short process note.

The contact section is plain text: email address, phone number, studio address in Seoul, each preceded by its Lucide icon (mail, phone, map-pin). No form widget, no submit button. The intent is that a prospective client will copy the email or dial the number — a person, not a brand.

# Do's and Don'ts

Do let the photography dominate every page. Use full-bleed images at natural aspect ratios. Trust the walnut to carry the visual weight.

Do maintain the 224 px maximum spacing between sections. Resist the urge to "fit more above the fold." The audience is browsing slowly, not scanning for conversions.

Do keep all text concise — one or two sentences per section. The maker's process is best explained in person, not in a wall of SEO copy.

Don't add SaaS-style credential rows ("10 years / 80 pieces / 30 clients"). The work visible on the page is the only metric that matters.

Don't use contact forms, chat widgets, or booking integrations. An email address and a phone number are sufficient and signal the kind of personal relationship the maker wants with clients.

Don't introduce a secondary accent colour or decorative illustration. Every visual element that is not typography or spacing should be a photograph of real furniture.