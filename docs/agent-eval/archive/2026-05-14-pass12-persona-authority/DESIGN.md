---
name: "walnut-seoul-sottsass"
version: "0.1"
description: "Portfolio for an independent walnut furniture maker in Seoul — joyful, polemical, alive"
colors:
  primary:
    "50": "#fef3e2"
    "100": "#fce5c0"
    "200": "#f7c97a"
    "300": "#f0a832"
    "400": "#e89310"
    "500": "#d97706"
    "600": "#b45f06"
    "700": "#8b4513"
    "800": "#6b3410"
    "900": "#4a2208"
  accent:
    "mint": "#2dd4bf"
    "pink": "#f472b6"
    "lapis": "#3b82f6"
    "citron": "#facc15"
    "terracotta": "#c2410c"
  neutral:
    "0": "#ffffff"
    "50": "#faf9f7"
    "100": "#f3f1ed"
    "200": "#e4e0d8"
    "300": "#cec7ba"
    "400": "#a9a090"
    "500": "#7d7568"
    "600": "#5a5347"
    "700": "#3e3831"
    "800": "#28231d"
    "900": "#1a1612"
    "1000": "#000000"
typography:
  display:
    family: "Druk Wide"
    weight: 700
  cjk-display:
    family: "Sandoll Noisuh"
    weight: 400
  body:
    family: "Outfit"
    weight: 300
spacing:
  "1": "6px"
  "2": "12px"
  "4": "24px"
  "6": "36px"
  "8": "48px"
  "12": "72px"
  "16": "96px"
  "20": "128px"
  "24": "160px"
  "28": "224px"
rounded:
  sm: "2px"
  md: "6px"
  lg: "12px"
  xl: "24px"
  full: "9999px"
metadata:
  variants:
    light:
      description: "Warm light — terrazzo wall, amber timber, full saturation"
      tokens:
        colors.background.primary: "#faf9f7"
        colors.text.primary: "#1a1612"
    dark:
      description: "Dark studio — charcoal walnut, citron and mint glow"
      tokens:
        colors.background.primary: "#1a1612"
        colors.text.primary: "#f5f3f0"
    brand-a:
      description: "Coral gallery — warm peach wall, dark walnut text"
      tokens:
        colors.background.primary: "#fef3e2"
        colors.text.primary: "#28231d"
    brand-b:
      description: "Mint workshop — cool workshop, warm timber text"
      tokens:
        colors.background.primary: "#edfcfa"
        colors.text.primary: "#28231d"
---

# Overview

A walnut furniture maker in Seoul deserves a portfolio that does not whisper. The convention for craft portfolios is restraint — cream backgrounds, elegant serifs, the hushed register of a gallery where you are afraid to breathe. I refuse this. Furniture that is hand-made from solid walnut is already an act of joyful stubbornness; the surface that presents it should match that energy.

This design uses a palette of burnt amber, mint, hot pink, and citron yellow against warm neutrals keyed to walnut timber. The display face is Druk Wide — a wide sans-serif with the confidence of a headline and the warmth of a woodcut stamp. For Korean text, Sandoll Noisuh provides play in its letterforms that matches Druk's bravado without competing. Body copy in Outfit Light stays readable while letting the display type do the shouting.

The spacing scale reaches 224 px for editorial breathing room. Cards are not always rectangles. Borders are not always thin. A decorative triangle is not decoration — it marks a structural division the way a dovetail joint marks the meeting of two planes. Every colour choice, every curve, every generous margin exists to make the furniture look more alive, not to make the designer look more tasteful.

# Colors

The primary palette is built on burnt amber (D97706) — the colour of walnut oiled and brought into warm light. This is not an "earth tone" in the coward's sense; it is a saturated, declarative amber that reads as timber and sunshine at once. Against this, mint (#2DD4BF) and hot pink (#F472B6) serve as structural accents. Mint on amber is complementary contrast — it vibrates at the edge where the eye cannot rest. Hot pink is used sparingly, like a single coloured dowel in a black walnut bench.

The neutral scale runs warm — #FAF9F7 as the lightest value, not pure white, because pure white is the background of hospitals and term sheets. The darkest neutral (#1A1612) carries a trace of umber that connects it to the material even in darkness. This warmth means walnut photography never looks cold or blue-shifted against the page.

The citron (#FACC15) appears only in small structural moments — a border, a label, a hover state — never as a background. Terracotta (#C2410C) is reserved for error states or the rare urgent call to action. Lapis (#3B82F6) appears only in links, where convention aids navigation. Every other colour is earned, not scattered.

# Typography

Druk Wide at display sizes is the voice of this portfolio. It is not a polite face. Its wide proportions and heavy weight fill the viewport with the same assurance that a dining table in solid walnut fills a room. For English headings — the maker's name, project titles, section markers — Druk at 64–96 px with tight tracking becomes architecture, not text.

Korean display text uses Sandoll Noisuh. Where many Korean typefaces default to restraint, Noisuh carries irregularity and warmth — the typographic equivalent of hand-planed grain. At display sizes it holds its own next to Druk without competing; the two faces share a register of confidence through different formal languages.

Body copy is Outfit Light (300 weight) at 16–18 px. Light weight, generous line height (1.6–1.7). This is where function is respected: you can read project descriptions, material notes, and pricing without strain. The display face does the emotional work; the body face does the practical work. Neither is ashamed of its role.

# Layout

The page is structured as a vertical scroll through the maker's world — not a grid of equal cards on a neutral field. The hero is full-bleed walnut photography with Druk Wide text overlaid asymmetrically, anchored to the lower-left. Below, selected work is presented in a staggered grid: images at varying scales (4:5, 1:1, 16:9) arranged so no two adjacent cards share the same height. This creates a visual rhythm that mirrors the experience of walking through a workshop — you see a table leg, then a full cabinet, then a close-up of grain.

Generous spacing between sections (160–224 px) creates magazine pacing. The viewer dwells on each piece before encountering the next. Within sections, narrower spacing (24–48 px) groups related information — title, material, year — into tight clusters that read as a unit.

Asymmetric balance governs every composition. When a project image occupies the left two-thirds of the viewport, its text rail sits on the right third, vertically centered. When a full-bleed image appears, a small mint-coloured triangle in the lower-right corner marks the scroll invitation. The triangle is structural: it tells you motion is possible, which is information you need.

# Elevation & Depth

There are no floating drop shadows on this site. Elevation is expressed through colour and border, not through the grey translucency that signals "tech platform." A selected project card lifts by gaining a 3 px amber border and shifting its background from neutral-100 to neutral-0. A pressed state inverts — background becomes primary-900, text becomes primary-50.

The only shadow is a hard-edged, minimal offset shadow (0 2 px 0 0) on interactive elements, used to signal depth the way a chamfered edge signals physical presence in woodwork. This shadow is always primary-900, never grey — it is the object's own dark timber casting its own shadow.

Layering is achieved through overlapping planes. A citron-coloured rectangle may extend 12 px behind a project card, visible only on one edge, creating the impression of a note card tucked into a drawer. These moments are controlled — never more than one per viewport — and always serve to identify the active or highlighted element.

# Shapes

Sharp corners (2 px radius) dominate the system — the language of joinery, not soft-tech consumer products. A 2 px radius is the digital equivalent of a broken edge on a hand-planed surface: it acknowledges material reality without rounding off the intention.

Curves are reserved for moments of deliberate warmth. Navigation pills use full rounding (9999 px). The decorative triangles that mark section transitions have no rounding at all — they are pure, structural geometry. A single large circle may appear in the background of the about section, rendered in primary-50, large enough to suggest the moon or a tabletop viewed from above.

The card system includes one non-rectangular variant: a card with its top-right corner cut at 12 degrees, mimicking the angled face of a splayed-leg table. This cut is not decoration — it signals "featured project" with the same economy that a chamfer signals "this edge is finished."

# Components

The navigation is a horizontal bar with the maker's name in Druk Wide on the left and page links in Outfit Light on the right. Between them, a thin amber line extends and retracts on scroll — a decorative element that does structural work by indicating scroll position. Menu items are spaced at 36 px; the active item is marked by a 6 px mint circle, not an underline.

Project cards show the photograph dominant (80 % of card area) with a narrow text rail below: project name in Druk, material and year in Outfit Light. No tags, no categories, no metadata soup. Hover state adds the 3 px amber border and a 4 px upward translate. The featured card uses the angled-corner variant and appears 20 % larger than adjacent cards.

The about section presents a photograph of the maker at work (full-bleed, 16:9, with a 12 px mint border on the bottom edge only) alongside a text block. Contact information is plain text: name, email, phone, workshop address in Seoul. No form. No submission button. A person you call, not a brand you submit tickets to.

The footer is minimal — three lines of text in Outfit Light, a row of Lucide icons (hammer, ruler, tree-deciduous) at 20 px, and a repeating dot pattern in primary-100 that references terrazzo without illustrating it.

# Do's and Don'ts

Do let the walnut photographs dominate every surface they touch. The portfolio exists to show the work; the design exists to make the work look like what it is — handmade, material, warm, precise, joyful.

Do use the full palette with intention. Mint on amber is not decoration; it is information about the maker's sensibility. A citron border on a single card says "this is the newest piece" more efficiently than a "NEW" badge ever could.

Do trust asymmetric balance. Symmetry is the register of institutions and corporations. This is one person in a workshop in Seoul. The layout should feel like their studio — things placed where they make sense, not where the grid demands them.

Don't add stat rows, progress bars, or quantitative metrics. "12 years of experience" is a résumé concern. The work is the metric. If the chairs are good, the chairs are enough.

Don't use grey shadows, soft rounded corners, or the muted earth-tone palette that signals "sophisticated craft brand." This maker works in walnut — one of the most characterful timbers alive. The surface should match the material's exuberance, not suppress it.

Don't default to sans-serif body text at 14 px because it is "clean." Druk Wide at 72 px is clean. Outfit Light at 18 px on generous leading is clean. "Clean" is a word people use when they are afraid of commitment. Commit.