# Agent Evaluation — 2026-05-14 — Tenth Pass (harmonize + asset standards)

> Pass 9 proved skill scaffolds steer the generator toward Pass 7's validated trajectory. Pass 10 wires the last two pieces ADR 0004 lists as Phase 7: automatic Google Fonts injection (Finding #24) and hero typography hookup (Finding #26). It also ships the `asset-standards` skill — a default-loaded scaffold encoding "no AI slop" defaults (Lucide icons, Fluent emoji, opinionated photography placeholders, anti-cliché copy hygiene).

## What got built

### Asset-standards default skill (`seed-skills/asset-standards/SKILL.md`)

A new bundled skill with `default: true` in frontmatter. It's loaded for every `aphrodite create` invocation regardless of intent tags — alongside any tag-matched domain skills. Contents:

- **Icons → Lucide** (lucide.dev/icons) with inline-SVG preference, MIT licence, anti-AI-slop icon-pick discipline ("for furniture craftsperson: `hammer`/`ruler`/`tree-deciduous` — avoid `sparkles`/`star`/`zap`")
- **Emoji → Fluent UI Emoji** (fluentemoji.com), embed URL pattern, "one emoji per page max" guidance
- **Photography** — opinionated *intent-named placeholders*, Unsplash photographer-cited fallback, avoid AI-generated stock
- **Illustrations** — undraw.co / heropatterns.com / haikei.app rules; avoid Memphis/purple-to-pink gradients/abstract-neural-network heros
- **Avatars** — dicebear.com, consistent style across the page
- **Copy hygiene** — explicit replace-table: "AI-powered" → concrete behavior; "seamless/robust/cutting-edge" → strip; round-number vanity stats → real-range numbers; "Trusted by industry leaders" → named clients
- **Real names + places** — Korean district names for Seoul intents (the Pass 7 generator did this organically — codified here)

The skill is `default: true` so `rank_for_intent` always includes it. New ADR 0004-aware behaviour: if a skill is `default: true` it scores a +80 bonus and is always considered, even with zero tag overlap.

### Phase 7 harmonize module (`crates/aphrodite-generator/src/harmonize.rs`)

Pure post-processing of the generated HTML — no LLM call. Responsibilities:

1. **Extract families from raw DESIGN.md.** Initial implementation tried to read the `DesignDocument` struct, but typography is stored as a `serde_yaml::Value` which doesn't round-trip cleanly through `serde_json::to_value`. Switched to a direct YAML-frontmatter scan that tolerates quoted, single-quoted, and unquoted family values.
2. **Build a Google Fonts CSS2 URL** for non-system families. Special-cased optical-sizing axis for Source Serif 4 / Newsreader / Fraunces (`opsz,wght@8..60,400;8..60,500;8..60,700`); standard `wght@300;400;500;600;700` for everything else.
3. **Inject `<link rel="stylesheet">`** into composition.html + hero.html just before `</head>`. Idempotent — skips if a Google Fonts link already exists.
4. **Fix hero typography.** Replaces the `font-family: ui-sans-serif, …` system fallback with a CSS-variable hookup that consumes the same typography tokens composition uses: `--aphrodite-typography-display`, `--aphrodite-typography-body`.

9 unit tests cover: URL building (system-skip, Source-Serif-4 opsz, all-system → None), head injection (canonical case, no-head fallback), hero typography replacement (matches + idempotent on already-fixed input), family extraction (canonical YAML, single-quoted, unquoted).

### Wiring into `create_cmd`

Phase 7 runs after the critic loop settles, before final write. The JSON output now includes a `harmonize` block:
```json
"harmonize": {
  "fonts_injected": ["Source Serif 4", "Inter"],
  "hero_typography_fixed": true,
  "notes": []
}
```
And `phases.harmonize` becomes `"applied"` or `"noop"`.

## Pass 10 dogfood — same intent, fresh state

```bash
mkdir -p /tmp/pass10b-harmonize && cd /tmp/pass10b-harmonize
git init && rm -f ~/.aphrodite/taste/preferences.toml
rm -rf ~/.aphrodite/skills
aphrodite create "a portfolio site for an independent furniture maker working in solid walnut, based in Seoul" --max-turns 3 --json
```

stderr trace:
```
● phase 8a / seeded bundled skills: asset-standards, editorial-portfolio
● phase 1 / loaded skill scaffolds: [asset-standards, editorial-portfolio] for tags [portfolio, furniture, solo-maker]
● phase 3 / design …                                  302 s
● phase 4 / self-critic refinement (max_turns=3, threshold=0.85)
  → turn 1 / critique …
    satisfaction=0.82  axis=prose_token_mismatch
    rationale="… editorial-portfolio skill specifically warns against Fraunces
              and recommends Source Serif 4 or Newsreader instead"
  → turn 1 / refine: Switch display family from Fraunces to Source Serif 4 …
  → turn 2 / critique …
    satisfaction=0.89  axis=—
● phase 7 / harmonize: injected fonts [Source Serif 4, Inter], hero typography hooked up
● done: 1 turns, satisfaction=0.89, total=553 s, llm_calls=6
```

### What landed

| Axis | Pass 7 final | Pass 9 (skill, 0 turns) | **Pass 10b (skill + harmonize, 1 turn)** |
|---|---|---|---|
| display | Source Serif 4 | Newsreader | **Source Serif 4** ✓ |
| body | Outfit | Outfit | **Inter** (different, equivalent register) |
| max spacing | 224 px | 224 px | 192 px (close, not identical) |
| brand-a bg | #faf9f6 | #f6f5f2 | **#faf9f6** ✓ identical |
| `@import` injected | n/a | n/a | **✓** Google Fonts with opsz axis |
| hero typography vars | n/a | n/a | **✓** `--aphrodite-typography-display/body` |
| Composition icons | mixed | mixed | 6 inline SVGs (not tagged Lucide-class, but inline-SVG path followed) |

### The critic's interesting failure mode

The critic *over-corrected*. The `editorial-portfolio` skill's actual text reads:

> "Display should be a contemporary serif with optical sizing (**Source Serif 4**, **Newsreader**, or **Fraunces (display optical)** are good candidates)."

Fraunces is *in* the validated set. But the critic's rationale claimed *"the editorial-portfolio skill specifically warns against Fraunces."* That's not what the skill says. The critic conflated the warning against "Renaissance / olde-style" serifs (Cormorant, EB Garamond) with Fraunces, which is a contemporary display-optical serif.

Net result was still correct (Source Serif 4 is unambiguously good), but the *reasoning* was wrong. This is the opposite failure mode of Pass 8's "too easily satisfied" — Pass 10's critic is **too eager to refine**, even against directionally-OK initial designs. The pendulum swung past center.

**Mitigation for v0.3.x:** add a `verify_skill_anchor` step where the critic must quote the exact skill text it's relying on. If it can't find the quote, fall back to satisfaction without refine. Future work; not blocking.

### Findings closed

- **#24 closed** — Google Fonts `@import` (`<link>`) is now injected into both composition.html and hero.html for non-system declared families. Verified in Pass 10b artifacts: `https://fonts.googleapis.com/css2?family=Source+Serif+4:opsz,wght@8..60,…&family=Inter:wght@…` present.
- **#26 closed** — Hero CSS now exposes `--aphrodite-typography-display` and `--aphrodite-typography-body` derived from the chosen families, replacing the generic `ui-sans-serif` fallback. Verified in Pass 10b's hero.html.

### Findings opened

- **#31** Critic over-corrects against skill scaffolds — quotes warnings that aren't there. Lower-impact than #27 was, but worth a future "skill-anchor verification" step before issuing a refine delta.
- **#32** Composition uses `<svg>` inline but doesn't tag them with Lucide CSS classes / data attributes. Asset-standards skill is being absorbed at the *philosophical* level (no icon-fonts, no AI-slop emoji) but not at the *implementation* level (specific Lucide path strings). Future work: extend asset-standards with a few example Lucide SVG snippets in the body so the generator has concrete copy-paste material.
- **#33** Pass 10b's wall clock 553 s vs Pass 9's 286 s reflects the additional refine round trip when the critic isn't satisfied at turn 1. With #31's fix this drops back toward Pass 9 territory.

### Findings carried

- **#25** Composition regenerates every refine (architectural; defer)
- **#29** Surface composer latency / transient network failures

## Honest agent satisfaction

| Aspect | Pass 7 | Pass 9 | **Pass 10b** |
|---|---|---|---|
| Output is the page asked for | 5 / 5 | 5 / 5 (4 axes) | 5 / 5 (4 axes) |
| Loop ran without external input | n/a | 5 / 5 | **5 / 5** |
| Caller cognitive load | High | Zero | **Zero** |
| Wall clock | 1036 s | 286 s | 553 s (one critic-spawned refine) |
| Font CSS actually loads in browser | 1 / 5 (#24) | 1 / 5 (#24) | **5 / 5** ✓ |
| Hero consumes typography tokens | 1 / 5 (#26) | 1 / 5 (#26) | **5 / 5** ✓ |
| Asset-standards philosophy applied (no slop) | n/a | n/a | 4 / 5 (philosophical yes, Lucide-class no) |

**Overall: 4.6 / 5.** The two long-standing harmonize findings closed; the asset-standards philosophy is now a default-load. The critic's over-correction is a tractable, narrow-scope issue.

## Status of ADR 0004 phases

| Phase | Status |
|---|---|
| 1. research | stub (intent-tag extraction acts as a poor man's research-stand-in) |
| 1a. skill loading | **done** (Pass 9) |
| 2. taste application | done since Pass 5b |
| 3. design | done since v0.2 |
| 4. self-critic refine | **done** (Pass 8 / Pass 10 evidence) |
| 5. asset create | stub (asset-standards skill now provides guidance; actual asset generation deferred) |
| 6. asset manage | stub |
| 7. harmonize | **done** (Pass 10) |
| 8. skillify proposal | **done** (Pass 9 surfaces correctly when conditions met) |

5/8 phases functional. The two asset phases (5, 6) are next big unlock — they're the only remaining stubs. Phase 1 (research / web reference fetch) is the most architecturally additive — adds a new tool (web_search) to the orchestrator.

## Reproduction

```bash
cargo install --path crates/aphrodite-cli
mkdir -p /tmp/pass10-harmonize && cd /tmp/pass10-harmonize
git init && rm -f ~/.aphrodite/taste/preferences.toml
rm -rf ~/.aphrodite/skills    # force bundled-skill re-seed
aphrodite create "a portfolio site for an independent furniture maker working in solid walnut, based in Seoul" --json
```

Archive at `docs/agent-eval/archive/2026-05-14-pass10-harmonize/`.
