---
name: editorial
description: "Calm, magazine-grade typography with restrained color. Best for content-led sites: longevity clinics, essay publications, lookbooks."
triggers:
  - editorial
  - magazine
  - essay
  - long-form
tags:
  - typography-led
  - high-contrast
  - print-inspired
---

# Editorial — Aphrodite skill pack

## Prompt fragment

When this skill is in scope, bias the design toward:

- **Typography first.** Display sizes step from 16 px body at a 1.25 ratio. Reserve display weights (≥ 600) for headlines and pull-quotes; everything else lives at 400–500.
- **One accent color, used scarcely.** Pick a single primary hue at L≈45 % so it survives WCAG-AA on both light and dark backgrounds. Avoid gradients in v0.1.
- **Generous vertical rhythm.** Section padding starts at 96 px desktop / 64 px mobile. Body line-height 1.6 minimum.
- **Restraint over delight.** Animation is reserved for state changes (button press, focus); no scroll-jacking, no parallax, no decorative motion.
- **Print-inspired layout.** Single column for prose; max line length ≤ 70 characters. Pull-quotes can break the grid, but only once per surface.

## Variant guidance

- **Light** is the canonical surface; `colors.background.primary` should be near `#ffffff` (warm whites OK; never pure cold #fff in dark UIs).
- **Dark** must clear 4.5:1 between `colors.text.primary` and the warmest tone of background. Avoid pure black (#000) — use `#0b0b10`–`#141417` range.
- **Brand variants** stay editorial; only swap the hue, not the layout. A brand variant that introduces a second accent is out of scope.

## Do's

- Use `metadata.variants.light.tokens` to override the body text color when the brand variant has a warm tint (e.g. `#241407` on `#fff8f1`).
- Pair a single bold display family (Inter Tight, Tiempos Headline, Söhne Breit) with a balanced body family.

## Don'ts

- Do not introduce more than two palettes (primary + secondary).
- Do not place the accent on body text — only on links, CTAs, and figure captions.
- Do not specify shadows above elevation 1 in this skill; depth is a tooling skill, not editorial.
