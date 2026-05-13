---
name: "longevity-editorial"
version: "0.1"
description: "A scholarly editorial site for a longevity research clinic with warm restraint and quiet authority"
colors:
  primary:
    "50": "#f6f4ee"
    "100": "#e8e4d8"
    "200": "#d4ccba"
    "300": "#b5a993"
    "400": "#969078"
    "500": "#7a7462"
    "600": "#615c4e"
    "700": "#4d493f"
    "800": "#3b3832"
    "900": "#2a2823"
  neutral:
    "0": "#ffffff"
    "50": "#fafaf8"
    "100": "#f2f1ed"
    "200": "#e4e2db"
    "300": "#c9c5ba"
    "400": "#a9a496"
    "500": "#8a8577"
    "600": "#6e6a5e"
    "700": "#565339"
    "800": "#3d3b35"
    "900": "#252420"
    "1000": "#0e0e0c"
  accent:
    "50": "#f4f7f4"
    "100": "#e2eae1"
    "200": "#c5d5c4"
    "300": "#9db69b"
    "400": "#7a9a78"
    "500": "#60815f"
    "600": "#4e6a4e"
    "700": "#405540"
    "800": "#354436"
    "900": "#2b362c"
typography:
  display:
    family: "'Libre Caslon Display', 'Georgia', serif"
    weight: 400
  body:
    family: "'Source Serif 4', 'Georgia', serif"
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
      description: "Default warm light mode with parchment warmth"
      tokens:
        colors.background.primary: "#faf8f2"
        colors.text.primary: "#2a2823"
    dark:
      description: "Dark scholarly mode reminiscent of aged leather"
      tokens:
        colors.background.primary: "#18160f"
        colors.text.primary: "#e8e4d8"
    brand-a:
      description: "Warm sand variant evoking morning light"
      tokens:
        colors.background.primary: "#f5f0e6"
        colors.text.primary: "#2e2b22"
    brand-b:
      description: "Cool stone variant with quiet grey-green"
      tokens:
        colors.background.primary: "#eef1ed"
        colors.text.primary: "#232623"
---

# Overview

This system serves a longevity research clinic that communicates through calm authority rather than urgency. The design draws from editorial typography and archival restraint — the feeling of a well-funded private institution that values patience and precision over disruption. Every visual choice reinforces the clinic's core message: thoughtful care, rigorous science, long time horizons.

The primary palette avoids saturated color entirely, building instead from warm stone greys and desaturated earth tones. The sage accent recedes further than expected, appearing only in small deliberate moments — a selected state, a link hover, a section divider — so that its presence feels earned rather than decorative. This restraint prevents the site from feeling like a wellness brand and keeps it firmly in the scholarly register.

Typography drives the entire personality. Libre Caslon Display at display sizes creates a literary, publication-quality tone without feeling precious. Source Serif 4 handles body text with warmth and excellent readability at sizes suited to longer reading sessions. Neither font is trendy; both project timelessness appropriate to a clinic focused on extending life.

The four variants shift atmosphere without changing structure. Light mode glows like aged paper. Dark mode suggests a leather-bound study. Brand-a warms toward sand. Brand-b cools toward lichen on stone. Each maintains the same hierarchical logic and typographic scale.

# Colors

The primary color family orbits warm grey-brown, deliberately avoiding the blue and purple hues common to medical and technology brands. This choice positions the clinic alongside editorial institutions — publishers, archives, universities — rather than SaaS dashboards. The 500 value (#7a7462) serves as the default text-accent shade for labels, timestamps, and secondary information, while 800-900 handle primary text across variants.

Accent sage (#60815f at 500) is used with extreme restraint. It never appears as a background fill larger than a badge or pill. Its job is to signal "nature-adjacent but clinical" in moments where the warm neutrals alone feel insufficient — a published date, an active navigation item, a success state. Overuse would push the palette into wellness cliché; its scarcity protects the scholarly tone.

Background tokens across all four variants pass WCAG-AA contrast against their paired text colors with ratios exceeding 9:1 in every case. The warm undertone in all backgrounds (even brand-b's cool variant carries faint grey-green) prevents the clinical sterility of pure white or pure neutral grey, creating a living-room comfort that serves a clinic asking patients to think about long time horizons.

Color application follows a simple rule: the warmer the element's role, the warmer its tone. Page backgrounds are warmest. Card surfaces are slightly cooler. Interactive elements are coolest (closest to neutral). This creates subtle thermal layering that users feel without consciously seeing.

# Typography

Libre Caslon Display handles headlines exclusively — it is not used below 24px. Its delicate hairline serifs and generous proportions create the editorial authority the system needs. At weight 400 it already carries enough visual weight; going bold would break its character. Headlines are set with generous letter-spacing (+0.01em) and tight leading (1.1) to reinforce the publication feel.

Source Serif 4 manages everything below the headline tier: subheadings, body copy, captions, navigation labels, form text. Its slightly larger x-height and sturdy serifs maintain readability at 16-18px body sizes across extended reading sessions. Weight 400 handles most use; 600 provides emphasis for subheadings without requiring a size change. This dual-weight system avoids the visual noise of switching between serif and sans-serif.

The typographic scale is modular (1.25 ratio) but applied conservatively. Most pages use only three sizes: display (48px+), subheading (20px), and body (17px). This limited range prevents the arbitrary sizing that makes sites feel unstructured. Line lengths stay between 55-75 characters; the grid enforces this automatically.

Paragraph spacing (24px) exceeds line-height (1.65) to create clear rhythmic separation between text blocks. This spacing gives research content room to breathe and prevents the dense wall-of-text effect that plagues scientific writing on the web.

# Layout

The page grid is a single centered column with a maximum width of 720px for body content, expanding to 1080px for editorial layouts that pair text with marginal imagery. Gutters are 24px at mobile breakpoints and 32px at desktop. These narrow measures enforce comfortable reading and prevent the stretched, lonely feeling that wide columns create on clinical content.

Sections stack vertically with 64px separators — generous spacing that gives each content block conceptual independence. This vertical rhythm creates a scrolling experience akin to turning pages rather than scrolling a feed. Section headings sit at the top of each block with no inset, establishing a clear entry point before the reader engages with the content below.

Navigation is minimal: a wordmark top-left, three to five links top-right, all set in Source Serif 4 at body size with subtle letter-spacing. No sticky behavior, no background blur, no drop shadow. The header exists as content, not as UI chrome. It leaves the page when the user scrolls, reinforcing the editorial metaphor where the masthead is part of the publication, not a control surface.

Cards and feature panels use a single-column internal layout. Side-by-side arrangements are reserved for photo essays or research highlight grids. This constraint prevents complex responsive rearrangement and keeps every component's internal logic simple and predictable.

# Elevation & Depth

Elevation is nearly absent by design. The system uses only two surface levels: the page background and the card/panel surface. Cards achieve subtle separation through a 1px border in primary-200 rather than through shadow, creating a printed-on quality that reinforces the editorial metaphor. This flatness prevents the layered-app aesthetic that would feel inappropriate for a research institution.

The single shadow token (0 1px 3px rgba(0,0,0,0.06)) is reserved for overlays: dropdowns, modals, tooltips — elements that genuinely float above the page. Its softness and low opacity prevent it from reading as interactive depth; it simply ensures legibility against varied backgrounds.

Dark mode does not increase shadow opacity or spread. Instead, surface separation relies entirely on the slight tonal difference between background and card fills. This approach works because the warm undertones in both surfaces create enough distinction without resorting to visible shadow geometry.

Hover states use no elevation change. Interactive feedback comes from gentle opacity shifts and color transitions, keeping the surface plane stable. This choice reinforces the calm, non-reactive personality of the system.

# Shapes

Border-radius values are minimal throughout. The sm token (2px) handles most elements — buttons, inputs, tags, pills. The md token (4px) applies only to larger containers and cards. The lg token (6px) is reserved for image containers and feature panels. Anything rounder would feel casual in a way that contradicts the scholarly tone.

Buttons are the primary shaped element and follow strict rules: height is determined by padding (10px vertical, 20px horizontal), and the 2px radius creates a slightly softened rectangle that reads as precise without feeling sharp. Secondary and tertiary button variants share the same dimensions, differentiated only by fill and border treatment.

Image containers maintain consistent aspect ratios: 3:2 for editorial photography, 16:9 for research imagery and video, 1:1 for author portraits. These ratios are enforced by the layout system, not by the images themselves, preventing the visual inconsistency that crop-to-fill creates across a multi-author research site.

Tags and category labels use the pill shape (border-radius: full) but at small scale — they never exceed 28px in height. This keeps them as quiet metadata markers rather than prominent interactive elements.

# Components

The component library is intentionally small: heading, paragraph, card, button (primary, secondary, tertiary), tag, navigation, footer, image-container, divider, and quote-block. This limited set reflects the editorial function — the site publishes written content with supporting imagery, not complex interactive tools. Each component is designed to do one thing well within the typographic system.

Cards are the workhorse component. They present research summaries, team member profiles, publication listings, and news items. All cards share the same internal structure: an optional image region (top), a content region (middle, flexible height), and an optional action region (bottom). This consistency allows varied content types to coexist on the same page without visual chaos.

Quote blocks are a specialty component for this domain. Longevity research involves patient narratives and clinician insights that benefit from typographic distinction. Quotes are set in Libre Caslon Display at 24px, indented 32px from the left margin, with a 2px-wide, 48px-tall primary-300 accent bar on the left edge. This treatment gives patient voices visual weight without requiring colorful callout boxes.

Form inputs (contact, consultation request) use the same typographic and spatial rules as content components. Labels sit above inputs with 8px gap. Inputs are 48px tall with 1px primary-200 borders that darken to primary-400 on focus. Error states use a muted terracotta (#9b6b5b) rather than alarm red, maintaining the warm register even during validation failures.

# Do's and Don'ts

Do maintain generous whitespace around text blocks. The editorial tone depends on visual breathing room; cramped layouts undermine the sense of calm authority that the typography establishes. When in doubt, add another 8px of margin rather than remove it.

Do use the sage accent only for interactive states and small markers. A single green link in a paragraph of warm brown text draws the eye precisely because of its scarcity. If everything is accented, nothing stands out, and the palette collapses into visual noise.

Do set body text at 17px minimum with 1.65 line-height. Longevity research content is text-heavy by nature, and the audience skews toward readers who will spend sustained time with articles. Prioritize their comfort over density of information above the fold.

Don't introduce a sans-serif typeface for UI elements. The scholarly personality requires typographic consistency, and mixing Source Serif 4 with an interloper sans-serif would fracture the editorial illusion. If something feels like it "needs" sans-serif, reconsider whether the element itself is necessary.

Don't use the primary-500 shade for large background areas. It is designed for text and borders, not for fills. Large warm-brown surfaces feel oppressive rather than warm; the palette's warmth works best in thin applications against light backgrounds.

Don't animate element entrances, exits, or state transitions beyond 200ms opacity changes. The site should feel like a well-set printed page that happens to scroll, not like a living interface that draws attention to its own mechanics. Motion is permitted but invisible; it should never be noticed.