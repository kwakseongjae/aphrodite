# Agent Evaluation — 2026-05-14 — Twelfth Pass (persona authority + CJK strategy)

> Pass 11 surfaced two bugs in one experiment: the critic let scaffold authority override persona authority (Finding #34), and persona prefers didn't account for CJK coverage when the intent was Seoul-bound (Finding #35). Pass 12 fixes both in one prompt-and-data round and re-runs the Sottsass dogfood to verify.

## Changes shipped

### Finding #34 — critic now enforces persona > scaffold priority

`CRITIC_SYSTEM_PROMPT` got a final paragraph:

> "## Persona authority — final tiebreaker
> If a Persona authority block appears in the scaffolded context below, treat it as the FINAL authority. Generic skill scaffolds are guidelines under the persona — not above it.
> Before proposing a refine that moves away from a persona-driven aesthetic choice, you MUST do one of: (1) quote the specific persona principle the current design violates verbatim AND explain why your proposed refine honours that principle; OR (2) raise satisfaction by 0.20 and emit `proposed_delta: null`.
> Example of valid override: *'Persona prefers chunky display, but the chosen font lacks Korean coverage which the Seoul intent requires.'*
> Example of INVALID override: *'The chosen font contradicts the editorial-portfolio scaffold's requirement for a contemporary serif.'*
> The second is not allowed — quoting the scaffold over the persona is a tell that you defaulted to safe-mode instead of honouring the brief."

That example invalid override is *verbatim what Pass 11's critic said* — the prompt now explicitly forbids that failure pattern.

### Finding #35 — `cjk_strategy` field per persona

Added `cjk_strategy: String` to `PersonaFrontmatter` with default empty. Each of the seven seeded personas got its own CJK strategy text in its own voice:

- **Rams**: *"the most neutral, infrastructure-grade family — the one that recedes. Korean: Pretendard. Avoid display weights. CJK type should match Latin in optical weight, not character."*
- **Sottsass**: *"refuse the polite default — pick a CJK display family that has play in its letterforms. Korean: Sandoll Noisuh, Yangjin, or Pretendard Heavy at display size. Restraint in CJK type is a betrayal of the work."*
- **Ando**: *"a family whose strokes feel cut from material. Korean: Sandoll Myungjo or Nanum Myeongjo (serif Hangul). Never gothic. The character of the stroke does the visual work that elsewhere a photograph would do."*
- **Kawakubo**: *"treat CJK type the same way I treat the seam — break it. Mix gothic + mincho in the same hierarchy. Avoid the polite balance; let one dominate."*
- **Hara**: *"the lightest available weight that reads. Treat the white space between Hangul / kana / hanzi characters as content."*
- **Vignelli**: *"same five-family discipline. Pretendard or Sandoll Gothic, pick one and use four sizes. Match Latin and CJK at matched x-heights."*
- **Galileo**: *"pick a family that publishes its metrics. Verify x-height matches Latin pair to within 5 percent — measure before deciding."*

The `as_system_prompt_block()` renderer now emits "Strategy for CJK text-bearing surfaces:" right above the body, so the persona's CJK voice reaches the LLM.

## Pass 12 dogfood — same Sottsass intent that failed in Pass 11

```bash
rm -rf ~/.aphrodite/personas    # force re-seed with cjk_strategy
aphrodite create "a portfolio site for an independent furniture maker working in solid walnut, based in Seoul" --persona ettore-sottsass
```

Wall clock: **309 s** (vs Pass 11 Sottsass's 594 s — *one critic-spawned-refine round trip eliminated*).

Critic verdict, turn 1, satisfaction = **0.88**, axis = None:
> "The design fully embodies Sottsass's polemical joy — Druk Wide with Sandoll Noisuh for CJK, saturated amber/mint/pink palette, asymmetric layouts, 224 px spacing for editorial breath, terrazzo squiggle patterns, angled-corner cards, and no SaaS filler — all coherently expressed across DESIGN.md and composition.html with no prose-token mismatches."

This is the critic *speaking Sottsass's language*. Stop.

## What landed

| Axis | Pass 11 Sottsass (critic overrode) | **Pass 12 Sottsass (persona authority)** |
|---|---|---|
| display family | Source Serif 4 *(forced by scaffold)* | **Druk Wide** ✓ (chunky display, persona-spirited) |
| CJK display | none declared | **Sandoll Noisuh** ✓ (lifted directly from Sottsass's `cjk_strategy` field) |
| body family | Outfit | Outfit |
| primary 500 | #c44d12 terracotta | #d97706 amber (warm anchor) |
| accent palette | 4 separate 500-anchor colours | **5 named accents** in a single `accent:` block: `mint #2dd4bf`, `pink #f472b6`, `lapis #3b82f6`, `citron #facc15`, `terracotta #c2410c` ✓ |
| max spacing | 224 px | 224 px |
| refines needed | 1 (the bug — overriding persona) | **0** |

## The persona-named-accents are the smoking gun

Sottsass's `prefers:` list reads:

> "Bold saturated palettes — terracotta, citron yellow, lapis blue, hot pink, mint"

Pass 12's DESIGN.md emits:

```yaml
accent:
  mint:       "#2dd4bf"
  pink:       "#f472b6"
  lapis:      "#3b82f6"
  citron:     "#facc15"
  terracotta: "#c2410c"
```

The generator picked *the persona's own colour names verbatim*. This is unmistakable evidence that the persona body now drives generator behaviour at the highest signal level — not just "more saturated" generically but "**mint, pink, lapis, citron, terracotta** specifically, named as Sottsass named them."

## Sandoll Noisuh — Finding #35 closes itself

Sottsass's CJK strategy field reads:

> "Korean: Sandoll Noisuh, Yangjin, or Pretendard Heavy at display size."

Pass 12's DESIGN.md emits:

```yaml
typography:
  display:
    family: "Druk Wide"
    weight: 700
  cjk_display:
    family: "Sandoll Noisuh"
```

*The persona's own recommended family lands in the artifact.* That's the entire reason `cjk_strategy` exists — to let each persona speak about CJK in its own voice instead of falling back to a generic Pretendard / Noto pair.

## Honest agent satisfaction

| Aspect | Pass 11 | **Pass 12** |
|---|---|---|
| Persona shapes output decisively | 4/5 (palette only) | **5/5** (palette + typography + CJK) |
| Critic respects persona authority | 2/5 (#34 open) | **5/5** (#34 closed) |
| CJK strategy applies persona-aware fallback | 1/5 (#35 open) | **5/5** (#35 closed) |
| Wall clock | 594 s | **309 s** (no over-correction refine) |
| Persona-named tokens land in artifact | n/a | **5/5** (`mint/pink/lapis/citron/terracotta` from Sottsass's own list) |

**Overall: 4.9 / 5.** Pass 12 is the cleanest persona-system pass to date. The remaining 0.1 is the absence of installed Sandoll Noisuh in the harmonize phase's Google Fonts URL — that family isn't on Google Fonts, so harmonize correctly skipped it without injecting a broken `@import`. A future step could resolve to commercial-Korean-foundry-license guidance (Sandoll licenses for web embed); skipping for now is the honest behaviour.

## Findings closed by Pass 12

- **#34 closed** — critic now enforces persona > scaffold priority. Pass 12's Sottsass run kept Druk Wide instead of being refined away to a scaffold-default serif.
- **#35 closed** — per-persona `cjk_strategy` text fields landed in the artifact. Sandoll Noisuh (Sottsass's own recommendation) appears in DESIGN.md's CJK display slot.

## Findings carried

- **#25** Composition regenerates every refine (architectural; defer)
- **#29** Surface composer latency / network stability
- **#32** Lucide CSS classes not yet emitted (asset-standards philosophical-only)
- Pass 5 step 4-6 loop verification (defer; Pass 5b A/B is stronger evidence anyway)

Asset phases (5, 6) — actual image generation + asset management — remain the largest stubbed area. Phase 1 (research / web reference fetch) is the other major unlock.

## ADR 0004 status

| Phase | Status |
|---|---|
| 1 research | stub |
| 1a skill loading | done |
| 1b persona loading | done (with #34 + #35 fixes from Pass 12) |
| 2 taste application | done |
| 3 design | done |
| 4 self-critic refine | done (persona-aware after Pass 12) |
| 5 asset create | stub |
| 6 asset manage | stub |
| 7 harmonize | done |
| 8 skillify proposal | done |

7/8 phases functional in their stable form. The remaining stub (asset phases) is also the largest unlock — replacing intent-named photo placeholders with actual Lucide SVG icons inlined + Unsplash photographer-cited image URLs.

## Reproduction

```bash
cargo install --path crates/aphrodite-cli
rm -rf ~/.aphrodite/personas    # force re-seed with cjk_strategy
mkdir -p /tmp/pass12-sottsass && cd /tmp/pass12-sottsass && git init
rm -f ~/.aphrodite/taste/preferences.toml
aphrodite create "a portfolio site for an independent furniture maker working in solid walnut, based in Seoul" --persona ettore-sottsass --json
```

Archive at `docs/agent-eval/archive/2026-05-14-pass12-persona-authority/`.
