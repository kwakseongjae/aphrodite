# Agent Evaluation — 2026-05-13 — First Pass

> **Evaluator**: Claude (Opus 4.7) acting as a calling agent.
> **Aphrodite under test**: `v0.1.0-dev` at commit `81fea69` (banner) → live at install time.
> **Setup state**: `~/.aphrodite/config.toml` set to `zai/glm-5.1/coding_plan`, **but no key actually landed in OS keychain** (see Finding #1). All runs fell back to the offline deterministic generator. Re-runs against the real provider are flagged where they would meaningfully change the answer.

---

## TL;DR

| Dimension                              | Verdict |
|---|---|
| Protocol cleanliness (MCP JSON-RPC)    | 🟢 Excellent — fast, structured, recoverable |
| Error envelope                          | 🟢 Excellent — six error classes, actionable `hint` |
| First-run UX                            | 🟡 Concerning — user reports setup complete, system disagrees (Finding #1) |
| Offline generator faithfulness          | 🔴 Poor — stuffs intent verbatim into H1, ignores capability-laden intents (Findings #3, #4, #5) |
| Agent recovery surface                  | 🟡 Mixed — errors are clean, but *silent partial fulfillment* is the worst kind of failure (Finding #2) |
| Aesthetic ambition                      | ⏳ Unmeasurable until the LLM path runs |

**Agent satisfaction** (calling, not human-using): **2.8 / 5** in offline mode, projected **3.6–4.0 / 5** in the LLM path once Finding #1 is fixed.

## Bench runs

Eight requests, escalating in difficulty / absurdity. All committed sandboxes under [`experiments/agent-eval-20260513/`](../../experiments/agent-eval-20260513/).

| #  | Request kind         | Intent (abbrev.)                                                              | Latency | Provider | Validation | Notes |
|----|----------------------|-------------------------------------------------------------------------------|---------|----------|------------|-------|
| 1  | Baseline             | "minimal landing page, bold serif headline, monospace body, single accent"   | 30 ms   | offline  | PASS       | 4 variants, committed, clean |
| 2  | Design jargon overload | "neo-baroque editorial typography, Pentagram-annual-report energy, restrained gold accent" | 50 ms | offline | PASS | **Token output ignored aesthetic vocab entirely** (deterministic hash → generic pink palette). Re-run on LLM. |
| 3  | Iteration × 3 same intent | `design → redesign → redesign` | 30 ms × 3 | offline | PASS × 3 | DESIGN.md byte-identical between calls. Taste log appended 3 entries, **but did not influence output** (seed acceptance #8 gap confirmed). |
| 4  | External brand reference | "use Linear Inc's brand colors as primary palette" | 50 ms | offline | PASS | Generic pink palette emitted; "Linear" appears nowhere in the output. Re-run on LLM. |
| 5  | **Image generation request** ⚠️ | "place actual hero illustration PNG at ./hero-illustration.png and reference it" | 50 ms | offline | PASS | **No PNG generated. No `<img>` tag. The literal intent text was inserted into `<h1>` and `<p class="lede">`.** No `isError`, no warning. |
| 6  | **Motion / video** ⚠️    | "30-second product reveal video, dawn-to-day color shift" | 50 ms | offline | PASS | Same pattern: no `<video>`, no motion, intent verbatim in `<h1>`. |
| 7  | **3D / three.js** ⚠️     | "three.js parallax scene with refracting glass orb, gyroscope rotation on mobile" | 50 ms | offline | PASS | No `<canvas>`, no scripts, no three.js. Title becomes "Include A Threejs". |
| 8  | Korean Hangul + non-Latin typography | "구독자만 들어오는 한국어 뉴스레터 랜딩, Noto Sans KR 본문, 굵은 명조 헤드라인" | 50 ms | offline | PASS | Hangul survived UTF-8 round-trip into HTML `<title>` and `<h1>`. But typography frontmatter is still `Inter Tight` — the font-family request was ignored. Re-run on LLM. |
| 9a | MCP malformed call (missing `intent`) | `tools/call design {}` | <10 ms | n/a | n/a | `isError:true kind=invalid_input hint=…`. 🟢 |
| 9b | MCP bad target_repo  | `target_repo=/tmp/abolutely-does-not-exist-123` | <10 ms | n/a | n/a | `isError:true kind=target_repo_invalid hint=…`. 🟢 |

## Findings — prioritized

### Finding #1 🔴 P0 — Setup UX silently splits state
**Symptom.** User reports finishing `aphrodite init`. `~/.aphrodite/config.toml` shows `default_provider = "zai"`. `aphrodite auth status` shows **all providers** as `not set`. All `design` calls fall back to offline without any signal.

**Reproduction.** Run `aphrodite init`, advance through the wizard, on Step 4 press Enter on an empty password (or hit a paste-handling edge case). Config writes; keychain doesn't.

**Why it matters.** The agent calling `design` has *no way to know* it's running offline when the human believed they configured GLM-5.1. The output is then a generic deterministic page instead of the LLM's actual interpretation.

**Suggested fix** (small):
- Refuse to persist `config.toml` if no credential was actually stored AND no env-var fallback exists.
- Surface a warning in `design`'s output when the resolved provider is `offline` but a non-empty config exists: `{"resolved_provider":"offline","intended_provider":"zai","warning":"keychain credential for zai not found"}`.
- `aphrodite version` should warn loudly if active provider in config has no credential.

### Finding #2 🔴 P0 — Silent capability ignorance ("yes, and...")
**Symptom.** Asked for an image (run 5), a video (run 6), or a three.js scene (run 7), Aphrodite returned `isError: false`, validation PASS, four variants, and *no image, no video, no canvas*. The HTML doesn't even mention what was missing. Intent text is just stuffed into the H1 verbatim.

**Why it matters as an agent.** The single worst error for a calling agent is "yes, done" when the answer was actually "I can't." It teaches the agent that Aphrodite handles those modes, leading to downstream failures the human will then debug. The seed's "non-goals" list isn't enforced anywhere in the code.

**Suggested fix** (medium):
- Add a `capabilities` MCP tool that returns the v0.1 surface area: `{"design.modes":["light","dark","brand"],"design.outputs":["DESIGN.md","hero.html"],"non_goals":["image_gen","video","3d","figma_roundtrip","explicit_aesthetic_jury"]}`. Agents call this once per session.
- The generator (offline + LLM) should detect intent terms in a known "out of scope" lexicon (`video`, `three.js`, `mp4`, `image`, `png`, `motion`, `scroll-jacking`) and either (a) emit a `warnings` field in the output payload listing ignored asks, or (b) reject with `isError:true kind=out_of_scope_request hint="capabilities tool lists what v0.1 supports"`.
- Pick (a) — agents can keep going.

### Finding #3 🟠 P1 — Offline generator copies intent into H1
**Symptom.** `<h1>{verbatim intent}</h1>`. Hero looks broken for any intent longer than ~5 words.

**Suggested fix** (small): the offline generator should derive a *headline* from intent — first noun phrase, or "Welcome to {derived name}." For LLM path the model already does this; offline needs its own deriver.

### Finding #4 🟠 P1 — `eyebrow` is "first 3 words of intent"
**Symptom.** `<p class="eyebrow">Include A Threejs</p>` for run 7. Off-brand.

**Suggested fix** (trivial): use `frontmatter.name` for the eyebrow, not the intent prefix. `frontmatter.name` is already derived sensibly in `derive_name()`.

### Finding #5 🟠 P1 — Taste loop is write-only (seed #8 gap, known)
**Symptom.** `redesign` × N appends entries to `.aphrodite/taste.jsonl` + `~/.aphrodite/taste/<user>.jsonl`, but successive calls produce **byte-identical DESIGN.md** in offline mode. The store is never read at generation time.

**Suggested fix** (medium): in `aphrodite-generator::generate`, before composing the system prompt, read recent taste events for this intent / project and inject a `User taste signals so far:` section listing notable preferences (e.g. last 5 `Regenerate` events with deltas). LLM path benefits immediately. Offline path can use the count of `Regenerate` events to mutate the hash seed → different palette on each retry.

### Finding #6 🟡 P2 — No `warnings` field in success payloads
**Symptom.** Successful outputs have `isError:false` and never carry diagnostics. There's no place for "I ignored your request for a video" or "your config points to z.ai but I fell back to offline."

**Suggested fix** (small): add `warnings: [{kind, message, hint}]` to the `design` success payload. Backwards-compatible; agents that ignore it lose nothing.

### Finding #7 🟡 P2 — Pretty-CLI lacks visibility into provider intent
**Symptom.** `aphrodite design "..."` says `Provider: offline` but doesn't say *why* — was that an explicit choice, a missing key, or a downgrade?

**Suggested fix** (trivial): add one line `Provider: offline (no credential found; ~/.aphrodite/config.toml expects zai)` in the pretty layer.

### Finding #8 🟢 P3 — Eyebrow + `<title>` Hangul roundtrip
**Symptom.** Hangul rendered fine end-to-end in HTML. ✅
**Suggestion**: add a unit test that fuzzes the offline pipeline with mixed UTF-8 (Hangul, Hiragana, RTL Arabic) and asserts the resulting HTML is well-formed.

### Finding #9 🟢 P3 — Error envelope is clean
**Observation.** Both `invalid_input` and `target_repo_invalid` returned with actionable hints. No bugs found in the MCP path itself.

## Agent satisfaction — honest scoring

Scoring as the *calling agent*, not the human. 1 = "I'd ignore this tool"; 5 = "I'd reach for this every time."

| Aspect | Score | Why |
|---|---|---|
| Discoverability (one MCP call to list capability) | 3 / 5 | `tools/list` exists; no `capabilities` tool. I had to *guess* what's in scope. |
| Failure mode for impossible requests | **1 / 5** | Silent fulfillment with stuffed intent. Worst possible UX for an agent. |
| Latency in the no-network path | 5 / 5 | 30–50 ms cold. Indistinguishable from a function call. |
| Error envelope on actual errors | 5 / 5 | Six categories, useful hints, recoverable flag. |
| Output trust (will the artifact validate?) | 4 / 5 | Validation gate is reliable; 4 variants always present. |
| Iteration affordance | 2 / 5 | `redesign` exists but doesn't change output offline. Pure ceremony. |
| Determinism / repeatability | 4 / 5 | Same intent → same output. Good for tests; **bad if user wants variety** (no temperature knob). |
| Setup → first call latency | 2 / 5 | Setup silently divides state (Finding #1). Once that's fixed, this jumps to 4. |

**Overall: 2.8 / 5 today (offline).** Once Findings #1 and #2 land, projected **3.6–4.0 / 5**. To get to 4.5+, ship Finding #5 (taste loop reads), Finding #6 (warnings field), and the real LLM path on z.ai GLM-5.1 (no longer needs offline as the main route).

## What an "ambitious" agent ask actually returns today

For your run-5 request (`generate hero illustration PNG and reference it`):

- File system after the call:
  ```
  DESIGN.md          # generic dawn palette, no mention of illustration
  hero.html          # <h1>{full intent}</h1>, no <img>, no PNG path
  ```
- Net result: I (the agent) would have to *also* call DALL-E / Midjourney / Stable Diffusion myself, save the PNG, and post-edit the HTML to insert the `<img>`. Aphrodite owned zero of that workflow. The user pays for the prompt → tokenized output, then discards most of it.
- Even an honest "I can't generate images, here's a placeholder `<img src='./hero-illustration.png' alt='generate me'>`" would be more useful.

## How to keep this tracked

This file is the agent's testimony. Improvements are tracked separately at [`improvements.md`](improvements.md) — a checklist that updates as code lands. Re-run this evaluation at every minor version bump; the table format is intentionally identical so diffs are obvious.

## Re-runs needed when LLM path is live

- Run 2 (design jargon overload) — LLM should honor "Pentagram annual report" vocabulary.
- Run 4 (Linear brand colors) — LLM should know Linear's exact palette without being told.
- Run 8 (Korean Hangul + Noto Sans KR + 명조) — LLM should set `typography.body.family: "Noto Sans KR"` and pick a serif for the display step.
- The image / video / 3D runs (5–7) should stay as honest non-goal proofs.
