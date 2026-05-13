//! `aphrodite-mcp` — JSON-RPC 2.0 over stdio. Implements the minimal MCP
//! surface: `initialize`, `tools/list`, `tools/call`, `notifications/initialized`.
//!
//! Tools (mirror the CLI):
//!   * `design`       — { intent, target_repo?, write_mode? } → artifacts
//!   * `redesign`     — same, with implicit Regenerate signal recorded
//!   * `validate`     — { design_md_path | design_md } → ValidationReport
//!   * `auth_status`  — → list of providers with `configured: bool`

use aphrodite_core::{
    parse_design, record_taste, resolve_variants, validate_design, Caller, Invocation, SignalKind,
    Surface, WriteMode,
};
use serde_json::{json, Value};
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

const PROTOCOL_VERSION: &str = "2024-11-05";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .json()
        .try_init();

    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let mut lines = BufReader::new(stdin).lines();

    while let Some(line) = lines.next_line().await? {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let req: Value = match serde_json::from_str(trimmed) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("aphrodite-mcp: parse error: {e}");
                continue;
            }
        };
        let id = req.get("id").cloned();
        let method = req.get("method").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let params = req.get("params").cloned().unwrap_or(Value::Null);

        // Notifications: no `id`, no response.
        if id.is_none() {
            continue;
        }

        let result = handle(&method, &params).await;
        let envelope = match result {
            Ok(v) => json!({ "jsonrpc": "2.0", "id": id, "result": v }),
            Err((code, msg)) => json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": { "code": code, "message": msg },
            }),
        };
        let out = serde_json::to_string(&envelope)?;
        stdout.write_all(out.as_bytes()).await?;
        stdout.write_all(b"\n").await?;
        stdout.flush().await?;
    }
    Ok(())
}

async fn handle(method: &str, params: &Value) -> Result<Value, (i32, String)> {
    match method {
        "initialize" => Ok(json!({
            "protocolVersion": PROTOCOL_VERSION,
            "serverInfo": { "name": "aphrodite-mcp", "version": env!("CARGO_PKG_VERSION") },
            "capabilities": { "tools": {} },
        })),
        "tools/list" => Ok(json!({ "tools": tools_list() })),
        "tools/call" => tools_call(params).await,
        _ => Err((-32601, format!("method not found: {method}"))),
    }
}

fn tools_list() -> Vec<Value> {
    vec![
        json!({
            "name": "design",
            "description": "Generate a DESIGN.md and hero HTML from an intent string. Writes to the target repo (commit by default).",
            "inputSchema": {
                "type": "object",
                "required": ["intent"],
                "properties": {
                    "intent": { "type": "string" },
                    "target_repo": { "type": "string", "description": "Absolute path. Defaults to MCP server cwd." },
                    "write_mode": { "type": "string", "enum": ["commit", "artifact_only"], "default": "commit" }
                }
            }
        }),
        json!({
            "name": "redesign",
            "description": "Same as `design`, but records an implicit Regenerate taste signal.",
            "inputSchema": {
                "type": "object",
                "required": ["intent"],
                "properties": {
                    "intent": { "type": "string" },
                    "target_repo": { "type": "string" },
                    "write_mode": { "type": "string", "enum": ["commit", "artifact_only"], "default": "commit" }
                }
            }
        }),
        json!({
            "name": "validate",
            "description": "Validate a DESIGN.md file or inline string against the Google Labs schema + WCAG-AA contrast.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "design_md_path": { "type": "string" },
                    "design_md": { "type": "string" }
                }
            }
        }),
        json!({
            "name": "auth_status",
            "description": "List configured providers (no key material returned).",
            "inputSchema": { "type": "object", "properties": {} }
        }),
    ]
}

async fn tools_call(params: &Value) -> Result<Value, (i32, String)> {
    let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let args = params.get("arguments").cloned().unwrap_or(Value::Object(Default::default()));

    let result_json: Value = match name {
        "design" => do_design(args, false).await.map_err(rt)?,
        "redesign" => do_design(args, true).await.map_err(rt)?,
        "validate" => do_validate(args).map_err(rt)?,
        "auth_status" => do_auth_status(),
        other => return Err((-32602, format!("unknown tool: {other}"))),
    };

    Ok(json!({
        "content": [{ "type": "text", "text": serde_json::to_string(&result_json).unwrap() }],
        "structuredContent": result_json,
        "isError": false,
    }))
}

fn rt(e: anyhow::Error) -> (i32, String) {
    (-32000, e.to_string())
}

async fn do_design(args: Value, is_redesign: bool) -> anyhow::Result<Value> {
    let intent = args
        .get("intent")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("missing `intent`"))?
        .to_string();
    let repo: PathBuf = args
        .get("target_repo")
        .and_then(|v| v.as_str())
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let write_mode = match args.get("write_mode").and_then(|v| v.as_str()) {
        Some("artifact_only") => WriteMode::ArtifactOnly,
        _ => WriteMode::Commit,
    };

    let invocation = Invocation {
        id: uuid::Uuid::new_v4().to_string(),
        caller: Caller::Agent,
        surface: Surface::Mcp,
        intent: intent.clone(),
        target_repo: repo.canonicalize().unwrap_or(repo),
        write_mode,
    };

    let output = aphrodite_generator::generate(&invocation).await?;
    let report = validate_design(&output.design_doc, &output.variants);

    let (design_path, hero_path, committed) = match invocation.write_mode {
        WriteMode::ArtifactOnly => {
            let out = invocation.target_repo.join(".aphrodite").join("out");
            std::fs::create_dir_all(&out)?;
            let dp = out.join("DESIGN.md");
            let hp = out.join("hero.html");
            std::fs::write(&dp, &output.design_md)?;
            std::fs::write(&hp, &output.hero_html)?;
            (dp, hp, false)
        }
        WriteMode::Commit => {
            let dp = invocation.target_repo.join("DESIGN.md");
            let hp = invocation.target_repo.join("hero.html");
            std::fs::write(&dp, &output.design_md)?;
            std::fs::write(&hp, &output.hero_html)?;
            let committed = try_git_commit(&invocation.target_repo, &[&dp, &hp], &intent).is_ok();
            (dp, hp, committed)
        }
    };

    let signal = if is_redesign { SignalKind::Regenerate } else { SignalKind::Accept };
    let _ = record_taste(
        &invocation,
        signal,
        json!({ "intent": intent, "provider": output.provider_used }),
    );

    Ok(json!({
        "invocation_id": invocation.id,
        "provider_used": output.provider_used,
        "design_path": design_path.to_string_lossy(),
        "hero_path": hero_path.to_string_lossy(),
        "variants": output.variants.iter().map(|v| v.kind.label()).collect::<Vec<_>>(),
        "committed": committed,
        "validation": {
            "ok": report.is_ok(),
            "violations": report.violations,
        },
    }))
}

fn do_validate(args: Value) -> anyhow::Result<Value> {
    let src = if let Some(s) = args.get("design_md").and_then(|v| v.as_str()) {
        s.to_string()
    } else if let Some(p) = args.get("design_md_path").and_then(|v| v.as_str()) {
        std::fs::read_to_string(p)?
    } else {
        anyhow::bail!("provide either `design_md` or `design_md_path`");
    };
    let doc = parse_design(&src)?;
    let variants = resolve_variants(&doc);
    let report = validate_design(&doc, &variants);
    Ok(json!({
        "ok": report.is_ok(),
        "violations": report.violations,
        "variants": variants.iter().map(|v| v.kind.label()).collect::<Vec<_>>(),
    }))
}

fn do_auth_status() -> Value {
    let providers = ["anthropic", "openai", "moonshot", "gemini"];
    let configured: Vec<_> = providers
        .iter()
        .map(|p| {
            let present = aphrodite_keyring::fetch(p).is_ok();
            json!({ "provider": p, "configured": present })
        })
        .collect();
    json!({ "providers": configured })
}

fn try_git_commit(repo: &std::path::Path, files: &[&std::path::Path], intent: &str) -> anyhow::Result<()> {
    if !repo.join(".git").exists() {
        anyhow::bail!("not a git repo");
    }
    use std::process::{Command, Stdio};
    for f in files {
        let _ = Command::new("git")
            .arg("-C")
            .arg(repo)
            .arg("add")
            .arg(f)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    let truncated: String = intent.chars().take(60).collect();
    let msg = format!("Aphrodite: design for \"{truncated}\"");
    let status = Command::new("git")
        .arg("-C")
        .arg(repo)
        .arg("commit")
        .arg("-m")
        .arg(&msg)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if !status.success() {
        anyhow::bail!("git commit failed");
    }
    Ok(())
}
