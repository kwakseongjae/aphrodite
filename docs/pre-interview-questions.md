# Pre-Ouroboros Interview — Crystallization Questions

Once `ooo interview` is live, Ouroboros will run its own Socratic loop. These are the high-leverage ambiguities **we already know exist** — answer them once here, and the official interview will converge much faster.

## A. Scope & identity

**A1.** Is "UI" strictly screen UI (web, mobile, desktop apps), or does it include presentation decks, marketing motion, 3D product visualizations, and AR/VR scenes? *(answer drives whether Blender + HyperFrames are core or optional)*

**A2.** Who is the primary caller — a **human at the terminal** typing `aphrodite`, or an **agent over MCP**? Pick one as the design north; the other gets best-effort.

**A3.** Does Aphrodite ever ship its *own* code into the user's repo, or only design artifacts (DESIGN.md, Figma file, component snippets) that the user / another agent lands? *(answer drives whether we need full repo-write authority or just artifact-export)*

## B. Distribution & language

**B1.** TypeScript-primary with Python sidecars (recommended), or Python-primary to match Ouroboros's ecosystem? Strong reason to override: if your team's day-to-day is Python.

**B2.** License: Apache-2.0 (recommended — matches HyperFrames, OpenHarness, friendly to enterprise) or MIT?

**B3.** Project name: stick with "Aphrodite", or prefix like the others — `oh-my-aphrodite`, `open-aphrodite`? *(naming convention signals lineage but is loud)*

## C. Auth & providers

**C1.** Day-1 provider set — confirm: OpenAI (OAuth), Moonshot/Kimi (OAuth), Google Gemini (OAuth), Anthropic (API key). Add Mistral? Local Ollama for the aesthetic jury?

**C2.** Subscription-reuse aggressiveness — should Aphrodite *automatically* read `~/.opencode/`, `~/.claude/`, `~/.config/codex/` to bootstrap auth, or only when the user opts in?

**C3.** Where does the OAuth callback land — `localhost:PORT` (standard), or a hosted proxy at `auth.aphrodite.dev` (smoother UX, but needs DNS / a domain)?

## D. DESIGN.md depth

**D1.** Do we need *multi-mode* DESIGN.md from day one (light/dark, dense/comfortable, brand-A/brand-B), or single-mode only?

**D2.** Code emit targets at v1 — confirm Tailwind v4 + CSS variables. Add Swift / Jetpack Compose / Flutter? *(every emitter is real maintenance cost)*

**D3.** Should the validator be *blocking* (refuse to compose UI until DESIGN.md is valid) or *advisory*?

## E. The aesthetic jury

**E1.** What does "undeniably beautiful" measure as? Options:
- (i) LLM jury (multi-model consensus, expensive)
- (ii) Visual-regression against a curated reference set
- (iii) Heuristic scoring (typography rhythm, contrast, whitespace ratios)
- (iv) Some weighted combination

**E2.** Is the aesthetic gate **mandatory** for every artifact, or **advisory** with a `--strict` flag?

**E3.** Do we ship reference sets per skill ("brutalist", "editorial", "glassmorphic")? Where do reference images come from licensing-wise?

## F. Stitch integration

**F1.** Stitch has no public API today. Options:
- (i) Playwright/Chromium bridge (works now, fragile)
- (ii) Wait for official API
- (iii) Skip Stitch initially; use Claude + DESIGN.md + Tailwind to generate UI directly

Recommended: (iii) for v0.1, (i) added in v0.2 behind feature flag.

## G. Persistence & sharing

**G1.** Per-project `.aphrodite/` directory committed to git: what *exactly* lives in it? Proposed: `DESIGN.md`, `decisions/*.md`, `policy.yaml`. Auth, caches, event store: never.

**G2.** Does Aphrodite have a global "memory" of the user's aesthetic preferences across projects, or is each project an island?

## H. The "first vertical slice" definition

**H1.** What single command should work end-to-end at v0.1?
- Candidate: `aphrodite design "a calm, editorial landing page for a longevity clinic"` → emits `DESIGN.md` + a single hero HTML file + opens it in browser. No Stitch, no 3D, no motion.

**H2.** What is the v0.1 demo we will record to prove the project lives? (One 60-second screencast worth filming.)

## I. Governance

**I1.** Public from day one, or invite-only / private until v0.1? Recommended: public empty repo + README from day one (signals intent, attracts contributors), code dropped at v0.1.

**I2.** Solo project, or do you want a CONTRIBUTING.md tuned for early collaborators?

**I3.** Roadmap visibility: GitHub Projects board (public) or private until v0.1?
