# SSR test harness — Gate 1 of v1.0 roadmap

Runs every emitted React component through three SSR frameworks and
asserts: builds clean, hydrates without warnings, themed colors reach
the DOM.

See `docs/roadmap-1.0.md` § "Gate 1" for the contract.

## Layout

```
tests/ssr/
├── next-app/             # Next.js 14 App Router scaffold (primary)
│   ├── app/
│   │   ├── layout.tsx
│   │   ├── page.tsx       # index — links to per-component routes
│   │   └── c/[name]/page.tsx  # dynamic route — one page per component
│   └── package.json
├── vite-app/             # Vite + React (added after Next.js green)
├── remix-app/            # Remix (added last)
├── playwright/
│   ├── playwright.config.ts
│   └── component-smoke.spec.ts
├── scripts/
│   ├── prepare-fixture.sh    # re-emit @aphrodite-design/example-banchan
│   └── run-matrix.ts         # orchestrate framework × component matrix
└── fixtures/
    └── banchan-react/        # generated, gitignored, file: linked
```

## How to run locally

```bash
# 1. Regenerate the canonical Banchan fixture (writes to fixtures/)
bash tests/ssr/scripts/prepare-fixture.sh

# 2. Install + build the Next.js app
cd tests/ssr/next-app
pnpm install
pnpm build       # ← Gate 1 layer 1: must exit 0
pnpm start       # serves on http://localhost:3000

# 3. Run Playwright smoke spec (in another shell)
cd tests/ssr/playwright
pnpm install
pnpm exec playwright test
```

## CI

GitHub Actions job `ssr-matrix` in `.github/workflows/ssr-matrix.yml`
runs this on every PR. Exits non-zero if any of the three layers fails
on any component. 3 consecutive green runs gate 1.0 release tag.

## Component ejection

If a component fails any layer after one remediation attempt, it is
REMOVED from the 1.0 catalog. Track ejections in `docs/component-status.md`.
