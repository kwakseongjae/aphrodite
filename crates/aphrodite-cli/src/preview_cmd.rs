//! `aphrodite preview` — open the current run's composition.html in the
//! default browser. Closes the "no visual editor" pain by giving users a
//! single-keystroke way to see their output.

use serde_json::json;
use std::path::PathBuf;
use std::process::Command;

pub fn run(
    repo: Option<PathBuf>,
    file: Option<String>,
) -> anyhow::Result<serde_json::Value> {
    use console::style;
    let target = match repo {
        Some(p) => p,
        None => std::env::current_dir()?,
    };
    let target = target.canonicalize().unwrap_or(target);

    // Pick which file to preview. Default = composition.html if present,
    // hero.html otherwise. User can override with --file <name>.
    let path = if let Some(f) = file {
        target.join(f)
    } else if target.join("composition.html").exists() {
        target.join("composition.html")
    } else if target.join("hero.html").exists() {
        target.join("hero.html")
    } else if target.join(".aphrodite/out/composition.html").exists() {
        target.join(".aphrodite/out/composition.html")
    } else {
        anyhow::bail!(
            "no composition.html or hero.html found in {}. Run `aphrodite create '<intent>'` first.",
            target.display()
        );
    };

    let opener_cmd = if cfg!(target_os = "macos") {
        "open"
    } else if cfg!(target_os = "linux") {
        "xdg-open"
    } else if cfg!(target_os = "windows") {
        "start"
    } else {
        "open"
    };

    let status = Command::new(opener_cmd).arg(&path).status();
    match status {
        Ok(s) if s.success() => {
            println!(
                "  {} {}",
                style("✓ opened in default browser").green(),
                style(path.display()).underlined()
            );
        }
        Ok(s) => {
            anyhow::bail!("{} exited with non-zero status {}", opener_cmd, s);
        }
        Err(e) => {
            anyhow::bail!(
                "could not spawn {opener_cmd}: {e}. Manually open: file://{}",
                path.display()
            );
        }
    }

    Ok(json!({
        "kind": "preview",
        "path": path.to_string_lossy(),
        "opener": opener_cmd,
    }))
}
