---
name: "jangsu"
version: "0.1"
description: "A contemplative longevity research clinic in Seoul — patience, restraint, the unforced passage of time. Research abstracts presented as a single-column list with hairline dividers."
colors:
  primary:
    "50": "#f7f6f3"
    "100": "#eae8e2"
    "200": "#d5d1c8"
    "300": "#b8b2a5"
    "400": "#9a9284"
    "500": "#7d7568"
    "600": "#645d52"
    "700": "#4a453d"
    "800": "#322e28"
    "900": "#1a1816"
  neutral:
    "0": "#ffffff"
    "50": "#fafaf8"
    "100": "#f0efec"
    "200": "#dddbd6"
    "300": "#c4c1ba"
    "400": "#a8a49c"
    "500": "#8d8880"
    "600": "#736e66"
    "700": "#5a564f"
    "800": "#3d3a35"
    "900": "#1f1d1a"
    "1000": "#000000"
typography:
  display:
    family: "Source Serif 4"
    weight: 400
  body:
    family: "Pretendard"
    weight: 300
  cjk:
    family: "Apple SD Gothic Neo"
    weight: 300
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
  "32": "128px"
  "40": "160px"
  "48": "192px"
rounded:
  sm: "2px"
  md: "4px"
metadata:
  variants:
    light:
      description: "Paper — the default surface, like hanji left in morning light"
      tokens:
        colors.background.primary: "#fafaf8"
        colors.text.primary: "#1a1816"
    dark:
      description: "Inkstone — the desk at dusk, calligraphy in lamplight"
      tokens:
        colors.background.primary: "#1a1816"
        colors.text.primary: "#f0efec"
    brand-a:
      description: "Earthenware — warm clay vessel, onggi breath"
      tokens:
        colors.background.primary: "#f0ede6"
        colors.text.primary: "#1a1816"
    brand-b:
      description: "Bamboo shadow — cool grey-green, the waiting room garden"
      tokens:
        colors.background.primary: "#eef1ee"
        colors.text.primary: "#1a1816"

---

# Overview

This design serves a longevity research clinic in Seoul whose brand voice is patience — the quiet conviction that time, given the right conditions, is the only medicine worth studying. The visual system borrows nothing from clinical aesthetics. There are no white coats, no cyan accents, no diagrams of molecular bonds rendered in primary blue. Instead, the register is closer to a Korean seonbi's study: ink, paper, undyed cloth, the light through a rice-paper window at four in the afternoon.

The palette is built from a single warm grey-brown family — the colour of hanji paper aged five years, of onggi clay before glazing, of the ink left in the stone after the brush has been lifted. One family, many values. The restraint is deliberate: in Korean aesthetic tradition, the unadorned surface is not emptiness but potential. The viewer brings the meaning; the design provides the vessel.

Every surface in this system holds its breath. Negative space occupies between sixty and seventy percent of any given canvas. Text is set light — Pretendard at weight 300 for Korean body text, Source Serif 4 at regular weight for English display headings. Bold is an event, reserved for a single word or a single line per page, never the default voice. The typography breathes because we have given it room to breathe.

Icons, where absolutely necessary, are drawn from Lucide's minimal stroke set and used at 20px — smaller than the default, closer to annotation than illustration. Photography is single-image, centred, vast-margined. One photograph per section. The photograph floats in paper-white like a specimen mounted on washi. This is not austerity for its own sake. It is the conviction that attention, like longevity, cannot be forced — only invited.

# Colors

The primary colour family is rooted in raw umber — a warm grey-brown that references aged paper, clay, and the pale wood of a scholar's desk. It is not a colour that announces itself. At its lightest (shade 50, #f7f6f3) it reads as a warm paper surface barely distinguishable from pure white. At its darkest (shade 900, #1a1816) it is an ink-black softened by warmth — the colour of sumi-e ink diluted with water rather than applied at full concentration.

There is no accent colour. The decision is intentional. A longevity clinic that introduces a saturated accent risks communicating urgency, energy, dynamism — precisely the qualities that undermine a brand built on patience. Instead, the full tonal range of the primary family provides all necessary differentiation. Shade 200 for hairline dividers. Shade 400 for secondary text. Shade 700 for subtle borders. The system is complete without a second hue.

For the four variants: light mode uses paper-white (#fafaf8) as its dominant surface — a white with the faintest warm undertone, like hanji rather than laser printer stock. Dark mode inverts to inkstone (#1a1816), the warm near-black of a calligraphy stone. Brand-a shifts warmer to earthenware (#f0ede6), the buff of unglazed clay. Brand-b introduces the faintest cool-green cast (#eef1ee), like light filtered through bamboo — the only moment the system acknowledges green, and it is more suggestion than statement.

All four variants maintain WCAG-AA contrast between primary text and background. Light mode: #1a1816 on #fafaf8 yields approximately 14.5:1. Dark mode: #f0efec on #1a1816 yields approximately 14.2:1. Brand-a: #1a1816 on #f0ede6 yields approximately 14.1:1. Brand-b: #1a1816 on #eef1ee yields approximately 14.8:1. Contrast is not a constraint to be met reluctantly; it is a courtesy to the reader that costs nothing and gives everything.

# Typography

Source Serif 4 serves as the display typeface for English-language headings and the clinic's name. It is chosen for its optical sizing axis — at display scales, the serifs open slightly, maintaining clarity without heaviness. Weight is 400, regular. There is no bold display weight in this system. The heading is distinguished from body text by size and position, not by thickness. This is the Korean way: hierarchy through placement and proportion, not through force.

Korean body text is set in Pretendard at weight 300 (light). For CJK surfaces, the lightest weight that reads clearly is always preferred. The inter-character spacing in Korean text must be treated as content — the air between Hangul syllables is not wasted space but rhythmic structure. Letter-spacing should be set to approximately 0.02–0.04em for body text, enough to let each character breathe but not enough to suggest spacing was applied deliberately. The effect should feel natural, like the spacing between stones in a garden path.

Body text size is 15px with a line-height of 1.75. This is slightly larger and significantly more generously leaded than web defaults, reflecting the reading pace the clinic wishes to model: unhurried, comfortable, willing to take an extra moment. Display headings peak at 36px — never approaching the 48px upper limit, because longevity is not a subject that benefits from shouting. English body text, where it appears alongside Korean, uses the same Pretendard at 300 or falls back to Inter at weight 300.

The type scale is deliberately flat. Three sizes: body (15px), sub-display (20px), and display (28–36px). A fourth size, caption (12px), exists for metadata and dates. There is no micro-copy below 11px and no super-display above 40px. The scale mirrors the clinic's philosophy: time measured in decades, not in dramatic moments.

# Layout

The layout system is built on a single column of generous width — max-width 680px for prose, 960px for image-and-text compositions — centered on the viewport with margins no narrower than 10% on either side. The content column floats in space like a page of a book laid open on a wide desk. There is never a reason for content to touch the viewport edge.

Vertical spacing follows a stepped scale that plateaus at 192px. Between major sections, 128 to 192 pixels of vertical space. Between a heading and its body text, 24px. Between body paragraphs, 32px. These are not gaps to be filled but silences to be inhabited. The reader's eye rests between sections the way a calligrapher's brush lifts between strokes — the pause is part of the stroke.

Horizontal layout uses a 12-column grid at desktop widths, but most compositions use only 6 to 8 of those columns, leaving the remainder as intentional margin. The research abstract list occupies the full prose column — max-width 680px — as a single continuous flow: title, body, hairline, title, body, hairline. No cards, no grid, no multi-column arrangement. The abstracts are laid out like entries in a scholarly index, each one given the full width of the reading column, separated from the next by nothing more than a single 1px rule.

Mobile layouts reduce to a single column with proportionally maintained margins (8% minimum on each side). Vertical spacing scales down by half at the 32px threshold: a 192px desktop gap becomes 96px on mobile. Below that, spacing holds firm. The mobile experience should feel like reading a small-format book — intimate but never cramped.

# Elevation & Depth

There are no drop shadows in this system. None. Depth is communicated through tonal difference alone — a card on a background is one shade lighter or darker, separated by a hairline border if separation is needed. This is a direct application of the design philosophy: shadows simulate physical elevation, but a longevity clinic's visual identity should not simulate anything. It should simply be.

Borders are 1px solid, using primary shade 200 on light surfaces and shade 700 on dark surfaces. These hairlines serve the same function as the thin rules in traditional Korean book design — they separate without asserting, like the single thin line of a seonbi's brush that divides heaven from earth. The hairline dividers between research abstracts follow this same principle: present, precise, and completely recessive.

The only form of layering is opacity-based: a semi-transparent overlay (primary-900 at 4% opacity) on photographs when text must sit atop them. This technique avoids the common pattern of dark gradient overlays and instead creates a whisper of tone that protects readability without drawing attention to itself. The overlay is felt but not seen, which is precisely the relationship a patient should have with good design.

Focus states for interactive elements use a 2px outline offset by 2px, coloured in primary-500. This is the only moment the system raises its voice, and it does so for accessibility rather than aesthetics. The focus ring is not decorative — it is functional, necessary, and designed to disappear the moment it is no longer needed.

# Shapes

Rounded corners are minimal: 2px for small elements (tags, labels, date pills) and 4px for larger containers (image frames, content cards). These radii reference the faint softening that occurs on the edges of handmade paper — not a deliberate curve but the natural result of material meeting edge. There are no fully rounded elements, no pill buttons, no circular avatars. The geometry is almost-sharp, referencing the clean edges of a wooden printing block.

Buttons are rectangular with 4px radius, set in Pretendard 300 at 14px, with 12px vertical padding and 24px horizontal padding. They are quiet — primary-200 background with primary-900 text on light surfaces, inverted on dark. Hover states shift one shade darker. There is no animation on hover beyond an instant colour change. Longevity does not animate; it persists.

Tags and labels use the 2px radius and are set at 12px in primary-500 on light surfaces. They reference the small annotation slips (낱장, natjang) attached to traditional Korean documents — informational but visually recessive. A tag should be readable if sought but invisible if ignored. This is the correct hierarchy for metadata in a contemplative system.

Photographs are unmasked rectangles at 4px radius, maintaining their natural aspect ratio. No circles, no rounded arches, no decorative clipping paths. The image is what it is — a rectangle of captured light, presented with the same straightforward honesty the clinic brings to its research.

# Components

The primary composition is the Research Abstract Block: a single-column list of research abstracts, each entry consisting of a title line in Source Serif 4 at 20px, weight 400, followed by 24px of space, then body text in Pretendard at 15px, weight 300, line-height 1.75. There are no icons, no cards, no grid — each abstract is simply text in space, stacked vertically within the prose column. Abstracts are separated from one another by 1px hairline dividers in primary-200 (light surfaces) or primary-700 (dark surfaces), with 32px of vertical padding above and below each rule. The whole list flows as a single uninterrupted column — no decorative containers, no background tints, no multi-column arrangement. Spacing alone distinguishes one entry from the next.

The Navigation Bar is a single hairline element fixed to the top of the viewport: the clinic's name in Pretendard 300 at 14px on the left, navigation links in the same type at the same size on the right, separated by 32px. No logo mark, no symbol, no coloured bar. The navigation is a piece of paper with words on it, pinned to the top of the page. On scroll, a 1px border-bottom in primary-200 appears — the only acknowledgment that the user has moved.

The Image Section places a single photograph centred on the canvas with 10% margins on each side and 128px of vertical space above and below. Below the image, a caption in 12px Pretendard 300 describes what the photograph shows — not alt text, but curatorial text. "Microscope station in the second-floor laboratory, afternoon light." The caption is a quiet voice in the room, speaking only to those who choose to listen.

The Contact element is plain text: clinic name, address in Seoul, telephone number, email address. No form, no input fields, no submit button. A person you call, not a brand you submit tickets to. The typography is body-weight, body-size. The address is not wrapped in a card. It is words on a surface, and the reader who needs it will find it, and the reader who does not will pass over it undisturbed.

# Do's and Don'ts

Do let the whitespace do the work. A section with a heading and two lines of text, followed by 160px of nothing before the next section, is not incomplete — it is paced. The silence between sections is the design equivalent of the pause between breaths, and for a longevity clinic, breath is the most fundamental unit of time.

Do use one image where instinct suggests three. A single photograph of the clinic's interior — morning light on a wooden bench, the curve of an onggi jar in the garden, a researcher's hands adjusting a microscope — says more than a gallery of twelve images ever could. The viewer completes the scene. Provide the fragment; the viewer imagines the whole.

Do set Korean text with inter-character breathing room. The space between Hangul syllables is not a technical concern but an aesthetic one. Crushed Korean text reads as urgently as an all-caps English headline — which is precisely the opposite of the clinic's voice. Let the characters stand at their natural distance, like stones in a river crossing, each one placed with enough room to step on comfortably.

Don't use blue. The longevity research field defaults to clinical blue, to teal-green, to the colours of medical authority. This clinic's authority comes from restraint, not from colour-coded professionalism. The warm grey-brown family carries the brand without competition. Introducing blue would be like adding soy sauce to a dish that was already complete — it would not improve the flavour; it would only announce that the cook doubted the original recipe.

Don't animate. No fade-ins, no scroll-triggered reveals, no parallax, no loading spinners with encouraging messages. The page loads. The content is there. The user reads at their own pace. Animation implies that the designer knows when the user should pay attention, which implies the designer believes their timing is more important than the user's. For a clinic devoted to the long term, the user's timing is the only timing that matters.

Don't explain the design. No "our philosophy" section that describes the aesthetic choices in prose. The whitespace, the restrained type, the single image — these are the philosophy, expressed in the only language design genuinely speaks. A visitor who notices and appreciates the restraint will feel respected. A visitor who does not notice will simply find the site easy to read. Both outcomes are correct.