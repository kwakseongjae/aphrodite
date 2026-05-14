---
name: "heo-studio"
version: "0.1"
description: "Portfolio site for an independent furniture maker working in solid walnut, based in Seoul — updated with Source Serif 4 display face"
colors:
  primary:
    "50": "#f4f6ef"
    "100": "#e4e8d8"
    "200": "#c9d1b1"
    "300": "#a3b07a"
    "400": "#7d8f52"
    "500": "#5a6b35"
    "600": "#465429"
    "700": "#36401f"
    "800": "#2a3118"
    "900": "#1e2412"
  neutral:
    "0": "#ffffff"
    "50": "#f9f9f8"
    "100": "#f2f1ef"
    "200": "#e5e4e1"
    "300": "#d1d0cb"
    "400": "#b0aea7"
    "500": "#8e8c84"
    "600": "#6b6a63"
    "700": "#504f49"
    "800": "#373632"
    "900": "#1f1e1b"
    "1000": "#000000"
typography:
  display:
    family: "Source Serif 4"
    weight: 400
  body:
    family: "Instrument Sans"
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
  "56": "224px"
rounded:
  sm: "2px"
  md: "4px"
metadata:
  variants:
    light:
      description: "Default light mode — cool gallery white"
      tokens:
        colors.background.primary: "#f2f1ef"
        colors.text.primary: "#1f1e1b"
    dark:
      description: "Dark mode — deep warm carbon"
      tokens:
        colors.background.primary: "#141412"
        colors.text.primary: "#e5e4e1"
    brand-a:
      description: "Gallery wall — warm off-white for portfolio viewing"
      tokens:
        colors.background.primary: "#f6f5f2"
        colors.text.primary: "#242320"
    brand-b:
      description: "Workshop — warm mid-ground evoking unfinished wood"
      tokens:
        colors.background.primary: "#ddd8ce"
        colors.text.primary: "#1a1917"

---

# Overview

This design serves as a quiet digital showroom for a Seoul-based furniture maker who works exclusively in solid walnut. The register is editorial and restrained — closer to a gallery monograph than a commercial website. Photography leads every section; text intervenes only to name the piece, note the wood, and indicate the year.

The primary hue is drawn from mossy-golden lichen rather than walnut itself. Walnut brown would compete with the material photography and flatten every image into the same tonal register. This sage-olive family sits alongside walnut as a companion, not a mirror — it reads as natural without duplicating the wood tones already present in every photograph.

Source Serif 4 at regular weight is the display choice — a contemporary serif with optical sizing that honours the editorial palette while reading as a practicing maker's space rather than a period setting. It deliberately avoids both the trendy geometric serifs and the historical revival faces that would costume the site as something other than a working studio. Instrument Sans at light weight handles body text with minimal visual presence. The weight contrast between the two typefaces creates hierarchy without shouting.

# Colors

The primary palette anchors to `#5a6b35`, a muted sage-olive that references the living material around a woodshop — lichen on stacked timber, oxidation on hand tools, the green-gold of raw walnut leaves. It avoids the obvious warm-brown direction that would collapse into the furniture photography. Three darker shades (`700`–`900`) provide heading and UI text options; lighter shades serve as washes and hover states.

Neutral tones run slightly warm across the board — `#f2f1ef` instead of pure white, `#1f1e1b` instead of true black. This warmth is calibrated to complement walnut without competing with it. The `brand-a` variant pushes to `#f6f5f2`, a gallery-wall off-white that lets photography breathe. The `brand-b` workshop variant uses `#ddd8ce`, a mid-tone that evokes unfinished timber and feels appropriate for process documentation or behind-the-scenes content.

All four variants maintain WCAG-AA contrast. The light pairing (`#1f1e1b` on `#f2f1ef`) exceeds 13:1. The dark pairing (`#e5e4e1` on `#141412`) exceeds 12:1. Brand-a and brand-b similarly clear the 4.5:1 threshold comfortably.

# Typography

Source Serif 4 at 400 weight carries all display text — project titles, section headings, the maker's name. Its optical sizing ensures the letterforms remain crisp and well-proportioned whether set at caption sizes or commanding display scales. The contemporary serif construction gives it a working-studio authority — the kind of typeface you would find on a maker's spec sheets and portfolio prints, not on a museum wall label. It pairs naturally with the sage-olive palette and walnut photography without overdressing the space.

Instrument Sans at 300 weight handles body copy, captions, and navigation labels. It is deliberately unremarkable — a workhorse that stays out of the way. The light weight prevents it from competing with Source Serif 4's presence at display sizes.

Type scale follows a modular approach: display headings at 72–96 px, sub-headings at 32–40 px, body at 18–20 px, captions at 14 px. Line heights are generous (1.4–1.6 for body, 1.1–1.2 for display) to reinforce the editorial, magazine-like pacing.

# Layout

The page unfolds vertically in long, breathing sections. A full-viewport hero opens with the maker's name in large serif type above a single commanding photograph of a finished piece. Below, selected work appears in a staggered grid — alternating between wide landscape images and narrower portrait-oriented ones, with generous gutters between them. This asymmetry prevents the grid from feeling like a catalog.

Project cards are image-dominant. Each shows a single photograph at nearly full bleed, with a narrow text rail below or alongside carrying the piece name, material notation, and year. No overlay text on images — the furniture speaks for itself.

The maximum content width sits at 1400 px, but many sections break wider for full-bleed photography. Vertical spacing between major sections reaches 192–224 px, creating the kind of pause between chapters that editorial layouts depend on. This is not a scrolling feed; it is a sequence of moments.

# Elevation & Depth

This design uses minimal elevation. The visual language comes from the physical world of flat surfaces — bench tops, planed boards, stacked sheets — not from layered software interfaces. Shadows appear only on hover states for interactive elements, and even then they are tight and subtle, offset by 2 px with 8 px of blur.

Depth instead comes from tonal shifts between sections. Alternating between the primary background and slightly different values creates visual separation without resorting to borders or drop shadows. The workshop variant background can serve as a section break, signalling a shift from finished work to process documentation.

Cards and interactive surfaces lift slightly on hover — a 1 px upward translate combined with a gentle shadow — to indicate clickability without breaking the restrained register.

# Shapes

Corner radii stay sharp: 2 px for small elements, 4 px for cards and containers. This is a deliberate reference to the precision of joinery. Rounded corners belong to software; this maker's world is one of squared edges meeting at exact angles. The sharpness is itself a small signal of craft.

Buttons and interactive elements use the same tight radii. A 4 px radius on a button is just enough to prevent harsh pixel-level edges without reading as "friendly" in the Silicon Valley sense. The visual vocabulary is one of restraint and exactitude.

Full-bleed images use no rounding at all. They meet the viewport edge directly, reinforcing the sense that the furniture occupies real space rather than floating in a decorative frame.

# Components

The navigation bar is minimal: the maker's name on the left, three to four text links on the right, all set in Instrument Sans light at 14 px with generous letter-spacing. No hamburger menu, no icon navigation. The site is small enough that everything is reachable.

The project card consists of a photograph (aspect ratio preserved, no cropping), a title in Source Serif 4, and a single line of metadata in Instrument Sans. No tags, no categories, no filtering. The grid order is the curatorial statement.

The contact section is plain text: an email address, a phone number with country code, and a workshop address in Seoul. The `mail` Lucide icon sits beside the email, `phone` beside the number, `map-pin` beside the address. No contact form. A person you call, not a brand you submit tickets to.

The about section holds one portrait photograph and two to three paragraphs of text. No stats row, no client logo carousel, no timeline. The work is the metric.

# Do's and Don'ts

Do let photography dominate every section. The furniture is the content; everything else is caption. Use full-bleed images wherever the layout supports them. Let negative space accumulate between sections — it is not wasted space but visual silence that lets each piece resonate.

Do maintain sharp visual edges. The 2–4 px radius ceiling reflects the precision of the maker's joinery. Avoid the temptation to soften corners for a more "approachable" feel — the audience for hand-cut walnut furniture does not need approachability signals from a social media design system.

Don't add SaaS-style credibility markers. No "12 years of craft" counter, no client logo strip, no testimonial carousel. The photographs of the work are the sole proof of competence. Anything else diminishes the work by implying it needs supplementary validation.

Don't use warm browns or wood tones as interface colors. They will compete with every photograph on the site. The sage-olive primary exists precisely because it accompanies walnut without mimicking it. Let the wood be wood; let the interface be something else entirely.