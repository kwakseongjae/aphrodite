---
name: "folio-stealth"
version: "0.1"
description: "Senior product designer portfolio with monochrome grid and oversized hover states"
colors:
  primary:
    "50": "#e6e9ef"
    "100": "#c4cad6"
    "200": "#9aa3b5"
    "300": "#707c94"
    "400": "#505f7b"
    "500": "#354767"
    "600": "#2d3d59"
    "700": "#24324a"
    "800": "#1b2538"
    "900": "#111828"
  neutral:
    "0": "#ffffff"
    "50": "#f7f8f9"
    "100": "#edf0f4"
    "200": "#d8dde5"
    "300": "#b4bcc9"
    "400": "#8a94a6"
    "500": "#6b7585"
    "600": "#555d6b"
    "700": "#444a55"
    "800": "#2d323a"
    "900": "#1a1d23"
    "1000": "#000000"
typography:
  display:
    family: "'Syne', 'Inter', sans-serif"
    weight: 800
  body:
    family: "'Inter', 'Syne', sans-serif"
    weight: 400
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
rounded:
  sm: "2px"
  md: "4px"
  lg: "8px"
  xl: "12px"
metadata:
  variants:
    light:
      description: "Clean light mode with soft slate tones"
      tokens:
        colors.background.primary: "#f7f8f9"
        colors.text.primary: "#1a1d23"
    dark:
      description: "Deep charcoal dark mode"
      tokens:
        colors.background.primary: "#111318"
        colors.text.primary: "#edf0f4"
    brand-a:
      description: "Warm paper variant with depth"
      tokens:
        colors.background.primary: "#f2ede6"
        colors.text.primary: "#201a10"
    brand-b:
      description: "Cool misted steel variant"
      tokens:
        colors.background.primary: "#e8ecf0"
        colors.text.primary: "#0f1520"
---

# Overview

Folio Stealth is a restrained, high-impact portfolio built around a six-card work grid. The design prioritizes spatial silence — generous gutters, a near-absence of chroma — so that each case study thumbnail becomes an event when hovered. Every surface is deliberately calm; interaction is where the energy lives.

The primary hue family is blue-slate, chosen for its neutrality and professionalism without falling into cliché navy territory. It reads as architectural rather than corporate, which suits a senior practitioner positioning themselves as a systems-level thinker. The palette whispers; it never raises its voice.

Syne at weight 800 handles display duties — its geometric warmth and slightly unconventional letterforms give headings personality without decoration. Inter serves body text, chosen for its exceptional legibility at small sizes and its tabular figures for metadata. The contrast between the two typefaces creates a rhythm: bold assertion followed by quiet competence.

The system is designed around four variants. Light and dark serve as baseline environments. Brand A introduces a warm parchment quality for editorial softness. Brand B pushes into a cool, almost atmospheric grey that feels industrial and precise. All four maintain WCAG-AA contrast on critical text-to-background pairs.

# Colors

The primary scale runs from a pale frost at shade 50 to a deep midnight at shade 900. The 500 value sits at a mid-slate that works for links, focus rings, and subtle emphasis without competing with the work imagery. Accent usage is limited to interactive states — a barely perceptible shift in tone on hover, a line appearing beneath a link.

Neutrals carry the visual weight. The portfolio uses a ten-step neutral scale that enables fine control over surface layering. Background surfaces, card fills, borders, and text all pull from this scale, ensuring tonal coherence across every element. The monochrome discipline means that when a project thumbnail appears, its own palette — whatever that may be — immediately becomes the richest color on screen.

Variant light pairs slate-white (#f7f8f9) with near-black (#1a1d23), contrast ratio approximately 14.5:1. Dark inverts this with deep charcoal (#111318) against frost (#edf0f4), ratio approximately 14:1. Brand A uses warm parchment (#f2ede6) against deep umber (#201a10), ratio approximately 13:1. Brand B uses cool steel (#e8ecf0) against midnight (#0f1520), ratio approximately 15:1. All exceed WCAG-AA comfortably.

Secondary surfaces — card backgrounds, hover overlays, navigation bars — use stepped neutral values two or three stops apart from their parent background. This creates depth without shadow, hierarchy without decoration.

# Typography

Syne at 800 weight is used exclusively for headings and the designer's name. It is set large — minimum 48px for section headings, 72-96px for the hero statement. The weight and scale mean a single word can anchor an entire viewport. Letter-spacing is tightened slightly (-0.02em) to compensate for Syne's generous spacing and to increase visual density.

Inter at 400 weight handles everything else: body copy, navigation labels, project metadata, captions, and footnotes. It is set at 16px base with 1.6 line-height for readable paragraphs. Navigation items sit at 14px in uppercase with 0.08em letter-spacing, creating a secondary typographic voice that is quiet but clearly structured.

The typographic hierarchy operates on three levels. Level one is Syne display — attentional, directional, commanding. Level two is Inter body — informational, comfortable, invisible. Level three is Inter small — navigational, meta, architectural. No level competes with another; they occupy clearly separated zones of size, weight, and family.

Line length is capped at 640px for body text. Case study descriptions may extend wider within their grid cells, but running prose respects a comfortable measure. Headings are unconstrained and may break across lines for dramatic effect, particularly in the hero section.

# Layout

The portfolio uses a twelve-column grid with 24px gutters on desktop, collapsing to six columns on tablet and a single column on mobile. The work grid occupies the full viewport width with generous padding — 80px on each side at desktop scale — so that six cards have room to breathe.

Each case study card in the grid is a 4-column span in the twelve-column system, yielding a three-across layout. Cards are taller than they are wide (approximately 4:5 aspect ratio), giving thumbnails a portrait orientation that suits screen-based work. Vertical spacing between rows matches the gutter width at 24px, maintaining grid rhythm.

The page follows a clear vertical narrative: hero statement, work grid, about section, contact footer. Each section is separated by 96px of vertical space — enough to create distinct moments without losing scroll momentum. The hero is viewport-height, centering the designer's name and a single positioning line.

On hover, each card expands its internal thumbnail to fill the frame using a scale transform, while a dark overlay fades in from the bottom carrying the project title and a one-line descriptor. The transition duration is 500ms with an ease-out curve, long enough to feel cinematic but short enough to feel responsive. Cards that are not hovered dim slightly, directing focus.

# Elevation & Depth

This system avoids drop shadows almost entirely. Depth is communicated through surface color, border presence, and scale. A card at rest sits flat against its background, distinguished only by a 1px border in neutral-200. On hover, the border dissolves as the image scales to fill, and the overlay appears — the card doesn't lift, it transforms.

Layering happens through opacity and z-index rather than shadow. The hover overlay sits at z-10 above the thumbnail at z-1. Navigation is pinned at z-50. Modals, if any, occupy z-100. This flat stacking order keeps the visual language consistent with the monochrome, minimal ethos.

The one exception is a subtle inner shadow on the hero section's background — a vignette effect that draws the eye toward center without adding a visible element. It is applied at 3% opacity and is nearly imperceptible, contributing to atmosphere rather than information.

Variant dark introduces a second depth signal: surface cards use a background two stops lighter than the page background (neutral-800 on neutral-900), creating a gentle lift without shadow. In brand-a, the inverse applies — cards are slightly darker than the parchment ground, as though pressed rather than raised.

# Shapes

Border radius is minimal throughout. Small elements — buttons, tags, input fields — use sm (2px), which rounds without softening. Medium elements — cards, image containers — use md (4px), just enough to prevent a hard pixel edge without declaring roundedness as a style choice. This restraint keeps the portfolio feeling precise and architectural.

Cards are the primary shaped element. They are simple rectangles with uniform md radius on all corners. The consistency means the eye reads the grid as a system rather than as individual objects. Variation comes from the content within, not the container itself.

The only larger radius appears on hover overlay panels that slide up within cards — these use lg (8px) on top corners only, softening the transition between overlay and image. This single rounded edge is a functional detail, not a decorative one.

Buttons follow a strict shape language: fully squared (sm radius), with hover state communicated through background fill change rather than shape change. Focus states add a 2px outline offset with primary-500 color, ensuring keyboard accessibility without altering the visual rhythm.

# Components

The work card is the signature component. At rest, it shows a muted thumbnail with a faint border. On hover, the image scales to 1.08, filling the card edge-to-edge, while a gradient overlay rises from the bottom carrying the project name in Syne 800 and a one-line summary in Inter 400 at 14px. The entire card is a single link target. Non-hovered siblings reduce to 85% opacity.

The hero section centers the designer's name in Syne at 80-96px, with a single subtitle line beneath in Inter at 18px. A scroll indicator — a thin line that tracks scroll progress — runs along the left edge of the viewport. This line provides orientation without clutter.

Navigation is a horizontal bar fixed to the top, containing three to five text links in Inter 14px uppercase. The current section is indicated by a 2px underline in primary-500. On scroll, the navigation background transitions from transparent to a frosted neutral-0 at 90% opacity, maintaining readability over content.

The about section uses a two-column split: one column for a portrait or abstract image, one for biographical text. The text column includes a short paragraph in Inter body, followed by two or three detail lines (location, availability, email) in the small meta style. A contact footer mirrors this brevity — a single line with email and social links, set in Inter at 14px.

# Do's and Don'ts

Do let the work dominate. The portfolio's job is to frame case studies, not to compete with them. Every design decision should make the project thumbnails more visible, more legible, more compelling. If an element doesn't serve the work, remove it.

Don't introduce color casually. The monochrome system exists so that project imagery provides all the chromatic variety. A colored button, a tinted background section, or an accent-color headline would break the visual contract. Reserve color for states: hover, focus, active.

Do maintain typographic discipline. Syne appears only at display scale (48px and above). Inter appears everywhere else. Mixing them at similar sizes, or introducing a third typeface, fragments the hierarchy. The two-family system is the portfolio's backbone.

Don't animate gratuitously. Hover effects on cards are the one sanctioned animation. Entrance animations on scroll, parallax effects, loader spinners, or cursor-following elements would undermine the restraint that makes the system credible. Motion should feel inevitable, not clever.