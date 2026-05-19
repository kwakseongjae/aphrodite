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
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
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

    // Opinionated default — z.ai GLM Coding Plan + GLM-5.1. Most users hit
    // this for the same reasons (cheapest Anthropic-compatible LLM with real
    // design fluency). The wizard only asks for what we genuinely cannot
    // default — the API key — and offers an "advanced" branch for the rare
    // user who wants something else.
    eprintln!("{}", style("◆ Aphrodite first-run setup").bold().cyan());
    eprintln!("  {}", style("Aphrodite generates designs via LLM. You'll need an API key.").dim());
    eprintln!("  {}", style("  • z.ai — recommended, ~$0.10/run. Sign up: https://zai.com (~2 min)").dim());
    eprintln!("  {}", style("  • Anthropic — also supported. Keys: https://console.anthropic.com").dim());
    eprintln!("  {}", style("Default: z.ai GLM Coding Plan · model glm-5.1").dim());
    eprintln!("  {}", style("Faster mode: glm-5-turbo (smaller, ~30% faster, slightly less rich)").dim());

    let modes = [
        "Use defaults (z.ai · glm-5.1) and paste my key",
        "Same, but use glm-5-turbo for faster iteration",
        "Advanced — let me pick provider / plan / model",
        "Offline only — no API, deterministic generator",
    ];
    let mode_idx = FuzzySelect::with_theme(&theme)
        .with_prompt("Mode")
        .default(0)
        .items(&modes)
        .interact()?;

    let (provider, plan_id, model_id): (ProviderId, &str, &str) = match mode_idx {
        0 => (ProviderId::Zai, "coding_plan", "glm-5.1"),
        1 => (ProviderId::Zai, "coding_plan", "glm-5-turbo"),
        2 => return run_advanced(&theme).await,
        _ => return finish_offline().await,
    };

    // Step — API key. Single input field, type or paste at the same spot.
    // dialoguer::Input uses readline-style editing (paste, backspace, arrow
    // keys all work). The captured value passes through `clean_secret()` to
    // strip any bracketed-paste markers the terminal may have wrapped around
    // the paste.
    eprintln!();
    eprintln!("{}", style("◆ API key").bold().cyan());
    eprintln!(
        "  {}",
        style("Type or paste your key, then Enter. Empty to skip.").dim()
    );
    let raw_key: String = Input::with_theme(&theme)
        .with_prompt(format!("{} API key", provider.human_name()))
        .allow_empty(true)
        .report(false)               // ← do NOT echo the captured value back
        .interact_text()?;
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
            composer_model: None,
            critic_model: None,
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

/// Advanced path — restores the 4-step wizard for users who want full control.
async fn run_advanced(theme: &ColorfulTheme) -> anyhow::Result<serde_json::Value> {
    eprintln!();
    eprintln!("{}", style("◆ Step 1/4 — Pick a provider").bold().cyan());
    let labels: Vec<&str> = PROVIDERS_TOP.iter().map(|(l, _)| *l).collect();
    let provider_idx = FuzzySelect::with_theme(theme).with_prompt("Provider").default(0).items(&labels).interact()?;
    if provider_idx == 3 {
        return finish_offline().await;
    }
    let provider = PROVIDERS_TOP[provider_idx].1;

    eprintln!();
    eprintln!("{}", style("◆ Step 2/4 — Pick a plan").bold().cyan());
    let plans = provider.plans();
    let plan_idx = FuzzySelect::with_theme(theme).with_prompt(format!("{} plan", provider.human_name())).default(0).items(&plans.iter().map(|(l, _)| *l).collect::<Vec<_>>()).interact()?;
    let plan_id = plans[plan_idx].1;

    eprintln!();
    eprintln!("{}", style("◆ Step 3/4 — Pick a model").bold().cyan());
    let models = provider.curated_models();
    let model_idx = FuzzySelect::with_theme(theme).with_prompt(format!("{} model", provider.human_name())).default(0).items(&models.iter().map(|(l, _)| *l).collect::<Vec<_>>()).interact()?;
    let model_id = models[model_idx].1;

    finish_with_key_prompt(provider, plan_id, model_id, theme).await
}

/// Shared post-decision flow: prompt for key, store, write config.
async fn finish_with_key_prompt(
    provider: ProviderId,
    plan_id: &str,
    model_id: &str,
    theme: &ColorfulTheme,
) -> anyhow::Result<serde_json::Value> {
    eprintln!();
    eprintln!("{}", style("◆ API key").bold().cyan());
    eprintln!("  {}", style("Type or paste your key, then Enter. Empty to skip.").dim());
    let raw_key: String = Input::with_theme(theme)
        .with_prompt(format!("{} API key", provider.human_name()))
        .allow_empty(true)
        .report(false)
        .interact_text()?;
    let key = clean_secret(&raw_key);
    persist_and_finish(provider, plan_id, model_id, &key)
}

fn persist_and_finish(provider: ProviderId, plan_id: &str, model_id: &str, key: &str) -> anyhow::Result<serde_json::Value> {
    if key.is_empty() {
        eprintln!(
            "  {} No key captured. Set APHRODITE_{}_API_KEY later or rerun init.",
            style("⚠").yellow(),
            provider.label().to_uppercase()
        );
    } else {
        eprintln!("  {} Captured {} characters. Writing to OS keychain…", style("✓").green(), key.chars().count());
        match aphrodite_keyring::store(provider.label(), key) {
            Ok(()) => eprintln!("  {} Verified.", style("✓").green()),
            Err(e) => eprintln!("  {} keychain: {e}", style("✖").red()),
        }
    }
    let mut cfg = config::load();
    cfg.default_provider = Some(provider.label().to_string());
    cfg.providers.insert(provider.label().to_string(), ProviderConfig {
        plan: Some(plan_id.to_string()),
        model: Some(model_id.to_string()),
        base_url: Some(provider.base_url_for_plan(plan_id).to_string()),
        composer_model: None,
        critic_model: None,
    });
    config::save(&cfg)?;
    eprintln!("  {} Preferences saved to {}", style("✓").green(), style(config::config_path().display()).underlined());
    eprintln!();
    eprintln!("{}", style("Try it now:").bold());
    eprintln!("  {}", style("aphrodite design \"<your intent>\"").yellow());
    eprintln!("  {}", style("aphrodite love     ← record a positive signal after you see something you like").dim());
    eprintln!("  {}", style("aphrodite hate     ← negative signal").dim());
    Ok(json!({
        "kind": "init",
        "provider": provider.label(),
        "plan": plan_id,
        "model": model_id,
        "key_stored": !key.is_empty(),
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

/// Visible prompt that reads one line from stdin. Paste works in every
/// terminal because we don't touch termios — the shell handles the paste
/// as if the bytes were typed, and we just read them. Key is visible on
/// screen during entry (user accepted this when picking this option).
fn read_visible_line(prompt: &str) -> std::io::Result<String> {
    use std::io::{BufRead, Write};
    eprint!("{prompt}");
    std::io::stderr().flush().ok();
    let stdin = std::io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    Ok(line)
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
