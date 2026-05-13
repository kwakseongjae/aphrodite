---
name: "Hanul-letter"
version: "0.1"
description: "Korean subscriber-only newsletter landing page with deep warm terracotta tones and bold serif headlines"
colors:
  primary:
    "50": "#fdf5ee"
    "100": "#f9e5d5"
    "200": "#f0c5a8"
    "300": "#e5a074"
    "400": "#d87d49"
    "500": "#b85a2e"
    "600": "#944422"
    "700": "#6e311a"
    "800": "#4a2113"
    "900": "#2c140b"
  neutral:
    "0": "#ffffff"
    "50": "#faf8f6"
    "100": "#f2ede8"
    "200": "#e0d8cf"
    "300": "#c4b9ab"
    "400": "#9e9284"
    "500": "#7a6f62"
    "600": "#5c5348"
    "700": "#3e3830"
    "800": "#2a2520"
    "900": "#1a1714"
    "1000": "#0d0b09"
  warm:
    "50": "#fef9f3"
    "100": "#fbf0e3"
    "200": "#f5dfc5"
    "300": "#eacc9f"
    "400": "#ddb476"
    "500": "#cda065"
    "600": "#a87d44"
    "700": "#7d5b30"
    "800": "#574022"
    "900": "#382816"
typography:
  display:
    family: "'Noto Serif KR', 'Batang', 'SimSun', serif"
    weight: 700
  body:
    family: "'Noto Sans KR', 'Malgun Gothic', sans-serif"
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
rounded:
  sm: "4px"
  md: "8px"
  lg: "12px"
  xl: "16px"
metadata:
  variants:
    light:
      description: "Warm cream with deep terracotta text"
      tokens:
        colors.background.primary: "#faf8f6"
        colors.text.primary: "#1a1714"
    dark:
      description: "Deep warm charcoal with ivory text"
      tokens:
        colors.background.primary: "#0d0b09"
        colors.text.primary: "#f5f0ea"
    brand-a:
      description: "Terracotta accent with warm cream"
      tokens:
        colors.background.primary: "#fdf5ee"
        colors.text.primary: "#2c140b"
    brand-b:
      description: "Deep terracotta with light warm text"
      tokens:
        colors.background.primary: "#2c140b"
        colors.text.primary: "#fbf0e3"
---

# Overview

Hanul-letter is a private Korean newsletter landing page designed exclusively for existing subscribers. The design draws from the warmth of 한지 (Korean mulberry paper) and the depth of traditional earth pigments, creating a sense of quiet invitation rather than aggressive marketing. Every element reinforces the feeling that this content is curated, intentional, and worth slowing down for.

The primary hue family is terracotta-brown — deeper and more muted than typical orange or amber schemes. This avoids the predictability of red-blue palettes while delivering the rich, warm atmosphere the brief demands. Combined with Noto Serif KR at heavy weight for headlines and Noto Sans KR for body text, the system balances gravitas with readability.

The four variants cover the full range from light cream to deep charcoal, all maintaining the warm undertone that unifies the system. Each variant meets WCAG-AA contrast requirements, ensuring the intimate reading experience extends to accessibility.

# Colors

The terracotta palette anchors the entire system. Unlike brighter amber or red schemes, this brown-leaning orange carries cultural resonance with Korean earth tones — 오징어 (dried squid) amber, 단청 (traditional temple paint) warmth, and 진흙 (raw clay) depth. The 500 shade at #b85a2e reads as confident without shouting.

Neutral tones are deliberately warm-shifted. Even the lightest backgrounds carry a faint cream undertone, and the darkest backgrounds avoid true black in favor of a brown-black (#0d0b09) that feels like candlelit darkness rather than digital void.

The warm scale provides bridge tones for subtle gradients, hover states, and secondary surfaces. These prevent the palette from feeling like two disconnected temperature zones and instead create a continuous warmth spectrum from light to dark.

# Typography

Noto Serif KR at weight 700 provides the headline voice. Its thick strokes and traditional 명조 proportions communicate editorial authority while remaining approachable. For a Korean newsletter landing page, serif headlines signal "this is worth reading" more effectively than geometric sans-serif alternatives.

Body text uses Noto Sans KR at weight 400, optimized for screen rendering of hangul syllables. The clean humanist forms maintain excellent readability at 15-17px sizes typical for long-form newsletter content. The sans-serif/serif pairing creates clear hierarchy without relying on size alone.

Line height for Korean text should run slightly looser than Latin defaults — 1.7 to 1.8 for body, 1.3 to 1.4 for headlines — to accommodate hangul block characters comfortably. Letter-spacing remains at default, as Korean characters are not spacing-adjusted in the way Latin text often is.

# Layout

The landing page follows a single-column editorial layout with generous margins. Maximum content width of 680px mirrors the comfortable reading column of a physical newsletter, reinforcing the private-content atmosphere. Subscribers should feel like they are reading a personal letter, not navigating a dashboard.

Vertical rhythm follows an 8px baseline grid with intentional pauses at 32px and 48px intervals between major sections. The spacing feels deliberate and unhurried — no cramped subscription forms or dense feature grids. Each section breathes.

Key interaction zones — subscription confirmation, archive links, and subscriber-only entry points — are centered and given extra padding. The layout does not compete with multi-column newsletter platforms; it strips away complexity in favor of focused reading.

# Elevation & Depth

Depth is expressed through warm shadows rather than cool gray drop-shadows. A subtle box-shadow using rgba values pulled from the primary palette creates the impression of layers of paper resting on each other, not floating UI cards.

On dark variants, elevation reverses to subtle inner glows and lighter border highlights. This prevents the common dark-mode problem where elevated surfaces become invisible against already-dark backgrounds. The warm undertone ensures these transitions feel natural.

Background surfaces use opacity and blur sparingly. A single frosted-glass treatment on the subscriber navigation bar adds modernity without introducing cold digital textures. All other depth cues remain solid and tactile.

# Shapes

Border radii stay conservative: 4px for small elements like tags and badges, 8px for cards and buttons, 12px for larger content panels. The slightly rounded but never circular shapes echo the softness of worn paper edges rather than digital interface chrome.

Full-round shapes are reserved exclusively for the subscriber avatar or profile indicator, creating a single focal anchor point. All other rectangular elements maintain their geometric honesty with gentle corner softening.

Button shapes follow a consistent 8px radius with generous horizontal padding (minimum 32px) and comfortable vertical padding (12-14px). The proportions feel substantial and clickable without appearing as oversized hero buttons.

# Components

The subscriber greeting component combines Noto Serif KR for the subscriber's name with a warm background wash, creating an immediate personal connection. This sits at the top of every page load, reinforcing exclusivity.

Newsletter issue cards use a minimal layout: issue number in primary-300, title in serif, date in neutral-400, and a subtle border-left accent in primary-500. Hover lifts the card with a warm shadow expansion. The design prioritizes scannability across many issues.

The subscription status indicator is a small pill-shaped badge using primary-500 text on primary-50 background (light mode) or primary-200 text on primary-900 background (dark mode). It communicates status at a glance without visual dominance.

# Do's and Don'ts

Do maintain warm undertones across all surfaces and shadows. Even neutral elements should carry a faint warmth to preserve the cohesive atmosphere. Do use generous line-height and paragraph spacing for Korean readability. Do limit the serif typeface to headlines and featured quotes — never set body copy in Noto Serif KR at small sizes.

Don't introduce cool grays, blues, or purples as accent colors. These break the warm tonal system immediately. Don't use tight letter-spacing or condensed layouts — Korean text requires more generous horizontal and vertical space than Latin text. Don't animate elements with fast spring physics; use eased durations above 300ms to maintain the unhurried editorial tone.