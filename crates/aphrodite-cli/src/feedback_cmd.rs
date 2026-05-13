//! `aphrodite love` / `aphrodite hate` — explicit feedback that updates the
//! accumulated TastePreferences. Both auto-detect the most recent run in the
//! target repo (DESIGN.md in cwd) and apply the corresponding weight.

use aphrodite_core::preferences::{self, Observation, TastePreferences};
use aphrodite_generator::extractor;
use serde_json::json;
use std::path::PathBuf;

pub fn run(weight: f32, label: &str, repo: Option<PathBuf>) -> anyhow::Result<serde_json::Value> {
    let target = match repo {
        Some(p) => p,
        None => std::env::current_dir()?,
    };
    let design_path = target.join("DESIGN.md");
    if !design_path.exists() {
        anyhow::bail!(
            "no DESIGN.md found at {} — run `aphrodite design \"…\"` first, then `aphrodite {label}` to record the signal.",
            design_path.display()
        );
    }
    let md = std::fs::read_to_string(&design_path)?;
    let obs = extractor::observe(&md);

    let mut prefs = preferences::load(&target);
    prefs.apply(obs.clone(), weight);
    preferences::save(&target, &prefs)?;

    let mut global = read_global();
    global.apply(obs.clone(), weight * 0.5); // half-weight to global; project is primary
    let _ = preferences::save_global(&global);

    Ok(json!({
        "kind": "feedback",
        "label": label,
        "weight": weight,
        "observation": {
            "hue_family": obs.hue_family,
            "display_font": obs.display_font,
            "body_font": obs.body_font,
            "density": obs.density,
            "serif_vs_sans": obs.serif_vs_sans,
            "accent_intensity": obs.accent_intensity,
        },
        "preferences_after": {
            "total_runs_observed": prefs.total_runs_observed,
            "top_hue_families": top_n(&prefs.hue_families, 5),
            "density": prefs.density,
            "serif_vs_sans": prefs.serif_vs_sans,
            "accent_intensity": prefs.accent_intensity,
        }
    }))
}

pub fn show(repo: Option<PathBuf>) -> anyhow::Result<serde_json::Value> {
    let target = match repo {
        Some(p) => p,
        None => std::env::current_dir()?,
    };
    let prefs = preferences::load(&target);
    println!("Aphrodite taste preferences (merged: global + project)");
    println!("  total runs observed : {}", prefs.total_runs_observed);
    if prefs.total_runs_observed > 0 {
        let hint = prefs.as_prompt_hint();
        if hint.is_empty() {
            println!("  (no strong signal yet — keep using design / love / hate)");
        } else {
            for line in hint.lines() {
                println!("  {line}");
            }
        }
    } else {
        println!("  (no feedback recorded — try `aphrodite design \"…\"` then `aphrodite love` or `aphrodite hate`)");
    }
    Ok(serde_json::to_value(&prefs)?)
}

fn read_global() -> TastePreferences {
    let path = preferences::global_path();
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default()
}

fn top_n<T: Clone + serde::Serialize>(map: &std::collections::BTreeMap<String, T>, n: usize) -> serde_json::Value
where
    T: PartialOrd,
{
    let mut entries: Vec<_> = map.iter().collect();
    entries.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));
    serde_json::to_value(entries.into_iter().take(n).map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>()).unwrap_or_default()
}
