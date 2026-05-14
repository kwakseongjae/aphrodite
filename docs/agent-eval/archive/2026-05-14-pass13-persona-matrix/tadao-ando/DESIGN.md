---
name: "jongno-gil"
version: "0.1"
description: "Portfolio for an independent architecture studio in Jongno-gu, Seoul — concrete, hanok tradition, and the light between walls"
colors:
  primary:
    "50":  "#f5f2ec"
    "100": "#e8e2d6"
    "200": "#d1c7b4"
    "300": "#b5a68c"
    "400": "#968264"
    "500": "#6b5a3e"
    "600": "#554832"
    "700": "#3e3526"
    "800": "#2c2519"
    "900": "#1a1816"
  neutral:
    "0":    "#ffffff"
    "50":   "#f8f7f5"
    "100":  "#eeece8"
    "200":  "#dbd8d1"
    "300":  "#c4bfb5"
    "400":  "#a39c8e"
    "500":  "#7d7668"
    "600":  "#5e584c"
    "700":  "#44403a"
    "800":  "#2e2b28"
    "900":  "#1a1816"
    "1000": "#000000"
typography:
  display:
    family: "Source Serif 4"
    weight: 700
  body:
    family: "Inter"
    weight: 400
  cjk-display:
    family: "Nanum Myeongjo"
    weight: 700
  cjk-body:
    family: "Nanum Myeongjo"
    weight: 400
spacing:
  "1":  "4px"
  "2":  "8px"
  "3":  "12px"
  "4":  "16px"
  "6":  "24px"
  "8":  "32px"
  "12": "48px"
  "16": "64px"
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
      description: "Concrete at dawn — dry light on raw walls"
      tokens:
        colors.background.primary: "#ffffff"
        colors.background.secondary: "#f5f2ec"
        colors.text.primary: "#1a1816"
        colors.text.secondary: "#5e584c"
    dark:
      description: "Night courtyard — shadow as material"
      tokens:
        colors.background.primary: "#121110"
        colors.background.secondary: "#1e1c18"
        colors.text.primary: "#eeece8"
        colors.text.secondary: "#a39c8e"
    brand-a:
      description: "Kiln-bleach gallery — the white room before the work enters"
      tokens:
        colors.background.primary: "#faf9f5"
        colors.text.primary: "#2c2519"
    brand-b:
      description: "Ondol warmth — the heated floor, the interior in winter"
      tokens:
        colors.background.primary: "#f2ede3"
        colors.text.primary: "#1a1816"
---

# Overview

This system designs for a studio that works in concrete and in the lineage of Korean hanok — two materials that share an ethic of honesty. Concrete records its formwork; hanok records its joinery. Neither hides behind finish. The digital surface should share that discipline. Every token here exists because the architecture demands it, not because a design system convention supplies it.

The studio sits in Jongno-gu, Seoul, among the palace walls and the low tile roofs of Bukchon. The palette is drawn from the warm-grey of cured concrete, the ochre undertone of exposed aggregate, and the deep wabi black that Korean craftsmen call jache — the lacquer-black that is never truly black, always warmed by the resin beneath. The primary hue family is amber-earth, not the cooler stone-greys one might first reach for. Concrete in Korea reads warmer than concrete in Osaka or Los Angeles; the aggregate is different, the light is different.

Typography carries the structural weight. For Latin text, Source Serif 4 at display sizes reads like letterforms cut into formwork — the optical sizing keeps the thins from collapsing. For Korean text, Nanum Myeongjo — a serifed Hangul face whose strokes have theSquare-cut termination of a chisel mark on stone. Never use a gothic (sans-serif) Hangul at display scale; it denies the material weight that serif strokes provide. The body face is Inter, chosen because it is nearly invisible at light weights, and body copy in an architecture portfolio should not compete with the photography.

Spacing is architectural. The largest token is 224px — nearly a room. The spacing between sections of this page should feel like passing through a doorway: a moment of transition that prepares the eye for what comes next. Smaller gaps are derived from the same proportional system, descending by ratios close to 1.618 where the mathematics serve the rhythm.

# Colors

The primary palette is keyed to raw concrete cured against timber formwork. The lightest shade (50) is the colour of a wall receiving full north light. The mid-tone (500) is the aggregate visible when you stand close enough to see the sand and stone. The deepest shade (900) is jache-black — the colour of Korean lacquerware, oxidized iron, the interior of a traditional storage jar. It reads as black but carries enough warmth to sit alongside concrete without the friction that a blue-black would introduce.

Neutrals are shared between concrete-grey and the yellow-grey of old hanji paper. At their lightest, they are indistinguishable from white. At their darkest, they converge with the primary 900 — the system does not need two different blacks. This unity is deliberate. The studio's material palette does not distinguish between structure and surface, and the colour system should not either.

Accent is handled by omission. A single warm-toned photograph — concrete illuminated by low western light, or the amber glow from a paper-covered window — provides all the colour the page needs. The design does not paint what photography can supply. If a project section requires a distinguishing mark, use the primary 500 as a rule-line, not a fill. The line is the architectural gesture: a joint between surfaces, a reveal between materials.

The four variants respond to light conditions. Light mode is the building at ten in the morning — full illumination, clear shadow edges. Dark mode is the same building at dusk, when the concrete becomes a dark mass and the windows glow. Brand-a is the gallery before installation — pure, expectant. Brand-b is ondol warmth, the Korean underfloor heating that makes a room livable in winter; it tints the background with the colour of warmth absorbed and re-radiated by stone.

# Typography

The display hierarchy uses two faces that never compete. Source Serif 4 handles all Latin display text — project names, section headings, the studio name itself. Its optical sizing means that at large scales the serifs sharpen rather than blob, preserving the sense of marks made by a tool. At body scale, Inter takes over at weight 300 or 400, light enough to recede behind the imagery.

For Korean text, Nanum Myeongjo serves both display and body roles. Hangul at display sizes does not need a different face — the stroke contrast inherent in a well-designed myeongjo (serif) provides the visual authority that Latin gets from weight and size. The characters are the architecture. A single line of Hangul at 48px in Nanum Myeongjo carries more structural presence than any photograph at thumbnail scale.

The vertical rhythm is anchored to a baseline grid derived from the leading of the body text. If the body is set at 16px with 28px leading, all spacing tokens are multiples or fractions of 28. Headings snap to this grid. Captions sit on it. The space between a photograph and its caption is one baseline unit — not arbitrary padding, but a measured interval, like the reveal between a concrete wall and a timber ceiling.

Line length is held to 55–65 characters for body text, 35–45 for Korean body text (Hangul characters carry more visual information per glyph). Display headings may extend wider but should never exceed the width of their accompanying image. The photograph is the primary content; text serves it.

# Layout

The page is a sequence of rooms. Each project is a room. The hero is the entry court — the madang of a hanok, open to the sky, bounded on all sides. It is a single rectangle, full-width, with the studio name positioned not centered but aligned to the structural grid — typically offset left, as a gate is offset from the axis of a Korean courtyard.

The project grid uses a two-column layout at full width, with generous gutters. Not three columns — that is the rhythm of a catalogue, and this is a portfolio of built work that demands individual attention. Each project card is image-dominant: the photograph occupies four-fifths of the card's area. The title and a single line of metadata sit below, left-aligned, with no hover effects that obscure the image. The image is the content. Everything else is the wall it hangs on.

Between sections, the 224px spacing token creates what the Japanese call ma and the Korean call bae — the meaningful interval. This is not "whitespace" in the Western design-system sense. It is the spatial equivalent of silence between notes. A visitor scrolling through the page should feel the pause before they see the next project, just as one feels the threshold before entering a new room.

Mobile does not reorganize the architecture. The rooms become taller, narrower, but the sequence remains. The two-column grid collapses to one column — each project receives the full width of the screen, as each room receives the visitor alone. Spacing reduces proportionally but never below 96px between sections. Even on a telephone screen, the architecture must breathe.

# Elevation and Depth

Depth in this system comes from tonal contrast, not shadow. A concrete wall creates depth through the shadow its own mass casts. Similarly, the distinction between a background and a content area is achieved through a step in the neutral scale — from white to the 50 tint, or from the 50 tint to the 100 — not through a box-shadow property.

If a functional shadow is unavoidable — a modal dialog, a navigation overlay — it is a single hard-edged shadow at a short offset, mimicking the shadow cast by a concrete lid resting on a concrete box. No spread, no blur, no multi-layered shadow stacks. The shadow says "one surface is above another," nothing more.

The photographic content provides all the depth the page needs. A monochrome image of a concrete interior, shot with a single light source, contains deeper shadow than any CSS can render. The design frames that photography; it does not compete with it. Elevation tokens are therefore minimal: two levels, surface and raised, separated by one tonal step and one hard shadow.

Z-axis layering is reserved for the navigation overlay, the project detail modal, and nothing else. The portfolio page itself is flat — a wall with photographs and text. This flatness is honest. The screen is a surface. Pretending otherwise through skeuomorphic shadow systems would be the equivalent of painting a fake shadow on a real wall: a decoration that acknowledges neither the light nor the material.

# Shapes

The border-radius tokens are intentionally severe. A maximum of 6px — the radius of a slightly worn concrete edge, not the radius of consumer electronics. Most elements use 2px or 4px, the barely-perceptible rounding that indicates a finished surface without announcing itself as curved. Sharp corners (0px) are available and preferred for image containers, where the photograph should read as a rectilinear opening in the page surface, like a window in a concrete wall.

The rectangle is the primary shape. The hero is a rectangle. The project cards are rectangles. The text columns are rectangles. Circles are reserved for the studio's mark — a single circular element, if the logotype demands it — and for nothing else. A circle in a field of rectangles reads as an opening, an oculus, the circle-of-light motif that appears in so much of the studio's built work. Use it once, with intent.

Diagonal elements, rounded blobs, and organic shapes do not exist in this system. They are the visual language of fluidity and dynamism, which is not the language of concrete. Concrete is poured liquid that becomes solid; its final form is always geometric. The shapes on this page should feel the same — deliberated, set, permanent.

Button shapes follow the same logic: tight radius, substantial padding (the touch-target minimum is met through generous horizontal and vertical padding, not through enlargement of the button shape itself), and a border rather than a fill when the button is secondary. A filled button uses the primary 900 on the light background, or the primary 50 on the dark background — the lightest possible mark on the darkest surface, or vice versa.

# Components

The navigation is a horizontal bar, fixed, containing the studio name on the left and three to four links on the right. It has no background fill when the page is at the top; as the user scrolls, it acquires the background-primary colour at slight opacity. This mimics the way a roof edge becomes visible only when you step back from the building. There is no hamburger menu at desktop scale; the links are few enough to display. On mobile, a single menu icon — Lucide's `menu` — opens a full-screen overlay with the links centered vertically, like text inscribed on a wall.

The project card is a figure containing a photograph, a caption (project name in the display face), and a single line of metadata (year, location, material — never more than three data points). The photograph is the interactive element; hovering or focusing it reveals nothing — the image is already complete. Clicking navigates to the project detail. The card has no border, no shadow, no decorative frame. It is a photograph on a wall.

The project detail view is a vertical sequence of full-width images interspersed with short text passages. The text is limited: a project description of no more than 150 words, set in a narrow column centered on the page, like a caption panel beside a building section drawing. Structural metadata (area, materials, completion date, location) is listed in a single column, set in the body face at small size, uppercase labels with the values in title case. This is the drawing-title-block convention translated to the screen.

The contact section is plain text: studio name, address in Jongno-gu, telephone number, email address. No form widget. No "get in touch" heading. The contact information is presented as it would appear on a nameplate beside a studio door. A person who wants to reach the studio will copy the address or dial the number. A form would be a barrier dressed as convenience.

The footer is a single line: the studio name, the current year, and a small Lucide `arrow-up` icon linked to the top of the page. It is the equivalent of the small marker a mason leaves at the corner of a foundation — present, minimal, oriented.

# Do's and Don'ts

**Do** let each project photograph occupy as much space as the viewport allows. The work is built at architectural scale; representing it at thumbnail scale is a disservice. Full-bleed images, generous aspect ratios (4:5 for portraits of spaces, 16:10 for landscape site contexts), and no decorative overlays.

**Do** use Korean text wherever the context is Korean. A project in Jongno-gu should have its project name in Hangul first, with a Latin transliteration below if needed. The Hangul is not a translation — it is the name. Typography should reflect this hierarchy.

**Do** maintain the spacing discipline. When in doubt, add space rather than remove it. The ma between sections is what allows each project to be seen on its own terms. Crowding the work is crowding the visitor.

**Don't** add stat counters, client logos, testimonial quotes, or team photographs to the homepage. The studio's authority is in the built work, not in numerical claims. The portfolio page shows projects; the about page may show people. These are different rooms with different purposes.

**Don't** use parallax scrolling, scroll-triggered animations, or loading-screen progress indicators. Movement on the page should come from the visitor's scroll action alone. Animated elements that move independently of the user's intent are the equivalent of a building facade that moves — attention-seeking rather than attention-holding.

**Don't** apply texture overlays to simulate concrete, paper, or timber. The screen is a screen. If the photography is good, the material is present in the image. Adding a noise texture or a grain filter over the background is the faux-wood-paneling of digital design — a simulation that degrades the real thing it references.

**Don't** center-align long text. Body copy is ranged left, with a consistent left margin that establishes the reading line. Center alignment is reserved for single lines: the studio name in the hero, items in the mobile menu overlay, the caption beneath a centered photograph. Center-aligned paragraphs longer than three lines are walls without a plumb line.