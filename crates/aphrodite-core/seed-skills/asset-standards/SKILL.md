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

When emitting icon markup, prefer **inline SVG with the exact Lucide path data** for static pages. The class name `lucide lucide-<name>` is the marker that lets downstream tools recognise the icon. Each SVG uses the canonical Lucide attributes: `xmlns="http://www.w3.org/2000/svg"` `width="24"` `height="24"` `viewBox="0 0 24 24"` `fill="none"` `stroke="currentColor"` `stroke-width="2"` `stroke-linecap="round"` `stroke-linejoin="round"` `aria-hidden="true"`.

**Copy-paste these 10 canonical Lucide SVGs verbatim** (these are the exact path strings from lucide.dev/icons as of 2026-05). PRESERVE the `class="lucide lucide-<name>"` attribute exactly — that class is the marker downstream tools use to detect Lucide-sourced icons. Stripping the class while keeping the path is a partial-credit failure mode; the class must travel with the path.

```html
<!-- arrow-right -->
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-arrow-right" aria-hidden="true"><path d="M5 12h14"/><path d="m12 5 7 7-7 7"/></svg>

<!-- arrow-up-right (for external / open-in-new patterns) -->
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-arrow-up-right" aria-hidden="true"><path d="M7 7h10v10"/><path d="M7 17 17 7"/></svg>

<!-- mail (contact rail) -->
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-mail" aria-hidden="true"><rect width="20" height="16" x="2" y="4" rx="2"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/></svg>

<!-- phone -->
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-phone" aria-hidden="true"><path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"/></svg>

<!-- map-pin (workshop / studio location) -->
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-map-pin" aria-hidden="true"><path d="M20 10c0 4.993-5.539 10.193-7.399 11.799a1 1 0 0 1-1.202 0C9.539 20.193 4 14.993 4 10a8 8 0 0 1 16 0"/><circle cx="12" cy="10" r="3"/></svg>

<!-- hammer (craft / making) -->
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-hammer" aria-hidden="true"><path d="m15 12-8.373 8.373a1 1 0 1 1-3-3L12 9"/><path d="m18 15 4-4"/><path d="m21.5 11.5-1.914-1.914A2 2 0 0 1 19 8.172V7l-2.26-2.26a6 6 0 0 0-4.202-1.756L9 2.96l.92.82A6.18 6.18 0 0 1 12 8.4V10l2 2h1.172a2 2 0 0 1 1.414.586L18.5 14.5"/></svg>

<!-- ruler (precision / measurement) -->
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-ruler" aria-hidden="true"><path d="M21.3 8.7 8.7 21.3a2.41 2.41 0 0 1-3.4 0l-2.6-2.6a2.41 2.41 0 0 1 0-3.4L15.3 2.7a2.41 2.41 0 0 1 3.4 0l2.6 2.6a2.41 2.41 0 0 1 0 3.4Z"/><path d="m7.5 10.5 2 2"/><path d="m10.5 7.5 2 2"/><path d="m13.5 4.5 2 2"/><path d="m4.5 13.5 2 2"/></svg>

<!-- flask-conical (research / clinical) -->
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-flask-conical" aria-hidden="true"><path d="M14 2v6a2 2 0 0 0 .245.96l5.51 10.08A2 2 0 0 1 18 22H6a2 2 0 0 1-1.755-2.96l5.51-10.08A2 2 0 0 0 10 8V2"/><path d="M6.453 15h11.094"/><path d="M8.5 2h7"/></svg>

<!-- chart-line (dashboard / analytics) -->
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-chart-line" aria-hidden="true"><path d="M3 3v16a2 2 0 0 0 2 2h16"/><path d="m19 9-5 5-4-4-3 3"/></svg>

<!-- chevron-right (disclosure / next) -->
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-chevron-right" aria-hidden="true"><path d="m9 18 6-6-6-6"/></svg>
```

When you need a Lucide icon not in this list (there are 1,500+), generate the SVG with the same canonical attributes and add a comment `<!-- lucide: <icon-name> path data from lucide.dev/icons/<icon-name> -->` so downstream verification can spot a hand-drawn approximation vs the real Lucide.

Or via CDN script if the page is interactive:
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
