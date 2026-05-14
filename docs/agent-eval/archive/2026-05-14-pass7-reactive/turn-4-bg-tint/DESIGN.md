---
name: "Walnut Seoul"
version: "0.2"
description: "Portfolio for an independent furniture maker in solid walnut, Seoul — expanded spacing scale for editorial gallery-wall pacing; brand-a variant background shifted to a cleaner near-white for neutral photography reproduction"
colors:
  primary:
    "50": "#f7f3ed"
    "100": "#ebe3d5"
    "200": "#d9c9b0"
    "300": "#c4aa85"
    "400": "#b29062"
    "500": "#9a7550"
    "600": "#7d5d3f"
    "700": "#614932"
    "800": "#4d3a2a"
    "900": "#3d2e22"
  neutral:
    "0": "#ffffff"
    "50": "#f8f7f5"
    "100": "#edebf0"
    "200": "#d4d2d8"
    "300": "#b0adb5"
    "400": "#8a8690"
    "500": "#67636d"
    "600": "#514d56"
    "700": "#3d3a42"
    "800": "#2a272e"
    "900": "#1a181e"
    "1000": "#000000"
typography:
  display:
    family: "Source Serif 4"
    weight: 400
  body:
    family: "Outfit"
    weight: 300
spacing:
  "1": "4px"
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
rounded:
  sm: "2px"
  md: "4px"
  lg: "6px"
metadata:
  variants:
    light:
      description: "Default light mode"
      tokens:
        colors.background.primary: "#f8f7f5"
        colors.text.primary: "#1a181e"
    dark:
      description: "Dark mode"
      tokens:
        colors.background.primary: "#1a181e"
        colors.text.primary: "#edebf0"
    brand-a:
      description: "Warm light variant"
      tokens:
        colors.background.primary: "#faf9f6"
        colors.text.primary: "#2a272e"
    brand-b:
      description: "Cool warm variant"
      tokens:
        colors.background.primary: "#f0ede8"
        colors.text.primary: "#3d3a42"
---

# Overview

This system serves a solitary craftsperson who shapes solid walnut into furniture across a small workshop in Seoul. The design must honour the material — its weight, its grain, its reluctance to be rushed. Every surface should feel like it has been considered with the same patience the maker applies to a dovetail joint.

Source Serif 4 was chosen deliberately over geometric or industrial sans-serifs. Its contemporary serif structure carries the warmth and materiality of a 20th-century workshop — think mid-century technical drawings and architectural lettering — without retreating into Renaissance literariness. The optical sizing axis ensures it reads well at display scales while maintaining clarity. Paired with Outfit at a light weight, the typographic palette avoids the cold precision of tech-oriented portfolios and instead evokes the quiet confidence of a well-made object. The restraint is intentional — this is a maker who lets the work speak.

The colour system avoids blue entirely. The warm taupe primary draws directly from the grey-brown undertone present in steamed walnut heartwood, while the cool violet-neutral greys reference the light of a Seoul winter morning. This interplay prevents the palette from becoming monotonous or predictable.

# Colors

Primary shades range from a pale linen at 50 to a deep espresso at 900, all rooted in the grey-brown ochre family. The 500 value sits firmly in a mid-tone that reads as a muted bronze rather than a conventional brand colour, providing warmth without shouting. This is a palette for backgrounds and accents that recede, allowing photography of finished furniture to carry visual weight.

The neutral scale leans cool — a deliberate counterpoint to the warm primary. At the dark end, the base carries a faint violet cast rather than pure black, which softens contrast in dark mode and prevents the interface from feeling cavernous. In light mode, the off-white base avoids clinical brightness.

The brand-a warm light variant uses a cleaner near-white (#faf9f6) rather than a yellower parchment tone. This slight warmth reads as gallery wall rather than aged paper, ensuring walnut photography reproduces without competing colour casts. The distinction is subtle but critical: a background should support the work, not tint it.

Accent usage is minimal. The maker's work is the colour. When UI elements need emphasis, the 700 or 800 primary shade provides it without competing with walnut browns and natural grain patterns shown in portfolio imagery.

# Typography

Source Serif 4 at regular weight handles all display text — hero headlines, section titles, project names. Its slightly mechanical serif construction gives written content the feeling of workshop drawings and mid-century specification sheets, an appropriate register for a practice rooted in precision craft and material honesty. The font carries authority without antiquarian fussiness.

Outfit at weight 300 handles body copy, navigation labels, metadata, and captions. Its geometric skeleton provides clean readability at small sizes while the light weight keeps it subordinate to the Source Serif display layer. The pairing works because both fonts occupy different registers — one industrious, one technical — much like the maker's own practice bridges design and craft.

Type scale follows a modular arrangement based on 1.25, with generous leading in body text (1.65) and tighter leading in headlines (1.1). Korean language content, when present, should fall back to system defaults that harmonise with Outfit's x-height and Source Serif's cap height.

# Layout

Pages breathe. Margins are generous — minimum 24px on mobile, scaling to 80px or more on desktop. The spacing scale now extends to 224px, delivering true gallery-wall pacing where portfolio entries receive the spatial gravitas they deserve. At 160px, the spacing creates deliberate editorial pauses between major sections; at 192px and 224px, full-bleed editorial breaks between portfolio entries establish the unhurried rhythm of a curated exhibition.

The grid is a twelve-column system but most pages use only four to six columns of actual content, leaving the remainder as intentional emptiness. This is not wasted space; it is the visual equivalent of the silence between notes.

Portfolio entries favour a horizontal rhythm: large images left-aligned with minimal text to the right, or full-bleed photographs broken by narrow text interludes. Vertical scrolling should feel unhurried — the upper spacing tokens (128px through 224px) create the gallery-wall cadence where each piece is given room to exist on its own terms, separated from neighbours by the same generous emptiness a physical gallery provides. Breakpoints are semantic — workshop, studio, gallery — reflecting the maker's spaces rather than device categories.

Mobile layouts collapse to single-column with maintained spacing integrity. Touch targets are never smaller than 44px. The maker's clients are architects and interior designers who often browse on phones during site visits; the mobile experience must feel as considered as a finished piece.

# Elevation & Depth

Shadows are used sparingly and only in functional contexts — dropdowns, modals, floating action elements. A single shadow token suffices: a soft, low-spread shadow at 8% opacity that suggests depth without drama. The furniture itself, shown in photographs, provides all the visual shadow the portfolio needs.

Layering comes through background shifts rather than shadow stacking. A card might sit on a background two shades lighter than its surroundings, creating subtle differentiation. This approach mirrors how light reveals form on a physical surface — gradation, not projection.

Dark mode inverts the logic: raised surfaces are slightly lighter than the base, as if catching ambient light. The effect should feel like viewing objects under dim gallery lighting rather than glowing screens.

# Shapes

Border radius values are kept small — 2px, 4px, 6px maximum. Sharp corners reference the precise joinery in the maker's work. A 2px radius softens just enough to prevent visual harshness without rounding into approachability. These are the radii of a well-tuned chisel edge, not a consumer product.

Full-bleed images use no radius at all. Containerized content uses the small radius. Buttons and interactive elements use 4px. Nothing on the page should appear moulded or injection-formed; the slight angularity maintains the handmade register.

Decorative shapes are avoided entirely. No circles, no blobs, no organic flourishes. The geometry is rectilinear because walnut is worked in straight lines and right angles, and the interface should extend that logic to every pixel.

# Components

Navigation is minimal: a wordmark top-left, three to five links top-right, rendered in Outfit at small size with generous letter-spacing. No hamburger menu on desktop; on mobile, a simple slide-in panel. Active states use the primary-700 colour. No underlines, no backgrounds on hover — just a colour shift, quiet and quick.

Portfolio cards are image-dominant with a narrow text rail showing project name, material, and year. Hover reveals a subtle arrow or "View" label. The interaction pattern is magazine-like: editorial pacing where each project receives its own spatial allocation rather than competing in a dense grid. The expanded spacing tokens (160px, 192px, 224px) create the breathing room between entries that transforms a portfolio from a scroll into a procession.

Contact information is presented as plain text — no form widgets, no styled inputs. An email address, a phone number, a workshop address in Seoul. The simplicity signals that this is a person you call, not a brand you submit tickets to. Typography handles the hierarchy; components stay out of the way.

# Do's and Don'ts

Do let photography dominate. The furniture is the content; the interface is the gallery wall. Use full-bleed images at high resolution, optimised for web but never compressed to the point of losing grain texture. The viewer should feel they can reach into the screen and touch the walnut.

Don't use bright accent colours. No red buttons, no blue links, no green success states. The palette is warm and restrained. Interactive elements use the primary scale or neutral shifts. If something needs to be bright to be noticed, reconsider whether it needs to exist.

Do maintain consistency with Korean typographic conventions when bilingual content appears. Respect hangul line-breaking rules, provide appropriate fallbacks, and test with real Korean-language content — not placeholder text. The maker is based in Seoul; the site should feel native to that context.

Don't animate excessively. Fade-ins on scroll are acceptable at 300ms with gentle easing. Parallax, bouncing elements, loading spinners, and page transitions are unnecessary. The site should load fast and move deliberately, like the maker's own pace.