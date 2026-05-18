# Aphrodite

> *Hand a UI brief to the goddess of beauty herself.*
> *Get back a typed React package, multi-page site, design system, and Quality Score — in one command.*

[![v1.0 RC — Toss/Karrot-adoptable](https://img.shields.io/badge/v1.0_RC-Toss%2FKarrot_grade-a78bfa)](docs/agent-eval/archive/journey.html)
[![49 dogfood passes recorded](https://img.shields.io/badge/dogfood-49_passes-9333ea)](docs/agent-eval/archive/journey.html)
[![42 typed React primitives](https://img.shields.io/badge/react-42_components-61dafb)](crates/aphrodite-generator/src/react_export.rs)
[![Figma Tokens Studio sync](https://img.shields.io/badge/figma-tokens_studio_sync-f24e1e)](crates/aphrodite-generator/src/figma_sync.rs)
[![docs site auto-emit](https://img.shields.io/badge/docs-auto_generated-22c55e)](crates/aphrodite-generator/src/docs_site.rs)
[![13 design authorities](https://img.shields.io/badge/personas-13-fcd34d)](crates/aphrodite-core/seed-personas/)
[![Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-57575c)](LICENSE)

## What you get from one `aphrodite create` (v1.0 RC)

```
<your-project>/
├── DESIGN.md                # Google Labs alpha schema, 4 brand variants, WCAG-AA validated
├── home.html                # primary page
├── pricing.html             # --pages secondary
├── about.html               # --pages secondary
├── sitemap.xml              # auto-listed
├── home-{mobile,tablet,desktop}.png    # auto-screenshot at 3 viewports
├── pricing-{mobile,tablet,desktop}.png
├── about-{mobile,tablet,desktop}.png
├── tokens.css               # CSS variables, scoped per variant
├── tokens.json              # Style Dictionary shape — value + type per leaf
├── components.html          # Storybook-style designer preview
└── react/                   # 🆕 publishable npm package
    ├── package.json         # @aphrodite/<name>, peer deps react ≥18
    ├── tsconfig.json
    ├── README.md
    └── src/
        ├── index.ts         # barrel export
        ├── tokens.ts        # typed const + VariantName union
        ├── cn.ts            # class-name helper
        ├── styles.css       # base styles + CSS-var scopes
        ├── Button.tsx       # forwardRef, variant/size/loading
        ├── Input.tsx        # error state, aria-invalid wired
        ├── Tag.tsx          # tone neutral/success/warning/danger
        ├── Avatar.tsx       # src OR initials, three sizes
        ├── Card.tsx
        ├── Modal.tsx        # Escape close, aria-modal
        ├── Drawer.tsx       # right-side, Escape close
        ├── Skeleton.tsx     # shimmer animation
        ├── FormField.tsx    # auto-wires id+aria to its child control
        ├── Switch.tsx       # role=switch, aria-checked
        ├── Badge.tsx        # count overflow + dot
        ├── Spinner.tsx      # role=status, three sizes
        └── *.stories.tsx    # Storybook CSF3 stories for every component
```

Plus a one-line **Aphrodite Quality Score** at the end of every run (`a11y / mobile / perf / semantic`, 0-100 each) so you can pin a CI gate.

A Toss/Karrot frontend engineer can `cd react && npm publish` and other projects can `import { Button } from "@aphrodite/<your-project>"`.

Aphrodite is an **open, model-agnostic UI generation harness**. It is not a Claude Code plugin. It is a standalone runtime that any human or AI agent can call to get authentic, production-grade, *beautiful* user interfaces — by orchestrating today's UI tooling under a single **DESIGN.md**-grounded design contract.

It is positioned as a **standalone product**, comparable to Hermes Agent (general agent canon) or Ouroboros (Seed-driven loop runtime) — but scoped *narrowly* and *deeply* to UI generation.

## Why Aphrodite, not an LLM directly?

The headline difference: Aphrodite **compiles** designs, it doesn't just *advise* on them.

| Use case | LLM chat | Aphrodite |
|---|---|---|
| "What font pairs with serif display?" | great | not the right tool |
| "Generate a landing page for a Seoul furniture maker" | OK-ish — 1 page, 0 validation, 0 reproducibility | a 4-variant DESIGN.md + WCAG-AA-validated composition.html + hero.html, auto-committed, with a *learned* skill on disk so next time is faster |
| "Use Dieter Rams's voice" | "OK, here's a more minimal version…" — vague | one of 13 hand-curated PERSONA.md authorities, declared principles + rejects + cjk_strategy, critic enforces the persona over scaffolds |
| "Use these reference sites" | doesn't remember next session | Karpathy LLM-Wiki pattern — curated wiki entries compound across runs, query at create-time by intent tag overlap |
| "I don't like that — try again" | starts from scratch | TastePreferences accumulate; same intent next time avoids the rejected direction |

The same property that makes Hermes loved (skills + memory + curator) is what makes Aphrodite trustworthy: **nothing the LLM learns is lost between runs**.

## What works today (v0.3)

A single command runs the autonomous creation harness — research → taste → skill scaffolds → persona authority → design → self-critic refine → harmonize → asset management → optional skillify proposal — and writes the finished artifacts to your repo:

```bash
$ aphrodite create "a portfolio site for an independent Seoul furniture maker working in solid walnut"
● phase 1 / loaded skill scaffolds: [asset-standards, editorial-portfolio] for tags [portfolio, furniture, solo-maker]
● phase 1 / loaded reference materials: [apartamento, are-na, pentagram]
● phase 3 / design …
  ✓ design in 251s (provider=zai, model=glm-5.1)
● phase 4 / self-critic refinement (max_turns=3, threshold=0.85)
  → turn 1 / critique …
    satisfaction=0.88  axis=—  rationale=""
● phase 7 / harmonize: injected fonts [Newsreader, Outfit]; hero typography hooked up
● done: 0 turns, satisfaction=0.88, total=251s, llm_calls=3
```

Output: `DESIGN.md` (Google Labs alpha schema with 4 variants — light / dark / brand-a / brand-b, all WCAG-AA), `composition.html` (intent-specific real surface), `hero.html` (4-variant showcase). Auto-commits to the calling repo's git tree by default.

## Surfaces

| Surface | Binary | Purpose |
|---|---|---|
| CLI | `aphrodite` | Human-driven; also sub-shell-callable |
| MCP | `aphrodite-mcp` | JSON-RPC 2.0 over stdio — for Claude Code / Codex / Hermes / OpenCode hosts |

The MCP server exposes 5 tools: `create` (autonomous), `design` (one-shot), `redesign` (with regenerate signal), `validate`, `auth_status`. Register it in your host's `.mcp.json` and the host's model gets the same execution path the CLI uses.

## Verbs

### Creation
- `aphrodite create "<intent>" [--persona <slug>] [--max-turns N]` — autonomous create harness
- `aphrodite design "<intent>"` — one-shot DESIGN.md + hero (no refine loop)
- `aphrodite refine "<delta>"` — apply a single named delta to the current DESIGN.md
- `aphrodite redesign "<intent>"` — `design` with implicit Regenerate signal

### Curation
- `aphrodite personas` — list 10 bundled design authorities (Rams, Vignelli, Ando, Kawakubo, Sottsass, Hara, Galileo, Paul Rand, Charlotte Perriand, Naoto Fukasawa)
- `aphrodite wiki list / show <slug> / add <url> [--auto-fetch]` — design-reference wiki (Karpathy LLM-Wiki pattern, 11 seeded entries)
- `aphrodite assets list / clean` — inspect / clean `<project>/.aphrodite/assets/`
- `aphrodite love` / `aphrodite hate` — record explicit taste signal on the most recent run
- `aphrodite prefs` — show accumulated taste preferences

### Operations
- `aphrodite init` — guided first-run wizard
- `aphrodite auth set <provider>` — store API key in OS keychain
- `aphrodite doctor` — health-check config + keychain + env
- `aphrodite gallery <dir>` — build gallery.html for a directory of runs

## Persona-driven design

Same intent, different persona — visibly different work:

```bash
$ aphrodite create "a portfolio site for a furniture maker working in walnut" --persona dieter-rams
  → Instrument Serif + Inter + single muted forest-green accent (#5a8a45)
  → 192 px max spacing, rectilinear, no decoration

$ aphrodite create "a portfolio site for a furniture maker working in walnut" --persona ettore-sottsass
  → Druk Wide + Sandoll Noisuh CJK + 5-color Memphis palette
    (terracotta + emerald + citron + lapis + mint)
  → 224 px max spacing, asymmetric layouts, terrazzo patterns
```

The persona body declares principles, rejects, prefers, and a CJK strategy in its own voice. The critic respects persona > scaffold authority — refining away from a persona-driven pick requires quoting the specific principle being violated.

## Compounding design-reference wiki

Curate once, query at every `create` call:

```bash
$ aphrodite wiki add https://stripe.com --tags "saas,landing,payments" --auto-fetch
  → fetching https://stripe.com …
  ✓ title=Stripe | Financial Infrastructure to Grow Your Revenue
  ✓ desc=Stripe is a financial services platform…
  ✓ og image=stripeassets.com/...Stripe.jpg
  ✓ palette hints=#031323, #4285f4, #34a853, #fbbc04, #ea4335, #000

$ aphrodite create "a landing page for a small B2B payments fintech"
  ● phase 1 / loaded reference materials: [linear-app, stripe]
  ● phase 6 / asset manage: materialised 1 reference image(s) into .aphrodite/assets/refs/
```

The next `aphrodite create` that matches the wiki entry's tags lifts pattern fragments directly into the design — Stripe's `#000` near-black, Apartamento's `4:5 portrait photography ratio`, Pentagram's `12-column / 24 px gutter`. Six of seven specifications in a typical output trace to a wiki entry; the LLM doesn't *imagine* references, it absorbs them from concrete prior art.

## Architecture

ADR 0004 (`docs/adr/0004-autonomous-creation-harness.md`) reframes Aphrodite as an autonomous creation harness with 9 numbered phases, all functional in v0.3:

```
aphrodite create
│
├─ phase 1   research          ~/.aphrodite/wiki/  (Karpathy LLM-Wiki)
├─ phase 1a  skill loading     ~/.aphrodite/skills/
├─ phase 1b  persona loading   ~/.aphrodite/personas/ (with --persona)
├─ phase 2   taste application ~/.aphrodite/taste/preferences.toml
├─ phase 3   design            LLM call with augmented intent
├─ phase 4   self-critic loop  while sat < threshold && turn < max
├─ phase 5   asset create      composer emits Lucide path data + classes
├─ phase 6   asset manage      <project>/.aphrodite/assets/refs/ dedupe
├─ phase 7   harmonize         @import injection + hero token hookup +
│                                Lucide class recovery by path fingerprint
└─ phase 8   skillify proposal new skill draft if trajectory non-trivial
```

Each phase has a separate testimony in `docs/agent-eval/`. The 19-pass journey is at `docs/agent-eval/archive/journey.html` (every pass with embedded composition preview). See `docs/evolution.md` for the methodology + how feedback shaped each decision.

## Install (from source)

```bash
git clone <repo> && cd aphrodite
cargo install --path crates/aphrodite-cli   # gives you `aphrodite`
cargo install --path crates/aphrodite-mcp   # gives you `aphrodite-mcp`
```

## Pick your provider (priority: z.ai → Anthropic → OpenRouter → offline)

```bash
# z.ai GLM Coding Plan (recommended for now — cheapest, Anthropic-compatible)
aphrodite auth set zai
# or headless:
APHRODITE_ZAI_API_KEY=... aphrodite create "…"

# Direct Anthropic:
aphrodite auth set anthropic

# OpenRouter (covers everything else):
aphrodite auth set openrouter
```

No key? `aphrodite design` still works — falls back to a deterministic offline generator that emits a valid 4-variant DESIGN.md. Useful for CI.

## Hand it to your agent

Register `aphrodite-mcp` in your MCP-capable host's `.mcp.json`:

```json
{
  "mcpServers": {
    "aphrodite": { "command": "aphrodite-mcp" }
  }
}
```

The host's model now sees 5 tools. The headline one — `create` — takes `{ intent, persona?, max_turns?, satisfaction_threshold?, target_repo?, write_mode? }` and returns the same structured JSON the CLI emits.

Errors come back as structured `isError:true` envelopes with `kind` (auth_failed / rate_limited / provider_outage / target_repo_invalid / …) and `hint` fields so the agent can recover without escalating.

## Pillars

1. **Authentic agency.** Full-trust harness — a calling agent (or human) gets end-to-end authority to plan, design, and ship UI work. Default = direct commit to the caller's repo; `--no-write` emits to `.aphrodite/out/` instead. Deny-list policy gates only the catastrophic moves.
2. **Agent-first.** On any UX conflict, the JSON/MCP shape wins; the human CLI is a thin pretty layer over it.
3. **DESIGN.md as the contract.** Every project carries a Google-Labs-compatible `DESIGN.md` — the single source of truth shared across code, Figma, and 3D.
4. **Multi-mode from day one.** Every emitted DESIGN.md carries light + dark + ≥2 brand variants, all WCAG-AA-validated independently.
5. **Adaptive taste + compounding knowledge.** Three layers: TastePreferences (declarative facts that decay), Skills (procedural workflows that get patched), Wiki (curated references that compound). Personas sit above as opinionated authorities.
6. **Open source.** Apache-2.0. Inspired in form by Hermes Agent (memory / skills / curator patterns) and Karpathy's LLM-Wiki gist. Scoped narrowly to *UI beauty*.

## Non-goals

- Not a general coding agent. (Use OpenCode / Claude Code / Codex for that — Aphrodite can be invoked *by* them over MCP.)
- Not a Claude Code skill or plugin.
- Not a hosted SaaS. Local-first.

## Project layout

```
aphrodite/
├── DESIGN.md                          # Aphrodite's own visual identity (dogfood)
├── Cargo.toml                         # Rust workspace
├── crates/
│   ├── aphrodite-core/                # DESIGN.md model, validator, taste,
│   │                                    skills, personas, wiki, assets
│   │   ├── seed-personas/             # 10 bundled PERSONA.md authorities
│   │   ├── seed-skills/               # bundled SKILL.md scaffolds
│   │   ├── seed-wiki/                 # 11 bundled design-reference entries
│   │   └── src/
│   ├── aphrodite-cli/                 # `aphrodite` binary
│   ├── aphrodite-mcp/                 # `aphrodite-mcp` JSON-RPC stdio server
│   ├── aphrodite-generator/           # orchestrator + provider router + critic +
│   │                                    refine + surface composer + harmonize +
│   │                                    wiki_fetch
│   └── aphrodite-keyring/             # OS keychain abstraction
├── docs/
│   ├── adr/0001..0004                 # architectural decisions
│   ├── agent-eval/                    # 19-pass testimony archive
│   │   └── archive/journey.html       # single-page visual narrative
│   ├── evolution.md                   # how feedback shaped each decision
│   └── vision.md / architecture.md
└── .ouroboros/seeds/                  # immutable Ouroboros seed (v0.1 contract)
```

## Attribution

- **Skill + persona substrate** (markdown on disk + usage tracking) — patterns borrowed from [NousResearch/hermes-agent](https://github.com/NousResearch/hermes-agent) (`agent/curator.py`, `tools/skill_usage.py`, `agent/prompt_builder.py`)
- **Design-reference wiki** — Karpathy's [LLM Wiki gist](https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f), packaged as Hermes' `llm-wiki` skill
- **Icons** — [Lucide](https://lucide.dev/icons) (MIT)
- **Emoji** — Microsoft [Fluent UI Emoji](https://fluentemoji.com) (MIT)

## License

[Apache-2.0](LICENSE).
