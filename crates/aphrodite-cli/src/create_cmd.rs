//! `aphrodite create "<intent>"` — autonomous creation harness entry point
//! (ADR 0004). Thin wrapper over `aphrodite_generator::orchestrator::run`.
//!
//! The orchestration logic lives in the generator crate so both CLI and
//! MCP surfaces share the same execution path.

use std::path::PathBuf;

pub async fn run(
    intent: String,
    max_turns: u32,
    satisfaction_threshold: f32,
    persona_slug: Option<String>,
    no_write: bool,
    repo: Option<PathBuf>,
) -> anyhow::Result<serde_json::Value> {
    aphrodite_generator::orchestrator::run(
        intent,
        max_turns,
        satisfaction_threshold,
        persona_slug,
        no_write,
        repo,
    )
    .await
}
