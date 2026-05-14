//! `aphrodite refine "<change>"` — multi-turn refinement. Reads the DESIGN.md
//! in the current repo, sends it as context to the LLM with the user's delta
//! request, writes the revised DESIGN.md + re-composes the surface.

use aphrodite_core::{parse_design, record_taste, resolve_variants, validate_design, Caller, Invocation, SignalKind, Surface, WriteMode};
use serde_json::json;
use std::path::PathBuf;

pub async fn run(delta: String, no_write: bool, repo: Option<PathBuf>) -> anyhow::Result<serde_json::Value> {
    let target = match repo {
        Some(p) => p,
        None => std::env::current_dir()?,
    };
    let target = target.canonicalize().unwrap_or(target);
    let design_path = target.join("DESIGN.md");
    if !design_path.exists() {
        anyhow::bail!(
            "no DESIGN.md at {} — run `aphrodite design \"<intent>\"` first, then `aphrodite refine \"<change>\"` to iterate.",
            design_path.display()
        );
    }
    let current_md = std::fs::read_to_string(&design_path)?;
    let resolved = aphrodite_generator::resolve_default_provider()
        .ok_or_else(|| anyhow::anyhow!("no provider credential reachable. Run `aphrodite doctor`."))?;

    eprintln!("Refining DESIGN.md with delta: {}", delta);
    let new_md = aphrodite_generator::refine::refine(&resolved, &current_md, &delta).await?;
    let new_doc = parse_design(&new_md)?;
    let new_variants = resolve_variants(&new_doc);
    let report = validate_design(&new_doc, &new_variants);

    // Re-compose surface against the revised DESIGN.md. Use the original
    // intent saved at first design call (if present) so the composer keeps
    // context; otherwise fall back to the delta itself.
    let intent_path = target.join(".aphrodite").join("intent.txt");
    let original_intent = std::fs::read_to_string(&intent_path)
        .ok()
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| delta.clone());
    let composition_text = match aphrodite_generator::surface::compose(&resolved, &original_intent, &new_md, &new_doc).await {
        Ok(s) => Some(aphrodite_generator::hero::inject_variant_css(&s.html, &new_doc, &new_variants)),
        Err(e) => {
            eprintln!("  ⚠ surface composer failed: {e} — keeping DESIGN.md change only");
            None
        }
    };

    // Hero (template fallback) always re-renders.
    let hero_html = aphrodite_generator::hero::render(&new_doc, &new_variants)
        .map_err(|e| anyhow::anyhow!("hero render: {e}"))?;

    // Write.
    let (design_dst, hero_dst, composition_dst) = if no_write {
        let out = target.join(".aphrodite").join("out");
        std::fs::create_dir_all(&out)?;
        let dp = out.join("DESIGN.md");
        let hp = out.join("hero.html");
        std::fs::write(&dp, &new_md)?;
        std::fs::write(&hp, &hero_html)?;
        let cp = composition_text.as_ref().map(|t| {
            let p = out.join("composition.html");
            std::fs::write(&p, t).ok();
            p
        });
        (dp, hp, cp)
    } else {
        std::fs::write(&design_path, &new_md)?;
        let hp = target.join("hero.html");
        std::fs::write(&hp, &hero_html)?;
        let cp = composition_text.as_ref().map(|t| {
            let p = target.join("composition.html");
            std::fs::write(&p, t).ok();
            p
        });
        // Git commit if possible.
        if target.join(".git").exists() {
            use std::process::{Command, Stdio};
            let mut paths = vec![design_path.clone(), hp.clone()];
            if let Some(c) = cp.as_ref() { paths.push(c.clone()); }
            for f in &paths {
                let _ = Command::new("git").arg("-C").arg(&target).arg("add").arg(f).stdout(Stdio::null()).stderr(Stdio::null()).status();
            }
            let _ = Command::new("git").arg("-C").arg(&target).arg("commit").arg("-m").arg(format!("Aphrodite refine: {}", truncate(&delta, 60))).stdout(Stdio::null()).stderr(Stdio::null()).status();
        }
        (design_path.clone(), hp, cp)
    };

    // Record this turn as a Regenerate event (multi-turn = explicit retry).
    let invocation = Invocation {
        id: uuid::Uuid::new_v4().to_string(),
        caller: Caller::Human,
        surface: Surface::Cli,
        intent: original_intent.clone(),
        target_repo: target.clone(),
        write_mode: if no_write { WriteMode::ArtifactOnly } else { WriteMode::Commit },
    };
    let _ = record_taste(&invocation, SignalKind::Regenerate, json!({ "kind": "refine", "delta": delta }));

    Ok(json!({
        "kind": "refine",
        "delta": delta,
        "invocation_id": invocation.id,
        "output": {
            "design_path": design_dst.to_string_lossy(),
            "hero_path": hero_dst.to_string_lossy(),
            "composition_path": composition_dst.as_ref().map(|p| p.to_string_lossy().to_string()),
            "validation": {
                "ok": report.is_ok(),
                "violations": report.violations,
            },
        }
    }))
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max { s.to_string() } else { s.chars().take(max - 1).chain(std::iter::once('…')).collect() }
}
