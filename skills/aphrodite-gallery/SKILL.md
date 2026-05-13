---
name: aphrodite-gallery
description: "Build a single-file gallery.html that previews every Aphrodite run subdirectory under a path. Each run renders as a card with intent text, palette swatches, metadata, and a scaled iframe of the actual output."
triggers:
  - aphrodite gallery
  - eval gallery
  - dogfood gallery
  - "preview all runs"
tags:
  - eval
  - tooling
  - aphrodite
---

# aphrodite-gallery skill

When the user says *"build a gallery for these runs"* or *"show me all the eval outputs"* — call the `aphrodite gallery <DIR>` CLI subcommand.

## Trigger phrases

- "build a gallery for `<path>`"
- "preview all the runs under `<path>`"
- "make a side-by-side of the eval results"

## Behavior

1. Verify the binary exists: `which aphrodite` should resolve.
2. Verify the path is a directory containing subdirectories named like `r01-foo/`, `A1-bar/`, etc. Each subdirectory should contain (some of):
   - `intent.txt` — the original intent string
   - `result.json` — the structured payload Aphrodite returned
   - `DESIGN.md` — the design system spec
   - `composition.html` — the real intent-specific surface (preferred preview)
   - `hero.html` — token showcase fallback
3. Run `aphrodite gallery <DIR>`. This writes `<DIR>/gallery.html`.
4. Tell the user to `open <DIR>/gallery.html` in a browser.

## Output shape

`gallery.html` is a single self-contained HTML file with:
- Dark theme by default.
- One card per run, sorted alphabetically.
- Each card shows: slug + status badge (valid / WCAG fail / parse fail), surface chip if a composition was generated, intent text, provider/model/duration/font metadata, palette swatches (hex on hover), iframe preview of `composition.html` (or `hero.html` fallback) scaled to ~40%, and a link strip for full-size open.

No external resources — open it from disk, no server needed.

## What this is NOT

- This skill does not run any Aphrodite design calls. Use `aphrodite design` (or the `aphrodite-mcp` `design` tool) to produce runs first; this skill only assembles the previews after the fact.
- This is a pure read of `<DIR>` — it doesn't modify any run artifacts.

## Example session

```
$ aphrodite gallery docs/agent-eval/archive/2026-05-14-pass4-glm51-surfaces
✓ wrote docs/agent-eval/archive/2026-05-14-pass4-glm51-surfaces/gallery.html

$ open docs/agent-eval/archive/2026-05-14-pass4-glm51-surfaces/gallery.html
```

## Next step

After the gallery is open, the user typically wants to:
- Compare a Pass-N run against the corresponding Pass-N+1 (one tab per gallery).
- Click "open ↗" on a card to see a single composition full-size.
- Identify the worst-looking card and ask for a re-run with adjusted intent.
