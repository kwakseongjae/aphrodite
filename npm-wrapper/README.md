# Aphrodite — the agent

> A CLI agent that generates Korean-first, production-grade multi-page brand sites + a publishable React component package from a single intent string.

```bash
npm i -g @aphrodite-design/aphrodite-agent
aphrodite auth set zai              # one-time: paste your z.ai API key
aphrodite create "한식 정기 구독 브랜드 — 모바일 우선, 따뜻한 톤" --pages home,menu,pricing,faq,about
```

What gets emitted (into your current directory):

- `DESIGN.md` — Google Labs alpha-schema design contract with 4 brand variants
- `<page>.html` × N — multi-page responsive site
- `tokens.{css,json,figma.json}` — design-system handoff
- `components.html` + `docs/index.html` — Storybook-style designer previews
- `react/` — publishable npm package (70 typed components + stories + tokens + workflows)
- Per-page screenshots at 3 viewports

## Requirements

- Node ≥ 18 (just for the postinstall wrapper)
- An API key from one of:
  - **z.ai** (recommended, cheapest, https://zai.com) — ~$0.10 per run
  - Anthropic Claude (claude.ai/settings/keys)
- **macOS** (Apple Silicon or Intel). Linux + Windows arriving in 1.0.

## Cost

A single `aphrodite create` run ≈ 8 LLM calls ≈ **$0.10 z.ai** (with `glm-5.1` + `glm-4.5-air` mix).

## What's an "agent" vs a "component library"?

You install Aphrodite once. Then you run it to generate **your own brand's design system** — your colors, your typography, your components — emitted as a fresh npm package you can publish under your own org via `aphrodite publish --rename @yourorg/yourbrand`.

This package (`@aphrodite-design/aphrodite-agent`) is the agent. Your brand is yours.

## Status

`0.0.0` — name-reservation. Real beta arriving as `1.0.0-beta.1`. See [roadmap](https://github.com/kwakseongjae/aphrodite/blob/main/docs/roadmap-1.0.md).

## License

Apache 2.0.
