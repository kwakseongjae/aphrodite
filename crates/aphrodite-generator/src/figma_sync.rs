//! v1.0 Figma Variables sync. Two flows:
//!
//!   1. **Export** — emit `tokens.figma.json` in [Tokens Studio]
//!      format. Designers import this with the free Tokens Studio
//!      plugin (https://tokens.studio) and get one Figma Variable per
//!      Aphrodite token, per variant. Zero API keys, zero plan
//!      requirements. This is the realistic adoption path because
//!      Figma's REST Variables-WRITE endpoint requires the Enterprise
//!      plan, which most teams do not have.
//!
//!   2. **Pull** — if the user supplies `APHRODITE_FIGMA_TOKEN` and a
//!      file key, fetch Figma's local-variables JSON and diff it
//!      against the current resolved DESIGN.md. Surfaces which tokens
//!      diverged in Figma so the user can decide whether to bring the
//!      design file back into sync.
//!
//! Tokens Studio JSON shape (one set per variant):
//!
//! ```json
//! {
//!   "$metadata": { "tokenSetOrder": ["light", "dark", "brand-a", "brand-b"] },
//!   "$themes": [
//!     { "id": "light", "name": "Light", "selectedTokenSets": { "light": "enabled" } },
//!     ...
//!   ],
//!   "light": {
//!     "colors": { "primary": { "500": { "value": "#16a34a", "type": "color" } } }
//!   }
//! }
//! ```

use aphrodite_core::variant::Variant;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize)]
pub struct FigmaDiff {
    pub matched: Vec<String>,
    pub only_in_design: Vec<String>,
    pub only_in_figma: Vec<String>,
    pub value_mismatches: Vec<ValueMismatch>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValueMismatch {
    pub key: String,
    pub design_value: String,
    pub figma_value: String,
}

/// Build a Tokens Studio-compatible JSON document from the resolved
/// variants. Designers paste this into the Tokens Studio plugin
/// (Settings → Sync → JSON paste) and Figma creates one Variable per
/// token, scoped to the matching mode/collection.
pub fn build_tokens_studio_json(variants: &[Variant]) -> String {
    let mut root = serde_json::Map::new();

    // $metadata + $themes — the Tokens Studio sync needs both for the
    // multi-set flow to work cleanly.
    let order: Vec<serde_json::Value> = variants
        .iter()
        .map(|v| serde_json::Value::String(v.kind.label()))
        .collect();
    let mut meta = serde_json::Map::new();
    meta.insert("tokenSetOrder".to_string(), serde_json::Value::Array(order.clone()));
    root.insert("$metadata".to_string(), serde_json::Value::Object(meta));

    let themes: Vec<serde_json::Value> = variants
        .iter()
        .map(|v| {
            let label = v.kind.label();
            let mut sel = serde_json::Map::new();
            sel.insert(label.clone(), serde_json::Value::String("enabled".into()));
            let mut theme = serde_json::Map::new();
            theme.insert("id".to_string(), serde_json::Value::String(label.clone()));
            theme.insert(
                "name".to_string(),
                serde_json::Value::String(title_case(&label)),
            );
            theme.insert("selectedTokenSets".to_string(), serde_json::Value::Object(sel));
            serde_json::Value::Object(theme)
        })
        .collect();
    root.insert("$themes".to_string(), serde_json::Value::Array(themes));

    // One nested set per variant — same shape as tokens.json but
    // without an outer wrapping object.
    for v in variants {
        let mut node = serde_json::Map::new();
        for (k, val) in &v.tokens {
            insert_nested(&mut node, k, val);
        }
        root.insert(v.kind.label(), serde_json::Value::Object(node));
    }

    serde_json::to_string_pretty(&serde_json::Value::Object(root)).unwrap_or_default()
}

fn insert_nested(node: &mut serde_json::Map<String, serde_json::Value>, key: &str, value: &str) {
    let parts: Vec<&str> = key.split('.').collect();
    if parts.is_empty() {
        return;
    }
    let (last, rest) = parts.split_last().unwrap();
    let mut cur = node;
    for p in rest {
        let entry = cur
            .entry(p.to_string())
            .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));
        if !entry.is_object() {
            *entry = serde_json::Value::Object(serde_json::Map::new());
        }
        cur = entry.as_object_mut().unwrap();
    }
    cur.insert(
        last.to_string(),
        serde_json::Value::Object(
            [
                ("value".to_string(), serde_json::Value::String(value.to_string())),
                ("type".to_string(), serde_json::Value::String(infer_type(value))),
            ]
            .into_iter()
            .collect(),
        ),
    );
}

fn infer_type(value: &str) -> String {
    let v = value.trim();
    if v.starts_with('#') || v.starts_with("rgb") || v.starts_with("hsl") {
        "color".into()
    } else if v.ends_with("px") || v.ends_with("rem") || v.ends_with("em") {
        "dimension".into()
    } else if v.ends_with("ms") || v.ends_with("s") {
        "duration".into()
    } else {
        "string".into()
    }
}

fn title_case(s: &str) -> String {
    let mut out = String::new();
    for (i, word) in s.split('-').enumerate() {
        if i > 0 {
            out.push(' ');
        }
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            out.extend(first.to_uppercase());
            out.extend(chars);
        }
    }
    out
}

// ---- Figma REST pull + diff -----------------------------------------

#[derive(Debug, Deserialize)]
struct FigmaVariablesResponse {
    meta: FigmaMeta,
}

#[derive(Debug, Deserialize)]
struct FigmaMeta {
    variables: BTreeMap<String, FigmaVariable>,
    #[serde(rename = "variableCollections")]
    variable_collections: BTreeMap<String, FigmaCollection>,
}

#[derive(Debug, Deserialize)]
struct FigmaVariable {
    name: String,
    #[serde(rename = "resolvedType")]
    resolved_type: String,
    #[serde(rename = "valuesByMode")]
    values_by_mode: BTreeMap<String, serde_json::Value>,
    #[serde(rename = "variableCollectionId")]
    variable_collection_id: String,
}

#[derive(Debug, Deserialize)]
struct FigmaCollection {
    name: String,
    modes: Vec<FigmaMode>,
}

#[derive(Debug, Deserialize)]
struct FigmaMode {
    #[serde(rename = "modeId")]
    mode_id: String,
    name: String,
}

/// Fetch a Figma file's local variables. Requires:
/// - `APHRODITE_FIGMA_TOKEN` env var (or `FIGMA_TOKEN`).
/// - The file_key from the Figma URL: `figma.com/file/<KEY>/...`.
///
/// Read-only — works on Pro/Org plans (write would need Enterprise).
pub async fn pull_figma_variables(
    file_key: &str,
) -> Result<BTreeMap<String, BTreeMap<String, String>>, FigmaError> {
    let token = std::env::var("APHRODITE_FIGMA_TOKEN")
        .or_else(|_| std::env::var("FIGMA_TOKEN"))
        .map_err(|_| FigmaError::MissingToken)?;
    let url = format!("https://api.figma.com/v1/files/{file_key}/variables/local");
    let resp = reqwest::Client::new()
        .get(&url)
        .header("X-Figma-Token", token)
        .send()
        .await
        .map_err(FigmaError::Http)?;
    if !resp.status().is_success() {
        return Err(FigmaError::ApiStatus(resp.status().as_u16()));
    }
    let body: FigmaVariablesResponse = resp.json().await.map_err(FigmaError::Http)?;

    // Collapse to { mode_label -> { dotted_key -> value_string } }.
    let mut out: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    for v in body.meta.variables.values() {
        let coll = match body.meta.variable_collections.get(&v.variable_collection_id) {
            Some(c) => c,
            None => continue,
        };
        for mode in &coll.modes {
            let raw = match v.values_by_mode.get(&mode.mode_id) {
                Some(r) => r,
                None => continue,
            };
            let key = format!("{}.{}", coll.name.to_ascii_lowercase(), v.name);
            let value = match v.resolved_type.as_str() {
                "COLOR" => figma_color_to_hex(raw).unwrap_or_default(),
                _ => raw.as_str().map(String::from).unwrap_or_else(|| raw.to_string()),
            };
            out.entry(mode.name.to_ascii_lowercase())
                .or_default()
                .insert(key, value);
        }
    }
    Ok(out)
}

fn figma_color_to_hex(v: &serde_json::Value) -> Option<String> {
    let obj = v.as_object()?;
    let r = (obj.get("r")?.as_f64()? * 255.0).round() as u32;
    let g = (obj.get("g")?.as_f64()? * 255.0).round() as u32;
    let b = (obj.get("b")?.as_f64()? * 255.0).round() as u32;
    Some(format!("#{r:02x}{g:02x}{b:02x}"))
}

/// Diff a fetched Figma variables map against the resolved DESIGN.md
/// variants. Returns matched / only-in-design / only-in-figma /
/// value-mismatches.
pub fn diff(variants: &[Variant], figma: &BTreeMap<String, BTreeMap<String, String>>) -> FigmaDiff {
    let mut matched: Vec<String> = Vec::new();
    let mut only_in_design: Vec<String> = Vec::new();
    let mut only_in_figma: Vec<String> = Vec::new();
    let mut value_mismatches: Vec<ValueMismatch> = Vec::new();

    for v in variants {
        let mode = v.kind.label().to_ascii_lowercase();
        let figma_set = figma.get(&mode);
        for (k, val) in &v.tokens {
            let qual = format!("{mode}.{k}");
            match figma_set.and_then(|m| m.get(k)) {
                Some(fv) => {
                    if normalize(fv) == normalize(val) {
                        matched.push(qual);
                    } else {
                        value_mismatches.push(ValueMismatch {
                            key: qual,
                            design_value: val.clone(),
                            figma_value: fv.clone(),
                        });
                    }
                }
                None => only_in_design.push(qual),
            }
        }
    }
    // Tokens that exist in Figma but not in our design.
    for (mode, set) in figma {
        for k in set.keys() {
            let qual = format!("{mode}.{k}");
            let in_design = variants
                .iter()
                .filter(|v| v.kind.label().to_ascii_lowercase() == *mode)
                .any(|v| v.tokens.contains_key(k));
            if !in_design {
                only_in_figma.push(qual);
            }
        }
    }
    FigmaDiff {
        matched,
        only_in_design,
        only_in_figma,
        value_mismatches,
    }
}

fn normalize(v: &str) -> String {
    v.trim().to_ascii_lowercase()
}

#[derive(Debug, thiserror::Error)]
pub enum FigmaError {
    #[error("APHRODITE_FIGMA_TOKEN (or FIGMA_TOKEN) env var not set")]
    MissingToken,
    #[error("Figma API returned HTTP {0}")]
    ApiStatus(u16),
    #[error("HTTP error: {0}")]
    Http(reqwest::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use aphrodite_core::variant::{Variant, VariantKind};

    fn fixture() -> Vec<Variant> {
        let mut t = BTreeMap::new();
        t.insert("colors.primary.500".into(), "#16a34a".into());
        t.insert("spacing.4".into(), "16px".into());
        vec![
            Variant { kind: VariantKind::Light, tokens: t.clone() },
            Variant { kind: VariantKind::Dark, tokens: t },
        ]
    }

    #[test]
    fn tokens_studio_json_has_metadata_themes_and_sets() {
        let s = build_tokens_studio_json(&fixture());
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert!(v["$metadata"]["tokenSetOrder"].is_array());
        assert!(v["$themes"].is_array());
        // Each variant is its own top-level set, with nested tokens.
        assert_eq!(v["light"]["colors"]["primary"]["500"]["value"], "#16a34a");
        assert_eq!(v["light"]["colors"]["primary"]["500"]["type"], "color");
        assert_eq!(v["light"]["spacing"]["4"]["type"], "dimension");
    }

    #[test]
    fn diff_finds_matches_and_mismatches() {
        let variants = fixture();
        let mut figma_light: BTreeMap<String, String> = BTreeMap::new();
        figma_light.insert("colors.primary.500".into(), "#16a34a".into()); // matched
        figma_light.insert("colors.primary.500.alpha".into(), "#ff0000".into()); // only_in_figma
        let mut figma_dark: BTreeMap<String, String> = BTreeMap::new();
        figma_dark.insert("colors.primary.500".into(), "#ff0000".into()); // mismatched value
        figma_dark.insert("spacing.4".into(), "16px".into()); // matched
        let mut figma = BTreeMap::new();
        figma.insert("light".into(), figma_light);
        figma.insert("dark".into(), figma_dark);
        let d = diff(&variants, &figma);
        assert!(d.matched.iter().any(|k| k == "light.colors.primary.500"));
        assert!(d.value_mismatches.iter().any(|m| m.key == "dark.colors.primary.500" && m.design_value == "#16a34a" && m.figma_value == "#ff0000"));
        assert!(d.only_in_design.iter().any(|k| k == "light.spacing.4"));
        assert!(d.only_in_figma.iter().any(|k| k == "light.colors.primary.500.alpha"));
    }

    #[test]
    fn figma_color_normalises_to_lowercase_hex() {
        let v: serde_json::Value = serde_json::from_str(r#"{"r":0.086,"g":0.639,"b":0.290}"#).unwrap();
        let hex = figma_color_to_hex(&v).unwrap();
        assert_eq!(hex.len(), 7);
        assert!(hex.starts_with('#'));
    }
}
