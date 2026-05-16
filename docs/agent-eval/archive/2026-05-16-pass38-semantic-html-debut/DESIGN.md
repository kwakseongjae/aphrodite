---
name: walnut-seoul
version: "0.1"
description: Editorial portfolio for an independent solid-walnut furniture maker in Seoul, featuring full-bleed project photography with alternating text rails
colors:
  primary:
    "50": "#f0f5f2"
    "100": "#d9e8df"
    "200": "#b3d1bf"
    "300": "#8dba9f"
    "400": "#5f8f74"
    "500": "#3d6b53"
    "600": "#2f5542"
    "700": "#244033"
    "800": "#1a2b24"
    "900": "#0f1a15"
  neutral:
    "0": "#ffffff"
    "50": "#faf9f6"
    "100": "#f0efeb"
    "200": "#dddbd5"
    "300": "#c4c1b9"
    "400": "#a8a49b"
    "500": "#8a857c"
    "600": "#6d6860"
    "700": "#524e47"
    "800": "#363330"
    "900": "#1a1917"
    "1000": "#000000"
  accent:
    "50": "#fbf4ed"
    "100": "#f3dfd0"
    "200": "#e5bfa0"
    "300": "#d49c6c"
    "400": "#c67f44"
    "500": "#b86830"
    "600": "#9a5326"
    "700": "#7a401e"
    "800": "#5a2e16"
    "900": "#3c1e0e"
typography:
  display:
    family: "DM Serif Display"
    weight: 400
  body:
    family: "Outfit"
    weight: 300
  mono:
    family: "JetBrains Mono"
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
  "20": "80px"
  "24": "96px"
  "32": "128px"
  "40": "160px"
  "48": "192px"
  "56": "224px"
rounded:
  none: "0px"
  xs: "2px"
  sm: "4px"
  md: "6px"
metadata:
  variants:
    light:
      description: "Default light mode — gallery white with deep charcoal text"
      tokens:
        colors.background.primary: "#ffffff"
        colors.background.secondary: "#faf9f6"
        colors.text.primary: "#1a1917"
        colors.text.secondary: "#524e47"
        colors.text.tertiary: "#8a857c"
        colors.border.primary: "#dddbd5"
        colors.surface.elevated: "#ffffff"
    dark:
      description: "Dark mode — near-black with warm stone text"
      tokens:
        colors.background.primary: "#0f100e"
        colors.background.secondary: "#1a1b18"
        colors.text.primary: "#e8e6e1"
        colors.text.secondary: "#a8a49b"
        colors.text.tertiary: "#6d6860"
        colors.border.primary: "#363330"
        colors.surface.elevated: "#242522"
    brand-a:
      description: "Warm gallery — aged paper for material photography contexts"
      tokens:
        colors.background.primary: "#faf9f6"
        colors.background.secondary: "#f0efeb"
        colors.text.primary: "#1a1917"
        colors.text.secondary: "#524e47"
        colors.text.tertiary": "#8a857c"
        colors.border.primary: "#c4c1b9"
        colors.surface.elevated: "#ffffff"
    brand-b:
      description: "Showroom — deep forest, for dramatic product presentation"
      tokens:
        colors.background.primary: "#111d17"
        colors.background.secondary: "#182920"
        colors.text.primary: "#e4ebe6"
        colors.text.secondary: "#96a99e"
        colors.text.tertiary: "#5f7a6c"
        colors.border.primary: "#2a3e33"
        colors.surface.elevated: "#1e3328"

---

# Overview

This is the design system for a Seoul-based independent furniture maker working exclusively in solid walnut. The visual language draws from three compositional traditions: the editorial pacing of interiors magazines, the UI restraint of research tools, and the project-as-case-study structure of design studios. The maker's work — warm-grained, precisely jointed, materially honest — sets the register. The website is a quiet frame for that work, never the work itself.

The palette avoids the expected walnut-warm defaults. Instead of reaching for amber, ochre, or terracotta, the primary hue is a cool sage-green — the color of walnut leaves in late summer, not the wood itself. This creates tonal distance between the UI and the material photography, preventing the interface from competing with the warm browns that will dominate every product image. The green recedes; the walnut advances.

Typography is intentionally split-register. DM Serif Display is a high-contrast Didot-era modern with razor-sharp serifs — it carries the editorial authority of a magazine masthead without the mustiness of old-style faces. It sits opposite Outfit at light weight, which is geometric and airy, providing clean legibility for captions and body text without visual noise. The pairing mirrors the maker's own practice: traditional form (the serif) executed with contemporary precision (the sans).

The spacing scale extends to 224 px because this portfolio needs to breathe like a gallery, not scroll like a feed. Large vertical intervals between sections create the magazine-pacing rhythm where each project lands as a distinct visual moment. Rushed spacing would collapse the work into a grid of thumbnails — the opposite of how handmade furniture deserves to be encountered.

# Colors

The primary palette is built on sage-green (hue ~150°, muted saturation, medium-dark key). Shade 50 reads as a pale mint breath; shade 500 is a confident forest-sage that can anchor buttons, links, and active states without shouting. The 900 shade is nearly black with a green undertone, providing a richer alternative to pure black for dark-mode backgrounds.

Neutral tones lean warm-stone — the 50 shade at #faf9f6 is a gallery white with a barely perceptible warmth that harmonizes with walnut photography without yellowing. The gray scale (100–900) moves through stone, clay, and charcoal rather than pure gray, ensuring the interface always feels materially present alongside the wood.

The accent color is a burnt copper (#b86830), used sparingly for interactive elements and single-word emphasis. It references the copper mallets, japanning, and warm metal hardware often found in Korean woodworking tradition. It appears only on hover states, active links, and at most one detail per view.

Each variant is contrast-validated. Light mode pairs #1a1917 text on #ffffff at roughly 17:1. Dark mode pairs #e8e6e1 on #0f100e at roughly 15:1. Brand-a places #1a1917 on #faf9f6 at roughly 16:1. Brand-b places #e4ebe6 on #111d17 at roughly 13:1. All exceed WCAG-AA by significant margins.

# Typography

DM Serif Display is used exclusively for project titles and the maker's name — never for body text, navigation, or UI labels. It is set at 48–72 px for project titles and 96–120 px for the homepage hero statement. It is a high-contrast modern face with hairline serifs and vertical stress, giving it the authority of editorial print without the preciousness of a Garalde or the sterility of a geometric sans at display size.

Outfit at weight 300 handles everything else: body text (17–19 px), navigation labels, captions, project metadata, and the contact section. Its geometric skeleton and wide apertures keep it legible at small sizes while its light weight ensures it never competes with the serif display. Line height for body is set at 1.65 — generous enough for comfortable reading of Korean-English mixed text.

JetBrains Mono at 400 is reserved for specific utility contexts: dimension annotations on technical drawings, material specifications, and price lists. Its use signals precision and craft-notation, reinforcing the workshop identity. It is never used for decorative or display purposes.

Type scale follows a 1.25 ratio with deliberate gaps: 13 / 16 / 19 / 24 / 32 / 48 / 72 / 120 px. The jumps between 32 and 48, and between 72 and 120, create visual hierarchy without requiring weight changes. A project title at 72 px next to a caption at 16 px generates a 4.5× size contrast — enough to clearly separate content layers.

# Layout

The page grid is 12 columns at 1440 px viewport with 24 px gutters. Content is constrained to columns 3–10 (an 8-column content well of ~920 px), leaving generous side margins of ~260 px each. This is the magazine-spread proportion: the work occupies the center, and the margins are the gallery wall.

Photography is formatted at 4:5 portrait ratio for individual pieces and 3:2 landscape for workshop and process contexts. Full-bleed images break out of the content well to fill the full viewport width, creating rhythm punctuations between the narrower text-and-image spreads. No image is ever placed on a colored card or inside a bordered frame — photographs touch the background directly.

Project cards are composed as full-bleed image panels. Each card fills the entire viewport width with the photograph at 4:5 aspect ratio. A narrow text rail — approximately 280–320 px wide — sits beside the image, containing only the piece name in DM Serif Display (24 px) and a one-line material caption in Outfit 13 px. Text rails alternate position per project: first project left, second right, third left, and so on. There are no borders, no frames, no decorative elements surrounding or overlaying the photography. The text rail sits on the page background; the image occupies its full side without competition.

The vertical rhythm alternates between dense clusters (hero image + title + material caption = 3 elements in 120 px) and long rests (160–224 px of pure background between projects). This is the editorial pacing principle: each project is a spread, and the space between spreads is where the viewer absorbs what they just saw.

Mobile collapses to a single column with 24 px side margins. Images go full-width with the text rail stacking beneath. The 224 px vertical spacing reduces to 96 px on small screens — still generous, but scrollable within reason. The serif display scales down to 36 px minimum, never below that threshold.

# Elevation & Depth

There are no drop shadows. Elevation is expressed through background tonal shifts — a card or highlighted element sits on a surface one shade lighter than the page background. In light mode, an elevated surface is pure white on a #faf9f6 page. In dark mode, an elevated surface is #242522 on a #0f100e page. This is how physical galleries signal depth: the wall and the floating panel are different whites.

Borders are 1 px in the neutral-300 tone, used only to separate navigation from content or to outline a technical drawing. Card borders do not exist — the image is the card. No borders frame the project card photographs or text rails. Interactive focus states use a 2 px outline in primary-500 with a 2 px offset, providing accessible focus indication without adding decorative chrome.

Layering is achieved through z-index discipline: page content at 0, fixed navigation at 100, overlays at 200, and modals at 300. There is one optional overlay context — a full-viewport image lightbox for project photography — which uses a 96% opacity background in the page's primary dark tone.

# Shapes

Corner radii are nearly flat: 2 px for small elements (tags, inline labels), 4 px for medium elements (input fields, code blocks), and 6 px for large elements (modal containers, full-bleed image wrappers). This is joinery language — sharp precision with the faintest chamfer, like a hand-planed edge that catches light.

There are no pill shapes, no fully rounded buttons, and no circular containers. Even the maker's portrait, if included, is rendered as a 4:5 rectangle — consistent with the photographic language of the entire site. Circles imply avatars and social-media conventions that do not belong in a craft portfolio.

Buttons are rectangular with 4 px radius, generous horizontal padding (24 px), and vertical padding (12 px). Primary buttons use primary-500 background with white text. Secondary buttons use transparent background, 1 px border in neutral-300, and neutral-800 text. Both states maintain a minimum 44 × 44 px touch target.

# Components

The **project card** is a full-bleed composition. The photograph fills one side of the viewport at 4:5 aspect ratio — edge to edge, no margins, no borders. Beside it, a narrow text rail (280–320 px) holds the piece name in DM Serif Display at 24 px and a one-line material caption in Outfit at 13 px ("Black walnut, brass joinery, 2024"). Nothing else — no price, no tag, no button, no overlay text on the image. Text rail position alternates with each project: odd-numbered projects place the rail on the left with the image right; even-numbered projects place the rail on the right with the image left. This alternating rhythm creates editorial pacing across the portfolio. No decorative elements, borders, or overlaid gradients compete with the photography. The entire card — image and text rail together — is a single click target leading to the case-study page.

The **case-study page** follows a strict vertical scroll: full-bleed hero image (the finished piece in its intended space), one paragraph of context (maximum 80 words), 6–12 photographs at various scales (detail shots, joinery close-ups, full-room context, technical drawings), and a final caption block listing materials and dimensions. There is no "back to projects" button — the browser back button is trusted.

**Navigation** is the maker's name (serif, left-aligned) and three links (Work, About, Contact) in Outfit 300 at 16 px, right-aligned. On scroll, the nav compresses to a thinner bar with reduced vertical padding but identical content. There is no hamburger menu until mobile breakpoint, where it becomes a full-screen overlay with the three links stacked vertically at 32 px size.

The **contact section** is plain text: the maker's name, studio address in Seoul, email address (as a mailto link), phone number, and a link to Instagram. There is no contact form, no Calendly widget, no chat bubble. This is a person you correspond with, not a brand you submit tickets to. The section is preceded by a single-line greeting in DM Serif Display ("Let's talk about a piece") and nothing more.

# Do's and Don'ts

**Do** let photography dominate every surface. If you are choosing between showing an image and showing text, show the image. The text exists to name and date the work — the photograph is the argument for the work's quality.

**Do** use the full vertical spacing scale. A 224 px gap between the last project and the contact section is correct. It gives the viewer time to transition from evaluating work to initiating conversation. Rushed spacing signals a rushed maker.

**Do** write captions as factual specifications. "Jinsol dining table, black walnut, 180 × 90 × 74 cm, 2024." Every word that is not a fact is a word that weakens the maker's authority. The work speaks. The caption names it.

**Do** alternate text rail position across projects. The left-right rhythm prevents visual monotony and creates the feeling of turning pages in a spreads layout. Consistency is in the structure, not the symmetry.

**Don't** add stats rows, counters, or achievement metrics. No "12 years of craft," no "87 pieces delivered," no "featured in [publication]." The portfolio is the proof. Counting is for corporations.

**Don't** use serif type for anything longer than a project title or a single greeting line. DM Serif Display at paragraph length becomes a novelty, not a tool. Body text belongs to Outfit, which was designed for sustained reading.

**Don't** introduce blue, purple, or saturated color anywhere in the interface. The palette is sage, stone, copper, and wood. If a client's project demands brand color, that color appears in the photograph of the finished work, not in the UI chrome surrounding it.

**Don't** overlay text on project photographs, add gradient washes to images, or place borders around the project cards. The photograph must breathe without decorative competition. If text is difficult to read beside an image, the solution is more space — not a scrim.

**Don't** animate scroll, parallax, or transitions between states. The site should load and move with the immediacy of turning a page. Delays, fades, and reveals signal digital showmanship that undermines the physical honesty of the work. If a transition is necessary, use 150 ms of opacity — nothing else.