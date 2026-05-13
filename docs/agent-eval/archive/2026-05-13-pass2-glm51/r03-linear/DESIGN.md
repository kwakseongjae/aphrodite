---
name: "devtool-launch"
version: "0.1"
description: "Feature-launch microsite for a developer tool with refined restraint and precision"
colors:
  primary:
    "50":  "#f0f0ee"
    "100": "#dbdbd6"
    "200": "#b8b8af"
    "300": "#909085"
    "400": "#6e6e63"
    "500": "#58584d"
    "600": "#4e4e44"
    "700": "#42423a"
    "800": "#373731"
    "900": "#2a2a26"
  neutral:
    "0":    "#ffffff"
    "50":   "#fafaf8"
    "100":  "#f2f1ed"
    "200":  "#e4e3dd"
    "300":  "#d0cec5"
    "400":  "#b3b0a4"
    "500":  "#989589"
    "600":  "#7d7a6f"
    "700":  "#65635a"
    "800":  "#525049"
    "900":  "#3a3935"
    "1000": "#0e0e0c"
  accent:
    "50":  "#fef3e7"
    "200": "#f5cca0"
    "400": "#c98e1a"
    "600": "#9a6c13"
    "800": "#6b4a0e"
  success:
    "500": "#5a8a5c"
  error:
    "500": "#c45b4a"
typography:
  display:
    family: "Instrument Serif"
    weight: 700
  body:
    family: "Commit Mono"
    weight: 400
spacing:
  "0": "0px"
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
  none: "0px"
  sm: "2px"
  md: "6px"
  lg: "10px"
metadata:
  variants:
    light:
      description: "Default light mode — warm minimalism"
      tokens:
        colors.background.primary: "#fafaf8"
        colors.text.primary: "#1a1917"
        colors.text.secondary: "#65635a"
        colors.surface.raised: "#ffffff"
        colors.border.default: "#e4e3dd"
    dark:
      description: "Dark mode — deep warm charcoal"
      tokens:
        colors.background.primary: "#121210"
        colors.text.primary: "#f2f1ed"
        colors.text.secondary: "#989589"
        colors.surface.raised: "#1e1e1b"
        colors.border.default: "#3a3935"
    brand-a:
      description: "Warm ochre variant — grounded and earthy"
      tokens:
        colors.background.primary: "#fef3e7"
        colors.text.primary: "#2a1f0a"
        colors.text.secondary: "#7a6535"
        colors.surface.raised: "#fff8f0"
        colors.border.default: "#e8d8b8"
    brand-b:
      description: "Sage linen variant — muted botanical restraint"
      tokens:
        colors.background.primary: "#f0f4f0"
        colors.text.primary: "#1a2a1a"
        colors.text.secondary: "#4a5f4a"
        colors.surface.raised: "#f8faf8"
        colors.border.default: "#c8d4c8"
---

# Overview

This design system governs a feature-launch microsite for a developer tool. It takes inspiration from the restraint and spatial discipline of Linear's design language but subverts expectations: instead of the expected indigo or violet primary, we anchor the palette in a warm neutral-grey family (olived-steel) with a single ochre accent. The result feels like a well-worn notebook — precise but human.

The typographic pairing is deliberately unconventional for this category. Instrument Serif brings a quiet editorial authority to headlines, while Commit Mono — a monospaced face designed for long-form readability — handles all body text. This reversal of the typical sans-serif hierarchy signals craft and intentionality to a developer audience.

Motion and interaction are understated. Nothing animates without purpose. The system rewards attention rather than demanding it, which aligns with how developers actually evaluate tools: carefully, skeptically, with a bias toward substance.

# Colors

The primary palette lives in the warm grey spectrum — specifically the range between raw umber and cold steel. This avoids the saturated indigo that has become a default in developer tooling. The 500-value (#58584d) functions as a versatile mid-tone for interactive elements, while the 900-value provides authority in text and the 50-value recedes as background texture.

A single warm ochre accent (#c98e1a at 400) punctuates the system. It appears sparingly — on key call-to-action elements, active states, and moments of emphasis. Its warmth acts as a counterpoint to the primary's cool restraint. The ochre draws from amber and raw sienna rather than orange, keeping it grounded rather than energetic.

Every text-and-background pairing across all four variants meets WCAG-AA at the 4.5:1 ratio for normal text. We verified contrast mathematically, not visually. The dark variant uses #f2f1ed on #121210 (approximately 14:1), the light variant uses #1a1917 on #fafaf8 (approximately 16:1), and both brand variants maintain ratios above 10:1.

# Typography

Instrument Serif at 700 weight carries all display text. Its bracketed serifs and moderate stroke contrast communicate authority without ostentation. Headlines sit at 48–72px with tight tracking (negative 0.02em) to create density. The serif choice deliberately breaks from the sans-serif convention of developer microsites, establishing a distinct editorial presence.

Commit Mono at 400 weight handles body copy, navigation labels, and interface text. Setting body text in a monospaced face is a statement — it signals that this tool respects the medium its users work in. Line height stays generous at 1.65 to maintain readability at 16–18px sizes. We avoid bolding within body text; hierarchy comes from size and color instead.

A strict type scale of 1.25 governs all sizes: 12, 14, 16, 20, 24, 32, 40, 48, 64, 80px. No intermediate values. This creates predictable rhythm and makes the spatial grid feel inevitable rather than arbitrary.

# Layout

The page operates on a 12-column grid with 24px gutters. Content maxes at 1200px, centered. The hero section breaks this constraint — its background bleeds full-width while its text content aligns to the grid. This tension between contained and uncontained space creates visual drama without chaos.

Vertical spacing follows the 8px base unit. Sections breathe at 96px intervals (spacing-24). Component-level padding uses 16px or 32px. The rhythm is regular enough to feel systematic but varied enough to avoid monotony. We reject the common pattern of uniform section spacing in favor of compression at the top (drawing the reader in) and expansion toward the bottom (giving the CTA room).

Responsive behavior is simple: below 768px, the grid collapses to a single column with 16px margins. No complex breakpoint choreography. The microsite should feel like a single coherent composition at every width, not a different layout at each size.

# Elevation & Depth

The system uses only three elevation levels: flat (no shadow), raised (a single subtle shadow), and floating (enhanced shadow plus border). This limited vocabulary prevents the depth chaos that undermines many microsites. Shadows are warm-tinted, not neutral gray, matching the palette's overall warmth.

Raised surfaces use a shadow of 0 1px 3px rgba(14, 14, 12, 0.06) with a 1px border in the variant's border.default token. Floating elements add a second shadow layer at 0 8px 24px rgba(14, 14, 12, 0.08). Both shadows remain close and tight — they suggest physicality without theatricality.

Dark mode inverts the logic subtly: borders become more visible relative to backgrounds, and shadows become less prominent. Elevation in dark mode communicates primarily through border luminance and surface color shifts, not shadow depth.

# Shapes

Border radii are restrained: 2px for small elements like tags and badges, 6px for buttons and inputs, 10px for cards and modals. The small increments between levels keep the system feeling architectural. We avoid fully rounded elements entirely — no pill buttons, no circular cards.

The overall impression is crisp and rectilinear with just enough softening to avoid feeling hostile. This aligns with developer expectations: tools should feel precise and engineered, not playful or bubbly. The microsite's shapes should feel like well-machined components.

Full-width elements (hero backgrounds, dividers, footer bands) use no border radius. This creates a clear distinction between contained components and ambient surfaces.

# Components

Buttons exist in two weights. Primary buttons use the ochre accent background with dark text (ochre-400 on dark backgrounds, ochre-600 on light). Secondary buttons use transparent backgrounds with a border in the neutral-400 range. Both share the same height (44px), horizontal padding (24px), and border radius (6px). Hover states darken the background by one shade and introduce a subtle scale transform (1.01).

Code blocks use a distinct surface token — a slightly recessed background pulled from the neutral-800 range in light mode and neutral-900 in dark. Syntax highlighting uses muted versions of the accent palette rather than saturated rainbow colors. Line numbers are optional but when present, use text.secondary color.

Navigation is minimal: a wordmark on the left, three to five links in Commit Mono on the right, all in text.secondary color with text.primary on hover. No background blur effects, no scroll-triggered color changes. The nav should be invisible until needed.

# Do's and Don'ts

Do maintain the warm ochre accent's scarcity. It should appear on no more than two element types per page — typically the primary CTA and a single highlight element. If everything is accented, nothing is.

Do set all body text in Commit Mono at 16–18px with 1.65 line height. Resist the urge to switch to a sans-serif for "readability" — the monospace face is the point.

Don't introduce additional accent colors. The system is warm grey plus ochre. If you feel tempted to add a blue for links or a green for success states, use text.secondary coloring instead. The palette's power comes from its limitation.

Don't use animation durations below 150ms (feels broken) or above 400ms (feels sluggish). Ease-out curves only. No spring physics, no bounce, no theatrical reveals. If an element animates, it should feel like it's settling into place, not performing.

Don't allow any section to exceed 640px in text width. The grid may be wider, but prose lines must be constrained for readability. This applies to all variants and all breakpoints.