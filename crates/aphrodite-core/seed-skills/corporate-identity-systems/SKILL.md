---
name: corporate-identity-systems
description: Corporate identity surfaces — what both Paul Rand and Massimo Vignelli would agree on. 2-3 color palette, geometric type, single dominant mark, no decoration.
version: 1.0.0
tags:
  - identity-system
  - corporate-identity
  - consultancy
  - landing
  - studio
related_skills:
  - asset-standards
agent_created: false
default: false
---

# Corporate identity systems — the Rand-Vignelli consensus

## When this skill applies

Intent mentions: corporate identity, brand presentation, consultancy site, design-studio identity, B2B marketing for a single product or service that needs *authority* register. NOT for editorial / photo-dominant portfolios (use `editorial-portfolio` instead) or SaaS dashboards (use `dev-tool-saas-landing` when shipped).

## Workflow

1. **Palette discipline.** TWO saturated colours maximum, plus a near-black and a paper-off-white. No third hue. Examples that work:
   - cardinal red + near-black (IBM, Coca-Cola lineage)
   - signal yellow + ink-blue (Vignelli's NYC subway map)
   - viridian + cream (Burberry, Aesop adjacent)
   - cobalt + bone (NeXT, contemporary Mark Boulton)
   Both Rand and Vignelli would refuse a 3-color system without an explicit structural reason.

2. **Type — geometric or transitional, regular and medium weights only.** Never bold display by default. Never serif display unless the brand register is editorial-adjacent (NYT-style). Working pairs:
   - **Geometric grotesque**: Inter Display + Inter, Söhne, Helvetica Now Display + Helvetica Now Text, Druk Wide (for headlines only)
   - **Transitional serif** (when the brief demands gravitas): Times New Roman, Tiempos, ITC Garamond
   - **Avoid**: Renaissance revival serifs (EB Garamond, Cormorant); proprietary commercial families without a free fallback explicitly named (Graphik, Lyon, Söhne when not licensed)

3. **One dominant graphic mark per page.** A wordmark, a geometric shape, a single off-grid element. Never two competing graphic marks above the fold.

4. **Layout — strict modular grid.** 12-column at 24-32 px gutters, content column 7-8 of 12 (58-67% of viewport). Side margins ≥ 15% of viewport width on desktop. Vignelli's grids carry from a Knoll catalog to a transit map — the system is the *only* visual constant.

5. **Type scale — modular, derivable.** Pick a base size (16 px body) and a ratio (1.25 or 1.333). 16 × 1.25 = 20. 20 × 1.25 = 25. 25 × 1.25 = 31. Pick four sizes from this sequence; *use only those four*. Sizes-by-feel are anti-pattern.

6. **No SaaS-funnel chrome.** No testimonials carousel. No "loved by leading teams" social-proof strip with logos. No countdown timer. No "free trial" gradient. A corporate identity site does not *sell* — it *presents*.

7. **Footer is a colophon.** Company name, address, contact, copyright. In small caps, 11 px, centered or left-aligned on a single line if possible. Not a social-icons + 8-column-link-list footer.

## Do / Don't

- DO use a single hero word or two-word headline at 96–144 px.
- DO place a *single* graphic element (a square, a circle, a slash) off-centre in the hero.
- DO use the modular grid visibly — the user should *feel* the alignment without seeing the gridlines.
- DON'T use gradients of any kind in the brand palette.
- DON'T use drop shadows simulating depth without functional purpose.
- DON'T use rounded corners > 4 px on functional elements.
- DON'T put more than three CTAs on a page.
- DON'T use emoji in copy — corporate identity copy is *adjective-light, verb-direct*.

## Reference fragments worth lifting

- Hero structure: single word/phrase headline 96-144 px → 18 px lede sentence below → single primary CTA → single graphic element off-axis right-of-headline
- Section padding: 160 px vertical, 80-120 px horizontal
- Body sizes: 16-18 px for primary text; 13-14 px for labels and captions
- Footer: 11 px caps with letterspacing 0.05em; one line, not three

## Anti-AI-slop discipline

The corporate-identity register attracts AI-slop the hardest because every word-cell asks for "premium / sophisticated / innovative". Aggressively replace:
- "Premium consulting" → "Brand strategy and identity design since 2018"
- "Innovative solutions" → the actual deliverable ("logo systems, signage, brand books")
- "Trusted by leading brands" → specific named clients with their permission
- "Get started" → the verb that names what happens next ("Schedule an intro call", "Read our manifesto")

## Notes from Pass 24 (2026-05-15)

Initial Paul Rand × consultancy dogfood picked Graphik (a proprietary commercial face from Commercial Type, no free CDN). Critic correctly flagged: the HTML will fall back to Helvetica Neue. This skill encodes the rule: **always pick a free-CDN-loadable family for the body family**, with display optionally proprietary if a free fallback is named. Saves a critic refine round trip.
