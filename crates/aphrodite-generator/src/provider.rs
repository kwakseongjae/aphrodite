//! LLM provider routing. v0.1 supports Anthropic via API key. The Messages
//! API call asks the model to return *only* DESIGN.md markdown — we strip any
//! fence wrapping and hand the raw body to `aphrodite_core::parse_design`.

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ProviderId {
    Anthropic,
    Openai,    // v0.2
    Moonshot,  // v0.2
    Gemini,    // v0.2
}

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("http: {0}")]
    Http(#[from] reqwest::Error),
    #[error("anthropic api: status {status}: {body}")]
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

pub async fn call_anthropic(api_key: &str, intent: &str) -> Result<String, ProviderError> {
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
        content: Vec<Block>,
    }
    #[derive(Deserialize)]
    struct Block {
        #[serde(rename = "type")]
        ty: String,
        #[serde(default)]
        text: String,
    }

    let user = format!(
        "Design intent: {intent}\n\nReturn the DESIGN.md now. Remember: start with `---`, no fences, all four variants, WCAG-AA contrast in every variant."
    );

    let req = Req {
        model: "claude-sonnet-4-6",
        max_tokens: 4096,
        system: SYSTEM_PROMPT,
        messages: vec![Msg { role: "user", content: &user }],
    };

    let resp = reqwest::Client::new()
        .post("https://api.anthropic.com/v1/messages")
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
    without_close.to_string()
}
