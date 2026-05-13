//! Taste preferences — accumulated patterns from user feedback.
//!
//! Unlike `TasteEvent` (one row per call), `TastePreferences` is a *summary*:
//! human-readable hints the generator can inject into LLM prompts so each
//! design call reflects what the user has liked / hated before.
//!
//! Update rules:
//!   Accept    →  +0.10 weight on this run's observable patterns
//!   Regenerate → −0.10
//!   Love      →  +0.30
//!   Hate      → −0.30
//!
//! Storage: `~/.aphrodite/taste/preferences.toml` (user-global) and
//! `<project>/.aphrodite/preferences.toml` (project-override). Reads merge,
//! writes go to project if a project dir is supplied.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TastePreferences {
    /// Hue family names → weight in [-1.0, +1.0]. Examples: "warm", "cool",
    /// "neutral", "saturated", "muted". Positive = preferred.
    #[serde(default)]
    pub hue_families: BTreeMap<String, f32>,
    /// Font families → weight.
    #[serde(default)]
    pub display_fonts: BTreeMap<String, f32>,
    #[serde(default)]
    pub body_fonts: BTreeMap<String, f32>,
    /// Categorical preferences in [-1, +1]:
    ///   `density` — negative = generous whitespace, positive = dense
    ///   `serif_vs_sans` — negative = serif display, positive = sans
    ///   `accent_intensity` — negative = restrained, positive = bold
    #[serde(default)]
    pub density: f32,
    #[serde(default)]
    pub serif_vs_sans: f32,
    #[serde(default)]
    pub accent_intensity: f32,
    #[serde(default)]
    pub total_runs_observed: u32,
    #[serde(default)]
    pub last_updated: String,
}

impl TastePreferences {
    fn clamp(x: f32) -> f32 {
        x.max(-1.0).min(1.0)
    }

    /// Apply a delta from a single observed run.
    pub fn apply(&mut self, observation: Observation, weight: f32) {
        if let Some(h) = observation.hue_family {
            let e = self.hue_families.entry(h).or_insert(0.0);
            *e = Self::clamp(*e + weight);
        }
        if let Some(f) = observation.display_font {
            let e = self.display_fonts.entry(f).or_insert(0.0);
            *e = Self::clamp(*e + weight);
        }
        if let Some(f) = observation.body_font {
            let e = self.body_fonts.entry(f).or_insert(0.0);
            *e = Self::clamp(*e + weight);
        }
        if let Some(d) = observation.density {
            self.density = Self::clamp(self.density + weight * d);
        }
        if let Some(s) = observation.serif_vs_sans {
            self.serif_vs_sans = Self::clamp(self.serif_vs_sans + weight * s);
        }
        if let Some(a) = observation.accent_intensity {
            self.accent_intensity = Self::clamp(self.accent_intensity + weight * a);
        }
        self.total_runs_observed = self.total_runs_observed.saturating_add(1);
        self.last_updated = crate::taste::now_rfc3339();
    }

    /// Render the preferences as a human-readable section for prompt injection.
    /// Returns empty string when no signal has accumulated.
    pub fn as_prompt_hint(&self) -> String {
        let mut lines = Vec::new();

        // Hue families — top positives + top negatives.
        let mut hues: Vec<(&String, &f32)> = self.hue_families.iter().filter(|(_, w)| w.abs() > 0.15).collect();
        hues.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));
        let liked_hues: Vec<&String> = hues.iter().filter(|(_, w)| **w > 0.0).map(|(k, _)| *k).take(3).collect();
        let disliked_hues: Vec<&String> = hues.iter().filter(|(_, w)| **w < 0.0).map(|(k, _)| *k).take(3).collect();
        if !liked_hues.is_empty() {
            lines.push(format!("- Prefers palettes in: {}", liked_hues.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")));
        }
        if !disliked_hues.is_empty() {
            lines.push(format!("- Has consistently disliked: {}", disliked_hues.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")));
        }

        // Fonts.
        let liked_display = top_n(&self.display_fonts, 3, true);
        let disliked_display = top_n(&self.display_fonts, 2, false);
        if !liked_display.is_empty() {
            lines.push(format!("- Display fonts the user has accepted: {}", liked_display.join(", ")));
        }
        if !disliked_display.is_empty() {
            lines.push(format!("- Display fonts the user has rejected: {}", disliked_display.join(", ")));
        }

        // Categorical axes.
        if self.density.abs() > 0.2 {
            let label = if self.density < 0.0 { "generous whitespace" } else { "dense, information-rich layouts" };
            lines.push(format!("- Leans toward {label} (signal {:+.1}).", self.density));
        }
        if self.serif_vs_sans.abs() > 0.2 {
            let label = if self.serif_vs_sans < 0.0 { "serif display typography" } else { "sans-serif at scale" };
            lines.push(format!("- Leans toward {label} (signal {:+.1}).", self.serif_vs_sans));
        }
        if self.accent_intensity.abs() > 0.2 {
            let label = if self.accent_intensity < 0.0 { "restrained, monochromatic accents" } else { "bold, saturated accents" };
            lines.push(format!("- Leans toward {label} (signal {:+.1}).", self.accent_intensity));
        }

        if lines.is_empty() {
            return String::new();
        }
        format!(
            "\nThe user has provided {} prior signal{} on similar tasks. Bias your output accordingly:\n{}",
            self.total_runs_observed,
            if self.total_runs_observed == 1 { "" } else { "s" },
            lines.join("\n")
        )
    }
}

fn top_n(map: &BTreeMap<String, f32>, n: usize, positive: bool) -> Vec<String> {
    let mut entries: Vec<(&String, &f32)> = map.iter().filter(|(_, w)| if positive { **w > 0.15 } else { **w < -0.15 }).collect();
    entries.sort_by(|a, b| if positive {
        b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal)
    } else {
        a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal)
    });
    entries.into_iter().take(n).map(|(k, _)| k.clone()).collect()
}

#[derive(Debug, Clone, Default)]
pub struct Observation {
    pub hue_family: Option<String>,
    pub display_font: Option<String>,
    pub body_font: Option<String>,
    pub density: Option<f32>,
    pub serif_vs_sans: Option<f32>,
    pub accent_intensity: Option<f32>,
}

pub fn global_path() -> PathBuf {
    let home = std::env::var_os("HOME").unwrap_or_default();
    let mut p = PathBuf::from(home);
    p.push(".aphrodite");
    p.push("taste");
    let _ = std::fs::create_dir_all(&p);
    p.push("preferences.toml");
    p
}

pub fn project_path(project_root: &Path) -> PathBuf {
    let mut p = project_root.to_path_buf();
    p.push(".aphrodite");
    let _ = std::fs::create_dir_all(&p);
    p.push("preferences.toml");
    p
}

/// Read project + global preferences and merge (project values are weighted
/// more heavily since they're scoped to the current work).
pub fn load(project_root: &Path) -> TastePreferences {
    let global = read_file(&global_path()).unwrap_or_default();
    let project = read_file(&project_path(project_root)).unwrap_or_default();

    let mut merged = global;
    // Merge project on top with 1.5× weight on shared keys.
    for (k, v) in &project.hue_families {
        let e = merged.hue_families.entry(k.clone()).or_insert(0.0);
        *e = TastePreferences::clamp(*e + v * 1.5);
    }
    for (k, v) in &project.display_fonts {
        let e = merged.display_fonts.entry(k.clone()).or_insert(0.0);
        *e = TastePreferences::clamp(*e + v * 1.5);
    }
    for (k, v) in &project.body_fonts {
        let e = merged.body_fonts.entry(k.clone()).or_insert(0.0);
        *e = TastePreferences::clamp(*e + v * 1.5);
    }
    merged.density = TastePreferences::clamp(merged.density + project.density * 1.5);
    merged.serif_vs_sans = TastePreferences::clamp(merged.serif_vs_sans + project.serif_vs_sans * 1.5);
    merged.accent_intensity = TastePreferences::clamp(merged.accent_intensity + project.accent_intensity * 1.5);
    merged.total_runs_observed = merged.total_runs_observed.saturating_add(project.total_runs_observed);
    merged
}

pub fn save(project_root: &Path, prefs: &TastePreferences) -> std::io::Result<()> {
    let path = project_path(project_root);
    let text = toml::to_string_pretty(prefs).map_err(|e| std::io::Error::other(e.to_string()))?;
    std::fs::write(&path, text)
}

pub fn save_global(prefs: &TastePreferences) -> std::io::Result<()> {
    let path = global_path();
    let text = toml::to_string_pretty(prefs).map_err(|e| std::io::Error::other(e.to_string()))?;
    std::fs::write(&path, text)
}

fn read_file(path: &Path) -> Option<TastePreferences> {
    let s = std::fs::read_to_string(path).ok()?;
    toml::from_str(&s).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn applies_weight_and_clamps() {
        let mut p = TastePreferences::default();
        let obs = Observation { hue_family: Some("warm".into()), density: Some(-0.5), ..Default::default() };
        for _ in 0..20 {
            p.apply(obs.clone(), 0.3);
        }
        assert_eq!(p.hue_families["warm"], 1.0); // clamped
        assert!(p.density <= -0.5);
    }

    #[test]
    fn prompt_hint_renders_when_signal_present() {
        let mut p = TastePreferences::default();
        p.hue_families.insert("warm".into(), 0.8);
        p.density = -0.6;
        let hint = p.as_prompt_hint();
        assert!(hint.contains("warm"));
        assert!(hint.contains("generous whitespace"));
    }
}
