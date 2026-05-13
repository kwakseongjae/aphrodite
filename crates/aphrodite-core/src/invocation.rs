//! The Invocation: a single call into Aphrodite. Mirrors the seed ontology
//! field `invocation`. Every entry point (CLI, MCP) builds one of these and
//! hands it to the engine.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Who initiated this invocation. Agent-first means the JSON shape wins on
/// any UX conflict.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Caller {
    Agent,
    Human,
}

/// The transport surface this invocation came in over.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Surface {
    Cli,
    Mcp,
}

/// What Aphrodite does with the emitted artifacts.
///
/// * `Commit` — the seed-mandated default for v0.1: stage + commit into the caller's repo.
/// * `ArtifactOnly` — opt-in via `--no-write` (CLI) or `"write_mode": "artifact_only"` (MCP).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum WriteMode {
    #[default]
    Commit,
    ArtifactOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invocation {
    pub id: String,
    pub caller: Caller,
    pub surface: Surface,
    pub intent: String,
    pub target_repo: PathBuf,
    pub write_mode: WriteMode,
}
