//! Self-critic — Phase 4 of the autonomous creation harness (ADR 0004).
//!
//! Given the current DESIGN.md + composition.html for a stated intent,
//! identify the SINGLE most impactful axis to refine next, or declare
//! satisfaction. The output is a structured `SelfCritique` whose
//! `proposed_delta` (when present) is directly consumable by `refine::refine`.
//!
//! This is the loop that Pass 7 ran *externally* through the calling agent's
//! reactive judgement. Internalising it is the v0.3 win condition.
//!
//! Anti-patterns the prompt actively guards against:
//!   - Multi-axis refinement (drift)
//!   - Oscillation (proposing the inverse of a delta you proposed two turns ago)
//!   - Over-refinement (always finding "one more thing" past satisfaction)
//!   - Taste hallucination (preferring serif just because — must anchor to
//!     intent register or to the user's accumulated TastePreferences)

use crate::provider::{self, ProviderError, ResolvedProvider};
use serde::{Deserialize, Serialize};

const CRITIC_SYSTEM_PROMPT: &str = r###"You are a senior design critic reviewing a DESIGN.md + composition.html that another LLM produced for a stated intent. You have ONE job: identify THE SINGLE most impactful axis to refine next — or declare satisfaction.

Output ONLY a JSON object, no prose around it, no code fences. Shape:

{
  "satisfaction": <float 0.0 to 1.0>,
  "weakest_axis": "<one of: color, typography, spacing, structure, brand_variants, harmony, prose_token_mismatch, accessibility, OR null>",
  "rationale": "<one sentence — WHY this axis is the weakest right now>",
  "proposed_delta": "<the literal refine instruction string, or null if satisfied>"
}

Stopping rules — apply BEFORE picking an axis:
- If `satisfaction >= 0.85`, emit `weakest_axis: null` and `proposed_delta: null`. Stop.
- If the prior-turn history shows you already proposed a similar delta on this axis in the last 2 turns, BUMP satisfaction by 0.10 and DO NOT propose the same axis again.
- If you're about to propose something that contradicts a delta from 2+ turns ago (e.g. you just made it cooler and now want warmer), recognise oscillation: bump satisfaction, stop.
- Hard cap: never propose more than what one refine call can change. ONE axis. NEVER bundle.

Critic priorities (apply in this order — pick the FIRST violation you see):
1. **prose_token_mismatch** — DESIGN.md prose claims something the tokens don't deliver. ("pages breathe" but max spacing 96px; "muted accents" but primary at saturation 0.9.)
2. **register mismatch** (typography / color) — font or palette feels wrong for the intent's domain. Renaissance serif for a Seoul craftsperson workshop, neon cyber palette for a longevity clinic.
3. **structure** — composition is missing a key UI element the intent requires, or contains a SaaS-style filler (stats-row vanity metrics on a maker portfolio).
4. **brand_variants** — light / dark / brand-a / brand-b serve overlapping or incoherent purposes (e.g. brand-a background is yellower than light variant — what's the point).
5. **accessibility** — WCAG-AA violation surfaced in the validation report.
6. **harmony** — final cross-file consistency (font declared but not imported, hero.html ignores DESIGN.md tokens).

What COUNTS as a refine-able delta — must be specific, axis-local, surgical:
  ✓ "switch display family from EB Garamond to Source Serif 4 — Renaissance register is wrong for a Seoul craftsperson workshop"
  ✗ "make it more modern" (too vague)
  ✗ "swap fonts AND adjust palette" (multi-axis)

Do NOT propose:
- Pure taste preferences with no anchor to the intent or the user's accumulated TastePreferences below.
- Complete redesigns.
- "Try a different style" without naming what.

If the user has accumulated TastePreferences hints, weight your axis pick by those — repeated past dislikes outrank single-turn aesthetic instincts.

## Persona authority — final tiebreaker

If a **Persona authority** block appears in the scaffolded context below, treat it as the FINAL authority on aesthetic decisions. Generic skill scaffolds (`editorial-portfolio`, `asset-standards`, etc.) are guidelines under the persona — not above it.

**Before proposing a refine that moves away from a persona-driven aesthetic choice**, you MUST do one of the following:

1. **Quote the specific persona principle the current design violates** (verbatim from the persona's principles / rejects list), and explain why your proposed refine honours that principle.
2. **OR** raise satisfaction by 0.20 and emit `proposed_delta: null` — i.e. accept the persona-driven pick even if it would have failed scaffold-only judgment.

Example of valid override of persona — accessibility / language support:
  ✓ "Persona prefers chunky display, but the chosen font Dela Gothic One lacks Korean coverage which the Seoul intent requires. Refining to a Hangul-capable chunky display preserves persona principle while fixing the WCAG-blocking glyph gap."

Example of INVALID override (the failure pattern from prior runs):
  ✗ "The chosen font contradicts the editorial-portfolio scaffold's requirement for a contemporary serif."

The second is *not allowed* when a persona is present, because the editorial-portfolio scaffold is *under* the persona. Quoting the scaffold over the persona is a tell that you've defaulted to safe-mode rather than honoured the brief.

If you cannot articulate a persona-anchored justification, the persona-driven pick stays. Restraint here is correct.

## Scaffold-language discipline (also applies when NO persona is present)

When reading a skill scaffold, distinguish between **prescriptive** and **suggestive** language:

- Prescriptive (treat as hard rule): "use X", "avoid X", "must be ≥ N px", "REJECTS:", explicit numeric thresholds.
- Suggestive (treat as guidance, not enforcement): "such as X, Y, Z", "good candidates: X, Y, Z", "for example: X", "consider X".

**Do NOT refine away from a pick simply because it does not appear in a suggestive list.** A scaffold saying *"Source Serif 4, Newsreader, or Fraunces are good candidates"* is naming THREE examples of a category (*contemporary display serif with optical sizing*). A fourth pick that satisfies the same category — Crimson Pro, Tiempos Display, GT Sectra — is equally valid. Refine only if the chosen family violates the category itself (e.g. a tech-geometric grotesque where a contemporary serif was named).

Before proposing a refine that cites a scaffold list, ask yourself: **does the current design violate the underlying category, or merely a specific named option?** Only the former warrants a refine. The latter is over-correction — bump satisfaction by 0.10 instead.
"###;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfCritique {
    pub satisfaction: f32,
    /// One of the axis labels listed in the system prompt, or None when satisfied.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub weakest_axis: Option<String>,
    /// One sentence justification. Empty when the model declines to elaborate
    /// (often the case on a "satisfied" verdict).
    #[serde(default, deserialize_with = "deserialize_nullable_string_to_empty")]
    pub rationale: String,
    /// The literal delta string to pass to `refine::refine`. None when satisfied.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub proposed_delta: Option<String>,
}

fn deserialize_nullable_string<'de, D>(d: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = Option::<String>::deserialize(d)?;
    Ok(v.and_then(|s| {
        let t = s.trim();
        if t.is_empty() || t == "null" || t == "None" {
            None
        } else {
            Some(s)
        }
    }))
}

fn deserialize_nullable_string_to_empty<'de, D>(d: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(Option::<String>::deserialize(d)?.unwrap_or_default())
}

impl SelfCritique {
    pub fn satisfied(&self) -> bool {
        self.satisfaction >= 0.85 || self.proposed_delta.is_none()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CriticError {
    #[error("provider: {0}")]
    Provider(#[from] ProviderError),
    #[error("critic returned invalid JSON: {0}")]
    BadJson(String),
}

/// Run one critic pass. The caller controls the loop.
///
/// `skill_scaffold` is an optional pre-rendered block of relevant skill bodies
/// (from `aphrodite_core::skills::build_scaffold_block`). When non-empty, it's
/// prepended to the system prompt so the critic has concrete checklists for
/// the intent's domain instead of judging in a vacuum.
pub async fn critique(
    resolved: &ResolvedProvider,
    intent: &str,
    design_md: &str,
    composition_html: &str,
    prior_deltas: &[String],
    taste_hint: &str,
    skill_scaffold: &str,
) -> Result<SelfCritique, CriticError> {
    let history_block = if prior_deltas.is_empty() {
        "(no prior refinement turns)".to_string()
    } else {
        prior_deltas
            .iter()
            .enumerate()
            .map(|(i, d)| format!("  turn {}: {}", i + 1, d))
            .collect::<Vec<_>>()
            .join("\n")
    };

    // Cap composition size to keep token budget bounded — composer pages
    // can run 1k+ lines and the structural signal is in the first 200ish.
    let composition_excerpt = excerpt(composition_html, 8_000);

    let system_prompt = if skill_scaffold.is_empty() {
        CRITIC_SYSTEM_PROMPT.to_string()
    } else {
        format!("{CRITIC_SYSTEM_PROMPT}\n\n{skill_scaffold}\n\nUse the scaffolded checklists above when picking the weakest axis — a violation of a scaffolded rule outranks an aesthetic instinct.")
    };

    let user = format!(
        "INTENT:\n{intent}\n\n\
         CURRENT DESIGN.md:\n----- BEGIN DESIGN.md -----\n{design_md}\n----- END DESIGN.md -----\n\n\
         CURRENT composition.html (excerpt up to 8k chars):\n----- BEGIN composition.html -----\n{composition_excerpt}\n----- END composition.html -----\n\n\
         PRIOR REFINEMENT TURNS (oldest first):\n{history_block}\n\n\
         USER TASTE PROFILE:\n{taste_hint}\n\n\
         Now emit the JSON verdict per the contract above."
    );

    let raw = provider::call_raw(resolved, &system_prompt, &user, 1024).await?;
    parse_verdict(&raw)
}

fn excerpt(s: &str, max_chars: usize) -> String {
    if s.len() <= max_chars {
        s.to_string()
    } else {
        let mut out = s.chars().take(max_chars).collect::<String>();
        out.push_str("\n…[truncated]");
        out
    }
}

/// Lenient JSON parse — tolerates code fences and surrounding prose because
/// providers are inconsistent about pure-JSON output.
pub fn parse_verdict(raw: &str) -> Result<SelfCritique, CriticError> {
    let stripped = strip_fences(raw.trim());
    // Find the outermost {...} balanced span.
    let json_blob = extract_json_object(stripped).ok_or_else(|| {
        CriticError::BadJson(format!(
            "no balanced JSON object in critic response: {}",
            preview(raw)
        ))
    })?;
    let mut v: SelfCritique = serde_json::from_str(json_blob)
        .map_err(|e| CriticError::BadJson(format!("{e}: {}", preview(raw))))?;
    // Normalise: empty-string axis/delta → None.
    if v.weakest_axis.as_deref().map(str::trim).unwrap_or("").is_empty()
        || matches!(v.weakest_axis.as_deref(), Some("null") | Some("None"))
    {
        v.weakest_axis = None;
    }
    if let Some(d) = v.proposed_delta.as_ref() {
        if d.trim().is_empty() || d.trim() == "null" {
            v.proposed_delta = None;
        }
    }
    v.satisfaction = v.satisfaction.clamp(0.0, 1.0);
    Ok(v)
}

fn strip_fences(s: &str) -> &str {
    let s = s.trim();
    if let Some(rest) = s.strip_prefix("```json") {
        return rest.trim_end_matches("```").trim();
    }
    if let Some(rest) = s.strip_prefix("```") {
        return rest.trim_end_matches("```").trim();
    }
    s
}

fn extract_json_object(s: &str) -> Option<&str> {
    let start = s.find('{')?;
    let bytes = s.as_bytes();
    let mut depth = 0i32;
    let mut in_str = false;
    let mut escape = false;
    for (i, &b) in bytes.iter().enumerate().skip(start) {
        let c = b as char;
        if in_str {
            if escape {
                escape = false;
            } else if c == '\\' {
                escape = true;
            } else if c == '"' {
                in_str = false;
            }
            continue;
        }
        match c {
            '"' => in_str = true,
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&s[start..=i]);
                }
            }
            _ => {}
        }
    }
    None
}

fn preview(s: &str) -> String {
    let p: String = s.chars().take(180).collect();
    if s.chars().count() > 180 {
        format!("{p}…")
    } else {
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_clean_json() {
        let raw = r#"{"satisfaction":0.6,"weakest_axis":"typography","rationale":"EB Garamond is wrong register","proposed_delta":"switch display to Source Serif 4"}"#;
        let v = parse_verdict(raw).unwrap();
        assert_eq!(v.satisfaction, 0.6);
        assert_eq!(v.weakest_axis.as_deref(), Some("typography"));
        assert!(!v.satisfied());
    }

    #[test]
    fn parses_with_code_fence() {
        let raw = "```json\n{\"satisfaction\":0.9,\"weakest_axis\":null,\"rationale\":\"ok\",\"proposed_delta\":null}\n```";
        let v = parse_verdict(raw).unwrap();
        assert!(v.satisfied());
        assert_eq!(v.weakest_axis, None);
    }

    #[test]
    fn parses_with_surrounding_prose() {
        let raw = "Sure, here is my verdict:\n{\"satisfaction\":0.4,\"weakest_axis\":\"spacing\",\"rationale\":\"breathe\",\"proposed_delta\":\"push max spacing to 200px\"}\nLet me know if you want more.";
        let v = parse_verdict(raw).unwrap();
        assert_eq!(v.weakest_axis.as_deref(), Some("spacing"));
        assert_eq!(v.proposed_delta.as_deref(), Some("push max spacing to 200px"));
    }

    #[test]
    fn empty_string_axis_becomes_none() {
        let raw = r#"{"satisfaction":0.95,"weakest_axis":"","rationale":"","proposed_delta":""}"#;
        let v = parse_verdict(raw).unwrap();
        assert_eq!(v.weakest_axis, None);
        assert_eq!(v.proposed_delta, None);
        assert!(v.satisfied());
    }

    #[test]
    fn satisfaction_clamped() {
        let raw = r#"{"satisfaction":1.5,"weakest_axis":null,"rationale":"x","proposed_delta":null}"#;
        let v = parse_verdict(raw).unwrap();
        assert_eq!(v.satisfaction, 1.0);
    }

    #[test]
    fn rejects_non_json() {
        let raw = "I don't know how to evaluate this design.";
        assert!(parse_verdict(raw).is_err());
    }

    #[test]
    fn satisfied_when_no_delta_even_if_score_low() {
        // Useful when the critic identifies an axis but can't articulate a
        // surgical delta — treated as a stop signal so we don't loop forever.
        let raw = r#"{"satisfaction":0.5,"weakest_axis":"harmony","rationale":"vague","proposed_delta":null}"#;
        let v = parse_verdict(raw).unwrap();
        assert!(v.satisfied());
    }

    #[test]
    fn parses_all_null_rationale_and_axis() {
        // Pass 8 regression: critic emitted {satisfaction:0.88, weakest_axis:null,
        // rationale:null, proposed_delta:null}. Previously failed because
        // `rationale` was declared `String`, not `Option<String>`. The semantic
        // intent — "satisfied, nothing to say" — should parse and stop the loop.
        let raw = r#"{"satisfaction":0.88,"weakest_axis":null,"rationale":null,"proposed_delta":null}"#;
        let v = parse_verdict(raw).unwrap();
        assert!(v.satisfied());
        assert_eq!(v.weakest_axis, None);
        assert_eq!(v.proposed_delta, None);
        assert_eq!(v.rationale, "");
        assert!(v.satisfaction >= 0.85);
    }
}
