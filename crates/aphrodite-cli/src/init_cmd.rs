//! `aphrodite init` — guided first-run wizard.
//!
//! Flow:
//!   1. Banner.
//!   2. Pick provider (arrow keys; z.ai pre-selected per project intent).
//!   3. Pick plan (Coding Plan vs Standard API — for z.ai). Other providers
//!      get a one-option plan list.
//!   4. Pick model from the curated list for that provider.
//!   5. Paste API key (hidden input) OR pick "use env var".
//!   6. Persist preferences to `~/.aphrodite/config.toml` and the key to OS keychain.
//!   7. Optional immediate smoke-call to confirm key + endpoint.

use crate::banner;
use aphrodite_core::config::{self, ProviderConfig};
use aphrodite_generator::provider::ProviderId;
use console::style;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use serde_json::json;

const PROVIDERS_TOP: &[(&str, ProviderId)] = &[
    ("z.ai GLM (subscription / coding plan recommended)", ProviderId::Zai),
    ("Anthropic — direct API key", ProviderId::Anthropic),
    ("OpenRouter — proxy to any model", ProviderId::Openrouter),
    ("Offline only — no API, deterministic generator", ProviderId::Zai), // sentinel; handled below
];

pub async fn run() -> anyhow::Result<serde_json::Value> {
    banner::print(env!("CARGO_PKG_VERSION"));

    let theme = ColorfulTheme::default();

    // Step 1 — provider
    eprintln!("{}", style("◆ Step 1/4 — Pick a provider").bold().cyan());
    let labels: Vec<&str> = PROVIDERS_TOP.iter().map(|(l, _)| *l).collect();
    let provider_idx = FuzzySelect::with_theme(&theme)
        .with_prompt("Provider")
        .default(0)
        .items(&labels)
        .interact()?;

    let is_offline = provider_idx == 3;
    if is_offline {
        return finish_offline().await;
    }
    let provider = PROVIDERS_TOP[provider_idx].1;

    // Step 2 — plan (z.ai has two; others have one)
    eprintln!();
    eprintln!("{}", style("◆ Step 2/4 — Pick a plan").bold().cyan());
    let plans = provider.plans();
    let plan_idx = FuzzySelect::with_theme(&theme)
        .with_prompt(format!("{} plan", provider.human_name()))
        .default(0)
        .items(&plans.iter().map(|(l, _)| *l).collect::<Vec<_>>())
        .interact()?;
    let plan_id = plans[plan_idx].1;

    // Step 3 — model
    eprintln!();
    eprintln!("{}", style("◆ Step 3/4 — Pick a model").bold().cyan());
    let models = provider.curated_models();
    let model_idx = FuzzySelect::with_theme(&theme)
        .with_prompt(format!("{} model", provider.human_name()))
        .default(0)
        .items(&models.iter().map(|(l, _)| *l).collect::<Vec<_>>())
        .interact()?;
    let model_id = models[model_idx].1;

    // Step 4 — API key (with multiple input paths because hidden prompts
    // are unreliable in some terminals).
    eprintln!();
    eprintln!("{}", style("◆ Step 4/4 — API key").bold().cyan());

    let methods = [
        "Paste from system clipboard (pbpaste / wl-paste / xclip) — most reliable",
        "Type or paste into a hidden prompt",
        "Read from a file path (file deleted after read)",
        "Skip — I'll set the env var myself later",
    ];
    let method_idx = FuzzySelect::with_theme(&theme)
        .with_prompt("How would you like to provide the key?")
        .default(0)
        .items(&methods)
        .interact()?;

    let raw_key: String = match method_idx {
        0 => read_from_clipboard()?,
        1 => rpassword::prompt_password(format!("  {} API key (hidden): ", provider.human_name()))?,
        2 => {
            use dialoguer::Input;
            let path: String = Input::with_theme(&theme)
                .with_prompt("File path containing the key")
                .interact_text()?;
            let p = std::path::PathBuf::from(path.trim());
            let body = std::fs::read_to_string(&p)?;
            // Best-effort delete to mirror `auth set --from-file` behaviour.
            if let Err(e) = std::fs::remove_file(&p) {
                eprintln!("  {} could not delete source file: {}", style("⚠").yellow(), e);
            } else {
                eprintln!("  {} source file deleted: {}", style("✓").green(), p.display());
            }
            body
        }
        _ => String::new(),
    };
    let key = clean_secret(&raw_key);

    // Diagnostic: show length so the user can verify the input was actually
    // received. Common failure modes on macOS are (a) paste mangled by
    // bracketed-paste, (b) input read by another process, (c) the user
    // pressed Enter on an empty line. Showing length lets us distinguish all
    // three without ever printing the secret.
    if key.is_empty() {
        eprintln!(
            "  {} Captured 0 characters. {}",
            style("⚠").yellow(),
            style("Nothing was read from stdin.").dim()
        );
        eprintln!(
            "  {} Aphrodite will look for APHRODITE_{}_API_KEY at call time, or rerun `aphrodite auth set {}`.",
            style("→").dim(),
            provider.label().to_uppercase(),
            provider.label()
        );
    } else {
        eprintln!(
            "  {} Captured {} characters. Writing to OS keychain (round-trip verified)…",
            style("✓").green(),
            key.chars().count()
        );
        match aphrodite_keyring::store(provider.label(), &key) {
            Ok(()) => {
                eprintln!(
                    "  {} Verified: keychain readback matches. (service `aphrodite`, account `provider:{}`)",
                    style("✓").green(),
                    provider.label()
                );
            }
            Err(e) => {
                eprintln!("  {} Keychain failed: {}", style("✖").red(), e);
                eprintln!(
                    "  {} On macOS this usually means you denied or dismissed the Keychain Access dialog.",
                    style("→").dim()
                );
                eprintln!(
                    "  {} Open Keychain Access → search `aphrodite` → delete stale entries → rerun init and click `Always Allow`.",
                    style("→").dim()
                );
                eprintln!(
                    "  {} Or skip the keychain entirely: `export APHRODITE_{}_API_KEY=…` in your shell rc.",
                    style("→").dim(),
                    provider.label().to_uppercase()
                );
            }
        }
    }

    // Persist config
    let mut cfg = config::load();
    cfg.default_provider = Some(provider.label().to_string());
    cfg.providers.insert(
        provider.label().to_string(),
        ProviderConfig {
            plan: Some(plan_id.to_string()),
            model: Some(model_id.to_string()),
            base_url: Some(provider.base_url_for_plan(plan_id).to_string()),
        },
    );
    config::save(&cfg)?;
    eprintln!(
        "  {} Preferences saved to {}",
        style("✓").green(),
        style(config::config_path().display()).underlined()
    );

    eprintln!();
    eprintln!("{}", style("Try it now:").bold());
    eprintln!("  {}", style("aphrodite design \"your first hero\"").yellow());
    eprintln!();

    Ok(json!({
        "kind": "init",
        "provider": provider.label(),
        "plan": plan_id,
        "model": model_id,
        "key_stored": !key.trim().is_empty(),
        "config_path": config::config_path().display().to_string(),
    }))
}

async fn finish_offline() -> anyhow::Result<serde_json::Value> {
    let mut cfg = config::load();
    cfg.default_provider = None;
    config::save(&cfg)?;
    eprintln!(
        "  {} Aphrodite will use the deterministic offline generator. No network, no key.",
        style("✓").green()
    );
    eprintln!();
    eprintln!("  Try it now:  {}", style("aphrodite design \"your first hero\"").yellow());
    Ok(json!({
        "kind": "init",
        "provider": "offline",
        "key_stored": false,
    }))
}

/// Read from the system clipboard using whichever native tool is on PATH.
/// macOS: `pbpaste`. Wayland: `wl-paste`. X11: `xclip -selection clipboard -o`.
/// Windows: PowerShell `Get-Clipboard`.
fn read_from_clipboard() -> anyhow::Result<String> {
    use std::process::Command;
    let candidates: &[(&str, &[&str])] = &[
        ("pbpaste", &[]),
        ("wl-paste", &["--no-newline"]),
        ("xclip", &["-selection", "clipboard", "-o"]),
        ("xsel", &["--clipboard", "--output"]),
        ("powershell.exe", &["-Command", "Get-Clipboard"]),
    ];
    for (bin, args) in candidates {
        if let Ok(out) = Command::new(bin).args(*args).output() {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout).to_string();
                if !s.trim().is_empty() {
                    eprintln!(
                        "  {} Read {} bytes from clipboard via `{}`",
                        console::style("✓").green(),
                        out.stdout.len(),
                        bin
                    );
                    return Ok(s);
                }
            }
        }
    }
    anyhow::bail!(
        "No clipboard tool found on PATH (tried pbpaste / wl-paste / xclip / xsel). \
         Pick a different input method or pipe the key in: \
         `pbpaste | aphrodite auth set zai --from-stdin`"
    )
}

/// Strip bracketed-paste wrappers + control characters + whitespace.
fn clean_secret(raw: &str) -> String {
    let mut s = raw.to_string();
    s = s.replace("\u{1b}[200~", "").replace("\u{1b}[201~", "");
    s = s.replace('\u{1b}', "");
    s.retain(|c| c != '\r' && c != '\n' && c != '\0');
    s.trim().to_string()
}
