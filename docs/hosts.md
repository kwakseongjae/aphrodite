# Hosting Aphrodite inside an AI coding agent

Aphrodite ships as an MCP server (`aphrodite-mcp`) so any host that speaks MCP — Claude Code, Codex CLI, OpenCode, Hermes, Cursor, Kiro — can call it directly. The contract is identical across hosts; only the registration file differs.

Once registered, the host can call four tools:
- `design` — generate DESIGN.md + hero HTML
- `redesign` — same, records an implicit "didn't like that one" signal
- `validate` — check an existing DESIGN.md against schema + WCAG-AA
- `auth_status` — list configured providers (no secrets returned)

## Prerequisite (one-time, on the human's machine)

```bash
cargo install --path crates/aphrodite-cli   # gives `aphrodite`
cargo install --path crates/aphrodite-mcp   # gives `aphrodite-mcp`
aphrodite auth set zai                      # paste z.ai GLM key → OS keychain
# or:  aphrodite auth set anthropic / openrouter
```

After this, *any* agent host registers `aphrodite-mcp` and the credential is reused — no per-host secret config.

## Claude Code

User scope (`~/.claude.json`):

```json
{
  "mcpServers": {
    "aphrodite": {
      "command": "aphrodite-mcp"
    }
  }
}
```

Project scope (`<repo>/.mcp.json`) — same shape. After editing, restart Claude Code; the four tools surface as `mcp__aphrodite__design` etc.

## Codex CLI

`~/.codex/config.toml`:

```toml
[mcp_servers.aphrodite]
command = "aphrodite-mcp"
```

Codex auto-discovers and exposes the tools to its session model.

## OpenCode

`~/.config/opencode/opencode.json`:

```json
{
  "mcp": {
    "aphrodite": {
      "type": "local",
      "command": ["aphrodite-mcp"],
      "enabled": true
    }
  }
}
```

## Hermes Agent

Hermes uses the cross-agent Skills Hub. Register the binary as an MCP server in `~/.hermes/mcp.json`:

```json
{
  "mcpServers": {
    "aphrodite": { "command": "aphrodite-mcp" }
  }
}
```

## Cursor

`~/.cursor/mcp.json` (same shape as Claude Code).

---

## Calling pattern (host-agnostic)

Once registered, the agent calls Aphrodite like any other MCP tool. Recommended sequence:

1. **First call per session** (cheap, returns the contract):
   ```
   tools/call auth_status
   ```
   Confirms a provider is configured before spending tokens.

2. **The main work**:
   ```
   tools/call design {
     "intent": "<freeform design brief>",
     "target_repo": "/absolute/path/to/repo",   // defaults to MCP server cwd
     "write_mode": "commit" | "artifact_only"   // default commit
   }
   ```

3. **Inspect the response**:
   - `isError: false` + `structuredContent.validation.ok == true` → done.
   - `isError: true` → consult `structuredContent.error.hint` for a recovery branch:
     - `auth_failed` → ask the human to run `aphrodite auth set …`
     - `target_repo_invalid` → fix the path and retry
     - `rate_limited` → backoff
     - `provider_outage` → unset keys to force offline, or wait

4. **Iteration**:
   ```
   tools/call redesign { same args }
   ```
   Each call appends an implicit Regenerate signal to the per-project + global taste store. (Active learning loop lands in v0.1's T1 milestone; storage already works.)

## What the agent should NOT do

- Do not parse `content[0].text` by hand. Use `structuredContent`.
- Do not retry an `auth_failed` automatically — escalate to the human.
- Do not invoke `design` outside an existing git repo with `write_mode=commit`; the server will reject with `target_repo_invalid` and a clear hint, but agents that ignore the hint will look broken to the human.

## Pricing intuition for the human

Aphrodite makes one provider call per `design`/`redesign` invocation. With z.ai GLM Coding Plan the marginal cost is bundled into the subscription; with Anthropic direct it's ~1 Sonnet call (~4k output tokens). Offline costs $0 — keep that as a fallback for CI.
