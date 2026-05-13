# Pass 3 — z.ai GLM-5.1 wide breadth (17 runs)

Third dogfood pass, designed to probe Aphrodite across five orthogonal axes:
A) aesthetic styles · B) domains · C) hard constraints · D) languages · E) convergence under repetition.

| # | Slug | Category | Time | Validation | Primary `500` | Display family (excerpt) | Verdict |
|---|------|----------|------|------------|---------------|--------------------------|---------|
| 1 | A1-editorial | aesthetic style | 103 s | ✅ | `#a66430` warm bronze | Fraunces | Editorial-correct serif, restrained warm palette. Ship-worthy. |
| 2 | A2-brutalist | aesthetic style | 92 s | ✅ | `#ffe600` caution yellow | system mono | Genuinely brutalist — high-contrast, no flourish. |
| 3 | A3-glassmorphic | aesthetic style | 116 s | **❌ WCAG-AA fail** | `#e35a1a` glow orange | SF Pro Display | DESIGN.md emitted, contrast gate caught the legitimate glassmorphism vs accessibility tension. Validator did its job. |
| 4 | A4-bauhaus | aesthetic style | 134 s | ✅ | `#e85d30` Bauhaus primary | Archivo Black | Geometric primaries, rounded, heavy sans. Pastel + primary contrast. |
| 5 | A5-cyberpunk | aesthetic style | 91 s | ✅ | `#e60073` neon magenta | (mono) | Dense data layout, dark surface + magenta/cyan accents. |
| 6 | B1-mobile | domain stretch | 95 s | ✅ | `#c2510f` warm coral | (sans) | Thumb-zone optimized; primary CTA bottom-anchored. |
| 7 | B3-pricing | domain stretch | 113 s | ✅ | `#8a6d3b` warm gold | Fraunces | Conversion-focused tier system, trust-coded palette. |
| 8 | C1-forced-palette | constraint | — | ❌ parse-fail | — | — | GLM returned `colors:` map in a non-conforming shape (likely the constraint hex list confused the schema). Open finding. |
| 9 | C2-forced-font | constraint | 78 s | ✅ | `#b8860b` accent | **Berkeley Mono** | Constraint honored exactly — Berkeley Mono in display + body. |
| 10 | D1-japanese | language | 88 s | ✅ | `#8b6d47` khaki | — | 余白を多く respected; the 白と藍 (white + indigo) palette claim was only partially honored — emitted a warm khaki, not indigo. |
| 11 | D2-arabic-rtl | language | 99 s | ✅ | `#c46a35` amber | — | Color matched العنبر (amber) and الكهرماني (amber-yellow) request. RTL layout not surfaced in our hero template (template limitation, not LLM). |
| 12 | D3-emoji | language | 99 s | ✅ | `#b5643a` rose-coral | — | Festive but tasteful. Emoji-laden intent didn't corrupt the schema. |
| 13 | E1-convergence | repetition × 5 | 140 s | ✅ | `#7a7462` warm taupe | — | Baseline call, no taste signal yet. |
| 14 | E2-convergence | repetition × 5 | 82 s | ✅ | `#8a6d47` warm brown | — | Same family. Natural LLM variance. |
| 15 | E3-convergence | repetition × 5 | 103 s | ✅ | `#3a6f63` deep sage | Libre Caslon Display | **Jumped families.** Sage green instead of warm earth. |
| 16 | E4-convergence | repetition × 5 | 83 s | ✅ | `#7a7062` warm taupe | Newsreader | Back to warm. |
| 17 | E5-convergence | repetition × 5 | 85 s | ✅ | `#448264` green-teal | Newsreader | Green family again, different shade. |

## Aggregate

- **Validation rate**: 15 / 17 (88%). After the parser-leniency fix (commit landed this session) the original 4 frontmatter-prefix failures all passed.
- **Latency mean**: 100 s (range 78–140 s).
- **Warnings emitted**: 0 across all runs (intents were in-scope).
- **Validator did its job**: A3-glassmorphic was caught for WCAG-AA. Production gate works.
- **Constraint following**: C2 honored verbatim (Berkeley Mono); C1 broke the parser due to a schema-shape mismatch (open finding).

## Convergence series (E1–E5) — palette evolution

Same intent, same seed, five sequential calls:

| Run | Primary 500 | Hue family | Notes |
|-----|-------------|------------|-------|
| E1 | `#7a7462` | warm taupe | baseline |
| E2 | `#8a6d47` | warm brown | same family, slight shift |
| E3 | `#3a6f63` | deep sage | **jumped** to green |
| E4 | `#7a7062` | warm taupe | reverted to warm |
| E5 | `#448264` | green-teal | green again |

**Caveat**: this run set did **not** call `redesign` between iterations, so the *Regenerate-count* signal in the taste store never incremented. The palette variation across E1–E5 is therefore *natural LLM sampling variance*, not a measured taste-loop response. The proper convergence test (Pass 4 candidate) needs `aphrodite redesign` calls between each `design` call to accumulate Regenerate events and exercise the read-at-generate-time injection (commit `fbfeaea`).

## New findings (will appear in `../../improvements.md`)

- **#16** parser still trips on some LLM YAML shapes (C1's `colors:` map). Need a more forgiving frontmatter normalisation step, or save the raw LLM response when parse fails so the user can inspect.
- **#17** hero template has no RTL flow — D2's Arabic content renders LTR. CSS `dir="rtl"` injection based on language detection is a small future fix.
- **#18** convergence test methodology bug — script should `redesign` between calls. Re-run for the next pass.

## How to inspect

Open any `r{NN}-{slug}/hero.html` in a browser. Each is fully self-contained.

```bash
open A2-brutalist/hero.html
open D1-japanese/hero.html
# etc.
```

DESIGN.md files validate independently:

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call",
       "params":{"name":"validate","arguments":{"design_md_path":"$PWD/A2-brutalist/DESIGN.md"}}}' \
  | aphrodite-mcp 2>/dev/null
```
