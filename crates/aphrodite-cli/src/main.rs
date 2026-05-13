//! `aphrodite` CLI. Thin pretty layer over a JSON contract — `--json` short-
//! circuits the pretty renderer for agent callers.

use clap::{Parser, Subcommand};
use serde_json::json;
use std::path::PathBuf;

mod banner;
mod design_cmd;
mod init_cmd;
mod setup_cmd;

#[derive(Parser)]
#[command(name = "aphrodite", version, about = "Aphrodite — UI generation harness")]
struct Cli {
    /// Emit raw JSON instead of the pretty terminal output.
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// First-run guided wizard — banner, provider, plan, model, key.
    Init,

    /// Print the ASCII banner + current config summary.
    Version,

    /// Non-interactive setup (env-var driven; kept for CI).
    Setup,

    /// Generate a DESIGN.md + hero HTML from an intent string.
    Design {
        intent: String,

        /// Skip the git commit; emit artifacts to `./.aphrodite/out/` instead.
        #[arg(long)]
        no_write: bool,

        /// Target repo (defaults to current working directory).
        #[arg(long)]
        repo: Option<PathBuf>,

        /// Refuse to fall back to offline if a real provider was intended.
        /// Useful for CI and agent calls where silent downgrade is unacceptable.
        #[arg(long)]
        require_llm: bool,
    },

    /// Regenerate with implicit "didn't like that one" signal recorded.
    Redesign {
        intent: String,

        #[arg(long)]
        no_write: bool,

        #[arg(long)]
        repo: Option<PathBuf>,

        #[arg(long)]
        require_llm: bool,
    },

    /// Show configured providers (without revealing key material).
    Auth {
        #[command(subcommand)]
        sub: AuthSub,
    },

    /// Run all health checks (config + keychain + env + reachability).
    Doctor,

    /// Print the v0.1 capability matrix — what Aphrodite can and cannot do.
    Capabilities,
}

#[derive(Subcommand)]
enum AuthSub {
    /// List configured providers.
    Status,
    /// Store an API key for a provider in the OS keychain.
    Set {
        provider: String,
        /// Read key from this env var instead of prompting.
        #[arg(long)]
        from_env: Option<String>,
    },
    /// Remove a provider's stored credential.
    Remove { provider: String },
    /// Verify that a provider's stored key is readable (round-trip test).
    Verify { provider: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .try_init();

    let payload = match cli.command {
        Command::Init => init_cmd::run().await?,
        Command::Version => version_summary(cli.json)?,
        Command::Setup => setup_cmd::run().await?,
        Command::Design { intent, no_write, repo, require_llm } => {
            design_cmd::run(intent, no_write, repo, false, require_llm).await?
        }
        Command::Redesign { intent, no_write, repo, require_llm } => {
            design_cmd::run(intent, no_write, repo, true, require_llm).await?
        }
        Command::Auth { sub } => match sub {
            AuthSub::Status => auth_status(),
            AuthSub::Set { provider, from_env } => auth_set(&provider, from_env.as_deref())?,
            AuthSub::Remove { provider } => auth_remove(&provider),
            AuthSub::Verify { provider } => auth_verify(&provider),
        },
        Command::Doctor => doctor(),
        Command::Capabilities => capabilities(),
    };

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else {
        render_pretty(&payload);
    }
    Ok(())
}

const ALL_PROVIDERS: &[&str] = &["zai", "anthropic", "openrouter", "openai", "moonshot", "gemini"];

fn version_summary(_json: bool) -> anyhow::Result<serde_json::Value> {
    banner::print(env!("CARGO_PKG_VERSION"));
    let cfg = aphrodite_core::config::load();
    let active = cfg.default_provider.clone();
    let configured: Vec<_> = ALL_PROVIDERS
        .iter()
        .filter(|p| aphrodite_keyring::fetch(p).is_ok())
        .map(|s| s.to_string())
        .collect();
    if let Some(p) = active.as_deref() {
        let pc = cfg.providers.get(p);
        eprintln!(
            "  active provider : {} {} model={} plan={}",
            console::style(p).bold().yellow(),
            console::style("·").dim(),
            console::style(pc.and_then(|c| c.model.as_deref()).unwrap_or("(default)")).cyan(),
            console::style(pc.and_then(|c| c.plan.as_deref()).unwrap_or("(default)")).cyan()
        );
    } else {
        eprintln!(
            "  active provider : {}  (run `aphrodite init` to pick one)",
            console::style("offline").dim()
        );
    }
    eprintln!(
        "  configured keys : {}",
        if configured.is_empty() { "(none)".into() } else { configured.join(", ") }
    );
    Ok(json!({
        "kind": "version",
        "version": env!("CARGO_PKG_VERSION"),
        "active_provider": active,
        "configured": configured,
    }))
}

fn auth_status() -> serde_json::Value {
    let configured: Vec<_> = ALL_PROVIDERS
        .iter()
        .map(|p| {
            let present = aphrodite_keyring::fetch(p).is_ok();
            json!({ "provider": p, "configured": present })
        })
        .collect();
    json!({ "kind": "auth_status", "providers": configured })
}

fn auth_set(provider: &str, from_env: Option<&str>) -> anyhow::Result<serde_json::Value> {
    if !ALL_PROVIDERS.contains(&provider) {
        anyhow::bail!("unknown provider `{provider}`; supported: {}", ALL_PROVIDERS.join(", "));
    }
    let key = match from_env {
        Some(name) => std::env::var(name).map_err(|_| anyhow::anyhow!("env var {name} unset"))?,
        None => rpassword::prompt_password(format!("API key for {provider} (paste, hidden): "))?
            .trim()
            .to_string(),
    };
    eprintln!("Captured {} characters.", key.chars().count());
    if key.is_empty() {
        anyhow::bail!("empty key — nothing stored. If hidden input isn't working in your terminal, run with `--from-env NAME` instead.");
    }
    aphrodite_keyring::store(provider, &key)?;
    let verified = aphrodite_keyring::fetch(provider).ok().as_deref() == Some(key.as_str());
    if !verified {
        anyhow::bail!(
            "Stored, but readback failed — macOS Keychain probably needs `Always Allow` permission for this binary. Open Keychain Access app, search service `aphrodite`, account `provider:{provider}`, and tick Always Allow."
        );
    }
    Ok(json!({ "kind": "auth_set", "provider": provider, "stored": true, "verified": true, "key_chars": key.chars().count() }))
}

fn auth_verify(provider: &str) -> serde_json::Value {
    use console::style;
    if !ALL_PROVIDERS.contains(&provider) {
        return json!({ "kind": "auth_verify", "provider": provider, "ok": false, "reason": "unknown provider" });
    }
    match aphrodite_keyring::fetch(provider) {
        Ok(k) => {
            eprintln!(
                "  {} {} present in OS keychain ({} chars)",
                style("✓").green(),
                provider,
                k.chars().count()
            );
            json!({ "kind": "auth_verify", "provider": provider, "ok": true, "key_chars": k.chars().count() })
        }
        Err(e) => {
            eprintln!(
                "  {} {} NOT readable from keychain: {}",
                style("✖").red(),
                provider,
                e
            );
            eprintln!(
                "  {} Try `aphrodite auth set {}` again, and on macOS click `Always Allow` if prompted.",
                style("→").dim(),
                provider
            );
            json!({ "kind": "auth_verify", "provider": provider, "ok": false, "error": e.to_string() })
        }
    }
}

fn auth_remove(provider: &str) -> serde_json::Value {
    let removed = aphrodite_keyring::delete(provider).is_ok();
    json!({ "kind": "auth_remove", "provider": provider, "removed": removed })
}

fn doctor() -> serde_json::Value {
    use console::style;
    let cfg = aphrodite_core::config::load();
    let mut issues: Vec<serde_json::Value> = Vec::new();
    let mut checks: Vec<serde_json::Value> = Vec::new();

    // Check 1: config existence + parseability.
    let cfg_path = aphrodite_core::config::config_path();
    let cfg_present = cfg_path.exists();
    eprintln!(
        "  {} config file ({})",
        if cfg_present { style("✓").green() } else { style("○").dim() },
        style(cfg_path.display()).dim()
    );
    checks.push(json!({ "name": "config_file_present", "ok": cfg_present }));

    // Check 2: default provider declared.
    let default = cfg.default_provider.clone();
    let has_default = default.is_some();
    eprintln!(
        "  {} default provider: {}",
        if has_default { style("✓").green() } else { style("○").dim() },
        default.as_deref().unwrap_or("(none — will use offline)")
    );

    // Check 3: keychain readback for the default provider.
    let mut keychain_ok = false;
    if let Some(p) = default.as_deref() {
        match aphrodite_keyring::fetch(p) {
            Ok(k) => {
                keychain_ok = !k.trim().is_empty();
                if keychain_ok {
                    eprintln!(
                        "  {} keychain entry for `{}` readable ({} chars)",
                        style("✓").green(),
                        p,
                        k.chars().count()
                    );
                } else {
                    eprintln!(
                        "  {} keychain entry for `{}` exists but is empty",
                        style("✖").red(),
                        p
                    );
                    issues.push(json!({
                        "code": "keychain_empty",
                        "fix": format!("aphrodite auth set {p}")
                    }));
                }
            }
            Err(e) => {
                eprintln!(
                    "  {} keychain entry for `{}` not readable: {}",
                    style("✖").red(),
                    p,
                    e
                );
                issues.push(json!({
                    "code": "keychain_unreadable",
                    "fix": format!("aphrodite auth set {p} — or on macOS check Keychain Access for 'Always Allow' on the aphrodite binary"),
                    "raw": e.to_string(),
                }));
            }
        }
    }
    checks.push(json!({ "name": "default_keychain_readable", "ok": keychain_ok }));

    // Check 4: env-var fallback for the default provider.
    let mut env_ok = false;
    if let Some(p) = default.as_deref() {
        let env_keys: &[&str] = match p {
            "zai" => &["APHRODITE_ZAI_API_KEY", "ZAI_API_KEY", "GLM_API_KEY"],
            "anthropic" => &["APHRODITE_ANTHROPIC_API_KEY", "ANTHROPIC_API_KEY"],
            "openrouter" => &["APHRODITE_OPENROUTER_API_KEY", "OPENROUTER_API_KEY"],
            _ => &[],
        };
        for name in env_keys {
            if std::env::var(name).map(|v| !v.trim().is_empty()).unwrap_or(false) {
                eprintln!("  {} env fallback present: {}", style("✓").green(), name);
                env_ok = true;
                break;
            }
        }
        if !env_ok {
            eprintln!(
                "  {} no env fallback for `{}` (tried: {})",
                style("○").dim(),
                p,
                env_keys.join(", ")
            );
        }
    }
    checks.push(json!({ "name": "env_fallback_present", "ok": env_ok }));

    // Verdict.
    let credential_ok = keychain_ok || env_ok;
    let verdict = if credential_ok {
        eprintln!("  {} ready for real provider calls", style("●").green());
        "healthy"
    } else if has_default {
        eprintln!(
            "  {} config intends `{}` but no credential is reachable — calls will fall back to offline",
            style("⚠").yellow(),
            default.as_deref().unwrap_or("?")
        );
        "degraded_offline_only"
    } else {
        eprintln!(
            "  {} no provider configured — running offline by design",
            style("○").dim()
        );
        "offline_by_design"
    };

    json!({
        "kind": "doctor",
        "verdict": verdict,
        "checks": checks,
        "issues": issues,
        "default_provider": default,
    })
}

fn capabilities() -> serde_json::Value {
    use console::style;
    let cap = json!({
        "kind": "capabilities",
        "version": env!("CARGO_PKG_VERSION"),
        "in_scope": {
            "design.modes": ["light", "dark", "brand"],
            "design.outputs": ["DESIGN.md (Google Labs alpha)", "hero.html (self-contained, no external network)"],
            "providers.api_key": ["zai", "anthropic", "openrouter"],
            "providers.offline_fallback": true,
            "validation": ["schema (Google Labs alpha)", "WCAG-AA contrast across all variants"],
            "taste_loop": "implicit (Regenerate signals bias next call)",
            "write_modes": ["commit (default)", "artifact_only"],
        },
        "out_of_scope_v01": [
            "image generation / asset fetching",
            "motion / video (HyperFrames adapter lands in v0.2)",
            "3D scenes / three.js / Blender (v0.3)",
            "Figma / Sketch round-trip (v0.2)",
            "explicit aesthetic jury (implicit signals only in v0.1)",
            "OAuth flows for any provider (v0.2; API-key only at v0.1)",
        ],
    });

    println!("{}", style("Aphrodite — v0.1 capabilities").bold().magenta());
    println!();
    println!("  {}", style("In scope:").bold());
    println!("    • design / redesign / validate / auth_status MCP tools");
    println!("    • 4 variants per DESIGN.md (light, dark, brand-a, brand-b)");
    println!("    • WCAG-AA contrast gate, schema gate");
    println!("    • Providers: z.ai GLM (API key), Anthropic (API key), OpenRouter (API key)");
    println!("    • Offline deterministic fallback (no network, no cost)");
    println!("    • Implicit taste loop — `redesign` shifts subsequent palettes");
    println!("    • Direct-commit by default; `--no-write` for artifact-only mode");
    println!();
    println!("  {}", style("Out of v0.1 scope (will surface as warnings if asked):").dim());
    println!("    • image generation, asset fetching");
    println!("    • motion / video (HyperFrames adapter, v0.2)");
    println!("    • 3D / three.js / Blender (v0.3)");
    println!("    • Figma round-trip (v0.2)");
    println!("    • OAuth (v0.2)");
    println!();
    cap
}

fn render_pretty(payload: &serde_json::Value) {
    use console::style;
    let kind = payload.get("kind").and_then(|v| v.as_str()).unwrap_or("");
    match kind {
        "auth_status" => {
            println!("{}", style("Aphrodite — auth status").bold().cyan());
            if let Some(arr) = payload.get("providers").and_then(|v| v.as_array()) {
                for p in arr {
                    let name = p.get("provider").and_then(|v| v.as_str()).unwrap_or("?");
                    let ok = p.get("configured").and_then(|v| v.as_bool()).unwrap_or(false);
                    let badge = if ok {
                        style("● configured").green()
                    } else {
                        style("○ not set    ").dim()
                    };
                    println!("  {badge}  {name}");
                }
            }
        }
        "setup" => {
            println!("{}", style("Aphrodite — setup").bold().cyan());
            if let Some(msg) = payload.get("message").and_then(|v| v.as_str()) {
                println!("  {msg}");
            }
        }
        "design" => {
            println!("{}", style("Aphrodite — design").bold().magenta());
            if let Some(out) = payload.get("output").and_then(|v| v.as_object()) {
                let provider = out.get("provider_used").and_then(|v| v.as_str()).unwrap_or("?");
                println!("  Provider     : {}", style(provider).yellow());
                if let Some(files) = out.get("files").and_then(|v| v.as_array()) {
                    println!("  Written      :");
                    for f in files {
                        if let Some(p) = f.as_str() {
                            println!("    • {p}");
                        }
                    }
                }
                if let Some(committed) = out.get("committed").and_then(|v| v.as_bool()) {
                    let badge = if committed {
                        style("yes").green()
                    } else {
                        style("no (artifact-only)").dim()
                    };
                    println!("  Committed    : {badge}");
                }
                if let Some(validation) = out.get("validation").and_then(|v| v.as_object()) {
                    let ok = validation.get("ok").and_then(|v| v.as_bool()).unwrap_or(false);
                    let n = validation
                        .get("violations")
                        .and_then(|v| v.as_array())
                        .map(|a| a.len())
                        .unwrap_or(0);
                    if ok {
                        println!("  Validation   : {}", style("PASS").green());
                    } else {
                        println!(
                            "  Validation   : {} ({n} violation{})",
                            style("FAIL").red(),
                            if n == 1 { "" } else { "s" }
                        );
                        if let Some(arr) = validation.get("violations").and_then(|v| v.as_array()) {
                            for v in arr.iter().take(5) {
                                println!("    - {}", v.get("message").and_then(|x| x.as_str()).unwrap_or("?"));
                            }
                        }
                    }
                }
                if let Some(warnings) = out.get("warnings").and_then(|v| v.as_array()) {
                    if !warnings.is_empty() {
                        println!("  Warnings     :");
                        for w in warnings {
                            let kind = w.get("kind").and_then(|v| v.as_str()).unwrap_or("?");
                            let msg = w.get("message").and_then(|v| v.as_str()).unwrap_or("");
                            println!("    {} {} — {}", style("⚠").yellow(), style(kind).yellow(), msg);
                            if let Some(hint) = w.get("hint").and_then(|v| v.as_str()) {
                                println!("      {} {}", style("→").dim(), style(hint).dim());
                            }
                        }
                    }
                }
                if let Some(hero) = out.get("hero_path").and_then(|v| v.as_str()) {
                    println!("\n  {} {}", style("Open in browser:").dim(), style(format!("file://{hero}")).underlined());
                }
            }
        }
        _ => {
            println!("{}", serde_json::to_string_pretty(payload).unwrap_or_default());
        }
    }
}
