//! `aphrodite log` / `aphrodite undo` — discoverable history + reversible
//! auto-commits.
//!
//! Aphrodite auto-commits every `create` / `refine` invocation. The user
//! review surfaced "no undo" as a usability pain — a refine that ruins
//! things should be one command to back out, not a manual `git reset`.

use serde_json::json;
use std::path::PathBuf;
use std::process::Command;

fn resolve_repo(repo: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    let r = match repo {
        Some(p) => p,
        None => std::env::current_dir()?,
    };
    Ok(r.canonicalize().unwrap_or(r))
}

pub fn show_log(repo: Option<PathBuf>, n: usize) -> anyhow::Result<serde_json::Value> {
    use console::style;
    let target = resolve_repo(repo)?;
    if !target.join(".git").exists() {
        anyhow::bail!("target is not a git repo: {}", target.display());
    }
    let out = Command::new("git")
        .arg("-C")
        .arg(&target)
        .arg("log")
        .arg(format!("-{n}"))
        .arg("--pretty=format:%h|%cr|%s")
        .arg("--no-merges")
        .output()?;
    let body = String::from_utf8_lossy(&out.stdout);
    let mut entries: Vec<serde_json::Value> = Vec::new();
    println!(
        "{} (last {} commits in {})",
        style("Aphrodite log").bold().magenta(),
        n,
        style(target.display()).dim()
    );
    for line in body.lines() {
        let mut parts = line.splitn(3, '|');
        let sha = parts.next().unwrap_or("?");
        let when = parts.next().unwrap_or("?");
        let subject = parts.next().unwrap_or("?");
        let kind_tag = if subject.starts_with("Aphrodite create:") {
            "create"
        } else if subject.starts_with("Aphrodite refine:") {
            "refine"
        } else if subject.starts_with("Aphrodite: design") {
            "design"
        } else {
            "other"
        };
        let badge = match kind_tag {
            "create" => style("create").green(),
            "refine" => style("refine").yellow(),
            "design" => style("design").cyan(),
            _ => style("other ").dim(),
        };
        println!(
            "  {} {} {} {}",
            badge,
            style(sha).cyan(),
            style(format!("{:>16}", when)).dim(),
            subject
        );
        entries.push(json!({
            "sha": sha,
            "when": when,
            "kind": kind_tag,
            "subject": subject,
        }));
    }
    Ok(json!({
        "kind": "log",
        "project": target.to_string_lossy(),
        "commits": entries,
    }))
}

pub fn undo(
    repo: Option<PathBuf>,
    n: usize,
    yes: bool,
) -> anyhow::Result<serde_json::Value> {
    use console::style;
    let target = resolve_repo(repo)?;
    if !target.join(".git").exists() {
        anyhow::bail!("target is not a git repo: {}", target.display());
    }
    // Show the commits about to be reverted so the user sees what they're
    // signing off on.
    let head_out = Command::new("git")
        .arg("-C")
        .arg(&target)
        .arg("log")
        .arg(format!("-{n}"))
        .arg("--pretty=format:%h %s")
        .arg("--no-merges")
        .output()?;
    let preview = String::from_utf8_lossy(&head_out.stdout);
    let lines: Vec<&str> = preview.lines().collect();
    if lines.is_empty() {
        anyhow::bail!("nothing to undo — git log is empty");
    }
    // Only allow undoing Aphrodite-authored commits — a user's manual commit
    // in the middle would be lost otherwise.
    for (i, line) in lines.iter().enumerate() {
        if i >= n {
            break;
        }
        let rest = line.splitn(2, ' ').nth(1).unwrap_or("");
        if !rest.starts_with("Aphrodite ") && !rest.starts_with("Aphrodite:") {
            anyhow::bail!(
                "refusing to undo: commit `{}` is not an Aphrodite auto-commit. Use `git reset` directly if you mean to drop it.",
                line
            );
        }
    }
    println!(
        "{} will revert the following {} commit(s) in {}:",
        style("aphrodite undo").bold().yellow(),
        n,
        style(target.display()).dim()
    );
    for line in &lines {
        println!("  {} {}", style("✗").red(), line);
    }
    if !yes {
        eprintln!(
            "\n{} Pass --yes to actually perform the revert.",
            style("(dry-run)").dim()
        );
        return Ok(json!({
            "kind": "undo",
            "dry_run": true,
            "would_drop": lines,
        }));
    }
    // Perform: git reset --hard HEAD~n
    let reset = Command::new("git")
        .arg("-C")
        .arg(&target)
        .arg("reset")
        .arg("--hard")
        .arg(format!("HEAD~{n}"))
        .output()?;
    if !reset.status.success() {
        anyhow::bail!(
            "git reset failed: {}",
            String::from_utf8_lossy(&reset.stderr)
        );
    }
    println!(
        "  {} {} commit(s) rolled back. `git reflog` shows the prior HEAD if you want to bring them back.",
        style("✓").green(),
        n
    );
    Ok(json!({
        "kind": "undo",
        "dry_run": false,
        "dropped": lines,
        "reflog_note": "the prior HEAD is recoverable via `git reflog`",
    }))
}
