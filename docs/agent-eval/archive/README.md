# Agent-eval archive

User-inspectable archive of every Aphrodite dogfood run. Each subdirectory below contains the *exact* artifacts the harness emitted for each test intent — no edits, no trimming. Open the `hero.html` files directly in a browser to see what the calling agent actually received.

## What's here

- `2026-05-13-pass1-offline/` — first pass (offline deterministic generator), 8 intents covering baseline, jargon, iteration, brand reference, image generation, video, three.js, Hangul.
- `2026-05-13-pass2-glm51/` — second pass (live z.ai GLM-5.1), 4 intents validating the LLM path after the keychain bug chain ended.
- `2026-05-13-pass3-glm51/` — third pass (live z.ai GLM-5.1), wider breadth: 5 aesthetic styles + 2 domain stretches + 2 constraint tests + 3 languages + 5-step taste-loop convergence.

## What's in each run directory

Each `r{NN}-{slug}/` contains some combination of:

- `intent.txt` — verbatim intent string that was passed to `aphrodite design`.
- `result.json` — the structured JSON payload returned to the calling agent (provider, model, validation report, warnings, file paths, committed flag).
- `DESIGN.md` — the Google-Labs-alpha schema artifact that Aphrodite emitted. Frontmatter (YAML) + sections (markdown). Run the validator yourself: `aphrodite-mcp` with `validate { design_md_path: "…" }`.
- `hero.html` — self-contained HTML page that uses the DESIGN.md tokens. No external network at render time. Just open it in a browser. Variant switcher in the top-right toggles light / dark / brand-a / brand-b.
- `duration.txt` — wall-clock seconds the LLM call took (Pass 2+ only).
- `err.txt` (if present) — stderr output, usually empty.

## Reading order recommended

1. **`docs/agent-eval/2026-05-13-narrative.md`** — the story across passes.
2. **`docs/agent-eval/2026-05-13-first-agent-pass.md`** — Pass 1 testimony (frozen).
3. **`docs/agent-eval/2026-05-13-second-agent-pass.md`** — Pass 2 testimony (frozen).
4. **`docs/agent-eval/2026-05-13-third-agent-pass.md`** — Pass 3 testimony (frozen, lands on completion).
5. **`docs/agent-eval/improvements.md`** — append-only backlog of every finding across all passes.
6. **`docs/agent-eval/archive/`** — *this directory*. Open the actual `hero.html` files.

## How to use this archive for the next iteration

When Aphrodite's behavior changes, re-run the same intents and put the new outputs alongside. Then `diff` the DESIGN.md frontmatter palettes and font choices between versions. The intents are intentionally stable for that reason — they're a regression suite for the aesthetic surface.
