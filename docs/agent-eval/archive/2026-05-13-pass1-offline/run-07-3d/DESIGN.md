---
name: Include A Threejs
version: "0.1"
description: "include a three.js parallax scene with a refracting glass orb hovering above the hero, gentle gyroscope-driven rotation on mobile"
colors:
  primary:
    "50":  "#fdf6f2"
    "100": "#f9e8dc"
    "200": "#f3d1ba"
    "300": "#eab28a"
    "400": "#e08d52"
    "500": "#c36522"
    "600": "#9c511c"
    "700": "#793f15"
    "800": "#572d0f"
    "900": "#341b09"
  secondary:
    "50":  "#f2f8fd"
    "500": "#2280c3"
    "900": "#092234"
  neutral:
    "0":    "#ffffff"
    "1000": "#000000"
typography:
  display:
    family: "Inter Tight"
    weight: 700
  body:
    family: "Inter"
    weight: 400
spacing:
  "1": "4px"
  "2": "8px"
  "4": "16px"
  "8": "32px"
rounded:
  sm: "4px"
  md: "8px"
  lg: "16px"
metadata:
  variants:
    light:
      description: "Default light mode"
      tokens:
        colors.background.primary: "#ffffff"
        colors.text.primary:        "#0b0b10"
    dark:
      description: "Dark mode"
      tokens:
        colors.background.primary: "#0b0b10"
        colors.text.primary:        "#f5f5f7"
    brand-a:
      description: "Warm brand variant"
      tokens:
        colors.background.primary: "#fff8f1"
        colors.text.primary:        "#241407"
    brand-b:
      description: "Cool brand variant"
      tokens:
        colors.background.primary: "#f1faf8"
        colors.text.primary:        "#04231d"
---

# Overview

include a three.js parallax scene with a refracting glass orb hovering above the hero, gentle gyroscope-driven rotation on mobile. This document is the offline-generated design baseline. It
satisfies the Google Labs DESIGN.md alpha schema and ships four variants
(light, dark, brand-a, brand-b), each WCAG-AA-validated for text-on-background
contrast.

# Colors

The primary palette anchors brand recognition. Use shade 500 for accent
fills, 700 for active states, and 50/100 for hairline surfaces. Secondary
exists as a deliberate complement; reach for it sparingly.

# Typography

Inter Tight for display, Inter for body. Display steps follow a 1.25 ratio
from 16px body. Line-height: 1.5 (body), 1.1 (display).

# Layout

8pt baseline grid. Container max-widths follow Tailwind defaults
(640 / 768 / 1024 / 1280 / 1536). Side gutters are spacing.4 (16px) on
mobile, scaling to spacing.8 (32px) above the 768 breakpoint.

# Elevation & Depth

Three depth levels: 0 (flat), 1 (subtle 1-2px shadow for cards), 2
(modal/overlay 8-16px shadow). Never combine elevations within a single
hierarchy.

# Shapes

Default radius is rounded.md (8px). Use rounded.sm (4px) on dense
controls and rounded.lg (16px) on hero containers. Pure-circular is
reserved for avatars and floating action buttons.

# Components

Buttons: solid primary, outline secondary, ghost tertiary. Inputs: bordered
with rounded.md, focus ring uses primary.500 at 40% alpha. Cards: primary.50
on light variant, primary.900 on dark.

# Do's and Don'ts

DO use primary.500 as the only accent fill in a single view. DON'T mix
primary and secondary as adjacent fills. DO maintain ≥ 4.5:1 contrast on
all text. DON'T introduce shades not defined in this document.
