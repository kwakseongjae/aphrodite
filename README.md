# Aphrodite

> *"As if you asked the goddess of beauty herself to build it — undeniably stunning UI, every time."*

Aphrodite is an **open, model-agnostic UI generation harness**. It is not a Claude Code plugin. It is a standalone runtime that any human or AI agent can call to get authentic, production-grade, *beautiful* user interfaces — by orchestrating the best of today's UI tooling (Google Stitch, Figma plugins, Blender MCP, three.js, HyperFrames, and more) under a single **DESIGN.md**-grounded design contract.

## Status

Pre-alpha. The spec is being crystallized via [Ouroboros](https://github.com/Q00/ouroboros) (interview → seed → execute → evaluate → evolve). See `docs/` for the live design notes.

## Pillars

1. **Authentic agency.** Full-trust harness — a calling agent (or human) gets end-to-end authority to plan, design, build, and ship UI work. No half-permissions, no plugin sandbox.
2. **Model-agnostic auth.** OAuth-first where possible (GPT/ChatGPT, Kimi/z.ai, Gemini), API-key fallback (Claude). Reuse existing subscriptions; never lock to one vendor.
3. **DESIGN.md as the design contract.** Every project carries a Google-Labs-compatible `DESIGN.md` — the single source of truth shared across Stitch, Figma, code, and 3D.
4. **Best-tool routing, not kitchen-sink.** Stitch for first-draft UI, Figma for refinement, Blender/three.js for 3D, HyperFrames for motion. Aphrodite picks per task; not all are required.
5. **Open source.** Apache-2.0 (TBD). Inspired in form by `oh-my-openagent`, `OpenHarness/Ohmo`, `opencode`, but scoped narrowly to *UI beauty*.

## Non-goals

- Not a general coding agent. (Use OpenCode / Claude Code / Codex for that — Aphrodite can be invoked *by* them.)
- Not a Claude Code skill or plugin.
- Not a hosted SaaS. Local-first.

## Project layout (post-seed)

```
aphrodite/
├── DESIGN.md                  # Aphrodite's own visual identity (multi-mode)
├── Cargo.toml                 # Rust workspace root
├── crates/
│   ├── aphrodite-core/        # DESIGN.md model, validator, taste store, policy
│   ├── aphrodite-cli/         # `aphrodite` binary
│   ├── aphrodite-mcp/         # `aphrodite-mcp` MCP server binary
│   ├── aphrodite-generator/   # prompt + skill → hero HTML
│   └── aphrodite-keyring/     # keychain abstraction (provider creds)
├── bindings/
│   ├── ts/                    # @aphrodite/sdk (napi-rs)
│   └── py/                    # aphrodite (pyo3)
├── skills/                    # bundled first-party skills (≥1 at v0.1)
├── docs/                      # vision, architecture, ADRs, research
├── .ouroboros/seeds/          # seed_*.yaml — immutable upstream contract
└── examples/
```

## License

TBD (Apache-2.0 leaning).
