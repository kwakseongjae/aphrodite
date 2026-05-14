---
name: "ara"
version: "0.1"
description: "Calm editorial presence for a longevity research clinic"
colors:
  primary:
    "50": "#f3f5f0"
    "100": "#e0e5d8"
    "200": "#c4ccb6"
    "300": "#a1ad8c"
    "400": "#84926a"
    "500": "#63744a"
    "600": "#4e5c3b"
    "700": "#3d4831"
    "800": "#333b2a"
    "900": "#2b3225"
  neutral:
    "0": "#ffffff"
    "50": "#fafaf7"
    "100": "#f0efe9"
    "200": "#e0ddd3"
    "300": "#c8c4b7"
    "400": "#a8a395"
    "500": "#8a8578"
    "600": "#6e6a5f"
    "700": "#565340"
    "800": "#3e3c35"
    "900": "#2a2924"
    "1000": "#0f0e0c"
typography:
  display:
    family: "'Fraunces'"
    weight: 700
  body:
    family: "'Source Serif 4'"
    weight: 400
spacing:
  "1": "4px"
  "2": "8px"
  "3": "12px"
  "4": "16px"
  "5": "20px"
  "6": "24px"
  "8": "32px"
  "12": "48px"
  "16": "64px"
  "24": "96px"
rounded:
  sm: "2px"
  md: "4px"
  lg: "6px"
metadata:
  variants:
    light:
      description: "Default light mode"
      tokens:
        colors.background.primary: "#fafaf7"
        colors.text.primary: "#2a2924"
    dark:
      description: "Dark mode"
      tokens:
        colors.background.primary: "#141310"
        colors.text.primary: "#e8e5dc"
    brand-a:
      description: "Warm stone variant"
      tokens:
        colors.background.primary: "#f7f3eb"
        colors.text.primary: "#2c2416"
    brand-b:
      description: "Cool sage variant"
      tokens:
        colors.background.primary: "#f0f2ec"
        colors.text.primary: "#1e2419"

# Overview

Ara draws from the quiet authority of scholarly journals and the grounded palette of things that endure — lichen, weathered stone, old paper. The system avoids clinical blue and medical white in favor of muted olive-sage greens and warm neutrals that suggest wisdom accumulated over time, not sterility.

The typographic pairing is deliberately unhurried. Fraunces provides a slightly eccentric, warm serif for headlines — its variable weight and optical sizing giving editorial pieces a literary quality. Source Serif 4 handles body text with the evenness and readability of a well-set book. Both are open-source and performant.

The overall feeling should be closer to a university press catalog than a medical waiting room. Generous whitespace, restrained color application, and a focus on reading comfort over visual excitement.

# Colors

The primary palette centers on a muted olive-green (#63744a) that reads as neither saturated nor dull — it simply exists, calm and present. This is not a accent color that demands attention; it is a foundational tone that reinforces credibility and quietude. Lighter tints serve as background washes; darker shades anchor headlines and interactive states.

Neutral tones lean warm. Pure whites are avoided in favor of off-whites with a faint yellow-green undertone (#fafaf7), and darks are warm blacks (#0f0e0c) rather than cold ones. This warmth prevents the system from feeling clinical without sacrificing readability.

Color is applied sparingly. A single primary-tinted surface can orient a page, but the dominant impression should be warm neutral with occasional green punctuation. Interactive elements use the 500 or 600 shade; decorative backgrounds stay at 50 or 100.

# Typography

Fraunces at 700 weight provides display headlines with personality — slightly condensed, with a gentle wedge serif that feels scholarly without being stuffy. It should be used at large sizes (32px minimum) where its character can breathe. Smaller headlines benefit from its optical sizing axis.

Source Serif 4 at 400 is the workhorse. It renders body text with comfortable x-height and moderate contrast, maintaining readability across long-form editorial content about research findings, clinical protocols, and patient narratives. It pairs with Fraunces because both share a warmth without matching too closely.

Type scale follows a 1.25 ratio. Body text defaults to 18px on desktop, 16px on mobile. Line height is generous at 1.7 for body, tighter at 1.15 for headlines. Paragraphs are capped at 65 characters wide.

# Layout

The grid is a single-column reading layout with generous margins — 120px on desktop, 32px on mobile. Content never stretches beyond 720px wide for running text. Wider sections (hero banners, data visualizations) may extend to 1120px but remain centered.

Vertical rhythm follows the spacing scale. Sections breathe with 96px between them. Subsections use 48px. Dense editorial content uses 24px between paragraphs. This regularity creates a calm, predictable reading experience that reduces cognitive load.

Asymmetric layouts are reserved for featured content — a full-bleed image with text pulled to one side, or a stat highlight offset from the main column. These moments are rare and therefore impactful.

# Elevation & Depth

Elevation is minimal. The system prefers flat surfaces with subtle borders (#e0ddd3 on light backgrounds) to distinguish cards and sections. When depth is needed, a single soft shadow (0 1px 3px rgba(15,14,12,0.06)) is sufficient.

Layering is achieved through background tone shifts rather than shadows. A primary-50 surface behind a neutral-0 card reads as layered without the artifice of drop shadows. This approach is more architectural than material — surfaces step down rather than float.

The one exception is sticky navigation and modal overlays, which use a slightly heavier shadow (0 4px 12px rgba(15,14,12,0.1)) to communicate their contextual elevation above page content.

# Shapes

Corners are barely rounded — 2px for small elements like tags and inputs, 4px for cards and buttons, 6px for large containers. The system avoids fully rounded shapes entirely. This subtle rounding prevents the design from feeling harsh without introducing playfulness.

Buttons and interactive elements maintain consistent border radius within their size tier. The slight rounding reads as refinement rather than friendliness, appropriate for a brand communicating about health and longevity.

Icon containers and avatar frames follow the same system. Circular elements are reserved for status indicators and small decorative marks, never for content containers.

# Components

Buttons are understated: medium-weight olive text (#63744a) on transparent backgrounds for secondary actions, solid olive fills (#63744a with white text) for primary actions. Hover states deepen to 600. All buttons use the 4px radius and 12px vertical / 24px horizontal padding.

Cards are flat rectangles with 1px borders in neutral-200 and 4px radius. They contain no decorative elevation — content hierarchy within (a small label, a headline, a brief excerpt) does the work. Image cards allow the photograph to bleed to the edge with a subtle inner gradient.

Navigation is a simple horizontal bar: logo left, links right, no background color until scroll (then neutral-0 with a bottom border). Active links use primary-500 text rather than underlines. Mobile collapses to a clean full-screen overlay with centered links at 24px size.

# Do's and Don'ts

Do let whitespace do the work. A long-form article about telomere research should feel as spacious and unhurried as the reading experience itself. Every element earns its place.

Do maintain the warm undertone across all surfaces and text. Even data tables and form inputs should sit on warm backgrounds with warm neutrals. The moment something pure white or cold gray appears, the clinical associations return.

Don't introduce bright accent colors for emphasis. If something needs attention, use size, weight, or position — not a red badge or orange callout. The palette's restraint is its authority.

Don't use Fraunces for body text or Source Serif for large headlines. Each has its role, and blurring those roles weakens the typographic hierarchy. If a third voice is needed, use system sans-serif sparingly for labels and metadata.

Don't animate for decoration. Transitions should be functional — a hover state, a modal entrance, a scroll reveal — and last 200ms or less with ease-out curves. Movement in a longevity brand should feel deliberate and calm, never surprising.