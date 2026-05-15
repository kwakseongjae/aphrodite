# Aphrodite — Evolution & Methodology

> A meta-document on what got built across the v0.3 sprint, how feedback shaped each decision, and the direction the project is converging toward.

## The 19-pass arc

The development sprint covered 2026-05-13 → 2026-05-15 across **19 dogfood passes**, each closing with a testimony in `docs/agent-eval/` and an archive of artefacts in `docs/agent-eval/archive/`. The arc breaks into four phases:

### Phase A — Foundations (Pass 1-6)
- **Pass 1** established the contract: offline deterministic generator producing valid DESIGN.md across 8 intents. No LLM credential needed; the contract gate is the point.
- **Pass 2-3** introduced the live z.ai/GLM-5.1 path, expanded to 17 intents across 5 orthogonal axes (style, domain, constraint, language, convergence). 17/17 valid after two same-day parser fixes.
- **Pass 4** added the surface composer — second LLM call producing the *real* page (pricing tiers, dashboard, mobile frame, magazine, portfolio) instead of the generic Pass 3 hero.
- **Pass 5/5b** verified the taste loop. Pass 5 ran into the first manifestation of Finding #20 (z.ai HTTP errors) but its directional evidence stood. Pass 5b's controlled A/B was the decisive proof: same intent + opposite pre-loaded preferences produced three independent axis flips on both sides. Sampling variance ruled out.
- **Pass 6** shipped `aphrodite refine "<delta>"`. Scripted 4-turn sequence proved surgical-edit fidelity: each delta lands on its named axis, earlier turns preserved.

### Phase B — Autonomy (Pass 7-10)
- **Pass 7** ran a *reactive* multi-turn — the agent reads each turn's output and authors the next delta in response. 4 turns × ~250 s = 1036 s total to land at Pass 7-final (Source Serif 4 + Outfit + 224 px + #faf9f6). This became the benchmark every later pass measures against.
- **Pass 8** introduced `aphrodite create` — the autonomous loop. Two runs surfaced two distinct failure modes (parser brittleness on null rationale; critic over-satisfaction without checklist). Run B reached 4/5 in 319 s vs Pass 7's 5/5 in 1036 s. The 1-point gap motivated Pass 9.
- **Pass 9** loaded the seeded `editorial-portfolio` skill into the design + critic system prompts. **0 refines, 286 s, full Pass 7 parity on all 4 measured axes.** The Newsreader pick was the smoking gun: the skill body listed three candidates; the generator landed on one of them.
- **Pass 10** shipped Phase 7 harmonize: Google Fonts `@import` auto-injection (with optical-sizing axis for variable-font families), hero typography hookup via CSS variables. `asset-standards` skill became `default: true` — Lucide icons / Fluent emoji / opinionated photography defaults applied to every run.

### Phase C — Personas (Pass 11-13)
- **Pass 11** introduced 7 seeded design authorities (Rams, Vignelli, Ando, Kawakubo, Sottsass, Hara, Galileo). Same Seoul furniture intent through Rams vs Sottsass produced visibly different work — Sottsass's 4-color Memphis palette is the smoking gun; Rams's single muted forest-green stays restrained. But the critic over-rode Sottsass's chunky display pick (Finding #34).
- **Pass 12** fixed the override: critic prompt now explicitly forbids citing scaffold over persona. Each persona declared a `cjk_strategy` field in its own voice. Re-run Sottsass: Druk Wide + Sandoll Noisuh CJK (lifted directly from cjk_strategy) survived. Accent palette named verbatim from Sottsass's `prefers` list.
- **Pass 13** ran 5 persona × intent pairings in parallel. **Spacing density inverted cleanly with persona register**: Galileo 96 → Vignelli/Hara 192 → Ando 224 → Kawakubo 256 px. The critic now quotes persona principles in its rationale. One transient z.ai 429 surfaced as Finding #29.

### Phase D — Polish + Research (Pass 14-19)
- **Pass 14** closed #29 with bounded retry (1s/2s/4s back-off on 429/5xx/connect/timeout). Embedded 10 canonical Lucide SVGs in the asset-standards skill. Dogfood: composer emitted semantically-correct path data (triangle-alert for adverse events, chart-line for time series) — class label stripped but path byte-perfect.
- **Pass 15** closed #31 (critic over-correction against suggestive lists) with prescriptive-vs-suggestive language rules in the critic prompt. Closed #32 with `harmonize::recover_lucide_classes` — fingerprint-based class injection backstop. Pass 15 Run A produced 7 properly-classed Lucide icons in semantic slots.
- **Pass 16** adopted **Karpathy's LLM-Wiki pattern** for design references after surveying Hermes / Karpathy research-skill options. 7 seeded entries (Pretendard / MUJI / Apartamento / Pentagram / Linear / Are.na / Naver). Six of seven Layout-section specs in the dogfood output traced directly to a loaded wiki entry. Zero LLM cost for retrieval.
- **Pass 17** shipped the wiki CLI (`list / show / add`). User-added entries immediately visible to the next create call.
- **Pass 18** added `--auto-fetch` — HTTP GET → extract `<title>`, meta description, `og:image`, palette hint hex values. Friction-to-ingest halved.
- **Pass 19** closed the last ADR 0004 stub: Phase 6 asset management. `<project>/.aphrodite/assets/{refs,uploads,icons}/` with FNV-1a-128 content-hash dedupe. Create flow auto-materialises wiki entries' og_images into refs/. **All 9 numbered phases functional.**

## How feedback shaped each decision

The same loop repeated every pass:

1. **Run the experiment** with a concrete intent + flag combination
2. **Read the output** — DESIGN.md frontmatter, composition.html structure, critic rationale, harmonize report, timing
3. **Identify the single most unsatisfying axis** (the critic's job; the human's job in reactive passes)
4. **Form a hypothesis** about what change closes the gap
5. **Ship the change** — code, skill body, prompt rule, new module
6. **Re-run with the SAME intent** to verify the gap closes
7. **Archive the result**, write the testimony, commit

Each finding traced back to a pass where the gap was visible:

| Finding | Pass surfaced | Pass closed | Mechanism |
|---|---|---|---|
| #20 wall-clock timeout | 5 | 6 | reqwest timeout config |
| #21 density extractor | 6 | 6 | broadened bucketing |
| #24 Google Fonts `@import` | 7 | 10 | harmonize injection |
| #26 hero typography hookup | 7 | 10 | CSS-var rewrite |
| #27 critic over-satisfaction | 8 | 9 | skill scaffold loading |
| #28 Cormorant-direction risk | 8 | 9 | skill encodes the rejection |
| #29 transient provider failures | 13 | 14 | retry with back-off |
| #30 warning false positives | 9 | 9 (same pass) | separate user_intent_for_warnings |
| #31 over-correction against suggestive lists | 14 | 15 | prescriptive/suggestive prompt rule |
| #32 Lucide class label stripped | 14 | 15 | in-prompt directive + fingerprint backstop |
| #34 critic ignores persona authority | 11 | 12 | critic prompt forbids scaffold-over-persona |
| #35 CJK glyph coverage in personas | 11 | 12 | `cjk_strategy` field per persona |

Carried (not yet closed):
- **#13** — brand-name palette recall (Linear's #5e6ad2 etc. not auto-recognised)
- **#25** — composition regenerates every refine (architectural, defer)
- **#36** — surface composer can't render Kawakubo-style anti-templated layouts

## Direction of convergence

Three independent threads converged on the same architecture:

### Thread 1 — *Generator → critic loop → harness*
The early v0.1 was "give the LLM an intent, get a DESIGN.md back." Pass 7 demonstrated that *reactive* multi-turn produced 5/5 results but required cognitive load on the caller. ADR 0004 reframed Aphrodite as an *autonomous creation harness*: the loop moves inside, the caller hands over the intent, the harness returns the finished thing. Pass 9-19 filled in the seven supporting phases (research, taste, design, self-critic refine, asset create/manage, harmonize, skillify).

### Thread 2 — *Knowledge: imagined → curated*
Pass 7's reactive critic was effectively reasoning from the LLM's *imagined* references. Pass 8 showed the cost: the autonomous critic, in isolation, judged in a vacuum and either over-corrected or under-corrected. The fix wasn't "tune the critic harder" — it was **load curated knowledge as the critic's checklist**. Skills (procedural workflows) and wiki entries (declarative references) became the architecture's compounding KB. Pass 16's Karpathy LLM-Wiki adoption made this explicit: *"compile knowledge once, query at use-time."*

### Thread 3 — *Voice: generic → opinionated*
The default LLM register, even with strong scaffolds, leans toward "tasteful" — restrained, modernist, monochrome. Pass 11 introduced personas as the *opinionated voice* layer. Same intent through Rams produced restraint; through Sottsass produced Memphis-color joyful chaos. Pass 12-13 made personas the *final authority* — outranking generic scaffolds in conflicts, declaring CJK strategies in their own voice, with the critic now quoting persona principles in its rationale.

These three threads — internalised loop, curated KB, opinionated voice — are not separable. The harness without curated knowledge produces vacuum judgments. The KB without an opinionated voice produces tasteful-average output. The voice without the internalised loop demands a human to police it.

## What the system looks like now

```
aphrodite create "<intent>" [--persona <slug>] [--max-turns N]
│
├─ phase 1   research          query ~/.aphrodite/wiki by intent tags
├─ phase 1a  skill loading     query ~/.aphrodite/skills (default-flag + tag-overlap)
├─ phase 1b  persona loading   load --persona slug from ~/.aphrodite/personas
├─ phase 2   taste application reads ~/.aphrodite/taste/preferences.toml
│
├─ phase 3   design            LLM call with augmented intent + scaffolds
│
├─ phase 4   self-critic loop  while satisfaction < threshold && turn < max:
│                                critique → if delta proposed: refine + recompose
│
├─ phase 5   asset create      composer emits Lucide path data (canonical SVGs in skill)
├─ phase 6   asset manage      materialise wiki og_images to <project>/.aphrodite/assets/
├─ phase 7   harmonize         inject Google Fonts @import; hookup hero typography;
│                                recover Lucide class labels by path-fingerprint
├─ phase 8   skillify proposal if trajectory non-trivial: propose new skill in JSON
│
└─ writes DESIGN.md, hero.html, composition.html; auto-commit if .git present;
   single Accept/Regenerate taste event for the run as a whole
```

`aphrodite wiki` and `aphrodite personas` and `aphrodite assets` are the curation surfaces. `aphrodite refine` remains as the manual delta verb. `aphrodite design` remains for one-shot baselines.

## Methodology principles (extracted from 19 passes)

1. **Same-intent regression.** The Seoul-furniture-maker intent ran through Pass 7, 8, 9, 10, 14a, 15b, 16, 19. Every pass measured itself against the previous run on the *exact same input*. Variation between passes attributes to code/prompt changes, not intent drift.
2. **Two-failure-mode runs are gold.** Pass 8's two distinct failures (parser brittleness; critic over-satisfaction) in one experiment surfaced more learning than a single clean success would have.
3. **Smoking-gun evidence > aggregate metrics.** "Newsreader literally appears in the skill body's candidate list and the generator picked it" is more convincing than "satisfaction average improved by 0.07."
4. **Cargo-cult prevention as a primitive.** Every seed skill and wiki entry has a `What NOT to copy` section. The system's strongest pull is toward imitation; the documentation needs to push back explicitly.
5. **Prompt-level fixes before code-level fixes.** Findings #27, #28, #31, #34, #35 were all closed with system-prompt edits. Only #29 (transient errors), #32 (label recovery), #36 (composer registers) needed actual Rust code.
6. **Pure functions in core, HTTP at the edge.** `aphrodite-core` has no async, no reqwest. Fetching lives in `aphrodite-generator`. This kept tests fast (the core test suite runs in <50ms total).

## Where this goes next

- **MCP surface** (ADR 0002 still names two surfaces; only the CLI ships today). `aphrodite-mcp` needs to expose `create` as an MCP tool so Claude Code / Codex can hand off intents directly.
- **Wiki growth.** Seven seeded entries cover the basics; the real value is when the wiki has 50-100 entries that the user has personally curated. The auto-fetch path makes this realistic.
- **Persona expansion.** Seven is enough to demonstrate the system; the design canon has more (Paul Rand, Naoto Fukasawa, Charlotte Perriand, Aldo Bakker, Stefan Sagmeister) worth bundling.
- **Finding #36 — radical-register composer.** Kawakubo's Pass 13 result had exceptional DESIGN.md prose ("crop characters at viewport edges; it refuses to accommodate you") but the surface composer didn't translate to HTML. The composer's prompt biases toward conventional editorial templates.
- **Cost transparency.** `aphrodite create --estimate` should preview LLM call count + wall clock before incurring cost.
- **Public release prep.** The system is now structurally complete enough to merit a README rewrite, a CHANGELOG, and a public examples gallery.

## How to read this archive

- `docs/agent-eval/archive/journey.html` — the single-page narrative, all 19 passes with embedded previews
- `docs/agent-eval/archive/index.html` — pass-by-pass index linking to per-pass galleries
- `docs/agent-eval/2026-05-1<X>-<Nth>-agent-pass.md` — individual pass testimonies
- `docs/agent-eval/improvements.md` — append-only finding backlog
- `docs/adr/0004-autonomous-creation-harness.md` — the architectural decision the v0.3 sprint executed against

Git log from `99a574c` (Pass 7) to `db4ea59` (journey gallery) is the executable counterpart to this narrative.
