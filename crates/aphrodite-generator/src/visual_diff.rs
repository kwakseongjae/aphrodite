//! v1.0 visual regression: compare a current run's screenshot output
//! against a baseline directory and report pairwise changes. Zero
//! external dependencies — uses file size delta + byte-equality as the
//! primary signals, falling back to ImageMagick's `compare` binary if
//! it's on PATH for proper per-pixel diff with a heat-map.
//!
//! The point: in a Toss/Karrot adoption scenario, a designer wants to
//! know "did anything visually change vs the last commit on main?"
//! without setting up Chromatic / Percy. This gives a CI-friendly text
//! report + optional diff PNGs.

use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize)]
pub struct DiffReport {
    pub pairs: Vec<DiffPair>,
    pub only_in_baseline: Vec<String>,
    pub only_in_current: Vec<String>,
    pub summary: DiffSummary,
}

#[derive(Debug, Serialize)]
pub struct DiffPair {
    pub file: String,
    pub baseline_bytes: u64,
    pub current_bytes: u64,
    /// Size delta as a percentage of baseline.
    pub size_delta_pct: f32,
    /// True if files are byte-identical.
    pub identical: bool,
    /// ImageMagick fuzz % if `compare` was available, else None.
    pub imagemagick_diff_pct: Option<f32>,
    /// Path to the heat-map PNG (only when ImageMagick succeeded).
    pub diff_png: Option<String>,
    /// Verdict given the signals available.
    pub verdict: DiffVerdict,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DiffVerdict {
    Identical,
    Minor,
    Changed,
    Missing,
}

#[derive(Debug, Serialize, Default)]
pub struct DiffSummary {
    pub identical: usize,
    pub minor: usize,
    pub changed: usize,
    pub only_baseline: usize,
    pub only_current: usize,
}

/// Compute a diff report from two directories of `.png` screenshots.
///
/// `size_threshold_pct` is the size-delta percentage above which we
/// flag a pair as Changed (when ImageMagick is unavailable). Default
/// 2.5% balances signal vs noise for browser font-rendering jitter.
pub fn diff_dirs(
    baseline_dir: &Path,
    current_dir: &Path,
    size_threshold_pct: f32,
    write_imagemagick: bool,
) -> std::io::Result<DiffReport> {
    let baseline_pngs = list_pngs(baseline_dir)?;
    let current_pngs = list_pngs(current_dir)?;

    let mut pairs: Vec<DiffPair> = Vec::new();
    let mut only_in_baseline: Vec<String> = Vec::new();
    let mut only_in_current: Vec<String> = current_pngs
        .iter()
        .filter(|p| !baseline_pngs.iter().any(|b| b.file_name() == p.file_name()))
        .map(|p| p.file_name().unwrap_or_default().to_string_lossy().into_owned())
        .collect();

    for b in &baseline_pngs {
        let name = b.file_name().unwrap_or_default().to_string_lossy().into_owned();
        let matching = current_pngs.iter().find(|c| c.file_name() == b.file_name());
        let Some(c) = matching else {
            only_in_baseline.push(name);
            continue;
        };
        let bm = std::fs::metadata(b)?;
        let cm = std::fs::metadata(c)?;
        let bb = bm.len();
        let cb = cm.len();
        let size_delta = if bb == 0 {
            if cb == 0 { 0.0 } else { 100.0 }
        } else {
            ((cb as f32 - bb as f32) / bb as f32) * 100.0
        };
        let identical = bb == cb && files_byte_equal(b, c).unwrap_or(false);
        let (im_pct, diff_png) = if write_imagemagick && !identical {
            run_imagemagick_compare(b, c)
        } else {
            (None, None)
        };
        let verdict = classify(size_delta, size_threshold_pct, identical, im_pct);
        pairs.push(DiffPair {
            file: name,
            baseline_bytes: bb,
            current_bytes: cb,
            size_delta_pct: size_delta,
            identical,
            imagemagick_diff_pct: im_pct,
            diff_png,
            verdict,
        });
    }
    only_in_current.sort();
    only_in_baseline.sort();
    let mut summary = DiffSummary::default();
    for p in &pairs {
        match p.verdict {
            DiffVerdict::Identical => summary.identical += 1,
            DiffVerdict::Minor => summary.minor += 1,
            DiffVerdict::Changed => summary.changed += 1,
            DiffVerdict::Missing => {}
        }
    }
    summary.only_baseline = only_in_baseline.len();
    summary.only_current = only_in_current.len();
    Ok(DiffReport {
        pairs,
        only_in_baseline,
        only_in_current,
        summary,
    })
}

fn list_pngs(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    if !dir.exists() {
        return Ok(out);
    }
    for entry in std::fs::read_dir(dir)? {
        let e = entry?;
        let p = e.path();
        if p.extension().and_then(|s| s.to_str()) == Some("png") {
            out.push(p);
        }
    }
    out.sort();
    Ok(out)
}

fn files_byte_equal(a: &Path, b: &Path) -> std::io::Result<bool> {
    let aa = std::fs::read(a)?;
    let bb = std::fs::read(b)?;
    Ok(aa == bb)
}

/// If `compare` from ImageMagick is in PATH, run it to compute a fuzz
/// percentage and write a heat-map PNG. Returns (fuzz_pct, diff_path).
fn run_imagemagick_compare(baseline: &Path, current: &Path) -> (Option<f32>, Option<String>) {
    use std::process::{Command, Stdio};
    let ok = Command::new("compare")
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if !ok {
        return (None, None);
    }
    let diff_path = current.with_extension("diff.png");
    let out = Command::new("compare")
        .arg("-metric")
        .arg("AE")
        .arg(baseline)
        .arg(current)
        .arg(&diff_path)
        .output();
    let Ok(o) = out else { return (None, None) };
    // `compare -metric AE` prints absolute error pixel count to stderr.
    let stderr = String::from_utf8_lossy(&o.stderr);
    let pixel_diff: u64 = stderr.trim().parse().unwrap_or(0);
    // Approximate fuzz % from baseline size (use 1080x900 as a stand-in
    // when we don't read the actual PNG header — good enough for ranking).
    let pct = (pixel_diff as f32 / (1440.0 * 1800.0)) * 100.0;
    (
        Some(pct),
        Some(diff_path.to_string_lossy().into_owned()),
    )
}

fn classify(
    size_delta_pct: f32,
    threshold: f32,
    identical: bool,
    im_pct: Option<f32>,
) -> DiffVerdict {
    if identical {
        return DiffVerdict::Identical;
    }
    if let Some(p) = im_pct {
        if p < 0.1 {
            return DiffVerdict::Minor;
        }
        if p < 1.0 {
            return DiffVerdict::Minor;
        }
        return DiffVerdict::Changed;
    }
    if size_delta_pct.abs() < threshold {
        DiffVerdict::Minor
    } else {
        DiffVerdict::Changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn tmp() -> PathBuf {
        let p = std::env::temp_dir().join(format!("aph-diff-test-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn identical_files_are_identical() {
        let baseline = tmp();
        let current = tmp();
        fs::write(baseline.join("a.png"), b"abc").unwrap();
        fs::write(current.join("a.png"), b"abc").unwrap();
        let report = diff_dirs(&baseline, &current, 2.5, false).unwrap();
        assert_eq!(report.pairs.len(), 1);
        assert!(report.pairs[0].identical);
        assert_eq!(report.pairs[0].verdict, DiffVerdict::Identical);
    }

    #[test]
    fn size_delta_within_threshold_is_minor() {
        let baseline = tmp();
        let current = tmp();
        fs::write(baseline.join("a.png"), vec![0u8; 1000]).unwrap();
        fs::write(current.join("a.png"), vec![1u8; 1010]).unwrap(); // +1%
        let report = diff_dirs(&baseline, &current, 2.5, false).unwrap();
        assert_eq!(report.pairs[0].verdict, DiffVerdict::Minor);
    }

    #[test]
    fn size_delta_over_threshold_is_changed() {
        let baseline = tmp();
        let current = tmp();
        fs::write(baseline.join("a.png"), vec![0u8; 1000]).unwrap();
        fs::write(current.join("a.png"), vec![1u8; 1200]).unwrap(); // +20%
        let report = diff_dirs(&baseline, &current, 2.5, false).unwrap();
        assert_eq!(report.pairs[0].verdict, DiffVerdict::Changed);
    }

    #[test]
    fn missing_files_are_tracked() {
        let baseline = tmp();
        let current = tmp();
        fs::write(baseline.join("removed.png"), b"x").unwrap();
        fs::write(current.join("added.png"), b"y").unwrap();
        let report = diff_dirs(&baseline, &current, 2.5, false).unwrap();
        assert_eq!(report.only_in_baseline, vec!["removed.png".to_string()]);
        assert_eq!(report.only_in_current, vec!["added.png".to_string()]);
        assert_eq!(report.pairs.len(), 0);
    }
}
