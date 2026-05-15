//! Persona system — opinionated design authorities that override defaults.
//!
//! A persona is structurally similar to a skill (markdown + YAML frontmatter
//! on disk) but semantically distinct: it carries the *voice* and the
//! *non-negotiable principles* of a recognised design authority. When
//! invoked with `--persona <slug>`, the persona body is injected into the
//! design + critic system prompts as the final authority — outranking
//! generic skill scaffolds when they conflict.
//!
//! Layout:
//!   ~/.aphrodite/personas/<slug>/PERSONA.md
//!   _archive/ for retired personas
//!
//! Personas ship as bundled seeds (include_str! at compile time) and
//! materialise on first use.

use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const PERSONA_FILENAME: &str = "PERSONA.md";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaFrontmatter {
    pub name: String,
    #[serde(default)]
    pub era: String,
    #[serde(default)]
    pub voice: String,
    #[serde(default)]
    pub principles: Vec<String>,
    #[serde(default)]
    pub rejects: Vec<String>,
    #[serde(default)]
    pub prefers: Vec<String>,
    #[serde(default)]
    pub when_to_invoke: String,
    /// Optional per-persona note on how to handle CJK (Korean / Japanese /
    /// Chinese) text-bearing surfaces. Each persona's preferred Latin display
    /// family rarely covers CJK glyphs; this field lets the persona speak
    /// to the override in its own voice. Empty ⇒ generic fallback.
    #[serde(default)]
    pub cjk_strategy: String,
}

#[derive(Debug, Clone)]
pub struct Persona {
    pub slug: String,
    pub frontmatter: PersonaFrontmatter,
    pub body: String,
    pub path: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum PersonaError {
    #[error("persona not found: {0}")]
    NotFound(String),
    #[error("persona io: {0}")]
    Io(#[from] io::Error),
    #[error("persona format: {0}")]
    Format(String),
    #[error("persona missing frontmatter")]
    NoFrontmatter,
}

// ---------------------------------------------------------------------------
// Paths
// ---------------------------------------------------------------------------

pub fn personas_root() -> PathBuf {
    let home = std::env::var_os("HOME").unwrap_or_default();
    let mut p = PathBuf::from(home);
    p.push(".aphrodite");
    p.push("personas");
    let _ = fs::create_dir_all(&p);
    p
}

pub fn persona_dir(slug: &str) -> PathBuf {
    let mut p = personas_root();
    p.push(slug);
    p
}

pub fn persona_md_path(slug: &str) -> PathBuf {
    let mut p = persona_dir(slug);
    p.push(PERSONA_FILENAME);
    p
}

// ---------------------------------------------------------------------------
// Parse / load / save
// ---------------------------------------------------------------------------

pub fn parse_persona(slug: &str, text: &str, path: PathBuf) -> Result<Persona, PersonaError> {
    let trimmed = text.trim_start();
    if !trimmed.starts_with("---") {
        return Err(PersonaError::NoFrontmatter);
    }
    let after = &trimmed[3..];
    let end = after.find("\n---").ok_or(PersonaError::NoFrontmatter)?;
    let fm_text = &after[..end];
    let body = after[end + 4..].trim_start_matches('\n').to_string();
    let frontmatter: PersonaFrontmatter =
        serde_yaml::from_str(fm_text).map_err(|e| PersonaError::Format(e.to_string()))?;
    Ok(Persona { slug: slug.to_string(), frontmatter, body, path })
}

pub fn load(slug: &str) -> Result<Persona, PersonaError> {
    let path = persona_md_path(slug);
    if !path.exists() {
        return Err(PersonaError::NotFound(slug.to_string()));
    }
    let text = fs::read_to_string(&path)?;
    parse_persona(slug, &text, path)
}

pub fn save(p: &Persona) -> Result<(), PersonaError> {
    let dir = persona_dir(&p.slug);
    fs::create_dir_all(&dir)?;
    let fm =
        serde_yaml::to_string(&p.frontmatter).map_err(|e| PersonaError::Format(e.to_string()))?;
    let text = format!("---\n{}---\n\n{}", fm, p.body.trim_end());
    fs::write(persona_md_path(&p.slug), text)?;
    Ok(())
}

pub fn list() -> Vec<String> {
    let mut out = Vec::new();
    if let Ok(entries) = fs::read_dir(personas_root()) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                if name == "_archive" {
                    continue;
                }
                if path.join(PERSONA_FILENAME).exists() {
                    out.push(name.to_string());
                }
            }
        }
    }
    out.sort();
    out
}

// ---------------------------------------------------------------------------
// Bundled seeds
// ---------------------------------------------------------------------------

const BUNDLED_PERSONAS: &[(&str, &str)] = &[
    ("dieter-rams", include_str!("../seed-personas/dieter-rams/PERSONA.md")),
    (
        "massimo-vignelli",
        include_str!("../seed-personas/massimo-vignelli/PERSONA.md"),
    ),
    ("tadao-ando", include_str!("../seed-personas/tadao-ando/PERSONA.md")),
    ("rei-kawakubo", include_str!("../seed-personas/rei-kawakubo/PERSONA.md")),
    (
        "ettore-sottsass",
        include_str!("../seed-personas/ettore-sottsass/PERSONA.md"),
    ),
    ("kenya-hara", include_str!("../seed-personas/kenya-hara/PERSONA.md")),
    (
        "galileo-galilei",
        include_str!("../seed-personas/galileo-galilei/PERSONA.md"),
    ),
    ("paul-rand", include_str!("../seed-personas/paul-rand/PERSONA.md")),
    (
        "charlotte-perriand",
        include_str!("../seed-personas/charlotte-perriand/PERSONA.md"),
    ),
    (
        "naoto-fukasawa",
        include_str!("../seed-personas/naoto-fukasawa/PERSONA.md"),
    ),
];

pub fn seed_bundled_personas() -> Vec<String> {
    let mut newly = Vec::new();
    for (slug, contents) in BUNDLED_PERSONAS {
        let target = persona_md_path(slug);
        if target.exists() {
            continue;
        }
        if let Some(parent) = target.parent() {
            if fs::create_dir_all(parent).is_err() {
                continue;
            }
        }
        if fs::write(&target, contents).is_ok() {
            newly.push((*slug).to_string());
        }
    }
    newly
}

pub fn all_bundled_slugs() -> Vec<&'static str> {
    BUNDLED_PERSONAS.iter().map(|(s, _)| *s).collect()
}

// ---------------------------------------------------------------------------
// Prompt rendering
// ---------------------------------------------------------------------------

/// Render the persona as a system-prompt block ready to prepend.
pub fn as_system_prompt_block(p: &Persona) -> String {
    let mut out = String::new();
    out.push_str("## Persona authority\n\n");
    out.push_str(&format!(
        "You are designing as **{}** would design ({}). Voice: {}\n\n",
        p.frontmatter.name,
        if p.frontmatter.era.is_empty() { "—" } else { &p.frontmatter.era },
        if p.frontmatter.voice.is_empty() { "—" } else { &p.frontmatter.voice }
    ));
    if !p.frontmatter.principles.is_empty() {
        out.push_str("Principles you hold:\n");
        for pr in &p.frontmatter.principles {
            out.push_str(&format!("  - {pr}\n"));
        }
        out.push('\n');
    }
    if !p.frontmatter.rejects.is_empty() {
        out.push_str("What you refuse to produce:\n");
        for r in &p.frontmatter.rejects {
            out.push_str(&format!("  - {r}\n"));
        }
        out.push('\n');
    }
    if !p.frontmatter.prefers.is_empty() {
        out.push_str("What you champion:\n");
        for pf in &p.frontmatter.prefers {
            out.push_str(&format!("  - {pf}\n"));
        }
        out.push('\n');
    }
    if !p.frontmatter.cjk_strategy.is_empty() {
        out.push_str("Strategy for CJK (Korean / Japanese / Chinese) text-bearing surfaces:\n");
        out.push_str(&p.frontmatter.cjk_strategy);
        out.push_str("\n\n");
    }
    if !p.body.trim().is_empty() {
        out.push_str("Additional notes from your own writing / practice:\n\n");
        out.push_str(p.body.trim());
        out.push_str("\n\n");
    }
    out.push_str(
        "Apply these as hard constraints. When persona principles conflict with generic skill scaffolds, the persona wins — but call out the conflict in your rationale.\n",
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Scratch {
        _td: tempfile::TempDir,
        _guard: std::sync::MutexGuard<'static, ()>,
    }
    impl Scratch {
        fn new() -> Self {
            let guard = crate::test_lock::GLOBAL.lock().unwrap_or_else(|e| e.into_inner());
            let td = tempfile::tempdir().expect("tempdir");
            unsafe { std::env::set_var("HOME", td.path()); }
            Self { _td: td, _guard: guard }
        }
    }

    #[test]
    fn seed_bundled_personas_materialises_seven() {
        let _s = Scratch::new();
        let newly = seed_bundled_personas();
        assert_eq!(newly.len(), 10, "expected 10 seeded personas, got {newly:?}");
        for slug in &[
            "dieter-rams", "massimo-vignelli", "tadao-ando",
            "rei-kawakubo", "ettore-sottsass", "kenya-hara", "galileo-galilei",
            "paul-rand", "charlotte-perriand", "naoto-fukasawa",
        ] {
            let p = load(slug).unwrap_or_else(|e| panic!("load {slug}: {e}"));
            assert!(!p.frontmatter.name.is_empty(), "{slug} missing name");
            assert!(
                !p.frontmatter.principles.is_empty(),
                "{slug} should declare principles"
            );
        }
        let again = seed_bundled_personas();
        assert!(again.is_empty(), "second seed should be no-op");
    }

    #[test]
    fn system_prompt_block_contains_voice_and_principles() {
        let _s = Scratch::new();
        seed_bundled_personas();
        let rams = load("dieter-rams").unwrap();
        let block = as_system_prompt_block(&rams);
        assert!(block.contains("Dieter Rams"));
        assert!(block.contains("Principles you hold"));
        assert!(block.contains("Apply these as hard constraints"));
    }
}
