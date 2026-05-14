---
name: " walnut-seoul"
version: "0.1"
description: "Portfolio for an independent furniture maker working in solid walnut, Seoul"
colors:
  primary:
    "50": "#f6f5f1"
    "100": "#e7e5db"
    "200": "#cfccc0"
    "300": "#b3b0a3"
    "400": "#9a9686"
    "500": "#6d6759"
    "600": "#524e43"
    "700": "#3a3832"
    "800": "#252420"
    "900": "#141310"
  neutral:
    "0": "#ffffff"
    "50": "#fafaf9"
    "100": "#f0eeea"
    "200": "#e0ddd6"
    "300": "#c7c3ba"
    "400": "#a9a49b"
    "500": "#8c867c"
    "600": "#6e6960"
    "700": "#524e46"
    "800": "#2e2b26"
    "900": "#1a1816"
    "1000": "#000000"
  accent:
    "500": "#b45830"
    "600": "#964825"
typography:
  display:
    family: "Instrument Serif"
    weight: 400
  body:
    family: "Inter"
    weight: 300
  mono:
    family: "JetBrains Mono"
    weight: 400
spacing:
  "0": "0"
  "0.5": "2px"
  "1": "4px"
  "2": "8px"
  "3": "12px"
  "4": "16px"
  "6": "24px"
  "8": "32px"
  "10": "40px"
  "12": "48px"
  "16": "64px"
  "20": "80px"
  "24": "96px"
  "32": "128px"
  "40": "160px"
  "48": "192px"
  "56": "224px"
rounded:
  none: "0"
  xs: "2px"
  sm: "4px"
  md: "8px"
metadata:
  variants:
    light:
      description: "Default light mode — bright studio atmosphere"
      tokens:
        colors.background.primary: "#fafaf9"
        colors.background.secondary: "#f0eeea"
        colors.text.primary: "#1a1816"
        colors.text.secondary: "#6e6960"
        colors.border.primary: "#e0ddd6"
        colors.border.secondary: "#c7c3ba"
    dark:
      description: "Dark mode — workshop after hours"
      tokens:
        colors.background.primary: "#13110f"
        colors.background.secondary: "#1e1c19"
        colors.text.primary: "#ede9e1"
        colors.text.secondary: "#9e998e"
        colors.border.primary: "#33302a"
        colors.border.secondary: "#47443d"
    brand-a:
      description: "Warm gallery — linen white walls"
      tokens:
        colors.background.primary: "#f6f5f1"
        colors.background.secondary: "#e7e5db"
        colors.text.primary: "#252420"
        colors.text.secondary: "#7a7568"
        colors.border.primary: "#d6d4ca"
        colors.border.secondary: "#c0bdb2"
    brand-b:
      description: "Studio charcoal — pigmented dark studio walls"
      tokens:
        colors.background.primary: "#1c1b17"
        colors.background.secondary: "#272521"
        colors.text.primary: "#e6e3db"
        colors.text.secondary: "#a5a196"
        colors.border.primary: "#38352e"
        colors.border.secondary: "#4a463e"

# Overview

This is a portfolio for a single furniture maker whose material is solid walnut and whose city is Seoul. The design borrows magazine pacing — each piece of furniture gets a full visual spread, not a thumbnail in a grid — but it is not a magazine. There are no bylines, no issue dates, no editorial scaffolding. The maker's name is implicit. The work is the subject.

The spatial logic comes from the workshop itself: generous clearance between stations, everything on the grid of the bench, sharp joinery where surfaces meet. Digital corners are 2–4 px, never pillowed. Spacing between sections maxes at 224 px because furniture needs room to breathe on screen the same way it needs room in a gallery.

The tonal register is quiet competence. Warm brown-greys reference walnut heartwood without literally matching it. A single terracotta accent (the colour of the maker's favourite Japanese pull-saw handle, if we're being specific) appears only on hover states and the contact section. The site should feel like walking into a well-lit workshop: you see the material first, the person second, the tools barely at all.

# Colors

The primary palette is built on a warm grey-brown axis that echoes the tonal range of black walnut timber — pale sapwood at the 50 level, heartwood mid-tones at 400–500, near-black end-grain at 900. This is not a direct match to any specific walnut board; it's a compression of the material's range into a usable interface scale.

Terracotta (#b45830) is the sole accent. It appears on interactive hover states, the contact section's underline, and the "available works" indicator. It does not tag, badge, or label anything. It is the colour of one small tool, not a brand identity system.

The neutral scale runs slightly warmer than pure grey — each step has 2–4 points more red channel than blue. This prevents the background from feeling clinical without tipping into parchment territory. The dark mode variants push toward warm charcoal (#13110f, #1c1b17) rather than cold blue-black, maintaining the workshop-at-night feeling across all dark themes.

# Typography

Instrument Serif at display size (48–72 px, optical weight 400) carries the maker's name and piece titles. It's a contemporary serif with sharp terminals and a slight forward lean — not precious, not antiquarian. It suggests craft without cosplay. Weight stays at regular; bold is never needed because the size differential does the work.

Inter Light (300) handles all body text at 15–16 px. The light weight keeps the sans receding behind the photography, which is where it should live. Navigation, captions, and metadata use Inter Regular (400) at 12–13 px for clear scannability without competing with display type.

JetBrains Mono appears only in dimension annotations — the "W 1800 × D 900 × H 720 mm" labels beneath each piece. Monospace at 12 px in the secondary text colour reads as technical specification, not decoration. It signals precision without calling attention to itself.

# Layout

The grid is a 12-column system with 24 px gutters on desktop, collapsing to 6 columns at tablet and a single column on mobile. The content column occupies 7 of 12 columns (roughly 58% of viewport width at 1440 px), leaving the remaining space as intentional margin. Photography sits at 4:5 portrait ratio for individual pieces and 3:2 landscape for workshop and process imagery.

Project pages follow a strict vertical cadence: full-bleed hero image (100 vw) → 224 px breathing space → piece title in Instrument Serif 48 px → 32 px gap → one paragraph of material and process notes in Inter Light 16 px → 128 px gap → image sequence at 7-column width → 192 px gap → next piece. No sidebars, no related-work carousels, no floating CTAs.

The navigation is a fixed left rail on desktop (studio name, three links, contact) at 12 px Inter Regular, vertically stacked. On mobile it collapses to a minimal top bar. The contact section is plain text: studio address in Seoul, phone number, email. No form widget. This is a person you call, not a brand you submit tickets to.

# Elevation and Depth

Elevation is nearly absent by intent. The background is flat; cards and containers are flush with the surface. The only depth cue is a 1 px border in the primary border colour, used sparingly to separate content sections. This flatness mirrors the physical vocabulary of the work: walnut planes meeting at sharp joints, not layered or stacked.

Photography provides all the dimensionality the page needs. Images are not given drop shadows, border treatments, or rotated frames. They sit flush in the content column with 24 px of space between them and the text rail. The material's own grain, shadow, and figure carry the visual weight.

Interactive elements use colour shift and opacity change rather than elevation change. A hovered link moves from text-secondary to text-primary and gains a 1 px underline offset by 2 px. Buttons are underlined text, never raised rectangles. The only box-shadow on the entire site is a subtle 0 1 px 2 px on the mobile navigation overlay, and even that is debatable.

# Shapes

Corners are sharp or nearly sharp: 2 px for images and cards, 4 px for inputs and small interactive elements, 0 px for full-width containers and section dividers. This is joinery geometry. Rounded corners suggest softness and approachability; sharp corners suggest precision and intention. A furniture maker's portfolio should signal the latter.

Image aspect ratios are prescribed: 4:5 portrait for individual furniture pieces (a dining table shot from the end, a chair in three-quarter profile), 3:2 landscape for workshop and process documentation. These ratios are enforced at the component level and do not adapt to the uploaded image's native proportions. Cropping is intentional — the maker selects the frame, not the CMS.

The overall page shape is a tall, narrow column of content in generous whitespace. There are no horizontal scrolling sections, no masonry layouts, no asymmetric overlaps. The vertical scroll is the pacing mechanism: each piece arrives in sequence, like turning pages in a monograph.

# Components

The piece card is the primary component: a 4:5 image above a narrow text block containing the piece name (Instrument Serif 20 px), material annotation (Inter Light 14 px), and dimensions (JetBrains Mono 12 px). Cards are arranged in a 2-column grid within the 7-column content area, with 24 px gaps. No card has a border, background, or hover animation beyond a subtle opacity shift on the image.

The hero section is full-viewport-width photography with no overlay, no text, no navigation. The image fills the screen and the user scrolls past it. Below the hero, a minimal text block identifies the maker: studio name in Instrument Serif 64 px, city in Inter Light 14 px, a single line of descriptive text. No tagline, no mission statement.

The contact section is a terminal block at the bottom of the page: studio address (Seoul neighborhood, not coordinates), phone number with country code, email address as a mailto link. The Lucide icons for mail, phone, and map-pin sit beside each line at 16 px in the secondary text colour. No social media links unless the maker specifically requests them. The section ends with the maker's name and the current year, set in Inter Light 12 px.

Navigation uses the Lucide hammer icon (16 px) beside the studio name in the left rail. The three navigation items — Work, About, Contact — are plain text links. About is a single photograph of the maker in the workshop (3:2, flush left) with a 200-word biographical paragraph beside it. No timeline, no philosophy statement, no client logo wall.

# Do's and Don'ts

Do let the photography dominate every surface. If a page section doesn't have an image, question whether it needs to exist. The maker's skill lives in the grain, the joint, the finish — the site's job is to present those things at scale with minimal interference.

Do use generous vertical spacing between sections. The 224 px maximum spacing token exists because furniture needs visual clearance. Cramming pieces together erodes the perception of value. Each work should feel like it was placed with intention, not tiled by an algorithm.

Don't add SaaS-style social proof: no client count, no years-of-experience badge, no testimonial carousel. The work is the credential. A dining table photographed in a Seoul apartment is worth more than "featured in 12 publications."

Don't use form widgets for contact. A furniture commission begins with a conversation. Present the phone number and email plainly. The kind of client who commissions handmade walnut furniture wants to talk to the maker, not fill out a lead-capture form.

Don't mix typographic voices. Instrument Serif for display, Inter for body, JetBrains Mono for dimensions. Three families, each with one job. Adding a fourth typeface — a script for quotes, a condensed sans for labels — dilutes the system's clarity without adding information.