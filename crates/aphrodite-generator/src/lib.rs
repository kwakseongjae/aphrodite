//! aphrodite-generator — orchestrates LLM provider + hero HTML rendering.

pub mod critic;
pub mod extractor;
pub mod harmonize;
pub mod hero;
pub mod offline;
pub mod orchestrator;
pub mod provider;
pub mod refine;
pub mod skill;
pub mod surface;
pub mod wiki_fetch;

use aphrodite_core::{parse_design, resolve_variants, DesignDocument, Invocation, Variant};
use provider::{ProviderId, ResolvedProvider, DEFAULT_PRIORITY};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationOutput {
    pub design_md: String,
    pub design_doc: DesignDocument,
    pub variants: Vec<Variant>,
    pub hero_html: String,
    /// Real intent-specific composition (pricing / dashboard / mobile / editorial /
    /// landing / portfolio). Empty when the second LLM call wasn't made (e.g.
    /// offline provider) — `hero_html` is then the only artifact.
    pub composition_html: String,
    pub surface_type: Option<String>,
    pub provider_used: String,
    pub model_used: String,
    /// Non-fatal observations: out-of-scope intent terms, provider downgrades,
    /// deprecated config keys. Empty array on clean runs.
    pub warnings: Vec<Warning>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warning {
    pub kind: String,
    pub message: String,
    pub hint: String,
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
    generate_with_user_intent(invocation, resolved, None).await
}

/// Like `generate_with`, but lets the caller declare the *user-visible* intent
/// separately from `invocation.intent` (which may include scaffolds, taste
/// hints, or other prompt augmentation). Out-of-scope warning detection uses
/// the user intent so scaffold content can't trigger false-positive warnings.
pub async fn generate_with_user_intent(
    invocation: &Invocation,
    resolved: Option<&ResolvedProvider>,
    user_intent_for_warnings: Option<&str>,
) -> Result<GenerationOutput, GenError> {
    let intent_for_warnings = user_intent_for_warnings.unwrap_or(&invocation.intent);
    // Finding #5 / seed acceptance #8: read accumulated taste + preferences
    // before generating so each call reflects what the user has signaled.
    let taste = aphrodite_core::taste_snapshot(&invocation.target_repo);
    let prefs = aphrodite_core::preferences::load(&invocation.target_repo);

    let (design_md, provider_used, model_used) = match resolved {
        Some(r) => {
            let md = call_with_retry(r, &invocation.intent, &taste, &prefs, &invocation.target_repo).await?;
            (md, r.id.label().to_string(), r.model.clone())
        }
        None => (
            offline::generate_with_taste(&invocation.intent, &taste),
            "offline".to_string(),
            "deterministic".to_string(),
        ),
    };

    let design_doc = parse_design(&design_md)?;
    let variants = resolve_variants(&design_doc);
    let hero_html = hero::render(&design_doc, &variants).map_err(GenError::Hero)?;
    // Scan the user-visible intent for v0.1 out-of-scope phrases; scaffold
    // content (which the caller may inject into invocation.intent) cannot
    // produce false-positive warnings about images/motion/etc.
    let warnings = warnings_for(intent_for_warnings, &provider_used);

    // Second LLM call — compose the *real* intent-specific surface. Skipped
    // in offline mode (no useful composition is possible without a real
    // model). On failure, we still return the design + hero; composition_html
    // stays empty and a warning is emitted.
    let (composition_html, surface_type, surface_warnings) = match resolved {
        Some(r) => match surface::compose(r, &invocation.intent, &design_md, &design_doc).await {
            Ok(out) => {
                let html = hero::inject_variant_css(&out.html, &design_doc, &variants);
                (html, Some(out.surface_type.label().to_string()), Vec::new())
            }
            Err(e) => (
                String::new(),
                None,
                vec![Warning {
                    kind: "surface_compose_failed".to_string(),
                    message: format!("second LLM call failed: {e}"),
                    hint: "DESIGN.md + hero.html still emitted. Retry the design call.".to_string(),
                }],
            ),
        },
        None => (String::new(), None, Vec::new()),
    };
    let mut warnings = warnings;
    warnings.extend(surface_warnings);

    Ok(GenerationOutput {
        design_md,
        design_doc,
        variants,
        hero_html,
        composition_html,
        surface_type,
        provider_used,
        model_used,
        warnings,
    })
}

/// Inspect intent + context for v0.1 out-of-scope asks and emit non-fatal
/// warnings. Agents can branch on `warnings[].kind` without parsing free text.
fn warnings_for(intent: &str, provider_used: &str) -> Vec<Warning> {
    let mut out = Vec::new();
    let lower = intent.to_ascii_lowercase();

    // Out-of-scope content vocabulary the v0.1 generator does not produce.
    let buckets: &[(&[&str], &str, &str)] = &[
        (
            &["image", "illustration", "photo", "photograph", "render an", "png", "jpg", "jpeg", "webp"],
            "image_generation",
            "Aphrodite v0.1 does not generate or fetch images. Variants ship token-driven HTML/CSS only. The intent for an image was ignored; consider adding the asset yourself or wait for the image adapter (post-v0.1).",
        ),
        (
            &["video", "mp4", "mov", "webm", "motion", "animate", "scroll-jacking", "parallax"],
            "motion_or_video",
            "Aphrodite v0.1 emits a static hero. Motion / video lands with the HyperFrames adapter in v0.2.",
        ),
        (
            &["three.js", "threejs", "webgl", "3d ", "canvas", " glb", " gltf"],
            "three_d_scene",
            "Aphrodite v0.1 has no 3D adapter. three.js/Blender support lands in v0.3.",
        ),
        (
            &["figma", "sketch.com", "framer"],
            "design_tool_roundtrip",
            "Aphrodite v0.1 does not round-trip with design tools. Figma Tokens import/export lands in v0.2.",
        ),
    ];

    for (keys, kind, hint) in buckets {
        if keys.iter().any(|k| lower.contains(k)) {
            out.push(Warning {
                kind: (*kind).to_string(),
                message: format!("intent mentions `{}`, which Aphrodite v0.1 does not produce; the request was satisfied without it.", keys[0]),
                hint: (*hint).to_string(),
            });
        }
    }

    // Provider downgrade — config intends one, runtime resolved another.
    let cfg = aphrodite_core::config::load();
    if let Some(intended) = cfg.default_provider.as_deref() {
        if provider_used == "offline" && intended != "offline" {
            out.push(Warning {
                kind: "provider_downgraded".to_string(),
                message: format!(
                    "config sets default_provider=`{intended}` but no credential was readable; fell back to offline."
                ),
                hint: format!(
                    "Run `aphrodite auth set {intended}` or set APHRODITE_{}_API_KEY. `aphrodite auth verify {intended}` checks the keychain entry.",
                    intended.to_ascii_uppercase()
                ),
            });
        }
    }

    out
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

/// Wrap the provider call with one retry on parse failure (Finding #19).
/// On retry, send a sharper instruction to force valid frontmatter. On the
/// final failure, dump the raw response to `<project>/.aphrodite/failures/`.
async fn call_with_retry(
    resolved: &provider::ResolvedProvider,
    intent: &str,
    taste: &aphrodite_core::TasteSnapshot,
    prefs: &aphrodite_core::preferences::TastePreferences,
    project_root: &std::path::Path,
) -> Result<String, GenError> {
    let pref_hint = prefs.as_prompt_hint();
    let intent_with_prefs = if pref_hint.is_empty() {
        intent.to_string()
    } else {
        format!("{intent}\n\n--- User taste profile (accumulated across past sessions) ---{pref_hint}\n--- End taste profile ---\n\nBias the output toward the preferences above when the intent doesn't otherwise specify.")
    };

    let first = provider::call_with_taste(resolved, &intent_with_prefs, taste).await?;
    if aphrodite_core::parse_design(&first).is_ok() {
        return Ok(first);
    }
    let _ = save_failure(project_root, "first-attempt", &first);
    let stricter = format!(
        "{intent_with_prefs}\n\nIMPORTANT: A previous attempt at this design returned text that failed to parse. \
         Your response MUST start with exactly `---` on line 1 (three dashes, nothing else), \
         then the YAML frontmatter, then exactly `---` on its own line, then the markdown body. \
         No prose before, no commentary after, no code fences."
    );
    let second = provider::call_with_taste(resolved, &stricter, taste).await?;
    match aphrodite_core::parse_design(&second) {
        Ok(_) => Ok(second),
        Err(e2) => {
            let _ = save_failure(project_root, "retry", &second);
            Err(GenError::Design(e2))
        }
    }
}

fn save_failure(project_root: &std::path::Path, label: &str, raw: &str) -> std::io::Result<std::path::PathBuf> {
    let dir = project_root.join(".aphrodite").join("failures");
    std::fs::create_dir_all(&dir)?;
    let ts = aphrodite_core::taste::now_rfc3339().replace(':', "-");
    let path = dir.join(format!("{ts}-{label}.md"));
    let body = format!("# Aphrodite parse failure\n\n- when: {ts}\n- attempt: {label}\n\n## Raw LLM response\n\n```\n{raw}\n```\n");
    std::fs::write(&path, body)?;
    Ok(path)
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
