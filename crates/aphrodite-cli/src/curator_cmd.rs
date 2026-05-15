//! `aphrodite curator [--dry-run] [--stale N] [--archive N]` — Hermes-pattern
//! periodic review of agent-created skills.

use aphrodite_core::curator;
use serde_json::json;

pub fn run(
    dry_run: bool,
    stale_after_days: Option<u32>,
    archive_after_days: Option<u32>,
) -> anyhow::Result<serde_json::Value> {
    use console::style;
    let mut cfg = curator::CuratorConfig::default();
    if let Some(s) = stale_after_days {
        cfg.stale_after_days = s;
    }
    if let Some(a) = archive_after_days {
        cfg.archive_after_days = a;
    }
    let report = curator::run(cfg, dry_run)?;
    println!(
        "{} {}{}",
        style("Curator").bold().magenta(),
        style(&report.now).dim(),
        if dry_run {
            style(" (DRY-RUN)").yellow().to_string()
        } else {
            String::new()
        }
    );
    println!(
        "  examined: {}  skipped (non-agent-created): {}",
        report.examined, report.skipped_non_agent_created
    );
    println!(
        "  active kept: {}  already-stale: {}  already-archived: {}",
        report.active_kept, report.already_stale_kept, report.already_archived_kept
    );
    if !report.stale_transitions.is_empty() {
        println!("\n  {}", style("Active → Stale:").yellow());
        for t in &report.stale_transitions {
            println!(
                "    {}  inactive {} days",
                style(&t.slug).cyan(),
                t.days_inactive
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "never used".into())
            );
        }
    }
    if !report.archive_transitions.is_empty() {
        println!("\n  {}", style("→ Archived:").red());
        for t in &report.archive_transitions {
            println!(
                "    {}  (from {:?}, inactive {} days)",
                style(&t.slug).cyan(),
                t.from_state,
                t.days_inactive
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "never used".into())
            );
        }
    }
    if !dry_run {
        println!(
            "\n  {} report written to {}",
            style("·").dim(),
            curator::report_path().display()
        );
    }
    Ok(json!({
        "kind": "curator_run",
        "dry_run": dry_run,
        "now": report.now,
        "config": report.config,
        "examined": report.examined,
        "skipped_non_agent_created": report.skipped_non_agent_created,
        "active_kept": report.active_kept,
        "already_stale_kept": report.already_stale_kept,
        "already_archived_kept": report.already_archived_kept,
        "stale_transitions": report.stale_transitions,
        "archive_transitions": report.archive_transitions,
    }))
}
