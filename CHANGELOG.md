# Changelog

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
