---
name: dev-tool-saas-landing
description: B2B developer-tool SaaS marketing surfaces — Linear / Vercel / Stripe register. Dark-default, near-black base, single accent, real product screenshots not glass-mock.
version: 1.0.0
tags:
  - saas
  - dev-tools
  - landing
  - product-marketing
  - dark-mode-default
related_skills:
  - asset-standards
agent_created: false
default: false
---

# Dev-tool SaaS landing — Linear / Vercel / Stripe register

## When this skill applies

Intent mentions: dev-tool, developer-facing SaaS, B2B infrastructure, API platform, CLI tool, build platform, observability, IDE-adjacent product. The audience runs IDEs in dark mode by default; meet them where they live.

## Workflow

1. **Dark-default.** The canonical variant is dark. light is the secondary. `colors.background.primary` for dark sits at `#08090a` to `#0a0a0b` territory. Never softer than `#0e0e10`.

2. **Cold accent palette.** ONE accent hue, cold (violet / blue / mint), saturated. Examples that work:
   - cold-violet `#5e6ad2` (Linear lineage)
   - emerald `#00d56c` (Vercel green, when not orange)
   - cobalt-blue `#3a7afe` (Stripe-adjacent)
   No second accent. The accent is used for primary CTA + one signal-color highlight.

3. **Typography — modern grotesque, Inter-class.** Inter Display + Inter is the safe-and-correct pair. Söhne is paid; Geist (Vercel) is Vercel's. Don't pick proprietary fonts without a free fallback. Body sizes 14-16 px; display sizes 56-96 px.

4. **Product screenshots, not glassmorphic mocks.** The hero shows real product UI: terminal output, build logs, dashboard tiles with real labels and numbers. Inline SVG can simulate this (the LLM should reach for inline rectangular shapes with `--colors-neutral-700` fills + bordered tiles), NEVER frosted-glass blur with orbiting cards.

5. **Code blocks are content.** A dev-tool landing should have AT LEAST one `<pre><code>` block showing the CLI command / API request / config file the user actually types. Mono 13-14 px, line-height 1.55, scrollable inline if long.

6. **Section pattern (in order):**
   - Eyebrow tag (small caps 11 px in accent) → 56-96 px headline → 18-20 px subhead → primary + ghost CTA stack → product visual / code block
   - Three-feature row, each feature in a 1/3 column with a 32-40 px Lucide icon (not stock-bullet checkmarks)
   - Code example or larger product screenshot
   - Pricing tiers (if the intent calls for it) — 3 cards, hover-distinct, no "most popular" badge clutter
   - Single final CTA at section padding 160-240 px

7. **Anti-AI-slop copy register.** Direct verbs, concrete tools, no superlatives.
   - "Track issues. Build software faster." ✓
   - "Streamline your team's productivity with our innovative platform." ✗
   - "Deploy a Next.js app in seconds." ✓
   - "Reimagine your deployment workflow." ✗

## Do / Don't

- DO emit at least one `<pre><code>` block with real terminal / config text.
- DO use Lucide icons (32-40 px size) for feature rows — `terminal`, `git-branch`, `database`, `cloud`, `lock`, `chart-line`.
- DO show three pricing tiers as columns with feature lists (not toggles).
- DO use a single hairline divider in `rgba(255,255,255,0.08)` instead of card-stacks.
- DON'T use gradient meshes in the hero (it's Vercel's specific brand artifact; copying it reads as homage).
- DON'T use frosted-glass / glassmorphism. It's a 2021 cliché.
- DON'T use stock photography of "diverse team in a meeting."
- DON'T use rounded corners > 8 px on buttons or cards.
- DON'T put more than 2 CTAs above the fold.

## Reference fragments worth lifting

- Body family: Inter Regular / Inter Medium
- Display family: Inter Display Semi-Bold or Bold
- Hero structure: eyebrow 11 px → headline `clamp(40px, 6vw, 84px)` → subhead 18-20 px → CTA stack → product visual at 60% viewport
- Section spacing: 160-240 px between major sections
- Pricing tier card: 320-400 px wide, 32 px padding, 1 px border in `rgba(255,255,255,0.10)`, hover state shifts the border to accent
- Footer: 2-3 columns, 13 px regular, 11 px caps section headers, single accent link colour

## Cross-references

- Wiki: `linear-app`, `vercel`, `stripe` (user-added) for concrete examples
- Persona: avoid pairing with `paul-rand` or `ettore-sottsass` (their registers conflict with dev-tool restraint). Pair with `dieter-rams` for maximum restraint, no persona for the genre-default.
