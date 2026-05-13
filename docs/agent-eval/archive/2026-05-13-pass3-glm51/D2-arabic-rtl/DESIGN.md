---
name: "iqra-landing"
version: "0.1"
description: "Arabic reading app landing page with warm terracotta-sienna palette, RTL layout, and Amiri Naskh typography"
colors:
  primary:
    "50":  "#fdf3ec"
    "100": "#f9dfd0"
    "200": "#f0c4a1"
    "300": "#e4a574"
    "400": "#d48850"
    "500": "#c46a35"
    "600": "#a55328"
    "700": "#844020"
    "800": "#63331b"
    "900": "#3d1e0f"
  neutral:
    "0":    "#ffffff"
    "50":   "#faf8f6"
    "100":  "#f2efeb"
    "200":  "#e4dfd9"
    "300":  "#d0c8be"
    "400":  "#b0a496"
    "500":  "#8a7d6f"
    "600":  "#6b6054"
    "700":  "#4d453c"
    "800":  "#302a23"
    "900":  "#1a1510"
    "1000": "#000000"
typography:
  display:
    family: "'Aref Ruqaa', 'Noto Naskh Arabic', serif"
    weight: 700
  body:
    family: "'Amiri', 'Noto Naskh Arabic', serif"
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
  sm: "4px"
  md: "8px"
  lg: "16px"
  xl: "24px"
metadata:
  variants:
    light:
      description: "Warm cream with deep terracotta text — inviting daylight reading"
      tokens:
        colors.background.primary: "#fdf8f3"
        colors.text.primary: "#2c1a0c"
    dark:
      description: "Deep umber night mode with warm cream text"
      tokens:
        colors.background.primary: "#15100c"
        colors.text.primary: "#f2ebe3"
    brand-a:
      description: "Burnt sienna foreground variant — clay pottery aesthetic"
      tokens:
        colors.background.primary: "#faf0e6"
        colors.text.primary: "#381808"
    brand-b:
      description: "Bronze-green accent variant — desert oasis mood"
      tokens:
        colors.background.primary: "#f5f0ea"
        colors.text.primary: "#1c2a1e"

---

# Overview

This design system serves a modern Arabic reading application landing page rooted in the warmth of terracotta and sienna rather than the expected amber-gold. The palette draws from burnt clay, sun-dried earth, and the coppery tones of desert canyon walls at golden hour — a deliberate departure from conventional amber palettes that feel overused in Arabic-language product design.

The typographic foundation pairs Amiri for sustained body reading — a Naskh face designed by Khaled Hosny with generous x-height and confident hairline-to-stroke contrast — with Aref Ruqaa for headlines, introducing a calligraphic expressiveness that feels distinctly literary without being precious. Both fonts carry the weight of Arabic manuscript tradition into a crisp digital present.

Every surface, spacing decision, and interaction pattern assumes a right-to-first reading direction. The layout breathes from the leading right edge inward, with generous margins on the left acting as a visual exhale rather than a structural boundary. This is not a mirrored LTR layout; it is an RTL composition from its bones.

The four variants — light, dark, and two brand explorations — all maintain WCAG-AA contrast ratios above 4.5:1 between primary text and background. Warmth is never achieved at the expense of legibility.

# Colors

The primary palette centers on sienna-terracotta, spanning from a pale peach bloom at shade 50 to an near-black espresso at shade 900. The 500 value — a confident burnt sienna — serves as the anchor for interactive elements, links, and primary call-to-action surfaces. It avoids the orange family entirely, landing instead in the reddened brown territory that feels more like handmade ceramics than digital UI chrome.

Neutral tones carry subtle warmth throughout the scale. Even the lightest grays retain a faint amber cast, preventing the sterility that pure gray introduces into warm palettes. The darkest neutrals lean toward umber rather than pure black, maintaining tonal cohesion under overlays and modal scrim.

Color application follows a principle of restraint: large areas stay in neutral territory with primary reserved for moments of emphasis and action. This prevents the terracotta from overwhelming the reading experience — the palette exists to frame content, not compete with it.

# Typography

Amiri at 400 weight handles all body text, feature descriptions, and navigation labels. Its Naskh construction — with clear baseline adherence and moderate stroke modulation — was specifically engineered for comfortable long-form reading in Arabic. The letterforms carry enough personality to feel considered without distracting from the content itself.

Aref Ruqaa at 700 weight governs all display moments: hero headlines, section titles, and the application wordmark. Ruqaa script carries an inherent informality and warmth compared to the geometric severity of many display typefaces. Its slightly exaggerated descenders and ascenders create a distinctive silhouette when set at large sizes.

Type scale follows a 1.25 ratio starting from a 16px base, with careful attention to line-height in Arabic. Body text receives 1.8 line-height to accommodate the vertical complexity of connected Arabic letterforms, while headlines tighten to 1.3 for visual density.

# Layout

The grid operates on an eight-column system with RTL as the default text-direction and layout-flow assumption. The primary content column spans six columns on desktop, leaving two columns of breathing room on the left side — the trailing edge in Arabic reading flow. On mobile, the grid collapses to a single column with persistent RTL padding: 24px on the right, 16px on the left.

Vertical rhythm follows the spacing scale strictly. Section gaps use the 12-unit (48px) value, while inner component spacing uses 4-unit (16px) or 6-unit (24px) values. This creates a consistent downward cadence that mirrors the rhythmic regularity of Arabic verse meter.

The hero composition places the primary headline flush right with the application mockup or illustration floating left — creating a natural zigzag reading path that begins at the right margin and sweeps across the viewport. Below the fold, feature cards stack in a right-aligned grid with generous internal padding, each card a self-contained reading moment.

# Elevation & Depth

Elevation in this system is subtle and warm-toned. Shadows use the primary 900 shade at very low opacity rather than pure black, producing shadow tones that feel like natural shade rather than digital layering artifacts. A card at rest elevation uses a 2px offset with 8px blur; on hover, it shifts to 4px offset with 16px blur.

Dark mode reverses the elevation logic: lighter surfaces float above darker bases. Instead of shadows, elevated components in dark mode use a faint 1px border in neutral 700 to separate layers, supplemented by a subtle inner glow in primary 800 at 8% opacity.

The overall depth language remains flat and typographic. Color, weight, and spacing carry hierarchy. Elevation is reserved for interactive cards and modal surfaces only — never for static content sections.

# Shapes

Border radius follows a gentle curve language: small interactive elements like buttons and inputs use the md value (8px), while larger surfaces like cards and feature sections use lg (16px). The hero container and full-width sections may use xl (24px) for a softer, more approachable silhouette.

There is no sharp-cornered severity in this system. Even the smallest chips and tags carry at least sm (4px) rounding. This softness aligns with the organic warmth of the color palette and the calligraphic nature of the typography — hard geometric angles would create dissonance against the Ruqaa headlines.

Icon containers use circular masks where applicable, breaking the rectangular rhythm with punctuated roundness. This provides visual variety without introducing a competing shape language.

# Components

Buttons follow a clear hierarchy: primary buttons fill with primary 500 and carry white text; secondary buttons use a 1px border in primary 500 with primary 700 text; ghost buttons remove the border entirely and rely on text color alone. All buttons use md rounding and include a right-aligned icon where applicable, maintaining the RTL flow.

Cards present feature descriptions in a stacked vertical layout: icon or illustration at top, headline in Aref Ruqaa at 24px, body text in Amiri at 16px. Padding follows the 6-unit (24px) value on all sides, with hover states elevating the shadow and shifting the card upward by 2px.

The navigation bar uses a transparent background that gains neutral 0 at 95% opacity on scroll. Navigation items are right-aligned with no divider between them — spacing alone creates separation. The active state uses primary 500 as an underline extending from the right edge of each label.

# Do's and Don'ts

Do maintain generous line-height for all Arabic text. Arabic script requires more vertical space than Latin due to its connected forms and frequent use of diacritical marks. Never compress Arabic body text below 1.7 line-height. Do align all text elements to the right by default and build layouts that flow from right to left. Never assume LTR and mirror — compose in RTL from the start.

Do use the warm neutral palette for backgrounds and reserve terracotta accents for moments of interaction and emphasis. Don't flood large surfaces with primary color, which will overwhelm the reader and undermine the calm reading atmosphere. Do pair Amiri with Aref Ruqaa consistently across the page. Don't introduce additional display faces — two typefaces are sufficient for a focused landing page. Do test all four variants on actual devices to verify that the warm undertones render correctly across different screen calibrations, particularly on OLED panels where dark umber tones can shift unpredictably.