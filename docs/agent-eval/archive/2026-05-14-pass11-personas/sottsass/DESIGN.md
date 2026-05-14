---
name: "GEOBUTI"
version: "0.1"
description: "Portfolio for a Seoul walnut furniture maker — joyful geometry, warm terracotta, editorial confidence, bilingual Korean/Latin typography"
colors:
  primary:
    "50": "#fde8db"
    "100": "#f9ccad"
    "200": "#f0a070"
    "300": "#e67339"
    "400": "#d85a1e"
    "500": "#c44d12"
    "600": "#9c3d0e"
    "700": "#752d0a"
    "800": "#4f1f08"
    "900": "#2b1204"
  accent:
    "50": "#e5faf3"
    "100": "#b1f0d9"
    "200": "#6de4b8"
    "300": "#22c896"
    "400": "#0ea87a"
    "500": "#058a63"
    "600": "#047050"
    "700": "#04563e"
    "800": "#033d2b"
    "900": "#022519"
  secondary:
    "50": "#fff5d4"
    "100": "#ffe8a0"
    "200": "#ffd94d"
    "300": "#e8be00"
    "400": "#bfa000"
    "500": "#998300"
    "600": "#736600"
    "700": "#4d4400"
    "800": "#332d00"
    "900": "#1a1700"
  neutral:
    "0": "#ffffff"
    "50": "#f7f2ea"
    "100": "#e6ddd0"
    "200": "#bfb5a5"
    "300": "#8a7e6d"
    "400": "#5c5244"
    "500": "#3e372c"
    "600": "#2a2419"
    "700": "#1e1a12"
    "800": "#14110c"
    "900": "#0c0a07"
    "1000": "#000000"
typography:
  display:
    family: "Source Serif 4"
    weight: 400
    axes:
      optical-sizing: true
  body:
    family: "Outfit"
    weight: 400
spacing:
  "1": "4px"
  "2": "8px"
  "4": "16px"
  "6": "24px"
  "8": "32px"
  "12": "48px"
  "16": "64px"
  "20": "80px"
  "24": "96px"
  "32": "128px"
  "40": "160px"
  "48": "192px"
  "56": "224px"
rounded:
  sm: "2px"
  md: "6px"
  lg: "12px"
  pill: "9999px"
metadata:
  variants:
    light:
      description: "Warm cream with terracotta accent — daytime gallery"
      tokens:
        colors.background.primary: "#f7f2ea"
        colors.background.secondary: "#e6ddd0"
        colors.text.primary: "#2a2419"
        colors.text.secondary: "#5c5244"
        colors.border.default: "#bfb5a5"
        colors.accent.default: "#c44d12"
        colors.accent.on-accent: "#fde8db"
    dark:
      description: "Deep umber with molten terracotta — night workshop"
      tokens:
        colors.background.primary: "#0c0a07"
        colors.background.secondary: "#1e1a12"
        colors.text.primary: "#f7f2ea"
        colors.text.secondary: "#bfb5a5"
        colors.border.default: "#3e372c"
        colors.accent.default: "#e67339"
        colors.accent.on-accent: "#2b1204"
    brand-a:
      description: "Near-white gallery wall — object focus, minimal visual noise"
      tokens:
        colors.background.primary: "#faf9f6"
        colors.background.secondary: "#ede9e1"
        colors.text.primary: "#1e1a12"
        colors.text.secondary: "#5c5244"
        colors.border.default: "#bfb5a5"
        colors.accent.default: "#c44d12"
        colors.accent.on-accent: "#fde8db"
    brand-b:
      description: "Mint freshness — spring exhibition mode"
      tokens:
        colors.background.primary: "#f0f9f5"
        colors.background.secondary: "#d4efe3"
        colors.text.primary: "#022519"
        colors.text.secondary: "#04563e"
        colors.border.default: "#6de4b8"
        colors.accent.default: "#c44d12"
        colors.accent.on-accent: "#fde8db"

---

# Overview

A furniture maker in Seoul who works in solid walnut does not need another grey portfolio. The walnut is warm. The hands are warm. The city is alive with neon and temple bell and street-cart steam. The portfolio should carry that warmth — not as decoration but as *information*: this maker builds objects that want to be lived with, not merely looked at.

The register is editorial-confidence meets playful-geometry. Think of a Memphis show catalogue designed by someone who actually loves wood grain — not someone performing irony. The terracotta primary says *earth, kiln, heated material*. The mint accent says *surprise*. The citron yellow says *this is not a funeral for furniture*.

Source Serif 4 is the display voice: a variable serif with optical sizing that delivers editorial authority in both Korean and Latin scripts. Its optical sizing axis ensures the letterforms remain crisp and well-proportioned whether set at 16 px or 88 px — the strokes thin, the serifs sharpen, the counters open. It is a face built for a bilingual Seoul studio that demands typographic seriousness without sacrificing warmth. Outfit serves as the calm body voice — a geometric sans that reads cleanly at small sizes without demanding attention.

The spacing is generous because the work is generous. A chair in walnut is not a thumbnail. Push the whitespace to 224 px between sections. Let the photography breathe. Let the page feel like a room with good lighting, not a catalogue page crammed with SKUs.

# Colors

The primary palette is terracotta — a family rooted in #c44d12 at its midpoint, ranging from pale peach (#fde8db) to deep burnt umber (#2b1204). This is not an arbitrary accent pulled from a trend carousel. It is the colour of iron-treated walnut, of the Seoul hillside in October, of a woodshop floor dusted with shavings under warm light. It *belongs* to the material.

The accent palette is mint green (#058a63 at 500) — chosen specifically because it sits across the colour wheel from terracotta and makes it vibrate. Complementary contrast, not harmonious blending. When the mint appears — on hover states, on price tags, on a section divider — it *pops*, and that pop is information: "pay attention here."

Citron yellow (#e8be00 at 300) serves as a tertiary signal colour — used sparingly for decorative elements, a stripe on a card border, the dot pattern's second colour. Three colours, not one. The convention is one accent on neutral; this system uses three, because the maker's work lives in a world of material richness, not minimalist reduction.

All four variants pass WCAG-AA. The light and brand-a variants use dark umber text on warm cream. The dark variant uses warm cream text on near-black. Brand-b uses deep forest green on pale mint. Every ratio exceeds 4.5:1. Accessibility is not optional — it is a structural requirement, like a dovetail joint.

# Typography

Source Serif 4 at display size is the architectural statement. It is authoritative, it is warm, it carries the editorial weight of a well-printed monograph, and with the optical sizing axis engaged, its letterforms adapt gracefully to any scale. At 64–80 px it becomes a *shape* — a typographic element that commands attention without shouting. This is the Sottsass principle reframed: the letterform is a column that holds the page up, not just a label on the page.

Outfit at body size (16–18 px, weight 400) is the quiet voice. It reads well at length. It does not compete with the display face. It handles Korean and Latin text with equal facility — critical for a Seoul-based studio that may present work in both languages.

The type scale follows a 1.25 ratio: 14, 18, 22, 28, 36, 44, 56, 72, 88. The jumps are deliberate — not a smooth gradient but a series of *steps*, like the shelves in the Carlton bookshelf. Each size has a job: 14 for captions, 18 for body, 28 for subheadings, 56+ for display.

Headings in Source Serif 4 are set in title case — not ALL CAPS, not sentence case. The mixed case lets the serifs breathe and shows off the font's balanced weight distribution. Body text in Outfit uses generous line height (1.6–1.75) because walnut furniture deserves room to be described well.

# Layout

The grid is a 12-column structure with asymmetric gutters — 32 px on the narrow side, 64 px on the wide side. Symmetry is the death of visual interest. The asymmetric gutter pulls the eye diagonally across the composition, creating movement without animation.

Sections flow vertically with 128–224 px spacing between them. This is magazine pacing — think Cereal, think Apartamento. Each project gets a full viewport or near-full-viewport moment. The work is not thumbnails in a grid; it is a sequence of encounters, like walking through the studio.

The hero section is full-bleed photography with a wide terracotta diagonal stripe cutting across at 12 degrees. The stripe carries the maker's name in Source Serif 4, reversed out in cream. This is not a centered logo over a faded background image. This is a *composition*.

Project cards are not rectangular. They use an 8 px radius — enough to soften the corner without becoming a tech-industry blob. On hover, a citron-yellow border appears on two sides only (left and bottom), creating an asymmetric frame that suggests the card is *lifting off* the page.

# Elevation & Depth

Drop shadows are used sparingly and only at one level: 0 4 px 24 px rgba(0,0,0,0.08). This is a warm shadow — not the cold grey of SaaS cards. On the dark variant, shadows invert to a subtle warm glow. The principle is joinery: a shadow should look like it *belongs* to the object, not like it was sprayed on by a CSS generator.

Borders are thick where they matter: 3 px on decorative frames, 2 px on card edges. Thin borders (1 px) are for subtle separators in body text areas. The thick border is a structural element — it frames a photograph the way a hand-cut frame frames a piece, with intention and physicality.

Background layers use the terracotta gradient at low opacity — a subtle wash that moves from cream to peach across the page. This is not a rainbow gradient. It is the visual equivalent of warm light shifting across a workshop floor throughout the day.

Elevation communicates hierarchy through colour, not just shadow. The most important content sits on the primary background. Secondary content shifts to the secondary background tint. The accent colour lifts interactive elements forward. Three planes, clearly delineated.

# Shapes

The terrazzo-dot pattern is this system's signature. It is constructed from a grid of circles at three sizes (4 px, 8 px, 16 px) in terracotta, mint, and citron yellow, scattered at pseudo-random intervals against a cream or umber background. It appears on section dividers, behind contact information, and as a subtle texture on the dark-mode background.

The diagonal stripe is the second motif — a terracotta bar at 12 degrees that cuts across the hero and reappears as a narrow accent on project detail pages. It is never horizontal. It is never centred. It *moves*.

Border radius stays tight: 2 px for text elements, 6 px for cards, 12 px for large containers. This is joinery, not soft-tech. Sharp corners with barely perceptible easing — the digital equivalent of a chamfered edge on a walnut table. Your hand catches it and knows it was considered.

Circles appear as decorative accents — a solid mint disc behind a project number, a terracotta dot as a bullet point, a citron ring framing a detail photograph. They are never arbitrary; each circle marks a piece of information the reader should notice.

# Components

The project card is the heart of the system: a tall (4:5 aspect ratio) photograph with a narrow text rail (280 px) to the right. The photograph has no overlay. The text sits *beside* the image, not *on top of* it. The maker's work deserves an unobstructed view. A terracotta tag in the top-left corner carries the project year. The card uses a 6 px radius and a 2 px border in neutral-200.

Navigation is minimal — a horizontal bar with the maker's name (Source Serif 4, 20 px) on the left and three links (Work, About, Contact) in Outfit 16 px on the right. No hamburger menu. No dropdown. The maker is a person with three things to say, not a corporation with a site map.

The contact section is plain text: email address, phone number, studio address in Seoul, and a small map outline (not an embedded Google Map — those are ugly and they leak data). The background is the terrazzo-dot pattern in terracotta and mint on cream. A 3 px terracotta border frames the entire section.

The about section uses a large portrait photograph (the maker in their workshop, not a headshot) alongside a text block in Outfit 18 px. The text is set in a single column, 600 px max-width, because reading about a craftsperson should feel like a letter, not a brochure. A citron-yellow horizontal rule separates the photograph from the text.

Project detail pages are full-bleed photography with a floating text panel that scrolls independently. The panel is 400 px wide, anchored to the left edge, with a semi-transparent cream background (opacity 0.95). As the user scrolls through photographs, the text panel remains visible — like a gallery label that follows you through the exhibition.

# Do's and Don'ts

**Do** use all three palette colours in every major section — terracotta, mint, and citron each appearing at least once. The page should feel *inhabited*, not monochromatic. **Do** let photography dominate every project card. The text rail is 280 px; the image takes the rest. **Do** use the terrazzo-dot pattern generously — it is this system's rhythm section, not a decorative afterthought. **Do** set display headings in Source Serif 4 at sizes large enough that the letterforms become architectural elements (56 px minimum, 72–88 px preferred), with optical sizing enabled so the serifs adapt properly at every scale. **Do** leave 128–224 px of vertical space between sections. The whitespace is doing structural work — it tells the reader "this is a new idea, take a breath."

**Don't** use a single accent colour on neutral and call it done. That is the beige-authoritarianism Sottsass warned us about. **Don't** set body text in Source Serif 4 at display weight — it is a display voice, not a reading voice; save it for headings where its editorial authority shines. **Don't** add statistics rows ("12 years / 87 pieces"). The work is the metric. **Don't** use form widgets for contact — plain text only. A craftsperson is a person you call, not a brand you submit tickets to. **Don't** use Lucide icons that are generic AI-slop markers (`sparkles`, `star`, `wand-2`). Use `hammer`, `ruler`, `tree-deciduous`, `hand-metal` — icons that belong to the workshop. **Don't** let the portfolio look like it could belong to anyone. It should look like it belongs to one specific maker in Seoul who works in walnut and has opinions about joy.