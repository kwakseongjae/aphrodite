//! `aphrodite setup` — interactive auth bootstrap.
//!
//! v0.1: API-key only. Provider priority: z.ai GLM (default for the user
//! who funded this project) > Anthropic > OpenRouter. Direct OpenAI/
//! Moonshot/Gemini API keys are also accepted but not first-class in v0.1.
//! Storage via aphrodite-keyring (OS keychain). Never to disk.

use serde_json::json;
use std::io::{BufRead, Write};

const PROVIDERS: &[(&str, &str)] = &[
    ("zai", "z.ai GLM Coding Plan (recommended; api.z.ai/api/anthropic)"),
    ("anthropic", "Anthropic direct (api.anthropic.com)"),
    ("openrouter", "OpenRouter proxy (openrouter.ai)"),
    ("openai", "OpenAI direct (v0.2 surface)"),
    ("moonshot", "Moonshot / Kimi (v0.2 surface)"),
    ("gemini", "Google Gemini (v0.2 surface)"),
];

pub async fn run() -> anyhow::Result<serde_json::Value> {
    eprintln!("Aphrodite setup — provider credentials");
    let already: Vec<&str> = PROVIDERS
        .iter()
        .filter(|(p, _)| aphrodite_keyring::fetch(p).is_ok())
        .map(|(p, _)| *p)
        .collect();
    if !already.is_empty() {
        eprintln!("  Already configured: {}", already.join(", "));
    }

    let mut stored: Vec<String> = Vec::new();

    // Non-interactive paths: env vars in priority order.
    for (id, env_names) in [
        ("zai", &["APHRODITE_ZAI_API_KEY", "ZAI_API_KEY", "GLM_API_KEY"][..]),
        ("anthropic", &["APHRODITE_ANTHROPIC_API_KEY", "ANTHROPIC_API_KEY"][..]),
        ("openrouter", &["APHRODITE_OPENROUTER_API_KEY", "OPENROUTER_API_KEY"][..]),
    ] {
        for n in env_names {
            if let Ok(k) = std::env::var(n) {
                if !k.trim().is_empty() && aphrodite_keyring::fetch(id).is_err() {
                    aphrodite_keyring::store(id, k.trim())?;
                    stored.push(id.to_string());
                    eprintln!("  ✓ Stored {id} key from {n}");
                    break;
                }
            }
        }
    }

    // Interactive prompt — z.ai first (user-stated preference).
    if atty_stdin() {
        for (id, label) in PROVIDERS.iter().take(3) {
            if aphrodite_keyring::fetch(id).is_ok() {
                continue;
            }
            eprint!("{label}\n  API key (paste, or Enter to skip): ");
            std::io::stderr().flush().ok();
            let mut line = String::new();
            std::io::stdin().lock().read_line(&mut line)?;
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                aphrodite_keyring::store(id, trimmed)?;
                stored.push((*id).to_string());
            }
        }
    }

    let configured: Vec<_> = PROVIDERS
        .iter()
        .map(|(p, _)| json!({ "provider": p, "configured": aphrodite_keyring::fetch(p).is_ok() }))
        .collect();

    Ok(json!({
        "kind": "setup",
        "message": if stored.is_empty() {
            "No new keys stored. Set APHRODITE_ZAI_API_KEY env var or rerun in a tty."
        } else {
            "Keys stored in OS keychain."
        },
        "stored": stored,
        "providers": configured,
    }))
}

fn atty_stdin() -> bool {
    #[cfg(unix)]
    {
        unsafe extern "C" {
            fn isatty(fd: i32) -> i32;
        }
        unsafe { isatty(0) == 1 }
    }
    #[cfg(not(unix))]
    {
        false
    }
}
