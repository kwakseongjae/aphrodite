//! Multi-provider LLM routing.
//!
//! v0.1 priority (per Goal 2026-05-13):
//!   1. **z.ai GLM Coding Plan** — Anthropic-format endpoint at api.z.ai
//!   2. Anthropic (Claude) direct
//!   3. OpenRouter (OpenAI-format proxy; useful for any model)
//!   4. Offline fallback (deterministic, no network)
//!
//! v0.2 adds OAuth flows for OpenAI / Moonshot / Gemini. The credential
//! abstraction in `aphrodite-keyring` already supports an `oauth_token`
//! variant; the call sites here will switch when those flows ship.

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ProviderId {
    Zai,         // z.ai GLM Coding Plan (Anthropic-format)
    Anthropic,
    Openrouter,
    Openai,      // v0.2
    Moonshot,    // v0.2
    Gemini,      // v0.2
}

impl ProviderId {
    pub fn keyring_id(self) -> &'static str {
        match self {
            Self::Zai => "zai",
            Self::Anthropic => "anthropic",
            Self::Openrouter => "openrouter",
            Self::Openai => "openai",
            Self::Moonshot => "moonshot",
            Self::Gemini => "gemini",
        }
    }

    pub fn from_label(s: &str) -> Option<Self> {
        Some(match s {
            "zai" => Self::Zai,
            "anthropic" => Self::Anthropic,
            "openrouter" => Self::Openrouter,
            "openai" => Self::Openai,
            "moonshot" => Self::Moonshot,
            "gemini" => Self::Gemini,
            _ => return None,
        })
    }

    pub fn default_model(self) -> &'static str {
        match self {
            // GLM-5.1 is the current flagship of the z.ai Coding Plan
            // (paired with GLM-5-Turbo). GLM-4.7 is the prior generation.
            Self::Zai => "glm-5.1",
            Self::Anthropic => "claude-sonnet-4-6",
            Self::Openrouter => "anthropic/claude-sonnet-4.6",
            Self::Openai => "gpt-4o",
            Self::Moonshot => "moonshot-v1-128k",
            Self::Gemini => "gemini-2.0-flash",
        }
    }

    /// Curated model list (label, model_id) shown to the user during `aphrodite init`.
    pub fn curated_models(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::Zai => &[
                ("glm-5.1     — flagship · default · agentic/long-horizon", "glm-5.1"),
                ("glm-5-turbo — balanced · paired with 5.1 in Coding Plan", "glm-5-turbo"),
                ("glm-4.7     — prior gen · still strong on SWE-bench", "glm-4.7"),
                ("glm-4.5-air — cheapest · used for Haiku-tier defaults",  "glm-4.5-air"),
            ],
            Self::Anthropic => &[
                ("claude-sonnet-4-6 — default", "claude-sonnet-4-6"),
                ("claude-opus-4-7 — strongest, slowest", "claude-opus-4-7"),
                ("claude-haiku-4-5 — fast & cheap", "claude-haiku-4-5"),
            ],
            Self::Openrouter => &[
                ("anthropic/claude-sonnet-4.6 — default", "anthropic/claude-sonnet-4.6"),
                ("anthropic/claude-opus-4.7 — premium", "anthropic/claude-opus-4.7"),
                ("openai/gpt-4o", "openai/gpt-4o"),
                ("z-ai/glm-4.7", "z-ai/glm-4.7"),
            ],
            _ => &[],
        }
    }

    /// Plan options recognised by `aphrodite init`. Each plan maps to a
    /// default base_url; the user can still override per-provider.
    pub fn plans(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::Zai => &[
                ("Coding Plan subscription — Anthropic-compatible (recommended)", "coding_plan"),
                ("Standard API — pay-per-token, OpenAI-format", "standard_api"),
            ],
            _ => &[("Standard API", "standard_api")],
        }
    }

    pub fn base_url_for_plan(self, plan: &str) -> &'static str {
        match (self, plan) {
            (Self::Zai, "coding_plan") => "https://api.z.ai/api/anthropic",
            (Self::Zai, "standard_api") => "https://api.z.ai/api/coding/paas/v4",
            (Self::Anthropic, _) => "https://api.anthropic.com",
            (Self::Openrouter, _) => "https://openrouter.ai/api/v1",
            (Self::Openai, _) => "https://api.openai.com/v1",
            (Self::Moonshot, _) => "https://api.moonshot.cn/v1",
            (Self::Gemini, _) => "https://generativelanguage.googleapis.com/v1beta",
            _ => "",
        }
    }

    pub fn label(self) -> &'static str {
        self.keyring_id()
    }

    pub fn human_name(self) -> &'static str {
        match self {
            Self::Zai => "z.ai GLM",
            Self::Anthropic => "Anthropic",
            Self::Openrouter => "OpenRouter",
            Self::Openai => "OpenAI",
            Self::Moonshot => "Moonshot / Kimi",
            Self::Gemini => "Google Gemini",
        }
    }

    /// Anthropic-compatible (uses `/v1/messages` with `x-api-key`)
    pub fn is_anthropic_format(self, plan: Option<&str>) -> bool {
        matches!((self, plan), (Self::Anthropic, _) | (Self::Zai, Some("coding_plan")) | (Self::Zai, None))
    }
}

/// The order Aphrodite tries providers in when none is explicitly requested.
pub const DEFAULT_PRIORITY: &[ProviderId] =
    &[ProviderId::Zai, ProviderId::Anthropic, ProviderId::Openrouter];

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("http: {0}")]
    Http(#[from] reqwest::Error),
    #[error("provider api: status {status}: {body}")]
    Api { status: u16, body: String },
    #[error("malformed response: {0}")]
    Malformed(String),
}

const SYSTEM_PROMPT: &str = r##"You are Aphrodite, a senior product designer. You return ONLY a single DESIGN.md document conforming to the Google Labs DESIGN.md alpha specification.

REQUIRED OUTPUT SHAPE — start IMMEDIATELY with `---` and a YAML frontmatter, then markdown body. Do NOT add prose before or after. Do NOT wrap in code fences.

The frontmatter MUST include:
  name: <short project name>
  version: "0.1"
  description: <one-line description>
  colors:
    primary:        # REQUIRED — at least shades 50, 500, 900
      "50":  "#..."
      "500": "#..."
      "900": "#..."
    neutral:
      "0":   "#ffffff"
      "1000":"#000000"
  typography:
    display:
      family: "<font family>"
      weight: 700
    body:
      family: "<font family>"
      weight: 400
  spacing:
    "1": "4px"
    "2": "8px"
    "4": "16px"
    "8": "32px"
  rounded:
    sm: "4px"
    md: "8px"
  metadata:
    variants:
      light:
        description: "Default light mode"
        tokens:
          colors.background.primary: "#ffffff"
          colors.text.primary: "#111111"
      dark:
        description: "Dark mode"
        tokens:
          colors.background.primary: "#0b0b10"
          colors.text.primary: "#f5f5f7"
      brand-a:
        description: "First brand variant"
        tokens:
          colors.background.primary: "#fff8f1"
          colors.text.primary: "#241407"
      brand-b:
        description: "Second brand variant"
        tokens:
          colors.background.primary: "#f1faf8"
          colors.text.primary: "#04231d"

For every variant, ensure WCAG-AA contrast (≥ 4.5:1) between `colors.text.primary` and `colors.background.primary`.

After the frontmatter, write the eight ordered sections:
  # Overview
  # Colors
  # Typography
  # Layout
  # Elevation & Depth
  # Shapes
  # Components
  # Do's and Don'ts

Each section is 2-5 short paragraphs of design rationale. No code, no JSON, no fences."##;

/// A resolved call target. CLI/MCP build one of these from keyring + config
/// lookups, then hand it to `call`.
pub struct ResolvedProvider {
    pub id: ProviderId,
    pub api_key: String,
    pub model: String,
    pub base_url: Option<String>,
}

impl ResolvedProvider {
    pub fn with_default_model(id: ProviderId, api_key: String) -> Self {
        Self {
            id,
            api_key,
            model: id.default_model().to_string(),
            base_url: None,
        }
    }
}

/// Run the resolved provider against the user intent. Returns DESIGN.md text.
pub async fn call(resolved: &ResolvedProvider, intent: &str) -> Result<String, ProviderError> {
    call_with_taste(resolved, intent, &aphrodite_core::TasteSnapshot::default()).await
}

/// Low-level provider call with a caller-supplied system prompt and max-tokens.
/// Used by the surface composer. Returns the model's raw text (after fence
/// stripping that's defensive against minor prefixes).
pub async fn call_raw(
    resolved: &ResolvedProvider,
    system: &str,
    user: &str,
    max_tokens: u32,
) -> Result<String, ProviderError> {
    let default_base = resolved.id.base_url_for_plan("coding_plan");
    let base = resolved.base_url.as_deref().unwrap_or(default_base);
    let anthropic_wire = match resolved.id {
        ProviderId::Anthropic => true,
        ProviderId::Zai => base.contains("/api/anthropic"),
        _ => false,
    };
    if matches!(resolved.id, ProviderId::Gemini) {
        return Err(ProviderError::Malformed("Gemini provider lands in v0.2".into()));
    }
    if anthropic_wire {
        call_anthropic_compat_custom(base, &resolved.api_key, &resolved.model, system, user, max_tokens).await
    } else {
        call_openai_compat_custom(base, &resolved.api_key, &resolved.model, system, user, max_tokens).await
    }
}

/// Same as `call`, but injects the accumulated taste snapshot into the user
/// message so the LLM can bias subsequent generations.
pub async fn call_with_taste(
    resolved: &ResolvedProvider,
    intent: &str,
    taste: &aphrodite_core::TasteSnapshot,
) -> Result<String, ProviderError> {
    let mut user = format!(
        "Design intent: {intent}\n\nReturn the DESIGN.md now. Remember: start with `---`, no fences, all four variants (light, dark, brand-a, brand-b), WCAG-AA contrast in every variant."
    );
    let regen = taste.regenerate_count();
    if regen > 0 {
        user.push_str("\n\nUser taste signals so far:");
        user.push_str(&format!(
            "\n- The user has explicitly asked for regeneration {regen} time(s) against similar intents. Avoid the obvious first-instinct palette; pick a different hue family, restraint level, and type pairing than you would have first reached for."
        ));
        if !taste.recent.is_empty() {
            user.push_str("\n- Recent signals (most recent last):");
            for ev in &taste.recent {
                user.push_str(&format!(
                    "\n  · [{:?}] {} at {}",
                    ev.signal_kind, ev.invocation_id, ev.ts
                ));
            }
        }
        user.push_str("\n\nMake this output noticeably different — different primary hue family, different typographic mood — from a baseline first attempt.");
    }
    let _ = intent; // satisfies the borrow checker post-format
    let local_user = user;
    let default_base = resolved.id.base_url_for_plan("coding_plan");
    let base = resolved.base_url.as_deref().unwrap_or(default_base);

    let anthropic_wire = match resolved.id {
        ProviderId::Anthropic => true,
        ProviderId::Zai => base.contains("/api/anthropic"),
        _ => false,
    };
    let _ = anthropic_wire; // placeholder — used below
    if matches!(resolved.id, ProviderId::Gemini) {
        return Err(ProviderError::Malformed("Gemini provider lands in v0.2".into()));
    }
    if anthropic_wire {
        call_anthropic_compat(base, &resolved.api_key, &resolved.model, &local_user).await
    } else {
        call_openai_compat(base, &resolved.api_key, &resolved.model, &local_user).await
    }
}

/// Anthropic-format with caller-supplied system/user/max_tokens.
async fn call_anthropic_compat_custom(
    base_url: &str,
    api_key: &str,
    model: &str,
    system: &str,
    user_msg: &str,
    max_tokens: u32,
) -> Result<String, ProviderError> {
    #[derive(Serialize)]
    struct Req<'a> {
        model: &'a str,
        max_tokens: u32,
        system: &'a str,
        messages: Vec<Msg<'a>>,
    }
    #[derive(Serialize)]
    struct Msg<'a> { role: &'a str, content: &'a str }
    #[derive(Deserialize)]
    struct Resp { #[serde(default)] content: Vec<Block> }
    #[derive(Deserialize)]
    struct Block { #[serde(rename = "type")] ty: String, #[serde(default)] text: String }
    let url = format!("{}/v1/messages", base_url.trim_end_matches('/'));
    let req = Req { model, max_tokens, system, messages: vec![Msg { role: "user", content: user_msg }] };
    let resp = reqwest::Client::new()
        .post(&url)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&req)
        .send().await?;
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(ProviderError::Api { status: status.as_u16(), body });
    }
    let parsed: Resp = resp.json().await?;
    let text = parsed.content.into_iter().filter(|b| b.ty == "text").map(|b| b.text).collect::<Vec<_>>().join("\n");
    Ok(strip_fences(&text))
}

async fn call_openai_compat_custom(
    base_url: &str,
    api_key: &str,
    model: &str,
    system: &str,
    user_msg: &str,
    max_tokens: u32,
) -> Result<String, ProviderError> {
    #[derive(Serialize)]
    struct Req<'a> { model: &'a str, max_tokens: u32, messages: Vec<Msg<'a>> }
    #[derive(Serialize)]
    struct Msg<'a> { role: &'a str, content: &'a str }
    #[derive(Deserialize)]
    struct Resp { #[serde(default)] choices: Vec<Choice> }
    #[derive(Deserialize)]
    struct Choice { #[serde(default)] message: Option<RespMsg> }
    #[derive(Deserialize)]
    struct RespMsg { #[serde(default)] content: String }
    let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));
    let req = Req { model, max_tokens, messages: vec![
        Msg { role: "system", content: system },
        Msg { role: "user", content: user_msg },
    ]};
    let resp = reqwest::Client::new()
        .post(&url)
        .header("authorization", format!("Bearer {api_key}"))
        .header("content-type", "application/json")
        .header("http-referer", "https://github.com/aphrodite-ui")
        .header("x-title", "Aphrodite")
        .json(&req)
        .send().await?;
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(ProviderError::Api { status: status.as_u16(), body });
    }
    let parsed: Resp = resp.json().await?;
    let text = parsed.choices.into_iter().next().and_then(|c| c.message).map(|m| m.content)
        .ok_or_else(|| ProviderError::Malformed("no choices/message in response".into()))?;
    Ok(strip_fences(&text))
}

/// Anthropic-format /v1/messages. Works for api.anthropic.com and api.z.ai/api/anthropic.
async fn call_anthropic_compat(
    base_url: &str,
    api_key: &str,
    model: &str,
    user_msg: &str,
) -> Result<String, ProviderError> {
    #[derive(Serialize)]
    struct Req<'a> {
        model: &'a str,
        max_tokens: u32,
        system: &'a str,
        messages: Vec<Msg<'a>>,
    }
    #[derive(Serialize)]
    struct Msg<'a> {
        role: &'a str,
        content: &'a str,
    }
    #[derive(Deserialize)]
    struct Resp {
        #[serde(default)]
        content: Vec<Block>,
    }
    #[derive(Deserialize)]
    struct Block {
        #[serde(rename = "type")]
        ty: String,
        #[serde(default)]
        text: String,
    }

    let url = format!("{}/v1/messages", base_url.trim_end_matches('/'));
    let req = Req {
        model,
        max_tokens: 4096,
        system: SYSTEM_PROMPT,
        messages: vec![Msg { role: "user", content: user_msg }],
    };
    let resp = reqwest::Client::new()
        .post(&url)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&req)
        .send()
        .await?;
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(ProviderError::Api { status: status.as_u16(), body });
    }
    let parsed: Resp = resp.json().await?;
    let text = parsed
        .content
        .into_iter()
        .filter(|b| b.ty == "text")
        .map(|b| b.text)
        .collect::<Vec<_>>()
        .join("\n");
    Ok(strip_fences(&text))
}

/// OpenAI-format /chat/completions. Works for OpenRouter, OpenAI direct, Moonshot.
async fn call_openai_compat(
    base_url: &str,
    api_key: &str,
    model: &str,
    user_msg: &str,
) -> Result<String, ProviderError> {
    #[derive(Serialize)]
    struct Req<'a> {
        model: &'a str,
        max_tokens: u32,
        messages: Vec<Msg<'a>>,
    }
    #[derive(Serialize)]
    struct Msg<'a> {
        role: &'a str,
        content: &'a str,
    }
    #[derive(Deserialize)]
    struct Resp {
        #[serde(default)]
        choices: Vec<Choice>,
    }
    #[derive(Deserialize)]
    struct Choice {
        #[serde(default)]
        message: Option<RespMsg>,
    }
    #[derive(Deserialize)]
    struct RespMsg {
        #[serde(default)]
        content: String,
    }

    let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));
    let req = Req {
        model,
        max_tokens: 4096,
        messages: vec![
            Msg { role: "system", content: SYSTEM_PROMPT },
            Msg { role: "user", content: user_msg },
        ],
    };
    let resp = reqwest::Client::new()
        .post(&url)
        .header("authorization", format!("Bearer {api_key}"))
        .header("content-type", "application/json")
        .header("http-referer", "https://github.com/aphrodite-ui")
        .header("x-title", "Aphrodite")
        .json(&req)
        .send()
        .await?;
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(ProviderError::Api { status: status.as_u16(), body });
    }
    let parsed: Resp = resp.json().await?;
    let text = parsed
        .choices
        .into_iter()
        .next()
        .and_then(|c| c.message)
        .map(|m| m.content)
        .ok_or_else(|| ProviderError::Malformed("no choices/message in response".into()))?;
    Ok(strip_fences(&text))
}

fn strip_fences(s: &str) -> String {
    let trimmed = s.trim();
    let without_open = trimmed
        .strip_prefix("```markdown\n")
        .or_else(|| trimmed.strip_prefix("```md\n"))
        .or_else(|| trimmed.strip_prefix("```\n"))
        .unwrap_or(trimmed);
    let without_close = without_open
        .strip_suffix("\n```")
        .or_else(|| without_open.strip_suffix("```"))
        .unwrap_or(without_open);
    // LLMs sometimes prefix DESIGN.md output with conversational prose
    // ("Here's your design system…") before the actual `---` frontmatter
    // delimiter. Find the first `---` on its own line and drop anything before.
    let s = without_close;
    if !s.starts_with("---") {
        let key = "\n---\n";
        if let Some(idx) = s.find(key) {
            return s[idx + 1..].to_string();
        }
        let key_cr = "\n---\r\n";
        if let Some(idx) = s.find(key_cr) {
            return s[idx + 1..].to_string();
        }
    }
    s.to_string()
}
