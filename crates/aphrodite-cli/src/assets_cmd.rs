//! `aphrodite assets <subcommand>` — inspect / clean `<project>/.aphrodite/assets/`.

use aphrodite_core::assets;
use serde_json::json;
use std::path::PathBuf;

fn resolve_project(repo: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    let p = match repo {
        Some(p) => p,
        None => std::env::current_dir()?,
    };
    Ok(p.canonicalize().unwrap_or(p))
}

pub fn list(repo: Option<PathBuf>) -> anyhow::Result<serde_json::Value> {
    use console::style;
    let project = resolve_project(repo)?;
    let summaries = assets::list(&project);
    let total = assets::total_bytes(&project);
    if summaries.is_empty() {
        println!(
            "  {}  no assets in {}",
            style("○").dim(),
            project.join(".aphrodite/assets").display()
        );
    } else {
        for s in &summaries {
            println!(
                "  {}  {:6} KB  {}  {}",
                style(format!("{:8}", s.category)).cyan(),
                s.size_bytes / 1024,
                style(format!(".{}", s.extension)).dim(),
                s.path
            );
        }
        println!(
            "  {} total: {} files, {} KB",
            style("·").dim(),
            summaries.len(),
            total / 1024
        );
    }
    Ok(json!({
        "kind": "assets_list",
        "project": project.to_string_lossy(),
        "total_bytes": total,
        "assets": summaries,
    }))
}

pub fn clean(repo: Option<PathBuf>) -> anyhow::Result<serde_json::Value> {
    let project = resolve_project(repo)?;
    let (n, bytes) = assets::clean_generated(&project)?;
    eprintln!(
        "  ✓ removed {n} generated asset(s) ({} KB). uploads/ preserved.",
        bytes / 1024
    );
    Ok(json!({
        "kind": "assets_clean",
        "project": project.to_string_lossy(),
        "removed_files": n,
        "reclaimed_bytes": bytes,
    }))
}
