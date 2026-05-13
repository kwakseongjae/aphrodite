//! Multi-mode variants. v0.1 ships light + dark + ≥2 brand variants.
//!
//! Resolution algorithm: take the base frontmatter token graph, flatten to
//! dotted keys (`colors.primary.500`), then overlay the variant's `tokens` map
//! (which is already dotted).

use crate::design::{DesignDocument, VariantOverlay};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VariantKind {
    Light,
    Dark,
    Brand(String),
}

impl VariantKind {
    fn from_name(name: &str) -> Self {
        match name.to_ascii_lowercase().as_str() {
            "light" => Self::Light,
            "dark" => Self::Dark,
            other => Self::Brand(other.to_string()),
        }
    }

    pub fn label(&self) -> String {
        match self {
            Self::Light => "light".into(),
            Self::Dark => "dark".into(),
            Self::Brand(b) => b.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    pub kind: VariantKind,
    pub tokens: BTreeMap<String, String>,
}

/// Resolve every variant declared in `metadata.variants`. If none declared,
/// returns a single synthetic "default" variant carrying the base tokens.
pub fn resolve(doc: &DesignDocument) -> Vec<Variant> {
    let base = flatten_base(doc);
    let Some(meta) = doc.frontmatter.metadata.as_ref() else {
        return vec![Variant {
            kind: VariantKind::Brand("default".into()),
            tokens: base,
        }];
    };
    if meta.variants.is_empty() {
        return vec![Variant {
            kind: VariantKind::Brand("default".into()),
            tokens: base,
        }];
    }
    meta.variants
        .iter()
        .map(|(name, overlay)| apply(name, overlay, &base))
        .collect()
}

fn apply(name: &str, overlay: &VariantOverlay, base: &BTreeMap<String, String>) -> Variant {
    let mut tokens = base.clone();
    for (k, v) in &overlay.tokens {
        tokens.insert(k.clone(), v.clone());
    }
    Variant {
        kind: VariantKind::from_name(name),
        tokens,
    }
}

fn flatten_base(doc: &DesignDocument) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    if let Some(colors) = doc.frontmatter.colors.as_ref() {
        for (shade, hex) in &colors.primary {
            out.insert(format!("colors.primary.{shade}"), hex.clone());
        }
        for (palette, shades) in &colors.others {
            for (shade, hex) in shades {
                out.insert(format!("colors.{palette}.{shade}"), hex.clone());
            }
        }
    }
    flatten_yaml(&mut out, "spacing", doc.frontmatter.spacing.as_ref());
    flatten_yaml(&mut out, "typography", doc.frontmatter.typography.as_ref());
    flatten_yaml(&mut out, "rounded", doc.frontmatter.rounded.as_ref());
    out
}

fn flatten_yaml(
    out: &mut BTreeMap<String, String>,
    prefix: &str,
    value: Option<&serde_yaml::Value>,
) {
    let Some(value) = value else { return };
    walk(out, prefix.to_string(), value);
}

fn walk(out: &mut BTreeMap<String, String>, prefix: String, value: &serde_yaml::Value) {
    match value {
        serde_yaml::Value::Mapping(map) => {
            for (k, v) in map {
                let key = match k {
                    serde_yaml::Value::String(s) => s.clone(),
                    other => format!("{other:?}"),
                };
                let next = if prefix.is_empty() {
                    key
                } else {
                    format!("{prefix}.{key}")
                };
                walk(out, next, v);
            }
        }
        serde_yaml::Value::String(s) => {
            out.insert(prefix, s.clone());
        }
        serde_yaml::Value::Number(n) => {
            out.insert(prefix, n.to_string());
        }
        serde_yaml::Value::Bool(b) => {
            out.insert(prefix, b.to_string());
        }
        _ => {}
    }
}
