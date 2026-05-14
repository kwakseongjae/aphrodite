---
name: "seoul-investigative"
version: "0.1"
description: "Long-form investigative journalism magazine for Seoul — Bodoni authority, two-color discipline, CJK rigour"
colors:
  primary:
    "50": "#f0f7f4"
    "100": "#d4e8de"
    "200": "#a8d1bd"
    "300": "#7cba9c"
    "400": "#509676"
    "500": "#2d6b50"
    "600": "#245842"
    "700": "#1b4433"
    "800": "#133125"
    "900": "#0a1f16"
  neutral:
    "0": "#ffffff"
    "50": "#f5f5f3"
    "100": "#e8e7e4"
    "200": "#d1cfca"
    "300": "#b3b0a9"
    "400": "#8a867e"
    "500": "#6b6760"
    "600": "#524e48"
    "700": "#3d3a36"
    "800": "#2a2825"
    "900": "#1a1917"
    "1000": "#000000"
typography:
  display:
    family: "Bodoni Moda"
    weight: 700
  body:
    family: "Source Han Sans KR"
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
metadata:
  variants:
    light:
      description: "Newsprint — white field, ink-black text, muted jade accent"
      tokens:
        colors.background.primary: "#ffffff"
        colors.background.secondary: "#f5f5f3"
        colors.text.primary: "#0f0f0e"
        colors.text.secondary: "#524e48"
        colors.accent: "#2d6b50"
    dark:
      description: "Late edition — near-black paper, pale ink"
      tokens:
        colors.background.primary: "#0e0e0c"
        colors.background.secondary: "#1a1917"
        colors.text.primary: "#e8e7e4"
        colors.text.secondary: "#b3b0a9"
        colors.accent: "#7cba9c"
    brand-a:
      description: "Broadsheet — warm cream stock, deep register"
      tokens:
        colors.background.primary: "#f8f5ee"
        colors.background.secondary: "#efeadf"
        colors.text.primary: "#1a1917"
        colors.text.secondary: "#6b6760"
        colors.accent: "#2d6b50"
    brand-b:
      description: "Cold press — blue-grey stock, noir authority"
      tokens:
        colors.background.primary: "#eef1f4"
        colors.background.secondary: "#dde2e8"
        colors.text.primary: "#111318"
        colors.text.secondary: "#4a4e56"
        colors.accent: "#245842"

---

# Overview

This is a typographic system for a Seoul-based investigative magazine — a publication that demands the same visual authority as its reporting. The design reduces itself to two typeface families and two colors because investigative journalism does not need decoration. It needs *structure*. Bodoni Moda at the display speaks with the certainty of an editorial voice that has done the work. Source Han Sans KR at the body speaks with the neutrality required for ten thousand words of reported prose.

The palette is black ink and a muted jade green — the color of a markup pen on a manuscript, of a censor's highlight through classified text. It is not decorative. It is editorial. When the accent appears, it marks something the reader should notice: a pull-quote, a data point, a section break. When it does not appear, the page is black and white, as a newspaper should be.

Korean text is not an afterthought here. It is the primary script. Every typographic decision was made for Hangul first, with Latin as the secondary register. The spacing, the line-length, the measure — all calibrated for mixed-script text where a line may contain both 한글 and English within a single sentence. This is the only honest way to design for Seoul.

# Colors

Two colors. Black and jade. This is not austerity for its own sake — it is the recognition that every additional color is a decision the reader must process, and investigative reporting already asks enough of the reader's attention.

The jade (#2d6b50 in light, #7cba9c in dark) was chosen because it reads as *institutional* rather than *natural*. This is not spa green or sustainability green. It is the green of a government stamp, of a ledger entry, of a document that has been filed and catalogued. It carries bureaucratic authority, which is exactly the register an investigative magazine needs.

Neutral tones are warm — pushed slightly toward umber. Pure greys are a screen convention; ink on paper is never truly grey, and this system remembers that. Even in the dark variant, the near-black background leans warm (#0e0e0c), the way newsprint darkens with age rather than glowing with the cold blue of a monitor.

# Typography

Bodoni Moda for display. Source Han Sans KR for everything else. Two families, no exceptions.

Bodoni Moda was chosen because its high contrast — hairline serifs against massive stems — creates headlines that *insist*. An investigative headline is not casual. It is a declaration. The thins and thicks of Bodoni make each word feel carved, monumental, permanent. Headlines are set at four sizes only: 96px for the lead story, 60px for section opens, 36px for subheads, and 24px for captions and meta. No intermediate sizes. No gradations. If two elements are different, they must be *clearly* different.

Source Han Sans KR serves as both the body typeface and the Korean register's primary voice. It was selected over Pretendard because its weight range includes a true Light (300) that pairs with Bodoni's weight at the optical level. When a line of Korean text sits beneath a Bodoni headline, the thickness of the strokes must belong to the same visual world. Source Han Sans achieves this. Body text is set at 18px with 30px leading — generous for long-form, necessary for Hangul's denser character forms.

Line length is held to 55–65 characters. This is non-negotiable. A reader tracking across 80 characters of mixed Korean and English text will lose the line. The margins are 20% of the canvas on each side. The web's default 8% margins produce lines that are too long and pages that feel like they were designed by someone who has never read a book.

# Layout

A 12-column modular grid. Not a suggestion — a law. Every element aligns to at least one column. The grid is consistent from the homepage to the article page to the archive page because consistency is not repetition; it is the grammar that makes variation legible.

Long-form articles use a single-column text measure (columns 4–9, leaving 3 columns of margin on each side at desktop width). This produces the 55-character line that Hangul requires. Images break out of the text column in controlled ways: a full-bleed photograph spans all 12 columns; a supporting image spans columns 3–10, creating a visual expansion that relieves the text column without abandoning the grid.

The homepage uses a 6+6 split. The left half carries the lead story — a single Bodoni headline, a single image, a single deck. The right half carries a stacked list of secondary stories, each a horizontal rule away from the next. No cards. No tiles. No masonry. A magazine homepage is a table of contents, not a shopping experience.

# Elevation & Depth

There is no elevation. There is no depth. This system uses no drop shadows, no layered surfaces, no floating cards. Depth in a journalistic publication comes from *reporting*, not from CSS.

Visual hierarchy is achieved through three mechanisms only: size, weight, and position. A Bodoni headline at 96px needs no shadow to assert itself. A horizontal rule needs no border-radius to separate one story from the next. If a design element cannot communicate its purpose through size and placement alone, it has failed and should be removed, not decorated.

The one concession to visual layering is the pull-quote, which uses a jade left-border (3px solid) and a slight left indent to set it apart from body text. This is not decoration — it is the typographic equivalent of a margin note, and it follows the same grid logic as everything else.

# Shapes

All corners are 2px or less. This is a journalistic publication, not a consumer app. Rounded corners suggest friendliness, approachability, softness. An investigative magazine should be none of these things.

Photographs are rectangular, full-bleed when they need to be, contained within the grid when they don't. No circular author photos — a rectangular crop treats the subject with the formality that reportage demands. No rounded containers, no pill-shaped buttons, no soft geometries.

Buttons — which appear rarely, for newsletter signup and subscription — are rectangular with 2px radius, set in Source Han Sans KR Medium at 14px, uppercase, letterspaced 0.08em. They look like they were typed on a form, because they were.

# Components

The article page is the system's core component. It opens with a full-width image (or a dense text opening, depending on editorial direction), followed by the headline in Bodoni at 96px, the byline and dateline in Source Han Sans at 14px, then the body text in a single centered column. Pull-quotes appear every 1500–2000 words. Data tables, when they appear, are simple bordered grids — no zebra striping, no hover states, just lines and numbers.

The homepage is a curated list, not a feed. The lead story occupies the top third. Below it, stories are arranged chronologically within their sections (조사, 정치, 사회, 국제), each separated by a thin rule. Section names are set in Bodoni at 24px, uppercase, letterspaced 0.12em — the only tracked-out type in the system, and it is acceptable here because section labels are *labels*, not reading text.

The archive page is a table. Date, headline, author, section. Sortable by column. No thumbnails, no excerpts. A reader who comes to an archive knows what they are looking for. Give them density, not decoration.

Navigation is a horizontal bar: logo (wordmark in Bodoni), section links, search icon (Lucide `search`), and a single accent-colored subscribe link. It does not move. It does not hide. It does not transform. It is always there, because a publication's sections are its skeletal structure.

# Do's and Don'ts

Do set headlines in Bodoni at four sizes and no others. Do use jade sparingly — when it appears, it must mean something. Do maintain 20% side margins on article pages. Do match Korean and Latin optical weights so mixed-script lines read as one voice. Do use horizontal rules as the primary separator between content blocks. Do treat the grid as the single source of structural truth.

Do not use a third typeface under any circumstance. If Bodoni and Source Han Sans cannot express the hierarchy, the hierarchy is wrong — fix the sizes and weights, not the font count. Do not track out headlines; Bodoni was drawn to be set tight. Do not center ragged text; flush-left is the only alignment that honors both Latin and Hangul. Do not use drop caps unless the editorial team specifically requests them for section openings within a long article — and even then, only as a structural marker, never as decoration. Do not add rounded corners beyond 2px. Do not use icons where a word will do; this is a text publication, not a dashboard. Do not use photography that looks like stock; every image must be reportage or explicitly art-directed editorial illustration.