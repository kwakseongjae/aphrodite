---
name: "cord-radio"
version: "0.1"
description: "Product site for a Tokyo studio's wall-mounted radio with pull-cord power switch. Uses Inter for all typographic families to achieve a transparent, 'without thought' reading experience."
colors:
  primary:
    "50": "#f7f3ed"
    "100": "#e8dfd3"
    "200": "#d4c7ac"
    "300": "#c0ad85"
    "400": "#ab935e"
    "500": "#8d7545"
    "600": "#7a6437"
    "700": "#67522d"
    "800": "#544123"
    "900": "#3d2f18"
  neutral:
    "0": "#ffffff"
    "50": "#fafaf8"
    "100": "#f3f2ef"
    "200": "#e6e5e0"
    "300": "#d0cec7"
    "400": "#a8a59c"
    "500": "#7d7a70"
    "600": "#5c5950"
    "700": "#3e3b35"
    "800": "#2a2824"
    "900": "#1a1816"
    "1000": "#0e0d0b"
  surface:
    "warm-white": "#faf8f4"
    "parchment": "#f3efe8"
    "clay": "#e4d9c8"
    "stone": "#bfb5a3"
    "bark": "#6b6255"
    "carbon": "#262420"
    "lamp-black": "#141310"
typography:
  display:
    family: "Inter"
    weight: 400
  body:
    family: "Inter"
    weight: 300
  caption:
    family: "Inter"
    weight: 300
spacing:
  "0.5": "2px"
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
rounded:
  none: "0"
  xs: "2px"
  sm: "4px"
  md: "8px"
metadata:
  variants:
    light:
      description: "Daylight — warm parchment, ink text"
      tokens:
        colors.background.primary: "#faf8f4"
        colors.text.primary: "#1a1816"
        colors.background.secondary: "#f3efe8"
        colors.text.secondary: "#6b6255"
    dark:
      description: "Evening — lamp-black, warm paper light"
      tokens:
        colors.background.primary: "#141310"
        colors.text.primary: "#e6e5e0"
        colors.background.secondary: "#262420"
        colors.text.secondary: "#a8a59c"
    brand-a:
      description: "Warm charcoal — tinted ground"
      tokens:
        colors.background.primary: "#f3efe8"
        colors.text.primary: "#262420"
        colors.background.secondary: "#e4d9c8"
        colors.text.secondary: "#5c5950"
    brand-b:
      description: "Cool parchment — aspirational gallery tone"
      tokens:
        colors.background.primary: "#f7f3ed"
        colors.text.primary: "#2a2824"
        colors.background.secondary: "#e8dfd3"
        colors.text.secondary: "#67522d"

---

# Overview

This is a site for one object. A wall-mounted radio. The pull cord turns it on. That single gesture — the reaching, the pulling — is the entire design philosophy made physical. The website must honor that restraint. One product. One action. One reading direction.

The design borrows its calm not from minimalism as style but from the observation that a well-made tool does not announce itself. You walk past the radio on your wall a hundred times. You notice it only when you need it. The site should behave the same way: present when looked at, invisible when not.

We use Inter Regular for display text. Fukasawa's principle guides this choice: type should disappear the way a well-designed kettle handle disappears in the hand. A Renaissance serif calls attention to itself — it has historical posture, decorative weight, a personality that insists on being noticed. That insistence violates the 'without thought' register this product demands. Inter at regular weight carries warmth without performance. It is a tool doing its work quietly. The body text is Inter Light at 14px. It recedes further. You read it without noticing you are reading.

Photography shows the radio mid-use. A hand pulling the cord. Morning light on a kitchen wall. The cord hanging still. Never the product isolated on seamless white — that is catalog photography, the photography of warehouses, not of homes.

# Colors

The palette is drawn from the materials the radio is made from: warm plastic housing, fabric cord, the plaster wall it hangs on. Primary is a quiet umber — not orange, not brown, but the color of unbleached linen or raw clay. It references the pull cord's fabric sheath without illustrating it.

Light mode grounds on warm white with ink-dark text. Dark mode inverts to lamp-black — the color of a room at dusk with one lamp on — with text the color of aged paper. Both meet WCAG AA contrast at all sizes. The accent color is used only for active states and the cord motif in illustrations. It appears sparingly. A colored surface is a raised voice; we speak quietly.

Brand-a shifts to a parchment ground with charcoal text, evoking the studio's workshop. Brand-b cools slightly, moving toward the neutral-warm tone of gallery walls where the product might be exhibited. Both variants maintain the same restraint: no more than three values on any single screen.

# Typography

Inter Regular at weight 400 for display. The same family as body, one step heavier. This is not economy — it is consistency. The typeface does not shift its identity between heading and paragraph. The hierarchy is communicated through size and spacing alone, which is how a well-made object communicates: through proportion, not decoration.

Inter Light for body at 14px with 1.65 line-height. This is the minimum comfortable size for extended reading at arm's length. Larger sizes signal importance through size alone, which is lazy. Importance here is communicated through placement — the first thing on the page is the most important thing, regardless of its point size.

For Japanese text, fall back to Hiragino Sans W3. For Korean, Pretendard Light. Never bold. Never semi-bold. The weight hierarchy is established through size and spacing alone. A heading at 24px and a body at 14px are already different enough. Adding weight to the heading is doubling the signal — it means you do not trust the size.

Captions sit beneath photographs in 12px Inter Light, set in the secondary text color. They name gestures, not features: "Cord at rest — power off." "Hand reaches — power on."

# Layout

Single column. The reading order matches the use order: you see the radio, you understand the gesture, you learn the details, you find the studio. No sidebar. No grid of features. No comparison table. The radio is the only product; there is nothing to compare it to.

Vertical rhythm is generous. The space above a section heading is three times the space below it. The product photograph holds a 1:3 ratio of product to surrounding background — the object floats in its environment the way it floats on your wall. This negative space is not emptiness; it is the room the object will occupy when you take it home.

Maximum content width is 600px. Wider measures invite distraction. The eye should travel in one direction: down. On mobile, the same column narrows naturally with increased padding at the edges. The pull-cord motif — a thin vertical line, or a single dangling curve — may appear as a sectional divider, drawn in the neutral-300 color, 1px weight. It is the only decorative element on the entire site.

# Elevation & Depth

None. The site is flat. Drop shadows model real light, and a screen is not a surface light falls on. Where depth is needed — a floating navigation bar, a modal for the purchase flow — it is expressed through a 1px border in neutral-200 on the light variant, or neutral-700 on the dark variant. The border is enough.

The single exception: product photography contains natural shadows from the studio lighting where the object was photographed. These shadows belong to the photograph and are not duplicated in the interface layer. The interface does not compete with the image.

Layering follows a simple stack: background, content, navigation. Navigation is fixed to the top of the viewport with a semi-transparent background that blurs the content beneath it. This is the only blur on the site. It is functional — it keeps text readable — not decorative.

# Shapes

All corners are sharp or nearly sharp — maximum radius of 4px, used only on interactive elements like buttons. The radio itself is a rectilinear object; the site's geometry should share its precision. Rounded corners on a page about a wall-mounted rectangular object would be a small dishonesty.

Buttons are the shape of their touch target: 44px minimum height, full-width on mobile, auto-width on desktop. The label sits centered in Inter Light, 14px, with 16px horizontal padding. The background is neutral-800 on light mode, neutral-100 on dark mode. The text inverts. There is no icon inside the button. The arrow-right Lucide icon may follow the label at 16px, stroke-width 1.5, to indicate direction — but only on the primary action.

The pull-cord divider mentioned in Layout is a single SVG curve — a catenary, the shape a cord makes when hanging from two points. It appears between the hero photograph and the details section. It is 60px wide, centered, drawn in neutral-300 at 1px stroke. It is the only illustrative element on the page. One cord, one curve.

# Components

Navigation: a horizontal bar, left-aligned studio name in Inter Regular 16px, right-aligned links in Inter Light 13px. Links: "Radio," "Studio," "Contact." No underline. Hover state: neutral-500 text color for 200ms. Active page indicated by neutral-400 color, not bold or underline.

Hero: full-viewport height on initial load. The product photograph — the radio on a real wall, cord visible, natural light — occupies the center. No headline overlays the image. Below the image, a single line of text in Inter Regular 20px: "Pull to listen." This is the only text on the hero screen.

Details section: a sequence of photographs, each showing the radio in a different domestic context — kitchen, bedroom, entryway. Each photograph is accompanied by a gesture-caption in 12px. Between photographs, a short paragraph in Inter Light 14px describes the object's construction. No bullet points. No specification table. Specifications are listed in a quiet row at the bottom: dimensions, weight, frequency range — set in the caption style.

Purchase: a single button at the bottom of the details section. It reads "Order" in the studio's language. No price on the page; the price is on the order page. The button is the full content width, neutral-800 background, warm-white text. Clicking it opens a clean form — name, address, payment — in the same column, replacing the details content. No modal. No redirect to a separate storefront.

Footer: studio name, address in Tokyo, a Lucide map-pin icon at 16px, and a mail icon linking to contact. Set in caption style. Centered. Sixty-four pixels of padding above.

# Do's and Don'ts

Do photograph the cord at rest and in motion. The cord is the interface; showing it twice teaches the product without words. Don't show the radio from more than three angles. Three is enough to understand the object. More is anxiety.

Do let the page breathe. The silence between sections is the sound of the room before you pull the cord. Don't fill negative space with illustrations, patterns, or brand marks. Emptiness here is communicative.

Do use the word "pull" more than once if needed. Don't use the words "innovative," "minimalist," "iconic," or "designed in Tokyo." The product speaks. The city does not need to speak for it.

Do set the body text at 14px and trust the reader. Don't increase the size because you fear it is too small. Fear of small type is fear of quiet. This site is quiet.

Do treat the dark variant as the evening version of the same room. Don't treat it as a dramatic inversion. The dark mode is not a different identity; it is the same room after sunset. The cord is still there. The radio still plays.