---
name: "hwal-studio"
version: "0.1"
description: "Portfolio for an independent walnut furniture maker in Seoul — editorial calm, material honesty, contemporary serif typography"
colors:
  primary:
    "50": "#f2eeea"
    "100": "#e0d8d0"
    "200": "#c4b8aa"
    "300": "#a89882"
    "400": "#8d7d66"
    "500": "#70634f"
    "600": "#5b4f3e"
    "700": "#463c30"
    "800": "#332b23"
    "900": "#201a14"
  accent:
    "50": "#f0f2f3"
    "100": "#d8dfe3"
    "200": "#b0bfc8"
    "300": "#8299a8"
    "400": "#5a7b90"
    "500": "#3f6478"
    "600": "#2f4d5e"
    "700": "#243b4a"
    "800": "#1a2a36"
    "900": "#101b24"
  neutral:
    "0": "#ffffff"
    "50": "#fafaf9"
    "100": "#f0efed"
    "200": "#dddbd7"
    "300": "#c4c0ba"
    "400": "#a9a39b"
    "500": "#8a8379"
    "600": "#6e675e"
    "700": "#534e47"
    "800": "#38342f"
    "900": "#1f1c19"
    "1000": "#000000"
typography:
  display:
    family: "Source Serif 4"
    weight: 300
  body:
    family: "Inter"
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
  "48": "192px"
rounded:
  sm: "2px"
  md: "4px"
  lg: "6px"
metadata:
  variants:
    light:
      description: "Default light mode — warm off-white, dark warm brown text"
      tokens:
        colors.background.primary: "#fafaf9"
        colors.text.primary: "#201a14"
    dark:
      description: "Dark mode — deep warm charcoal, cream text"
      tokens:
        colors.background.primary: "#141210"
        colors.text.primary: "#ede9e3"
    brand-a:
      description: "Gallery white — clean exhibition-wall neutral"
      tokens:
        colors.background.primary: "#faf9f6"
        colors.text.primary: "#241e16"
    brand-b:
      description: "Mist — cool overcast, muted blue-grey cast"
      tokens:
        colors.background.primary: "#f3f4f5"
        colors.text.primary: "#1a1c1e"
---

# Overview

Hwal Studio is a portfolio for a Seoul-based independent furniture maker working exclusively in solid walnut. The design register is editorial restraint — the kind of calm found in a well-lit gallery or a quietly confident monograph. Photography leads. Typography supports. Everything else steps back.

The maker's material dictates the palette. Walnut is warm, grey-brown, slightly ochre — never chocolate, never red. The primary scale is pulled directly from the wood's tonal range: silvered tannin at the light end, deep heartwood at the dark. A cool blue-grey accent (`#3f6478`) provides temperature contrast without competing. It reads as Seoul winter light through a workshop window — not as a tech-blue call to action.

Source Serif 4 at optical display weight (300, light) is the typographic anchor. It is a clean contemporary serif with optical sizing — structured and legible without decorative excess. The serif forms are crisp and purposeful, matching the 'serious studio' register without competing with the photography. Inter at 400 for body copy is deliberately unremarkable: it stays out of the way of the images. The pairing says "working furniture studio with a clear-voiced website," not "design agency showing off."

# Colors

The primary palette is a grey-brown ochre scale keyed to black walnut's natural range. Shade 500 (`#70634f`) sits at the visual midpoint — the colour of raw walnut with a drying oil finish. The scale extends from near-white (`#f2eeea` at 50) to near-black (`#201a14` at 900), ensuring the system can handle both text and surface roles without leaving the family.

The accent scale (`#3f6478` at center) is a desaturated blue-grey — think oxidised steel or a winter Han River sky. It appears sparingly: link hovers, a single accent rule, perhaps a state indicator. It exists to keep the warm palette from collapsing into monotony, not to become a second brand colour.

Neutral tones carry the light-grey warmth of the primary family but are desaturated further, ensuring they read as "background" rather than "surface." The warm cast (`#fafaf9` instead of pure `#ffffff`) prevents the clinical brightness that makes photography look flat on screen.

# Typography

Source Serif 4 is set at weight 300 for display — this is lighter than typical portfolio defaults and produces a refined, understated headline presence. At large sizes (48–80px viewport-relative), the clean serif strokes create vertical rhythm without visual mass. The optical sizing ensures the letterforms remain well-proportioned and legible across the entire scale, from large hero text down to smaller subheadings. The contemporary serif construction avoids both historical pastiche and trendy geometric abstraction — it simply reads as confident and considered.

Body copy uses Inter at 400 with slightly relaxed tracking (0.01em). Line height is generous at 1.65 for prose blocks. The reading experience should feel like a well-set monograph — unhurried, properly leaded.

Type scale: 12 / 14 / 16 / 20 / 24 / 32 / 48 / 64 / 80px. Nothing decorative. Size jumps are deliberate — a 24px subheading should feel distinct from 20px body, not like a rounding error.

# Layout

The site follows a magazine-paced vertical scroll: hero statement, generous breathing room, selected works as a photo-dominant grid, a brief about section, and plain-text contact details. No sidebar, no sticky nav chrome, no floating chat bubble.

The grid is an 8-column system with 32px gutters on desktop, collapsing to a single column with 16px margins on mobile. Project cards are image-first: the photograph occupies 75–80% of the card area, with the piece name, material, and year in a narrow text rail beneath. No overlay text on images — the furniture speaks clearly without typographic interference.

Section spacing pushes to 128px between major blocks and 192px between the last work and the footer. This is deliberately excessive by SaaS standards and correct by editorial ones. The silence between sections is part of the composition. It lets each project breathe the way pieces breathe in a real gallery — separated by wall, not crammed shelf-to-shelf.

# Elevation & Depth

Elevation is minimal. The site does not use cards with shadows, floating headers, or layered modals. Depth comes from photographic content — the grain, shadow, and dimension already present in well-shot furniture photography — not from interface skeuomorphism.

The only elevation tokens: `flat` (no shadow, default), `raised` (a single hard-edged 1px border in neutral-200, used on image containers to define edges against light backgrounds), and `overlay` (a subtle 0 4px 24px rgba(0,0,0,0.08) for any rare modal or overlay panel, should one be needed).

On dark mode, the raised token flips to a 1px border in neutral-800. No glow, no coloured shadows, no blur-backed glass panels. This is a studio portfolio, not a fintech dashboard.

# Shapes

Corner radii are deliberately tight: 2px (sm), 4px (md), 6px (lg). This references the precise, sharp joinery of the maker's practice — a through-tenon doesn't round itself. Even the largest container on the site (a full-bleed project photo) uses no more than 4px radius. Buttons and interactive elements use 2px.

There are no circles, pills, or rounded-full containers in the system. The maker's aesthetic is rectilinear. The design respects that geometry.

The single decorative shape permission is a thin horizontal rule (1px, primary-200 on light, primary-800 on dark) used as a section divider. It appears sparingly — once between works and about, once between about and contact — never under every heading.

# Components

**Hero.** Full-viewport height. Left-aligned display text (studio name, one-line descriptor) over a muted full-bleed background photograph at 15% opacity. No scroll animation, no parallax. Text sits in the bottom-left quadrant with 64px padding.

**Project card.** Image-dominant block. Aspect ratio 4:5 for the photograph. Below: piece name in display at 20px, material and year in body at 14px in neutral-500. Hover state: a 2px bottom-border in primary-300 (light) or primary-700 (dark). No scale transform, no overlay shift.

**About section.** Single column, max-width 640px, centred. One photograph (the maker in workshop, portrait orientation, 3:4 aspect) floated right with 24px margin. Two to three paragraphs of prose. No pull quote, no stat block, no client logo grid.

**Contact.** Plain text. Email address as a clickable link, phone number, studio address in Seoul. No form. No map embed. The tone is "here is how to reach a person," not "please fill out this ticket."

**Navigation.** Minimal top bar — studio name on the left, three to four text links on the right (Work, About, Contact, Instagram icon via Lucide). No hamburger menu on mobile; the links remain visible, set smaller. The nav is transparent over the hero, gains a neutral-0 background on scroll.

**Footer.** Small text, centred. Studio name, year, small print. A single Lucide `arrow-up-right` icon next to the Instagram link.

# Do's and Don'ts

**Do** let photography occupy the majority of every surface it touches. A project page should feel like opening a monograph — image first, text in service to the object.

**Do** use the maker's own photographs exclusively. During design, use descriptive placeholder captions ("photo: Jandi Bench — black walnut, danish oil, 2024, shot from above on linen") that double as art direction briefs. Never ship AI-generated images.

**Do** maintain WCAG-AA contrast across all four variants. The palette was built to guarantee this: darkest text on lightest background exceeds 12:1 in every variant; lightest text on darkest background exceeds 14:1.

**Don't** add stats rows ("10 years / 47 pieces / 12 exhibitions"). The work is the metric. A portfolio that needs to count its contents has already lost confidence.

**Don't** use decorative illustration, mesh gradients, or background patterns. The furniture is the visual texture. Adding graphic decoration is like playing music over a concert.

**Don't** round corners beyond 6px, use drop shadows, or introduce glass-morphism effects. The maker's joinery is sharp, precise, and structurally honest — the interface should be too.

**Don't** pick icons from the AI-slop register (`sparkles`, `star`, `wand-2`, `rocket`). Use `hammer` for process, `tree-deciduous` for material sourcing, `ruler` for dimension, `arrow-up-right` for external links. Each icon should be a noun, not an adjective.