---
name: "seoul-walnut-portfolio"
version: "0.1"
description: "Portfolio site for an independent furniture maker working in solid walnut, based in Seoul"
colors:
  primary:
    "50": "#f2f7f1"
    "100": "#e4efdf"
    "200": "#c8dfbf"
    "300": "#a6c996"
    "400": "#82b06d"
    "500": "#5a8a45"
    "600": "#487038"
    "700": "#39572d"
    "800": "#2d4525"
    "900": "#1a2a16"
  neutral:
    "0": "#ffffff"
    "50": "#f7f7f5"
    "100": "#edede8"
    "200": "#d6d4cc"
    "300": "#b5b1a5"
    "400": "#8f897b"
    "500": "#716b5e"
    "600": "#5a5549"
    "700": "#46423a"
    "800": "#33302b"
    "900": "#1e1c19"
    "1000": "#0a0a08"
typography:
  display:
    family: "Instrument Serif"
    weight: 400
  body:
    family: "Inter"
    weight: 300
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
  sm: "0px"
  md: "2px"
metadata:
  variants:
    light:
      description: "Default light mode — warm neutral gallery wall"
      tokens:
        colors.background.primary: "#f7f7f5"
        colors.text.primary: "#1e1c19"
    dark:
      description: "Dark mode — deep warm charcoal"
      tokens:
        colors.background.primary: "#1e1c19"
        colors.text.primary: "#edede8"
    brand-a:
      description: "Near-white gallery — exhibition context"
      tokens:
        colors.background.primary: "#faf9f6"
        colors.text.primary: "#2d2a24"
    brand-b:
      description: "Sage-tinted — material echo of workshop patina"
      tokens:
        colors.background.primary: "#f4f5f0"
        colors.text.primary: "#2a2c20"
---

# Overview

This is a portfolio for a furniture maker. Nothing more. The site exists to present photographs of walnut furniture and the text necessary to name each piece, its dimensions, its material, and its year. Every decision follows from that purpose.

The palette is built on a muted sage green — the colour of walnut leaves before they darken, the colour of workshop patina on a well-used chisel handle. It is not decorative. It is the material's own register, abstracted. Warm neutrals carry the background. The green appears only where a signal is needed: an active state, a link, a selected filter.

Instrument Serif at display weight is chosen deliberately against my usual preference for sans-serifs. A furniture maker works in a discipline older than modernism. The serif acknowledges that lineage without nostalgic ornament. Its optical sizing holds at large scales. Inter at light weight handles everything else — captions, navigation, contact information. Two faces. No more.

# Colors

The primary scale runs from a near-white sage at 50 to a deep forest at 900. Use 500 for interactive accents. Use 700 and above for small labels where the green must read against light backgrounds. Use 50–100 for subtle background tints on hover states or card surfaces.

Neutral carries a warm bias — the 50 is not pure grey but a faintly warm off-white, keyed to the colour of oiled walnut under gallery lighting. This warmth must persist across all variants. Pure grey would flatten the photography it surrounds.

The four variants serve distinct contexts. Light is the default. Dark inverts for low-light viewing. Brand-a is the gallery-white exhibition context — the closest to a neutral wall a collector would hang work against. Brand-b introduces the faintest sage tint, echoing the workshop environment where sawdust meets light. All four pass WCAG-AA contrast between their primary text and background. This is non-negotiable.

Restrict colour usage to three roles: background, text, accent. If an element carries colour outside these roles, remove it.

# Typography

Instrument Serif at 400 weight for display headings. It is not bold. It does not need to be. Its authority comes from scale and spacing, not weight. Set display text at 48px minimum, with 120px as the upper bound for hero statements. Letter-spacing at negative values — tight, not loose. The serif draws the eye; the spacing gives it room to breathe.

Inter at 300 weight for body. Light weight, not regular. The contrast between the serif display and the light sans body establishes hierarchy without relying on size alone. Body text at 16px, line-height at 1.5. Captions and metadata at 13px, letter-spacing opened slightly to 0.02em.

Navigation is set in Inter at 400 weight, 13px, all-caps, letter-spacing 0.08em. This is the only place all-caps is permitted. It marks navigation as functional infrastructure, distinct from content.

# Layout

A twelve-column grid at 1280px maximum width. Gutters at 24px. Margins scale from 32px on mobile to 64px on desktop. These are facts, not suggestions.

The hero occupies full width. A single line of display text — the maker's name — sits above a full-bleed photograph of a single piece. No overlay. No gradient. The photograph stands on its own. Below, a narrow text rail on the left (4 columns) carries a one-sentence description of the practice. The right (8 columns) holds the image. This ratio inverts for individual project pages: text on the right, image on the left. The reversal signals transition without announcing it.

Selected work follows. A grid of project cards: full-bleed image, no border, no shadow. Below each image, the piece name in Instrument Serif, the material and year in Inter light at 13px. Spacing between cards at 48px. Spacing between sections at 128px. Magazine pacing demands room. Cramped spacing is a failure of confidence.

Contact is plain text. An email address. A phone number. A workshop address in Seoul. Set in Inter at body weight, centred. No form. No submit button. A person calls another person. The page does not mediate.

# Elevation & Depth

Elevation is not used. There are no drop shadows. If a component needs to appear elevated, use background colour or a single-pixel border in neutral-200. Photographs sit flush against the background. Cards have no shadow. Modals do not exist in this system.

Depth is conveyed through the photography itself. The furniture carries shadow, texture, grain. The interface does not compete with it.

This is a direct conflict with common design-system practice, where elevation tokens denote interactive layers. I reject that convention here. A furniture portfolio is not a dashboard. Layers and z-indices add complexity without serving the viewer. The work sits on a surface. The surface is flat.

The only border permitted is a 1px line in neutral-200, used sparingly to separate navigation from content, or to delineate a project caption from the image above. Even this should be questioned before implementation.

# Shapes

Radius is 0px by default. Sharp corners. Rectilinear forms. Radius 2px is permitted on interactive elements — buttons, input fields — where a literal sharp corner would create visual noise at small scales. Nowhere else.

This follows from the material. Walnut joinery is square. Dovetails, mortises, tenons — the geometry is orthogonal. The interface mirrors the workshop. A rounded corner on a furniture portfolio is a lie about the work.

Cards are rectangles. Buttons are rectangles with 2px radius. The viewport itself is a rectangle. The grid is rectangular. Consistency in form is not a stylistic choice; it is intellectual honesty about the medium.

# Components

**Navigation.** A horizontal bar, fixed to the top. Left: the maker's name in Instrument Serif. Right: page links in Inter, all-caps. Background matches the variant's primary background. No blur. No transparency. A 1px bottom border in neutral-200 separates it from content. Links underline on hover with primary-500. No other animation.

**Project card.** A rectangular image with a 4:5 aspect ratio. Below: piece name, material, year. No overlay on hover. The cursor changes to a pointer. Clicking navigates to the project detail. The card is a door, not a display case.

**Project detail.** Full-width image at the top. Below, a narrow column of text: dimensions, wood source, finishing method, year. Below that, a grid of detail photographs — joints, grain, edges — at smaller scale. The page tells the whole truth about one piece. Nothing is hidden. Nothing is embellished.

**Contact section.** Three lines of text, centred. Email, phone, address. An optional line in primary-500 for current availability or lead time. This section is the last thing on the page. It does not repeat.

**Image placeholder.** During development, before the maker's photography is ready, use a solid block in neutral-100 with centred text describing the intended photograph. Example: "Hakwi Dining Table — black walnut, oil finish, 2024. Full-length side view." The text serves as an art direction brief. When real photography arrives, it replaces the block exactly.

# Do's and Don'ts

Do let the photography dominate every page. The furniture is the content. The interface is the frame. A frame that draws attention to itself has failed.

Do use white space as the primary compositional tool. Space between sections at 128px minimum. Space between projects at 48px. Space within project detail at 32px. Generous spacing is not emptiness — it is the rhythm of a well-paced publication.

Do write in direct statements. "Walnut dining table, 1800 × 900 × 750mm, oil finish, 2024." Every word is a fact. If a word is not a fact, remove it.

Don't add stats rows. No "12 years of practice" or "87 pieces delivered." The work is the metric. If the work does not speak, no number will compensate.

Don't use testimonials or client logos. This is not a B2B service page. A collector does not need social proof. They need to see the furniture.

Don't animate scroll. No parallax, no fade-in on scroll, no sticky elements that follow the viewport. The page scrolls at the speed the reader chooses. Animation that the reader did not request is interference.

Don't use the primary green as a background colour for large areas. It is a signal colour. Signals are only useful when they are rare. A green background is no longer a signal — it is a decision about decoration, and decoration is not justified here.

Don't round corners beyond 2px. Don't add shadows. Don't use gradients. Don't centre body text. Don't use emoji. Don't write exclamation marks. Don't describe the work as "timeless" or "elegant" — if it is those things, the reader will know. If it is not, the word will not make it so.