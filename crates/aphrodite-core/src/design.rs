//! DESIGN.md model + parser.
//!
//! Format (Google Labs alpha): YAML frontmatter (`---` fenced) carries the token
//! graph; the markdown body carries rationale in ordered sections.
//!
//! Multi-mode lives under `metadata.variants` as an Aphrodite-specific extension,
//! per ADR-0002 (the spec's `metadata` escape hatch).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DesignError {
    #[error("frontmatter missing: a DESIGN.md must open with `---` YAML frontmatter")]
    FrontmatterMissing,
    #[error("frontmatter parse: {0}")]
    FrontmatterParse(#[from] serde_yaml::Error),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DesignDocument {
    pub frontmatter: Frontmatter,
    pub body: String,
    pub sections: Vec<Section>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Frontmatter {
    pub name: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub colors: Option<ColorTokens>,
    #[serde(default)]
    pub typography: Option<serde_yaml::Value>,
    #[serde(default)]
    pub rounded: Option<serde_yaml::Value>,
    #[serde(default)]
    pub spacing: Option<serde_yaml::Value>,
    #[serde(default)]
    pub components: Option<serde_yaml::Value>,
    #[serde(default)]
    pub metadata: Option<DesignMetadata>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ColorTokens {
    pub primary: BTreeMap<String, String>,
    /// Non-primary palettes (neutral, accent, …) AND any incidental keys the
    /// LLM may have included (e.g. a `description:` string). We accept any
    /// `serde_yaml::Value` here and let downstream resolution skip non-map
    /// entries — this is the difference between "the model wrote slightly
    /// outside our spec" and "the whole DESIGN.md is rejected."
    #[serde(flatten)]
    pub others: BTreeMap<String, serde_yaml::Value>,
}

impl ColorTokens {
    /// Iterate only the palette-shaped entries in `others` (Map<String,String>).
    /// Use this from variant resolution so a stray `description: "..."` doesn't
    /// break the build.
    pub fn other_palettes(&self) -> impl Iterator<Item = (&String, BTreeMap<String, String>)> {
        self.others.iter().filter_map(|(k, v)| match v {
            serde_yaml::Value::Mapping(m) => {
                let mut out = BTreeMap::new();
                for (mk, mv) in m {
                    if let (serde_yaml::Value::String(sk), serde_yaml::Value::String(sv)) = (mk, mv) {
                        out.insert(sk.clone(), sv.clone());
                    }
                }
                if out.is_empty() {
                    None
                } else {
                    Some((k, out))
                }
            }
            _ => None,
        })
    }
}

/// Aphrodite-specific frontmatter metadata for multi-mode variants.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DesignMetadata {
    /// `variants[name] = { tokens: {...} }` — overlay each on the base.
    #[serde(default)]
    pub variants: BTreeMap<String, VariantOverlay>,
    /// Free-form passthrough so we don't trip on extra keys.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VariantOverlay {
    /// Flat overrides, dotted keys (e.g. `colors.primary.500`).
    #[serde(default)]
    pub tokens: BTreeMap<String, String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub kind: SectionKind,
    pub heading: String,
    pub markdown: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SectionKind {
    Overview,
    Colors,
    Typography,
    Layout,
    Elevation,
    Shapes,
    Components,
    DosAndDonts,
    Unknown,
}

impl SectionKind {
    fn from_heading(h: &str) -> Self {
        let n = h.trim().to_ascii_lowercase();
        if n.starts_with("overview") {
            Self::Overview
        } else if n.starts_with("colors") || n.starts_with("color") {
            Self::Colors
        } else if n.starts_with("typography") {
            Self::Typography
        } else if n.starts_with("layout") {
            Self::Layout
        } else if n.starts_with("elevation") {
            Self::Elevation
        } else if n.starts_with("shapes") || n.starts_with("shape") {
            Self::Shapes
        } else if n.starts_with("components") || n.starts_with("component") {
            Self::Components
        } else if n.contains("do") && n.contains("don") {
            Self::DosAndDonts
        } else {
            Self::Unknown
        }
    }

    /// Canonical ordering per the Google Labs spec.
    pub fn order(self) -> u8 {
        use SectionKind::*;
        match self {
            Overview => 0,
            Colors => 1,
            Typography => 2,
            Layout => 3,
            Elevation => 4,
            Shapes => 5,
            Components => 6,
            DosAndDonts => 7,
            Unknown => 99,
        }
    }
}

/// Parse a DESIGN.md string. Splits frontmatter from body and walks the body
/// for top-level (`#`/`##`) section headings.
pub fn parse(src: &str) -> Result<DesignDocument, DesignError> {
    let (fm_yaml, body) = split_frontmatter(src).ok_or(DesignError::FrontmatterMissing)?;
    let frontmatter: Frontmatter = serde_yaml::from_str(fm_yaml)?;
    let sections = scan_sections(body);
    Ok(DesignDocument {
        frontmatter,
        body: body.to_string(),
        sections,
    })
}

/// Find the byte indices of every line that is exactly `---` (possibly with
/// trailing whitespace) in `s`. A line is identified by being preceded by
/// either start-of-string or `\n` and followed by `\n` or end-of-string.
fn find_marker_lines(s: &str) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let line_start = i;
        // Find end of line.
        let mut j = i;
        while j < bytes.len() && bytes[j] != b'\n' {
            j += 1;
        }
        let line = &s[line_start..j];
        let trimmed = line.trim();
        if trimmed == "---" {
            // Report (start_of_line, end_of_line_including_newline).
            let next = if j < bytes.len() { j + 1 } else { j };
            out.push((line_start, next));
        }
        i = if j < bytes.len() { j + 1 } else { j };
    }
    out
}

fn split_frontmatter(src: &str) -> Option<(&str, &str)> {
    // Find every `^---\s*$` line. Three cases:
    //   - 2+ markers: frontmatter = between marker[0].end and marker[1].start,
    //     body = after marker[1].end
    //   - 1 marker only and it's at the very top: frontmatter = after marker,
    //     body = empty (rare)
    //   - 1 marker not at top: frontmatter = between marker.end and next `# `
    //     heading; treats the prose preceding it as an instructional preamble
    //   - 0 markers: if first non-empty line looks YAML, treat whole thing
    //     as frontmatter up to first `\n# ` heading; else give up
    let markers = find_marker_lines(src);
    if markers.len() >= 2 {
        let (_, fm_start) = markers[0];
        let (fm_end, body_start) = markers[1];
        return Some((&src[fm_start..fm_end], &src[body_start..]));
    }
    if markers.len() == 1 {
        let (line_start, fm_start) = markers[0];
        if src[..line_start].trim().is_empty() {
            // Marker at top, no closing. Take everything after.
            let after = &src[fm_start..];
            let body_start = after.find("\n# ").unwrap_or(after.len());
            return Some((&after[..body_start], &after[body_start..]));
        }
        // Marker after some prose. Frontmatter is the YAML after it.
        let after = &src[fm_start..];
        let body_start = after.find("\n# ").unwrap_or(after.len());
        return Some((&after[..body_start], &after[body_start..]));
    }
    // No markers at all. Try markerless-YAML fallback.
    let trimmed = src.trim_start();
    let first_line = trimmed.lines().next().unwrap_or("");
    let looks_like_yaml = first_line.contains(':')
        && !first_line.starts_with('#')
        && !first_line.starts_with('<');
    if !looks_like_yaml {
        return None;
    }
    let body_start = trimmed.find("\n# ").unwrap_or(trimmed.len());
    Some((&trimmed[..body_start], &trimmed[body_start..]))
}

fn split_frontmatter_legacy(src: &str) -> Option<(&str, &str)> {
    // Kept for reference; not used.
    let trimmed = src.trim_start();
    let stripped = trimmed.strip_prefix("---\n")?;
    let idx = stripped.find("\n---")?;
    let fm = &stripped[..idx];
    let after = &stripped[idx + 4..];
    let body = after.strip_prefix('\n').or_else(|| after.strip_prefix("\r\n")).unwrap_or(after);
    Some((fm, body))
}

fn scan_sections(body: &str) -> Vec<Section> {
    let mut out = Vec::new();
    let mut current: Option<(SectionKind, String, Vec<&str>)> = None;
    for line in body.lines() {
        let heading = line
            .strip_prefix("## ")
            .or_else(|| line.strip_prefix("# "));
        if let Some(h) = heading {
            if let Some((kind, heading, buf)) = current.take() {
                out.push(Section {
                    kind,
                    heading,
                    markdown: buf.join("\n"),
                });
            }
            current = Some((SectionKind::from_heading(h), h.trim().to_string(), Vec::new()));
        } else if let Some((_, _, buf)) = current.as_mut() {
            buf.push(line);
        }
    }
    if let Some((kind, heading, buf)) = current.take() {
        out.push(Section {
            kind,
            heading,
            markdown: buf.join("\n"),
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parses_minimal() {
        let src = "---\nname: Sample\nversion: \"0.1\"\ncolors:\n  primary:\n    \"500\": \"#3366ff\"\n---\n\n# Overview\n\nBody text.\n\n# Colors\n\nPrimary is blue.\n";
        let d = parse(src).unwrap();
        assert_eq!(d.frontmatter.name, "Sample");
        assert_eq!(d.sections.len(), 2);
        assert_eq!(d.sections[0].kind, SectionKind::Overview);
        assert_eq!(d.sections[1].kind, SectionKind::Colors);
    }

    #[test]
    fn rejects_missing_frontmatter() {
        let src = "# Overview\n\nNothing";
        assert!(matches!(parse(src), Err(DesignError::FrontmatterMissing)));
    }
}
