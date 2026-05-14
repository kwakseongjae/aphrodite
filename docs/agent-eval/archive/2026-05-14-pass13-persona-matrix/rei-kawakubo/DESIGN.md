---
name: "hakwi-deconstruct"
version: "0.2"
description: "Lookbook composition page for an independent Seoul designer — left-cropped season hero, asymmetric garment grid with Hangul fabric-labels, bottom contact rail, zero-radius sharp containers"
colors:
  primary:
    "50": "#e8e6e1"
    "100": "#c8c3bb"
    "200": "#8a8279"
    "300": "#5c544b"
    "400": "#3a332c"
    "500": "#231c14"
    "600": "#1a150f"
    "700": "#120e0a"
    "800": "#0c0906"
    "900": "#050403"
  accent:
    "50": "#fff4cc"
    "100": "#ffe599"
    "200": "#ffd24d"
    "300": "#ffbf00"
    "400": "#d9a200"
    "500": "#b38600"
    "600": "#8c6900"
    "700": "#664d00"
    "800": "#403000"
    "900": "#1a1400"
  neutral:
    "0": "#ffffff"
    "50": "#d4cfc4"
    "100": "#9a9486"
    "200": "#5f5a50"
    "300": "#44403a"
    "400": "#2e2a24"
    "500": "#1a1714"
    "600": "#12100e"
    "700": "#0e0c0a"
    "800": "#090807"
    "900": "#040303"
    "1000": "#000000"
typography:
  display:
    family: "Pretendard"
    weight: 900
  display-serif:
    family: "Noto Serif KR"
    weight: 200
  body:
    family: "Noto Sans KR"
    weight: 300
  body-latin:
    family: "Instrument Sans"
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
  "20": "80px"
  "24": "96px"
  "32": "128px"
  "40": "160px"
  "48": "192px"
  "64": "256px"
rounded:
  sm: "0px"
  md: "0px"
  lg: "0px"
metadata:
  variants:
    light:
      description: "Bone gallery — warmed off-white, barely there"
      tokens:
        colors.background.primary: "#e6e0d4"
        colors.text.primary: "#1a140c"
        colors.background.secondary: "#d4cfc4"
        colors.text.secondary: "#3a332c"
    dark:
      description: "Bleached black — the default register"
      tokens:
        colors.background.primary: "#0a0908"
        colors.text.primary: "#d4cfc4"
        colors.background.secondary: "#12100e"
        colors.text.secondary: "#8a8279"
    brand-a:
      description: "Raw — unprinted paper, arsenic whisper"
      tokens:
        colors.background.primary: "#bcb59e"
        colors.text.primary: "#12100d"
        colors.background.secondary: "#a8a08a"
        colors.text.secondary: "#2e2a24"
    brand-b:
      description: "Insect — wrong and alive"
      tokens:
        colors.background.primary: "#1c1208"
        colors.text.primary: "#d6c89a"
        colors.background.secondary: "#2a1e10"
        colors.text.secondary: "#8a7d5c"

---

# Overview

This is not a portfolio. It is a garment seen from behind — you must walk around it. The site refuses the viewer's expectation of immediate comprehension. Every surface asks to be read twice.

The palette lives in black's gradations because black does not flatter. The warm brown-blacks carry the memory of dye on cotton, of ash on concrete. The single accent — amber at #b38600 — appears only when something must be irreconcilable with everything around it. It is the colour of a warning label sewn into a seam.

Hangul dominates because the designer works in Seoul. Latin appears only where the work demands it — exhibition titles, stockists in other languages. The type does not balance. Pretendard 900 at 120 px cuts the air; Noto Serif KR 200 at the same size dissolves. They are not paired. They collide.

The grid exists only to be violated. The 12-column structure holds the back of the composition; what the viewer sees is the tear along that structure. Asymmetric margins shift between breakpoints. Content lands where it refuses to centre.

composition.html builds the lookbook's primary structure in three vertical acts: a left-cropped season hero anchored by Pretendard 900 at 120+ px over a full-bleed dark ground; an asymmetric garment grid carrying 3:4 photographs with minimal Hangul fabric-labels; and a bottom-placed contact rail. All containers use the declared token spacing and colours, with zero-radius sharp edges throughout.

# Colors

The primary palette is not black. It is fifteen shades of almost-black, each carrying a different temperature. #050403 is cold, mineral — the black of volcanic glass. #231c14 is warm, fibrous — the black of linen washed a hundred times. These are different colours. They must be treated as different colours.

The accent exists to be wrong. Amber #b38600 against #0a0908 produces a jolt — not harmony. It is used for one element per view: a season marker, a single price, the warning that this garment cannot be worn the way you think. If the accent feels comfortable, remove it.

The neutral scale was built by mixing black with the dust of old paper. #d4cfc4 is the colour of a gallery wall after ten years of exhibitions. #9a9486 is the colour of a seam allowance left visible. Neither is decorative. Both are structural.

Warm brown-blacks were chosen over cool blue-blacks or pure neutral blacks. The hue family is ochre-derived, referencing raw earth pigments — Korean 황토, the clay under Seoul. This differs from the grey-black one might first reach for. The warmth is the designer's handwriting.

# Typography

Pretendard 900 is the display voice. It is used for season names, for Hangul at sizes where the stroke structure becomes architectural — 80 px minimum, 160 px for single-character moments. The weight fills space; the forms become walls, not letters.

In composition.html the season hero sets Pretendard 900 at a minimum of 120 px, left-aligned and cropped by the viewport edge. The letters bleed off-screen; the viewer receives only the right two-thirds of each syllable block. The incomplete character forces a mental completion — the viewer leans in, then reconstructs what the crop denied them.

Noto Serif KR 200 exists to dissolve. Set at the same size alongside Pretendard, it is the vanishing seam — visible only at the edge of perception. It handles exhibition texts, dates, the names of fabrics. It does not compete. It haunts.

Body copy uses Noto Sans KR 300 — thin but not hairline, legible at 14–16 px for the Hangul passages that describe materials and construction. Latin body text uses Instrument Sans 400, chosen for its industrial evenness. Neither voice is pleasant. Both are precise.

The type breaks intentionally. Season titles crop the bottom third of characters at the viewport edge. Lookbook numbers jump from 24 px to 96 px between adjacent entries. A collection statement might be set entirely in Pretendard 900 at 10 px — too small to read comfortably, forcing the viewer to lean in. Comfort is not the objective. Reading is the objective.

# Layout

The page is cut, not composed. Content is placed against the left margin and allowed to bleed off the right, or pushed to the right edge and cropped on the left. Centre alignment is reserved for a single use: the lookbook's title character, floating in 256 px of vertical space, monumental and alone.

composition.html enforces three stacked zones. The season hero occupies the full viewport height, full-bleed dark background (#0a0908 in the dark variant), with the season word left-cropped at roughly 60% visibility. Below, the garment grid fills the central scroll region: an asymmetric 12-column layout where photograph columns alternate between 4 and 5 columns, with 3-column gaps, producing a staggered rhythm. The bottom contact rail is pinned flush to the viewport's lower edge — a single 64 px-tall bar containing Hangul address and email, zero-radius, using background-secondary (#12100e) against text-secondary (#8a8279).

Photography occupies 70–80% of any given viewport. Images are full-bleed where possible. Cropping is intentional: a garment shown from the back, a model's face excluded, the hem entering the frame without its shoulder. The photograph's subject is not the garment — it is the question the garment raises.

Between sections, silence. The spacing scale reaches 256 px because the work requires distance. A viewer who scrolls past an image in half a second has not seen it. The void forces duration. Some sections are nearly empty — a single word in 160 px of vertical space, or nothing at all — as structural silence.

The responsive behaviour does not re-flow into comfortable columns. At narrower viewports, the asymmetry inverts: what was left-aligned becomes right-aligned, what was cropped on the right crops on the left. The composition maintains its refusal to settle.

# Elevation & Depth

There is no elevation. The surface is the surface. Shadows are not used because the work does not float above anything — it occupies the same plane as the wall, the floor, the viewer's body.

Depth is achieved through tonal compression alone. A card is not raised; it is a slightly denser area of the same darkness. The difference between #0a0908 and #12100e is enough to distinguish content zones. This is how black creates hierarchy — not through shadow, but through the accumulated weight of barely-different densities.

Layering is reserved for the lookbook detail view, where a garment photograph overlaps the previous image by 15%. The overlap is deliberate — it denies the viewer a clean transition, forcing both images to coexist in the same glance, the way a garment's front and back coexist on a body.

Borders, when they appear, are 1 px of neutral-200 against neutral-900 — visible only when you look for them, present as a structural line the way an internal seam is present.

# Shapes

All corners are zero. Curves are earned through content, never through container geometry. A photograph of a curved sleeve provides the only organic line on the page. The containers that hold it are rectilinear because the rectangle is honest about its artificiality.

Aspect ratios are chosen to unsettle. Garment photographs use 3:4, but cropped asymmetrically — the division falls at 2:5 rather than 1:2. Thumbnail grids use 5:7, a ratio slightly wrong compared to the expected 2:3, producing a subtle vertical stretch that makes each garment feel taller than expected.

The button is a rectangle with no radius, minimum 56 px tall, filled with the background colour, bordered in text colour, text in the same weight as body. It does not invite clicking. It states that clicking is possible. The hover state inverts — text colour fills, background text disappears into it.

Cards are not cards. They are regions of tonal difference, bounded by a top-edge rule or nothing at all. The content knows where it ends because the spacing tells it, not because a container draws a border.

# Components

The lookbook grid presents twelve to sixteen garments per season. Each entry is a photograph with a single line of text below — fabric weight in grams, or the Korean word for the technique, nothing else. The photographs are ordered not chronologically but by structural affinity — garments that share a seam logic appear adjacent, regardless of when they were made.

In composition.html the garment grid is explicitly asymmetric. Each cell contains a 3:4 photograph above a single Hangul fabric-label — two to four characters in Noto Sans KR 300 at 12 px, tracking loose at 0.08em. Labels name the material only: 모시 (ramie), 울 (wool), 면 (cotton). No prices. No garment names. The grid columns stagger: the first photograph spans 5 of 12 columns, the next spans 4, then a 3-column void, then 5 again. This stagger prevents the eye from settling into a predictable scan.

Navigation is a single word at the top-left corner: the Korean character for the current section. Clicking it does nothing. The actual navigation is a list that appears only when the viewer presses and holds for one second — a gesture borrowed from the way you touch fabric to understand it.

The collection statement is one sentence set in Pretendard 900 at 14 px, centered in 192 px of vertical space, in the accent colour. It is the only centered element on the entire site. It is small and it refuses to accommodate you. Season markers use the accent: SS25, FW24, rendered in Instrument Sans at 64 px, placed flush-left, cropped by the viewport edge.

Contact information is Hangul-only text in the bottom contact rail: a Seoul address and an email, pinned to the viewport's lower edge, persistently visible. No form. No social links. No photograph of the designer. The rail is a sharp rectangle — zero radius, full viewport width, 64 px tall, using the secondary background token. The work is what speaks.

# Do's and Don'ts

**Do** crop characters at viewport edges. The cropped Hangul syllable is more present than the complete one — the viewer completes it mentally, which means they have engaged with it.

**Do** use the accent colour only when it creates a problem. A season title in amber against the dark background is not decorative — it is a fissure. If the accent colour ever feels like it belongs, remove it immediately.

**Do** show the back of the garment. Show the inside of the garment. Show the garment on the floor after it has been removed. The front view is what every other lookbook provides. Refuse it.

**Do** keep fabric-labels minimal — material name only, in Hangul, at text sizes that refuse decorative enlargement. The word "모시" at 12 px is more honest than "Hand-woven Ramie from Jeju" at 24 px.

**Don't** symmetrize the layout to "look professional." Professionalism is the enemy of the work. The left margin is not equal to the right margin. The header is not centred above the content. The grid is something you cut against.

**Don't** explain the collection in English. The designer is Korean. The text is Korean. If a non-Korean-speaking viewer cannot read it, they can look at the photographs — which is the point. Translation is available as a single toggle, hidden at the bottom, that shifts the language without changing anything else.

**Don't** add hover animations, loading spinners, or scroll-triggered reveals. The page loads. The content is there. The viewer scrolls. Nothing performs. The garments do not need your help to be seen.

**Don't** round the contact rail or any container. Zero radius is non-negotiable. The rectangle is honest about its artificiality.