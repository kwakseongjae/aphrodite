//! v1.0 RC.9 `aphrodite eval` — automate the cross-brand alpha sweep.
//!
//! Drives N invocations of the orchestrator end-to-end, captures per-run
//! Quality Score + audit warning count + wall-clock + LLM-call count,
//! aggregates a JSON report + a human-readable summary table.
//!
//! The point: cross-brand alpha sweeps (Pass 50-55 Banchan + Hada +
//! Junjong + Yeoreum + Sori) were run manually one intent at a time. An
//! `aphrodite eval intents.json` lets the user (or CI) launch the whole
//! sweep with one command and review aggregated regressions.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, Clone, Deserialize)]
pub struct EvalSpec {
    /// Identifier for this row in the sweep — used as the output
    /// directory name (lowercased + slugified).
    pub name: String,
    /// The intent string passed to `aphrodite create`.
    pub intent: String,
    /// Optional persona slug (`dieter-rams`, `kenya-hara`, etc).
    pub persona: Option<String>,
    /// Optional multi-page list. Empty / missing → single-page run.
    #[serde(default)]
    pub pages: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EvalResult {
    pub name: String,
    pub intent: String,
    pub persona: Option<String>,
    pub pages: Vec<String>,
    pub quality_score: u32,
    pub a11y: u32,
    pub mobile: u32,
    pub performance: u32,
    pub semantic: u32,
    pub audit_warning_count: usize,
    pub audit_warnings: Vec<String>,
    pub llm_calls: u32,
    pub wall_clock_s: f32,
    pub status: EvalStatus,
    pub error: Option<String>,
    pub output_dir: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EvalStatus {
    Ok,
    Warning,
    Failed,
}

#[derive(Debug, Clone, Serialize)]
pub struct EvalSummary {
    pub total: usize,
    pub ok: usize,
    pub warning: usize,
    pub failed: usize,
    pub mean_quality: f32,
    pub min_quality: u32,
    pub max_quality: u32,
    pub total_wall_clock_s: f32,
    pub total_llm_calls: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct EvalReport {
    pub summary: EvalSummary,
    pub results: Vec<EvalResult>,
}

/// Run a single intent and return its EvalResult. Each spec creates its
/// own subdirectory under `out_root` so artifacts don't collide.
pub async fn run_one(
    spec: &EvalSpec,
    out_root: &PathBuf,
    max_turns: u32,
    satisfaction_threshold: f32,
) -> EvalResult {
    let slug = slug(&spec.name);
    let out_dir = out_root.join(&slug);
    let _ = std::fs::create_dir_all(&out_dir);

    let t0 = Instant::now();
    let result = crate::orchestrator::run(
        spec.intent.clone(),
        max_turns,
        satisfaction_threshold,
        spec.persona.clone(),
        false,
        Some(out_dir.clone()),
        spec.pages.clone(),
    )
    .await;

    match result {
        Ok(v) => {
            let qa = &v["quality_axes"];
            let warnings: Vec<String> = v["harmonize"]["quality_warnings"]
                .as_array()
                .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect())
                .unwrap_or_default();
            let quality_score = v["quality_score"].as_u64().unwrap_or(0) as u32;
            let status = if quality_score >= 90 {
                EvalStatus::Ok
            } else if quality_score >= 75 {
                EvalStatus::Warning
            } else {
                EvalStatus::Failed
            };
            EvalResult {
                name: spec.name.clone(),
                intent: spec.intent.clone(),
                persona: spec.persona.clone(),
                pages: spec.pages.clone(),
                quality_score,
                a11y: qa["a11y"].as_u64().unwrap_or(0) as u32,
                mobile: qa["mobile"].as_u64().unwrap_or(0) as u32,
                performance: qa["performance"].as_u64().unwrap_or(0) as u32,
                semantic: qa["semantic"].as_u64().unwrap_or(0) as u32,
                audit_warning_count: warnings.len(),
                audit_warnings: warnings,
                llm_calls: v["telemetry"]["llm_calls"].as_u64().unwrap_or(0) as u32,
                wall_clock_s: v["telemetry"]["wall_clock_s"].as_f64().unwrap_or(t0.elapsed().as_secs_f64()) as f32,
                status,
                error: None,
                output_dir: out_dir.to_string_lossy().into_owned(),
            }
        }
        Err(e) => EvalResult {
            name: spec.name.clone(),
            intent: spec.intent.clone(),
            persona: spec.persona.clone(),
            pages: spec.pages.clone(),
            quality_score: 0,
            a11y: 0,
            mobile: 0,
            performance: 0,
            semantic: 0,
            audit_warning_count: 0,
            audit_warnings: Vec::new(),
            llm_calls: 0,
            wall_clock_s: t0.elapsed().as_secs_f32(),
            status: EvalStatus::Failed,
            error: Some(format!("{e}")),
            output_dir: out_dir.to_string_lossy().into_owned(),
        },
    }
}

pub fn summarise(results: &[EvalResult]) -> EvalSummary {
    let total = results.len();
    let ok = results.iter().filter(|r| r.status == EvalStatus::Ok).count();
    let warning = results.iter().filter(|r| r.status == EvalStatus::Warning).count();
    let failed = results.iter().filter(|r| r.status == EvalStatus::Failed).count();
    let qs: Vec<u32> = results.iter().map(|r| r.quality_score).collect();
    let mean_quality = if total == 0 {
        0.0
    } else {
        qs.iter().map(|x| *x as f32).sum::<f32>() / total as f32
    };
    EvalSummary {
        total,
        ok,
        warning,
        failed,
        mean_quality,
        min_quality: qs.iter().copied().min().unwrap_or(0),
        max_quality: qs.iter().copied().max().unwrap_or(0),
        total_wall_clock_s: results.iter().map(|r| r.wall_clock_s).sum(),
        total_llm_calls: results.iter().map(|r| r.llm_calls).sum(),
    }
}

fn slug(s: &str) -> String {
    s.trim()
        .to_ascii_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Parse a JSON spec file. Two shapes accepted:
///   1. Plain array: `[ {spec...}, ... ]`
///   2. Object with `intents`: `{ "intents": [ ... ] }`
pub fn parse_spec_file(path: &std::path::Path) -> anyhow::Result<Vec<EvalSpec>> {
    let raw = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("cannot read {}: {e}", path.display()))?;
    let v: serde_json::Value = serde_json::from_str(&raw)?;
    let arr = if v.is_array() {
        v
    } else {
        v.get("intents").cloned().ok_or_else(|| {
            anyhow::anyhow!("spec file must be an array of intents OR an object with `intents` key")
        })?
    };
    let specs: Vec<EvalSpec> = serde_json::from_value(arr)?;
    Ok(specs)
}

/// Render a one-line summary table row for the terminal. Padded for
/// alignment when called from a loop.
pub fn format_row(r: &EvalResult) -> String {
    let badge = match r.status {
        EvalStatus::Ok => "✓",
        EvalStatus::Warning => "•",
        EvalStatus::Failed => "✗",
    };
    format!(
        "{badge} {name:<24} {q:>3}/100  (a11y={a:>3} mobile={m:>3} perf={p:>3} semantic={s:>3})  warnings={w} llm={llm} wall={wall:.0}s",
        name = truncate(&r.name, 24),
        q = r.quality_score,
        a = r.a11y,
        m = r.mobile,
        p = r.performance,
        s = r.semantic,
        w = r.audit_warning_count,
        llm = r.llm_calls,
        wall = r.wall_clock_s,
    )
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(n - 1).collect();
        out.push('…');
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_spec_array_form() {
        let path = std::env::temp_dir().join("aph-eval-spec-array.json");
        std::fs::write(
            &path,
            r#"[{"name":"banchan","intent":"food brand","pages":["home","about"]}]"#,
        )
        .unwrap();
        let specs = parse_spec_file(&path).unwrap();
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].name, "banchan");
        assert_eq!(specs[0].pages, vec!["home", "about"]);
    }

    #[test]
    fn parse_spec_object_form() {
        let path = std::env::temp_dir().join("aph-eval-spec-obj.json");
        std::fs::write(
            &path,
            r#"{"intents":[{"name":"hada","intent":"fintech","persona":"dieter-rams"}]}"#,
        )
        .unwrap();
        let specs = parse_spec_file(&path).unwrap();
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].persona.as_deref(), Some("dieter-rams"));
    }

    #[test]
    fn summarise_computes_correct_aggregates() {
        let mk = |q: u32, status: EvalStatus, llm: u32, wall: f32| EvalResult {
            name: "x".into(),
            intent: "y".into(),
            persona: None,
            pages: vec![],
            quality_score: q,
            a11y: 100,
            mobile: 100,
            performance: 100,
            semantic: q,
            audit_warning_count: 0,
            audit_warnings: vec![],
            llm_calls: llm,
            wall_clock_s: wall,
            status,
            error: None,
            output_dir: "x".into(),
        };
        let results = vec![
            mk(100, EvalStatus::Ok, 8, 1000.0),
            mk(85, EvalStatus::Warning, 10, 1200.0),
            mk(60, EvalStatus::Failed, 7, 800.0),
        ];
        let s = summarise(&results);
        assert_eq!(s.total, 3);
        assert_eq!(s.ok, 1);
        assert_eq!(s.warning, 1);
        assert_eq!(s.failed, 1);
        assert!((s.mean_quality - 81.66667).abs() < 0.01);
        assert_eq!(s.min_quality, 60);
        assert_eq!(s.max_quality, 100);
        assert_eq!(s.total_llm_calls, 25);
        assert!((s.total_wall_clock_s - 3000.0).abs() < 0.01);
    }
}
