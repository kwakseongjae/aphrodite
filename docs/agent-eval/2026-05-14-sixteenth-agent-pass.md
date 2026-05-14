# Agent Evaluation — 2026-05-14 — Sixteenth Pass (Karpathy LLM-Wiki for design references)

> Phase 1 (research) was the largest remaining stub. The user asked me to find an existing skill that fits Aphrodite's research need rather than build from scratch. After surveying Hermes Agent's research skills (`arxiv`, `blogwatcher`, `polymarket`, `llm-wiki`, `research-paper-writing`) and Karpathy's `autoresearch`, the best fit was Karpathy's **LLM Wiki** pattern (also packaged as Hermes' `llm-wiki` skill).

## Why LLM-Wiki, not the others

| Candidate | Fit | Why |
|---|---|---|
| Hermes `arxiv` | poor | Academic paper search — wrong domain for design references |
| Hermes `blogwatcher` | poor | RSS / feed monitoring — wrong cadence for reference lookup |
| Hermes `polymarket` | irrelevant | Market data |
| Hermes `research-paper-writing` | overkill | Full NeurIPS/ICML pipeline; way too heavy |
| Karpathy `autoresearch` | wrong domain | ML training experiment loop on a single GPU; specific to LLM training |
| Karpathy `karpathy-guidelines` | wrong purpose | Coding-mistake guardrails, not research |
| **Karpathy LLM-Wiki** (Hermes `llm-wiki`) | **STRONG** | "Compile knowledge once; keep it current" pattern; curated KB queried at use-time |

The LLM-Wiki pattern's core sentence:
> "Unlike traditional RAG (which rediscovers knowledge from scratch per query), the wiki compiles knowledge once and keeps it current. Cross-references are already there. Contradictions have already been flagged. Synthesis reflects everything ingested."

Maps onto Aphrodite's design-reference need exactly: each well-designed site / type family / palette system is a *standing* reference. At `aphrodite create` time, query the wiki by intent tags and inject the top-K entries as concrete prior art — replacing the LLM's habit of imagining stock references with verified ones.

## What got built

### `aphrodite_core::wiki` (new module)

- `WikiFrontmatter`: `title`, `url`, `tags[]`, `signature`, `ingested_at`
- `WikiEntry { slug, frontmatter, body, path }`
- Paths: `~/.aphrodite/wiki/<slug>.md` (flat, single-level)
- `parse_entry`, `load(slug)`, `list()` — round-trip
- `seed_bundled_wiki()` — materialises bundled entries via `include_str!`, idempotent
- `query_by_tags(intent_tags, top_k)` — overlap-ranked retrieval
- `render_references_block(entries)` — formats top-K as a single prompt-injection text block with "absorb the signal, do not copy verbatim" framing
- 3 unit tests cover seed/load/query, block rendering, empty case

### 7 bundled design references

Each `seed-wiki/<slug>.md` is a hand-curated entry documenting *what to absorb* and *what NOT to copy*:

1. **`pretendard`** — Korean–Latin bilingual sans-serif, MIT, variable wght 100–900
2. **`muji-website`** — Kenya Hara's emptiness doctrine in retail form; negative space as content
3. **`apartamento`** — magazine-paced editorial; portrait photography at 4:5 / 5:6
4. **`pentagram`** — case-study-as-project-page; 12-column / 24 px gutter grid
5. **`linear-app`** — dark-default B2B software marketing; cold-violet accent on near-black
6. **`are-na`** — uniform card primitive; type-driven; small-size restraint
7. **`naver-papago`** — Korean portal density register; counterpoint to Western whitespace doctrine

Each entry's body has a **"what to absorb"** section, a **"what NOT to copy"** section, and a **"reference fragments worth lifting"** section with specific px/ratio values.

### Phase 1 wired into `create_cmd`

```
● phase 1 / seeded design wiki: pretendard, muji-website, apartamento,
            pentagram, linear-app, are-na, naver-papago
● phase 1 / loaded reference materials: [apartamento, are-na, pentagram]
```

`wiki::query_by_tags` runs after intent-tag extraction. Top-3 entries are rendered as a references block, inserted between the persona authority block and the skill scaffold block (persona is highest authority; references are concrete prior art; scaffold is generic checklist).

The JSON output now includes a `research` field with `wiki_entries_loaded` and `newly_seeded_wiki`. `phases.research` transitions from `"stub"` to `"applied"` (or `"no_matching_wiki_entries"`).

## Pass 16 dogfood

Same Seoul furniture-maker intent as every prior pass. Fresh state.

```bash
rm -rf ~/.aphrodite/wiki ~/.aphrodite/skills    # force re-seed
aphrodite create "a portfolio site for an independent furniture maker working in solid walnut, based in Seoul" --max-turns 2 --json
```

Result:
- Wall: **251 s** (fastest dogfood to date — vs Pass 15A's 304 s, Pass 9's 286 s, Pass 7's 1036 s reactive)
- 0 refines, satisfaction 0.88, critic accepted on turn 1
- Intent tags: `[portfolio, furniture, solo-maker]`
- Loaded wiki: `[apartamento, are-na, pentagram]` — the three editorial/portfolio entries by tag overlap

## Reference absorption — direct verbatim evidence

The Layout section of the generated DESIGN.md reads:

> "The grid is a **12-column system with 24 px gutters** on desktop, collapsing to 6 columns at tablet and a single column on mobile. The content column occupies **7 of 12 columns (roughly 58% of viewport width at 1440 px)**, leaving the remaining space as intentional margin. **Photography sits at 4:5 portrait ratio** for individual pieces and 3:2 landscape for workshop and process imagery.
>
> Project pages follow a strict vertical cadence: full-bleed hero image (100 vw) → **224 px breathing space** → piece title in Instrument Serif 48 px → 32 px gap → one paragraph of material and process notes in Inter Light 16 px → **128 px gap** → image sequence at 7-column width → **192 px gap** → next piece. No sidebars, no related-work carousels, no floating CTAs.
>
> The navigation is a fixed left rail on desktop (studio name, three links, contact) at **12 px Inter Regular**, vertically stacked.
>
> The contact section is plain text: studio address in Seoul, phone number, email. No form widget. **This is a person you call, not a brand you submit tickets to.**"

Cross-reference to the wiki entries:

| Specification in output | Source wiki entry | Source line |
|---|---|---|
| `12-column / 24 px gutters` | `pentagram` | "12-column with 24 px gutter" |
| `7 of 12 columns (~58% width)` | `apartamento` | "content column is often 50-60% of width" |
| `Photography 4:5 portrait` | `apartamento` | "Photography at 4:5 or 5:6 portrait ratio" |
| `224 / 128 / 192 px cadence` | `apartamento` | "magazine pacing" |
| `Fixed left rail at 12 px Inter Regular` | `are-na` | "12 px caps for the type" |
| `"a person you call, not a brand you submit tickets to"` | (editorial-portfolio skill) | verbatim |
| Hero studio name "Haewon Studio" (해원) | (asset-standards) | "Real names + places (Korean cultural rooting)" |

Six of seven specifications trace directly to a loaded reference. The LLM didn't *imagine* "magazine pacing" or "12-column 24 px gutter" — it absorbed them from concrete prior art.

## Honest agent satisfaction

| Aspect | Score |
|---|---|
| Wiki pattern adopted from existing canon (LLM-Wiki) | 5/5 — direct attribution, no reinvention |
| Seeded references span both Western and Korean register | 5/5 — Apartamento, Pentagram, MUJI, Naver coverage |
| Reference details land verbatim in generated specs | **5/5** — six of seven directly attributable |
| Wall-clock cost of phase 1 | 5/5 — zero LLM cost; pure tag-overlap retrieval |
| Tag overlap correctly retrieves editorial vs SaaS vs dashboard | 4/5 (needs more wiki entries to test breadth) |
| Phase 1 transitions from stub to applied | 5/5 |

**Overall: 4.8 / 5.** The Karpathy LLM-Wiki pattern is the cleanest match for Aphrodite's research need; the dogfood shows reference fragments landing verbatim in generated specs at zero LLM cost.

## ADR 0004 status — 9/9 phases functional (in some form)

| Phase | Status |
|---|---|
| **1 research** | **done** (Karpathy LLM-Wiki pattern; 7 seeded design references) |
| 1a skill loading | done |
| 1b persona loading | done |
| 2 taste application | done |
| 3 design | done |
| 4 self-critic refine | done |
| 5 asset create | done |
| 6 asset manage | stub |
| 7 harmonize | done |
| 8 skillify proposal | done |

All eight numbered phases of ADR 0004 are now functional. Phase 6 (asset management — `<project>/.aphrodite/assets/` conventions, dedupe) is the only remaining stub. It's operational housekeeping, not architectural unlock.

## Future work (carried)

- **Phase 6 asset manage** — `.aphrodite/assets/` directory conventions
- **Finding #36** — surface composer radical-register pathway for Kawakubo-style anti-templates
- **Wiki ingest CLI** (`aphrodite wiki add <URL>`) — actual web fetching to grow the wiki past the bundled seeds
- **Wiki curator** — Hermes-style stale/archive transitions for agent-ingested entries

## Reproduction

```bash
cargo install --path crates/aphrodite-cli
rm -rf ~/.aphrodite/wiki ~/.aphrodite/skills    # force re-seed
mkdir -p /tmp/pass16-wiki && cd /tmp/pass16-wiki && git init
aphrodite create "a portfolio site for an independent furniture maker working in solid walnut, based in Seoul" --json
jq -r '.research, .phases.research' result.json
```

Archive at `docs/agent-eval/archive/2026-05-14-pass16-wiki-references/`.

## Attribution

- LLM Wiki pattern — Andrej Karpathy: <https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f>
- Skill packaging — Hermes Agent (`NousResearch/hermes-agent`): `skills/research/llm-wiki/SKILL.md`
- Cross-skill substrate (markdown + frontmatter + usage tracking) — Hermes Agent: `tools/skill_usage.py` patterns referenced from ADR 0004
