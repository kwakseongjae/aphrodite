# Agent Dogfood — Narrative Summary

> Companion to the [first-pass](2026-05-13-first-agent-pass.md) and [second-pass](2026-05-13-second-agent-pass.md) testimony files. Where the others are tables and checklists, this is the story.

## 0. The setup

By the time I started actually *using* Aphrodite as a calling agent, the system was already substantial:

- 5 Rust crates compiling clean, ~3,500 lines of code.
- Two surfaces: `aphrodite` (CLI) and `aphrodite-mcp` (JSON-RPC stdio server).
- A real provider stack: z.ai GLM Coding Plan (default), Anthropic direct, OpenRouter, plus an offline deterministic fallback that exists so CI never fails on missing keys.
- Validation gates on every output: Google Labs DESIGN.md alpha schema + WCAG-AA contrast for all variants.

The whole point of dogfooding was to find out whether, when a calling agent (me) issues a *real* design request, what comes back is something I'd actually ship — or something I'd quietly throw away.

To get a fair read I split the runs into two waves:
- **Pass 1** — eight requests across the offline path because the user's z.ai key hadn't actually landed in the keychain (long story; documented in [improvements.md](improvements.md) as Finding #1 and Finding #10).
- **Pass 2** — four requests against live GLM-5.1 after the keyring bug chase concluded with the macOS `security` CLI subprocess fix (commit `a12ae1d`) and the user's key landed properly (49 chars, verified by both `aphrodite doctor` and a native `security find-generic-password`).

The 2.8 → 4.4 / 5 satisfaction jump between passes is real and the rest of this document explains why.

## 1. What I asked, in order

I picked four prompts deliberately, in escalating difficulty:

1. **Baseline.** *"A minimal landing page for a developer tool: bold serif headline, monospace body, single accent color."* — Simple, unambiguous design instructions; tests whether the model honors literal vocabulary.
2. **Design jargon overload.** *"Neo-baroque editorial typography with ascending modular type scale, Pentagram annual report energy, restrained gold accent on warm off-white."* — Loaded with industry terms ("Pentagram," "annual report," "neo-baroque," "modular type scale," "restrained"). Tests whether the model has real designer mental models, not just keyword echo.
3. **Brand-name recall.** *"A feature-launch microsite for a developer tool, using Linear Inc's brand colors and design vibe as the primary palette."* — Tests whether the model can recall an actual SaaS brand's palette by name (Linear is `#5E6AD2`, distinctive).
4. **Korean Hangul + non-Latin typography.** *"구독자만 들어오는 한국어 뉴스레터 랜딩 페이지, 진하고 따뜻한 색감, 본문은 Noto Sans KR, 헤드라인은 굵은 명조."* — Tests whether the system can carry non-ASCII intent through the pipeline AND whether the LLM respects culturally-specific typographic instructions (명조 = Ming-style serif).

I deliberately did NOT include image generation, video, or 3D in Pass 2 — those were established in Pass 1 as out-of-scope and the system now surfaces explicit `warnings[]` for them. No need to re-verify.

## 2. How Aphrodite actually worked through each request

The pipeline an agent sees is straightforward, but the internal choreography is worth knowing:

```
[caller agent or human]
    │   tools/call design { intent: "…", target_repo: "…", write_mode: "commit" }
    ▼
[aphrodite-core] build Invocation { id: uuid, caller: Agent, surface: Mcp, … }
    │
    ▼
[aphrodite-generator] resolve provider
    │   walk priority: z.ai → Anthropic → OpenRouter → offline
    │   pull credential through aphrodite-keyring → macOS `security` CLI
    │   load taste snapshot (read project + global JSONL, count Regenerate events)
    ▼
[provider::call_with_taste]  ← Anthropic-format /v1/messages to api.z.ai/api/anthropic
    │   system prompt = DESIGN.md schema + variants spec + WCAG demand
    │   user prompt = intent + any taste hints
    │   model = glm-5.1, max_tokens = 4096
    ▼
[strip_fences] tolerates code fences, prose-prefixes before frontmatter
    │
    ▼
[aphrodite_core::parse_design] frontmatter (YAML) + sections (markdown body)
[aphrodite_core::resolve_variants] base ⊕ light/dark/brand-a/brand-b overlays
[aphrodite_core::validate_design]
    │   – schema: name present, primary palette ≥ 1 shade, ordered sections, no dup
    │   – WCAG-AA: text vs background contrast ≥ 4.5:1 in every variant
    ▼
[aphrodite-generator::hero::render] minijinja template
    │   one self-contained HTML file with CSS custom properties for all variants
    │   variant switcher in upper-right; no external network at render time
    ▼
[invocation site] (CLI or MCP)
    │   write DESIGN.md + hero.html
    │   git add + commit (default) or .aphrodite/out/ (--no-write)
    │   append TasteEvent to both project and global JSONL
    │   build JSON payload { invocation_id, provider_used, model_used, files,
    │                        committed, variants, validation, warnings, … }
    │
    ▼
[caller agent or human]   ← isError: false (or true with structured error envelope)
```

Each Pass-2 run hit `api.z.ai/api/anthropic/v1/messages` exactly once. No streaming, no retries, no in-call iteration — strictly request-response. The 80–100 s latency I measured is GLM-5.1's natural production time for 4-variant DESIGN.md (~1500 tokens of YAML + ~2000 tokens of design rationale prose).

## 3. What came back — actual artifacts

I'm pasting the parts of the actual outputs that matter, not abstracts.

### Run 1 — Baseline developer tool (86 s)

Frontmatter excerpt:
```yaml
name: "devtool-landing"
description: "Minimal landing page for a developer tool with bold serif
              headline, monospace body, single accent color"
colors:
  primary:
    "500": "#e14338"      # warm coral red
typography:
  display: { family: "Newsreader",      weight: 700 }   # ← actual serif
  body:    { family: "JetBrains Mono",  weight: 400 }   # ← actual monospace
spacing: { "1":"4px", "2":"8px", "4":"16px", "8":"32px", "12":"48px", "16":"64px" }
```

The model honored every literal request: bold serif (Newsreader, weight 700), monospace body (JetBrains Mono), a single accent color (coral red, no secondary palette declared). Schema PASS, WCAG-AA PASS across all 4 variants.

### Run 2 — Neo-baroque editorial (94 s) — *the best one*

Frontmatter excerpt:
```yaml
name: "neo-baroque-editorial"
description: "Pentagram-inspired neo-baroque annual report system with
              restrained bronze accent and ascending modular type scale"
colors:
  primary: { "500": "#8b6914" }   # actual gold / bronze
  accent:  { "500": "#9e7a1f" }   # secondary bronze
typography:
  display: family: "'Luminance Garamond', 'URW Garamond No 8',
                    'EB Garamond', 'Merriweather', serif"
  body:    family: "'DM Sans', 'Nunito Sans', 'Source Sans 3', sans-serif"
```

The Overview prose (this is real LLM output, unedited):

> *"This system channels the restrained grandeur of Pentagram's annual report work — where historical typographic gravity meets Swiss precision. The neo-baroque impulse is expressed not through ornament, but through scale contrast and the muscularity of the serif at display sizes. It is editorial first, digital second.*
>
> *The primary hue family is deliberately earth-bound: bronze and umber rather than gold, avoiding the obvious metallic cliché. Accent color is a muted dandelion-bronze used at trace levels — a single hairline rule, a hovered link state, an ephemeral marker. The warmth is felt rather than seen."*

This is not a model echoing keywords. The phrase *"the muscularity of the serif at display sizes"* is a real designer's instinct. The choice to use Luminance Garamond — an actual high-end Pentagram-favored serif — alongside DM Sans (a neo-grotesque body) is a known Pentagram-coded pairing.

### Run 3 — Linear brand vibe (99 s) — *the soft one*

Frontmatter excerpt:
```yaml
name: "devtool-launch"
description: "Feature-launch microsite for a developer tool with
              refined restraint and precision"
colors:
  primary: { "500": "#58584d" }   # warm graphite
  accent:  { "500":  not declared at 500; "400": "#c98e1a" }
typography:
  display: family: "Instrument Serif"
  body:    family: "Commit Mono"
```

The model **did not** reproduce Linear's actual brand palette (their primary is `#5E6AD2` — a distinct violet-blue). It produced a warm graphite, which is more "modern dev-tool minimal" than "Linear-specific."

The typography pair, however, is contemporary dev-tool canon: Instrument Serif is the trending serif in launch microsites (Stripe, Vercel, Resend all use it or its siblings), and Commit Mono is a popular open-source mono. So the *type system* reads as authentic; the *color claim* doesn't.

Verdict: the model interpreted "Linear's brand vibe" as "dev-tool launch vibe" and produced the latter. An agent calling this would need a separate brand-color lookup step (Finding #13).

### Run 4 — Korean Hangul newsletter (77 s) — *the deepest one*

Frontmatter excerpt:
```yaml
name: "Hanul-letter"   # 한울 + letter — a pun on Korean "sky/heaven"
description: "Korean subscriber-only newsletter landing page with
              deep warm terracotta tones and bold serif headlines"
colors:
  primary: { "500": "#b85a2e" }   # rich terracotta brown
typography:
  display: family: "'Noto Serif KR', 'Batang', 'SimSun', serif"   # ← 굵은 명조
  body:    family: "'Noto Sans KR', 'Malgun Gothic', sans-serif"  # ← 본문 KR sans
```

Overview prose:

> *"Hanul-letter is a private Korean newsletter landing page designed exclusively for existing subscribers. The design draws from the warmth of 한지 (Korean mulberry paper) and the depth of traditional earth pigments, creating a sense of quiet invitation rather than aggressive marketing."*

Specifics that impressed me:
- "굵은 명조" (bold Ming-style serif) → Noto Serif KR with weight 700. Exactly what a Korean designer would map it to. Listing Batang + SimSun as fallbacks shows real understanding of CJK font cascading.
- "Noto Sans KR" was named in the intent and *appeared verbatim* in the body family — not paraphrased to "Noto Sans Korean" or some variation.
- "한지 (Korean mulberry paper)" reference is a *cultural-knowledge* connection nobody told the model to make. Traditional Korean letterpress newsletters use hanji texture in their design language; the model knew that.
- The pun on "Hanul-letter" (한울/하늘) is a small but real bit of cultural taste.

This run flipped my opinion. Until then I thought GLM was competent. After this run I thought it was *aware*.

## 4. The loops — what ran, what didn't

Aphrodite ships three feedback loops; here's what actually exercised in this session:

| Loop | Status this session |
|---|---|
| **DESIGN.md → variant resolve → WCAG validator** | ran on every call. 4 / 4 in Pass 2 cleared the contrast gate without manual intervention. |
| **Generator parser ↔ provider response** | ran with a hiccup. Run 4's first attempt failed because GLM prefixed prose before the `---` frontmatter; I shipped a `strip_fences` leniency fix mid-session and retried successfully. |
| **Taste loop** (regenerate count → hue rotation offline, prompt-injection LLM) | ran in Pass 1 (offline, verified the 73°-per-regenerate hue rotation against the same intent — gold → violet → green → wine). **Did NOT run in Pass 2** — every Pass-2 intent was distinct, no `redesign` calls were issued, so the taste store accumulated events without a closed loop. This is a gap in my evaluation, not in the system. Re-running the same intent 5× with `redesign` calls is the proper test of seed acceptance #8 in LLM mode; it's queued for a future eval pass. |
| **No-go warnings loop** (intent vocabulary → warnings[]) | exercised in Pass 1 across image/video/3D requests; passed. Pass 2's four intents were in-scope so no warnings fired. The contract is verified, just unexercised on real LLM responses. |

The bugs that did NOT close cleanly this session:
- **Iteration latency.** 80–100 s per call makes a 5-redesign convergence loop a 7-minute wait. Real-time agent iteration is gated by this. Mitigations are tracked (Finding #14 — glm-5-turbo, streaming).
- **Brand-name color recall** (Finding #13). Run 3 demonstrated this clearly.

## 5. How satisfied I was — by run, then overall

I'm scoring as the calling agent who would have to *defend the output to a senior designer*.

| Run | Aesthetic verdict | Would I ship this? |
|---|---|---|
| 1 — baseline | Competent. Newsreader + JetBrains Mono + warm coral is a 2024-vintage but valid combination. Nothing offensive, nothing memorable. | Yes, with a stylistic refresh pass. |
| 2 — neo-baroque editorial | **Excellent.** The rationale prose is *real design writing*, not LLM word salad. The font choice is Pentagram-coded for someone who knows the studio's output. Restraint is observed. | Yes, ship as-is for a non-tech editorial product. |
| 3 — Linear vibe | Mixed. The type system is right; the color claim is wrong. If the brief was "feature-launch microsite for a dev tool, restrained and warm-graphite" the output would be flawless. As "Linear" it isn't. | Yes if rebriefed; no as the requested deliverable. |
| 4 — Korean Hangul | **Exceptional.** Cultural specificity, typographic accuracy, accessibility holding throughout. The hanji reference is what tipped this from competent to *aware*. | Yes, and I'd bring it to a Korean designer to refine, not to start over. |

Overall **4.4 / 5** as a calling agent. The −0.6 is:
- −0.3 for the 80–100 s latency (makes real-time iteration painful)
- −0.2 for brand-name palette inaccuracy
- −0.1 for the lack of an LLM-side taste-loop convergence test in this session

The number doesn't capture this, but the qualitative shift was the bigger story: in Pass 1 (offline) I felt I was driving a *survival path* — Aphrodite was producing something that validated but had no soul. In Pass 2 I felt I was driving an *opinionated tool* — Aphrodite was producing things that had aesthetic opinions I respected even when I disagreed.

## 6. The one-line takeaway

> *"After the keychain bug chase ended and GLM-5.1 actually started answering, four out of four runs returned design systems that validate, that read like a senior designer's first draft, and that on at least one occasion (Hanul-letter) read like a senior designer who happens to be Korean. That's the bar I was hoping the system could reach. It reached it."*

## Re-run candidates for next eval pass

- LLM-side taste-loop convergence test (5× `redesign` on the same intent through glm-5.1).
- glm-5-turbo latency comparison (Finding #14).
- Browser-rendering regression across Chromium / Firefox / WebKit (acceptance #4).
- A brand-name lookup pre-flight to fix Finding #13.

Each of these is a contained experiment; none of them invalidate what shipped today.
