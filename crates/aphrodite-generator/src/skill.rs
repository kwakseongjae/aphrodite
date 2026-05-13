//! Skill packs — markdown-described "looks". Format mirrors Anthropic's
//! SKILL.md convention so any harness can reuse our skills and vice versa.

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillPack {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub triggers: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub prompt_fragments: Vec<String>,
}

pub fn load_dir(dir: &Path) -> std::io::Result<Vec<SkillPack>> {
    let mut out = Vec::new();
    if !dir.exists() {
        return Ok(out);
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let skill_file = entry.path().join("SKILL.md");
            if skill_file.exists() {
                if let Ok(pack) = parse_skill_file(&skill_file) {
                    out.push(pack);
                }
            }
        }
    }
    Ok(out)
}

fn parse_skill_file(path: &Path) -> anyhow::Result<SkillPack> {
    let src = std::fs::read_to_string(path)?;
    let (fm, body) = src
        .strip_prefix("---\n")
        .and_then(|s| s.split_once("\n---\n"))
        .ok_or_else(|| anyhow::anyhow!("SKILL.md missing frontmatter at {}", path.display()))?;
    let mut pack: SkillPack = serde_yaml::from_str(fm)?;
    if pack.prompt_fragments.is_empty() {
        pack.prompt_fragments = vec![body.trim().to_string()];
    }
    Ok(pack)
}
