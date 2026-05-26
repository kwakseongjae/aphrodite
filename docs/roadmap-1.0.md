# Aphrodite v1.0 Roadmap

> **Status**: Active planning artifact, revised 2026-05-19.
> **Source**: Ouroboros Seed `seed_778d10b9444b` (interview `interview_20260519_051320`, ambiguity 0.17) + ADDENDUM from interview `interview_20260519_064346` (engine reached partial saturation after 6 force-choice rounds; ambiguity stuck at 0.28 — engine stalled, but the decisions captured below are authoritative).
> **Current**: v1.0 RC.9 + Gate 1 partial (`b1d6908`), 78 unit tests, **Gate 1 SSR Next.js 14: 69/69 green** ✓, 70 React components, 5/5 cross-brand alpha verified.
> **Target**: `1.0.0` stable cut, published as `@aphrodite-design/aphrodite-agent` on npm.

This document is the source of truth. Every PR toward 1.0 must reference a section here. Drift means re-running an Ouroboros turn and updating this file's source-line.

---

## 0. CRITICAL CORRECTION (2026-05-19, second interview)

The original Seed assumed the published 1.0 artifact was a pre-generated component package (`@aphrodite-design/example-banchan`). **That was wrong.**

**Aphrodite's product is the CLI/agent itself**: users install the agent, run `aphrodite create "<their brand>"`, and get their own brand's component package emitted locally.

- ✅ **1.0 publish target**: `@aphrodite-design/aphrodite-agent` — the CLI binary, distributed via npm wrapper + `cargo install` + Homebrew tap.
- ❌ **NOT**: `@aphrodite-design/example-banchan` — this is internal SSR-fixture only, never published.
- Banchan / Hada / Junjong / Yeoreum / Sori are **fictional alpha-test fixtures** for cross-brand verification, not products.

### Distribution model

- **npm wrapper package (~5KB)**: `postinstall` script detects `process.platform + process.arch`, downloads matching Rust binary from GitHub Releases into `node_modules/.bin/`. NEVER bundles binaries in the npm tarball (would be 80MB+).
- **Cross-platform binaries** built in GitHub Actions matrix: macOS arm64, macOS x86_64 (beta-blocking); Linux x64, Linux arm64, Windows x64 (1.0-blocking, Windows deferrable to 1.0.1).
- **Cargo path** unchanged: `cargo install aphrodite-cli` from source for Rust devs.
- **Homebrew tap** (post-1.0): `brew install aphrodite-design/tap/aphrodite-agent`.
- **postinstall UX**: explicit + progress bar. Silent feels nicer but masks the network step — first-time users on slow networks hit confusing pauses.
- **API-key onboarding**: user supplies own z.ai (or Anthropic) API key. No shared credit pool — security + cost risk too high. README states "Sign up at zai.com → `aphrodite auth set zai`" with cost estimate (~$0.10/run).

---

## 1. North Star — Why this exists

Aphrodite is being built to one specific bar: **a Korean designer + frontend-engineer pair at a Toss / Karrot / Gangnam-Unni-grade company, on a 30-minute first-look, decides Aphrodite is the right starting point for their next 0-1 service / brand / side-project.**

Concrete signals of that bar:
- `npm i @aphrodite-design/example-banchan` works on day one from a public registry.
- They `npm run dev` in a Next.js 14 App Router project, import a `<Button>`, and it renders the correct brand colors without throwing hydration warnings.
- They browse `docs.aphrodite.dev`, search "modal", find a working live demo + copy-paste snippet within seconds.
- Their designer opens Figma, imports `tokens.figma.json` via Tokens Studio, and sees the same palette + scale.
- They run `aphrodite create "<their brand>" --pages home,pricing,signup`, get a multi-page site + a publishable React package, and trust it enough to deploy.

User-quoted philosophy (verbatim from this session):

| Quote | Implication |
|---|---|
| "최소 우로보로스급 포지셔닝… Hermes-Agent처럼 사람들이 깔자마자 신뢰하고 쓰는" | Adoption velocity > feature count |
| "토스, 당근, 강남언니 으로 멀티 페이지, 멀티 플랫폼, 멀티 브랜드 production 레디" | Korean professional bar, not hobby |
| "디자이너, 프론트엔드가 적극 활용할 수 있을 수준" | Both consumer types must onboard cleanly |
| "0-1 서비스 페이지들을 만든다는 가정" | New-brand starting point, not enterprise migration target |
| "z.ai로 만족할 만한 퀄리티 우선… openrouter는 추후" | Cost-aware, ship what works |
| "browser-harness 있으니까 시각적으로 보고 확인" | Visual evidence at every step, not just type checks |
| "각 단계별로 알파 테스트, 1.0 전 베타, 그 다음 런칭" | Falsifiable milestones, not RC name games |

Non-goals for 1.0 (deferred to v1.1+):

- RTL / Arabic / Hebrew support (KR market is LTR)
- True i18n system (`t()` helper, locale bundles, ICU MessageFormat) — `<html lang>` patch + Pretendard suffices
- Components 70 → 100+ (RichTextEditor, DataGrid, advanced Calendar UI)
- Full Figma plugin with bidirectional write-back (Tokens Studio one-way export is acceptable)
- Lighthouse / axe-core CI enforcement (advisory hook ships now; enforcement later)
- Business model / pricing definition
- Hosted offering / SaaS
- 13 personas → 20+ (incremental)

---

## 2. Where we are (RC.9 baseline)

### Shipped capabilities

- **One-command full output**: `aphrodite create "<intent>" --persona <slug> --pages a,b,c` emits, into the target repo:
  - `DESIGN.md` + 4 variant tokens (light/dark/brand-a/brand-b), WCAG-AA validated
  - N `<page>.html` + `sitemap.xml` + `hero.html`
  - `tokens.css` + `tokens.json` (Style Dictionary) + `tokens.figma.json` (Tokens Studio)
  - `components.html` (designer Storybook preview)
  - `docs/index.html` (Material-UI-grade docs)
  - `react/` — publishable npm package, 70 typed forwardRef components + 70 CSF3 stories + tokens.ts + styles.css + package.json + tsconfig + README + CHANGELOG + .npmignore + `.github/workflows/publish.yml`
  - 3-viewport screenshots (mobile 360 / tablet 768 / desktop 1440) per page
- **Aphrodite Quality Score 0–100**: 4 axes (a11y / mobile / perf / semantic) + JSON result for CI gating
- **CLI verbs**: `create`, `refine`, `redesign`, `components`, `react`, `docs`, `figma export|pull`, `diff` (visual regression vs baseline), `eval` (cross-brand sweep with aggregate report), plus the keychain/auth/skill/wiki/persona surface
- **13 personas** (Dieter Rams, Tadao Ando, Kenya Hara, Massimo Vignelli, Galileo Galilei, Rei Kawakubo, Ettore Sottsass, Charlotte Perriand, Naoto Fukasawa, Stefan Sagmeister, Christoph Niemann, Aldo Bakker, Paul Rand)
- **13 wiki references** (Apartamento, Are.na, Pentagram, Linear, Vercel, …)
- **6 skill scaffolds** (asset-standards default + editorial-portfolio, corporate-identity-systems, dev-tool-saas-landing, clinical-dashboard, mobile-app-screen)

### Verified alpha sweep (Pass 50 → 55)

| Brand (fictional alpha fixture) | Domain | Quality | Notes |
|---|---|---|---|
| Banchan | 식품 정기구독 | 100/100 | canonical reference brand |
| Hada | B2B 핀테크 | 100/100 | SaaS form-heavy |
| Junjong | 인디 디자인 매거진 | 93/100 | editorial archetype |
| Yeoreum | DTC 아이스크림 | 100/100 | e-commerce photo-heavy |
| Sori | 인디 음악 스트리밍 | 93/100 | content / streaming |

5/5 cross-brand alpha pass: 0 phone-frame regressions, 0 external-img leaks. landing-pin fix (commit `59c2a06`) robust across all service-intent domains.

### Honest gaps blocking 1.0

| # | Gap | Adoption-blocker? | Effort |
|---|---|---|---|
| 1 | SSR / Next.js App Router compat unaudited | 🔴 yes — silent killer | ~5 days |
| 2 | No npm-published package | 🔴 yes — friction wall | ~2 days |
| 3 | No public docs site | 🔴 yes — eval surface missing | ~3-4 days |
| 4 | `aphrodite publish --rename @org/name` missing | 🟠 high — adopter blocker | ~1 day |
| 5 | Components 70 → 100+ | 🟡 wishlist, defer | — |
| 6 | RTL / true i18n | 🟢 defer | — |
| 7 | Figma plugin write-back | 🟡 defer | — |
| 8 | External alpha testers | 🟠 beta phase | — |
| 9 | Lighthouse CI enforcement | 🟡 advisory hook ships now | — |
| 10 | Business model / community | 🟢 post-launch | — |

---

## 3. Architecture decisions (locked by Seed)

### Package model — Model A with N=1 test surface

- Each `aphrodite create` emits a full per-brand package (`@<org>/<brand>`) with all 70 components themed via `tokens.ts` + `styles.css`.
- The 70 `.tsx` files are **byte-identical across brand emits** — only `tokens.ts` (data) + `styles.css` (CSS-var scopes) differ.
- This means the SSR test matrix is **70 components × 1 canonical brand (Banchan)** — NOT 70 × N brands. The component logic surface is N=1.
- Brand variation is data; component variation requires explicit changes, surfaced via existing tests.
- Aphrodite publishes ONE evaluator package: `@aphrodite-design/example-banchan@<version>` — the canonical install referenced from README, docs, and CLI defaults.
- Adopters who use Aphrodite to build their own product publish under their own namespace via `aphrodite publish --rename @theirorg/their-brand`.

### npm namespace — `@aphrodite-design`

**Decision rationale** (probe results, 2026-05-19):
- `aphrodite` (unscoped) → owned by Khan Academy (popular CSS-in-JS library, v2.4.0, ~50M weekly downloads). Cannot use.
- `@aphrodite` (scope) → squat-held by user named `aphrodite` (0 packages but scope reserved).
- `@aphrodite-ui`, `@aphro` → 200 responses with empty members — registered but stale. Risk of conflict.
- `@aph` → registered with owner.
- **`@aphrodite-design`** → 404 "Scope not found" — clean, available.
- Alternatives if `@aphrodite-design` becomes unavailable before registration: `@aphrodite-kit`, `@aphrodite-system`, `@aphrodite-dev`.

**Action item (user-owned, this week)**:
```bash
npm login                                 # kwakseongjae
npm org create aphrodite-design           # org owner = kwakseongjae
# placeholder publish to reserve canonical names:
npm publish @aphrodite-design/example-banchan@0.0.0 --access public
npm publish @aphrodite-design/cli@0.0.0 --access public
npm publish @aphrodite-design/react@0.0.0 --access public
```

Once org is registered, all generator code paths and docs that reference `@aphrodite/<x>` get one batch swap to `@aphrodite-design/<x>`.

---

## 4. The 3 Gates to 1.0

Strict ordering: a gate cannot start until the prior gate's exit criteria are met for at least one cross-brand sweep.

### Gate 1 — SSR / Next.js App Router compatibility (HIGHEST PRIORITY)

**Why first**: silent failure mode. If a component throws on hydration in Next.js production build, the Toss engineer doesn't file an issue — they uninstall and walk away. We have zero visibility into this risk today.

**Test harness layout** (in this repo):

```
tests/ssr/
├── next-app/           # Next.js 14 App Router scaffold, pnpm
├── vite-app/           # Vite + React + react-router
├── remix-app/          # Remix scaffold
├── playwright/
│   └── component-smoke.spec.ts   # per-component build + hydration + themed
└── scripts/
    ├── install-example.sh        # link the canonical Banchan package
    └── run-matrix.ts             # iterate 70 components × 3 frameworks
```

**Three test layers — all blocking for 1.0**:

1. **Build layer** (free; catches structural defects)
   - `next build` (App Router) → exit 0
   - `vite build` → exit 0
   - `remix build` → exit 0
   - Catches: missing `"use client"` directives, top-level `window` access, broken import paths, missing peer-dep declarations.

2. **Hydration layer** (medium; catches the silent killer)
   - Playwright loads `next start` page that imports each of the 70 components.
   - Captures `console.error` + `console.warn` during hydration.
   - Asserts zero entries matching `hydration|mismatch|server-rendered|Hydration failed`.
   - Catches: `useId` drift, `useEffect`-during-SSR, conditional browser-only rendering, ToastProvider mount-order bugs.

3. **Themed-render layer** (high but bounded; catches CSS-var failures)
   - Playwright `page.locator('[class*="aph-btn"]').evaluate(el => getComputedStyle(el).backgroundColor)`.
   - Asserts the resolved value is NOT the default-gray fallback (i.e., the CSS variable cascade reached the rendered DOM).
   - Catches: theme not applied, `data-variant` scope failing on body, styles.css not loaded.

**Not in 1.0 gate** (deferred to v1.1):
- Interactive scripts (click/type/focus assertions per component) — defer
- Pixel-perfect baseline visual regression — `aphrodite diff` infra exists; baseline management defer
- axe-core full dynamic scan — static a11y rules ship now

**CI wiring**:
- New GitHub Actions job `ssr-matrix` runs on every PR.
- Job invokes `tests/ssr/scripts/run-matrix.ts`.
- Exits non-zero on any layer failure.
- PR cannot merge without green.
- 3 consecutive green runs across all 3 frameworks required before tagging 1.0.

**Component ejection rule**: If after one remediation attempt a component still fails any of the 3 layers, it is REMOVED from the 1.0 catalog and deferred to a post-1.0 release. We do not ship a broken `<Toast>` with a footnote.

**Estimated effort**: 5 working days
- Day 1: scaffold next-app + vite-app + remix-app + install canonical example-banchan
- Day 2: Playwright spec for build + hydration capture
- Day 3: themed-render assertion + matrix runner
- Day 4: GitHub Actions wiring, first end-to-end CI run, fix any structural component bugs
- Day 5: full remediation pass + 3 consecutive green runs

**Exit criteria (falsifiable)**:
- [ ] 70/70 components: `next build` exit 0
- [ ] 70/70 components: `vite build` exit 0
- [ ] 70/70 components: `remix build` exit 0
- [ ] 70/70 components: zero hydration warnings on Next.js App Router page render
- [ ] 70/70 components: themed CSS-var color resolves to non-default
- [ ] 3 consecutive green CI runs on all 3 frameworks
- [ ] Any failed component is explicitly listed in `docs/component-status.md` as "deferred to post-1.0" with reason

### Gate 2 — CLI distribution as `@aphrodite-design/aphrodite-agent`

**Why second**: gates the "install and use" path. The CLI itself is the published 1.0 artifact — without an npm-installable agent, users can't do anything.

**Pre-work (user-owned, ✓ partial)**:
- [x] Register `@aphrodite-design` npm org (kwakseongjae account, completed 2026-05-19).
- [ ] Placeholder-publish `@aphrodite-design/aphrodite-agent@0.0.0` reserving the canonical name.

**Engineering work**:

1. **Cross-platform Rust release builds in CI**
   - GitHub Actions matrix: `macos-latest` (arm64 + x86_64), `ubuntu-latest` (x64 + arm64 via `cross`), `windows-latest` (x64).
   - Per-target build: `cargo build --release -p aphrodite-cli` → strip → tar.gz.
   - Artifacts uploaded to GitHub Releases on every tag matching `agent-v*.*.*`.
   - **Beta-blocking**: macOS arm64 + macOS x86_64 (KR FE market is 95%+ M-series; Intel macs still in some service).
   - **1.0-blocking**: Linux x64 + Linux arm64 (Docker / CI consumers).
   - **1.0.1-deferrable**: Windows x64 (KR FE has ~0 Windows users; if first-pass linking has issues, ship as point release).
   - Estimate: 2 days.

2. **npm wrapper package `@aphrodite-design/aphrodite-agent`**
   - Lightweight (~5KB) Node.js package, not the binary itself.
   - `package.json`: `bin: { aphrodite: "./bin/aphrodite-wrapper.js" }`, peer `node >= 18`.
   - `postinstall` script: detects `process.platform + process.arch`, downloads matching binary from GitHub Releases into `node_modules/@aphrodite-design/aphrodite-agent/bin/`.
   - **Explicit UX**: "Downloading aphrodite v1.0.0-beta.1 for darwin-arm64 (18MB)..." with progress bar — silent install hides network failures from new users.
   - Wrapper shim execs the downloaded binary, passes args + stdio through.
   - Estimate: 1 day.

3. **`aphrodite publish --rename @org/name [--dry-run]` CLI verb (for adopters)**
   - Reads the generated `react/` directory.
   - Substitutes `@aphrodite-design/<old-kebab>` → `@<org>/<new-name>` in `package.json` name + README badges + CHANGELOG seed footer + any Aphrodite-self URL references.
   - Invokes `npm publish --access public`.
   - `--dry-run` flag prints what would change without executing.
   - This is for adopters who use Aphrodite to make THEIR OWN brand emit and want to publish under THEIR namespace, not for the canonical CLI publish.
   - Estimate: 1 day.

4. **Auto-publish workflow `.github/workflows/agent-publish.yml`**
   - Triggers on tag `agent-vX.Y.Z`.
   - Steps: full cross-platform binary build (job 1 above) → upload to GitHub Releases → `cd npm-wrapper && npm publish --access public --provenance`.
   - `NPM_TOKEN` secret with publish scope on `@aphrodite-design`. 2FA on npm account.
   - Estimate: 0.5 day.

5. **API-key onboarding flow polish**
   - `aphrodite init` first-run wizard already exists; add a "you need a z.ai or Anthropic key" preamble.
   - README explicit: "Sign up at zai.com (~2min), then `aphrodite auth set zai`."
   - Cost transparency: "1 `aphrodite create` run = ~8 LLM calls ≈ $0.10 z.ai."
   - Estimate: 0.5 day.

**Exit criteria (falsifiable)**:
- [ ] macOS arm64 + x86_64 binaries on GitHub Releases for tag `agent-v1.0.0-beta.1`
- [ ] `npm i -g @aphrodite-design/aphrodite-agent@1.0.0-beta.1` from a fresh shell succeeds (no aphrodite repo cloned)
- [ ] postinstall downloads the correct platform binary, prints progress, leaves usable `aphrodite` binary in PATH
- [ ] `aphrodite --version` returns the matching version
- [ ] `aphrodite create "test brand"` end-to-end on a stranger ubuntu/macos CI VM produces a valid emit (Quality Score ≥ 85)
- [ ] `aphrodite publish --rename @testorg/testbrand --dry-run` correctly shows substitutions without executing
- [ ] Auto-publish workflow on tag triggers: build → release upload → npm publish, all green
- [ ] Linux x64 binaries present (1.0 blocker, not beta) — postinstall handles Linux correctly

### Gate 3 — `docs.aphrodite.dev` public docs site

**Why third**: gates the "look and decide" path. Without searchable per-component docs with live demos, evaluators bounce in <30 minutes.

**Scope decision — separate repo or in-tree?**

Recommendation: **separate repo `aphrodite-docs/`** for clean deployment (Vercel/Netlify), but generated by Aphrodite itself via a new mode of the `docs` CLI verb. Specifically:

- New verb mode: `aphrodite docs --site` emits a multi-page Next.js or Astro site to `docs-site/`
- Existing per-run `docs/index.html` mode kept (it ships inside every adopter's emit)
- The site repo's CI subscribes to aphrodite-cli releases and re-builds

**Content scope for 1.0**:
- Landing / hero with one-line value prop + agent install snippet: `npm i -g @aphrodite-design/aphrodite-agent`
- "Getting started" page: install agent → `aphrodite auth set zai` → `aphrodite create "<your brand>" --pages home,a,b` → "Open the emitted react/ and docs/" walkthrough
- 70-component catalog (live demos from a baked Banchan reference; not "install this", but "see what gets generated")
- Per-component page: short description + live demo + JSX snippet + props table
- "Themes" page: brand-token switcher (light/dark/brand-a/brand-b + 5 alpha-fixture brand previews)
- "Examples gallery": 5 cross-brand alpha screenshots (Banchan home, Hada signup, Junjong issues, Yeoreum flavors, Sori home) with the intent strings that produced them
- "How adopters publish their emit" page: `aphrodite publish --rename @theirorg/their-brand`
- "Roadmap" page linking to this file
- "Aphrodite Quality Score" explanation page

**Out of scope for 1.0**:
- API reference for the Rust harness (defer)
- Persona documentation (link to repo)
- Tutorial videos (defer)
- Multi-language docs (defer; KR + EN bilingual)

**Estimated effort**: 4 working days
- Day 1: site scaffold (Astro recommended for content-heavy + low JS), routing, layout, design tokens
- Day 2: 70-component catalog page + search
- Day 3: per-component page generator (reuse `docs_site.rs` Rust module to emit the JSON metadata)
- Day 4: deploy to Vercel/Netlify with `docs.aphrodite.dev` DNS, polish, dark mode

**Exit criteria (falsifiable)**:
- [ ] `docs.aphrodite.dev` resolves with HTTPS
- [ ] 70 component pages live, each with at least one rendered demo
- [ ] Search box returns matching components for "modal", "input", "차트", "skeleton"
- [ ] Install snippet on home is `npm i -g @aphrodite-design/aphrodite-agent` (matches Gate 2)
- [ ] Brand-token switcher works (light/dark/brand-a/brand-b at minimum)
- [ ] Lighthouse Performance ≥ 90 on the docs site itself (eat our own cooking)
- [ ] Mobile (360px) responsive, no horizontal scroll, Korean text reads cleanly

---

## 5. Alpha → Beta → Launch cycle

### Per-gate alpha test

Each gate's exit requires a fresh cross-brand sweep, run via `aphrodite eval`:

```bash
# tests/eval/sweep-1.0.json
[
  {"name": "Banchan",  "intent": "...", "persona": "dieter-rams", "pages": ["home","menu","pricing","faq","about"]},
  {"name": "Hada",     "intent": "...", "persona": "dieter-rams", "pages": ["home","features","pricing","signup","faq"]},
  {"name": "Junjong",  "intent": "...", "persona": "kenya-hara",  "pages": ["home","issues","subscribe","archive","about"]},
  {"name": "Yeoreum",  "intent": "...", "persona": "dieter-rams", "pages": ["home","flavors","shop","delivery","about"]},
  {"name": "Sori",     "intent": "...", "persona": "dieter-rams", "pages": ["home","artists","charts","subscribe","about"]}
]
```

```bash
aphrodite eval tests/eval/sweep-1.0.json --out eval-out/
```

**Pass criteria per gate**:
- All 5 brands hit Quality Score ≥ 90
- Mean Quality ≥ 95
- Zero `Failed` status (i.e., zero brand with score < 75)
- Zero phone-frame mockup regressions (the landing-pin discipline must hold)
- Zero external `<img src="https://...">` leaks
- After Gate 1 specifically: zero SSR hydration warnings on the Banchan run when fed through `tests/ssr/`

The eval sweep is the gate's regression-detection net. Any regression discovered during gate work must be fixed before the gate is declared closed.

### Beta phase (after all 3 gates pass)

**Entry condition**: all 3 gates show ✓ on the checklist above + at least 1 internal full cross-brand sweep with mean Quality ≥ 95 + beta-1 binary tested locally by maintainer on a fresh macOS account.

**Version progression**: `1.0.0-beta.1` → `1.0.0-beta.N` (bug-driven iterations) → `1.0.0-rc.1` → `1.0.0`. Standard semver prerelease ordering (`1.0.0-beta.1` < `1.0.0`), matches React/Next/Vite convention. Decision (2026-05-26): the product line is `1.0.0` from the first public cut — no `0.x` framing; betas are prereleases of `1.0.0`.

**Beta tester recruitment**:
- Target: **3-5 external alpha testers** (real Korean designer + FE pairs).
- Channels (priority order): personal KR FE network at Toss / Karrot / Naver / Kakao / Coupang via DM; Disquiet (KR maker community); Geeknews (KR tech aggregator); X/Twitter Korean dev community.
- Compensation: zero monetary. Public credit in README + early-adopter Discord access + free use of any v1.1+ premium features (undefined now).
- Recruitment starts **1 week before Gate 3 closes** (testers warm by beta-1 publish).
- Per-tester time investment: 15-20min mechanical pass + 5min docs eyeball; the integrator (≥ 1 tester) spends 1-3 hours over the cycle.

**Beta success per-tester (falsifiable)**:

For each tester, criteria 1-7 ALL hold:

1. `npm i -g @aphrodite-design/aphrodite-agent@1.0.0-beta.X` succeeds; `aphrodite --version` returns
2. `aphrodite create "<their brand>"` completes; Quality Score ≥ 85
3. `cd <brand>-design && npm install` succeeds (no peer-dep conflicts, no missing-binary errors)
4. `npm run build` exits 0 (`tsup` pipeline)
5. `npm test` exits 0 (Vitest + JSDOM smoke: each component loads + renders + non-empty)
6. `npm run typecheck` exits 0 (`tsc --noEmit`)
7. Tester opens `docs/index.html`, reports the design system "fits their brand intent" (5min eyeball)

**Beta exit criteria across the pool (falsifiable)**:

- [ ] **≥ 3 testers complete criteria 1-7**
- [ ] **≥ 1 of those testers completes criteria 8 (adoption)**: integrates ≥ 1 generated React component into a real project. "Real project" = personal landing deployed to Vercel/Netlify/CF Pages, OR open-source side project in GitHub, OR internal tool used by their team (private, vouched). NOT CodeSandbox / StackBlitz / isolated demo.
- [ ] **All P0 / P1 bugs filed during beta are closed**; P2 / P3 may roll to v1.1
- [ ] **No new critical SSR regression** introduced after `1.0.0-beta.1`
- [ ] **Beta cycle duration**: target 2-4 calendar weeks; **hard cap 4 weeks**

**Mid-cycle progress signals** (don't wait for the 4-week deadline):

| Week | Healthy signal | Action if missing |
|---|---|---|
| W1 | ≥ 1 tester completed criterion 1 (install OK) | Investigate postinstall bug; re-publish beta.N+1 |
| W2 | ≥ 1 tester completed criteria 1-7 (mechanical pass + visual eyeball) | DM testers personally; if recruitment failed, lower target 3 → 2 |
| W3 | ≥ 1 tester started integrating into a real project (signal of intent for criterion 8) | Extend tester pool by 2 more, OR explicit user decision: ship with "early-adopter" framing on (8) |
| W4 | All criteria 1-10 met OR explicit user decision to extend / ship | User picks: extend 2 weeks; OR ship `1.0.0` with explicit "early-adopter" disclaimer in README + drop criterion 8 to "expected soon" |

**Telemetry** (opt-in only, default OFF):
- At first `aphrodite create` after agent install, prompt: "Send anonymous run telemetry to help us improve? [y/N]"
- Payload: `{run_id (uuid), quality_score, llm_calls, wall_clock_s, intent_tag_count, persona_slug, agent_version, platform}`. Never includes intent string, brand name, or any user content. IP hashed at edge.
- Helps mid-cycle progress measurement without manual surveys. Falls back to direct DM check-ins if user opts out.

**Feedback intake**:
- GitHub Issues template `BETA-FEEDBACK`
- Direct DM channel (Discord or Slack — maintainer-driven)
- 1 mandatory 30-min interview per tester (recorded notes)

**Rollback policy**:
- P0 reported (broken install / data loss / silent corruption): publish `1.0.0-beta.N+1` with fix or revert within **24 hours**.
- If P0 unfixable in 24h: `npm deprecate` the broken beta version and announce in tester channel.

### 1.0 launch criteria (final, falsifiable)

Tag `v1.0.0` ONLY when ALL of these hold simultaneously:

- [ ] **Gate 1 SSR**: all 70 components green on Next.js 14 ✓ + Vite ✓ + Remix ✓ — 3 consecutive green CI runs on each framework
- [ ] **Gate 2 distribution**: 
  - macOS arm64 + x86_64 binaries on GitHub Releases for `agent-v1.0.0` tag
  - Linux x64 binaries also on releases (Linux arm64 may slip)
  - `npm i -g @aphrodite-design/aphrodite-agent@1.0.0` works from fresh shell on macOS + Linux
  - Windows x64: ship in 1.0.0 if green; otherwise document as "1.0.1 soon" with explicit issue
  - Auto-publish workflow + provenance + 2FA enabled on npm account
- [ ] **Gate 3 docs**: `docs.aphrodite.dev` live, hero install = `npm i -g @aphrodite-design/aphrodite-agent`, 70-component catalog + search + brand-token switcher + KR responsive
- [ ] **Beta phase passed**: criteria 1-7 met by ≥ 3 testers, criterion 8 met by ≥ 1 (or explicit early-adopter ship decision)
- [ ] **CHANGELOG** documents every breaking change since RC.9
- [ ] **`aphrodite eval`** sweep on 5 reference brands: mean Quality ≥ 95, zero Failed
- [ ] **24-hour docs content freeze** before tag; any docs change during freeze re-runs beta acceptance suite
- [ ] Honest gap list in this document marked either "✓ done" or "deferred to v1.1" (no open critical)

Post-tag actions:
- [ ] `npm publish @aphrodite-design/example-banchan@1.0.0 --access public`
- [ ] Announcement: README hero rewrite, blog post (if blog exists), Disquiet / Geeknews / X post
- [ ] Public roadmap update — what's in v1.1

---

## 6. Schedule estimate

Working assumptions: solo developer + AI pair (this session pattern), z.ai cost-aware. Effort listed in working days; calendar time can be 1.5-2× depending on context-switch tax.

| Phase | Days | Calendar | Notes |
|---|---|---|---|
| **Pre-work**: npm org register + placeholder publish | 0.5 | 1 day | user-owned |
| **Gate 1**: SSR test harness + remediation + 3 green runs | 5 | 7-10 days | longest; component bugs may extend |
| **Gate 2**: publish workflow + rename verb + namespace swap | 2 | 3 days | runs in parallel with Gate 1 tail |
| **Gate 3**: docs.aphrodite.dev | 4 | 6-8 days | can start in parallel after Gate 1 day 2 |
| **Beta phase**: tester recruit + cycle | (passive) | 14-28 days | gating by external responsiveness |
| **1.0 cut + release** | 1 | 2-3 days | tag + announce + monitor |
| **Total** | ~12 dev-days | **~6-8 weeks calendar** | dominated by beta wait |

If beta is shortened to 1 week + 2 testers, calendar drops to ~4 weeks. Recommendation: do not shortcut beta — that's the entire point of the discipline.

---

## 7. Risks + mitigations

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| `@aphrodite-design` org gets squat-claimed before registration | Low | Medium | Register today; fallback list ready (`@aphrodite-kit`, `@aphrodite-system`, `@aphrodite-dev`) |
| Multiple components fail SSR hydration | Medium | High | Component ejection rule — remove and defer; don't block 1.0 on every component |
| External beta testers don't respond | Medium | High | Lower bar to 3 testers; relax exit criteria to ≥ 1 successful deploy + ≥ 5 written feedback items if 3 testers infeasible |
| z.ai API instability during beta | Low | High | Document fallback to `anthropic` provider in adopter docs; keep `composer_model` override path tested |
| Korean designer / FE adoption tepid because they prefer Toss internal kit | Medium | Medium | Position Aphrodite as 0-1 / side-project / new-service tool, not internal-DS replacement |
| Component count perceived as too low vs Material-UI (130+) | Medium | Low | Roadmap page explicitly shows 70 → 100+ in v1.1; ship what we have |
| `aphrodite eval` sweep takes hours and blocks iteration | Low | Medium | Eval verb already supports per-brand subdirectories; can run subset for fast feedback |

---

## 8. Open questions (parking lot — answer before tagging 1.0)

1. **Who maintains `@aphrodite-design` org membership long-term?** Single owner (kwakseongjae) is a single-point-of-failure. By v1.1 add 1-2 maintainers.
2. **Provenance / 2FA on npm publish?** Recommended: enable 2FA + `--provenance` flag on publish workflow for supply-chain credibility.
3. **`docs.aphrodite.dev` DNS — who registers?** Same user account; if a project-org Github account is created later, DNS migrates.
4. **License**: currently Apache 2.0. Confirm before 1.0 publish — Apache 2.0 is OSS-safe + permissive for the 0-1 adopter use case.
5. **Telemetry**: Do we instrument `aphrodite create` runs (anonymous count, success/fail, time) for post-launch dashboard? Decide before 1.0; current default is zero telemetry.
6. **Korean naming for the canonical evaluator brand**: Banchan is fictional. Some Western adopters might find it opaque. Options: keep as cultural marker, or rename to `@aphrodite-design/example-starter`. Decide before publishing canonical version.

---

## 9. Decisions log (this document is the source of truth)

| Date | Decision | Source |
|---|---|---|
| 2026-05-19 | Adopt 3-gate pipeline: SSR → npm → docs | Ouroboros Seed `seed_778d10b9444b` |
| 2026-05-19 | Package architecture: Model A with N=1 test surface | Ouroboros interview turn 2 |
| 2026-05-19 | npm namespace: `@aphrodite-design` (fallback `@aphrodite-kit`, `@aphrodite-system`) | npm registry probe; `@aphrodite` squat-held |
| 2026-05-19 | **CORRECTION**: published 1.0 artifact is the CLI itself (`@aphrodite-design/aphrodite-agent`), not a pre-emitted component package. The Banchan brand is internal SSR-fixture only. | Second Ouroboros interview `interview_20260519_064346` |
| 2026-05-19 | Distribution model: npm wrapper (~5KB postinstall) + cargo + Homebrew. Binary in GitHub Releases per platform, never bundled in npm tarball. | Same interview, turn 1 |
| 2026-05-19 | Beta = macOS arm64 + x86_64 only. Linux x64 = 1.0 gate. Windows x64 = 1.0 if green, else 1.0.1. | Same interview, turn 2 |
| 2026-05-19 | Beta success = mechanical (1-7) + ≥1 tester adoption (8). 4-week hard cap. Mid-cycle W1/W2/W3/W4 checkpoints. | Same interview, turn 3 |
| 2026-05-19 | API key policy: self-signup only, NO shared credit pool. Document cost ~$0.10/run. | Same interview, turn 5 |
| 2026-05-19 | Telemetry default OFF, opt-in. Privacy-clean payload. | Same interview, turn 5 |
| 2026-05-19 | Add `npm test` (Vitest + JSDOM smoke) to every emit BEFORE beta-1 ships. | Same interview, turn 4 |
| 2026-05-19 | Version progression: `0.9.0-beta.N` → `1.0.0-rc.1` → `1.0.0` | Same interview, turn 3 |
| 2026-05-26 | **Superseded**: product line is `1.0.0` from first public cut. Progression `1.0.0-beta.1` → `1.0.0-beta.N` → `1.0.0-rc.1` → `1.0.0`. No `0.x` framing. | User directive |
| 2026-05-19 | Test layers in 1.0 SSR gate: build + hydration + themed-render (defer interactive + axe + pixel-baseline) | Ouroboros interview turn 3 |
| 2026-05-19 | Component ejection rule: failed component is removed, not patched | Ouroboros interview turn 3 |
| 2026-05-19 | Beta requires ≥ 3 external testers + ≥ 1 real deploy | This document, derived from user's stated discipline |
| 2026-05-19 | Banchan kept as canonical evaluator brand (fictional Korean food subscription) | This document, parking-lot question 6 |
| 2026-05-19 | RTL, true i18n, components 70→100+, Figma bidirectional sync, Lighthouse CI enforcement: deferred to v1.1 | Ouroboros interview turn 1 (gap ranking) |

---

## 10. Living document

This roadmap is the source of truth for any "what's next" question on Aphrodite v1.0 work. Any deviation from this plan must:

1. Surface as an issue or PR description referencing the section being deviated from
2. Note whether the deviation closes or opens an Ouroboros Seed acceptance criterion
3. If material (changes a gate's exit criteria or architecture decision), re-run an Ouroboros interview turn and update the Seed reference at the top of this file

Drift detection: weekly check that the alpha sweep (`aphrodite eval sweep-1.0.json`) still passes. If a regression appears with no obvious cause, the relevant gate is re-opened until investigated.
