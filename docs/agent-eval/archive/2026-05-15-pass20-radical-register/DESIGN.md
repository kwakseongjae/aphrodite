---
name: "silhouette-break"
version: "0.2"
description: "Lookbook for a Seoul deconstructive fashion house — anti-symmetry, off-black, Hangul-dominant, intra-line weight-tear typography"
colors:
  primary:
    "50": "#e6f2ed"
    "100": "#b3d9cc"
    "200": "#80bfaa"
    "300": "#4da689"
    "400": "#269970"
    "500": "#008c57"
    "600": "#007a4c"
    "700": "#00603b"
    "800": "#00462a"
    "900": "#003020"
  neutral:
    "0": "#ffffff"
    "50": "#f0eeeb"
    "100": "#d4d0cb"
    "200": "#a8a098"
    "300": "#7d7268"
    "400": "#5a4f44"
    "500": "#3a302a"
    "600": "#2a2220"
    "700": "#1e1918"
    "800": "#140f0e"
    "900": "#0a0808"
    "1000": "#000000"
typography:
  display:
    family: "Pretendard"
    weight: "900/100"
  body:
    family: "Pretendard"
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
  "20": "80px"
  "24": "96px"
  "32": "128px"
  "40": "160px"
rounded:
  sm: "0px"
  md: "0px"
  lg: "0px"
metadata:
  variants:
    light:
      description: "Bone gallery — sharp, cold white, text like graphite on plaster"
      tokens:
        colors.background.primary: "#f0eeeb"
        colors.text.primary: "#1e1918"
    dark:
      description: "Bleached black — charcoal-inflected black, bone-white type"
      tokens:
        colors.background.primary: "#0a0808"
        colors.text.primary: "#f0eeeb"
    brand-a:
      description: "Raw fibre — unbleached cotton with soot-black lettering"
      tokens:
        colors.background.primary: "#e6dfcf"
        colors.text.primary: "#1a1816"
    brand-b:
      description: "Seoul night — warm near-black with cold jade type"
      tokens:
        colors.background.primary: "#141210"
        colors.text.primary: "#e6f2ed"

---

# Overview

This is not a website that reassures. The Seoul designer whose work fills these pages cuts into fabric to reveal the body's refusal of symmetry; the screen must perform the same violence on layout convention. Every surface begins from the assumption that the grid is a lie the user has been told too many times. The grid is present — it is the seam — and then it is torn.

Black is the only honest colour. The palette is built from off-blacks, charcoal-inflected neutrals, and a single violent jade-green that appears only when the composition has earned the right to be interrupted. Bone-white serves as ground. The jade does not decorate; it punctures.

Typography carries the weight of the collection's identity. Hangul at monumental sizes — 72 to 120 pixels — because the character structure must be legible as shape, not just decoded as language. Latin recedes. Pretendard at hairline weight for captions; Pretendard at Black weight for display. But the weight gap is no longer a contrast between two typographic registers — it is a rupture within the same line. Display text splits itself: syllable blocks at weight 900 sheared against syllable blocks at weight 100, the same family eating itself. The tear is no longer between headings and body. The tear is inside the heading. Type enacts the seam-break the garments propose.

The lookbook is structured as a vertical scroll with magazine pacing: one dominant photograph per spread, narrow text rail offset to one side, generous emptiness where the eye expects marketing copy. Photographs are 3:4 portrait, sometimes cropped asymmetrically. The back of the garment is shown before the front. The viewer must work.

# Colors

The primary is jade-verdigris (#008c57), a colour drawn from oxidised copper — the moment metal begins to change into something unrecognisable. It appears sparingly: a single horizontal rule, one cropped character stroke, a collection season marker. It does not appear in backgrounds or large surfaces.

The neutral scale runs from bone (#f0eeeb) through graphite (#5a4f44) to charcoal-black (#0a0808). None of these are pure white or pure black. Pure white is a lie told by screens. The off-whites carry warmth because the gallery wall is never sterile — it is plaster, it breathes.

Brand-a pushes into unbleached territory (#e6dfcf), raw cotton before dye. Text against this is near-black (#1a1816) — contrast ratio above 13:1. Brand-b inverts: warm charcoal ground (#141210) with cold jade-tinted text (#e6f2ed) at contrast above 14:1. Both pass WCAG-AAA. Accessibility is not achieved through blandness but through committed extremity.

The dark variant is the default in my conviction, though light ships first for convention's sake. A black page with bone-white Hangul at 96 pixels is the closest a screen comes to the experience of entering the atelier — half-lit, the garment suspended, waiting to be read.

# Typography

Display type is Pretendard, but it does not settle into a single weight. Collection titles and season names use intra-line weight rupture: characters or syllable blocks at weight 900 placed directly adjacent to characters at weight 100 within the same display line, at 72–120 pixels. The transition from Black to Thin is not bridged — it is a fault line. Some compositions place the heavy syllables first, ending in dissolution; others reverse it, emerging from hairline into full black. The direction is a compositional decision, not a template.

This is the deconstructive gesture the persona demanded and that Noto Serif KR, for all its quality, could not provide. A conventional serif at heavy weight is still one voice. Pretendard 900 and Pretendard 100 on the same line are two voices in the same throat. The weight-tear is the seam made visible in type.

Body and captions use Pretendard at weight 300 — hairline, almost disappearing. The contrast between the intra-line rupture of display text and the uniform thinness of body text creates two distinct structural zones: where type performs violence, and where type recedes to let the garment speak.

Latin text appears only where necessary — contact information, season codes, material compositions in English. It is set in the same Pretendard, smaller, never dominating. The one concession: collection season codes use a monospace (JetBrains Mono) at 14px for a mechanical counterpoint to the weight-tear above.

Asymmetric type compositions: a collection title may begin at 120px and crop mid-character at the viewport edge. Captions align to a left margin that shifts 12px between sections. A single Hangul character may appear alone on an otherwise empty spread — not centred, but placed at roughly 30% from left, 25% from top. The placement feels wrong until the viewer realises it mirrors the off-centre closure of a deconstructed jacket.

# Layout

The vertical scroll is divided into spreads, not sections. Each spread is a self-contained compositional unit: one photograph or one typographic statement, set within generous negative space. The gap between spreads is 128–160 pixels — enough to create genuine pause, the magazine turn.

Content width is constrained to 60% of viewport on desktop (roughly 864px on 1440). The remaining 40% is intentional emptiness on one side or the other — the side alternates. Spread one offsets left; spread two offsets right. This asymmetry is not decoration; it is the page acknowledging that the body is not centred, that the garment hangs from one shoulder.

The navigation rail is minimal: designer name in Hangul at 24px, top-left, permanent. A thin horizontal rule in jade (#008c57) marks the current season. No hamburger menus, no dropdowns. Contact information is plain text at the bottom — an address in Seongsu-dong, a phone number, a mailto link. No contact form. You call, or you do not call.

Photographs are 3:4 portrait, full-bleed within the content column, never extending into the margin. Some spreads show the photograph cropped — the bottom 15% cut off by the viewport edge, scrollable into view. The garment is never fully visible in the first glance. You must move to see the whole.

# Elevation and Depth

There is no elevation. No drop shadows, no card lifts, no layered surfaces. The screen is flat because fabric is flat until the body enters it. Depth is achieved through scale contrast and photographic composition — a close-up of a seam at 100% viewport width reads as "closer" than a full garment shot at 60%, and that perceptual depth replaces the lie of shadow.

Hover states and interactions receive no visual elevation. A photograph on hover shifts 4px to the right — a flinch, not a lift. Text links gain a jade underline that appears from left to right over 200ms. These movements are lateral, not vertical. The screen refuses to pretend it has z-axis depth.

The single exception: the collection season overlay. When a season is selected, a full-viewport overlay slides in from the left (not from the centre, not from the bottom). The overlay sits at background #0a0808 with text at #f0eeeb. It is not a modal — it is a room you enter. The entry direction is deliberate: left-to-right mirrors Korean reading direction and the way a curtain pulls open.

# Shapes

All corners are zero radius. Sharp edges — no apologies, no softening. A rounded corner on this site would be equivalent to finishing a raw edge that should remain exposed. The rectangle is the only shape. Cards, images, buttons, inputs — all square-cornered.

Buttons are rectangles with a 1px border in the current text colour. No fill, no gradient. The hover state fills the rectangle with the text colour and inverts the text to the background colour — a simple negation. Height is 48px for touch targets; width is content plus 32px padding. The button does not announce itself as friendly. It announces itself as available.

Horizontal rules are 1px, full content width, in jade (#008c57) for season markers and in neutral-300 (#7d7268) for section divisions. No decorative lines, no SVG patterns. A line is a seam — it joins or it cuts, it does not decorate.

The overall shape language is architectural: rectangles in deliberate asymmetry, negative space as a shaped void. Where most sites use rounded corners to signal approachability, this site uses sharp corners to signal precision and refusal.

# Components

The Lookbook Spread is the primary component: one 3:4 image, a caption in Pretendard 300 at 14px, an optional season code in JetBrains Mono at 14px. The image occupies 70% of the content column; the caption occupies the remaining 30%, aligned to the bottom of the image. The caption may be offset 12px left of the image edge — the misalignment is the component's signature.

The Collection Nav is a horizontal strip at the top: season names in Pretendard 300 at 16px, separated by 24px gaps. The current season is marked by jade text colour; others are neutral-300. No background, no bounding box. Scrolling through collections replaces the current spread with a crossfade of 400ms — slow enough to feel like a page turn, not a teleport.

The Designer Statement is a single block of text, Pretendard 300 at 18px, line-height 1.8, maximum width 480px, placed at 30% from the left edge of the content column. It may run to 150 words. Below it, the designer's name in Pretendard 900 at 48px — surname only, Hangul only. The name uses the weight-tear: the first syllable block at 900, the second at 100, the family name dissolving into air or congealing from it depending on reading direction. No biography, no headshot, no credentials.

Contact Footer is plain text: the studio address in Seongsu-dong, a Seoul phone number, an email address. Each on its own line. Pretendard 300 at 14px. The jade horizontal rule appears above. Below the contact, a single line in neutral-400: the current year, the designer's name in Hangul. No social links, no newsletter signup, no footer navigation.

# Do's and Don'ts

**Do** let the photograph breathe. A spread with one image and five words of caption is complete. The silence around the garment is part of the presentation.

**Do** use Hangul as the dominant typographic voice. This is a Seoul designer; the Korean language is not a localization option, it is the material. Latin is the secondary voice and is sized accordingly.

**Do** break the expected alignment. If a caption feels like it should be centred under the image, offset it 12px to the left. If a title feels like it should be at the top, start it at 35% from the top. The slight wrongness is the design's honesty.

**Do** use the weight-tear within display lines. The rupture from 900 to 100 (or 100 to 900) in the same heading is the typographic equivalent of an exposed seam. It should feel violent, not decorative.

**Don't** use rounded corners anywhere. If a component feels like it needs softening, reconsider whether the component should exist — do not soften it.

**Don't** show statistics, testimonials, press logos, or social proof. The work is the argument. The garment either convinces or it does not; the number of people who have been convinced is irrelevant to the next viewer.

**Don't** centre anything unless the composition is monumental — a single Hangul character on an otherwise empty spread, or the designer's name as a closing statement. Centring is earned, not defaulted.

**Don't** add hover effects that lift, glow, or scale upward. Movement is lateral or it does not happen. The screen does not pretend to be three-dimensional.

**Don't** use display typefaces that carry conventional serif warmth. The weight-tear within Pretendard replaces the need for a display face entirely — the violence is structural, not stylistic.