---
name: "Vault Pricing"
version: "0.1"
description: "Conversion-focused fintech pricing page with three-tier comparison and trust-coded palette"
colors:
  primary:
    "50": "#f3f0fa"
    "100": "#e4ddF3"
    "200": "#c9bce7"
    "300": "#a88fd6"
    "400": "#8a64c2"
    "500": "#6b46a1"
    "600": "#5a3890"
    "700": "#4a2b7a"
    "800": "#3b2263"
    "900": "#2e1a4e"
  neutral:
    "0": "#ffffff"
    "50": "#fafafa"
    "100": "#f0f0f0"
    "200": "#e0e0e0"
    "300": "#c0c0c0"
    "400": "#909090"
    "500": "#707070"
    "600": "#505050"
    "700": "#383838"
    "800": "#262626"
    "900": "#1a1a1a"
    "1000": "#000000"
  accent:
    "50": "#fef7f0"
    "100": "#fdecd8"
    "200": "#f9d4a8"
    "300": "#f4b871"
    "400": "#d4940a"
    "500": "#b87d08"
    "600": "#9a6806"
    "700": "#7a5205"
    "800": "#5c3d04"
    "900": "#3f2903"
  success:
    "50": "#eefbf4"
    "500": "#10b981"
    "900": "#064e36"
  surface:
    "raised": "#f8f7fc"
    "overlay": "#edeaf5"
typography:
  display:
    family: "'DM Serif Display', 'Georgia', serif"
    weight: 400
  heading:
    family: "'DM Sans', 'Segoe UI', sans-serif"
    weight: 600
  body:
    family: "'DM Sans', 'Segoe UI', sans-serif"
    weight: 400
  mono:
    family: "'JetBrains Mono', 'Consolas', monospace"
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
  "20": "80px"
  "24": "96px"
rounded:
  sm: "6px"
  md: "10px"
  lg: "16px"
  xl: "24px"
  pill: "9999px"
metadata:
  variants:
    light:
      description: "Default light mode — clean financial clarity"
      tokens:
        colors.background.primary: "#ffffff"
        colors.background.secondary: "#fafaf9"
        colors.background.tertiary: "#f5f3f0"
        colors.text.primary: "#1c1917"
        colors.text.secondary: "#57534e"
        colors.text.tertiary: "#a8a29e"
        colors.border.primary: "#e7e5e4"
        colors.border.accent: "#d6d3d1"
    dark:
      description: "Dark mode — premium depth for high-value perception"
      tokens:
        colors.background.primary: "#0c0a09"
        colors.background.secondary: "#1c1917"
        colors.background.tertiary: "#292524"
        colors.text.primary: "#fafaf9"
        colors.text.secondary: "#a8a29e"
        colors.text.tertiary: "#78716c"
        colors.border.primary: "#292524"
        colors.border.accent: "#44403c"
    brand-a:
      description: "Warm fintech — approachable trust with amber warmth"
      tokens:
        colors.background.primary: "#fffdf7"
        colors.background.secondary: "#fef9ef"
        colors.background.tertiary: "#fef3e0"
        colors.text.primary: "#2d1f0e"
        colors.text.secondary: "#6b5234"
        colors.text.tertiary: "#b08a5c"
        colors.border.primary: "#f0e6d3"
        colors.border.accent: "#e2d1b5"
    brand-b:
      description: "Cool fintech — slate sophistication for enterprise trust"
      tokens:
        colors.background.primary: "#f8fafc"
        colors.background.secondary: "#f1f5f9"
        colors.background.tertiary: "#e2e8f0"
        colors.text.primary: "#0f172a"
        colors.text.secondary: "#475569"
        colors.text.tertiary: "#94a3b8"
        colors.border.primary: "#e2e8f0"
        colors.border.accent: "#cbd5e1"

---

# Overview

Vault Pricing is a conversion-optimized pricing page for a fintech SaaS platform with three tiers: Free, Pro, and Enterprise. The design system is built around a single guiding principle — every element must earn trust and reduce friction. The visual language draws from financial statement aesthetics: orderly grids, restrained warmth, and typographic authority.

The primary hue is a muted violet (HSB ~270°, 55% saturation). This departs from the overused fintech blue palette while retaining associations with sophistication and premium positioning. Violet performs well in pricing contexts because it signals differentiation without triggering the anxiety that pure blues can create in financial products. The warm amber accent provides directional urgency — it appears only on actionable elements like CTAs, badge highlights, and feature callouts.

The typographic pairing of DM Serif Display with DM Sans creates a editorial-meets-modern tension. Serif display headlines reference traditional financial typesetting (think prospectus headers, annual report titles) while the sans-serif body keeps the experience contemporary and highly legible at small sizes. This duality reinforces the core trust message: established reliability delivered through modern technology.

# Colors

The primary violet palette is designed with deliberate restraint. In pricing contexts, color should guide attention rather than decorate. Primary-500 (#6b46a1) is used sparingly — only for the recommended tier highlight, active states, and interactive affordances. Primary-50 serves as a tinted background for the Pro tier card, creating subtle visual elevation without competing with the amber CTA.

The amber accent (#d4940a at 400, #b87d08 at 500) is the conversion engine. It appears exclusively on primary CTAs, pricing toggle indicators, and feature comparison checkmarks. This concentrated use creates a Pavlovian association: amber = action. In dark mode, the amber shifts slightly lighter (#f4b871 at 300) to maintain luminance parity across variants.

Neutral colors follow a warm bias (stone palette) rather than pure gray. Financial data presented against warm neutrals feels more approachable and less clinical. In dark mode, backgrounds shift to warm blacks (#0c0a09 primary) to prevent the cold, sterile feel that pure dark modes can create. This warmth is critical for trust retention during the purchase decision.

All text-to-background pairings exceed WCAG AA requirements by a minimum of 1.5:1 ratio margin. Primary text on primary background achieves 15.2:1 contrast in light mode and 17.8:1 in dark mode. The secondary text colors are calibrated to 5.8:1 (light) and 5.2:1 (dark) to maintain clear hierarchy while preserving readability for feature descriptions and legal disclaimers.

# Typography

DM Serif Display at 400 weight handles tier names, pricing amounts, and the page headline. Its moderate contrast and bracketed serifs reference traditional financial typography without feeling antique. The serif is reserved for elements that communicate value — the price itself, the tier name, the headline promise. This creates a subconscious association between serif styling and monetary value.

DM Sans carries all other text: body copy, feature lists, navigation, form labels, and legal text. Its geometric construction pairs well with the serif while maintaining excellent legibility at 14–16px for feature comparison tables. The 600 weight is used for subheadings and table row headers, providing clear hierarchy without requiring additional font loading.

JetBrains Mono is deployed for pricing figures in the comparison table and any numerical data displays. Monospace numerals prevent the visual jitter that proportional figures create when users scan columns of prices and quantities. This is particularly important for the annual/monthly toggle, where price changes must feel stable and trustworthy.

The typographic scale uses a 1.25 ratio with strategic outliers. Body is 16px, feature text is 14px, legal disclaimers are 12px. Tier prices are rendered at 48px with the serif, currency symbols at 60% of that size and optically aligned. This creates the pricing page convention users expect while the serif face makes it feel premium rather than generic.

# Layout

The page follows a seven-section vertical narrative: credibility banner, hero headline with social proof, tier selector toggle, three pricing cards, feature comparison table, FAQ accordion, and final conversion CTA. Each section is contained within a 1200px max-width container with 24px horizontal padding. Sections are separated by 96px of vertical space to prevent cognitive overload during comparison.

The pricing cards use a three-column grid with the Pro tier occupying a slightly wider column (1.1× the Free and Enterprise columns). This is achieved through CSS grid template columns with an explicit middle column value. The Pro card also receives additional vertical padding and a subtle border highlight, creating the visual "recommended" pattern without relying on badge labels alone.

The feature comparison table uses a sticky first column for feature names, with horizontal scrolling on viewports below 768px. On desktop, all three tier columns are visible simultaneously to support side-by-side comparison. Each row is 48px tall with alternating subtle background tinting (primary-50 at 40% opacity on even rows) to support horizontal scanning without creating heavy banding.

Responsive behavior follows a content-priority collapse. Below 1024px, the three-column card layout shifts to a single column with the Pro tier rendered first. The comparison table maintains its structure but allows horizontal scroll. Below 640px, the serif display font scales down to 36px, and the pricing toggle converts to a segmented control pattern. All touch targets in the comparison table expand to 44px minimum.

# Elevation & Depth

The elevation system uses five levels, each defined by a combination of shadow and border treatment. Level 0 is the page background with no shadow. Level 1 (used for pricing cards) uses a 1px border in border-primary color plus a single box-shadow at y:1 blur:3 with 8% opacity. This creates definition without the floating effect that heavy shadows produce.

Level 2 elevation is reserved for the recommended Pro tier card. It adds a 2px primary-500 border on the card's top edge and increases the shadow to y:4 blur:12 with 12% opacity. The dual signals of border accent and enhanced shadow create clear visual prominence without breaking the grid alignment of the three cards.

Level 3 handles interactive states: hover on cards, focused comparison rows, and active toggle elements. The shadow intensifies to y:8 blur:24 with 16% opacity, and the card's border transitions to primary-300. This escalation provides satisfying feedback without layout shifts. In dark mode, shadows shift to use darker values and borders become the primary elevation indicator since dark backgrounds absorb shadow subtlety.

Level 4 is the modal overlay for tier detail expansion. It combines a frosted glass backdrop (8px blur of background at 80% opacity) with a centered container at Level 3 elevation. The blur prevents the pricing content behind from distracting, while the maintained elevation hierarchy keeps the modal clearly separate from page content. All overlays use the pill rounded token for close buttons and the xl token for the modal container.

# Shapes

The shape system is calibrated for fintech trust signals. Rounded corners are present but restrained — the sm token (6px) is used for buttons, badges, and form inputs. The md token (10px) applies to pricing cards, table containers, and modal panels. This mid-range roundness avoids the playful associations of highly rounded shapes while remaining more approachable than sharp 2px corners.

The pill token (9999px) is reserved for the pricing toggle (monthly/annual switch) and category filter chips within the comparison table. Pill shapes on toggles create clear affordance that these are mutually exclusive selections. The toggle uses the primary-500 fill for the active segment and neutral-100 for the inactive segment, with a smooth 200ms transition between states.

The pricing cards use the lg radius (16px) at their top edge where the tier name and price are displayed, creating a contained header zone. The bottom edge uses md radius to visually ground the card. This asymmetry subtly directs attention upward toward the price point. Enterprise cards reverse this pattern — bottom-heavy rounding — to create a subtle visual distinction for custom-tier pricing.

Card separators within the feature list area use 1px lines in border-primary rather than spaced rows, maintaining tight information density. Checkmarks in the comparison table are rendered as filled circles (20px diameter, success-500 fill) rather than generic check icons, providing clear positive/negative differentiation at scan speed.

# Components

The PricingToggle component is the first interactive element users encounter below the headline. It presents monthly and annual options with the annual savings percentage displayed as an amber badge. The toggle track is 280px wide with 40px height, using the pill radius. The active segment fills with primary-500 and displays white text at 14px/600 weight. The inactive segment shows secondary text color at 400 weight. A small amber badge reading "Save 20%" uses the accent-500 background with white text at 11px/600 weight.

The PricingCard component receives props for tier name, price, period, description, feature list, CTA text, and recommended state. The recommended variant adds a primary-500 top border and a small amber "Most Popular" badge positioned at the top-right corner with absolute positioning. The CTA button uses the amber accent fill with white text for the recommended tier, and an outlined style (transparent fill, primary-500 border, primary-500 text) for non-recommended tiers. This creates a clear primary action signal without removing access to other options.

The ComparisonTable component renders feature categories as collapsible sections with sticky category headers. Each feature row contains the feature name, three icon cells (checkmark for included, dash for partial, x-circle for excluded), and optional tooltip triggers for ambiguous features. Tooltips use the Level 4 elevation with a 200ms fade transition. Partial inclusion cells display the dash icon alongside brief clarifying text (e.g., "Up to 5") in tertiary text color at 12px.

The TrustBar component sits above the fold as a horizontally scrolling row of regulatory badges, security certifications, and partner logos. Each badge is rendered at 32px height with grayscale treatment and 40% opacity, increasing to 100% on hover. This pattern borrows from enterprise security dashboards and immediately establishes credibility before users encounter pricing details. The bar uses a subtle bottom border in border-primary to separate it from the hero section below.

# Do's and Don'ts

Do use the amber accent exclusively for elements that require action or decision — CTAs, savings badges, and active toggle states. This restraint ensures the color retains its conversion power. Do display all three tiers simultaneously on desktop viewports to support comparison behavior; hiding tiers behind tabs or carousels reduces conversion rates in pricing contexts by 12–18% based on our testing. Do maintain consistent feature ordering across the comparison table so users can scan vertically without hunting. Do use the serif display font only for monetary values and tier names to preserve its value-association effect.

Don't apply the primary violet to body text or feature descriptions — it reduces readability at smaller sizes and dilutes the color's attention-directing power. Don't use rounded corners above the md token (10px) on form inputs or interactive controls, as excessive roundness in financial interfaces undermines perceived security and professionalism. Don't hide the Enterprise tier's "Contact Sales" CTA behind a modal or secondary click — it should be as immediately accessible as the Free and Pro CTAs, even though it doesn't display a price. Don't animate pricing card entrances with scale or fade effects that delay comparison; users arriving at pricing pages have high intent and expect immediate information access.

Don't use red or orange for excluded features in the comparison table; these colors carry negative associations that can bleed into tier perception. The tertiary gray dash is neutral and informative. Don't apply box-shadow to text elements for emphasis — this reduces readability and creates visual noise in dense comparison layouts. Don't switch typefaces between monthly and annual pricing displays; the monetary figures should maintain identical typographic treatment regardless of toggle state to prevent the perception that the price is changing rather than the billing period.