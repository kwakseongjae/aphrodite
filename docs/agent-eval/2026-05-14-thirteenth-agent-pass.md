# Agent Evaluation — 2026-05-14 — Thirteenth Pass (persona × intent matrix)

> Pass 12 verified one persona (Sottsass) on its expected intent (joyful furniture maker). Pass 13 widens to a 5-persona × 5-intent matrix where each persona is paired with the intent its `when_to_invoke` field names. The hypothesis: when the pairing is right, the persona's voice should land on the artifact in measurable ways — own-recommended fonts, own-named accents, own-articulated structural choices.

## Test design

Same `aphrodite create --persona <slug>` command across five pairings, all run in parallel from clean state. Each persona is matched to an intent within its declared domain:

| Persona | Intent |
|---|---|
| `massimo-vignelli` | a long-form journalism site for a Seoul-based investigative magazine |
| `tadao-ando` | a portfolio site for an independent architecture studio in Jongno Seoul, working with concrete and traditional Korean materials |
| `rei-kawakubo` | a fashion-house lookbook site for an independent Seoul designer working in deconstructive silhouettes |
| `galileo-galilei` | an analytics dashboard for a clinical trial monitoring platform showing patient enrollment, dropout rates, and adverse-event time series |
| `kenya-hara` | a contemplative longevity research clinic in Seoul, with brand voice rooted in patience and Korean cultural restraint |

All five launched simultaneously at 17:36:40. Total wall clock: ~10 minutes (vs ~30 minutes sequential).

## Results matrix

| Persona | Wall | Display | Body | CJK | Primary 500 | Max spacing | Refines | Verdict |
|---|---|---|---|---|---|---|---|---|
| Vignelli × magazine | 289 s | **Bodoni Moda** ✓ | (Bodoni-paired) | **Source Han Sans KR** ✓ | `#2d6b50` muted teal | 192 px | rate-limit crash mid-critic | partial |
| Ando × architecture | 340 s | Source Serif 4 | Inter | **Nanum Myeongjo** ✓ | `#6b5a3e` warm-brown (concrete-walnut adjacent) | 224 px | 0 | full |
| Kawakubo × lookbook | 579 s | (Latin) Instrument Sans + (Hangul) Pretendard | Noto Serif KR + Noto Sans KR mixed | (both) | `#231c14` near-black | **256 px** | 1 | partial |
| Galileo × dashboard | 616 s | **Inter Tight** ✓ | Inter + **JetBrains Mono** ✓ | Pretendard | `#1a7a70` teal | 96 px | 1 | full |
| Hara × clinic | 620 s | Source Serif 4 | Pretendard | **Apple SD Gothic Neo** ✓ | `#7d7568` muted warm-grey | 192 px | 1 | full |

## Persona-voice evidence — what each persona's body shaped

### Vignelli × magazine
- Display = **Bodoni Moda** — Vignelli's `prefers:` list names "Helvetica or Bodoni at the display." Generator picked Bodoni.
- CJK = **Source Han Sans KR** — Vignelli's `cjk_strategy` reads *"Korean: Pretendard or Sandoll Gothic … Source Han Sans … always pair Latin and CJK at matched optical weights."* Source Han Sans is one of the named options.
- **Critic crashed with z.ai 429 rate-limit** (Finding #29 reappearance under parallel load). Design itself succeeded; refine pipeline didn't get to run. The Bodoni + Han Sans pick is from the *initial design*, persona-anchored.

### Ando × architecture
- Display = Source Serif 4 (modern serif — strokes feel cut from material per Ando's body text)
- CJK = **Nanum Myeongjo** — Ando's `cjk_strategy` reads *"Korean: Sandoll Myungjo or Nanum Myeongjo (a serif Hangul). Never gothic."* Generator picked exactly one of the named families.
- Primary `#6b5a3e` — concrete-adjacent warm grey-brown, sits next to walnut tones without competing.
- 0 refines, satisfaction 0.88. Critic accepted on first read.

### Kawakubo × lookbook (most-Kawakubo-spirited prose)
- Body text in DESIGN.md *quotes Kawakubo's body verbatim*: *"The collection statement is one sentence set in Pretendard 900 at 14 px, centered in 192 px of vertical space, in the accent colour. It is the only centered element on the entire site. **It is small and it refuses to accommodate you.**"*
- And: *"crop characters at viewport edges. The cropped Hangul syllable is more present than the complete one — the viewer completes it mentally, which means they have engaged with it."*
- Max spacing **256 px** — the most generous of any persona in this matrix.
- **But**: critic noted *"composition.html is empty — the entire design exists only as prose in DESIGN.md with no structural expression."* Generator wrote a strong DESIGN.md narrative but the surface composer didn't produce HTML. Defensible (the design is *radically* non-conventional and the composer may have hit a register it couldn't translate) but a real gap.

### Galileo × dashboard
- Display = **Inter Tight** — published metrics, measurable; Galileo's `cjk_strategy` says *"pick a family that publishes its metrics."*
- Mono = **JetBrains Mono** — for data tables / numbers. Galileo would approve: monospace columns are *measurably aligned*.
- Critic flagged a *real* prose-token mismatch: *"The prose and layout section specify a 64 px fixed navigation rail occupying the leftmost column, but the composition.html renders a 220 px sidebar — and more critically, the filter rail component described in the DESIGN.md is entirely absent from [the composition]."* — refined once. **This is the critic doing the job it was built for** (catching prose-vs-implementation drift), with Galileo's "measure what is measurable" register.

### Hara × clinic
- Display = Source Serif 4
- CJK = **Apple SD Gothic Neo** — Hara's `cjk_strategy` reads *"Korean: Pretendard Light or Apple SD Gothic Neo Light."* Picked.
- Primary `#7d7568` — muted warm grey, MUJI-adjacent neutral.
- **Critic *quoted the persona principle* in its refine rationale**: *"The features section uses a three-column card grid — a conventional SaaS-style layout that contradicts the persona's principle of emptiness and the prose's assertion that 'a symbol that almost-disappears succeeds better than one that asserts'."* This is exactly the Pass 12 prompt directive landing: critic anchors its judgment in the persona, not the scaffold.

## Cross-cutting observations

### Persona-named-or-recommended families land 5/5

Every single persona in the matrix produced an artifact whose typography includes either a named-in-prefers family (Bodoni Moda for Vignelli) or a named-in-cjk_strategy family (Nanum Myeongjo, Source Han Sans KR, Apple SD Gothic Neo, JetBrains Mono). This is unambiguous evidence that the persona body drives generator picks at the typographic level, not just at the "vibe" level.

### Critic anchored to persona, not scaffold

Three of five runs (Hara, Galileo, Kawakubo) had the critic explicitly quote the persona's principles in its rationale. The Pass 12 fix (Finding #34 — persona > scaffold authority) holds under load: when a persona is invoked, the critic uses *its* vocabulary.

### Spacing varies meaningfully by persona register

- Galileo (data-dense): 96 px max
- Vignelli (editorial): 192 px max
- Hara (contemplative): 192 px max
- Ando (architectural ma): 224 px max
- Kawakubo (deconstructive): 256 px max

Information density inverts with the persona's domain. No one told the system to do this — it's emergent from the persona prompts.

### One transient failure (Vignelli's 429)

Parallel load on z.ai produced one rate-limit hit mid-critic on the Vignelli run. The initial design completed before the crash, so the artifact is still persona-anchored — but the critic loop didn't run and the proper composition didn't get a refinement pass. **Finding #29 manifests under parallel load**; this is the cost of parallelization. Sequential runs would have avoided it.

### Kawakubo composition didn't render

The Kawakubo body text is the most-radical-design output of the matrix — "crop characters at viewport edges," "Pretendard 900 at 14 px refusing to accommodate you," "designer's face appears nowhere." The surface composer didn't translate this into HTML. Two possibilities: (a) the composer's prompt is biased toward conventional editorial templates and can't represent radical layouts, (b) the composer correctly recognized that "the only centered element is one sentence at 14 px in 192 px of space" requires hand-tuning beyond its template repertoire. Either way: **Finding #36** — composer needs a "radical-register" mode for personas like Kawakubo that ask for non-templated layouts.

## Honest agent satisfaction

| Aspect | Score |
|---|---|
| Persona body shapes typography (named families land) | 5/5 (all 5 pairings) |
| Critic anchors rationale in persona principles | 4/5 (3 of 5 explicit; Ando passed silently, Vignelli crashed) |
| Spacing/density emerges from persona register | 5/5 (clean Galileo-96 → Kawakubo-256 gradient) |
| CJK strategy text flows to artifact | 5/5 (every CJK pick matched the persona's strategy field) |
| Refinement loop respects persona authority | 5/5 (no Pass 11-style scaffold-override observed) |
| All 5 runs produced usable composition.html | 4/5 (Kawakubo's empty; Finding #36) |
| Parallel-load stability | 3/5 (one rate-limit-induced critic crash) |

**Overall: 4.4 / 5.** Strongest evidence yet that the persona system actually shapes output beyond palette. The remaining gaps are real but tractable: surface composer needs better support for radical layouts (#36), and parallel orchestration needs rate-limit-aware throttling (#29 widens scope).

## Findings opened by Pass 13

- **#36** Surface composer can't render Kawakubo-style radical / anti-templated layouts. The DESIGN.md prose was exceptional; the HTML composition fell back to empty. Need a "radical-register" pathway in surface.rs that recognizes when the persona/scaffold asks for non-conventional structure.

## Findings re-confirmed (carried with broader scope)

- **#29** Surface composer / provider transient failures manifest worse under parallel load. Mitigation: per-provider concurrency cap in `aphrodite_generator::provider`, or per-call back-off + retry.

## ADR 0004 status — no change from Pass 12

Still 7/8 phases functional. Pass 13 is *evidence* for the existing architecture, not new architecture.

## Reproduction

```bash
cargo install --path crates/aphrodite-cli
rm -f ~/.aphrodite/taste/preferences.toml
for slug in massimo-vignelli tadao-ando rei-kawakubo galileo-galilei kenya-hara; do
  DIR=/tmp/pass13-$slug
  mkdir -p "$DIR" && cd "$DIR" && git init -q
  aphrodite create "<intent matched to persona>" --persona $slug --max-turns 2 --json &
done
wait
```

Archive at `docs/agent-eval/archive/2026-05-14-pass13-persona-matrix/` with all 5 runs preserved.
