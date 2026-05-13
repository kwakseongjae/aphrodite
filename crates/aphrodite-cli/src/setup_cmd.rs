//! `aphrodite setup` — interactive auth bootstrap.
//!
//! v0.1: API-key only (Anthropic mandatory; others optional). Storage via
//! aphrodite-keyring (OS keychain). Never to disk.

use serde_json::json;
use std::io::{BufRead, Write};

pub async fn run() -> anyhow::Result<serde_json::Value> {
    // Check what's already in the keychain.
    let providers = ["anthropic", "openai", "moonshot", "gemini"];
    let already: Vec<&str> = providers
        .iter()
        .copied()
        .filter(|p| aphrodite_keyring::fetch(p).is_ok())
        .collect();

    eprintln!("Aphrodite setup — provider credentials");
    if !already.is_empty() {
        eprintln!("  Already configured: {}", already.join(", "));
    }

    // Env-var preferred path (non-interactive).
    let mut stored: Vec<String> = Vec::new();
    if let Ok(k) = std::env::var("APHRODITE_ANTHROPIC_API_KEY") {
        if !k.trim().is_empty() {
            aphrodite_keyring::store("anthropic", k.trim())?;
            stored.push("anthropic".into());
        }
    }

    // Interactive prompt only if stdin is a tty and Anthropic still missing.
    if !stored.contains(&"anthropic".to_string())
        && aphrodite_keyring::fetch("anthropic").is_err()
        && atty_stdin()
    {
        eprint!("Anthropic API key (paste, or press Enter to skip): ");
        std::io::stderr().flush().ok();
        let stdin = std::io::stdin();
        let mut line = String::new();
        stdin.lock().read_line(&mut line)?;
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            aphrodite_keyring::store("anthropic", trimmed)?;
            stored.push("anthropic".into());
        }
    }

    let configured: Vec<_> = providers
        .iter()
        .map(|p| json!({ "provider": p, "configured": aphrodite_keyring::fetch(p).is_ok() }))
        .collect();

    Ok(json!({
        "kind": "setup",
        "message": if stored.is_empty() {
            "No new keys stored. Run again with APHRODITE_ANTHROPIC_API_KEY env or in a tty."
        } else {
            "Keys stored in OS keychain."
        },
        "stored": stored,
        "providers": configured,
    }))
}

fn atty_stdin() -> bool {
    // Very small portable check; avoids the `atty` crate.
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
