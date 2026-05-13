//! Deny-list policy: the one gate around the seed's "full write" default.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Policy {
    #[serde(default)]
    pub deny: Deny,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Deny {
    #[serde(default)]
    pub fs: Vec<String>,
    #[serde(default)]
    pub shell: Vec<String>,
    #[serde(default)]
    pub net: Vec<String>,
}

pub fn ship_defaults() -> Policy {
    Policy {
        deny: Deny {
            fs: vec![
                "../**".into(),
                "~/.ssh/**".into(),
                "**/.git/hooks/**".into(),
            ],
            shell: vec![
                "rm -rf /".into(),
                "git push --force* origin main".into(),
                "git push --force* origin master".into(),
            ],
            net: vec![],
        },
    }
}

pub fn load(project_root: &Path) -> Policy {
    let p: PathBuf = project_root.join(".aphrodite").join("policy.yaml");
    if let Ok(s) = std::fs::read_to_string(&p) {
        if let Ok(parsed) = serde_yaml::from_str::<Policy>(&s) {
            return merge(ship_defaults(), parsed);
        }
    }
    ship_defaults()
}

fn merge(mut base: Policy, over: Policy) -> Policy {
    base.deny.fs.extend(over.deny.fs);
    base.deny.shell.extend(over.deny.shell);
    base.deny.net.extend(over.deny.net);
    base
}

/// Returns Err if the path violates any fs deny rule.
pub fn check_fs(policy: &Policy, project_root: &Path, target: &Path) -> Result<(), String> {
    let abs = if target.is_absolute() {
        target.to_path_buf()
    } else {
        project_root.join(target)
    };
    let s = abs.to_string_lossy().to_string();
    for rule in &policy.deny.fs {
        let r = expand_home(rule);
        if glob_match::glob_match(&r, &s) {
            return Err(format!("fs deny: `{s}` matches rule `{rule}`"));
        }
    }
    // Hard escape: never resolve above the project root.
    let proj = project_root.canonicalize().unwrap_or(project_root.to_path_buf());
    let abs = abs.canonicalize().unwrap_or(abs.clone());
    if !abs.starts_with(&proj) {
        return Err(format!("fs deny: `{}` escapes project root `{}`", abs.display(), proj.display()));
    }
    Ok(())
}

fn expand_home(rule: &str) -> String {
    if let Some(rest) = rule.strip_prefix("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            return format!("{}/{}", home.to_string_lossy(), rest);
        }
    }
    rule.to_string()
}
