---
name: "edgefn"
version: "0.1"
description: "B2B SaaS landing page for a serverless edge-function platform built for Next.js teams"
colors:
  primary:
    "50": "#f0fdf4"
    "100": "#dcfce7"
    "200": "#bbf7d0"
    "300": "#86efac"
    "400": "#34d399"
    "500": "#10b981"
    "600": "#059669"
    "700": "#047857"
    "800": "#065f46"
    "900": "#064e3b"
  neutral:
    "0": "#ffffff"
    "50": "#fafafa"
    "100": "#f0f0f0"
    "200": "#d4d4d4"
    "300": "#a3a3a3"
    "400": "#737373"
    "500": "#525252"
    "600": "#3a3a3a"
    "700": "#2a2a2a"
    "800": "#1a1a1a"
    "900": "#0e0e10"
    "950": "#08080a"
    "1000": "#000000"
  surface:
    "raised": "#161618"
    "sunken": "#0a0a0c"
typography:
  display:
    family: "Space Grotesk"
    weight: 700
  body:
    family: "IBM Plex Sans"
    weight: 400
  mono:
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
  "24": "96px"
  "40": "160px"
rounded:
  sm: "4px"
  md: "8px"
  lg: "12px"
metadata:
  variants:
    light:
      description: "Clean documentation mode — light background for long-form reading and API reference"
      tokens:
        colors.background.primary: "#ffffff"
        colors.background.secondary: "#f5f5f5"
        colors.text.primary: "#1a1a1a"
        colors.text.secondary: "#525252"
        colors.border.primary: "rgba(0,0,0,0.10)"
    dark:
      description: "Canonical dark-default — matches the IDE environment where the audience lives"
      tokens:
        colors.background.primary: "#08080a"
        colors.background.secondary: "#161618"
        colors.text.primary: "#e8e8ec"
        colors.text.secondary: "#8b8b96"
        colors.border.primary: "rgba(255,255,255,0.08)"
    brand-a:
      description: "Warm cream — editorial warmth for case studies and long-form storytelling"
      tokens:
        colors.background.primary: "#f9f5ee"
        colors.background.secondary: "#efe9de"
        colors.text.primary: "#2c2518"
        colors.text.secondary: "#6b5d48"
        colors.border.primary: "rgba(44,37,24,0.12)"
    brand-b:
      description: "Cold slate — technical register for pricing and feature-comparison surfaces"
      tokens:
        colors.background.primary: "#f0f2f5"
        colors.background.secondary: "#e2e5ea"
        colors.text.primary: "#1c2028"
        colors.text.secondary: "#5a6170"
        colors.border.primary: "rgba(28,32,40,0.10)"

---

# Overview

EdgeFn targets Next.js engineering teams evaluating serverless edge infrastructure. The design system defaults to dark mode because the audience spends their working hours in IDEs, terminals, and browser devtools — all dark surfaces. Meeting them there removes a micro-friction that light-mode dev-tool sites create. Light mode exists as a secondary variant for documentation pages and long-form reading.

The primary accent is emerald (#10b981), not the expected violet or cobalt. This is deliberate: the hue maps to "running, live, deployed" in the mental model of developers who associate green with successful builds and healthy deploys. It differentiates from the Linear-violet and Stripe-cobalt that saturate the category.

Typography pairs Space Grotesk for display with IBM Plex Sans for body. Space Grotesk carries geometric confidence at large sizes without the crowd-identifiability of Inter. IBM Plex Sans is a workhorse with excellent readability at 14–16 px and a slightly wider stance than Inter, which prevents the "condensed newspaper" feeling at body scale.

Real product screenshots replace glassmorphic abstractions. The hero shows a terminal with an actual deploy command and output. Feature sections embed code blocks with real configuration files. This is a tool for people who write code — the page should look like code.

# Colors

The palette is strictly two-colour: emerald accent on a near-black or paper-white base. No secondary accent. No gradient CTAs. Emerald appears on the primary button, active states, links, and a single signal highlight per section — never as a background wash or decorative fill.

In dark mode, backgrounds sit at #08080a (primary) and #161618 (raised surfaces like cards and code blocks). Text is #e8e8ec for primary content and #8b8b96 for secondary labels. This maintains a contrast ratio above 15:1 for primary text, well beyond WCAG-AAA requirements. Emerald on dark base exceeds 8:1 contrast.

In light mode, the base flips to pure white with #1a1a1a text (contrast > 14:1). Emerald darkens to #059669 for text links to maintain > 4.5:1 on white. Brand-a (warm cream) and brand-b (cold slate) provide tonal shifts for long-form content without altering the core accent relationship.

Neutral grays are warm-shifted by roughly 2% — they lean slightly toward the emerald hue family rather than pure achromatic gray. This prevents the "clinical terminal" feeling that pure #808080 midtones create.

# Typography

Space Grotesk at 700 weight handles all display text: hero headlines, section titles, pricing tier names, and eyebrow tags. It excels at large sizes (56–96 px) because its geometric construction remains legible even at extreme weights. Letter-spacing tightens slightly at display sizes (-0.02em) to prevent the geometric forms from feeling airy.

IBM Plex Sans at 400 and 500 covers body text, navigation items, button labels, and form inputs. Its x-height is generous enough for comfortable reading at 14 px, making it suitable for dense feature lists and pricing tables. Medium weight (500) is reserved for interactive states and emphasis — never for decorative bolding.

JetBrains Mono at 13–14 px renders all code blocks, CLI examples, and inline code snippets. Line-height is set to 1.6 for code (slightly more generous than prose) to accommodate underscore-heavy identifiers common in Next.js configuration. Code blocks use the surface.raised background token with a 1 px border in the variant's border color.

The type scale follows a 1.25 ratio from a 16 px base: 16 → 20 → 25 → 31 → 39 → 49 → 61 → 76. Only five sizes are used in practice: 13 (captions, mono), 16 (body), 20 (subheadings), 31 (section titles), and 61–76 (hero headline, responsive).

# Layout

A 12-column grid at 24 px gutters governs all layouts. Content occupies 8 columns (66% of viewport) on desktop, centered. Side margins are never less than 15% of viewport width. This creates the breathing room that dense technical content demands — feature descriptions next to code blocks need spatial separation, not tight packing.

Section padding is generous: 160 px vertical between major sections. The hero gets 240 px of bottom padding before the first feature section. This is slower to scroll but rewards with clarity. Dev-tool audiences evaluate products through reading, not scanning — respect the reading pace.

The hero pattern is: eyebrow tag (11 px, small caps, emerald) → headline (clamp 48–84 px) → subhead (18 px, secondary text color) → dual CTA stack (primary button + ghost button) → product visual (terminal screenshot, 60% viewport width, left-aligned with the content column). The product visual is never centered — it anchors to the grid and extends rightward into the margin.

Feature rows use a 3-column layout with 32 px gaps. Each column contains a 36 px Lucide icon (terminal, zap, globe, shield, timer, layers) at emerald stroke, a 20 px title, and a 14 px description. No cards, no borders — the icons and type hierarchy alone create structure.

# Elevation and Depth

Elevation is minimal and functional. Dark mode uses two surface levels: primary (#08080a) and raised (#161618). Code blocks, cards, and interactive elements sit on raised. The page background is primary. This two-tier system is enough — any additional layers read as decorative noise.

Borders replace shadows for component definition. A 1 px border in rgba(255,255,255,0.08) separates cards from the background in dark mode. On hover, the border shifts to emerald at 30% opacity. This is faster to render than box-shadow transitions and reads as more precise — appropriate for an infrastructure product.

No drop shadows. No frosted glass. No layered z-depth. The design carries authority through typographic contrast and spatial generosity, not through simulated physical depth. A developer tool should feel like a terminal, not a design-system showcase.

Buttons use a single internal shadow on press (inset 0 1 px 2 px rgba(0,0,0,0.2)) for tactile feedback. This is the only shadow in the system. It exists because buttons are interactive elements that benefit from physical-state signaling.

# Shapes

Border radius is limited to three tokens: 4 px (sm), 8 px (md), and 12 px (lg). Buttons and inputs use md. Code blocks use sm. Pricing tier cards use lg. Nothing exceeds 12 px — larger radii soften the technical register and drift toward consumer-product aesthetics.

Icons are rendered at their native stroke-based construction. Lucide icons at 24 px viewBox with 2 px stroke, scaled to 36 px for feature highlights. No filled icon variants, no rounded icon backgrounds, no icon-within-circle patterns. The stroke-only approach matches the typographic restraint.

The single graphic element on the page is a 1 px horizontal rule in the variant's border color, used between sections. No decorative shapes, no background patterns, no geometric ornaments. The emerald accent color and the type contrast provide all the visual interest the page needs.

Component corners are never mixed — a card uses lg on all four corners consistently. Inconsistent radius within a single component signals indecision, not dynamism.

# Components

Primary button: emerald background (#10b981), white text, 8 px radius, 12 px vertical padding, 24 px horizontal padding, IBM Plex Sans Medium at 14 px, uppercase tracking 0.04em. Hover darkens to #059669. Focus ring is 2 px emerald offset by 2 px. This button appears once above the fold and once at the final CTA.

Ghost button: transparent background, #e8e8ec text (dark mode) or #1a1a1a (light), 1 px border matching text color, same sizing and type treatment as primary. Hover fills with rgba(255,255,255,0.05). Used as the secondary CTA alongside primary, and for navigation links that need button affordance.

Code block: surface.raised background, 1 px border, 4 px radius, JetBrains Mono at 13 px, line-height 1.6, 16 px padding. Language label in upper-right corner at 11 px caps in secondary text color. Scrollable horizontally when content overflows. A copy button appears on hover in the upper-right corner.

Feature column: Lucide icon at 36 px in emerald, Space Grotesk title at 20 px, IBM Plex Sans description at 14 px in secondary text. 24 px gap between icon and title, 8 px between title and description. No card wrapper, no border, no background — vertical spacing alone separates features from each other.

Pricing tier card: 320–400 px wide, 32 px padding, 1 px border, 12 px radius. Tier name in Space Grotesk at 24 px. Price in IBM Plex Sans at 48 px with 14 px period label. Feature list in 14 px body text with Lucide check icons at 16 px. Hover shifts border to emerald.

Navigation: horizontal top bar, 64 px height, fixed. Logo left, navigation links center, CTA button right. Links in IBM Plex Sans at 14 px, secondary text color, hover to primary text color. Active link has 2 px emerald underline offset 4 px from text baseline.

# Do's and Don'ts

Do show real terminal output and configuration files in the hero. The audience needs to see what deploying an edge function actually looks like — the command, the flags, the response. Abstract illustrations of "cloud infrastructure" signal that the product is not ready for production use.

Do use direct, verb-first copy. "Deploy edge functions in 8 regions" is a headline. "Ship Next.js middleware to the edge without configuration" is a subhead. "Reimagine your deployment pipeline" is a red flag that the copy was generated without understanding what the product does.

Do maintain strict accent discipline. Emerald appears on the primary CTA, active navigation states, icon strokes in feature sections, and inline code highlights. If you find yourself reaching for emerald in a fourth context, remove it from one of the existing three. A single accent color only works if it stays single.

Don't use gradient meshes, animated or static. This is Vercel's specific brand artifact. A different product using gradient meshes reads as a Vercel cover band, not a differentiated platform. The hero visual is a terminal, not a painting.

Don't use testimonial carousels, logo strips, or "trusted by" sections. This is a developer tool for technical evaluators. They read documentation, check GitHub repositories, and test deployment speed. Social proof for this audience is a fast deploy time and clean API, not a quoted endorsement.

Don't round corners beyond 12 px or add decorative shadows. The product deploys serverless functions to global infrastructure — the visual register should match that precision. Soft, rounded, shadow-heavy design language signals consumer SaaS, not infrastructure tooling.

Don't center-align long-form body text. Headlines can be center-aligned in the hero. Everything else — feature descriptions, pricing details, documentation — is left-aligned on the grid. Centered paragraphs beyond two lines reduce readability and signal template-level design thinking.