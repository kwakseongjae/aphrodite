---
name: "ember-fit-hero"
version: "0.1"
description: "Thumb-zone-optimized mobile fitness hero with warm copper-terracotta palette and bottom-arc CTA placement"
colors:
  primary:
    "50":  "#fdf3ed"
    "100": "#f9dfcf"
    "200": "#f0b99a"
    "300": "#e59060"
    "400": "#d96d35"
    "500": "#c2510f"
    "600": "#a33f0c"
    "700": "#81300e"
    "800": "#662812"
    "900": "#4d200f"
  neutral:
    "0":    "#ffffff"
    "50":   "#faf9f7"
    "100":  "#f3f1ed"
    "200":  "#e5e1da"
    "300":  "#d0c9be"
    "400":  "#b3a898"
    "500":  "#958672"
    "600":  "#7a6b58"
    "700":  "#5e5142"
    "800":  "#3d352c"
    "900":  "#1e1a16"
    "1000": "#000000"
typography:
  display:
    family: "'Anybody', 'DM Sans', sans-serif"
    weight: 800
  body:
    family: "'Work Sans', 'DM Sans', sans-serif"
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
  sm: "6px"
  md: "12px"
  lg: "20px"
  xl: "28px"
  pill: "9999px"
metadata:
  variants:
    light:
      description: "Warm light with cream backdrop"
      tokens:
        colors.background.primary: "#faf9f7"
        colors.text.primary: "#1e1a16"
    dark:
      description: "Deep espresso dark mode"
      tokens:
        colors.background.primary: "#13110e"
        colors.text.primary: "#f3f1ed"
    brand-a:
      description: "Terracotta warm variant"
      tokens:
        colors.background.primary: "#fdf3ed"
        colors.text.primary: "#3d1a0a"
    brand-b:
      description: "Sage-warm earth variant"
      tokens:
        colors.background.primary: "#f4f5f0"
        colors.text.primary: "#1a2114"

---

# Overview

This design system serves a single-scroll mobile fitness tracker hero screen, engineered around the natural arc of a right-handed thumb swipe. Every interactive element lives in the bottom 40% of the viewport. The palette draws from sun-baked terracotta and late-afternoon copper — warm without tipping into high-visibility orange, restrained enough to feel premium. The motivational register is earned through confident contrast and ample negative space rather than exclamation marks.

The typographic pairing is deliberately unconventional for fitness. Anybody, a variable-weight grotesque with eccentric cuts, handles display text with raw energy. Work Sans grounds the experience with clean, humanist readability at body sizes. Together they resist the glossy-tech aesthetic common to fitness apps and instead suggest craft and warmth.

Touch targets are a minimum of 48 by 48 pixels, with the primary CTA button spanning the full bottom-arc zone at 64 pixels tall. All secondary actions sit above the fold but below the visual midpoint, keeping the hero area breathable at the top and functional at the base.

Four variants cover the spectrum from bright daylight outdoor use to deep nighttime reading, with two brand-aligned alternatives for partner integrations. Every variant has been verified for WCAG-AA contrast between primary text and background.

# Colors

The primary family anchors around a true terracotta — hue 22, warm but restrained. The 500 shade at `#c2510f` reads as ember, not flame, distinguishing it from the typical energetic orange or electric coral found in fitness products. Lighter shades bring warmth to surfaces; darker shades approach bitter chocolate and provide gravitas for headlines.

Neutrals carry a subtle warm bias across the entire scale. Even the lightest background `#faf9f7` has a faint cream undertone that prevents the sterile feeling of pure white. The darkest neutral `#1e1a16` is an espresso tone rather than cold black, maintaining warmth even in dark mode. This warmth permeates every surface, keeping the experience inviting without being cloying.

Accent usage is limited. The primary 400 shade `#d96d35` appears on the CTA button and nowhere else on the hero. Restraint in saturation prevents visual fatigue during repeat visits — a critical concern for daily-use fitness tracking. Supporting data visualizations use primary 300 and 200, which recede appropriately against both light and dark backgrounds.

# Typography

Anybody at weight 800 for display text introduces angular personality at large sizes. The letterforms have slightly condensed proportions that fill their containers well, ideal for short motivational headlines like "Your streak is alive." Line height at display sizes stays tight at 1.05, creating visual compression that amplifies intensity.

Work Sans at weight 400 handles everything else. It is explicitly not a geometric sans — its stroke modulation and open apertures give it a humanist quality that pairs naturally with the terracotta palette. Body text runs at 16px with 1.55 line height for comfortable extended reading in workout descriptions and stat labels.

Scale is limited to three sizes on the hero: 36px display headline, 20px supporting headline, and 16px body. This restraint avoids the visual noise of too many text treatments competing in a single viewport. Weight shifts, not size shifts, carry hierarchy within each tier.

# Layout

The viewport is divided into three horizontal zones. The upper 35% is the inspirational zone — a full-bleed image or gradient wash with minimal text overlay, designed for emotional impact. The middle 25% holds supporting content: yesterday's summary, a progress ring, or a streak counter. The bottom 40% is the interaction zone, shaped to follow the natural thumb arc.

Within the interaction zone, elements are arranged along a gentle upward curve. The primary CTA sits dead center at the lowest point — the position requiring the least grip adjustment. Secondary buttons flank it slightly higher and outward, following the thumb's natural pivot path. This creates a fan arrangement that mirrors actual hand ergonomics rather than a rigid grid.

Margins are generous at 24px left and right, preventing accidental edge touches and giving the warm background room to breathe. Gutters between interactive elements measure 16px minimum, ensuring no overlapping touch zones even on smaller devices.

# Elevation & Depth

Elevation is minimal and purposeful. The CTA button rises to the highest level with a subtle shadow at 0 4px 16px rgba(194, 81, 15, 0.3), giving it warm-toned lift. Cards in the middle zone rest at a lower level with a neutral shadow that disappears against the background. This two-tier system makes the action button visually inescapable.

In dark mode, shadows shift to use a warm black with 10% primary tint. Pure black shadows on dark warm backgrounds create an uncanny cold void. The tinted shadow maintains chromatic consistency and feels like natural dimming rather than digital overlay.

No element uses more than two elevation levels. The background is always flat, cards are slightly lifted, and the CTA floats above all. This simplicity is deliberate — depth on mobile adds cognitive load, and the hero should feel like a single cohesive surface with one obvious point of interaction.

# Shapes

The CTA button uses full pill rounding at 9999px, creating an approachable, friendly shape that fills the bottom arc zone naturally. Its width spans the full content area minus margins, and its 64px height exceeds the 48px minimum touch target, giving generous surface area for imperfect thumb landings.

Cards in the middle zone use 20px border radius — rounded enough to feel modern without approaching bubble territory. Inner elements like stat pills and tags use 12px rounding. This hierarchy of rounding signals importance: the most rounded element is the most important interactive surface.

The overall container has no visible border or rounding — it runs edge to edge on mobile. Visual containment comes from background color and spacing, not from hard edges. This maximizes the usable area for thumb interaction and eliminates dead zones at rounded corners.

# Components

The hero comprises five component types: a full-bleed atmospheric header, a headline block with display typography, a stat card row, a motivational message strip, and the primary CTA button. Each is designed as a self-contained unit that composes vertically in a single column.

The stat card component supports one to three items in a horizontal row, each with a value in primary 500 and a label in neutral 500. Cards use the md border radius and sit on a slightly elevated surface. Touch targets within stat cards are the full card area, not just the visible number — tapping anywhere on a stat card could drill into detail.

The CTA button contains a label and a subtle arrow icon, both centered. On press, it scales to 0.97 with a brief 150ms ease, providing tactile feedback. The disabled state reduces opacity to 40% and removes the shadow, making it clearly inactive without removing it from the layout and disrupting the thumb-zone muscle memory.

# Do's and Don'ts

Do place the primary action in the bottom-center thumb arc. Do use the warm terracotta exclusively for interactive elements and key data. Do keep the upper third of the hero visually quiet — this is scan space, not action space. Do test all layouts one-handed on actual devices in motion. Do let the background gradient carry warmth so the foreground elements can stay functionally neutral.

Don't scatter interactive elements throughout the viewport height. Don't use primary 500 for decorative elements — it is reserved for action and meaning. Don't introduce a secondary bright accent color; the palette's restraint is its strength. Don't use Anybody below 20px — its personality becomes noise at small sizes. Don't place text over the atmospheric header without checking contrast against the specific image or gradient used in production.