# ADR 0002 — DESIGN.md (Google Labs spec) as the canonical design contract

Date: 2026-05-13
Status: Accepted (revised 2026-05-13 post-seed — Stitch interop killed)

## Context
We need a single, agent-readable design system format that survives across Stitch, Figma, code (Tailwind / CSS variables / native), and 3D. Candidates:
- Google Labs `DESIGN.md` (open spec, Apr 2026 open-sourced from Stitch).
- Figma Tokens / W3C Design Tokens Community Group format.
- Style Dictionary.
- Roll our own.

## Decision
Adopt **DESIGN.md** as the canonical contract. Convert to/from Figma Tokens and Style Dictionary at the edges via `aphrodite design emit`. Claude Code adoption is in progress (anthropics/skills#1008). Markdown-native format means LLMs read and write it natively.

**Revision 2026-05-13 (post-seed):** Stitch was previously a co-adopter of DESIGN.md and we contemplated a Stitch ↔ Aphrodite round-trip. The seed interview killed that path — Stitch has no programmatic API, the Playwright-bridge approach was rejected, and the v0.1 generation path is `Claude + DESIGN.md + Tailwind` direct. DESIGN.md remains the contract; we just no longer have Stitch as a partner that round-trips it. Figma Tokens / Style Dictionary interop is retained.

## Consequences
- **+** Already supports YAML frontmatter tokens + prose rationale (machine + human).
- **+** Validation tooling already exists upstream.
- **+** LLMs read/write Markdown natively; no DSL adoption cost.
- **−** Spec is "alpha" — schema may shift. Mitigate: pin to a `version:` field; track spec releases.
- **−** Less expressive than full Figma Tokens for very deep multi-mode theming. Mitigate: v0.1 ships *full multi-mode* (light + dark + multi-brand) via the `metadata` escape hatch the spec already allows — this is now a load-bearing requirement (see seed `acceptance_criteria`), not optional.
- **−** Lost Stitch as a round-trip partner; we no longer get a free first-draft generator. Mitigate: the v0.1 generator path (Claude + skills + DESIGN.md context + Tailwind) is owned end-to-end inside Aphrodite.

## Required minimums for Aphrodite-emitted DESIGN.md
- `name`, `version`, `description` frontmatter.
- `primary` color palette (spec requirement).
- All eight ordered sections, even if some are stubs initially.
- **Multi-mode mandatory from v0.1**: light + dark + at least two brand variants, each independently WCAG-AA-validated.
- Tailwind v4 emitter as the default code target.
