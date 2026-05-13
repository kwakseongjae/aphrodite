---
name: "devtool-landing"
version: "0.1"
description: "Minimal landing page for a developer tool with bold serif headline, monospace body, single accent color"
colors:
  primary:
    "50": "#fef3f2"
    "100": "#fde4e2"
    "200": "#f9c7c3"
    "300": "#f4a19b"
    "400": "#ec6f66"
    "500": "#e14338"
    "600": "#d12e24"
    "700": "#af231c"
    "800": "#90211c"
    "900": "#78221e"
  neutral:
    "0": "#ffffff"
    "50": "#fafaf9"
    "100": "#f5f5f4"
    "200": "#e7e5e4"
    "300": "#d6d3d1"
    "400": "#a8a29e"
    "500": "#78716c"
    "600": "#57534e"
    "700": "#44403c"
    "800": "#292524"
    "900": "#1c1917"
    "1000": "#000000"
typography:
  display:
    family: "Newsreader"
    weight: 700
  body:
    family: "JetBrains Mono"
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
rounded:
  sm: "2px"
  md: "4px"
  lg: "6px"
metadata:
  variants:
    light:
      description: "Default light mode — warm paper with soft charcoal"
      tokens:
        colors.background.primary: "#faf9f7"
        colors.text.primary: "#1c1917"
    dark:
      description: "Dark mode — deep slate with warm off-white"
      tokens:
        colors.background.primary: "#0f0e0d"
        colors.text.primary: "#e7e5e4"
    brand-a:
      description: "First brand variant — cream parchment with burnt sienna"
      tokens:
        colors.background.primary: "#f5f0e8"
        colors.text.primary: "#292524"
    brand-b:
      description: "Second brand variant — cool charcoal with rose"
      tokens:
        colors.background.primary: "#e8e4e0"
        colors.text.primary: "#1a1816"

---

# Overview

This design system serves a single-purpose developer tool landing page. It pairs Newsreader, a sharp editorial serif, against JetBrains Mono, a code-native monospace. The tension between these two voices — one literary, one technical — gives the page its identity. There is no attempt to be playful or ornamental. The page simply states what the tool does and gets out of the way.

The palette centers on a warm vermillion red, chosen for its visibility against neutral grounds without the aggressive saturation of a pure red or the ubiquity of blue-indigo. The neutrals are warm stone tones, not cool grays, which keeps the page from feeling corporate. Accent color appears sparingly: one button, a few linked words, perhaps a cursor-blink on a terminal mockup. Everything else earns its place through spacing and weight contrast.

The system is intentionally constrained. Eight spacing values. Three radii. Two typefaces. One accent hue. This is not a design system meant to scale to a hundred components — it is meant to produce one page that feels considered and finished.

# Colors

The primary palette builds from a single vermillion hue at its 500 shade, surrounded by warmer, earthier steps rather than clean saturation ramps. At shade 50 it reads as a whisper of clay; at 900 it approaches dried brick. This range handles subtle highlights and strong calls to action without reaching for a second accent.

Neutrals follow a stone scale — faintly warm at every step. This matters because pure grays read as inert on a developer-focused page. A warm neutral feels more like a material the user might encounter in a physical notebook or a well-aged terminal. The dark mode background at #0f0e0d is not pure black but a near-black with visible warmth.

Every variant pair has been verified for WCAG-AA contrast. The darkest text on the lightest background exceeds 12:1. The lightest text on the darkest background exceeds 13:1. Mid-range combinations using the 600–800 neutral steps also maintain safe contrast for secondary text at readable sizes.

# Typography

Newsreader at weight 700 provides the display voice. It is a high-contrast serif originally designed for editorial use, which gives headlines a sense of authority without decorative fuss. Letterforms are crisp at large sizes, which matters when the headline is the first — and possibly only — thing a visitor reads before deciding to stay.

JetBrains Mono at weight 400 handles all body text, feature descriptions, and code snippets. It is a monospace designed for legibility, with distinguishable characters and comfortable proportions. Using it for body text rather than just code blocks is a deliberate choice: it signals that this page is for people who spend their days in terminals and editors.

The type scale uses a 1.25 ratio. Display headlines sit at 3rem, section heads at 1.5rem, and body at 1rem with 1.7 line-height. Line lengths are capped at 64 characters. No text centering except for the hero headline — everything else is left-aligned to an 8px grid.

# Layout

The page is a single vertical column, max-width 720px, centered. This is narrower than conventional landing pages because the monospace body text requires tighter measure for comfortable reading. Horizontal breathing room is generous — the content never touches the viewport edges, with minimum 32px padding on each side.

Sections are separated by 64px of vertical space. Within sections, elements use 16px or 24px gaps. This creates a rhythm of alternating density and rest: a headline cluster, a breath, a feature list, a breath, a call to action. No multi-column layouts, no sidebars, no sticky navigation.

Grid alignment follows an 8px baseline. All spacing tokens, type sizes, and component dimensions land on this grid. The result feels deliberate without requiring complex layout systems.

# Elevation & Depth

Elevation is minimal by choice. The page does not rely on shadows or layered surfaces. The single elevation level — 2px offset, 8px blur, 8% neutral opacity — is reserved for interactive elements like the CTA button on hover or an optional terminal mockup frame.

In dark mode, elevation uses lighter borders instead of shadows — a 1px border at neutral-800 reads as a subtle raised edge against the #0f0e0d background. This avoids the "glowing shadow" effect that can make dark UIs feel artificial.

Backgrounds are flat. There is no gradient overlay, no mesh pattern, no dot grid. If visual texture is needed, it should come from the typography itself — the texture of serif letterforms at large sizes against a quiet ground.

# Shapes

Border radii are deliberately small: 2px, 4px, and 6px. This is not a rounded, friendly interface — it is a precise, engineered one. Buttons use 4px radius. Code blocks and input fields use 2px. The largest radius, 6px, might apply to a feature card if the layout calls for one.

The button shape is a simple rectangle with minimal rounding. It is 48px tall to meet touch targets, with 32px horizontal padding. There is no icon inside the button — just text in the monospace face, medium weight, centered. Hover state darkens the background by one shade and applies the single shadow.

All shapes respect the 8px grid. Even the 2px radius is possible because the 4px grid unit is the smallest increment. This keeps the visual system tight.

# Components

The headline component is a single Newsreader line or two lines, left-aligned, with no subhead styling. If a subtitle exists, it is body-size monospace in neutral-500, sitting 16px below the headline. There is no gradient text, no text-shadow, no animated reveal.

The CTA button uses the primary-600 background with neutral-0 text. It is the only element on the page that wears the accent color at full saturation. Hover moves to primary-700 and adds the elevation shadow. Active state presses to primary-800. Focus ring is 2px, primary-500, offset 2px. The button does not animate its shape.

The feature list is a vertical stack of label-description pairs. The label is monospace at body size in neutral-900. The description is monospace at body size in neutral-500, sitting 4px below. Pairs are separated by 24px. No icons, no checkmarks, no decorative bullets — the monospace alignment is the visual structure.

# Do's and Don'ts

Do let the type do the work. A bold serif at 3rem against a quiet ground is already a statement. It does not need color, animation, or embellishment to be noticed. Do keep the accent color for interactive elements only. Do use the full vertical space — a sparse page reads as confident, not empty. Do test the dark variant on actual monitors, not just simulators.

Don't add a second accent color. One is enough for a single-purpose landing page. Don't center-align body text or feature lists. Left alignment is easier to read and more credible for a technical audience. Don't use Newsreader below 1.25rem — its high contrast thins become difficult to read at small sizes. Don't add illustrations, abstract shapes, or decorative SVGs unless they convey information. The page is about the tool, not the design.