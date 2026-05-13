//! aphrodite-generator — orchestrates LLM provider + hero HTML rendering.
//!
//! Pipeline:
//!   1. Resolve a provider: keyring lookup in priority order (z.ai → Anthropic
//!      → OpenRouter), or use the caller-supplied `ResolvedProvider`.
//!   2. Call the provider OR fall back to the offline generator.
//!   3. Parse returned DESIGN.md.
//!   4. Resolve variants (light/dark/brand-A/brand-B).
//!   5. Render hero HTML against the resolved variants.

pub mod hero;
pub mod offline;
pub mod provider;
pub mod skill;

use aphrodite_core::{parse_design, resolve_variants, DesignDocument, Invocation, Variant};
use provider::{ProviderId, ResolvedProvider, DEFAULT_PRIORITY};
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

/// End-to-end generation. Walks the default provider priority looking for a
/// usable credential; falls back to the offline generator if nothing is
/// configured.
pub async fn generate(invocation: &Invocation) -> Result<GenerationOutput, GenError> {
    let resolved = resolve_default_provider();
    generate_with(invocation, resolved.as_ref()).await
}

/// Generate against an explicit provider (or `None` for offline).
pub async fn generate_with(
    invocation: &Invocation,
    resolved: Option<&ResolvedProvider>,
) -> Result<GenerationOutput, GenError> {
    let (design_md, provider_used) = match resolved {
        Some(r) => {
            let md = provider::call(r, &invocation.intent).await?;
            (md, r.id.label().to_string())
        }
        None => (offline::generate(&invocation.intent), "offline".to_string()),
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

/// Walk the keyring priority order and return the first provider with a
/// stored credential. Also honors env-var fallbacks for headless / CI:
///   - `APHRODITE_ZAI_API_KEY`         → z.ai
///   - `APHRODITE_ANTHROPIC_API_KEY`   → Anthropic
///   - `ANTHROPIC_API_KEY`             → Anthropic (compat with native env)
///   - `APHRODITE_OPENROUTER_API_KEY`  → OpenRouter
///   - `OPENROUTER_API_KEY`            → OpenRouter (compat)
pub fn resolve_default_provider() -> Option<ResolvedProvider> {
    for id in DEFAULT_PRIORITY.iter().copied() {
        if let Some(key) = fetch_key(id) {
            return Some(ResolvedProvider::with_default_model(id, key));
        }
    }
    None
}

fn fetch_key(id: ProviderId) -> Option<String> {
    if let Ok(k) = aphrodite_keyring::fetch(id.keyring_id()) {
        if !k.trim().is_empty() {
            return Some(k);
        }
    }
    let env_names: &[&str] = match id {
        ProviderId::Zai => &["APHRODITE_ZAI_API_KEY", "ZAI_API_KEY", "GLM_API_KEY"],
        ProviderId::Anthropic => &["APHRODITE_ANTHROPIC_API_KEY", "ANTHROPIC_API_KEY"],
        ProviderId::Openrouter => &["APHRODITE_OPENROUTER_API_KEY", "OPENROUTER_API_KEY"],
        ProviderId::Openai => &["APHRODITE_OPENAI_API_KEY", "OPENAI_API_KEY"],
        ProviderId::Moonshot => &["APHRODITE_MOONSHOT_API_KEY", "MOONSHOT_API_KEY"],
        ProviderId::Gemini => &["APHRODITE_GEMINI_API_KEY", "GEMINI_API_KEY"],
    };
    for n in env_names {
        if let Ok(k) = std::env::var(n) {
            if !k.trim().is_empty() {
                return Some(k);
            }
        }
    }
    None
}
