---
title: Vercel (vercel.com)
url: https://vercel.com
tags:
  - saas
  - dev-tools
  - landing
  - product-marketing
  - dark-mode-default
  - infrastructure
  - quality-bar
signature: Geist-typeface system; dark default; mesh-gradient hero accepted because it's their actual brand artefact; product screenshots that read as IDEs not glassmorphic mockups
ingested_at: 2026-05-15
---

# Vercel — why it matters

Vercel's marketing site is the more polished cousin of Linear's — same dark-default register but at the *infrastructure* level (deploy URLs, build outputs, framework comparisons). It taught the 2024-2026 cohort of dev-tool marketing pages how to ship a mesh-gradient hero *without* it looking like AI slop.

## What to absorb

- **Geist (their proprietary type family).** When you don't have access to Geist, the closest free pair is Inter Display + Inter — sharp grotesque + the same letterforms at body size. Geist's specimen is *narrower* than Inter, so display sizes can hit 80-120 px without crowding.
- **Dark base** with a near-black background `#000` and a single near-white text `#ededed`. No off-black, no near-charcoal compromise.
- **Product screenshots as IDEs.** Vercel's heroes show actual terminal output, deploy logs, dashboard data. Not abstract orbiting glass cards. Lift the *register* (real screenshots, monospace text inside frame, scrolling code) not the specific brand chrome.
- **Mesh gradients earn their place** because Vercel ships them in the actual product (Geist UI's gradient backgrounds). Don't lift gradients for a brand that doesn't ship them downstream.
- **Hero pattern: 1-word eyebrow → 80-120 px headline → 18 px subhead → dual CTA (primary + ghost) → product visual.** Same as Linear, but the visual is busier (real data) rather than calmer (Linear's restrained screenshots).

## What NOT to copy

- The gradient mesh is *Vercel's* visual signature. Copying it produces "trying to look like Vercel" — the worst kind of dev-tool homage.
- The black is the *darkest possible* black (#000). Many sites soften this to #08090a or #0e0e10. Vercel's hard black only works because their typography is *also* maximally crisp. If your type isn't Geist-crisp, soften the black.
- "Deploy" / "Ship" / "Iterate faster" copy register is theirs. Find your own verb stack.

## Reference fragments worth lifting

- Hero stack vertical spacing: 16 / 24 / 32 / 80 (between major blocks)
- Body sizes: 14 / 16 (default 16 in 2026)
- Headline weight: 600 (semi-bold, not full bold)
- Section divider: single 1px line in `rgba(255,255,255,0.08)`, not a coloured bar
- Code block: ZWS (Zero Wakizashi Sans — Vercel's monospace) or JetBrains Mono at 13 px, line-height 1.55
