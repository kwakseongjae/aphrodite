---
name:
  walnut-seoul
version:
  "0.1"
description:
  A considered portfolio for an independent solid-walnut furniture maker in Seoul
colors:
  primary:
    "50":  "#f4f0eb"
    "100": "#e6ddd3"
    "200": "#cbbfac"
    "300": "#b09e88"
    "400": "#958264"
    "500": "#7a6b53"
    "600": "#655644"
    "700": "#514236"
    "800": "#3e322a"
    "900": "#2c231e"
  neutral:
    "0":    "#ffffff"
    "50":   "#f5f4f2"
    "100":  "#e8e6e2"
    "200":  "#d1cdc6"
    "300":  "#b3ac9f"
    "400":  "#958c7e"
    "500":  "#7a7162"
    "600":  "#5e5649"
    "700":  "#433d34"
    "800":  "#2a261f"
    "900":  "#15130f"
    "1000": "#000000"
typography:
  display:
    family: "Instrument Serif"
    weight: 400
  body:
    family: "Inter"
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
rounded:
  sm: "2px"
  md: "4px"
  lg: "6px"
metadata:
  variants:
    light:
      description: "Warm off-white, breathe and quiet"
      tokens:
        colors.background.primary: "#f5f4f2"
        colors.background.secondary: "#eae7e2"
        colors.text.primary: "#1c1915"
        colors.text.secondary: "#5e5649"
        colors.border.subtle: "#d4cfc7"
    dark:
      description: "Low-lamp atmosphere, wood in shadow"
      tokens:
        colors.background.primary: "#12100e"
        colors.background.secondary: "#1e1b17"
        colors.text.primary: "#e4dfd6"
        colors.text.secondary: "#958c7e"
        colors.border.subtle: "#3a352d"
    brand-a:
      description: "Seoul fog, morning hanok paper"
      tokens:
        colors.background.primary: "#e9e4db"
        colors.background.secondary: "#d9d2c6"
        colors.text.primary: "#1f1b14"
        colors.text.secondary: "#5a5344"
        colors.border.subtle: "#c4bba9"
    brand-b:
      description: "Charcoal ink wash, gallery evening"
      tokens:
        colors.background.primary: "#201d19"
        colors.background.secondary: "#2c2720"
        colors.text.primary: "#d9d3c7"
        colors.text.secondary: "#8e8576"
        colors.border.subtle: "#3e382e"
---

# Overview

This system serves a single craftsperson's portfolio — no commerce, no hustle, just the work. The palette is built around the material itself: warm mid-tones pulled from the heartwood of Juglans regia, the European walnut commonly worked in Korean studios. Every color decision starts from that warm gray-brown center and moves outward with restraint.

Seoul's design culture prizes negative space as much as positive form. The layout system borrows from that sensibility — generous margins, asymmetric grids, and a deliberate slowness to the scroll rhythm. The furniture deserves room to be seen. Nothing should compete with the object photography.

Instrument Serif at display weight is a conscious choice against the geometric sans-serifs common in Korean web portfolios. It carries a quiet editorial authority that matches the studio's positioning: independent, considered, uninterested in trends. Paired with Inter for body text, it creates a clear hierarchy without shouting.

The variant system reflects different viewing contexts and moods — from a bright morning browse to a late-night return visit. Each variant maintains the same tonal center but shifts the ambient light, as if the gallery walls themselves were repapered.

---

# Colors

The primary palette sits firmly in the yellow-red neutral range — warm but not earthy, sophisticated but not cold. This is not the orange-brown of rustic woodworking; it is the muted taupe of a well-aged walnut board under studio lighting. The 500 value (#7a6b53) is the anchor, and every other shade is mixed against it rather than simply darkening or lightening a single hue.

Neutral tones are warm-shifted throughout. Pure grays feel clinical against walnut; a faint warmth in every neutral value maintains the material connection. Even the darkest backgrounds carry a brown undertone, like a darkroom safelight or the glow of a low lamp on oiled wood.

Accent usage is minimal — the furniture and its surroundings provide whatever color the page needs. The primary palette exists for interactive elements and subtle emphasis only. A well-placed primary-600 link or a primary-200 background panel is all the accent most layouts require.

Variant shifts maintain consistent contrast ratios while changing the ambient temperature. The light variant is slightly warmer than a pure white; brand-a pushes further toward the ochre tones of aged hanji paper. Dark variants avoid blue-black in favor of warm charcoal, keeping the wood-tone connection even in shadow.

---

# Typography

Instrument Serif is an uncommon choice for a furniture portfolio — most reach for a clean sans-serif or a geometric display face. Its tapered strokes and subtle irregularity echo the hand-finished quality of the work. At large sizes, it has presence without weight. At smaller sizes, it remains legible and quietly distinguished.

Body text in Inter provides mechanical clarity. Its tall x-height and open counters serve Korean-language content equally well when the site includes Hangul descriptions. The weight stays at 400 for running text, jumping to 500 only for labels and metadata. Bold is reserved for the display face.

Type scale follows a modular ratio of roughly 1.25, with deliberate breaks at the largest sizes for impact. A single portfolio title might sit at 64px while body text holds at 16px — that 4:1 ratio creates dramatic hierarchy without needing color or weight changes.

Line lengths are kept short: 55–65 characters for body text, fewer for captions and descriptions. This matches the pace of the content — thoughtful, unhurried. Wide measure would make the text feel like documentation; short measure makes it feel like a conversation.

---

# Layout

The grid is a 12-column system that rarely uses all twelve columns. Full-width layouts feel overwhelming for this content; most pages settle into 8-column centered compositions with generous side margins. This creates a natural frame around the work, like a mat board in a gallery.

Vertical rhythm follows the spacing scale strictly. Sections breathe at 96px (spacing-24), subsections at 48px (spacing-12), and internal groups at 16px (spacing-4). This creates a clear cadence as the reader scrolls — dense clusters of information separated by deliberate pauses.

The portfolio detail page is the primary layout concern. A large hero image (aspect ratio 4:3 or 3:2, never cropped to square) sits above a two-column zone: material and process notes on the left, specifications and dimensions on the right. Below, a horizontal strip of detail images scrolls independently. This structure serves the maker's narrative — see the whole, understand the thinking, examine the craft.

Mobile layouts do not simply compress the desktop. They reorganize priority: image first, title second, key dimensions third, full description last. The horizontal detail strip becomes a vertical stack. Touch targets on navigation sit at 48px minimum, reflecting the physicality of the work itself — everything here is meant to be grasped.

---

# Elevation & Depth

Depth in this system is subtle to the point of near-invisibility. The highest elevation level (level 3) uses only 8px of offset shadow with 12% opacity — enough to lift a card off the surface without creating a floating effect that would feel disconnected from the material world of the furniture.

Level 1 elevation is a 2px shadow at 6% opacity, used for interactive elements like buttons on hover. Level 2 is 4px at 8% opacity for cards and modals. Level 3 is reserved for overlays and the mobile navigation drawer. No combination exceeds a visual impression of "this object is resting on a surface" rather than "this object is hovering."

Background shifts handle most depth perception. A secondary background on a section break creates more spatial separation than any shadow. This approach mirrors actual architectural space — you perceive depth through material changes (floor to wall to ceiling) more than through shadows alone.

Dark mode inverts the shadow logic slightly: shadows become darker-warm rather than lighter-warm, blending into already-dark surfaces. The effect should feel like a subtle recess rather than a projection, as though elements are pressed gently into a dark wood surface rather than floating above it.

---

# Shapes

Border radii stay small across the system: 2px for buttons and inputs, 4px for cards and containers, 6px for larger panels. The furniture itself features tight, precise joinery — the interface should reflect that precision. Rounded corners beyond 8px would feel soft in a way that contradicts the crisp edges of well-made cabinetry.

Image containers use no border radius at all. Photography is presented as-is, with the natural edges of the composition respected. This borrows from gallery conventions where the print or frame defines the boundary, not the wall. Padding and spacing create separation; the rectangle of the image stays true.

Interactive elements (buttons, links, form controls) receive the smallest radius and a consistent 1px border in the subtle border color. On hover, the border shifts to primary-500. This creates a clear interactive state without relying on background fills that might compete with nearby photography.

The overall shape language is orthogonal. Circular elements appear only for the maker's portrait and small status indicators. Everything else — cards, images, buttons, navigation items — occupies a rectangle. This restraint lets the organic curves present in the furniture photography stand out by contrast.

---

# Components

The **gallery grid** is the core component. It renders a masonry-style layout of portfolio pieces, each card consisting of an uncropped image, a title in Instrument Serif at 20px, and a one-line material summary in Inter at 13px. Cards carry no visible border until hover, when a subtle primary-500 bottom edge appears. Click targets cover the entire card area.

The **detail hero** fills the viewport width with a single image at 80vh height on desktop, 60vh on mobile. Below it, a narrow metadata bar displays the piece name, year, and primary material in Inter 500 at 12px, uppercase, tracked at 0.08em. This bar fixes to the top of the viewport on scroll, providing persistent context while the user examines detail images below.

The **process timeline** presents making-of photography in a horizontal scroll strip with snap points. Each image is accompanied by a brief caption in Inter 400 at 14px. Navigation dots appear below; arrow buttons are available but secondary. The scroll itself is the primary interaction — tactile and direct, like running your hand along a work surface.

The **contact section** is intentionally minimal: a single line of text in Instrument Serif at 24px ("Let's discuss a commission") followed by an email link and a studio address. No form, no calendar widget, no social media grid. The maker's process is slow and personal; the contact experience should match that pace.

---

# Do's and Don'ts

**Do** let the photography dominate. Every layout decision should ultimately serve the images. If a component or section doesn't enhance the viewer's understanding of a piece, remove it. White space is not emptiness — it is the gallery wall that gives the work context.

**Do** maintain warm tones consistently. Even neutral grays should carry a faint brown or yellow cast. Cool grays, blue-tinted blacks, and pure whites all pull the palette away from the material and toward something clinical. The screen should feel like a warmly lit studio, not an operating theater.

**Don't** use the primary palette decoratively. Primary-500 is for interactive emphasis, not for backgrounds or large fills. A page dominated by primary tones will feel branded and commercial rather than personal and crafted. The maker's work provides visual richness; the interface provides structure.

**Don't** animate beyond necessity. Transitions should be brief (150–200ms) and functional (opacity changes, subtle position shifts). Entrance animations, parallax scrolling, and decorative motion all distract from the static, physical reality of the furniture. The work does not move; neither should the page.

**Don't** mix type families. Instrument Serif for display, Inter for body — no exceptions. Bringing in a third face for accents, quotes, or Korean text breaks the clear hierarchy and introduces visual noise. If Hangul support is needed, Inter handles it well within the body role.