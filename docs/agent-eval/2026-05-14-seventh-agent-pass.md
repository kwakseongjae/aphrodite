# Agent Evaluation — 2026-05-14 — Seventh Pass (reactive multi-turn)

> Pass 6 verified that `aphrodite refine` honors a *pre-scripted* sequence of surgical edits. The user pointed out — correctly — that this wasn't actually multi-turn *conversation*. It was a checklist of pre-planned deltas. Pass 7 closes that gap: the calling agent (me) looks at each turn's output, forms a genuine reaction, and writes the next delta in response. No script.

## Methodology — what makes this "reactive"

Before running anything I wrote down my pre-stated taste hypotheses for the intent ("solid walnut furniture portfolio, Seoul"):

- warm tones expected, but not "sweet"
- serif display preferable, but **fancy / olde-style is a deal-breaker**
- generous spacing for gallery feeling
- editorial work-grid + about + contact structure

These were committed *before* turn 1 ran. They serve as a forensic check on whether my later reactions were genuine or post-hoc rationalization.

For each turn:
1. Run `aphrodite design` or `aphrodite refine`.
2. **Actually read** the DESIGN.md frontmatter, the prose body, and the rendered composition.html structure.
3. Form a reaction: what's wrong? What's the *single most* unsatisfying thing?
4. Write the next delta. Run again.

Deltas were *not* prepared in advance — turn 2's delta was written after turn 1 landed, turn 3's after turn 2, and so on.

## Results — 4 turns, 4 surgical edits, 1 satisfied agent

| Turn | display | body | max spacing | brand-a bg | primary 500 |
|---|---|---|---|---|---|
| 1 — baseline | EB Garamond | Outfit | 96 px | #f7f3ed | #9a7550 |
| 2 — display swap | **Source Serif 4** | Outfit | 96 px | #f7f3ed | #9a7550 |
| 3 — spacing extend | Source Serif 4 | Outfit | **224 px** | #f7f3ed | #9a7550 |
| 4 — brand-a tint | Source Serif 4 | Outfit | 224 px | **#faf9f6** | #9a7550 |

Each turn, **exactly one cell changes**. Body (Outfit) and primary-500 (#9a7550 walnut) survive all four turns untouched. Spacing in turn 3 extends *only* the upper end — the 4-24 px tokens the agent explicitly asked to preserve are byte-identical.

## What I actually reacted to (turn by turn)

### Turn 1 → Turn 2: "EB Garamond is wrong register"
EB Garamond hit my pre-stated dislike for "fancy olde-style" serifs almost exactly. The DESIGN.md body even acknowledged the tension: *"its humanist irregularity mirrors the subtle imperfections in hand-worked wood"* — but that's renaissance manuscript thinking, not Seoul workshop thinking. I named three replacement candidates (Source Serif 4, Newsreader, Fraunces); the system picked the first one. Surgical change.

### Turn 2 → Turn 3: "your prose makes a promise your tokens don't keep"
The Layout section of DESIGN.md says *"Margins are generous — minimum 24px on mobile, scaling to 80px or more on desktop"* and *"pages breathe."* But the actual spacing scale capped at 96 px. For a magazine-paced furniture portfolio that's barely enough for a section padding, let alone full-bleed editorial pacing between projects. I asked for ≥160 and ≥200; got 128 / 160 / 192 / 224 — overdelivered on the upper bound while keeping the lower 4-96 untouched.

### Turn 3 → Turn 4: "parchment background competes with walnut photography"
By this point I was mostly satisfied. The remaining gripe was small: brand-a's `#f7f3ed` background read as parchment-yellow. For photo-dominant portfolios where walnut grain has to carry visual weight, you want a near-neutral gallery wall, not a competing warm tint. I proposed `#faf9f6` or `#fbfaf7`; system landed exactly on `#faf9f6`. Notably, the request "only touch brand-a, leave the other three variants alone" was respected to the byte.

After turn 4 I called the session accepted — no further reactive delta was forming.

## Side effects worth noting (not failures, just findings)

**(a) Prose self-consistency is maintained automatically.** Turn 2 changed *only* the font in frontmatter. But the Overview body section in turn-2's DESIGN.md was rewritten to read *"Source Serif 4 was chosen deliberately over geometric or industrial sans-serifs..."* — the LLM understood that leaving the old EB Garamond justification would create a contradiction, and re-authored the narrative. I didn't ask for this and didn't think to ask for it; it happened correctly anyway.

**(b) Composition.html is regenerated every refine, not preserved.** Turn 1's composition had project names "Hakwi Dining Table / Jongno Cabinet / Samcheong Stool..." Turn 3's composition had "Jomun Table / Duru Bench / Garak Cabinet..." — same surface type, same intent, but the actual content was re-drawn. Both name sets are excellent (Korean cultural words / Seoul districts), but a designer who liked the *layout* of turn 1 and only wanted to change spacing would be annoyed that their composition got rebuilt. This is **Finding #25** — refine should consider keeping composition structure when only frontmatter tokens changed.

**(c) Composition emits `'Source Serif 4'` as font-family but doesn't `@import` the Google Fonts CSS.** So in a user's browser without Source Serif 4 pre-installed (most browsers, since it's an Adobe font), the page falls back to Georgia. The whole point of the deliberate type choice is invisible. **Finding #24** — composer should inject the appropriate `<link rel="stylesheet" href="https://fonts.googleapis.com/...">` or `@import` statement.

**(d) The hero.html still uses generic `ui-sans-serif` system stack.** This was present pre-Pass-7 (Pass 4 onward) — hero.html doesn't consume the DESIGN.md typography tokens. **Finding #26** — hero should use the same CSS-var font setup as composition.

## How this differs from Pass 6 — and why it matters

Pass 6 was scripted: I wrote four delta strings into a bash file before running anything. That validates the *system contract* (each delta lands on the requested axis; other axes preserved). But it doesn't test what a designer actually does, which is *look at the output and decide what's wrong*.

Pass 7 introduces the human-in-the-loop variable. The trajectory of the session — what I reacted to and how I phrased the next request — is shaped by what I see. Specifically:

- I would not have written turn 4 if turn 3's composition had had a clearly visible parchment-vs-walnut conflict on screen. The fact that I went looking for that conflict in `composition.html` source means my acceptance threshold is hit before pixel-level review.
- I almost wrote a turn 4 about the "12 years / 87 pieces / 24 clients" SaaS-stats row in turn 1's composition, but turn 3's re-compose vaporized them as a side effect. The reactive trajectory changed *because* of side effect (b).

## Honest agent satisfaction (post-Pass-7)

| Aspect | Pass 6 | Pass 7 |
|---|---|---|
| Surgical contract honored across turns | 5 / 5 | 5 / 5 |
| Multi-turn driven by *actual* observation | (not measured) | **5 / 5** |
| Prose self-consistency after token edits | (not measured) | **5 / 5** |
| Composition preservation across non-composition deltas | (not measured) | **2 / 5** (regenerates every time) |
| Font CSS actually loads in browser | (not measured) | **1 / 5** (missing import — Finding #24) |
| Latency stability | 4 / 5 | 4 / 5 (191-254 s/turn) |

**Overall: 4.0 / 5** — the surgical-edit machinery works exactly as designed under reactive load, and the LLM even maintains prose-token consistency for free. The remaining gaps are operational: font import, composition preservation, hero typography hookup.

## Open findings carried into Pass 8

- **#24** composer doesn't emit Google Fonts `@import` — designer's font choice invisible in default browser
- **#25** refine re-composes always; could fast-path when only token-level frontmatter changed
- **#26** hero.html still uses system font stack, ignores DESIGN.md typography tokens
- **#13** (carried) brand-name palette recall
- **#23** (carried) spacing-only refinement still triggers full re-compose

## Reproduction

```bash
# Sandbox
mkdir -p /tmp/pass7-reactive && cd /tmp/pass7-reactive
git init -q -b main
rm -f ~/.aphrodite/taste/preferences.toml .aphrodite/preferences.toml

aphrodite design "a portfolio site for an independent furniture maker working in solid walnut, based in Seoul" --require-llm
# look at DESIGN.md + composition.html, decide what's wrong

aphrodite refine "EB Garamond reads too Renaissance / literary — wrong register for a Seoul craftsperson workshop. Replace with Source Serif 4, Newsreader, or Fraunces. Keep palette, spacing, body."
# look again, react again

aphrodite refine "spacing tops out at 96px which doesn't deliver the 'pages breathe' your prose promised. Push tokens to 160 / 200+. Keep lower end and fonts/colors untouched."

aphrodite refine "brand-a #f7f3ed has a yellow cast competing with walnut photos. Shift to gallery-white #faf9f6. Only brand-a; leave other variants and palette tokens alone."
```

Gallery at [`archive/2026-05-14-pass7-reactive/gallery.html`](archive/2026-05-14-pass7-reactive/gallery.html).
