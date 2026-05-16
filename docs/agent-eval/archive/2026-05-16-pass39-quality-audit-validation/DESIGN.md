---
name:
  walnut-seoul
version:
  "0.2"
description:
  Portfolio site for an independent furniture maker working in solid walnut, based in Seoul — magazine-spread layout with single-column full-bleed portrait images and generous inter-project spacing
colors:
  primary:
    "50": "#f2f0f5"
    "100": "#e4e1eb"
    "200": "#c9c3d7"
    "300": "#aea5c3"
    "400": "#9387af"
    "500": "#6b5b91"
    "600": "#564875"
    "700": "#413659"
    "800": "#2c243e"
    "900": "#171224"
  neutral:
    "0": "#ffffff"
    "50": "#f8f8f8"
    "100": "#f0efef"
    "200": "#e2e1e0"
    "300": "#c7c5c4"
    "400": "#9e9b99"
    "500": "#767370"
    "600": "#5a5754"
    "700": "#42403d"
    "800": "#2c2a28"
    "900": "#1a1918"
    "1000": "#000000"
typography:
  display:
    family: "Instrument Serif"
    weight: 400
  body:
    family: "Pretendard"
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
  "48": "192px"
rounded:
  sm: "2px"
  md: "4px"
metadata:
  variants:
    light:
      description: "Default light mode — cool gallery white with warm black"
      tokens:
        colors.background.primary: "#f8f8f8"
        colors.text.primary: "#1a1918"
    dark:
      description: "Dark mode — near-black walnut stain with cool white text"
      tokens:
        colors.background.primary: "#1a1918"
        colors.text.primary: "#e8e7e6"
    brand-a:
      description: "Gallery wall — warm white keyed to gallery conditions"
      tokens:
        colors.background.primary: "#faf8f4"
        colors.text.primary: "#24211e"
    brand-b:
      description: "Workshop concrete — cool neutral grey of the studio floor"
      tokens:
        colors.background.primary: "#eeedec"
        colors.text.primary: "#1e1d1b"

# Overview

This portfolio exists to present the work of one furniture maker in Seoul. The site must disappear. When a visitor loads the page, the first thing they perceive is a dining table in black walnut, not a website. Every decision below serves that end.

The visual register is editorial, not commercial. Magazine pacing, not marketing funnel. One project per spread. Generous emptiness between sections. The visitor scrolls at leisure or does not scroll at all; the design does not push.

Material drives colour. Walnut is warm, dense, and dark. The palette answers with cool stone neutrals and a single desaturated violet accent pulled from the faint purple undertone visible in raw walnut end-grain under gallery light. The accent is a signal, not a theme. It marks interactive elements and nothing else.

Korean-language text appears throughout. Pretendard at light weight handles Hangul with the same optical neutrality Inter brings to Latin. Instrument Serif handles display headlines — a contemporary transitional serif with no historical costume. The pairing is quiet, not charismatic.

# Colors

The primary palette is stone-neutral. Backgrounds range from cool grey-white to near-black. Text ranges from warm black to cool off-white. This monotone field lets the walnut photographs carry all the warmth in the composition.

The violet accent (primary-500, #6b5b91) appears on links, active navigation states, and the contact line. It is the only chromatic colour on the page. Used sparingly, it marks the interactive without decorating the static. Used liberally, it would become a brand device — which this site is not.

Four variants cover the range of viewing conditions. Light and dark are functional. Brand-a is the gallery wall: a warm white that does not compete with the warm tones in walnut photography. Brand-b is the workshop: cool concrete grey. Both serve the same purpose — a ground on which the work sits without competition. All four pass WCAG-AA contrast between primary text and primary background.

# Typography

Instrument Serif at display sizes (48–80 px) for project titles and the maker's name. Weight 400 only — there is no bold variant, and none is needed. A contemporary transitional serif with sharp terminals and no flourish. It reads as considered without reading as precious.

Pretendard at body sizes (15–17 px) for all running text. Weight 300 for body, 400 for labels and navigation. This is the infrastructure-grade Korean grotesque — optically neutral, drawn for screen, no personality to shed. It pairs with Instrument Serif by staying out of its way.

Korean and Latin text sit on the same line at the same size. No font-size adjustments between scripts. Pretendard's Latin glyphs are designed to match Hangul in optical weight, so the texture holds. Line height is set to 1.6 for body and 1.1 for display — enough air to read, not enough to scatter.

The type scale is a 1.25 ratio: 15, 19, 24, 30, 38, 48, 60, 75 px. These are the only sizes. Intermediate values are not permitted.

# Layout

The index page is a single-column sequence of magazine spreads. Each project receives its own dedicated section. The section contains a full-bleed portrait image spanning the full viewport width, followed by one paragraph of description. The description includes the project title in display type, then factual details — wood, dimensions, year, commission context — set in body type. Then emptiness: a minimum of 192 px of vertical spacing before the next project spread begins.

No grid. No thumbnails. No category tags. The portrait image is presented at full width with no lateral constraint — the photograph touches both edges of the viewport. Portrait orientation is mandatory (4:5 ratio). The description paragraph sits below, centred or left-aligned within a narrow content column (maximum 640 px), with 32 px of space between image bottom and text top.

Project detail pages retain their own vertical scroll structure: full-width hero image of the finished piece in situ, one paragraph of factual description, then 8–12 photographs at various scales. Captions are narrow, set in body text, and factual: "Hakwi dining table, black walnut, 180 × 90 × 74 cm, 2024."

The pacing is magazine, not Pinterest. Each project is a spread. The 192 px gap between spreads is not wasted space — it is the rhythm that lets each piece be seen on its own, as a singular work rather than one tile among many.

# Elevation and Depth

There are no drop-shadows. Depth is communicated through photography, not through interface chrome. A walnut table has real shadow; the interface around it does not need to simulate any.

Layering is achieved through background value shifts alone. The footer is a slightly darker neutral than the body. The active navigation state is the accent colour. No overlays, no frosted glass, no blurred backgrounds. These are decorative effects pretending to be functional.

Z-index is used twice: navigation fixed above content, and modal overlay for enlarged photographs. That is the complete list of layers.

# Shapes

All rectangles. Corner radius is 2 px on images and 4 px on interactive elements (buttons, inputs). These radii exist to soften the aliasing artefact at pixel boundaries, not to round the shape. A 4 px radius on a button reads as rectilinear. A 16 px radius reads as a pill — which is a decorative form with no functional justification.

Photographs maintain their native aspect ratio. Portrait orientation is required for project spread images on the index (4:5). Landscape orientation is used only for interior shots on detail pages, showing the piece in context (3:2). Cropping to uniform ratios is not permitted — the photograph serves the furniture, not the grid.

Buttons are text with a 1 px border, 4 px radius, 8 px vertical and 24 px horizontal padding. Nothing more. A button's shape should not call attention to itself; its label should.

# Components

Navigation is a horizontal bar: maker's name on the left, section links on the right. Fixed. Background matches the page background with full opacity. No backdrop blur. Links are body type, weight 400. Active state is the violet accent. No underline on hover — the colour change is sufficient.

The project spread is the primary unit of the index page. Each spread contains: one full-bleed portrait image, one paragraph of text. No thumbnails, no category tags, no metadata chips, no hover states. The project title within the paragraph links to the detail page — the violet accent identifies it as interactive.

The contact section is plain text: postal address in Seoul, telephone number, email address. No form. No "Get in Touch" heading. A person who wants to commission a piece of furniture calls or writes. The interface does not intermediate.

Image presentation: full-bleed to viewport edges on the index. On detail pages, full-bleed within the content column. No border, no shadow, no caption box. The caption sits below the image in body type, left-aligned, with 16 px of space between image bottom and text top.

# Do's and Don'ts

Do let the photography carry the composition. The walnut surface, the joinery, the light — these are the content. The interface is a transparent container.

Do use the violet accent only where the user needs to identify an interactive element. A link in a paragraph. The current section in navigation. The email address in the contact line. Three uses. No more.

Do set Korean and Latin text at the same size and let Pretendard's optical balancing handle the rest. Do not add letter-spacing to Hangul. Do not use a different font for Korean than for Latin; use Pretendard for both.

Do not add a "philosophy" section with a portrait photograph and a paragraph about the beauty of grain. The work is the statement. If the work does not communicate the maker's values, no amount of copy will.

Do not animate anything. No fade-ins on scroll, no parallax, no hover-scale on images. Movement draws attention to the interface. The interface should not compete with the furniture for attention.

Do not use the words "craftsmanship," "timeless," "bespoke," or "heritage" anywhere in the copy. These words have been drained of meaning by every luxury brand with a marketing budget. State the material. State the dimension. State the year. The reader will conclude the rest.