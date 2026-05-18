# Changelog

## 1.0.0-rc.6 — 2026-05-18

### Added — Material-UI-grade docs site (`docs/index.html`)
- **`aphrodite docs`** CLI verb (also auto-emitted in phase 8.7).
  Renders a single-file documentation site: sticky sidebar TOC,
  install snippet, color swatch grid, type scale, and 42 component
  sections each with live demo + escaped JSX snippet. Switcher in the
  docs page itself cycles all 4 variants.
- 46 KB Korean-language docs site emitted on every run.

## 1.0.0-rc.5 — 2026-05-18

### Added — 12 advanced primitives (30 → 42)
- SegmentedControl, PinInput (OTP), NumberInput, SearchInput,
  FileUploader (drag-drop), DatePicker, Combobox (autocomplete),
  Sheet (bottom sheet), Alert (persistent banner), Stat (KPI),
  Toolbar, HoverCard.
- All with full ARIA, Korean stories.

## 1.0.0-rc.4 — 2026-05-18

### Added — Figma Variables sync
- **`aphrodite figma export`** — write `tokens.figma.json` in Tokens
  Studio plugin format. Auto-emitted by phase 8.7 on every
  `aphrodite create`. Designers import via the free Tokens Studio
  plugin (no Figma Enterprise plan required) to get one Figma Variable
  per Aphrodite token, scoped per variant.
- **`aphrodite figma pull <file_key>`** — fetches the linked Figma
  file's local variables via REST and diffs against DESIGN.md. Reports
  matched / only-in-design / only-in-figma / value-mismatches.
  Read-only (works on Pro/Org plans). Requires APHRODITE_FIGMA_TOKEN
  env var.
- Per emitted run now: 5-file design-system handoff
  (tokens.css + tokens.json + tokens.figma.json + components.html +
  components-{mobile,tablet,desktop}.png).
- 3 new figma_sync tests pass (Tokens Studio shape, diff correctness,
  color hex normalisation).

## 1.0.0-rc.3 — 2026-05-18 (v1.0 Release Candidate)

The Toss/Karrot-adoption milestone. v0.6-v0.8 closed the design-system
handoff gap; v1.0 closes the developer-adoption gap: actual typed React
components, npm-publishable package, Storybook stories, visual regression,
and automated publish workflow.

### Added — v1.0 RC.3 incremental
- **30 React primitives + 30 stories** (was 12 in RC.1) — Textarea,
  Select, Checkbox, Radio, RadioGroup, Tabs (+ TabList/Tab/TabPanel),
  Accordion (+ AccordionItem), Toast (+ ToastProvider/useToast),
  Tooltip, Popover, Menu (+ MenuItem), ProgressBar, Stepper, Slider,
  Breadcrumb, Pagination, Divider, EmptyState. Full ARIA wiring on
  every primitive.
- **`aphrodite diff <baseline> [<current>]`** — visual regression CLI
  verb. Pair-matches PNG screenshots, computes size-delta + byte
  equality, shells out to ImageMagick `compare` when available for
  per-pixel diff. Returns structured JSON for CI gates.
- **npm publish workflow auto-emitted** — every `react/` package now
  ships `.github/workflows/publish.yml` (tag-triggered npm publish),
  `.npmignore` (excludes stories + ci files from tarball), and a
  preserved CHANGELOG.md seed. `git tag v0.1.0 && git push --tags`
  publishes.
- **Per-run output**: 70 files in the `react/` package.

### Added — v1.0 React component package (RC.1 baseline)
- **`aphrodite react`** CLI verb: generates a publishable npm package

### Added — v1.0 React component package
- **`aphrodite react`** CLI verb: generates a publishable npm package
  (`react/`) from DESIGN.md. Auto-runs as part of every `aphrodite
  create` (phase 8.7).
- **12 typed React primitives**: Button, Input, Tag, Avatar, Card,
  Modal, Drawer, Skeleton, FormField, Switch, Badge, Spinner. Each is
  `forwardRef`, fully-typed Props, ARIA-correct, with sensible
  variant/size/state props matching DESIGN.md tokens.
- **`tokens.ts`** exporting a typed const `tokens` with the resolved
  4-variant palette + spacing + typography + a `VariantName` union.
- **Storybook stories** (CSF3) for every component — Korean copy
  baked in so designers see real intent.
- **`package.json`** with `@aphrodite/<project>` name, peer deps
  `react ≥ 18`, exports map, build script via `tsup`.
- **`README.md`** auto-generated quick-start.

### Added — v0.8 Aphrodite Quality Score
One-line composite production-readiness score on every run:
`● Aphrodite Quality Score: ✓ 92/100  (a11y=100 mobile=100 perf=90 semantic=100)`.
Surfaced in JSON result as `quality_score` + `quality_axes` for CI
gating.

### Added — v0.7 i18n
- harmonize::patch_lang_attribute — Hangul → `lang="ko"` automatic
  rewrite. Kana → `lang="ja"`. Idempotent.
- SURFACE_SYSTEM_PROMPT now has explicit i18n discipline section.
- Korean layout shim: `word-break: keep-all` + variant switcher mobile
  wrap auto-injected when Hangul present.

### Added — v0.6 design-system handoff
- `tokens.css` (CSS vars per variant, body[data-variant] scope)
- `tokens.json` (Style Dictionary shape, type-tagged leaves)
- `components.html` (Storybook-style HTML preview — designer
  inspection only, FE uses the React package)
- `aphrodite components` CLI verb (rebuild standalone after DESIGN.md
  edit)

### Closed gaps from v0.8 honest-assessment retrospective
- ✅ React/TSX component export
- ✅ Components 5 → 12+
- ✅ npm-publishable package output
- ✅ Storybook stories (CSF3)
- Remaining (not blocking adoption): Figma Variables sync, automated
  publishing workflow, axe/Lighthouse in CI gate.

### Catalog (after 1.0.0-rc.1)
- 60+ generator tests
- 11 CLI verbs (added `components`, `react`)
- 13 personas + 13 wiki entries + 6 skills
- 4 brand variants (light / dark / brand-a / brand-b)
- 3-viewport auto-screenshots (mobile / tablet / desktop) per page

## 0.3.7 — 2026-05-18

### Fixed — Pass 42/43 visual review surfaced four production-grade bugs

- **font-stack false positive in audit** — `extract_families_from_md` now
  splits on `,` and keeps the first family. Composer routinely writes
  `family: "Instrument Serif, Noto Serif KR, Georgia, serif"` into
  DESIGN.md; treating the entire stack as a font name caused
  `audit_composition` to flag a Google Fonts mismatch on every
  multi-family stack. Closes the Pass 42 ⚠ font-stack noise.
- **auto-promote h2 → h1 when zero h1** — `auto_fix_h1_count` was
  downgrade-only. Composer occasionally ships a section-only page with
  no top-level heading; harmonize now promotes the first `<h2>` instead
  of just warning. Pass 43's "Atelier" came in as h2 and was correctly
  promoted.
- **vh cap on placeholder pages** — `cap_vh_property` clamps
  `min-height: NNvh` ≤ 40 and `height: NNvh` ≤ 60 when the composition
  contains `class="image-placeholder"` figures. Composer habitually
  emits `.hero { min-height: 80vh }` + `.hero figure { height: 80vh }`
  for full-bleed photographs; without a real asset shipped, that's
  1500+px of dead vertical space. Pass 43's hero went from
  ~80% empty space to a tight, intentional layout.
- **variant CSS bleed onto switcher buttons** — `hero::inject_variant_css`
  now scopes to `body[data-variant=...]` instead of bare
  `[data-variant=...]`. The unscoped selector matched both `<body>` and
  the variant-switcher `<button>` elements (which carry `data-variant`
  for the click handler), painting the "dark" button's label invisible
  against its own dark-theme background.

### Added — Composition write guard
- Orchestrator now refuses to write `composition.html` if the rendered
  body is < 1 KB or lacks a `<body` tag. Surfaced in Pass 41 where
  harmonize injected a Google Fonts `<link>` into otherwise-empty
  composer output and produced a 292-byte file that falsely implied
  composition had succeeded.

### Catalog (after 0.3.7)
- 22 harmonize unit tests (4 new this release)
- All passes 38-43 in dogfood archive carry a Chrome-headless visual
  review screenshot.

## 0.3.5 — 2026-05-15 (late)

### Added — Per-call model override (closes Finding #37)
- **`[providers.<name>] composer_model` + `critic_model`** in `~/.aphrodite/config.toml`. Lets users send composer / critic calls to a faster z.ai tier (e.g. `glm-4.5-air`) while keeping the design call on the quality model (`glm-5.1`).
- `ResolvedProvider::with_model(m)` — clone the resolved provider with a different model.
- `aphrodite_generator::composer_model_override()` / `critic_model_override()` — config lookup helpers.
- Orchestrator logs `● using composer model override: <model>` and `● using critic model override: <model>` when active.

### Changed — Composer hardening
- `surface::trim_design_for_composer(design_md)` — pre-composer trim that keeps frontmatter + Components + Do/Don't, drops Overview/Colors/Typography/Layout prose. Reduces composer input by ~50% on heavy DESIGN.md outputs.
- Composer system + user prompt now end with an "Emit-immediately" directive forbidding pre-marker reasoning narration.
- `max_tokens` 12288 → 16384 (both composer attempts).

### Fixed — Finding #37 CLOSED
- Pass 37 dogfood: same clinical-dashboard × Galileo intent that hit empty composition in Passes 27/33/34/36 now produces a 43,761-byte composition.html with `composer_model = "glm-4.5-air"` configured. Critic moves from "composition.html is empty" complaints to content-level critique (missing status strip etc.).

### Findings open (after 0.3.5)
- **#13** brand-name palette recall
- **#25** composition regenerates every refine (architectural)

### Catalog (after 0.3.5)
- 13 personas + 13 wiki entries + 6 skills + 9 MCP tools + 11 CLI verbs
- 18 closed Findings, 2 carry

## 0.3.4 — 2026-05-15 (evening)

### Added
- **`aphrodite log [--n N]`** — recent Aphrodite-authored commits with kind-tag (create/refine/design/other), short SHA, relative time. Colour-coded.
- **`aphrodite undo [--n N] [--yes]`** — rolls back the last N Aphrodite auto-commits via `git reset --hard`. Dry-run by default; refuses to drop non-Aphrodite commits in the range so user work is never silently lost. `git reflog` recovers prior HEAD if needed.

### Negative result (committed and reverted, documented honestly)
- **Pass 34/35 — design/critic context split** (commit `8e7bf3b`, reverted in `2594b2f`). Hypothesis: lighten design-call augmentation (titles/tags/principles only; bodies reserved for critic) for speed gain on heavy intents. Empirical result: Pass 35 on the Seoul furniture-maker intent went from Pass 16's 251 s / 0 refines → 838 s / 1 refine. Hypothesis was wrong — with thinner anchoring the LLM iterates longer on its own picks, costing both speed and quality. Reverted. The architecture might resurface as an opt-in `--fast` flag in a later release.

### Findings carried
- **#13** brand-name palette recall
- **#25** composition regenerates every refine
- **#37 partial** — clinical-dashboard slow-DESIGN ceiling. Pass 33/34 confirmed bottleneck is z.ai's intent-class inference behaviour, not augmentation size. Right next experiment is *composer DESIGN.md truncation* (composer's input is heavy independent of augmentation), or per-class fast-model fallback.

### Catalog (after 0.3.4)
- 13 personas + 13 wiki entries + 6 skills + 9 MCP tools + **11 CLI verbs**

## 0.3.3 — 2026-05-15 (afternoon)

### Added
- **3 new bundled personas** (now 13): `stefan-sagmeister` (provocative autobiographical brand identity), `christoph-niemann` (conceptual illustration), `aldo-bakker` (austere artisan objects). All with `cjk_strategy` field.
- **2 new wiki entries** (now 13): `figma-site` (design-tool light-default), `aesop-website` (retail with editorial register).
- **`aphrodite curator`** — Hermes-pattern skill lifecycle review. `--dry-run`, `--stale-after-days`, `--archive-after-days` flags. Only touches `agent_created: true` skills. Active → Stale after 30 days; Active or Stale → Archived after 90 days. Writes `~/.aphrodite/curator/last-run.md`.
- **CHANGELOG.md** in `Keep a Changelog` format.

### Changed
- **Self-healing seeders** — `seed_bundled_personas` and `seed_bundled_wiki` now overwrite an on-disk entry only if it fails to parse. User edits to a parseable entry are preserved.
- **Composer retry v2** (Finding #37 mitigation) — when first composer call returns parsed-but-short, retries with `strip_augmentation(intent)` removing persona/skill/wiki blocks (they're already encoded in DESIGN.md). max_tokens bumped 8192 → 12288 on both attempts.
- **Persona + scaffold context caps** — `MAX_SCAFFOLD_BODY_CHARS` 4000 → 2500; new `MAX_PERSONA_BLOCK_CHARS` 4000 (with truncation marker).
- **Provider retry for z.ai wrapped-5xx** — `is_transient` matches `400` status with body containing `"code":"5"` or "Operation failed".

### Fixed
- **#37 partial close** — Pass 32 (Niemann × children's book) validates the composer retry path: critic now complains about CONTENT (dark variant green undertone), not empty composition. Pass 33 (clinical-dashboard × Galileo) reveals a separate slow-DESIGN-call ceiling that carries as partial.

### Open
- **#13** brand-name palette recall
- **#25** composition regenerates every refine
- **#37 partial** — clinical-dashboard-shape intents still hit slow-DESIGN-call ceiling even with caps. Mitigation ideas: drop the clinical-dashboard scaffold from the design call (apply at critic time only); fast-model for the design call; or accept the cost.

### Catalog (after 0.3.3)
- 13 personas + 13 wiki entries + 6 skills + 9 MCP tools
- 9/9 ADR 0004 phases functional

## 0.3.2 — 2026-05-15

### Added
- **3 new bundled skills** (skill catalog 3 → 6):
  - `dev-tool-saas-landing` — Linear / Vercel / Stripe register. Dark-default, single cold accent, real product screenshots, code-block-as-content.
  - `clinical-dashboard` — operator surfaces. 13-14 px body, monospace numbers, semantic colour with redundant icon, Galileo register.
  - `mobile-app-screen` — phone-framed composition (390×844), safe areas, 44×44 px touch target floor, single-handed thumb-reach.
- **MCP tool surface 5 → 9**: added `personas_list`, `wiki_list`, `wiki_show`, `assets_list`. Host model can now introspect the curated canon, not just call `create`.
- **3 new personas** (Pass 22/24/25 debut): `paul-rand`, `charlotte-perriand`, `naoto-fukasawa`. Each with `cjk_strategy` field in their own voice.
- **4 new wiki entries**: `vercel`, `nyt-magazine`, `pitchfork`, `teenage-engineering`.
- **`aphrodite create --estimate`** — dry-run preview of intent tags + loaded skills/personas/wiki + LLM call budget + wall-clock estimate. Zero provider calls.
- **`aphrodite wiki list / show / add [--auto-fetch]`** — user-curation of the design-reference wiki.
- **`aphrodite assets list / clean`** — inspect / clean `<project>/.aphrodite/assets/`.
- **`aphrodite personas`** — list installed personas with voice + when-to-invoke metadata.
- **Karpathy LLM-Wiki pattern** adopted as the Phase 1 research substrate. 7 hand-curated seed entries (Pretendard, MUJI, Apartamento, Pentagram, Linear, Are.na, Naver/Papago) at Pass 16; expanded to 11 by Pass 18.
- **Asset standards as default skill** (`asset-standards`, `default: true`) — Lucide icons / Fluent UI emoji / opinionated photography placeholders / anti-AI-slop copy hygiene, loaded on every create.
- **10 canonical Lucide SVGs** embedded in the asset-standards skill body (`arrow-right`, `arrow-up-right`, `mail`, `phone`, `map-pin`, `hammer`, `ruler`, `flask-conical`, `chart-line`, `chevron-right`).

### Changed
- **`aphrodite create`** is now the headline verb — autonomous creation harness (ADR 0004), 9 numbered phases all functional.
- **Orchestrator extracted** to `aphrodite_generator::orchestrator` so the CLI and MCP surfaces share one execution path.
- **README rewritten** for v0.3 — autonomous create as the headline, persona-driven demo, compounding wiki workflow, full verb table.
- **Persona authority over scaffold** (commit `46eab88`) — critic prompt explicitly forbids citing a generic scaffold to override a persona-driven aesthetic pick.
- **Critic prescriptive-vs-suggestive language rule** (commit `9c6589a`) — "good candidates: X, Y, Z" no longer triggers a refine when the chosen family is outside the list.
- **`harmonize::recover_lucide_classes`** — fingerprint-based recovery of `class="lucide lucide-X"` attributes for any SVG whose first path matches an embedded Lucide fingerprint.
- **Surface composer radical-register exception** — explicit handling of anti-templated briefs ("anti-fashion", "refuses to accommodate", "crop at viewport edges").
- **Surface composer minimum-output guard** (Finding #37 mitigation) — compositions under 1 KB are treated as malformed and surface a clear error rather than write an empty page.
- **Provider retry on z.ai wrapped-5xx** — `is_transient` now matches `400` status with body containing `"code":"5"` or "Operation failed" as transient.
- **Scaffold + persona context caps** — `MAX_SCAFFOLD_BODY_CHARS` 4000 → 2500; new `MAX_PERSONA_BLOCK_CHARS` 4000 with truncation marker. Keeps inference time predictable on heavy persona × skill stacks.
- **Cross-module test mutex** — single `aphrodite_core::test_lock::GLOBAL` shared across skills / personas / wiki test modules. Fixes parallel-test races.

### Fixed
- **#24** Google Fonts `@import` injection (Pass 10)
- **#26** hero typography token hookup via CSS variables (Pass 10)
- **#27** critic over-satisfaction without skill scaffolds (Pass 9)
- **#28** Cormorant-direction risk encoded in editorial-portfolio skill (Pass 9)
- **#29** provider transient failures retried with 1s/2s/4s back-off (Pass 14); extended to z.ai wrapped-5xx (Pass 28)
- **#30** warning false positives from scaffold content (Pass 9)
- **#31** critic over-corrects against suggestive scaffold lists (Pass 15)
- **#32** Lucide class labels stripped — in-prompt directive + harmonize-phase fingerprint recovery (Pass 15)
- **#34** critic ignores persona authority (Pass 12)
- **#35** CJK glyph coverage in personas (Pass 12; `cjk_strategy` field)
- **#36** surface composer can't render radical layouts (Pass 20)

### Open
- **#13** brand-name palette recall (Linear's `#5e6ad2` not auto-recognised)
- **#25** composition regenerates every refine (architectural; defer)
- **#37** clinical-dashboard context-window pressure — composer ceiling under heavy stacked context (Pass 27). Partial mitigation shipped (caps); fast-model fallback for composer call is the next intervention.

## 0.3.1 — 2026-05-14 (mid-day)

### Added
- **`aphrodite refine "<delta>"`** — multi-turn refinement verb (Pass 6)
- **`aphrodite create "<intent>"`** — autonomous creation harness (Pass 8)
- **Skill substrate** — `~/.aphrodite/skills/` with markdown frontmatter + usage tracker (Pass 9)
- **Persona system** — 7 seeded design authorities (Rams, Vignelli, Ando, Kawakubo, Sottsass, Hara, Galileo). `--persona <slug>` flag.
- **Phase 7 harmonize** — Google Fonts `@import` auto-injection, hero typography token hookup.
- **Asset management** — `<project>/.aphrodite/assets/{refs,uploads,icons}/`, content-hash dedupe, `aphrodite assets list / clean`.

## 0.2 — 2026-05-13

### Added
- **Surface composer** (Pass 4) — second LLM call producing real intent-specific compositions (pricing, dashboard, mobile_app, editorial, landing, portfolio) instead of the generic hero.
- **Taste loop** verified end-to-end (Pass 5b) — A/B with opposite pre-loaded preferences produces three independent axis flips.
- **WCAG-AA validator** across all 4 variants (light / dark / brand-a / brand-b).

## 0.1 — 2026-05-13

### Added
- `aphrodite design "<intent>"` — one-shot DESIGN.md + hero generation.
- `aphrodite-mcp` JSON-RPC stdio server (4 tools: design / redesign / validate / auth_status).
- Provider router: z.ai / Anthropic / OpenRouter / offline fallback.
- Google Labs alpha DESIGN.md schema (4 variants).
- Keychain-stored API keys (no plaintext files).
