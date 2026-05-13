//! Surface composition — the second LLM call.
//!
//! v0.1 only emitted a token-showcase `hero.html` regardless of intent. That
//! left users staring at a button-and-headline when they asked for a pricing
//! page or a dashboard. v0.2 fixes the gap: after DESIGN.md is generated, we
//! make a *second* LLM call that asks the model to compose a real,
//! structurally-appropriate HTML surface using the DESIGN.md tokens.
//!
//! The model classifies the intent into one of six surface types and emits a
//! complete `<!DOCTYPE html>` document with inline CSS that references the
//! CSS custom properties the harness injects from the DESIGN.md.
//!
//! Output shape (LLM contract):
//!   SURFACE: <type>\n
//!   <!DOCTYPE html>...
//!
//! `parse_response` splits the type marker from the body.

use crate::provider::{self, ProviderError, ResolvedProvider};
use aphrodite_core::DesignDocument;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceType {
    Pricing,
    Dashboard,
    MobileApp,
    Editorial,
    Landing,
    Portfolio,
}

impl SurfaceType {
    pub fn from_label(s: &str) -> Option<Self> {
        Some(match s.trim().to_ascii_lowercase().as_str() {
            "pricing" => Self::Pricing,
            "dashboard" => Self::Dashboard,
            "mobile_app" | "mobile" => Self::MobileApp,
            "editorial" => Self::Editorial,
            "landing" => Self::Landing,
            "portfolio" => Self::Portfolio,
            _ => return None,
        })
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Pricing => "pricing",
            Self::Dashboard => "dashboard",
            Self::MobileApp => "mobile_app",
            Self::Editorial => "editorial",
            Self::Landing => "landing",
            Self::Portfolio => "portfolio",
        }
    }
}

const SURFACE_SYSTEM_PROMPT: &str = r#####"You are a senior product designer building a production-quality web page.

You receive:
  1. A DESIGN.md (the design system the harness already generated — palette,
     typography, spacing, components, do's and don'ts).
  2. The user's original intent string.

STEP 1 — Classify the intent into exactly one of these surface types:
  - "pricing"   — pricing page, tier comparison, billing plans
  - "dashboard" — data dashboard, analytics screen, monitoring view
  - "mobile_app" — mobile-first app screen (must be phone-framed)
  - "editorial" — long-form content, magazine, blog, article
  - "landing"   — marketing landing page with hero + features (default fallback)
  - "portfolio" — work showcase, case studies, project grid

STEP 2 — Build a COMPLETE, SELF-CONTAINED HTML document for that surface that:

  Layout structure (REQUIRED per surface type — must include these elements):
    pricing    → eyebrow + h1 + lede + 3 tier cards (Starter/Pro/Enterprise unless
                  intent says otherwise) + feature comparison table (≥ 8 rows) +
                  FAQ (≥ 4 questions) + final CTA
    dashboard  → top bar with title + 4-6 metric tiles + at least 2 inline-SVG
                  charts (line or bar — use simple paths, no JS) + a recent-activity
                  data table (≥ 6 rows) + a sidebar nav with ≥ 5 items
    mobile_app → wrap everything in a 390×844 phone frame (a div with explicit
                  width and rounded corners) + status bar + screen content (title,
                  primary content card, list of ≥ 4 items) + bottom-anchored CTA
                  inside the frame + bottom tab bar with ≥ 3 icons
    editorial  → magazine-style: kicker + headline + byline + lede with drop cap
                  on the first paragraph + body of ≥ 6 paragraphs + 1 pull quote
                  styled differently + footer with category and date
    landing    → hero (eyebrow + headline + lede + dual CTA) + features section
                  with ≥ 3 feature cards + testimonial section + final CTA band
    portfolio  → header + grid of ≥ 6 project cards (each with project name,
                  category tag, year, hover-distinct treatment via :hover CSS) +
                  small about/contact footer

  Styling rules:
    - All colors and typography come from CSS custom properties the harness
      injects (do NOT hardcode hex values). Use:
        --colors-primary-50 … --colors-primary-900
        --colors-text-primary, --colors-background-primary, --colors-text-muted
        --typography-display-family, --typography-body-family
        --spacing-1 … --spacing-16
        --rounded-sm, --rounded-md, --rounded-lg
      For palettes other than `primary`, use `--colors-<name>-<shade>`
      (e.g. `--colors-accent-500`, `--colors-neutral-100`).
    - NO external resources: no `<script>` tags, no remote fonts via @import,
      no `<img>` tags (use inline SVG instead for any visual element).
    - The page must include a top-right variant switcher matching the existing
      hero template: a `<nav class="switcher">` with buttons named `light`,
      `dark`, and any brand variants present in the DESIGN.md, toggling
      `document.body.dataset.variant`. Include the inline `<script>` for it.
    - Use `<style>` at the top of `<head>` for all CSS. Inline SVG everywhere
      visual interest is needed.

  Quality bar:
    - The page must look like a real page someone would ship, not a placeholder.
    - Visual hierarchy: hero/headline should be ≥ 48px display; metrics in
      dashboards should be ≥ 36px; pricing tier numbers ≥ 40px.
    - Use whitespace generously — section padding ≥ 64px desktop / 32px mobile.
    - Aim for ≥ 8 000 bytes of HTML — short outputs read as placeholders.

OUTPUT FORMAT — exactly this shape, no prose around it:

SURFACE: <one of the six labels>
<!DOCTYPE html>
<html ...>
...complete document...
</html>

No commentary. No code fences. No explanation. Start your response with the
word "SURFACE:" on line 1.
"#####;

pub struct SurfaceOutput {
    pub surface_type: SurfaceType,
    pub html: String,
}

/// Compose a real, structurally-appropriate HTML surface from a DESIGN.md +
/// intent via a second LLM call. Falls back to the hero template only when
/// the LLM response can't be parsed.
pub async fn compose(
    resolved: &ResolvedProvider,
    intent: &str,
    design_md: &str,
    doc: &DesignDocument,
) -> Result<SurfaceOutput, ProviderError> {
    let user = format!(
        "Original user intent:\n{intent}\n\n\
         The DESIGN.md you (or a peer) just generated for this intent:\n\
         ----- BEGIN DESIGN.md -----\n{design_md}\n----- END DESIGN.md -----\n\n\
         Now classify the intent and build the surface. Output the SURFACE: marker on line 1, then the full <!DOCTYPE html>."
    );

    let raw = provider::call_raw(resolved, SURFACE_SYSTEM_PROMPT, &user, 8192).await?;
    parse_response(&raw, doc).ok_or_else(|| {
        ProviderError::Malformed(
            "surface response missing SURFACE: marker or <!DOCTYPE html>".into(),
        )
    })
}

fn parse_response(raw: &str, _doc: &DesignDocument) -> Option<SurfaceOutput> {
    // Strip fences / leading whitespace.
    let s = raw.trim_start();
    let s = s
        .strip_prefix("```html\n")
        .or_else(|| s.strip_prefix("```\n"))
        .unwrap_or(s);
    let s = s.trim_end().trim_end_matches("```").trim_end();

    // Find SURFACE: marker.
    let surface_idx = s.find("SURFACE:")?;
    let after_marker = &s[surface_idx + "SURFACE:".len()..];
    let newline = after_marker.find('\n')?;
    let label = after_marker[..newline].trim();
    let surface_type = SurfaceType::from_label(label).unwrap_or(SurfaceType::Landing);

    // Find the start of the HTML document.
    let body_start = after_marker[newline + 1..].find("<!DOCTYPE")?;
    let html = after_marker[newline + 1 + body_start..].to_string();

    Some(SurfaceOutput { surface_type, html })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_clean_response() {
        let raw = "SURFACE: pricing\n<!DOCTYPE html><html><body>x</body></html>";
        let doc = DesignDocument {
            frontmatter: Default::default(),
            body: String::new(),
            sections: Vec::new(),
        };
        let out = parse_response(raw, &doc).unwrap();
        assert_eq!(out.surface_type, SurfaceType::Pricing);
        assert!(out.html.starts_with("<!DOCTYPE html>"));
    }

    #[test]
    fn parses_with_prose_prefix() {
        let raw = "Sure, here is the page.\n\nSURFACE: dashboard\n<!DOCTYPE html><html></html>";
        let doc = DesignDocument {
            frontmatter: Default::default(),
            body: String::new(),
            sections: Vec::new(),
        };
        let out = parse_response(raw, &doc).unwrap();
        assert_eq!(out.surface_type, SurfaceType::Dashboard);
    }
}
