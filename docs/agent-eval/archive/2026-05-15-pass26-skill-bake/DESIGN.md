---
name: "forma-industrial"
version: "0.1"
description: "Corporate identity site for a mid-century industrial design consultancy — authority, restraint, craft"
colors:
  primary:
    "50": "#f0faf6"
    "100": "#c8f0e0"
    "200": "#7de4bd"
    "300": "#2dd494"
    "400": "#00bf7b"
    "500": "#009966"
    "600": "#007a52"
    "700": "#005c3d"
    "800": "#003d29"
    "900": "#001f15"
  neutral:
    "0": "#ffffff"
    "50": "#f7f6f1"
    "100": "#e8e6df"
    "200": "#c9c5b8"
    "300": "#9e9889"
    "400": "#6b6560"
    "500": "#4a4540"
    "600": "#2a2724"
    "700": "#1a1816"
    "800": "#0f0e0c"
    "900": "#070706"
    "1000": "#000000"
typography:
  display:
    family: "Garamond, ITC Garamond, EB Garamond, Georgia, serif"
    weight: 700
  body:
    family: "Garamond, ITC Garamond, EB Garamond, Georgia, serif"
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
rounded:
  sm: "2px"
  md: "4px"
metadata:
  variants:
    light:
      description: "Default — warm bone paper, ink-black text, viridian accent"
      tokens:
        colors.background.primary: "#f7f6f1"
        colors.text.primary: "#1a1816"
    dark:
      description: "Reversed — charcoal ground, parchment text, viridian accent"
      tokens:
        colors.background.primary: "#0f0e0c"
        colors.text.primary: "#e8e6df"
    brand-a:
      description: "Studio warm — cream paper with olive cast"
      tokens:
        colors.background.primary: "#f2f1eb"
        colors.text.primary: "#1a1816"
    brand-b:
      description: "Studio cool — pale mint ground, dark text"
      tokens:
        colors.background.primary: "#f0faf6"
        colors.text.primary: "#0f0e0c"

---

# Overview

This is a corporate identity site for a mid-century industrial design consultancy. The consultancy needs to present itself the way a Knoll catalog presents a chair — with the authority of a thing that has already been decided. Nothing provisional. Nothing trendy. The site does not persuade; it declares.

The palette is viridian green and ink-black on warm bone paper. This is not the expected red-and-black IBM lineage, nor the yellow-and-blue Vignelli lineage. Viridian is the colour of oxidised brass, of shop floors, of the green glass in a draftsman's lamp. It belongs to industrial practice, not to corporate branding textbooks. Paired with a Garamond serif — the typeface Rand used for his own Yale course materials — it establishes a register that is scholarly and material rather than slick and digital.

The layout follows a strict 12-column grid with content occupying seven columns and the remaining five left as structured breathing room. Nothing centres. Asymmetric composition is not an affectation — it is the correct response to a rectangular viewport. Centre-alignment is the default of people who have not made a decision.

A single geometric element — a solid viridian circle, approximately 120 px diameter — sits off-grid in the hero, positioned to the right of the headline and overlapping the second column boundary. It is the only decorative mark on the page. It does not illustrate anything. It is there because the page needed one thing that was not type, and a circle is the most neutral shape that exists.

# Colors

The system uses two colours with intention: viridian (#009966) as the single saturated accent, and ink-black (#1a1816) as the primary text colour on warm bone (#f7f6f1). There is no third hue. A third hue would require a structural reason — a client colour, a section divider that cannot be solved by spacing or weight. If you find yourself reaching for a third colour, you have not solved the layout problem.

Viridian at shade 500 reads as confident without aggression. It is not the green of nature or sustainability — it is the green of industrial enamel, of painted steel, of the(patina on a bronze casting. The tint scale (50–400) provides backgrounds for hover states and caption strips. The shade scale (600–900) provides text on light backgrounds when black would be too severe.

Warm bone (#f7f6f1) replaces pure white. Pure white is a screen colour, not a paper colour, and this consultancy deals in physical objects. The slight yellow undertone is the colour of a drafting sheet, of acid-free archive stock, of the wall in a well-lit workshop. It changes the entire temperature of the page without changing any palette values.

In dark mode, the background drops to charcoal (#0f0e0c) and text lifts to the bone tint (#e8e6df). Viridian remains the accent at shade 400 for contrast on dark. The dark variant is not an inversion — it is the same room with the lights dimmed. The ratios hold. The hierarchy holds. The viridian circle stays viridian.

# Typography

Garamond is the sole typeface. Not a geometric grotesque, not a neo-grotesque. Garamond is what Rand set his own books in, what Yale Press used for decades, what reads as intellectual authority without shouting. Regular weight (400) for body text. Bold weight (700) for headlines and section heads. No medium. No light. Two weights solve the whole problem.

The type scale uses a 1.333 ratio against a 16 px base: 16, 21, 28, 37, 50, 67, 89. The hero headline sits at 96 px, which is the 89-step nudged upward to fill the architectural space. The lede sentence is 21 px. Body is 16 px. Captions and labels are 13 px. These four sizes are the only sizes. If you need a fifth size, you need fewer things on the page.

All type is set ragged-right, flush-left. Justified text is for newspapers with column constraints. On a screen, justified text creates rivers of whitespace that move as the viewport changes. Flush-left is honest about what it is. Line length maxes at 640 px — approximately 65 characters at body size. This is not a guideline; it is a hard limit.

For CJK surfaces — Korean project names, Japanese client names — use Sandoll Myungjo (Korean) or Hiragino Mincho Pro W6 (Japanese). These are Mincho faces that carry the same gravitas register as Garamond. Pair with the same viridian accent. Set at the same scale. Do not switch to a sans-serif for CJK; that would be a register violation.

# Layout

The grid is 12 columns with 32 px gutters. Content occupies columns 2 through 8 — seven columns, left-aligned, with five columns of structured whitespace on the right. On viewports below 1024 px, content expands to 10 of 12 columns. On mobile, content goes full-width with 24 px side margins.

Vertical rhythm uses 160 px section padding on desktop, 96 px on tablet, 64 px on mobile. These are generous. A corporate identity site needs air the way a gallery wall needs air — the space around the work is part of the presentation. Cramped sections signal anxiety. Generous sections signal confidence.

The hero block contains: one word at 96 px bold Garamond (left-aligned), a single sentence at 21 px below it, and the viridian circle positioned absolutely to the right of the headline, its top edge aligned with the x-height of the hero word. Below the hero, a thin viridian rule (2 px, spanning the full content column) separates the introduction from the body. This rule is the only horizontal line on the page.

Navigation is a single row: consultancy name on the left (Garamond bold, 18 px), three to five navigation links on the right (Garamond regular, 14 px, uppercase, 0.08em letterspacing). No hamburger menu on desktop. The mobile menu is a full-screen overlay with the same type at 32 px, vertically stacked. The footer is a colophon: name, address, contact, year. 11 px uppercase. One line if possible.

# Elevation and Depth

There is no elevation system. Elevation is a material metaphor that belongs to interfaces with layers — dashboards, tool palettes, card grids. This site is a single surface, like a printed page. Things are adjacent, not stacked.

The only depth cue is the viridian circle, which sits on top of the headline at reduced opacity (8%) when it overlaps text. This is not a shadow. It is a transparent overlay that allows the type to remain legible while confirming the circle's position in front of the text plane. If the circle does not overlap text, it sits at full opacity.

Hover states use background colour shifts (neutral-100 on light, neutral-700 on dark) rather than shadows or scale transforms. Focus states use a 2 px viridian outline offset by 2 px from the interactive element. These are accessibility requirements, not design features — they should be visible when needed and invisible when not.

# Shapes

All interactive elements — buttons, input fields, card boundaries — use 4 px border radius. This is the minimum rounding that reads as intentional rather than sharp. It is also the maximum. Anything above 4 px belongs to a consumer product, not a corporate identity system.

The viridian circle is the sole decorative shape. It is 120 px diameter on desktop, 80 px on tablet, 60 px on mobile. It does not have a border radius because circles do not need one. It does not animate, rotate, pulse, or respond to scroll. It is a fixed element that anchors the hero composition. If the page had no circle, it would be only type on a ground — which would also be acceptable, but the circle gives the eye one non-verbal landmark.

Image containers are rectangular with 4 px radius. Photographs of the consultancy's work — products, signage, installations — are shown at large scale within the content column, captioned in 13 px below. No rounded image masks, no circular avatars, no parallax scroll effects on images.

# Components

The site requires very few components because a corporate identity site is fundamentally a document, not an application. The components that exist are: navigation bar, hero block, text section, image-with-caption, project card, and footer colophon.

Navigation bar: fixed to the viewport top, background matches the variant's primary background, 80 px height. Consultancy name left-aligned at 18 px bold. Links right-aligned at 14 px regular uppercase with 0.08em letterspacing. The active link is underlined with a 2 px viridian rule. The bar has a 1 px bottom border in neutral-200 (light) or neutral-700 (dark).

Project card: a rectangular block within the content column, containing a single photograph at full column width, a project title in 28 px bold Garamond, a one-line description in 16 px regular, and a viridian "View project" text link. No card border. Separation from the next card is achieved through 80 px vertical spacing. Cards do not hover, lift, or reveal. They are photographs with labels, which is how a Knoll catalog presents a chair.

Contact section: a single line of text in 16 px Garamond — "Reach us at" followed by an email address in viridian. Below it, a physical address and phone number in 14 px neutral-400. No contact form. A consultancy that designs corporate identities does not need a form with a dropdown for "project type." An email address is sufficient. The buyer is intelligent.

# Do's and Don'ts

Do set the hero word at 96 px and let it occupy the space it deserves. Do use Garamond for everything — headlines, body, captions, navigation. Do leave five columns of structured whitespace on the right side of every desktop page. Do place the viridian circle off-grid, overlapping the second column boundary, and never move it or animate it. Do use real photographs of real work at large scale. Do caption every photograph with factual text — "Identity system, applied to entrance signage, 240 × 60 cm" — and nothing more.

Do not introduce a third colour. Do not use gradients. Do not set any text in the geometric element. Do not centre-align anything. Do not use a geometric sans-serif when Garamond is available. Do not use testimonials, client logo strips, or social proof of any kind. Do not write copy that uses the words "innovative," "premium," "bespoke," "solutions," or "crafting." The consultancy's work is the proof. The presentation is the frame. The frame should be invisible.