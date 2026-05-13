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

fn split_frontmatter(src: &str) -> Option<(&str, &str)> {
    // Tolerate (a) leading whitespace, (b) LLM prose before the opening `---`,
    // (c) LLM omitting markers entirely when the response is YAML-only.
    let trimmed = src.trim_start();
    let stripped = if let Some(s) = trimmed.strip_prefix("---\n").or_else(|| trimmed.strip_prefix("---\r\n")) {
        s
    } else if let Some(idx) = trimmed.find("\n---\n") {
        &trimmed[idx + 5..]
    } else {
        // No `---` markers at all. If the head looks like YAML (a top-level
        // key on the first non-empty line), accept the whole thing as
        // frontmatter and find the first `# ` markdown heading as the body
        // boundary.
        let first_line = trimmed.lines().next().unwrap_or("");
        let looks_like_yaml = first_line.contains(':')
            && !first_line.starts_with('#')
            && !first_line.starts_with('<');
        if !looks_like_yaml {
            return None;
        }
        // Body starts at the first `\n# ` (markdown H1); if none, the whole
        // thing is frontmatter and body is empty.
        let body_start = trimmed.find("\n# ").unwrap_or(trimmed.len());
        return Some((&trimmed[..body_start], &trimmed[body_start..]));
    };
    // find the closing `---` on its own line
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
