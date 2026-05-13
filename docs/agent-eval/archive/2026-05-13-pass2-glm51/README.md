# Pass 2 — z.ai GLM-5.1 live (4 runs)

After the keychain bug chase (commits a12ae1d, 782ecba, 253b7b1, c0c1eed, 54987b4) the user's API key finally landed and `aphrodite doctor` returned `● ready for real provider calls`. These four runs verified the live LLM path end-to-end.

| # | Slug | Intent (abbreviated) | Time | Verdict |
|---|------|----------------------|------|---------|
| 1 | r01-baseline | "minimal landing page for a dev tool: bold serif headline, monospace body, single accent" | 86 s | ✓ Newsreader + JetBrains Mono + coral. Competent, ship-with-refresh. |
| 2 | r02-jargon | "neo-baroque editorial, Pentagram annual report energy, restrained gold accent" | 94 s | **✓✓ Best of the set.** Gold #8b6914 + Luminance Garamond + DM Sans. Real Pentagram-coded output. |
| 3 | r03-linear | "use Linear Inc's brand colors as primary palette" | 99 s | ✓ but inaccurate — graphite #58584d, not Linear's actual #5E6AD2. Type system reads right; color claim doesn't. |
| 4 | r04-hangul | "구독자만 들어오는 한국어 뉴스레터, 진하고 따뜻, 굵은 명조, Noto Sans KR" | 77 s | **✓✓ Exceptional.** Noto Serif KR for 굵은 명조 + Noto Sans KR body + warm terracotta #b85a2e. Even referenced 한지 in rationale. |

All four cleared WCAG-AA across light + dark + 2 brand variants.

Open `hero.html` in any run dir for the actual visual.

## Findings surfaced this pass

- **#11** parser rejected GLM responses with prose-prefix before `---`. Fixed same session, commit in pass-2 testimony.
- **#12** headline derivation defaults to "Meet {name}." for all four (LLM descriptions exceed 80-char cap). Open.
- **#13** brand-name palette recall is weak (Run 3). Open.
- **#14** 80–100 s latency makes tight iteration painful. Open.
- **#15** `doctor` lacks last-call status. Open.
