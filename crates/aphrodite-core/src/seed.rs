//! Seed reader. Aphrodite is the *consumer* of Ouroboros `seed.yaml`
//! (ADR-0001). We never generate one ourselves; we never modify the file.

use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SeedError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse: {0}")]
    Parse(#[from] serde_yaml::Error),
    #[error("unsupported seed schema version: {0}")]
    UnsupportedVersion(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seed {
    pub goal: String,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub acceptance_criteria: Vec<String>,
    #[serde(default)]
    pub ontology_schema: serde_yaml::Value,
    #[serde(default)]
    pub evaluation_principles: serde_yaml::Value,
    #[serde(default)]
    pub exit_conditions: serde_yaml::Value,
    #[serde(default)]
    pub metadata: SeedMetadata,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SeedMetadata {
    #[serde(default)]
    pub seed_id: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
}

/// Load `seed.yaml` from disk. Refuses to operate on schema versions outside
/// the supported set.
pub fn load(path: &Path) -> Result<Seed, SeedError> {
    let bytes = std::fs::read(path)?;
    let seed: Seed = serde_yaml::from_slice(&bytes)?;
    if let Some(v) = seed.metadata.version.as_deref() {
        if v != "0.1.0-seed" {
            return Err(SeedError::UnsupportedVersion(v.into()));
        }
    }
    Ok(seed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn loads_minimal_seed() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("aphrodite_seed_test_{}.yaml", std::process::id()));
        std::fs::write(
            &path,
            indoc! {r#"
                goal: test
                constraints: []
                acceptance_criteria: []
                metadata:
                  seed_id: t1
                  version: "0.1.0-seed"
            "#},
        )
        .unwrap();
        let s = load(&path).unwrap();
        assert_eq!(s.goal, "test");
        let _ = std::fs::remove_file(&path);
    }
}
