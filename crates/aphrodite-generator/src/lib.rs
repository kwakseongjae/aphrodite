//! aphrodite-generator — orchestrates LLM provider + hero HTML rendering.

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
    pub model_used: String,
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

pub async fn generate(invocation: &Invocation) -> Result<GenerationOutput, GenError> {
    let resolved = resolve_default_provider();
    generate_with(invocation, resolved.as_ref()).await
}

pub async fn generate_with(
    invocation: &Invocation,
    resolved: Option<&ResolvedProvider>,
) -> Result<GenerationOutput, GenError> {
    let (design_md, provider_used, model_used) = match resolved {
        Some(r) => {
            let md = provider::call(r, &invocation.intent).await?;
            (md, r.id.label().to_string(), r.model.clone())
        }
        None => (offline::generate(&invocation.intent), "offline".to_string(), "deterministic".to_string()),
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
        model_used,
    })
}

/// Resolve a provider. Honors `~/.aphrodite/config.toml` for default_provider
/// + per-provider model/base_url overrides. Falls back to scanning all
/// providers in priority order. Honors env-var fallbacks for headless / CI.
pub fn resolve_default_provider() -> Option<ResolvedProvider> {
    let cfg = aphrodite_core::config::load();

    // 1) If user pinned default_provider in config, try that first.
    if let Some(name) = cfg.default_provider.as_deref() {
        if let Some(id) = ProviderId::from_label(name) {
            if let Some(key) = fetch_key(id) {
                return Some(resolve(id, key, cfg.providers.get(name).cloned()));
            }
        }
    }
    // 2) Walk the canonical priority order.
    for id in DEFAULT_PRIORITY.iter().copied() {
        if let Some(key) = fetch_key(id) {
            let pcfg = cfg.providers.get(id.keyring_id()).cloned();
            return Some(resolve(id, key, pcfg));
        }
    }
    None
}

fn resolve(
    id: ProviderId,
    api_key: String,
    pcfg: Option<aphrodite_core::config::ProviderConfig>,
) -> ResolvedProvider {
    let model = pcfg
        .as_ref()
        .and_then(|c| c.model.clone())
        .unwrap_or_else(|| id.default_model().to_string());
    let base_url = pcfg
        .as_ref()
        .and_then(|c| c.base_url.clone())
        .or_else(|| pcfg.as_ref().and_then(|c| c.plan.as_deref()).map(|plan| id.base_url_for_plan(plan).to_string()));
    ResolvedProvider { id, api_key, model, base_url }
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
