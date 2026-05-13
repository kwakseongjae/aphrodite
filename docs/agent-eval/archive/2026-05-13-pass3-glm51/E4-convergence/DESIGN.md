---
name: "longevity-editorial"
version: "0.1"
description: "Calm editorial site for a longevity research clinic with warm off-white grounds, muted sage accents, and scholarly serif typography"
colors:
  primary:
    "50": "#f2efe8"
    "100": "#ded8cb"
    "200": "#c4bba6"
    "300": "#a89d84"
    "400": "#8a7f6a"
    "500": "#7a7062"
    "600": "#635a4e"
    "700": "#524b42"
    "800": "#443f38"
    "900": "#3a3530"
  accent:
    "50": "#f7f5f3"
    "100": "#e5e3e0"
    "200": "#c9c5be"
    "300": "#aaa49a"
    "400": "#908878"
    "500": "#7a7268"
    "600": "#5e574e"
    "700": "#4a443d"
    "800": "#3d3833"
    "900": "#34302c"
  neutral:
    "0": "#ffffff"
    "50": "#faf9f7"
    "100": "#f0eeea"
    "200": "#ddd9d2"
    "300": "#c4bfb5"
    "400": "#a49e92"
    "500": "#878179"
    "600": "#6b6660"
    "700": "#55514c"
    "800": "#3d3a37"
    "900": "#2a2826"
    "1000": "#1a1816"
typography:
  display:
    family: "Newsreader"
    weight: 700
  body:
    family: "Cormorant Garamond"
    weight: 400
spacing:
  "1": "4px"
  "2": "8px"
  "4": "16px"
  "8": "32px"
rounded:
  sm: "3px"
  md: "6px"
metadata:
  variants:
    light:
      description: "Warm parchment light mode with earthy terracotta-brown text"
      tokens:
        colors.background.primary: "#f8f4eb"
        colors.text.primary: "#2a2318"
    dark:
      description: "Deep warm charcoal with warm cream text"
      tokens:
        colors.background.primary: "#1e1c18"
        colors.text.primary: "#ede8de"
    brand-a:
      description: "Warm honey-cream ground with deep umber text"
      tokens:
        colors.background.primary: "#faf5ea"
        colors.text.primary: "#261e12"
    brand-b:
      description: "Sage-linen with deep forest text"
      tokens:
        colors.background.primary: "#f3f0e6"
        colors.text.primary: "#1a2418"
---

# Overview

This design system serves a longevity research clinic that needs to feel authoritative yet approachable—more scholarly journal than medical waiting room. The palette draws from aged paper, raw linen, and the quiet greens of medicinal botanicals rather than clinical whites or sterile blues. Warmth is structural, not decorative.

The typographic foundation rests on Newsreader for display headings, chosen for its newspaper-editorial authority and slightly condensed letterforms that nod to scientific journals. Cormorant Garamond handles body text with a more open, classical serif that remains comfortable at reading sizes across long-form research summaries. This pairing creates a subtle tension between authority and elegance.

Spacing is generous and rhythmic, favoring breathing room over density. The system avoids the temptation to fill every pixel, instead trusting that negative space communicates as much about credibility as any headline can. This restraint extends to color usage: accents appear sparingly, like marginalia in a well-kept research notebook.

# Colors

The primary palette centers on warm earth tones—a family of muted browns, ochres, and taupes that evoke natural materials rather than digital screens. The 500-level primary sits at a mid-tone warm gray-brown, versatile enough for subtle borders, secondary text, or quiet interactive elements without competing for attention.

The accent scale introduces a restrained sage that feels medicinal without being sterile. It appears on links, active states, and small directional cues. Neither the primary nor accent families are saturated enough to feel decorative; they serve functional hierarchy.

Backgrounds across all variants stay warm-toned, avoiding cool grays that would undermine the editorial warmth. The darkest variant uses a warm charcoal with brown undertones rather than blue-blacks, maintaining palette coherence even in low-light contexts.

# Typography

Newsreader at 700 weight anchors headlines with editorial gravity. Its italic variant serves pull quotes and research citations, creating a natural hierarchy within the serif family alone. Headings use tight tracking and generous line height to reinforce the journal aesthetic.

Cormorant Garamond at 400 handles body text comfortably at 17–19px with 1.6–1.7 line-height. Its slightly wider letterforms and open counters maintain readability across the warm, slightly textured backgrounds. This is not a neutral choice—it brings classical gravitas that rewards sustained reading.

Type scale follows a modular progression with a 1.25 ratio, creating clear separation between heading levels without jumping too aggressively. Body text never drops below 16px, respecting the long-form reading patterns expected of research-minded visitors.

# Layout

Content width maxes at 680px for running text, deliberately narrower than common 800px frameworks. This decision supports the editorial reading experience and pairs well with the serif typography. Wider sections use a 12-column grid for card layouts and research summaries.

Vertical rhythm follows an 8px base unit, with major sections separated by 64px or 96px. This generous spacing lets each content block feel considered and complete rather than crammed. Sidebar navigation, when present, stays fixed and narrow—no more than 240px—to keep the focus on content.

Responsive behavior prioritizes the reading experience. On narrower viewports, margins shrink proportionally but the content column maintains its narrow comfort. Navigation collapses into a discreet menu rather than overwhelming the page.

# Elevation & Depth

This system uses minimal elevation. Surfaces rarely lift beyond a single shadow level, and even that is reserved for cards containing research summaries or team profiles. The dominant visual plane is flat and paper-like, reinforcing the editorial metaphor.

Shadow tokens use warm gray tones with minimal spread, creating a subtle lifting effect rather than dramatic depth. On dark backgrounds, shadows become barely perceptible warm glows. This approach keeps the design grounded and avoids the floating-card aesthetic common to more commercial health sites.

Border-based separation is preferred over shadow-based separation in most contexts. A 1px border in the 200-level neutral creates quiet structural clarity without suggesting physical depth that would contradict the flat, scholarly tone.

# Shapes

Corner radius stays small and consistent—3px for minor elements like tags and badges, 6px for larger containers and cards. These radii soften edges just enough to feel considered without introducing roundness that would undermine the editorial seriousness.

Buttons and interactive elements use the same tight radii, maintaining a rectilinear character throughout. Circular elements are reserved for author avatars and small status indicators, used sparingly to avoid breaking the grid-aligned visual language.

The overall shape language draws from printed materials: sharp corners with just enough softening to prevent visual harshness on screen. This positions the design closer to a well-typeset book than a mobile app.

# Components

Core components include a restrained navigation bar with wordmark and minimal links, a hero section featuring a single editorial headline with subtle animated reveal, and research article cards with date, title, and a one-line excerpt. Each component serves content first, never decoration.

Buttons use outline or ghost styling by default, with filled variants reserved for primary calls to action like appointment booking. Input fields echo the warm palette with subtle bottom-border styling rather than full outlines, reducing visual noise in forms.

Pull quotes, citation blocks, and research stat callouts receive distinct typographic treatment—larger sizes, italic variants, or accent color—to break up long-form content without requiring images. These components leverage the type system's full expressive range.

# Do's and Don'ts

Do maintain warm undertones across all surfaces and text, even in dark mode. Do let the serif typography lead the visual hierarchy. Do use generous spacing between content sections. Do keep accent color usage below 10% of visible surface area. Do pull colors from the defined scales rather than introducing new hues.

Don't introduce saturated colors outside the defined accent scale. Don't use sans-serif typefaces except for small labels or metadata. Don't add decorative illustrations where typography can communicate instead. Don't use drop shadows on text. Don't allow body text to exceed 680px width. Don't use rounded corners larger than 6px.