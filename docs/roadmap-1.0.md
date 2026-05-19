# Aphrodite v1.0 Roadmap

> **Status**: Active planning artifact.
> **Source**: Ouroboros Seed `seed_778d10b9444b` (Interview `interview_20260519_051320`, 2026-05-19, ambiguity 0.17).
> **Current**: v1.0 RC.9 (commit `bec04cd`), 78 tests, 70 React components, 5/5 cross-brand alpha verified (Banchan/Hada/Junjong/Yeoreum/Sori).
> **Target**: v1.0.0 stable cut.

This document is the immutable plan. Every PR toward 1.0 must reference a section here. Drift means re-running the Ouroboros interview cycle to update the Seed before continuing.

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

### Gate 2 — npm-published evaluator package

**Why second**: gates the "install and look" path. Without an npm-installable package, the docs site has nothing to point evaluators to.

**Pre-work (user-owned)**:
- [ ] Register `aphrodite-design` org on npm (kwakseongjae account).
- [ ] Placeholder-publish reserved names: `@aphrodite-design/example-banchan@0.0.0`, `@aphrodite-design/cli@0.0.0`, `@aphrodite-design/react@0.0.0`.

**Engineering work**:

1. **`aphrodite publish --rename @org/name [--dry-run]` CLI verb**
   - Reads the generated `react/` directory.
   - Substitutes `@aphrodite/<old-kebab>` → `@<org>/<new-name>` in `package.json` name + README badges + CHANGELOG seed footer + any Aphrodite-self URL references.
   - Invokes `npm publish --access public`.
   - `--dry-run` flag prints what would change without executing.
   - Estimate: 1 day.

2. **Aphrodite-self publish workflow**
   - New `.github/workflows/example-banchan-publish.yml` at repo root.
   - Triggers on aphrodite-cli minor tags (e.g. `cli-v1.0.0`).
   - Steps: install latest aphrodite-cli → `aphrodite create "<canonical Banchan intent>"` → `cd react && npm publish --access public --provenance`.
   - Token: `NPM_TOKEN` secret with publish scope on `@aphrodite-design`.
   - Estimate: 0.5 day.

3. **README / docs / CLI default updates**
   - Replace every `@aphrodite/` reference with `@aphrodite-design/` across `crates/`, `README.md`, `CHANGELOG.md`, emitted templates.
   - Add an "Install" section to top-of-README pointing at `@aphrodite-design/example-banchan`.
   - Estimate: 0.5 day.

**Exit criteria (falsifiable)**:
- [ ] `npm i @aphrodite-design/example-banchan@<version>` succeeds from a fresh shell with no aphrodite repo cloned
- [ ] Installed package contains all 70 components + tokens.ts + styles.css
- [ ] Peer-dep declarations correct: `react: ">=18"`, `react-dom: ">=18"` as `peerDependencies`
- [ ] No `dependencies` block in published package.json (we ship CSS keyframes, zero runtime deps)
- [ ] `aphrodite publish --rename @testorg/testbrand --dry-run` correctly shows substitutions without executing
- [ ] First external evaluator does `npm i @aphrodite-design/example-banchan` and reports success (counts toward beta gate too)

### Gate 3 — `docs.aphrodite.dev` public docs site

**Why third**: gates the "look and decide" path. Without searchable per-component docs with live demos, evaluators bounce in <30 minutes.

**Scope decision — separate repo or in-tree?**

Recommendation: **separate repo `aphrodite-docs/`** for clean deployment (Vercel/Netlify), but generated by Aphrodite itself via a new mode of the `docs` CLI verb. Specifically:

- New verb mode: `aphrodite docs --site` emits a multi-page Next.js or Astro site to `docs-site/`
- Existing per-run `docs/index.html` mode kept (it ships inside every adopter's emit)
- The site repo's CI subscribes to aphrodite-cli releases and re-builds

**Content scope for 1.0**:
- Landing / hero with one-line value prop + install snippet
- 70-component catalog page (search box → filtered list)
- Per-component page: short description + live iframe demo (loaded from canonical example-banchan via UMD or jsdelivr) + escapable JSX snippet + props table
- "Getting started" page: `npm i @aphrodite-design/example-banchan` + `<Button>` import example + 3 most-common patterns (signup form, pricing tiers, FAQ)
- "Themes" page: brand-token switcher (light/dark/brand-a/brand-b + Banchan/Hada/Junjong/Yeoreum/Sori reference brand previews)
- "Examples gallery": 5 cross-brand alpha screenshots (Banchan home, Hada signup, Junjong issues, Yeoreum flavors, Sori home) with the intent strings that produced them
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
- [ ] Install snippet on home is `npm i @aphrodite-design/example-banchan` (matches Gate 2)
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

**Entry condition**: all 3 gates show ✓ on the checklist above + at least 1 internal full cross-brand sweep with mean Quality ≥ 95.

**Beta scope**:
- Recruit 3–5 external alpha testers (real Korean designer + FE pairs).
  - Channel suggestions: existing personal network at Toss / Karrot / Naver / Kakao / Coupang frontend communities, Disquiet, Geeknews, post in r/typescript or Korean dev Twitter.
  - Compensation: free public credit + early-adopter Discord access; no equity / payment for 1.0.
- Each tester runs `aphrodite create` on a real brand they're working on or imagining, then attempts to:
  1. Install `@aphrodite-design/example-banchan` (or their renamed package) into a Next.js project
  2. Browse `docs.aphrodite.dev` for ≥ 30 minutes
  3. Deploy at least one page to a public URL (Vercel/Netlify)
- Feedback intake via:
  - GitHub Issues template `BETA-FEEDBACK`
  - Direct DM channel (Discord or Slack)
  - 1 mandatory 30-min interview per tester

**Beta exit criteria (falsifiable)**:
- [ ] ≥ 3 external testers complete the full flow (install → docs → deploy)
- [ ] At least 1 tester deploys to a real public URL we can link to
- [ ] All P0 / P1 bugs filed during beta are closed (P2 / P3 may roll to v1.1)
- [ ] No new critical SSR regression introduced
- [ ] Beta cycle duration: minimum 2 calendar weeks, maximum 4

### 1.0 launch criteria (final, falsifiable)

Tag `v1.0.0` ONLY when ALL of these hold simultaneously:

- [ ] Gate 1 SSR: all 70 components green on 3 frameworks × 3 consecutive CI runs
- [ ] Gate 2 npm: `@aphrodite-design/example-banchan@1.0.0-rc.X` published, installable, peer-deps correct
- [ ] Gate 3 docs: `docs.aphrodite.dev` live with full catalog + search
- [ ] Beta phase passed (≥ 3 external testers, ≥ 1 real deploy, all P0/P1 closed)
- [ ] CHANGELOG documents every breaking change since RC.9
- [ ] `aphrodite eval` sweep on 5 reference brands: mean Quality ≥ 95, zero Failed
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
