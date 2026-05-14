//! Phase 7 (harmonize) — final cross-file consistency pass.
//!
//! Runs AFTER the critic loop has settled. Pure post-processing of the
//! generated HTML; no LLM calls. Closes Findings #24 and #26.
//!
//! Responsibilities:
//!   1. Inject Google Fonts `@import` (or `<link>`) for the declared display
//!      + body families so a fresh browser actually loads the chosen fonts.
//!   2. Replace hero.html's generic `ui-sans-serif` system fallback with the
//!      same CSS-variable typography hookup the composition uses.
//!   3. Surface a small report telling the caller what was injected.
//!
//! Anti-design choices worth flagging:
//!   - We DON'T attempt to parse arbitrary HTML structure with regex magic;
//!     all injection is on `</head>` close-tag and `font-family:` declarations
//!     known to come from our own templates. Foreign HTML is left alone.
//!   - We DON'T fetch the actual font file — just emit the `<link>` and let
//!     the user's browser decide. This keeps harmonize purely synchronous.

use aphrodite_core::DesignDocument;

#[derive(Debug, Clone, Default)]
pub struct HarmonizeReport {
    /// Families we generated a Google Fonts link for.
    pub fonts_injected: Vec<String>,
    /// True if the hero CSS was rewritten to use typography CSS vars.
    pub hero_typography_fixed: bool,
    /// Non-fatal diagnostics — kept terse, the caller decides what to surface.
    pub notes: Vec<String>,
}

/// Run the full harmonize pass over a (composition.html, hero.html) pair.
/// Takes the raw `design_md` text so font extraction is independent of the
/// DesignDocument's internal YAML→JSON shape (which varies across versions).
pub fn harmonize(
    composition_html: &str,
    hero_html: &str,
    design_md: &str,
    _design: &DesignDocument,
) -> (String, String, HarmonizeReport) {
    let mut report = HarmonizeReport::default();

    let (display, body) = extract_families_from_md(design_md);
    let google_url = google_fonts_url(&[display.clone(), body.clone()]);
    let link_tag = google_url
        .as_deref()
        .map(|u| format!(r#"<link rel="preconnect" href="https://fonts.googleapis.com"><link rel="preconnect" href="https://fonts.gstatic.com" crossorigin><link rel="stylesheet" href="{u}">"#));

    let composition_out = match link_tag.as_deref() {
        Some(tag) if !composition_html.contains("fonts.googleapis.com") => {
            inject_into_head(composition_html, tag)
        }
        _ => composition_html.to_string(),
    };
    let hero_after_fonts = match link_tag.as_deref() {
        Some(tag) if !hero_html.contains("fonts.googleapis.com") => {
            inject_into_head(hero_html, tag)
        }
        _ => hero_html.to_string(),
    };

    if google_url.is_some() {
        if let Some(d) = display.as_ref() {
            if !is_system_family(d) {
                report.fonts_injected.push(d.clone());
            }
        }
        if let Some(b) = body.as_ref() {
            if !is_system_family(b) && Some(b) != display.as_ref() {
                report.fonts_injected.push(b.clone());
            }
        }
    }

    let (hero_out, hero_changed) =
        fix_hero_typography(&hero_after_fonts, display.as_deref(), body.as_deref());
    report.hero_typography_fixed = hero_changed;

    if !report.fonts_injected.is_empty() && composition_html.contains("fonts.googleapis.com") {
        report
            .notes
            .push("composition.html already had a Google Fonts link; skipped re-injection".into());
    }

    (composition_out, hero_out, report)
}

/// Extract display + body family names from raw DESIGN.md frontmatter.
/// Looks for the canonical shape:
///   typography:
///     display:
///       family: "Newsreader"
///     body:
///       family: "Outfit"
/// Tolerates single quotes, no quotes, and varying indentation.
pub fn extract_families_from_md(design_md: &str) -> (Option<String>, Option<String>) {
    let mut display: Option<String> = None;
    let mut body: Option<String> = None;
    let mut in_typography = false;
    let mut in_display = false;
    let mut in_body = false;

    for line in design_md.lines() {
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();
        if trimmed.starts_with("typography:") {
            in_typography = true;
            in_display = false;
            in_body = false;
            continue;
        }
        if in_typography {
            // Leaving the typography block: any top-level key resets us.
            if indent == 0 && !trimmed.is_empty() && !trimmed.starts_with('#') {
                in_typography = false;
                in_display = false;
                in_body = false;
                continue;
            }
            if trimmed.starts_with("display:") {
                in_display = true;
                in_body = false;
                continue;
            }
            if trimmed.starts_with("body:") {
                in_body = true;
                in_display = false;
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("family:") {
                let value = parse_yaml_scalar(rest.trim());
                if in_display && display.is_none() {
                    display = Some(value);
                } else if in_body && body.is_none() {
                    body = Some(value.clone());
                }
            }
        }
        // Frontmatter closes at the second `---`; we just keep scanning the
        // whole file but never match (typography won't appear in body prose).
    }
    (display, body)
}

fn parse_yaml_scalar(s: &str) -> String {
    let s = s.trim();
    if let Some(inner) = s.strip_prefix('"').and_then(|r| r.strip_suffix('"')) {
        return inner.to_string();
    }
    if let Some(inner) = s.strip_prefix('\'').and_then(|r| r.strip_suffix('\'')) {
        return inner.to_string();
    }
    s.to_string()
}

fn is_system_family(family: &str) -> bool {
    let lower = family.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "system-ui"
            | "ui-sans-serif"
            | "ui-serif"
            | "ui-monospace"
            | "ui-rounded"
            | "sans-serif"
            | "serif"
            | "monospace"
            | "-apple-system"
            | "blinkmacsystemfont"
            | "segoe ui"
            | "helvetica"
            | "arial"
            | "georgia"
            | "times new roman"
    )
}

/// Build a single Google Fonts CSS2 URL for the given families. Skips system
/// families. Returns None if no remote-loadable family remains.
fn google_fonts_url(families: &[Option<String>]) -> Option<String> {
    let mut seen: Vec<String> = Vec::new();
    for fam in families.iter().flatten() {
        if is_system_family(fam) {
            continue;
        }
        if !seen.iter().any(|s| s.eq_ignore_ascii_case(fam)) {
            seen.push(fam.clone());
        }
    }
    if seen.is_empty() {
        return None;
    }
    let parts: Vec<String> = seen
        .into_iter()
        .map(|name| {
            // Source Serif 4 ships with an optical-sizing axis; opt in
            // generically. For other families, request a reasonable weight set.
            let plus = name.replace(' ', "+");
            if name.to_ascii_lowercase().contains("source serif") || name.to_ascii_lowercase().contains("fraunces") || name.to_ascii_lowercase().contains("newsreader") {
                format!("family={plus}:opsz,wght@8..60,400;8..60,500;8..60,700")
            } else {
                format!("family={plus}:wght@300;400;500;600;700")
            }
        })
        .collect();
    Some(format!(
        "https://fonts.googleapis.com/css2?{}&display=swap",
        parts.join("&")
    ))
}

fn inject_into_head(html: &str, tag: &str) -> String {
    // Try lowercase `</head>` first, then any-case via find_ignore_ascii_case.
    if let Some(idx) = html.find("</head>") {
        let mut out = String::with_capacity(html.len() + tag.len() + 1);
        out.push_str(&html[..idx]);
        out.push_str(tag);
        out.push('\n');
        out.push_str(&html[idx..]);
        return out;
    }
    if let Some(idx) = find_ignore_ascii_case(html, "</head>") {
        let mut out = String::with_capacity(html.len() + tag.len() + 1);
        out.push_str(&html[..idx]);
        out.push_str(tag);
        out.push('\n');
        out.push_str(&html[idx..]);
        return out;
    }
    // No <head> — prepend at top inside <html>. Last-resort attach.
    if let Some(idx) = html.find("<body") {
        let mut out = String::with_capacity(html.len() + tag.len() + 12);
        out.push_str(&html[..idx]);
        out.push_str("<head>");
        out.push_str(tag);
        out.push_str("</head>\n");
        out.push_str(&html[idx..]);
        return out;
    }
    // Give up — concatenate. Better than dropping the tag.
    format!("{tag}\n{html}")
}

fn find_ignore_ascii_case(haystack: &str, needle: &str) -> Option<usize> {
    let h = haystack.as_bytes();
    let n = needle.as_bytes();
    if n.is_empty() || h.len() < n.len() {
        return None;
    }
    'outer: for i in 0..=(h.len() - n.len()) {
        for (j, &nb) in n.iter().enumerate() {
            if !h[i + j].eq_ignore_ascii_case(&nb) {
                continue 'outer;
            }
        }
        return Some(i);
    }
    None
}

/// Replace the generic `font-family: ui-sans-serif, …;` declaration the hero
/// template ships with one that consumes the design-tokens' typography vars
/// the composition already exposes. Returns `(html_out, changed)`.
fn fix_hero_typography(html: &str, display: Option<&str>, body: Option<&str>) -> (String, bool) {
    // The hero template emits at most one `font-family: ui-sans-serif,…sans-serif;`
    // on the body selector. We look for that exact prefix and replace through
    // the trailing semicolon.
    let needle = "font-family: ui-sans-serif";
    let Some(start) = html.find(needle) else {
        return (html.to_string(), false);
    };
    let Some(rel_end) = html[start..].find(';') else {
        return (html.to_string(), false);
    };
    let end = start + rel_end + 1;

    let body_name = body.unwrap_or("system-ui");
    let display_name = display.unwrap_or(body_name);
    let replacement = format!(
        r#"font-family: "{body_name}", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif;
    --aphrodite-typography-display: "{display_name}", "{body_name}", Georgia, "Times New Roman", serif;
    --aphrodite-typography-body: "{body_name}", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif;"#
    );

    let mut out = String::with_capacity(html.len() + replacement.len());
    out.push_str(&html[..start]);
    out.push_str(&replacement);
    out.push_str(&html[end..]);
    (out, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fonts_url_skips_system_families() {
        let url = google_fonts_url(&[Some("system-ui".into()), Some("Inter".into())]).unwrap();
        assert!(url.contains("Inter"));
        assert!(!url.contains("system-ui"));
    }

    #[test]
    fn fonts_url_uses_opsz_for_source_serif_4() {
        let url = google_fonts_url(&[Some("Source Serif 4".into())]).unwrap();
        assert!(url.contains("opsz,wght@"));
        assert!(url.contains("Source+Serif+4"));
    }

    #[test]
    fn fonts_url_none_when_all_system() {
        assert!(google_fonts_url(&[Some("system-ui".into()), Some("sans-serif".into())]).is_none());
    }

    #[test]
    fn inject_into_head_finds_close_tag() {
        let html = "<html><head><title>x</title></head><body>hi</body></html>";
        let out = inject_into_head(html, "<link rel=stylesheet>");
        assert!(out.contains("<link rel=stylesheet>"));
        let link_idx = out.find("<link").unwrap();
        let close_idx = out.find("</head>").unwrap();
        assert!(link_idx < close_idx);
    }

    #[test]
    fn inject_into_head_handles_missing_head() {
        let html = "<html><body>only body</body></html>";
        let out = inject_into_head(html, "<link rel=stylesheet>");
        assert!(out.contains("<head>"));
        assert!(out.contains("<link rel=stylesheet>"));
        assert!(out.contains("</head>"));
    }

    #[test]
    fn hero_typography_replaces_system_fallback() {
        let hero = "body { font-family: ui-sans-serif, system-ui, -apple-system, sans-serif; margin: 0; }";
        let (out, changed) = fix_hero_typography(hero, Some("Newsreader"), Some("Outfit"));
        assert!(changed);
        assert!(out.contains("\"Outfit\""));
        assert!(out.contains("--aphrodite-typography-display"));
        assert!(!out.contains("font-family: ui-sans-serif"));
    }

    #[test]
    fn hero_typography_idempotent_when_no_match() {
        let hero = "body { font-family: \"Inter\", sans-serif; }";
        let (out, changed) = fix_hero_typography(hero, Some("Newsreader"), Some("Inter"));
        assert!(!changed);
        assert_eq!(out, hero);
    }

    #[test]
    fn extract_families_from_md_canonical_shape() {
        let md = r#"---
name: x
typography:
  display:
    family: "Newsreader"
    weight: 400
  body:
    family: "Outfit"
    weight: 300
spacing:
  "1": "4px"
---

# Body
"#;
        let (d, b) = extract_families_from_md(md);
        assert_eq!(d.as_deref(), Some("Newsreader"));
        assert_eq!(b.as_deref(), Some("Outfit"));
    }

    #[test]
    fn extract_families_from_md_handles_single_quotes_and_unquoted() {
        let md = r#"---
typography:
  display:
    family: 'Source Serif 4'
  body:
    family: Inter
---
"#;
        let (d, b) = extract_families_from_md(md);
        assert_eq!(d.as_deref(), Some("Source Serif 4"));
        assert_eq!(b.as_deref(), Some("Inter"));
    }
}
