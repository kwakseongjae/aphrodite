//! `aphrodite` CLI. Thin pretty layer over a JSON contract — `--json` short-
//! circuits the pretty renderer for agent callers.

use clap::{Parser, Subcommand};
use serde_json::json;
use std::path::PathBuf;

mod design_cmd;
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
    /// First-run: configure providers, write API keys to OS keychain.
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
    },

    /// Regenerate with implicit "didn't like that one" signal recorded.
    Redesign {
        intent: String,

        #[arg(long)]
        no_write: bool,

        #[arg(long)]
        repo: Option<PathBuf>,
    },

    /// Show configured providers (without revealing key material).
    Auth {
        #[command(subcommand)]
        sub: AuthSub,
    },
}

#[derive(Subcommand)]
enum AuthSub {
    Status,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .try_init();

    let payload = match cli.command {
        Command::Setup => setup_cmd::run().await?,
        Command::Design { intent, no_write, repo } => {
            design_cmd::run(intent, no_write, repo, false).await?
        }
        Command::Redesign { intent, no_write, repo } => {
            design_cmd::run(intent, no_write, repo, true).await?
        }
        Command::Auth { sub: AuthSub::Status } => auth_status(),
    };

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else {
        render_pretty(&payload);
    }
    Ok(())
}

fn auth_status() -> serde_json::Value {
    let providers = ["anthropic", "openai", "moonshot", "gemini"];
    let configured: Vec<_> = providers
        .iter()
        .map(|p| {
            let present = aphrodite_keyring::fetch(p).is_ok();
            json!({ "provider": p, "configured": present })
        })
        .collect();
    json!({ "kind": "auth_status", "providers": configured })
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
