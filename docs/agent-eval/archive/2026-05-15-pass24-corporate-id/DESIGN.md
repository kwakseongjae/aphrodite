---
name: "forma-industrial"
version: "0.1"
description: "Mid-century industrial design consultancy identity — geometric restraint, typographic authority"
colors:
  primary:
    "50":  "#fef5ee"
    "100": "#fce0c9"
    "200": "#f5b88e"
    "300": "#e88a52"
    "400": "#d66428"
    "500": "#c8521a"
    "600": "#a83f12"
    "700": "#822e0d"
    "800": "#5f210e"
    "900": "#3d160a"
  neutral:
    "0":    "#ffffff"
    "50":   "#faf9f7"
    "100":  "#f0eeea"
    "200":  "#e0ddd6"
    "300":  "#c9c4ba"
    "400":  "#9e9890"
    "500":  "#73706a"
    "600":  "#524f4b"
    "700":  "#3d3a37"
    "800":  "#2a2826"
    "900":  "#1a1918"
    "1000": "#000000"
  accent:
    "50":  "#eef6f3"
    "100": "#c8e8dc"
    "200": "#8dd4ba"
    "300": "#53be98"
    "400": "#2d9d76"
    "500": "#1a7a5a"
    "600": "#145f45"
    "700": "#104836"
    "800": "#0c3327"
    "900": "#081f18"
typography:
  display:
    family: "Playfair Display"
    weight: 700
  body:
    family: "Inter"
    weight: 400
  mono:
    family: "IBM Plex Mono"
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
rounded:
  none: "0px"
  sm: "2px"
  md: "4px"
  lg: "8px"
metadata:
  variants:
    light:
      description: "Default light mode — warm paper ground, burnt sienna authority"
      tokens:
        colors.background.primary: "#faf9f7"
        colors.text.primary: "#1a1918"
    dark:
      description: "Dark mode — foundry floor"
      tokens:
        colors.background.primary: "#1a1918"
        colors.text.primary: "#f0eeea"
    brand-a:
      description: "Patina — oxidized copper warmth"
      tokens:
        colors.background.primary: "#eef6f3"
        colors.text.primary: "#0c3327"
    brand-b:
      description: "Shop floor — raw steel"
      tokens:
        colors.background.primary: "#e0ddd6"
        colors.text.primary: "#2a2826"
---

# Overview

Forma Industrial is a consultancy that believes designed objects carry moral weight. The identity system reflects this conviction by refusing to entertain. Every surface is a assertion of competence — not of cleverness. The palette is burnt sienna and near-black on a warm paper ground, because those are the colours of a workshop that respects itself.

The typographic spine is Playfair Display at hero weight, paired with Inter for body. This is not a nostalgic choice. Playfair's thick-thin contrast at large sizes reads as architectural — a column, a beam. Inter disappears into its work. Together they reproduce the authority register of a mid-century trade catalog without costume.

Three geometric shapes are available as off-grid accent elements: a sienna square, a forest-green circle, and a black diagonal slash. Only one may appear per composition. This constraint is the system. The shapes are infrastructure, not illustration — they divide space, they anchor the eye, they do not narrate.

The CJK register matches this authority. Korean in Sandoll Myungjo Heavy, Japanese in Hiragino Mincho Pro W6, Chinese in Source Han Serif Bold. At hero scale, a single two-character phrase in burnt sienna, 96 px, left-aligned. The weight and serif structure of these families carry the same building-column authority as Playfair does in English.

# Colors

The primary is burnt sienna — Pantone 152 C, hex #c8521a. It is the colour of rust, of terracotta pipe, of the clay model in a design studio. It is warm without being friendly. It asserts without shouting. Paired with near-black (#1a1918) on paper white (#faf9f7), it produces the contrast ratio of a well-lit workshop.

Forest green (#1a7a5a) is the accent, used sparingly — a single underline, a tab-state, a pulled quote rule. It is the colour of a machinist's file handle, of patinated brass. It exists to prove the palette has depth without sprawl.

Neutral tones are warm throughout — the 50 and 100 stops read as vellum and bone, not clinical white. This warmth is not decorative; it is structural. Cold neutrals would make the sienna read as aggressive rather than authoritative. The palette must feel like materials, not pixels.

WCAG-AA compliance is maintained across all four variants. The darkest text on the lightest background exceeds 14:1 contrast. The lightest text on the darkest background exceeds 10:1. The patina and shop-floor variants use their respective dark greens and charcoal as text against tinted grounds, each exceeding 7:1. These ratios are not aspirations; they are entry requirements.

# Typography

Playfair Display at 700 weight is the display typeface. It appears at four sizes: 96 px for hero single-word dominance, 64 px for section heads, 36 px for pull quotes, and 24 px for editorial subheads. It never appears below 24 px. Below that threshold, its hairline serifs lose structural integrity and become noise.

Inter at 400 weight is the body typeface. It appears at 16 px for body text (line-height 1.55), 13 px for captions and metadata (line-height 1.45), and 11 px for labels and small caps (letter-spacing 0.05em). It is chosen because it does the work without asking to be noticed — the definition of a proper body face. Inter's geometric grotesque structure reads with the same authority at these sizes as any commercial alternative, freely available via Google Fonts.

IBM Plex Mono at 400 weight is reserved for project codes, technical specifications, and dimensional data. It is not used decoratively. A monospace face in an industrial-design context is a tool, not a texture. It appears at 13 px only, always uppercase for codes, mixed-case for specifications.

Drop-cap initial caps appear in editorial contexts — case study long-form text, methodology descriptions. The drop-cap is set in Playfair Display 700 at 64 px, sitting three lines deep, coloured in burnt sienna. This is the only decorative typographic gesture the system permits. It is enough.

# Layout

The page is never centered. This is not arbitrary rebellion against European modernism — it is functional. Left-aligned content with asymmetric margins produces visual tension that signals intentionality. The left margin is tight (24 px mobile, 64 px desktop); the right margin breathes wider. Content leans left, whitespace pools right.

The grid is a 12-column structure with 24 px gutters. Hero compositions use a 7-5 split — dominant word or image occupies seven columns, the off-grid geometric element occupies the remaining five. This split is not symmetrical and should never feel balanced. Balance is the enemy of interest.

Vertical rhythm follows a 4 px baseline. Headlines sit on a 48 px leading. Body text on 24 px. This creates clear sectional separation without requiring horizontal rules — though a 1 px sienna rule is available for explicit breaks. The rule is never decorative; it marks a genuine shift in content register.

Mobile layouts collapse to single column but maintain left alignment and asymmetric padding. The off-grid element, if present, repositions to the top-right corner, partially cropped by the viewport edge. This partial visibility is intentional — it suggests continuation beyond the frame, the mark of a real object in real space.

# Elevation and Depth

There are no drop shadows. Elevation is expressed through background colour value alone. The primary surface is #faf9f7. A raised card sits on #ffffff. A recessed area drops to #f0eeea. This three-step value ladder produces depth without the atmospheric artifice of shadows, which belong to a different visual tradition entirely.

Borders are used structurally, not decoratively. A 1 px solid border in neutral-300 (#c9c4ba) defines a card edge. A 2 px border in burnt sienna marks a selected or active state. No border ever appears without a functional purpose — separation, selection, or emphasis. Ornamental borders are prohibited.

Overlap is the primary depth tool. An image may overlap a text block by 24 px. A geometric accent may overlap both. These overlaps are not layered with z-index tricks — they are compositional decisions that create spatial relationships between flat elements. The design lives on a single plane, but that plane is intelligently occupied.

# Shapes

Three shapes exist in the system. A square, filled burnt sienna (#c8521a). A circle, filled forest green (#1a7a5a). A diagonal slash, 8 px thick, in near-black (#1a1918). Only one shape may be used per page or section. They are never outlined, only filled. They are never rotated beyond their canonical orientation.

The square is 120 × 120 px on desktop, 80 × 80 px on mobile. It sits off-grid, aligned to the right margin but offset vertically from any content line. It does not contain text. It does not have rounded corners. It is a flat, opaque, positional element — a brick in the composition.

The circle is 96 px diameter on desktop, 64 px on mobile. It sits off-grid, typically in the lower-left quadrant of a hero, partially cropped by the section boundary. It anchors peripheral vision without competing with the typographic content. It is a period, not an exclamation point.

The slash is a 45-degree line, spanning from the top-right to bottom-left of its containing area, never crossing text. It divides a composition into two zones — the seen and the implied. It is the most aggressive of the three shapes and should be used rarely, only when a composition demands structural bifurcation rather than mere accent.

# Components

The hero is a single word — "Form" or "Craft" or "Design" — set in Playfair Display 700 at 96 px, left-aligned, in near-black on the paper ground. The off-grid shape sits in the right third. A single subline in Inter 13 px, uppercase, letter-spaced, provides context. No photograph. No illustration. The word is the visual.

The project card is a photograph of a designed object — studio-lit, single coloured backdrop — with the project name in Playfair Display 36 px, left-aligned below, and a one-line description in Inter 16 px italic beneath. No metadata strip, no tag soup, no CTA stack. A single "View project" text link in burnt sienna, no icon.

The case study page uses drop-cap editorial text. The opening paragraph begins with a Playfair Display 64 px initial in burnt sienna, sinking three lines. The body is Inter 16 px on a 24 px baseline. Images are full-bleed within the content column, never the viewport. Captions are Inter 13 px, italic, preceded by the project code in IBM Plex Mono 13 px uppercase.

The contact section is a sentence, not a form: "Write to us at [address in burnt sienna, underlined on hover]." Below it, the studio address in Inter 13 px, with a Lucide map-pin icon in neutral-400. No social media icons. No newsletter signup. The consultancy speaks when spoken to.

# Do's and Don'ts

**Do** use burnt sienna as the single dominant colour signal. It appears in the hero word, the drop-cap, the active states, the single geometric accent. Everything else is warm neutral. This restraint is the system's voice.

**Don't** introduce a second saturated colour to "add visual interest." Interest is not added; it is composed. If the sienna feels insufficient, the composition is wrong — not the palette.

**Do** let the off-grid shape sit partially outside the visible area. Cropping suggests real dimension. A shape entirely contained within the frame becomes a sticker; a shape escaping the frame becomes architecture.

**Don't** use Playfair Display for body text, or Inter for headlines. Each face has its job. Playfair at small sizes is illegible hairlines. Inter at large sizes is bland. Respect the structural role of each.

**Do** write copy that is laconic and precise. "Forma designs objects for manufacture" is a complete hero subline. It does not need elaboration. The white space around it does the breathing.

**Don't** apply any gradient, texture overlay, or photographic filter to a background surface. The ground is flat colour. It is always flat colour. Texture belongs to the photography of real objects — which is where the materiality of this practice actually lives.

**Do** test all compositions with CJK content at the same scale and position as Latin content. Sandoll Myungjo Heavy at 96 px in burnt sienna should carry identical visual weight to Playfair Display at 96 px. If the CJK feels lighter, increase size by 10% — but first question whether the composition is depending on weight rather than structure.