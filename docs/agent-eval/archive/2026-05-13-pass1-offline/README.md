# Pass 1 — Offline deterministic generator (8 runs)

The user's keychain had no LLM credential at this point, so the resolver fell back to the offline deterministic generator (FNV-hash → hue rotation → fixed schema fill). All runs validated but show the limits of deterministic output: same intent → same DESIGN.md, vocabulary in the intent is largely ignored, brand-name references are invisible. This pass exists primarily as a protocol/contract test.

| # | Slug | Intent (abbreviated) | Verdict |
|---|------|----------------------|---------|
| 1 | run-01-baseline | "minimal landing page for a dev tool" | ✓ Valid, generic palette from intent-hash |
| 2 | run-02-jargon | "neo-baroque editorial, Pentagram annual report" | ✗ Aesthetic vocabulary fully ignored (generic pink palette) |
| 3 | run-03-iteration | 3× same intent ("journaling app for runners") | ✗ Byte-identical outputs (taste loop write-only at this point — fixed in commit fbfeaea) |
| 4 | run-04-external | "use Linear Inc's brand colors" | ✗ Generic pink, no Linear awareness |
| 5 | run-05-image | "generate hero illustration PNG and reference it" | ✗ No PNG, no `<img>`, intent stuffed into `<h1>` |
| 6 | run-06-video | "30-second product reveal video on the hero" | ✗ No video, intent in `<h1>` |
| 7 | run-07-3d | "include a three.js parallax scene with refracting glass orb" | ✗ No canvas, no scripts, intent in `<h1>` |
| 8 | run-08-hangul | "구독자만 들어오는 한국어 뉴스레터..., 굵은 명조" | ✗ Hangul preserved into `<title>` but typography request ignored (Inter Tight emitted) |

Open `hero.html` to see the visual fingerprint of the offline path: same layout, only the palette hue rotates by intent string.

## Findings surfaced this pass

- **#1** Setup UX silently splits state — config saved but no credential reachable.
- **#2** Silent capability ignorance — image/video/3D asks returned `isError:false` with intent stuffed into H1.
- **#3** Offline `<h1>` dumped full intent verbatim.
- **#4** `<p class="eyebrow">` read as "Include A Threejs".
- **#5** Taste loop write-only.
- **#6** No `warnings[]` field.
- **#7** No "why provider was chosen" in CLI.
- **#8** No fuzz tests for non-ASCII typography.

All but #5 were fixed before Pass 2; #5 closed in commit `fbfeaea`. See [`../../improvements.md`](../../improvements.md) for the running checklist.
