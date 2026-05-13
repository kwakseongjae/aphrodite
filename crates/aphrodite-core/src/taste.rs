//! Taste store. Append-only JSONL, resolved as `project ⊕ user-global` at read.

use crate::Invocation;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasteEvent {
    pub ts: String, // RFC3339
    pub invocation_id: String,
    pub signal_kind: SignalKind,
    pub source: Source,
    #[serde(default)]
    pub delta: serde_json::Value,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SignalKind {
    Regenerate,
    EditDiff,
    Accept,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    Global,
    Project,
}

pub fn now_rfc3339() -> String {
    let t = time::OffsetDateTime::now_utc();
    t.format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| t.unix_timestamp().to_string())
}

pub fn user_global_path() -> PathBuf {
    let home = std::env::var_os("HOME").unwrap_or_default();
    let user = std::env::var_os("USER")
        .or_else(|| std::env::var_os("LOGNAME"))
        .unwrap_or_else(|| std::ffi::OsString::from("user"));
    let mut p = PathBuf::from(home);
    p.push(".aphrodite");
    p.push("taste");
    let _ = std::fs::create_dir_all(&p);
    p.push(format!("{}.jsonl", user.to_string_lossy()));
    p
}

pub fn project_path(project_root: &Path) -> PathBuf {
    let mut p = project_root.to_path_buf();
    p.push(".aphrodite");
    let _ = std::fs::create_dir_all(&p);
    p.push("taste.jsonl");
    p
}

/// Append an event to both project and global stores. Project gets a `source = Project`
/// copy; the global mirror tags `source = Global` so reads can disambiguate.
pub fn record(invocation: &Invocation, kind: SignalKind, delta: serde_json::Value) -> std::io::Result<()> {
    let now = now_rfc3339();
    let ev_project = TasteEvent {
        ts: now.clone(),
        invocation_id: invocation.id.clone(),
        signal_kind: kind,
        source: Source::Project,
        delta: delta.clone(),
    };
    let ev_global = TasteEvent {
        ts: now,
        invocation_id: invocation.id.clone(),
        signal_kind: kind,
        source: Source::Global,
        delta,
    };
    append(&project_path(&invocation.target_repo), &ev_project)?;
    append(&user_global_path(), &ev_global)?;
    Ok(())
}

fn append(path: &Path, event: &TasteEvent) -> std::io::Result<()> {
    let line = serde_json::to_string(event).expect("event serialization is infallible");
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    writeln!(f, "{line}")?;
    Ok(())
}

/// Snapshot of accumulated taste, ready to feed into a generation pass.
/// Resolved as global ⊕ project at read time (seed-mandated).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TasteSnapshot {
    /// Number of explicit Regenerate events in the project store for any intent.
    pub project_regenerate_count: u32,
    /// Same, in the global store.
    pub global_regenerate_count: u32,
    /// Most recent events (up to 8) — used by LLM provider to seed a
    /// "User taste signals so far" system-prompt section, and by the
    /// offline generator to perturb its deterministic hash seed.
    pub recent: Vec<TasteEvent>,
}

impl TasteSnapshot {
    /// Combined Regenerate count; offline path uses this to perturb the hash.
    pub fn regenerate_count(&self) -> u32 {
        // Project takes precedence (per seed); cap weight so a noisy global
        // store doesn't drown a fresh project.
        self.project_regenerate_count * 2 + self.global_regenerate_count
    }
}

/// Snapshot for a generation call. Reads project + global, dedupes, sorts.
pub fn snapshot_for(project_root: &Path) -> TasteSnapshot {
    let events = read_all(project_root).unwrap_or_default();
    let mut project_re = 0u32;
    let mut global_re = 0u32;
    for ev in &events {
        if ev.signal_kind == SignalKind::Regenerate {
            match ev.source {
                Source::Project => project_re += 1,
                Source::Global => global_re += 1,
            }
        }
    }
    let mut recent = events;
    // Keep last 8 by timestamp (string sort works for RFC 3339).
    recent.sort_by(|a, b| a.ts.cmp(&b.ts));
    let cut = recent.len().saturating_sub(8);
    let recent = recent.split_off(cut);
    TasteSnapshot {
        project_regenerate_count: project_re,
        global_regenerate_count: global_re,
        recent,
    }
}

/// Read both stores; project-source events take precedence on conflict.
pub fn read_all(project_root: &Path) -> std::io::Result<Vec<TasteEvent>> {
    let mut out = Vec::new();
    for p in [user_global_path(), project_path(project_root)] {
        if let Ok(s) = std::fs::read_to_string(&p) {
            for line in s.lines().filter(|l| !l.trim().is_empty()) {
                if let Ok(ev) = serde_json::from_str::<TasteEvent>(line) {
                    out.push(ev);
                }
            }
        }
    }
    out.sort_by(|a, b| a.ts.cmp(&b.ts));
    Ok(out)
}
