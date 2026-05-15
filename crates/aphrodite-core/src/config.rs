//! User-level config at `~/.aphrodite/config.toml`. Stores *non-secret* user
//! preferences. Secrets live exclusively in the OS keychain (aphrodite-keyring).
//!
//! Shape:
//! ```toml
//! default_provider = "zai"
//!
//! [providers.zai]
//! plan = "coding_plan"          # "coding_plan" | "standard_api"
//! model = "glm-4.7"
//! base_url = "https://api.z.ai/api/anthropic"   # optional override
//!
//! [providers.anthropic]
//! plan = "standard_api"
//! model = "claude-sonnet-4-6"
//! ```

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub default_provider: Option<String>,
    #[serde(default)]
    pub providers: BTreeMap<String, ProviderConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// `coding_plan` | `standard_api` — informational, drives default base_url
    /// when no explicit `base_url` override is set.
    #[serde(default)]
    pub plan: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub base_url: Option<String>,
    /// Optional per-call model overrides. When set, the orchestrator uses
    /// these instead of `model` for the respective phase. Use a *faster*
    /// model (e.g. z.ai `glm-4.5-air`) for the composer and critic calls
    /// where reasoning depth isn't the bottleneck — and keep `model` (e.g.
    /// `glm-5.1`) for the design call where it is.
    #[serde(default)]
    pub composer_model: Option<String>,
    #[serde(default)]
    pub critic_model: Option<String>,
}

pub fn config_path() -> PathBuf {
    let home = std::env::var_os("HOME").unwrap_or_default();
    let mut p = PathBuf::from(home);
    p.push(".aphrodite");
    let _ = std::fs::create_dir_all(&p);
    p.push("config.toml");
    p
}

pub fn load() -> Config {
    let path = config_path();
    match std::fs::read_to_string(&path) {
        Ok(s) => toml::from_str(&s).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}

pub fn save(cfg: &Config) -> std::io::Result<()> {
    let path = config_path();
    let text = toml::to_string_pretty(cfg).map_err(|e| std::io::Error::other(e.to_string()))?;
    std::fs::write(&path, text)
}
