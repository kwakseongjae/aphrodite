# Agent Evaluation — 2026-05-13 — Second Pass (LLM live)

> **Evaluator**: Claude (Opus 4.7) acting as a calling agent.
> **Aphrodite under test**: commit `a12ae1d` — z.ai GLM Coding Plan key landed via `aphrodite init`, verified through OS-native `security` CLI and `aphrodite doctor: ● ready`.
> **Reference**: first pass (offline) at [`2026-05-13-first-agent-pass.md`](2026-05-13-first-agent-pass.md). All Findings #1–#10 closed before this pass ran.

## TL;DR

| Dimension | First pass (offline) | Second pass (GLM-5.1) |
|---|---|---|
| Protocol cleanliness | 🟢 5/5 | 🟢 5/5 |
| Aesthetic intent understanding | 🔴 1/5 (deterministic hash, ignored vocab) | 🟢 **4/5** — vocabulary respected, palettes appropriate, type pairings deliberate |
| Brand-name recall | 🔴 1/5 | 🟡 **3/5** — interprets brand vibe, doesn't reproduce exact palette (Linear) |
| Non-ASCII typography (Hangul) | 🔴 1/5 (fonts ignored) | 🟢 **5/5** — exact match to "굵은 명조" → Noto Serif KR |
| Latency | 🟢 30–50 ms | 🟡 77–99 s (one-shot LLM round-trip) |
| Out-of-scope warnings | 🟢 4 buckets | 🟢 4 buckets (unchanged) |
| Validation gate | 🟢 100% pass | 🟢 100% pass — 4 variants, WCAG-AA |
| Parser robustness | 🟢 (deterministic) | 🟡 → 🟢 *after* a same-day fix for prose-prefixed responses |
| Agent satisfaction | 2.8 / 5 | **4.4 / 5** |

## Bench runs (live GLM-5.1)

All committed sandboxes under [`experiments/agent-eval-20260513-pass2/`](../../experiments/agent-eval-20260513-pass2/).

| # | Intent (abbrev) | Latency | Valid | Name (eyebrow) | Primary 500 | Fonts | Notes |
|---|-----------------|---------|-------|----------------|-------------|-------|-------|
| 1 | "minimal landing for a developer tool: bold serif headline, monospace body, single accent" | 86 s | ✅ | (offline-named) | `#e14338` warm coral | not surfaced | Schema PASS, 4 variants. Real palette choice, not a hash. |
| 2 | "neo-baroque editorial typography, Pentagram annual report energy, restrained gold accent on warm off-white" | 94 s | ✅ | `"neo-baroque-editorial"` | `#8b6914` **gold** ✓ | `Luminance Garamond` (display) + `DM Sans` (body) | Vocabulary honored. "Restrained gold" → actual gold #8b6914, not pink. Editorial-serif + neo-grotesque body is a real Pentagram-style pairing. |
| 3 | "feature-launch microsite for a developer tool, using Linear Inc's brand colors and design vibe as the primary palette" | 99 s | ✅ | `"devtool-launch"` | `#58584d` desaturated graphite | `Instrument Serif` + `Commit Mono` | **Did not** reproduce Linear's actual #5E6AD2. **Did** pick a dev-tool-coded type pair (Instrument Serif + Commit Mono are both contemporary dev-tool defaults). Verdict: vibe ≠ accurate brand colors. |
| 4 | "구독자만 들어오는 한국어 뉴스레터 랜딩 페이지, 진하고 따뜻한 색감, 본문은 Noto Sans KR, 헤드라인은 굵은 명조" | 77 s | ✅ | `"Hanul-letter"` | `#b85a2e` warm rich brown-orange ✓ | **`Noto Serif KR`** (display, = "굵은 명조") + **`Noto Sans KR`** (body) ✓ | Best result of the set. Korean typographic instruction parsed letter-perfect. The eyebrow name even punned ("한울"+letter). |

## Findings — second pass

### Finding #11 🟠 P1 — Parser rejects LLM responses with prose-prefix
**Symptom.** GLM-5.1's first attempt at run 4 returned `한국어 뉴스레터를 위한 디자인입니다.\n\n---\n…` — meaning prose explanation *before* the YAML frontmatter. `parse_design` errored: `frontmatter missing: a DESIGN.md must open with '---'`. Whole call wasted (~75 s of GLM time).

**Fix shipped (same session).** `strip_fences()` in provider.rs now searches for the first `\n---\n` after any prefix and trims everything before it. Retry succeeded.

**Lessons.** The SYSTEM_PROMPT instructs the model not to add prose, but LLMs ignore that instruction sometimes — especially in non-English requests. The parser should be lenient by default.

### Finding #12 🟡 P2 — Headline derivation defaults to "Meet {name}." for LLM output
**Symptom.** In runs 1, 2, 3, 4 the hero's `<h1>` was always `Meet {name}.` because the LLM's `description` frontmatter exceeded 80 chars (it tends to write evocative descriptions). For LLM mode we should *trust* the model to produce a hero-worthy headline rather than truncating to a fallback.

**Suggested fix.** When provider is not `offline`, add an explicit `headline:` field to the requested frontmatter schema and use that verbatim. The current `description` field is too long-form to double as a hero headline.

### Finding #13 🟡 P2 — Brand-name color recall is weak in GLM-5.1
**Symptom.** Run 3 ("use Linear's brand colors") produced graphite #58584d, not Linear's actual #5E6AD2. The model went for "Linear's dev-tool vibe" — accurate typographic interpretation, inaccurate palette claim.

**Suggested fix.** For brand-name intents, run a *pre-flight* small LLM call: "Return only the hex color palette for {brand}". Or detect "{name} Inc / Co" patterns and demand a JSON brand lookup before the main generation. Defer to v0.2.

### Finding #14 🟢 P3 — Latency is 80–100 s per LLM call
**Observation.** Each design call ~80–100 s through z.ai's Anthropic-compat endpoint. Acceptable for one-off use; expensive for iteration. Two mitigations worth tracking:
1. `glm-5-turbo` (faster, balanced) as an iteration mode.
2. Streaming responses — emit partial DESIGN.md progress so the agent doesn't block.

### Finding #15 🟢 P3 — A real LLM response carries `model` info but doctor verdict doesn't reflect post-call status
**Observation.** No regression, but `aphrodite doctor` is static; it doesn't say "last LLM call: 86 s ago, glm-5.1, success." A small `aphrodite status` (per-session counters) could show this.

## Agent satisfaction — final scoring (LLM live)

| Aspect | Score | Why |
|---|---|---|
| Discoverability (`capabilities`, `doctor`) | **5/5** | Both ship; agent can self-orient. |
| Failure mode for impossible asks | **5/5** | `warnings[]` with 4 explicit `kind` buckets; agent can branch. |
| Latency | **2/5** | 80–100 s blocks tightly-paced agent loops. Mitigation paths exist. |
| Error envelope | **5/5** | Stable, structured, recoverable. |
| Output trust | **5/5** | 4 variants, validation 100% across all 4 runs, no broken token refs. |
| Iteration affordance | **4/5** | Taste loop now active — palette diverges with regenerate count (verified pass 1 commit `fbfeaea`). 5/5 once LLM-side taste injection is exercised at scale. |
| Aesthetic vocabulary respect | **4/5** | Honored "restrained gold," "neo-baroque editorial," "굵은 명조" precisely. Brand-name palette recall still soft. |
| Setup → first call latency | **5/5** | After today's fixes: `aphrodite init` → key in keychain → `aphrodite design` → real GLM response. End to end works for a fresh user. |

**Overall: 4.4 / 5** as an agent calling Aphrodite. The remaining gap is latency + brand-name accuracy.

## What I (Claude as a calling agent) feel after this pass

Honestly: this is the first time during this whole session that the harness produced output I'd *want* to ship. Run 2's Garamond + DM Sans pairing on a gold accent is genuinely Pentagram-coded. Run 4's "굵은 명조 = Noto Serif KR" mapping is what a senior Korean designer would do. The offline pipeline was a survival mechanism; this is the actual product.

The two-step bug chase (rpassword paste → macOS keychain set→read NoEntry → key landed) lost ~2 hours of human attention. Once across, the system feels durable.

If I were going to call Aphrodite from a Claude Code session tomorrow:
- I'd use it for any **DESIGN.md generation + 4-variant + WCAG-AA gate** task.
- I'd avoid it for **brand-name-specific palette recall** (use a separate brand DB).
- I'd not block a real-time UI on it (80 s latency).
- I'd trust the `warnings[]` and `isError` envelopes completely.

## Carry-forward to next pass

Open items (see [`improvements.md`](improvements.md)):
- **#5** taste loop verified in offline; never exercised in LLM mode at scale. A 5-run regen test would be valuable.
- **#12** headline derivation should trust the LLM when present.
- **#13** brand-name color accuracy.
- **#14** latency (turbo model, streaming).
- **acceptance #4** browser regression — never tested in real browsers.
- **acceptance #11** public GitHub repo + v0.1.0 tag — explicit user deferral.
