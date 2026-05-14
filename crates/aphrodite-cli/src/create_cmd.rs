//! `aphrodite create "<intent>"` — autonomous creation harness entry point
//! (ADR 0004 / Phase 4 prototype).
//!
//! Internal loop:
//!   1. design   — initial generation (same as `aphrodite design`)
//!   2. critique — self-critic identifies weakest axis or declares satisfaction
//!   3. refine   — apply the critic's proposed delta
//!   4. goto 2 until satisfied or `--max-turns` exhausted
//!   5. emit final artifacts + structured turn-by-turn report
//!
//! Future phases (research, asset, harmonizer, skillify) are stubbed at the
//! report-shape level: the JSON output includes `phases: { ... }` so callers
//! can already key off them while the implementations land later.

use aphrodite_core::{
    parse_design, preferences, record_taste, resolve_variants, validate_design, Caller,
    Invocation, SignalKind, Surface, WriteMode,
};
use aphrodite_generator::{critic, refine, surface};
use serde_json::json;
use std::path::PathBuf;
use std::time::Instant;

pub async fn run(
    intent: String,
    max_turns: u32,
    satisfaction_threshold: f32,
    no_write: bool,
    repo: Option<PathBuf>,
) -> anyhow::Result<serde_json::Value> {
    let target = match repo {
        Some(p) => p,
        None => std::env::current_dir()?,
    };
    let target = target.canonicalize().unwrap_or(target);
    let resolved = aphrodite_generator::resolve_default_provider().ok_or_else(|| {
        anyhow::anyhow!("no provider credential reachable. Run `aphrodite doctor`.")
    })?;

    let write_mode = if no_write {
        WriteMode::ArtifactOnly
    } else {
        WriteMode::Commit
    };
    let invocation = Invocation {
        id: uuid::Uuid::new_v4().to_string(),
        caller: Caller::Human,
        surface: Surface::Cli,
        intent: intent.clone(),
        target_repo: target.clone(),
        write_mode,
    };

    let started = Instant::now();
    let mut llm_calls = 0u32;

    // ---- Phase 3: initial design --------------------------------------------
    eprintln!("● phase 3 / design …");
    let phase3_start = Instant::now();
    let output = aphrodite_generator::generate_with(&invocation, Some(&resolved)).await?;
    llm_calls += 2; // design + surface compose
    let mut design_md = output.design_md;
    let mut composition_html = output.composition_html;
    let mut hero_html = output.hero_html;
    eprintln!(
        "  ✓ design in {:.1}s (provider={}, model={})",
        phase3_start.elapsed().as_secs_f32(),
        output.provider_used,
        output.model_used
    );

    // ---- Phase 4: self-critic refinement loop -------------------------------
    eprintln!("● phase 4 / self-critic refinement (max_turns={max_turns}, threshold={satisfaction_threshold:.2})");
    let prefs = preferences::load(&target);
    let pref_hint = prefs.as_prompt_hint();
    let mut turns: Vec<serde_json::Value> = Vec::new();
    let mut prior_deltas: Vec<String> = Vec::new();
    let mut final_satisfaction = 0.0f32;
    let mut stop_reason = "unknown".to_string();

    for turn in 1..=max_turns {
        let t_start = Instant::now();
        eprintln!("  → turn {turn} / critique …");
        let verdict_result = critic::critique(
            &resolved,
            &intent,
            &design_md,
            &composition_html,
            &prior_deltas,
            &pref_hint,
        )
        .await;
        llm_calls += 1;
        let verdict = match verdict_result {
            Ok(v) => v,
            Err(e) => {
                eprintln!("    ⚠ critic failed: {e} — stopping loop");
                stop_reason = format!("critic_error: {e}");
                turns.push(json!({
                    "turn": turn,
                    "phase": "critique",
                    "error": e.to_string(),
                }));
                break;
            }
        };
        eprintln!(
            "    satisfaction={:.2}  axis={}  rationale={:?}",
            verdict.satisfaction,
            verdict.weakest_axis.as_deref().unwrap_or("—"),
            verdict.rationale
        );
        final_satisfaction = verdict.satisfaction;

        if verdict.satisfied() {
            turns.push(json!({
                "turn": turn,
                "phase": "critique",
                "satisfaction": verdict.satisfaction,
                "weakest_axis": verdict.weakest_axis,
                "rationale": verdict.rationale,
                "proposed_delta": null,
                "duration_s": t_start.elapsed().as_secs_f32(),
            }));
            stop_reason = if verdict.satisfaction >= satisfaction_threshold {
                "satisfaction_threshold_met".to_string()
            } else {
                "critic_emitted_no_delta".to_string()
            };
            break;
        }
        let delta = verdict
            .proposed_delta
            .clone()
            .expect("satisfied() guarantees Some when not satisfied");

        eprintln!("  → turn {turn} / refine: {}", short(&delta, 90));
        let refine_start = Instant::now();
        let new_md = match refine::refine(&resolved, &design_md, &delta).await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("    ⚠ refine failed: {e} — stopping loop");
                stop_reason = format!("refine_error: {e}");
                turns.push(json!({
                    "turn": turn,
                    "phase": "refine",
                    "delta": delta,
                    "error": e.to_string(),
                }));
                break;
            }
        };
        llm_calls += 1;
        let new_doc = match parse_design(&new_md) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("    ⚠ refined DESIGN.md failed to parse: {e} — keeping prior turn");
                stop_reason = format!("refine_parse_error: {e}");
                turns.push(json!({
                    "turn": turn,
                    "phase": "refine",
                    "delta": delta,
                    "error": e.to_string(),
                }));
                break;
            }
        };
        let new_variants = resolve_variants(&new_doc);
        let new_composition = match surface::compose(&resolved, &intent, &new_md, &new_doc).await {
            Ok(out) => {
                llm_calls += 1;
                aphrodite_generator::hero::inject_variant_css(&out.html, &new_doc, &new_variants)
            }
            Err(e) => {
                eprintln!("    ⚠ surface re-compose failed: {e} — keeping prior composition");
                composition_html.clone()
            }
        };
        let new_hero = aphrodite_generator::hero::render(&new_doc, &new_variants)
            .map_err(|e| anyhow::anyhow!("hero render: {e}"))?;
        let report = validate_design(&new_doc, &new_variants);

        design_md = new_md;
        composition_html = new_composition;
        hero_html = new_hero;
        prior_deltas.push(delta.clone());

        turns.push(json!({
            "turn": turn,
            "phase": "refine",
            "satisfaction_before": verdict.satisfaction,
            "weakest_axis": verdict.weakest_axis,
            "rationale": verdict.rationale,
            "delta": delta,
            "validation_ok": report.is_ok(),
            "violations": report.violations.len(),
            "duration_s": refine_start.elapsed().as_secs_f32(),
        }));

        if turn == max_turns {
            stop_reason = "max_turns_reached".to_string();
        }
    }

    // ---- Write final artifacts ---------------------------------------------
    let final_doc = parse_design(&design_md)?;
    let final_variants = resolve_variants(&final_doc);
    let final_report = validate_design(&final_doc, &final_variants);

    let (design_path, hero_path, composition_path) = if no_write {
        let out = target.join(".aphrodite").join("out");
        std::fs::create_dir_all(&out)?;
        let dp = out.join("DESIGN.md");
        let hp = out.join("hero.html");
        std::fs::write(&dp, &design_md)?;
        std::fs::write(&hp, &hero_html)?;
        let cp = if !composition_html.is_empty() {
            let cp = out.join("composition.html");
            std::fs::write(&cp, &composition_html)?;
            Some(cp)
        } else {
            None
        };
        (dp, hp, cp)
    } else {
        let dp = target.join("DESIGN.md");
        let hp = target.join("hero.html");
        std::fs::write(&dp, &design_md)?;
        std::fs::write(&hp, &hero_html)?;
        let cp = if !composition_html.is_empty() {
            let cp = target.join("composition.html");
            std::fs::write(&cp, &composition_html)?;
            Some(cp)
        } else {
            None
        };
        // Persist intent for future refine calls.
        let _ = std::fs::create_dir_all(target.join(".aphrodite"));
        let _ = std::fs::write(target.join(".aphrodite").join("intent.txt"), &intent);
        // Single auto-commit summarising the whole create run.
        if target.join(".git").exists() {
            use std::process::{Command, Stdio};
            let mut paths = vec![dp.clone(), hp.clone()];
            if let Some(c) = cp.as_ref() {
                paths.push(c.clone());
            }
            for f in &paths {
                let _ = Command::new("git")
                    .arg("-C")
                    .arg(&target)
                    .arg("add")
                    .arg(f)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();
            }
            let _ = Command::new("git")
                .arg("-C")
                .arg(&target)
                .arg("commit")
                .arg("-m")
                .arg(format!(
                    "Aphrodite create: {} ({} turns, satisfaction {:.2})",
                    short(&intent, 50),
                    prior_deltas.len(),
                    final_satisfaction
                ))
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
        (dp, hp, cp)
    };

    // Single Accept (or Regenerate if no turns made it satisfied) taste event
    // covering the create run as a whole. Per-turn events would double-count.
    let signal = if final_satisfaction >= satisfaction_threshold && prior_deltas.is_empty() {
        SignalKind::Accept
    } else {
        SignalKind::Regenerate
    };
    let _ = record_taste(
        &invocation,
        signal,
        json!({
            "kind": "create",
            "intent": intent,
            "turns": prior_deltas.len(),
            "satisfaction": final_satisfaction,
            "provider": output.provider_used,
        }),
    );

    let total_s = started.elapsed().as_secs_f32();
    eprintln!(
        "● done: {} turns, satisfaction={:.2}, total={:.1}s, llm_calls={}",
        prior_deltas.len(),
        final_satisfaction,
        total_s,
        llm_calls
    );

    Ok(json!({
        "kind": "create",
        "invocation_id": invocation.id,
        "intent": intent,
        "output": {
            "design_path": design_path.to_string_lossy(),
            "hero_path": hero_path.to_string_lossy(),
            "composition_path": composition_path.as_ref().map(|p| p.to_string_lossy().to_string()),
            "validation": {
                "ok": final_report.is_ok(),
                "violations": final_report.violations,
            },
            "warnings": output.warnings,
        },
        "loop": {
            "turns_completed": prior_deltas.len(),
            "max_turns": max_turns,
            "final_satisfaction": final_satisfaction,
            "satisfaction_threshold": satisfaction_threshold,
            "stop_reason": stop_reason,
            "deltas": prior_deltas,
            "turn_log": turns,
        },
        "phases": {
            "research": "stub",
            "taste_application": if pref_hint.is_empty() { "no_prior_signal" } else { "applied" },
            "design": "done",
            "self_critic_refine": "done",
            "asset_create": "stub",
            "asset_manage": "stub",
            "harmonize": "stub",
            "skillify": "stub",
        },
        "telemetry": {
            "llm_calls": llm_calls,
            "wall_clock_s": total_s,
            "provider": output.provider_used,
            "model": output.model_used,
        },
    }))
}

fn short(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(max - 1).collect();
        out.push('…');
        out
    }
}
