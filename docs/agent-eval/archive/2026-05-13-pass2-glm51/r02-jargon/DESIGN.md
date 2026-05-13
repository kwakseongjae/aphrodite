---
name: "neo-baroque-editorial"
version: "0.1"
description: "Pentagram-inspired neo-baroque annual report system with restrained bronze accent and ascending modular type scale"
colors:
  primary:
    "50": "#fdf6f0"
    "100": "#f7e8d8"
    "200": "#edc9a6"
    "300": "#dba76e"
    "400": "#c88b48"
    "500": "#8b6914"
    "600": "#6d5210"
    "700": "#513d0c"
    "800": "#372908"
    "900": "#1e1604"
  neutral:
    "0": "#ffffff"
    "50": "#faf8f5"
    "100": "#f0ede7"
    "200": "#dbd5ca"
    "300": "#c2b9ab"
    "400": "#a69b8a"
    "500": "#8d806e"
    "600": "#746854"
    "700": "#5d5242"
    "800": "#483f32"
    "900": "#342d23"
    "1000": "#1a160f"
  accent:
    "50": "#fdf8f1"
    "100": "#f8ecce"
    "200": "#f0d48e"
    "300": "#e6b94f"
    "400": "#d4a029"
    "500": "#9e7a1f"
    "600": "#7a5e18"
    "700": "#594412"
    "800": "#3a2e0d"
    "900": "#1f1807"
typography:
  display:
    family: "'Luminance Garamond', 'URW Garamond No 8', 'EB Garamond', 'Merriweather', serif"
    weight: 700
  body:
    family: "'DM Sans', 'Nunito Sans', 'Source Sans 3', sans-serif"
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
rounded:
  sm: "2px"
  md: "4px"
  lg: "6px"
metadata:
  variants:
    light:
      description: "Warm parchment with deep espresso text — the editorial default"
      tokens:
        colors.background.primary: "#faf8f5"
        colors.text.primary: "#1a160f"
        colors.text.secondary: "#5d5242"
        colors.border.default: "#dbd5ca"
    dark:
      description: "Charcoal slate with warm ivory type — nocturnal baroque"
      tokens:
        colors.background.primary: "#0d1216"
        colors.text.primary: "#ede8e0"
        colors.text.secondary: "#9a948c"
        colors.border.default: "#2a2d31"
    brand-a:
      description: "Buttered cream with burnt umber — warmest archival tone"
      tokens:
        colors.background.primary: "#f5f0e6"
        colors.text.primary: "#2a1e10"
        colors.text.secondary: "#6b5a48"
        colors.border.default: "#d4c9b5"
    brand-b:
      description: "Fog and verdigris — cool museum variant with aged copper"
      tokens:
        colors.background.primary: "#f0f2ed"
        colors.text.primary: "#12181a"
        colors.text.secondary: "#4e5557"
        colors.border.default: "#c5c9c2"
---

# Overview

This system channels the restrained grandeur of Pentagram's annual report work — where historical typographic gravity meets Swiss precision. The neo-baroque impulse is expressed not through ornament, but through scale contrast and the muscularity of the serif at display sizes. It is editorial first, digital second.

The primary hue family is deliberately earth-bound: bronze and umber rather than gold, avoiding the obvious metallic cliché. Accent color is a muted dandelion-bronze used at trace levels — a single hairline rule, a hovered link state, an ephemeral marker. The warmth is felt rather than seen.

Four variants cover the necessary range. Light and dark anchor the system. Brand-a pushes toward a more golden, archivally warm register. Brand-b cools the temperature entirely — museum fog, aged copper, zinc — providing a counterpoint that keeps the system from drifting into monochrome warmth.

# Colors

The bronze primary scale reads almost as a warm neutral until paired with brighter accents. Shades 50 through 200 function as tinted backgrounds and subtle fills. The middle range (300–500) carries interactive and accent responsibilities. The deep end (700–900) anchors text and rules.

Neutral tones run warm across the board — even the dark scale carries faint umber undertones rather than pure carbon. This keeps the system feeling printed rather than rendered, inked rather than illuminated. The neutral scale also doubles as a secondary tonal palette for illustrations and data visualization where the primary bronze would be too assertive.

The accent gold appears at exactly three levels of intensity: ghost (100–150) for background washes, quiet (300–400) for interactive highlights, and deep (600–700) for text on light surfaces. It should never appear at 500 on its own — always modulated by context.

# Typography

The display serif is chosen for its generous x-height and slightly condensed proportions at large sizes — a workhorse Garamond lineage that carries baroque authority without decorative excess. At display scales (48px and above), the letterforms breathe with historical confidence. Below 18px, the serif hands off cleanly to the body sans-serif.

The body face is a geometric-neutral sans with slightly rounded terminals. Its warmth complements the serif without competing. The typographic mood is understated competence — the sort of pairings you see in Finn̈iκ Pentagram publications where the sans is barely noticed and the serif does all the talking.

The modular type scale ascends at a 1.250 ratio (major third), creating deliberate stepwise drama. This is not a smooth gradient but a staircase — each level feels like a deliberate promotion. Display text commands attention not through color but through sheer gravitational mass. Body text at 16–18px sits quietly in the warm neutral range.

# Layout

The grid is built on an 8px base unit with deliberate asymmetry at the page level. Content regions favor a 5/8 split rather than symmetry — the golden ratio approximated without dogma. Margins breathe generously; the design earns its baroque character partly through the courage of empty space.

Vertical rhythm follows the modular type scale. Spacing between sections maps to type sizes: a 24px gap follows 18px body text, a 48px gap precedes an h2 at 32px. This creates a predictable cadence that readers feel rather than see, the way a well-set page feels inevitable.

Full-bleed elements are used sparingly and intentionally — a single hero image, a tinted divider band, a pull-quote breaking the margin. Each breach of the grid is a calculated editorial decision, not a default layout behavior.

# Elevation & Depth

Elevation is minimal and tonal rather than shadow-based. Layering is communicated through background warmth shifts — a card sits on a slightly cooler neutral than its page, creating recession without the theatricality of drop shadows. This approach is rooted in print, where depth comes from ink density, not projected light.

When shadows are used, they are warm and extremely tight — a 2px offset with 4px blur in a umber-tinted tone. They signal interactive affordance rather than spatial hierarchy. The deepest elements in the system use no more than two stacked shadow values.

Overlay states use the primary bronze at 5–10% opacity over white, creating a stained-parchment effect. This keeps the system's material language consistent — everything feels like it exists on a single continuous surface, folded and creased rather than floating in space.

# Shapes

Corner radii are almost vestigial — 2px to 4px at most. The system leans into the rectilinear tradition of editorial design. Rounded corners would undermine the gravitas; the slight radiusing merely prevents optical harshness at small sizes, a technical correction rather than a design statement.

Buttons and interactive elements use the 4px radius consistently. Containers and cards use 2px. The only exception is portrait-ratio image containers, which may use 6px to acknowledge the softer content within. These rules exist so that roundedness is never a variable the designer has to think about.

Dividers and rules are always full-width within their container and 1px thick. The accent gold may appear as a rule, but never thicker than 1px. Thicker rules default to the neutral scale. This restraint is what prevents the system from sliding into costume-baroque.

# Components

Buttons exist in two weights: filled (primary bronze on cream text) and ghost (bronze border, transparent fill). Both use the tight 4px radius and the body sans-serif at semibold weight. Hover states increase the fill warmth by one shade. There is no third button variant — the system values scarcity.

Pull-quotes are the signature component. They break the left margin, run in the display serif at 36–48px, and are preceded by a 1px gold rule exactly 48px wide. The quote text sits in primary-700. Attribution sits below in the sans-serif at 12px, neutral-400. This component alone should communicate the system's editorial ambition.

Navigation is minimal and horizontal, set in the body sans-serif at 14px with generous tracking (0.08em). Active states use the accent gold underline, 2px thick. Hover states shift text to primary-700 without underline. The nav should disappear into the page architecture rather than announcing itself.

# Do's and Don'ts

Do let the serif breathe at display sizes. Do restrict the gold accent to thin rules, small text, and hover states. Do maintain warm undertones even in the dark variant. Do use scale contrast as the primary tool for establishing hierarchy. Do treat every element as if it were going to be printed on archival paper.

Don't pair the display serif with another decorative face. Don't use the gold at saturation levels above 400 on large surface areas. Don't introduce rounded corners beyond 6px. Don't use pure black or pure white — always bias toward the warm or cool neutral. Don't add ornamental borders, filigrees, or decorative rules — the baroque quality comes from scale and restraint, not applied decoration.