---
name: asset-standards
description: Default asset sourcing + AI-slop avoidance rules for every Aphrodite output. Lucide icons, Fluent emoji, real photography, opinionated copy.
version: 1.0.0
tags:
  - default
  - assets
  - quality-bar
related_skills: []
agent_created: false
default: true
---

# Asset Standards — applies to every output

This skill is loaded for every `aphrodite create` run. It encodes cross-cutting rules that prevent the output from looking like AI slop and ensure consistent high-quality asset sourcing.

## Icons — Lucide (https://lucide.dev/icons)

**Always use Lucide for icons.** Reasons:
- MIT-licensed, royalty-free
- 1,500+ pencil-perfect 24×24 strokes, consistent 2 px line weight
- Each icon ships SVG + React/Vue/Svelte components, easy to inline
- Visual register is *neutral-modern* — pairs with any typographic direction without fighting

When emitting icon markup, prefer inline SVG (Lucide's raw SVG endpoint) for static pages:

```html
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-arrow-right" aria-hidden="true"><path d="M5 12h14"/><path d="m12 5 7 7-7 7"/></svg>
```

Or via CDN script if interactive:
```html
<script src="https://unpkg.com/lucide@latest"></script>
<i data-lucide="arrow-right" class="size-5"></i>
<script>lucide.createIcons();</script>
```

**Icon-pick discipline.** Pick the *specific* icon, not the generic. For a furniture craftsperson portfolio: `hammer`, `ruler`, `tree-deciduous`, `hand-metal` (for handcraft); avoid `sparkles`, `star`, `zap`, `wand-2` — those are AI-slop hallmarks. For a longevity research site: `flask-conical`, `microscope`, `dna`, `activity`; avoid `heart-pulse` cliché.

## Emoji — Fluent UI Emoji (https://fluentemoji.com)

When a UI surface calls for emoji (status pills, casual chips, copy), prefer **Microsoft Fluent UI Emoji** (free, MIT). Two style options inside the set:
- `Flat` for restrained design systems
- `3D` for playful surfaces

Avoid:
- The native OS emoji font in marketing pages — it renders inconsistently across Windows/Mac/Linux and breaks the brand
- Apple-style emoji in copyrighted public contexts
- Overuse — at most one emoji per heading, none in body copy

Embed Fluent emoji as inline SVG or PNG from the GitHub release:
```
https://github.com/microsoft/fluentui-emoji/raw/main/assets/{Name}/Flat/{name}_flat.svg
```

## Photography

Aphrodite v0.3 doesn't generate or fetch images. When the composition needs photographic content, leave **opinionated placeholders** that signal intent — never use Lorem-style image generators or AI-generated stock.

For *real* sourcing later, prefer in this order:
1. **The maker's own photography** — explicitly request it; tell the user "drop image-1.jpg through image-N.jpg here, named accordingly"
2. **Unsplash** (https://unsplash.com) — credit the photographer; pick photographers, not search results
3. **Pexels** as fallback

**Avoid:**
- AI-generated photography (looks AI even at high quality — uncanny hands, glassy skin, identical-eye syndrome)
- "Diverse stock-photo team in a meeting" cliché
- The Unsplash homepage's top 30 — overused

For placeholders during the design phase, use a neutral aspect-ratio block in primary-100 tinted with primary-300 stripes, never a gradient. Example:
```html
<figure class="aspect-[4/5] bg-[var(--colors-primary-100)] flex items-center justify-center text-sm text-[var(--colors-primary-700)]">
  [photo: Hakwi Dining Table — black walnut, oil finish, 2024]
</figure>
```
The caption text *describes the photo* rather than saying "image placeholder" — that doubles as art-direction-brief for the user/photographer.

## Illustrations & decoration

- **undraw.co** for editorial illustrations — free, recolorable to brand palette via a single hex picker. Filter to flat-vector sets, not the 3D-isometric ones.
- **heropatterns.com** for SVG background patterns — tintable, lightweight.
- **haikei.app** for mesh-gradient blobs and wave dividers when (and only when) the brand register tolerates them. Restrained portfolios: skip entirely.
- **NEVER**: Memphis-style geometric scatters (1980s revival cliché), purple-to-pink gradients, "abstract neural network" hero backgrounds.

## Avatars

- **dicebear.com** — deterministic, brand-color-tintable, suitable for review/testimonial UI. Pick a *consistent style across the page* (e.g., `personas` or `lorelei`, not mixed).
- Avoid: pravatar (uses real faces under unclear license), randomuser.me (same issue), AI face generators.

## Copy hygiene — anti AI-slop

These appear constantly in LLM output. Aggressively replace.

| AI-slop pattern | Replacement |
|---|---|
| "AI-powered" / "intelligent" / "smart" as a feature noun | the actual concrete behavior |
| "seamless" / "robust" / "cutting-edge" / "next-generation" | strip; show, don't tell |
| "Get started today!" CTA with exclamation | a verb that names what happens (`Open in workshop` / `Read the journal`) |
| "Built for {persona} like you" | the user's actual job in their language |
| "Premium" / "luxury" / "elegant" as bare adjective | a noun phrase that names the material register |
| Vanity stats with round-number-X ("10,000+ users") | real numbers in real ranges ("87 commissions since 2018") |
| Three-bullet feature comparisons each ending in a checkmark | full sentences, prose pacing |
| Generic placeholder names ("John Doe") | culturally-rooted names matching the project's domain |
| "Trusted by industry leaders" trust badges | specific named clients (with the user's permission) |
| Stat row "100+ years experience / 50+ countries / 24/7 support" | omit or replace with one concrete fact |

## Real numbers, real places, real names

The Pass 7 generator showed strong signal here — picking Seoul district names (Jongno, Samcheong, Insadong, Gwangjang) as project names for a Seoul craftsperson. **Continue this discipline:**
- Names of people: pull from the actual cultural context, not "default-American."
- Place names: real neighborhoods/streets in the city the intent specifies.
- Year ranges: real ones (2018–2024), not "X+ years."
- Material spec on a furniture site: actual species names (black walnut, white oak, Korean red pine — not "premium wood").

## When in doubt

Restrain. The Pass 7 / Pass 9 trajectory consistently picked the **less-decorated, more-considered** option and it consistently read as higher quality. Default to:
- 1 hero, 1 portfolio grid, 1 plain-text contact — not a marketing-funnel multi-section page.
- Single accent color usage — not a 3-color "system."
- One emoji per page max — not in body copy.
- Inline icons over icon fonts.
- No gradient unless the brand register explicitly demands it.
