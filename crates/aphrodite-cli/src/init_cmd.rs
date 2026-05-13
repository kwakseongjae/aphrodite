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

    // Step 4 — API key
    eprintln!();
    eprintln!("{}", style("◆ Step 4/4 — API key").bold().cyan());
    eprintln!(
        "  {} {}",
        style("Tip:").dim(),
        style(format!(
            "Paste your key (input hidden, paste-safe). Press Enter on an empty line to skip and set APHRODITE_{}_API_KEY later.",
            provider.label().to_uppercase()
        ))
        .dim()
    );
    let prompt = format!("  {} API key ", provider.human_name());
    let key: String = rpassword::prompt_password(prompt)?.trim().to_string();

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
