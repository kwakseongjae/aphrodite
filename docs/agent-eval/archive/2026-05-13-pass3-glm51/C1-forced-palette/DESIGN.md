---
name: "stillwater"
version: "0.1"
description: "A wellness retreat site grounded in forest stillness and quiet breath"
colors:
  primary:
    "50": "#D8F3DC"
    "100": "#B7E4C7"
    "200": "#95D5B2"
    "300": "#74C69D"
    "400": "#52B788"
    "500": "#40916C"
    "600": "#2D6A4F"
    "700": "#1B4332"
    "800": "#143728"
    "900": "#0D2B1E"
  neutral:
    "0": "#ffffff"
    "50": "#fafaf8"
    "100": "#f0efe8"
    "200": "#e0ddd2"
    "300": "#c7c3b5"
    "400": "#a8a294"
    "500": "#8a8474"
    "600": "#6b6558"
    "700": "#4d483d"
    "800": "#2e2b24"
    "900": "#1a1814"
    "1000": "#000000"
  accent:
    warm:
      "50": "#fef9f0"
      "500": "#d4a574"
      "900": "#6b4226"
    clay:
      "50": "#faf0eb"
      "500": "#c4907a"
      "900": "#5e3a2e"
typography:
  display:
    family: "Fraunces"
    weight: 700
  body:
    family: "Work Sans"
    weight: 400
  mono:
    family: "JetBrains Mono"
    weight: 400
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
rounded:
  xs: "2px"
  sm: "4px"
  md: "8px"
  lg: "12px"
  xl: "16px"
  pill: "9999px"
metadata:
  variants:
    light:
      description: "Mist — a pale warm ground for morning calm"
      tokens:
        colors.background.primary: "#fafaf8"
        colors.background.secondary: "#f0efe8"
        colors.text.primary: "#1a1814"
        colors.text.secondary: "#4d483d"
        colors.text.tertiary: "#8a8474"
        colors.border.default: "#e0ddd2"
        colors.surface.elevated: "#ffffff"
    dark:
      description: "Canopy — deep forest darkness with filtered light"
      tokens:
        colors.background.primary: "#0f1a14"
        colors.background.secondary: "#162219"
        colors.text.primary: "#e8f0eb"
        colors.text.secondary: "#a3b5ab"
        colors.text.tertiary: "#6b8075"
        colors.border.default: "#1e3328"
        colors.surface.elevated: "#1a2b21"
    brand-a:
      description: "Cedar — warm amber undertone for rooted hospitality"
      tokens:
        colors.background.primary: "#fef9f0"
        colors.background.secondary: "#fdf0dd"
        colors.text.primary: "#1e1509"
        colors.text.secondary: "#5a4528"
        colors.text.tertiary: "#917a56"
        colors.border.default: "#e8d9bf"
        colors.surface.elevated: "#ffffff"
    brand-b:
      description: "Stone — mineral grey with green memory"
      tokens:
        colors.background.primary: "#f4f5f2"
        colors.background.secondary: "#e8eae4"
        colors.text.primary: "#16180f"
        colors.text.secondary: "#43472f"
        colors.text.tertiary: "#7a7f68"
        colors.border.default: "#d4d7cc"
        colors.surface.elevated: "#ffffff"

---

# Overview

Stillwater is a wellness retreat identity built on the principle of deceleration. Every design decision serves a single goal: the visitor should feel their shoulders drop within the first three seconds of arriving. The forest-green palette is non-negotiable because it anchors the entire experience in living, breathing nature — not the clinical blue of healthcare, not the saccharine peach of lifestyle brands, but the specific deep green of canopy shadow and moss after rain.

The typographic pairing of Fraunces and Work Sans is deliberately asymmetric. Fraunces brings an organic, slightly awkward warmth to display text — its variable axis allows gentle weirdness that feels handmade without being precious. Work Sans is its straight-backed counterpoint: clean, generous, unremarkable in the best way. Together they model the relationship between wildness and structure that defines good retreat architecture.

Restraint is the governing principle. We use three shades of green, two neutrals, and one optional warm accent. If a component needs more than that, the component is doing too much. Every surface breathes. Every element earns its space.

# Colors

The primary palette is anchored by three mandatory greens: #1B4332 at the deep end, #52B788 at mid-tone, and #D8F3DC as the lightest breath. These map to emotional registers — stillness, vitality, and openness — rather than functional roles. The deep green is authority without severity. The mid green is aliveness without urgency. The pale green is air.

Neutral warmth prevents the greens from feeling clinical. All neutrals carry a slight yellow-brown undertone that references linen, raw wood, and unbleached paper. This is critical: pure grey would make the greens feel medicinal. The warmth makes them feel inhabited.

The warm accent (#d4a574) and clay accent (#c4907a) exist for moments that need to interrupt the green without breaking it — a booking CTA, a testimonial highlight, a limited-edition program badge. They reference terracotta, baked earth, the color of a campfire ember. Use them rarely enough that they still mean something.

# Typography

Fraunces operates at display scale only: hero statements, section openings, pull quotes. Its optical-size axis should be engaged so that larger sizes carry more personality while smaller display sizes tighten into something more structured. Weight should stay between 600 and 800 — heavy enough to be declarative, light enough to avoid shouting.

Work Sans handles everything else. Body text sits at 16px minimum, 18px preferred, with 1.6 line-height. This is non-negotiable for a wellness audience that may be reading on mobile devices in low-light conditions or while intentionally slowing down. Generous leading is not a stylistic choice here; it is an accessibility commitment.

Type scale follows a 1.25 ratio: 16, 20, 25, 31, 39, 49, 61. This keeps the jump between body and display gradual rather than theatrical. Headings should feel like a natural increase in presence, not a sudden shift in tone. Maximum display size is 61px — large enough to be immersive, restrained enough to avoid parody.

# Layout

The page is a vertical scroll through rooms. Each section is a room with its own atmosphere, separated by generous whitespace that functions as a threshold. The content width never exceeds 720px for running text. Wider layouts use a 12-column grid with 24px gutters, but the content itself stays centered and bounded.

Vertical rhythm follows the 8px baseline grid. Spacing tokens map to emotional distance: 4px is intimate (inline elements), 16px is conversational (within a card), 32px is formal (between sections), 64px is ceremonial (between major acts of the page). The 96px token exists for moments of genuine silence — the space after a closing statement, before a footer.

Horizontal breathing room is structural, not decorative. On desktop, content never touches the viewport edge closer than 48px. On mobile, that minimum is 20px. These margins are the architectural equivalent of a path through a garden — they frame the walk, they keep the experience contained without feeling constrained.

# Elevation & Depth

Elevation is minimal because this is a retreat, not a dashboard. Two levels suffice. Level 0 is the page surface itself — flat, matte, present. Level 1 uses a 1px border in a muted tone and a single soft shadow (0 2px 8px rgba(27,67,50,0.08)) to lift cards and surfaces just barely off the ground. Anything more dramatic than that risks feeling corporate.

Depth comes from color contrast, not shadow stacking. A deep-green call-to-action button on a pale ground has more visual weight than any shadow could provide. The dark variant reverses this relationship: light text on deep green creates its own sense of dimensionality through luminance difference alone.

Glass effects, heavy shadows, and layered translucency are explicitly excluded. They belong to a different emotional vocabulary — the vocabulary of busy interfaces and complex information hierarchies. Stillwater's hierarchy is flat by design. The most important thing on any screen should be the thing the user most needs to calm down about.

# Shapes

Border radii are kept small and consistent. The sm (4px) token is for inputs, tags, and inline elements. The md (8px) token is for cards, modals, and image containers. The lg (12px) token is for larger feature cards that need to feel softer. Nothing exceeds 16px. Pill shapes are reserved for badges and status indicators only.

Sharp corners (0 radius) are available for full-width images and horizontal dividers where the element should feel architectural rather than organic. The tension between sharp structural elements and slightly rounded content containers creates a subtle rhythm — like the difference between a building's frame and the furniture inside it.

Circular elements are restricted to avatar images and small icon buttons. Anything larger than 48px should not be a circle. Circles at scale feel playful in a way that conflicts with the retreat's quiet authority. Small circles feel intentional and grounded.

# Components

Buttons exist in two weights: primary (filled with deep green #1B4332, light text) and secondary (outlined or ghost, green text on transparent ground). Primary buttons are for commitment actions — booking, subscribing, confirming. Secondary buttons are for exploration — reading more, browsing programs, learning about facilitators. A single warm-accent button variant exists for the rare high-priority conversion moment.

Cards are content containers with 8px radius, 1px border, and 24px internal padding. They never include visible shadows by default; elevation is achieved through background-color contrast with the page surface. Card content is vertically centered when single-element, top-aligned when multi-element. Cards never contain more than three distinct content types (e.g., image + heading + description).

The navigation is a flat horizontal bar on desktop, a slide-out panel on mobile. It contains no more than five items. The active state uses text color change, not background highlight. The retreat logo — a wordmark in Fraunces — sits at the left with generous left margin. A single CTA button sits at the right. Nothing else. Navigation should disappear from consciousness within ten seconds of arrival.

# Do's and Don'ts

Do let the green palette carry the identity. The three mandatory greens are the brand. Typography, layout, and imagery should serve them, not compete with them. Do use whitespace as an active design element — empty space is not wasted space, it is the spatial equivalent of a pause between sentences. Do test all text-on-background combinations against WCAG AA at minimum. Do use the warm accent sparingly enough that it retains semantic meaning.

Don't introduce new colors without mapping them to the existing palette through lightness or transparency adjustments. Don't use Fraunces below 20px — its personality becomes noise at small sizes. Don't animate anything faster than 300ms — speed is the enemy of the retreat state. Don't use stock photography of people meditating unless the image could also function as a landscape photograph. The environment is the subject; humans are witnesses, not performers. Don't center-align body text. Center alignment is for poetry and headings, not paragraphs. Don't use the mid green (#52B788) as a background for small text — it fails contrast at body sizes.