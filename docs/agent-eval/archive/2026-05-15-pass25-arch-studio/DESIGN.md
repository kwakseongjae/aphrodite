---
name: "studio-geon"
version: "0.1"
description: "Portfolio site for a Korean architecture-and-furniture studio practising modular wood-and-steel construction"
colors:
  primary:
    "50": "#f7f5f0"
    "100": "#ede9e0"
    "200": "#ddd6c8"
    "300": "#c4b9a5"
    "400": "#a89a7e"
    "500": "#8a7a5c"
    "600": "#6e6046"
    "700": "#554934"
    "800": "#3b3224"
    "900": "#241e14"
  neutral:
    "0": "#ffffff"
    "50": "#fafaf8"
    "100": "#f0efec"
    "200": "#dddbd6"
    "300": "#b8b5ad"
    "400": "#8e8a7f"
    "500": "#6b665a"
    "600": "#524d42"
    "700": "#3a362e"
    "800": "#252320"
    "900": "#141210"
    "1000": "#000000"
  accent:
    "olive":
      "50": "#f3f4ef"
      "500": "#7a7a3e"
      "900": "#2e2e15"
    "iron":
      "50": "#f2f3f4"
      "500": "#6b7280"
      "900": "#1f2937"
typography:
  display:
    family: "Nanum Myeongjo, Georgia, serif"
    weight: 400
  body:
    family: "Pretendard, Inter, system-ui, sans-serif"
    weight: 300
  mono:
    family: "JetBrains Mono, Menlo, monospace"
    weight: 400
spacing:
  "0.5": "2px"
  "1": "4px"
  "1.5": "6px"
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
  "64": "256px"
rounded:
  none: "0px"
  xs: "2px"
  sm: "4px"
  md: "6px"
metadata:
  variants:
    light:
      description: "Default light mode — pale gallery wall, charcoal type"
      tokens:
        colors.background.primary: "#fafaf8"
        colors.background.secondary: "#f0efec"
        colors.text.primary: "#141210"
        colors.text.secondary: "#6b665a"
        colors.border.primary: "#dddbd6"
    dark:
      description: "Dark mode — workshop interior at dusk"
      tokens:
        colors.background.primary: "#141210"
        colors.background.secondary: "#1e1c18"
        colors.text.primary: "#f0efec"
        colors.text.secondary: "#8e8a7f"
        colors.border.primary: "#3a362e"
    brand-a:
      description: "Warm birch — keyed to pale ash and unbleached paper"
      tokens:
        colors.background.primary: "#f7f5f0"
        colors.background.secondary: "#ede9e0"
        colors.text.primary: "#241e14"
        colors.text.secondary: "#6e6046"
        colors.border.primary: "#c4b9a5"
    brand-b:
      description: "Cold steel — keyed to galvanised metal and concrete"
      tokens:
        colors.background.primary: "#f2f3f4"
        colors.background.secondary: "#e4e6e9"
        colors.text.primary: "#1f2937"
        colors.text.secondary: "#4b5563"
        colors.border.primary: "#c5c9cf"

---

# Overview

This system designs for a Korean studio that joins wood to steel using modular logic — the same joinery intelligence a seventeenth-century daepan stool carries, expressed through contemporary industrial material. The palette, the grid, and the type are all armature. Nothing decorates; everything structures.

Charlotte Perriand's principle holds: design the load-bearing frame first, then let the cushions be sparse. Here the frame is a modular grid built on a root-2 ratio, type in a 1.5 scale, and photography that shows objects mid-use. The page must argue for the work, not for the studio's taste. A table should be seen from above (plan), from the side (section), and with hands working at it — the fourth view, which is the body.

Korean text receives the same structural care as the architecture. Pretendard Light at generous line-height gives Hangul syllable blocks room to breathe — the gap between a chair leg and the floor, measured and deliberate. Nanum Myeongjo at Regular weight, never Bold, carries display text with the restraint of architectural lettering on a drawing. CJK character spacing is not tightened; it is set to match the pause between structural members.

# Colors

The primary hue family is warm ochre-olive, drawn from the material palette the studio actually works with: oiled walnut, raw steel, birch plywood, beeswax. Shade 900 is nearly black but carries the warmth of charcoal from a wood fire — never the cold blue-black of screens. Shade 50 is unbleached hanji paper, not the clinical white of a hospital.

The accent is a muted olive (#7a7a3e), used once per composition at most — a single steel bracket in a timber frame. A secondary cool iron grey exists for the steel members of the palette but is never the dominant voice. Wood speaks first; metal answers.

All four variants maintain WCAG-AA contrast between primary text and primary background. The light and brand-a variants use near-black on warm off-white. The dark variant reverses to warm off-white on near-black. Brand-b uses charcoal on cool grey — the register of a workshop drawing pinned to a concrete wall.

# Typography

Display is Nanum Myeongjo at Regular weight. Not bold — never bold. Architectural drawings are lettered in consistent weight; the hierarchy comes from size, not thickness. The type scale is 1.5 modular: 16 / 24 / 36 / 54 / 80 px. Each size is exactly 1.5× its predecessor, so any compositional element — a margin, a column, a photograph — reads against the same proportional logic. The grid and the type share a spine.

Body is Pretendard Light (300) at 17 px with 1.7 line-height. Korean text demands more vertical space than Latin; the generous leading treats each syllable block as a structural unit, not a compressed token. When Latin text appears alongside, it benefits from the same openness — the page breathes like a room with tall windows.

Captions and measurements use JetBrains Mono at 13 px, uppercased for dimensional annotations — the register of a technical drawing. " walnut, 900 × 450 × 750 mm, 2024 " reads like a note on a blueprint. No decorative italics, no em-dashes posing as editorial flair.

# Layout

The grid is modular at every scale. The macro layout uses a root-2 (√2 ≈ 1.414) column relationship: the primary content rail is √2 times wider than the secondary rail. On a 1440 px viewport, this yields roughly a 840 / 600 split with 64 px margins — the secondary rail is not a sidebar, it is the caption margin, the space where a hand rests beside the work.

Sections breathe at 160–224 px vertical spacing. Magazine pacing requires that each project receive the visual equivalent of a full spread — generous air above and below, never crammed against its neighbour. The spacing scale extends to 256 px for this reason. A 96 px maximum would compress the rhythm into a product catalogue; this studio's work deserves the pace of a monograph.

Photography occupies the primary rail at portrait ratios (4:5 or 3:4), never 16:9 widescreen. Portrait orientation respects the subject — a chair, a person at a table, a joint detail. When a piece is shown in multi-view (plan, section, side, body), the four images form a 2×2 grid within the primary rail, each annotated with the view name in mono caption text.

# Elevation & Depth

Depth is expressed through material shadow, not through the layered-card metaphor of consumer software. A steel bracket casting shadow on a timber surface — that is the reference. Elevation tokens are sparse: two levels only. Level 1 is the subtle lift of a photograph from its background. Level 2 is the spatial recession of a modal or overlay, used rarely.

Borders carry more structural weight than shadows. A 1 px border in the neutral-300 range defines a photograph the way a frame defines a drawing — it is an edge condition, not a decorative border. The border is always slightly warmer than pure grey, carrying the material temperature of the palette.

No frosted glass. No gradient overlays on photography. No drop-shadow on typography. The page is a wall: photographs pinned to it, text set directly on it. If a composition needs depth, the answer is better spacing, not a more complex shadow token.

# Shapes

Corners are nearly sharp — 2–4 px radius maximum. This is joinery, not consumer electronics. A mortise-and-tenon joint is a right angle with just enough ease for the material to meet without splintering. The same logic applies to image corners, card edges, and button shapes. Rounded corners signal softness; this studio's work is precise, not soft.

Photographs may be full-bleed (no rounding) when they occupy their own sectional band — the edge of the image is the edge of the composition. When images sit within a content rail, a 2 px radius prevents the pixel-fringe artifact at the boundary without softening the visual register.

Buttons and interactive elements use a 4 px radius — the minimum tolerance for a finger-tap target. No pill shapes, no circular containers. A button is a steel plate: flat, rectangular, precisely dimensioned. The hover state darkens the background by one shade; the active state darkens by two. No scale transforms, no shadow blooms.

# Components

The project card is the primary component. It is not a card in the SaaS sense — it is a photograph with a caption rail. The image occupies the full width of the primary column; the text sits below or beside in a narrow secondary rail. Title in Nanum Myeongjo Regular at 24 px. Material, dimension, year in mono at 13 px. No tags, no category pills, no hover overlays with gradient and play button.

The multi-view component presents a single piece in four views: plan, section, side, body. These are four equal images in a 2×2 arrangement within the primary rail, each labelled in mono caption. This component is the page's argument for the work — it shows that the studio thinks in three dimensions plus the human body, not just in the single heroic angle of a product photograph.

Contact is plain text: studio name, address in Seoul, telephone number, email. No form widget. A contact form is a brand submitting a ticket; a telephone number is a person you call. The mail, phone, and map-pin Lucide icons sit beside their respective lines at 20 px, stroke-width 1.5, in the text-secondary colour. The address links to a map; the email is a mailto link. Nothing else.

Navigation is whisper-quiet: studio name on the left, three to five section links on the right, set in Pretendard Light at 14 px, letter-spacing 0.5 px. No hamburger menu on desktop. On mobile, a simple vertical list appears on tap — no animated drawer, no overlay. The nav is a drawing's title block: present, informative, never louder than the drawing itself.

# Do's and Don'ts

**Do** show furniture with hands on it, with bodies near it, with tools resting beside it. The fourth view — the body in relation to the object — is what separates a design document from a catalogue photograph.

**Do** use the multi-view component for at least one piece in every project section. Plan, section, side, body. This is not decoration; it is the studio's thinking made visible.

**Do** let the spacing be generous. If you are unsure whether a margin should be 96 px or 160 px, choose 160 px. A room with too much air is always preferable to a room with too little.

**Don't** add statistics rows — "12 years / 87 pieces / 24 clients." The work is the metric. If a visitor wants to know the studio's reach, they will look at the projects.

**Don't** use bold weights for CJK display text. Nanum Myeongjo Regular at large sizes carries more authority than Bold ever could. Hierarchy through size, not weight — the same discipline an architectural drawing demands.

**Don't** apply filters, grain, or colour casts to photography. The photographs should look like they were taken in the studio on an overcast morning — even, neutral, honest. If the photograph needs a filter to work, it is the wrong photograph.