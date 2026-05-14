# Agent Evaluation — 2026-05-14 — Eleventh Pass (persona system)

> Aphrodite now ships seven seeded persona authorities — Dieter Rams, Massimo Vignelli, Tadao Ando, Rei Kawakubo, Ettore Sottsass, Kenya Hara, Galileo Galilei. The user's case for personas was: "Aphrodite should have voices it recognises as authorities — the kind of people you'd hesitate to overrule." Pass 11 ships that and dogfoods two extreme registers (Rams vs Sottsass) against the same intent to verify they produce visibly different work.

## What got built

### `aphrodite_core::personas` (new module)

Mirrors `skills` shape but separate namespace — personas live at `~/.aphrodite/personas/<slug>/PERSONA.md`. Frontmatter:

```yaml
name: Dieter Rams
era: 1932–present, German industrial designer; chief designer at Braun 1961–1995
voice: terse, prescriptive, no patience for ornament
principles: [10 of them]
rejects: [9 categorical rejections]
prefers: [7 explicit choices]
when_to_invoke: dev tools, hardware products, B2B utility
```

Body contains a 200-300 word section in the persona's own register — biographical anchor + applied lessons. The `as_system_prompt_block` function renders the frontmatter + body as a single text block ready to prepend to LLM prompts with framing:

> "When persona principles conflict with generic skill scaffolds, the persona wins — but call out the conflict in your rationale."

### Seven bundled personas

| Slug | Authority | When | Voice fragment |
|---|---|---|---|
| `dieter-rams` | Modernist minimalism (Braun) | dev tools, B2B utility | "as little design as possible" |
| `massimo-vignelli` | Typographic discipline | editorial, signage | "you can design well with five typefaces" |
| `tadao-ando` | Architectural light + ma | spaces, contemplative | "architecture is the experience of light entering a space" |
| `rei-kawakubo` | Anti-fashion deconstruction | unmistakable, art-adjacent | "comfort is irrelevant to whether the work is true" |
| `ettore-sottsass` | Memphis postmodernism | joyful, anti-corporate | "design that makes you smile is doing more work" |
| `kenya-hara` | MUJI emptiness | contemplative, East Asian | "white is not a colour — it is receptivity" |
| `galileo-galilei` | First-principles measurement | scientific, data-dense | "measure what is measurable, and make measurable what is not" |

Galileo is the user's named example — slotted as the *first-principles physics-of-design* persona rather than the design canon. He demands measurable proportions, derived type scales, golden/root-2 ratios, and refuses any "best practice" invoked as authority.

### CLI surface

- `aphrodite personas` — list all installed personas with name, era, voice
- `aphrodite create "<intent>" --persona <slug>` — invoke a persona for one run

### Wiring into `create_cmd`

When `--persona` is set:
1. The persona's `as_system_prompt_block()` is prepended to the design call's intent (going *before* any skill scaffold).
2. The same block is passed as the critic's context (the field previously holding the scaffold), with scaffold scaffolds appended below it under a `---` separator.
3. The persona slug shows up in the JSON output's top-level `persona` field.

## Test design

Same intent that Pass 7/8/9/10 used (Seoul walnut craftsperson portfolio), run twice with **maximally-opposed** personas:

- **Run A: Dieter Rams** — modernist restraint, monochrome, grid, no decoration
- **Run B: Ettore Sottsass** — Memphis joy, saturated colour, pattern, ornament

If personas work, these should produce visibly different artifacts. If they don't, the persona system is decoration.

## Results

### Run A — Dieter Rams (306 s, 0 refines, satisfaction 0.91)

The critic accepted on turn 1 with rationale:
> "The design is coherent and functional: warm neutral palette keyed to walnut, correct typographic register with a contemporary serif, magazine-pacing spacing up to 192px, four well-differentiated variants, rectilinear forms matching the joinery discipline, no decorative noise, no SaaS filler, and no prose-token mismatches. It passes."

Tokens that landed:
- Display: Instrument Serif (restrained modern serif)
- Body: Inter
- Primary 500: `#5a8a45` (muted forest green — counterpoint to walnut)
- Max spacing: 192 px
- Single accent system; no decorative shapes

### Run B — Ettore Sottsass (594 s, 1 refine, satisfaction 0.87)

The critic at turn 1 emitted 0.35 with rationale:
> "Dela Gothic One is a chunky manga-style display face with no Korean character support — fundamentally wrong register for a Seoul-based studio that needs to present work in both languages, and contradicts the editorial-portfolio scaffold's requirement for a contemporary serif with optical sizing."

→ refined display family to Source Serif 4. Turn 2 critic emitted satisfaction 0.87, stop.

Tokens that landed:
- Display: Source Serif 4 (after refine) — **but the initial pick was Dela Gothic One**, a chunky Memphis-spirited display
- Body: Outfit
- Primary 500: **`#c44d12` terracotta**
- *Additional 500-anchor palettes*: `#058a63` emerald, `#998300` citron yellow, `#3e372c` deep olive
- Max spacing: 224 px

## Side-by-side

| Axis | Rams (run A) | Sottsass (run B) |
|---|---|---|
| display family | Instrument Serif | Source Serif 4 *(critic forced — bug)* |
| body family | Inter | Outfit |
| primary 500 | `#5a8a45` forest green | **`#c44d12` terracotta** ✓ |
| other palettes | single family | **terracotta + emerald + citron + olive — 4-color Memphis** ✓ |
| max spacing | 192 px | 224 px |
| refines needed | 0 | 1 |

## The persona-vs-scaffold conflict (Finding #34)

The Sottsass run's typography got *steered away* from a genuinely-Sottsass-spirited pick (Dela Gothic One — a chunky display face that would have made Sottsass himself smile) and *toward* a more conservative serif (Source Serif 4) that's in the `editorial-portfolio` skill's validated list.

The critic chose to follow the scaffold over the persona, citing two reasons:
1. Korean glyph support (legitimate concern for Seoul intent)
2. "Contradicts the editorial-portfolio scaffold's requirement"

ADR 0004 §"Architectural choices borrowed from Hermes" makes the priority explicit: *"persona principles outrank generic skill scaffolds when conflict — but call out the conflict in your rationale."* The critic instead let the scaffold win silently.

**Finding #34**: Critic does not enforce persona > scaffold priority. Likely fix: re-prompt with explicit instruction *"if you're refining away from a persona-driven choice, you must quote the specific persona principle being violated and explain why scaffolding overrides it."* Without that, the path of least resistance (citing the scaffold's bulleted requirements) wins.

The Korean-glyph concern is *real*, though. Dela Gothic One has limited CJK coverage. A genuine Sottsass-spirited pick that also covers Korean would be something like Recoleta Alt, Hubot Sans, or a custom Hangul-aware display family. The critic should have proposed a CJK-aware Memphis-spirited alternative instead of falling back to the editorial default.

## What survived — the most distinctive axis

Despite typography getting overridden, **the Sottsass palette landed unmistakably**:

```
"500": "#c44d12"  ← terracotta
"500": "#058a63"  ← emerald
"500": "#998300"  ← citron
"500": "#3e372c"  ← deep olive
```

This is *pure Memphis vocabulary*. The Rams run has one muted forest-green primary; the Sottsass run has *four saturated independent colour anchors*. No critic conversation could explain this away — the persona shaped the highest-signal axis decisively.

## Honest agent satisfaction

| Aspect | Pass 11 verdict |
|---|---|
| Two personas produce visibly different artifacts | **5 / 5** — palette delta is unmistakable |
| Persona reasoning shows up in the critic's rationale | 4 / 5 (Sottsass critic referenced register; Rams critic referenced rectilinear / no-decoration) |
| Persona authority outranks scaffold when they conflict | **2 / 5** — Finding #34 |
| CLI ergonomics (`--persona` flag, `aphrodite personas` list) | 5 / 5 |
| Seeded persona quality (writing in their own voice) | 4 / 5 (Rams + Sottsass + Hara strong; Galileo and Ando also strong; Vignelli and Kawakubo respectable; Rams's "forest green" pick is mild stretch) |

**Overall: 4.0 / 5.** Personas work. The most-signal axis (palette) lands decisively. Typography axis loses to scaffold authority — fixable in a follow-up prompt tweak.

## Findings opened by Pass 11

- **#34** Critic doesn't enforce persona > scaffold priority. Mitigation: critic system prompt addendum requiring quoted persona principle before refining away from a persona-driven choice.
- **#35** Persona system doesn't yet handle CJK glyph coverage when a persona's preferred display family lacks Korean/Japanese coverage. Mitigation: extend each persona's frontmatter with `cjk_alternatives: []` field; orchestrator picks accordingly for CJK-mentioning intents.

## Findings carried

- **#25** Composition regenerates every refine
- **#29** Surface composer latency
- **#31** Critic over-corrects against skill scaffolds (now also applies to personas)
- **#32** Lucide CSS classes not yet emitted
- **#33** Wall clock regression on critic-spawned refines

## ADR 0004 status update

Adding personas to the matrix:

| Phase | Status |
|---|---|
| 1 research | stub |
| 1a skill loading | done |
| 1b persona loading | **done** (new) |
| 2 taste application | done |
| 3 design | done |
| 4 self-critic refine | done (with Finding #34 nuance) |
| 5 asset create | stub |
| 6 asset manage | stub |
| 7 harmonize | done |
| 8 skillify proposal | done |

6/8 phases functional (counting 1a + 1b separately). Two asset phases remain.

## Reproduction

```bash
cargo install --path crates/aphrodite-cli
aphrodite personas    # list installed

mkdir -p /tmp/pass11-rams && cd /tmp/pass11-rams && git init
rm -f ~/.aphrodite/taste/preferences.toml
aphrodite create "a portfolio site for an independent furniture maker working in solid walnut, based in Seoul" --persona dieter-rams --json

mkdir -p /tmp/pass11-sottsass && cd /tmp/pass11-sottsass && git init
aphrodite create "a portfolio site for an independent furniture maker working in solid walnut, based in Seoul" --persona ettore-sottsass --json
```

Archive at `docs/agent-eval/archive/2026-05-14-pass11-personas/` with both runs preserved.
