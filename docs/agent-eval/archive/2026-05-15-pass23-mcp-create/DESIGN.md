---
name: "cord-radio-tokyo"
version: "0.1"
description: "Portfolio for a Tokyo studio designing a single wall-mounted radio with pull-cord power switch — single long-scroll case study layout with no project grid"
colors:
  primary:
    "50": "#faf9f6"
    "100": "#f0efea"
    "200": "#e0ddd6"
    "300": "#c7c2b8"
    "400": "#a9a294"
    "500": "#8a8172"
    "600": "#6b6358"
    "700": "#524c43"
    "800": "#3a3630"
    "900": "#252320"
  neutral:
    "0": "#ffffff"
    "50": "#faf9f7"
    "100": "#f4f2ef"
    "200": "#e8e5e0"
    "300": "#d5d0c8"
    "400": "#b5aea3"
    "500": "#8f877a"
    "600": "#6d665c"
    "700": "#4a453e"
    "800": "#2e2b27"
    "900": "#1a1816"
    "1000": "#000000"
typography:
  display:
    family: "'Source Serif 4', serif"
    weight: 300
  body:
    family: "Pretendard, 'Hiragino Sans', 'Inter', sans-serif"
    weight: 300
spacing:
  "0": "0px"
  "1": "4px"
  "2": "8px"
  "3": "12px"
  "4": "16px"
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
      description: "Default — warm white, like studio wall plaster"
      tokens:
        colors.background.primary: "#faf9f6"
        colors.text.primary: "#1a1816"
    dark:
      description: "Dark mode — charcoal of cast aluminum"
      tokens:
        colors.background.primary: "#1a1816"
        colors.text.primary: "#f0efea"
    brand-a:
      description: "Natural ash — pale wood gallery"
      tokens:
        colors.background.primary: "#f4f2ef"
        colors.text.primary: "#252320"
    brand-b:
      description: "Evening concrete — cool neutral"
      tokens:
        colors.background.primary: "#e8e5e0"
        colors.text.primary: "#1a1816"

---

# Overview

This portfolio holds a single object — a wall-mounted radio with a pull-cord switch. The site must feel like the object it presents: one gesture, no choices, the thing you came for already where your eye goes. Naoto Fukasawa designed the wall-mounted CD player for MUJI by borrowing the vocabulary of a ceiling fan. This site borrows the vocabulary of a gallery label — small type beside a large, quiet surface.

The palette avoids hue almost entirely. A very pale ochre undertone runs through the light variant — the warmth of sanded ash, not paint. Dark mode is the grey of raw aluminum, not the blue-black of a screen. Color would be a statement. This studio makes a single radio. Statements are not needed.

Display type uses Source Serif 4 at optical light weight (300). This is a deliberate departure from Fukasawa's own preference for sans-serif ubiquity — here, the serif carries the quiet authority of a museum caption, which is the register a solo-object portfolio needs. Body text is Pretendard Light, which renders Japanese kana and kanji at small sizes without visual noise. Both typefaces disappear into reading. Nothing announces.

# Colors

The primary scale is a warm-neutral ladder running from near-white (#faf9f6) through stone (#8a8172) to near-black (#252320). There is no accent hue. The radio itself — its material, its photographed context — provides whatever color appears on the page. The site's job is to hold still.

WCAG-AA contrast is verified across all four variants. Light (#faf9f6 / #1a1816) exceeds 14:1. Dark (#1a1816 / #f0efea) exceeds 14:1. Brand-a (#f4f2ef / #252320) exceeds 13:1. Brand-b (#e8e5e0 / #1a1816) exceeds 12:1. All well above the 4.5:1 minimum.

The palette supports only one decision per surface: background and foreground. No secondary buttons, no status chips, no color-coded categories. A studio with one product does not need a system.

# Typography

Source Serif 4 Display at light weight (300) handles all headings. The optical sizing axis ensures the serifs remain crisp at large sizes without thickening — this matters because the headings will be set large (48–72 px) but must still read as quiet. Source Serif 4 was chosen over sharper contemporaries (Fraunces, Newsreader) because its letterforms carry no historical costume. It is a tool serif — present, functional, unremarkable.

Body text is Pretendard Light (300) at 14–15 px. Line height is generous (1.65) so that Japanese and English mixed text breathes. Never bold. Never display weight. The type should recede the way a well-designed kettle handle disappears in the hand. If the reader notices the typeface, it has failed.

CJK text strategy: Pretendard serves Korean and Latin. Hiragino Sans W3 serves Japanese. The shared register is "small optical size, thin stroke, no personality." Headings in Source Serif 4 accept that Japanese characters will fall back to Hiragino Mincho — the slight mismatch between Latin serif and Japanese mincho is acceptable because both carry the same museum-caption register.

# Layout

Single column. Max content width of 640 px for text, 960 px for photography. Side margins absorb the rest — on a 1440 px viewport, roughly 35–40% of the horizontal space is intentional emptiness. This ratio mirrors the product-to-background ratio Fukasawa demands in photography (1:3).

Vertical spacing follows a magazine-spread rhythm, not a component-stack rhythm. Between major sections: 192–224 px. Between a photograph and its caption: 16 px. Between caption and next photograph: 96 px. The pace is slow. Each image gets room. The scroll should feel like turning a page, not scrolling a feed.

The project page is one long scroll: hero photograph (full-bleed, 4:5 portrait, mid-use — hand reaching toward the pull cord) → one paragraph of context (the gesture, not the features) → a sequence of 6–10 photographs at varying scales (detail, context, installation) → a short materials note → plain-text contact. No navigation bar with multiple sections. No grid of thumbnails. One object, one path.

There is no projects-section and no project-grid. This studio has a single product — a thumbnail grid of one item is a category error. The portfolio is the case study itself, laid out as a continuous vertical narrative. Every photograph, every caption, every spacing decision serves the one object. The page does not link outward to individual project pages because there is nothing to link to. The home page is the project page. The project page is the entire site.

# Elevation & Depth

None. The site is flat the way a wall is flat. The radio mounts flush to the wall — the site mounts flush to the screen.

If a photograph needs to lift from the background, it does so with a hard-edged border (1 px of neutral-300) rather than a shadow. Shadows model real material; a screen is not material. A thin line is honest about the surface it sits on.

The single permissible shadow is on the radio's pull cord in the hero photograph — but that is the photographer's concern, not the site's. On the page itself, everything lies on one plane.

# Shapes

Corners are cut sharp: 2 px radius maximum on interactive elements, 0 px on containers and images. This is joinery logic — the radio's housing meets the wall at a crisp edge, and the site's boxes should meet the viewport the same way.

Photographs maintain their native aspect ratio (4:5 portrait preferred). No rounding, no masking into circles or organic blobs. The rectangle is the most neutral container — it makes no statement about the content it holds.

Buttons, if any exist, are text-only with a thin underline on hover. No rounded pill shapes, no filled containers. A button that looks like a button is a button that draws attention to itself. The only action on this site is contacting the studio — an email address, plain text, is sufficient.

# Components

The component inventory is deliberately impoverished. A studio presenting one object needs fewer parts, not more.

**Hero block.** Full-bleed photograph, 4:5 ratio. A single line of display text below: the studio name in Source Serif 4, 24 px, light. No overlay, no gradient, no text on the image. The photograph and the name occupy separate vertical space — they do not compete.

**Caption block.** Body text at 13 px, neutral-500 on light backgrounds, neutral-400 on dark. One line maximum. Names the gesture, not the feature: "Pull cord — power on" or "Speaker grille, stamped aluminum, 1.2 mm." No descriptions of intent, process, or philosophy.

**Section spacer.** 192–224 px of empty vertical space. No divider lines. The gap is the divider.

**Contact block.** Plain text: studio name, address in Tokyo, email, telephone. No form. No social icons. A person you call, not a brand you submit tickets to.

**Navigation.** None visible on the project page. The studio name in the upper-left links home (which is the project page). There is nowhere else to go.

There is no projects-section component. There is no project-grid component. These structures exist for studios with many objects to show. A single-product portfolio unfolds as one continuous case study — the components listed above are sufficient to build it. Adding a grid of one, or a "projects" section containing a single card, would be architecture for architecture's sake. The page is the story, top to bottom, and nothing interrupts it.

# Do's and Don'ts

**Do** photograph the radio mid-use. A hand reaching for the pull cord. The cord mid-pull, slightly blurred. The radio mounted on a real wall in a real room with real light. The body of the user should appear in at least one frame — the object is the protagonist but use is the plot.

**Do** let the type sit small. Headings at 48 px maximum for the studio name, 24 px for section openings, 14–15 px for body. Large type shouts. This studio does not shout.

**Do** allow generous emptiness. The ratio of product to background in photography should be at least 1:3. The ratio of content to margin on the page should be similar. Restraint is not the absence of decision — it is the discipline of removal.

**Don't** add visible screws to any render or photograph of the radio. This is a Fukasawa hard constraint. The housing must read as a single surface.

**Don't** frame the product like a celebrity. No dramatic lighting, no isolated-on-white studio shots, no hero angles. The radio is a household object. Photograph it the way you would photograph a light switch — present, unremarkable, in its place.

**Don't** use multiple colors on any single surface. The radio is one material, one color. The page is one background, one foreground. The typography is one weight per role. Every additional variation is a decision that needs justifying, and most decisions are better removed.

**Don't** add a logo or brand mark. The studio name in type is sufficient. A logomark on a page that already displays the product would be a frame around a frame — one level of removal too many.

**Don't** introduce a projects-section or project-grid. A thumbnail gallery of one product is an empty gesture — the grid implies multiplicity the studio does not have. The case study is the page. Remove the scaffold and let the object occupy the space directly.