# Improvements backlog — agent-driven

Tracked, checkable, and append-only. Source of truth for "what an agent calling Aphrodite needs that it doesn't get today." Each entry has a discovered-on date, priority, reproduction, suggested fix, and a checkbox that flips when the fix lands.

When a finding here resolves, update the row and link the commit. When a new agent-eval pass exposes more, append it; never silently delete.

---

## Status legend
- 🔴 **P0** — agent can't trust the tool until this is fixed
- 🟠 **P1** — degrades trust or quality, has a workaround
- 🟡 **P2** — would meaningfully improve agent UX
- 🟢 **P3** — polish

---

## Open

| # | Pri | Title | Discovered | Repro | Fix sketch | Fixed in |
|---|-----|-------|------------|-------|------------|----------|
| 1 | 🔴 P0 | Setup UX silently splits state (`config.toml` saved but keychain empty) | 2026-05-13 | `aphrodite init` Step 4 with paste/typing failure → config writes, key doesn't. | Diagnostics in `init` (chars captured + readback). `design` emits `provider_downgraded` warning when config intends ≠ resolved. Pretty CLI prints it. | ✅ shipped (partial: stale config still allowed; warning makes it visible) |
| 2 | 🔴 P0 | Silent capability ignorance — image/video/3D asks return `isError:false` with intent stuffed into `<h1>` | 2026-05-13 | "include three.js parallax" → no canvas, no signal. Same for image/video. | Detect out-of-scope vocab → emit `warnings[]` (4 buckets: image_generation, motion_or_video, three_d_scene, design_tool_roundtrip) with agent-branchable `kind`. | ✅ shipped |
| 3 | 🟠 P1 | Offline generator dumps full intent into `<h1>` verbatim | 2026-05-13 | Long intents → unreadable hero. | First-clause split with whitespace-required terminators (`three.js`/`DESIGN.md` survive), 80-char cap, `Meet {name}.` fallback. | ✅ shipped |
| 4 | 🟠 P1 | `<p class="eyebrow">` reads as imperative ("Include A Threejs") | 2026-05-13 | First-3-words rule. | `derive_name()` skips imperatives + framework/format names; takes first 2 informative words. | ✅ shipped |
| 5 | 🟠 P1 | Taste loop is write-only — `redesign × N` produces byte-identical output | 2026-05-13 (seed #8) | Storage works, read at generate-time absent. | `aphrodite_core::taste_snapshot()` returns project + global counts + recent events. `generate_with` reads at call start. Offline path rotates hue by 73° × project-weighted-regenerate count; LLM path injects "User taste signals so far" section into user prompt. | ✅ shipped — measured: 5 successive runs against same intent produce 4 distinct hue families (gold → violet → green → wine). Far exceeds seed #8's 20% threshold. |
| 6 | 🟡 P2 | Success payloads have no `warnings` field | 2026-05-13 | Provider downgrade, out-of-scope, deprecation — no slot. | Added `warnings: [{kind, message, hint}]` to design/redesign payloads (CLI + MCP). | ✅ shipped |
| 7 | 🟡 P2 | Pretty CLI doesn't explain *why* a provider was chosen | 2026-05-13 | Just "Provider: offline" with no reason. | Provider-downgrade warning above carries the reason. The `Provider:` line itself still doesn't annotate. | ✅ partial |
| 8 | 🟢 P3 | No fuzz tests for non-ASCII typography roundtrip | 2026-05-13 | Hangul worked by accident in run 8. | Unit + property tests for UTF-8 in intent, frontmatter, body. | ⏳ |
| 9 | 🟢 P3 | No `aphrodite capabilities` CLI / MCP introspection | 2026-05-13 | Agents guess scope from docs. | `aphrodite capabilities` CLI shipped — returns structured `in_scope`/`out_of_scope_v01`. MCP-side `capabilities` tool still pending (CLI is enough to unblock humans). | ✅ partial — CLI ships, MCP tool pending |
| 10 | 🟠 P1 | macOS Keychain "Always Allow" first-prompt may silently fail | 2026-05-13 | `store` returns OK while subsequent `fetch` fails. | Store→immediate-fetch verification in `init`/`auth set`; new `auth verify`. Once-per-binary onboarding hint in docs still pending. | ✅ shipped (CLI), doc note pending |

## Done (history)

| # | Date | Title | Commit |
|---|------|-------|--------|
| — | 2026-05-13 | Structured MCP error envelope + target_repo validation | `8b0dd78` |
| — | 2026-05-13 | rpassword swap (broken paste in dialoguer) | `3b95645` |
| — | 2026-05-13 | Banner pivot: photo → identity-led (♀ + figlet wordmark + gradient) | `81fea69` |
| #1 | 2026-05-13 | Setup diagnostics + provider-downgrade warning | (this commit) |
| #2 | 2026-05-13 | Out-of-scope intent vocab → `warnings[]` (4 buckets) | (this commit) |
| #3 | 2026-05-13 | Headline derivation respects `.js`/`.md` mid-word + 80-char cap | (this commit) |
| #4 | 2026-05-13 | `derive_name()` skips imperatives + framework names | (this commit) |
| #6 | 2026-05-13 | `warnings: [{kind,message,hint}]` in design/redesign payloads (CLI+MCP) | (this commit) |
| #10 | 2026-05-13 | `auth verify <provider>` + immediate readback in `auth set` / `init` | (this commit) |
| #5 | 2026-05-13 | Taste loop read-at-generate-time; satisfies seed acceptance #8 | (this commit) |
| #9 | 2026-05-13 | `aphrodite doctor` + `aphrodite capabilities` for one-shot self-diagnosis | (this commit) |
| #11 | 2026-05-13 | `auth set --from-stdin` / `--from-file` + bracketed-paste sanitization (rpassword bypass for terminals where hidden input is unreliable) | (this commit) |

## Re-runs scheduled for next eval pass

When Finding #1 is verified-fixed and a real provider call is live:

1. Run 2 — design-jargon overload (does the LLM actually obey "Pentagram annual report" tone?)
2. Run 4 — Linear brand colors (does the LLM know them by name?)
3. Run 8 — Hangul + Noto Sans KR + 명조 (typography frontmatter populated correctly?)
4. New Run 10 — same intent, 5× `redesign` with the taste-loop fix landed → measure ≥20% token delta (seed #8 acceptance criterion).
5. New Run 11 — `capabilities` tool exists, agent calls it first, design respects.

## How to update this file

When you fix a finding:
1. Move its row from "Open" to "Done (history)" with the commit ref.
2. Replace the open-row checkbox with the commit short-hash.
3. If new findings drop out of the fix, append them to "Open" with a fresh row.

The eval testimony at [`2026-05-13-first-agent-pass.md`](2026-05-13-first-agent-pass.md) is frozen — never edit it. New testimony lives in a new dated file.
