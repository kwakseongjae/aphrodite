//! aphrodite-generator — orchestrates LLM provider + hero HTML rendering.
//!
//! Pipeline:
//!   1. Compose system prompt (DESIGN.md schema + skill prompt + taste hints).
//!   2. Call provider (Anthropic v0.1) OR fall back to the offline generator.
//!   3. Parse returned DESIGN.md.
//!   4. Resolve variants (light/dark/brand-A/brand-B).
//!   5. Render hero HTML against the resolved variants.

pub mod hero;
pub mod offline;
pub mod provider;
pub mod skill;

use aphrodite_core::{parse_design, resolve_variants, DesignDocument, Invocation, Variant};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationOutput {
    pub design_md: String,
    pub design_doc: DesignDocument,
    pub variants: Vec<Variant>,
    pub hero_html: String,
    pub provider_used: String,
}

#[derive(Debug, thiserror::Error)]
pub enum GenError {
    #[error("provider: {0}")]
    Provider(#[from] provider::ProviderError),
    #[error("design parse: {0}")]
    Design(#[from] aphrodite_core::DesignError),
    #[error("hero render: {0}")]
    Hero(String),
}

/// End-to-end generation. Picks the provider strategy based on what's
/// available (Anthropic key in keychain → Anthropic; else offline).
pub async fn generate(
    invocation: &Invocation,
    anthropic_key: Option<String>,
) -> Result<GenerationOutput, GenError> {
    let (design_md, provider_used) = match anthropic_key {
        Some(key) if !key.trim().is_empty() => {
            let md = provider::call_anthropic(&key, &invocation.intent).await?;
            (md, "anthropic".to_string())
        }
        _ => {
            let md = offline::generate(&invocation.intent);
            (md, "offline".to_string())
        }
    };

    let design_doc = parse_design(&design_md)?;
    let variants = resolve_variants(&design_doc);
    let hero_html = hero::render(&design_doc, &variants).map_err(GenError::Hero)?;

    Ok(GenerationOutput {
        design_md,
        design_doc,
        variants,
        hero_html,
        provider_used,
    })
}
