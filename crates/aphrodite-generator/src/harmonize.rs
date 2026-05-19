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
    /// Lucide icon slugs we re-labelled by path-data fingerprint matching.
    pub lucide_labels_recovered: Vec<String>,
    /// Quality warnings — visible-from-HTML problems we found but didn't fix
    /// (no h1, no section tags, fewer fonts loaded than declared, etc).
    /// These flag production-readiness concerns without auto-mutating.
    pub quality_warnings: Vec<String>,
    /// Non-fatal diagnostics — kept terse, the caller decides what to surface.
    pub notes: Vec<String>,
    /// v0.8 Aphrodite Quality Score (0-100). Composite of accessibility,
    /// mobile-first, performance, and semantic-HTML axes. ≥ 85 = ship.
    pub quality_score: u32,
    /// Per-axis breakdown (a11y / mobile / perf / semantic), 0-100 each.
    pub quality_axes: QualityAxes,
}

#[derive(Debug, Clone, Default)]
pub struct QualityAxes {
    pub a11y: u32,
    pub mobile: u32,
    pub performance: u32,
    pub semantic: u32,
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

    // Finding #32 full close: walk composition's <svg> tags. Any whose
    // first `d="..."` matches a known Lucide path fingerprint and which
    // does NOT already carry `class="lucide lucide-<name>"` gets relabeled.
    let (composition_out_labeled, recovered) = recover_lucide_classes(&composition_out);
    report.lucide_labels_recovered = recovered;

    if !report.fonts_injected.is_empty() && composition_html.contains("fonts.googleapis.com") {
        report
            .notes
            .push("composition.html already had a Google Fonts link; skipped re-injection".into());
    }

    // Auto-fix safe quality issues before audit, so the audit reports
    // what's *really* still wrong rather than what we already fixed.
    let composition_post_fix = auto_fix_h1_count(&composition_out_labeled);
    let composition_post_fix = fix_broken_img_placeholders(&composition_post_fix);
    // Pass 55+ security: <video> and <audio> are higher-stakes than <img>
    // for external-URL leaks (autoplay, autobuffer can pull MBs without
    // user action, plus video CDNs commonly track). Strip any
    // <video src="https://..."> or <audio src="..."> unless the tag
    // carries `data-aphrodite-asset-verified`.
    let composition_post_fix = strip_unverified_external_media(&composition_post_fix);
    // Pass 43 surfaced: composer routinely sets `.hero { min-height: 80vh }`
    // assuming a full-bleed photograph. When the actual hero content is a
    // placeholder figure (no real asset shipped yet), 80vh becomes 1500+px
    // of dead vertical space. Cap section min-heights when placeholders
    // are present in the composition.
    // Count viewport-height violations BEFORE the cap fires, so the audit
    // can surface "composer ignored prompt-level placeholder-height ban".
    let placeholder_present = composition_post_fix.contains("class=\"image-placeholder\"");
    let vh_violations_pre_fix = if placeholder_present {
        count_viewport_height_violations(&composition_post_fix)
    } else {
        0
    };
    // v0.8 polish: Korean pages need `word-break: keep-all` on display
    // headings (otherwise long Hangul phrases overflow at narrow mobile
    // viewports), and the variant switcher needs to wrap rather than
    // clip when there isn't horizontal room. Inject a small CSS shim if
    // Hangul is present and these rules are not already declared. The
    // shim goes inside an existing <style> block if one exists, else
    // wraps in its own <style>.
    let composition_post_fix =
        inject_korean_layout_shim(&composition_post_fix);
    // Locale auto-detect: if the composition contains Hangul code points
    // but `<html lang="en">` (or no lang attribute), patch the tag to
    // `lang="ko"`. The composer's hardcoded default is "en" inherited
    // from the hero.rs template; this is a v0.7 i18n correction.
    let composition_post_fix = patch_lang_attribute(&composition_post_fix);
    let composition_post_fix = if placeholder_present {
        let capped = cap_vh_property(&composition_post_fix, "min-height", 40);
        let capped = cap_vh_property(&capped, "height", 60);
        // Pass 44 surfaced: composer authored figures directly with
        // `height: calc(100vw * 1.25)` (1800px at 1440px viewport) — no
        // `vh` units at all. Enforce a max-height on every placeholder
        // figure regardless of which path produced it.
        let with_fig_cap = ensure_placeholder_max_height(&capped, 720);
        // Composer also writes `.hero { height: calc(100vw * 1.25) }` etc
        // on the *section* surrounding the placeholder. At 1440px viewport
        // that's 1800px of empty section, dwarfing the now-capped figure.
        // Inject a max-height alongside every `height: calc(100v...)` so
        // the section can't outgrow the placeholder it contains.
        clamp_calc_viewport_heights(&with_fig_cap, 720)
    } else {
        composition_post_fix
    };
    // Production-readiness audit on the FIXED composition (read-only after this point).
    report.quality_warnings = audit_composition(&composition_post_fix, display.as_deref(), body.as_deref());
    if vh_violations_pre_fix > 0 {
        report.quality_warnings.push(format!(
            "{vh_violations_pre_fix} viewport-relative height rule(s) around placeholder figures — composer ignored prompt-level ban; harmonize capped them, but if this warning persists across runs the prompt may need tightening."
        ));
    }
    // v0.8: composite Aphrodite Quality Score.
    let (axes, score) = score_quality(&composition_post_fix, &report.quality_warnings);
    report.quality_axes = axes;
    report.quality_score = score;

    (composition_post_fix, hero_out, report)
}

/// Compute the v0.8 Aphrodite Quality Score and per-axis breakdown.
/// 0-100 each axis, composite is the average. Calibrated so a Pass 46
/// clean run lands around 90+ and a Pass 45 broken run lands < 60.
fn score_quality(html: &str, warnings: &[String]) -> (QualityAxes, u32) {
    // Accessibility: starts at 100, -10 per a11y-flavoured warning.
    let a11y_warning_keywords = [
        "alt=", "lang", "viewport", "aria", "contrast", "<h1>", "heading hierarchy",
    ];
    let a11y_hits: u32 = warnings
        .iter()
        .filter(|w| a11y_warning_keywords.iter().any(|k| w.contains(k)))
        .count() as u32;
    let a11y = 100u32.saturating_sub(a11y_hits.saturating_mul(15));

    // Mobile: 50 base, +25 if @media (min-width) present, +25 if Korean
    // viewport meta tag explicitly declared.
    let mut mobile = 50u32;
    if html.contains("@media (min-width") || html.contains("@media(min-width") {
        mobile += 25;
    }
    if html.contains("name=\"viewport\"") || html.contains("name='viewport'") {
        mobile += 25;
    }

    // Performance heuristic: total HTML byte size. Under 30 KB = 100,
    // 30-80 KB = 90, 80-150 KB = 75, > 150 KB = 60. (Hand-tuned vs
    // observed Pass 30-48 distribution.)
    let bytes = html.len();
    let performance = if bytes < 30_000 {
        100
    } else if bytes < 80_000 {
        90
    } else if bytes < 150_000 {
        75
    } else {
        60
    };

    // Semantic: 1 h1 + nav + footer + ≥ 2 major regions each worth 25.
    // v1.0 RC.8: <article> counts toward the region budget too — Junjong
    // editorial layout used <article>+<main>+<aside> instead of nested
    // <section> tags, which is equally semantic but the old metric was
    // section-only. Accept any of section/article as a major region.
    let mut semantic = 0u32;
    if html.matches("<h1").count() == 1 {
        semantic += 25;
    }
    if html.contains("<nav") {
        semantic += 25;
    }
    if html.contains("<footer") {
        semantic += 25;
    }
    let regions = html.matches("<section").count() + html.matches("<article").count();
    if regions >= 2 {
        semantic += 25;
    }

    let composite = (a11y + mobile + performance + semantic) / 4;
    (
        QualityAxes {
            a11y,
            mobile,
            performance,
            semantic,
        },
        composite,
    )
}

/// Walk `<video>` and `<audio>` tags. Replace any with an external `src`
/// (or with `<source src="https://...">` children) that lacks the
/// `data-aphrodite-asset-verified` opt-out with a styled placeholder
/// `<figure>`. Pass 55+ security: video/audio external URLs are higher
/// bandwidth + tracking risk than images, so we go stricter — any
/// remote src triggers the swap.
fn strip_unverified_external_media(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut cursor = 0usize;
    while let Some(rel) = html[cursor..].find("<video").or_else(|| html[cursor..].find("<audio")) {
        let start = cursor + rel;
        out.push_str(&html[cursor..start]);
        // Determine which tag we matched.
        let (tag, kind_label) = if html[start..].starts_with("<video") {
            ("video", "video")
        } else {
            ("audio", "audio")
        };
        let close_marker = format!("</{tag}>");
        let close_rel = match html[start..].find(close_marker.as_str()) {
            Some(c) => c,
            None => {
                out.push_str(&html[start..]);
                return out;
            }
        };
        let block = &html[start..start + close_rel + close_marker.len()];
        let opt_out = block.contains("data-aphrodite-asset-verified");
        let has_remote = block.contains("src=\"http://")
            || block.contains("src=\"https://")
            || block.contains("src='http://")
            || block.contains("src='https://");
        if has_remote && !opt_out {
            let label = pick_attr(block, "aria-label")
                .or_else(|| pick_attr(block, "title"))
                .unwrap_or_else(|| kind_label.to_string());
            out.push_str(&format!(
                r#"<figure class="image-placeholder" style="aspect-ratio: 16/9; max-width: 720px; max-height: 405px; margin: 0 auto; background: var(--colors-primary-100, #f0ead8); display: flex; align-items: center; justify-content: center; color: var(--colors-text-muted, #71717a); font-size: 13px; padding: 24px; text-align: center; border-radius: 4px;">[{kind_label}: {label}]</figure>"#
            ));
        } else {
            out.push_str(block);
        }
        cursor = start + close_rel + close_marker.len();
    }
    out.push_str(&html[cursor..]);
    out
}

/// Walk `<img>` tags. If src is empty / fake / placeholder-shaped, replace
/// the whole `<img>` with a styled `<figure>` placeholder. Hand-rolled
/// scan to avoid an extra crate dep.
///
/// Closes the "broken image icon" visual bug surfaced by Pass 39 review.
pub fn fix_broken_img_placeholders(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut cursor = 0usize;
    while let Some(rel) = html[cursor..].find("<img") {
        let img_start = cursor + rel;
        out.push_str(&html[cursor..img_start]);
        let end_rel = match html[img_start..].find('>') {
            Some(i) => i,
            None => {
                out.push_str(&html[img_start..]);
                return out;
            }
        };
        let tag_end = img_start + end_rel + 1;
        let img_tag = &html[img_start..tag_end];
        let src = pick_attr(img_tag, "src").unwrap_or_default();
        let alt = pick_attr(img_tag, "alt").unwrap_or_default();
        let src_trim = src.trim();
        // Pass 45 surfaced: composer wrote `<img src="/img/dining-table.jpg">`
        // pointing at nonexistent paths. The narrow "fake URL" heuristic
        // didn't catch them, so the page rendered broken-image icons.
        // Treat any LOCAL relative path as a placeholder unless the
        // caller verifies the asset exists (a future asset-validate phase
        // could opt out by setting `data-aphrodite-asset-verified`).
        let is_local_path = !src_trim.starts_with("http://")
            && !src_trim.starts_with("https://")
            && !src_trim.starts_with("data:")
            && !src_trim.is_empty();
        // Pass 50 Banchan surfaced: composer used a stock unsplash-style
        // URL that happened to load — a 1.77MB hero image. Even when
        // such an URL resolves, it's not an asset the user authored, and
        // it leaks an external dependency into a brand surface. Treat
        // every `<img>` (local OR external) as a placeholder unless it
        // carries `data-aphrodite-asset-verified`. Users who actually
        // want a real image add the opt-out attribute; the default
        // behaviour is safe.
        let is_unverified_remote = !is_local_path
            && !src_trim.is_empty()
            && !src_trim.starts_with("data:")
            && !img_tag.contains("data-aphrodite-asset-verified");
        let is_placeholder = src_trim.is_empty()
            || src_trim.starts_with('#')
            || src_trim.starts_with("[photo")
            || src_trim.starts_with("[image")
            || src_trim.starts_with("placeholder")
            || src_trim.contains("example.com")
            || src_trim.contains("placehold.")
            || src_trim.contains("via.placeholder")
            || src_trim.contains("picsum.photos")
            || (is_local_path && !img_tag.contains("data-aphrodite-asset-verified"))
            || is_unverified_remote;
        if is_placeholder {
            let label = if !alt.is_empty() { alt } else { "photo".to_string() };
            out.push_str(&format!(
                r#"<figure class="image-placeholder" style="aspect-ratio: 4/5; max-width: 480px; max-height: 600px; margin: 0 auto; background: var(--colors-primary-100, #f0ead8); display: flex; align-items: center; justify-content: center; color: var(--colors-text-muted, #71717a); font-size: 13px; padding: 24px; text-align: center; border-radius: 4px;">[photo: {label}]</figure>"#
            ));
        } else {
            out.push_str(img_tag);
        }
        cursor = tag_end;
    }
    out.push_str(&html[cursor..]);
    out
}

fn pick_attr(tag: &str, attr: &str) -> Option<String> {
    let needle = format!("{attr}=\"");
    if let Some(i) = tag.find(&needle) {
        let after = &tag[i + needle.len()..];
        if let Some(end) = after.find('"') {
            return Some(after[..end].to_string());
        }
    }
    let needle_sq = format!("{attr}='");
    if let Some(i) = tag.find(&needle_sq) {
        let after = &tag[i + needle_sq.len()..];
        if let Some(end) = after.find('\'') {
            return Some(after[..end].to_string());
        }
    }
    None
}

/// If composition has > 1 `<h1>` tags, keep the FIRST one (typically the
/// page hero) and downgrade all others to `<h2>`. This is a safe transform
/// because the page-level hero should be h1 and section headers should be
/// h2. Pass 39 surfaced this — composer emitted h1 per section.
/// Inject a minimal `word-break: keep-all` + variant-switcher wrap shim
/// when the composition contains Hangul. Idempotent — bails if the
/// shim's marker is already present. Solves the Pass 47/48 mobile h1
/// overflow + brand-a/brand-b switcher clipping issues.
fn inject_korean_layout_shim(html: &str) -> String {
    let has_hangul = html.chars().any(is_korean_char);
    if !has_hangul {
        return html.to_string();
    }
    let marker = "aphrodite-ko-shim";
    if html.contains(marker) {
        return html.to_string();
    }
    let shim = format!(
        "\n<style data-aphrodite-ko-shim=\"{marker}\">\n\
         h1, h2, h3, .hero h1 {{ word-break: keep-all; overflow-wrap: anywhere; }}\n\
         /* Korean grid-item collapse: when a multi-column grid is too narrow per cell,\n\
            word-break: keep-all causes one-char-per-line vertical wrap. Force a sensible\n\
            min-width per grid item so cells either fit horizontally or the grid drops to\n\
            a single column at narrow viewports. Pass 50 Banchan pricing surfaced this. */\n\
         [class*=\"grid\"] > *, [class*=\"pricing\"] > *, [class*=\"plans\"] > *,\n\
         [class*=\"tiers\"] > *, [class*=\"cards\"] > * {{ min-width: 0; }}\n\
         @media (max-width: 768px) {{\n\
         [style*=\"grid-template-columns\"] {{ grid-template-columns: 1fr !important; }}\n\
         }}\n\
         /* Mobile body padding safety net — Pass 51 Banchan home surfaced text\n\
            running off the right edge at 360px. Apply edge-padding on the body\n\
            ONLY when no other padding rule wins (lowest specificity, no \\!important). */\n\
         @media (max-width: 600px) {{\n\
           body {{ padding-left: max(16px, env(safe-area-inset-left)); padding-right: max(16px, env(safe-area-inset-right)); }}\n\
         }}\n\
         /* Narrow card / tier body text: allow standard word-break so short Korean\n\
            phrases (선택하기, 20가지 반찬) don't wrap mid-word inside a narrow column. */\n\
         [class*=\"tier\"] p, [class*=\"plan\"] p, [class*=\"card\"] p,\n\
         [class*=\"tier\"] li, [class*=\"plan\"] li, [class*=\"card\"] li,\n\
         button {{ word-break: normal; overflow-wrap: break-word; }}\n\
         .aphrodite-variant-switcher {{ flex-wrap: wrap; max-width: calc(100vw - 32px); }}\n\
         @media (max-width: 480px) {{\n\
         .aphrodite-variant-switcher {{ position: static !important; margin: 8px 16px; }}\n\
         }}\n\
         </style>\n"
    );
    if let Some(idx) = html.find("</head>") {
        let mut out = String::with_capacity(html.len() + shim.len());
        out.push_str(&html[..idx]);
        out.push_str(&shim);
        out.push_str(&html[idx..]);
        out
    } else if let Some(idx) = html.find("<body") {
        let mut out = String::with_capacity(html.len() + shim.len());
        out.push_str(&html[..idx]);
        out.push_str(&shim);
        out.push_str(&html[idx..]);
        out
    } else {
        format!("{shim}{html}")
    }
}

/// v0.7 i18n: detect Korean (Hangul / Jamo / CJK common) characters in
/// the composition body. If present, ensure `<html lang="ko">` is set.
/// Composer's default came from the hero template's hardcoded `lang="en"`
/// — that misclassifies a fully-Korean page to screen readers, SEO
/// crawlers, and browser translation prompts.
fn patch_lang_attribute(html: &str) -> String {
    let body_start = html.find("<body").unwrap_or(0);
    let body_text = &html[body_start..];
    let has_hangul = body_text.chars().any(is_korean_char);
    let has_japanese = body_text.chars().any(is_japanese_char);
    let target_lang = if has_hangul {
        "ko"
    } else if has_japanese {
        "ja"
    } else {
        return html.to_string();
    };
    // Find the existing <html lang="..."> if any and rewrite.
    if let Some(open_start) = html.find("<html") {
        if let Some(open_end_rel) = html[open_start..].find('>') {
            let open_end = open_start + open_end_rel;
            let opening = &html[open_start..open_end];
            // Rewrite an existing lang= attribute, else inject.
            let new_opening = if let Some(lang_pos) = opening.find("lang=\"") {
                let lang_value_start = lang_pos + "lang=\"".len();
                if let Some(close_rel) = opening[lang_value_start..].find('"') {
                    let close = lang_value_start + close_rel;
                    let current = &opening[lang_value_start..close];
                    if current == target_lang {
                        return html.to_string();
                    }
                    let mut new = String::with_capacity(opening.len());
                    new.push_str(&opening[..lang_value_start]);
                    new.push_str(target_lang);
                    new.push_str(&opening[close..]);
                    new
                } else {
                    opening.to_string()
                }
            } else {
                // No lang attribute — inject after `<html`.
                format!("<html lang=\"{target_lang}\"{}", &opening["<html".len()..])
            };
            let mut out = String::with_capacity(html.len());
            out.push_str(&html[..open_start]);
            out.push_str(&new_opening);
            out.push_str(&html[open_end..]);
            return out;
        }
    }
    html.to_string()
}

fn is_korean_char(c: char) -> bool {
    matches!(c as u32,
        0xAC00..=0xD7A3      // Hangul syllables
        | 0x1100..=0x11FF    // Hangul Jamo
        | 0x3130..=0x318F    // Hangul Compatibility Jamo
        | 0xA960..=0xA97F    // Hangul Jamo Extended-A
        | 0xD7B0..=0xD7FF    // Hangul Jamo Extended-B
    )
}

fn is_japanese_char(c: char) -> bool {
    matches!(c as u32,
        0x3040..=0x309F      // Hiragana
        | 0x30A0..=0x30FF    // Katakana
        | 0x31F0..=0x31FF    // Katakana Phonetic Extensions
    )
}

/// Count violations of the prompt-level "no viewport-relative heights
/// around placeholders" rule. Sees `height: NNvh` ≥ 70, `min-height:
/// NNvh` ≥ 70, and any `height: calc(100v...)` rule. Run BEFORE the
/// harmonize cap to capture the original composer output.
fn count_viewport_height_violations(html: &str) -> usize {
    let mut n = 0usize;
    for prop in ["height:", "min-height:"] {
        let mut cursor = 0usize;
        while let Some(rel) = html[cursor..].find(prop) {
            let i = cursor + rel;
            // Skip `height:` matches that are tails of `min-height:` /
            // `max-height:` — otherwise we'd double-count.
            if prop == "height:" && i >= 4 {
                let prev = &html[i - 4..i];
                if prev == "min-" || prev == "max-" {
                    cursor = i + prop.len();
                    continue;
                }
            }
            let tail = &html[i + prop.len()..];
            let tail = tail.trim_start();
            let num: String = tail.chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(v) = num.parse::<u32>() {
                if v >= 70 && tail[num.len()..].starts_with("vh") {
                    n += 1;
                }
            }
            cursor = i + prop.len();
        }
    }
    n += html.matches("height: calc(100v").count();
    n
}

/// Inject `max-height: <cap_px>px;` immediately after every
/// `height: calc(100v...);` declaration. Idempotent — skips declarations
/// that already have an adjacent `max-height`.
fn clamp_calc_viewport_heights(html: &str, cap_px: u32) -> String {
    let needle = "height: calc(100v";
    if !html.contains(needle) {
        return html.to_string();
    }
    let mut out = String::with_capacity(html.len() + 128);
    let mut cursor = 0usize;
    while let Some(rel) = html[cursor..].find(needle) {
        let i = cursor + rel;
        out.push_str(&html[cursor..i]);
        // Find the `;` ending this declaration.
        if let Some(semi_rel) = html[i..].find(';') {
            let semi = i + semi_rel + 1;
            out.push_str(&html[i..semi]);
            // Peek ahead to see if max-height is already present in the
            // immediately-following window (within ~120 chars of this rule).
            let look = &html[semi..(semi + 120).min(html.len())];
            if !look.contains("max-height:") {
                out.push_str(&format!(" max-height: {cap_px}px;"));
            }
            cursor = semi;
        } else {
            out.push_str(&html[i..]);
            return out;
        }
    }
    out.push_str(&html[cursor..]);
    out
}

/// Walk every `<figure class="image-placeholder" style="...">` and ensure
/// the inline `style` includes `max-height: <max_px>px`. If the style has
/// no max-height, append one. If it has a max-height greater than the
/// cap, rewrite it. Pass 44 surfaced: composer wrote
/// `height: calc(100vw * 1.25)` which produces 1800px-tall placeholders
/// at desktop viewport. Capping at 720px keeps placeholders generous
/// enough to read but stops them from eating the whole hero.
fn ensure_placeholder_max_height(html: &str, max_px: u32) -> String {
    let marker = r#"<figure class="image-placeholder""#;
    if !html.contains(marker) {
        return html.to_string();
    }
    let mut out = String::with_capacity(html.len() + 256);
    let mut cursor = 0usize;
    while let Some(rel) = html[cursor..].find(marker) {
        let i = cursor + rel;
        out.push_str(&html[cursor..i]);
        // Find the end of this opening tag.
        let tag_end = match html[i..].find('>') {
            Some(n) => i + n,
            None => {
                out.push_str(&html[i..]);
                return out;
            }
        };
        let tag = &html[i..tag_end + 1];
        let already_has_cap = tag.contains("max-height:") && {
            // crude check: if it already has max-height: NNpx with N <= cap, leave alone
            let after = tag.split("max-height:").nth(1).unwrap_or("");
            let n: u32 = after
                .chars()
                .skip_while(|c| c.is_whitespace())
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap_or(u32::MAX);
            n <= max_px
        };
        if already_has_cap {
            out.push_str(tag);
        } else if let Some(style_pos) = tag.find("style=\"") {
            let injected = format!("max-height: {max_px}px; ");
            let abs = i + style_pos + "style=\"".len();
            out.push_str(&html[i..abs]);
            out.push_str(&injected);
            out.push_str(&html[abs..tag_end + 1]);
        } else {
            // No style attribute — add one before `>`.
            let injected_attr = format!(" style=\"max-height: {max_px}px;\"");
            out.push_str(&html[i..tag_end]);
            out.push_str(&injected_attr);
            out.push('>');
        }
        cursor = tag_end + 1;
    }
    out.push_str(&html[cursor..]);
    out
}

/// Scan `<prop>: NNvh` declarations. Any value > `max_vh` is clamped to
/// `max_vh`. Used when the composition contains placeholder figures —
/// full-viewport heroes don't make sense without real artwork.
fn cap_vh_property(html: &str, prop: &str, max_vh: u32) -> String {
    let needle_owned = format!("{prop}:");
    let needle = needle_owned.as_str();
    if !html.contains(needle) {
        return html.to_string();
    }
    let mut out = String::with_capacity(html.len());
    let mut cursor = 0usize;
    while let Some(rel) = html[cursor..].find(needle) {
        let i = cursor + rel;
        out.push_str(&html[cursor..i]);
        out.push_str(needle);
        let after = i + needle.len();
        // Skip whitespace
        let bytes = html.as_bytes();
        let mut j = after;
        while j < bytes.len() && (bytes[j] == b' ' || bytes[j] == b'\t') { j += 1; }
        out.push_str(&html[after..j]);
        // Read digits
        let num_start = j;
        while j < bytes.len() && bytes[j].is_ascii_digit() { j += 1; }
        if j > num_start && j + 2 <= bytes.len() && &html[j..j + 2] == "vh" {
            let n: u32 = html[num_start..j].parse().unwrap_or(0);
            if n > max_vh {
                out.push_str(&max_vh.to_string());
                out.push_str("vh");
                cursor = j + 2;
                continue;
            }
        }
        // Pass through unchanged
        out.push_str(&html[num_start..j]);
        cursor = j;
    }
    out.push_str(&html[cursor..]);
    out
}

fn auto_fix_h1_count(html: &str) -> String {
    let h1_count = html.matches("<h1").count();
    if h1_count == 0 {
        // Pass 42 surfaced: composer sometimes ships zero h1. Promote the
        // first h2 to h1 so the page has exactly one page-level heading.
        if let Some(i) = html.find("<h2") {
            // Find the matching closing tag for this opening h2.
            if let Some(open_end_rel) = html[i..].find('>') {
                let open_end = i + open_end_rel + 1;
                if let Some(close_rel) = html[open_end..].find("</h2>") {
                    let close_start = open_end + close_rel;
                    let close_end = close_start + "</h2>".len();
                    let mut out = String::with_capacity(html.len());
                    out.push_str(&html[..i]);
                    out.push_str(&html[i..open_end].replacen("<h2", "<h1", 1));
                    out.push_str(&html[open_end..close_start]);
                    out.push_str("</h1>");
                    out.push_str(&html[close_end..]);
                    return out;
                }
            }
        }
        return html.to_string();
    }
    if h1_count == 1 {
        return html.to_string();
    }
    let mut out = String::with_capacity(html.len());
    let mut cursor = 0usize;
    let mut seen_h1_open = false;
    while let Some(rel) = html[cursor..].find("<h1") {
        let i = cursor + rel;
        out.push_str(&html[cursor..i]);
        // Each occurrence: is it an opening or closing-tag-like? Should be
        // matched as `<h1` followed by space or `>`. We map opening + closing
        // <h1...>...</h1> as a pair on each occurrence.
        if !seen_h1_open {
            // Keep the first <h1...> as-is + find its </h1> and keep it.
            seen_h1_open = true;
            // Find matching </h1>
            let close_search_start = i;
            if let Some(close_rel) = html[close_search_start..].find("</h1>") {
                let close_end = close_search_start + close_rel + "</h1>".len();
                out.push_str(&html[i..close_end]);
                cursor = close_end;
            } else {
                out.push_str(&html[i..]);
                return out;
            }
        } else {
            // Downgrade this <h1 ...> ... </h1> to <h2 ...> ... </h2>
            // Find end of opening tag.
            let open_end = match html[i..].find('>') {
                Some(o) => i + o + 1,
                None => {
                    out.push_str(&html[i..]);
                    return out;
                }
            };
            let opening_tag = &html[i..open_end];
            let opening_rewritten = opening_tag.replacen("<h1", "<h2", 1);
            out.push_str(&opening_rewritten);
            // Find matching </h1> and replace with </h2>
            if let Some(close_rel) = html[open_end..].find("</h1>") {
                let close_start = open_end + close_rel;
                let close_end = close_start + "</h1>".len();
                out.push_str(&html[open_end..close_start]);
                out.push_str("</h2>");
                cursor = close_end;
            } else {
                out.push_str(&html[open_end..]);
                return out;
            }
        }
    }
    out.push_str(&html[cursor..]);
    out
}

/// Read-only semantic-HTML audit. Surfaces gaps that a production page
/// would not ship with: missing `<h1>`, zero `<section>` tags, declared
/// fonts not actually loaded, etc.
fn audit_composition(html: &str, display: Option<&str>, body: Option<&str>) -> Vec<String> {
    let mut warnings = Vec::new();
    let h1_count = html.matches("<h1").count();
    let section_count = html.matches("<section").count() + html.matches("<article").count();
    let nav_count = html.matches("<nav").count();
    let footer_count = html.matches("<footer").count();
    if h1_count == 0 {
        warnings.push("no `<h1>` element — production pages must have exactly one page-level heading.".into());
    } else if h1_count > 1 {
        warnings.push(format!(
            "{h1_count} `<h1>` elements — production pages should have exactly one."
        ));
    }
    if section_count == 0 {
        warnings.push("zero `<section>`/`<article>` tags — major regions should be wrapped in <section> or <article> for semantic accessibility.".into());
    }
    if nav_count == 0 && (html.contains("class=\"nav") || html.contains("class=\"sidebar")) {
        warnings.push("nav/sidebar found in classes but no `<nav>` element — wrap navigation in `<nav>` for semantic accessibility.".into());
    }
    if footer_count == 0 && html.contains("class=\"footer") {
        warnings.push("footer class found but no `<footer>` element — wrap site footer in `<footer>`.".into());
    }
    // ---- Accessibility (axe-like rules, no external dep) ------------
    // Real a11y audits (Lighthouse, axe-core) would run in headless
    // Chrome; ship those as a v0.8 phase. For now, the static checks
    // below catch the most common Korean production-blocking failures.

    // 1. Every <img> needs an `alt` attribute (empty alt is allowed for
    //    decorative images, but missing is not).
    let img_count = html.matches("<img").count();
    let img_with_alt = html.matches("alt=").count();
    if img_count > img_with_alt {
        warnings.push(format!(
            "{} `<img>` element(s) missing `alt=` attribute — every image needs an alt for screen readers (empty alt='' is allowed for decorative).",
            img_count - img_with_alt
        ));
    }

    // 2. <html lang="..."> must be set and non-empty. Default "en" still
    //    counts but composer should set "ko" for Korean pages.
    if !html.contains("<html lang=") && !html.contains("<html  lang=") {
        warnings.push("`<html>` has no `lang` attribute — required for screen readers and SEO. Set `lang=\"ko\"` for Korean pages.".into());
    }

    // 3. <meta name="viewport"> is required for mobile rendering. Without
    //    it Safari iOS renders at 980px and shrinks — kills mobile UX.
    if !html.contains("name=\"viewport\"") && !html.contains("name='viewport'") {
        warnings.push("missing `<meta name=\"viewport\">` — page will render at desktop width and shrink on mobile. Toss/Karrot bar requires this.".into());
    }

    // 4. <button> elements should have text or aria-label. Bare
    //    `<button></button>` is a screen-reader trap.
    let bare_button_re_count = html.matches("<button></button>").count()
        + html.matches("<button> </button>").count();
    if bare_button_re_count > 0 {
        warnings.push(format!(
            "{bare_button_re_count} empty `<button>` element(s) — buttons need text content or `aria-label`."
        ));
    }

    // 5. Heading hierarchy: h1 → h3 without an h2 is a structural gap.
    if html.contains("<h1") && html.contains("<h3") && !html.contains("<h2") {
        warnings.push("heading hierarchy gap: page has `<h1>` and `<h3>` but no `<h2>`. Screen readers expect sequential levels.".into());
    }

    // 6. Color-contrast static check: explicit `color: #ccc` (and similar
    //    near-white) on `background: #fff` patterns is a common low-
    //    contrast tell. Lighthouse would catch this dynamically; flag
    //    obvious cases statically.
    for low in ["color: #ccc", "color: #ddd", "color: #eee"] {
        if html.contains(low) && (html.contains("background: #fff") || html.contains("background-color: #fff")) {
            warnings.push(format!("low-contrast pattern detected (`{low}` on white background) — likely fails WCAG-AA 4.5:1. Use ≥ #767676 for body text on white."));
            break;
        }
    }

    // Mobile-first audit: every page should have at least one
    // `@media (min-width: ...)` block. A page with zero media queries
    // either targets desktop exclusively (collapses on phone) or relies
    // on intrinsic-only layout — both fail the Korean production bar.
    if !html.contains("@media (min-width") && !html.contains("@media(min-width") {
        warnings.push("no `@media (min-width: ...)` block — composition has no mobile-first responsive rules. Production targets (Toss, Karrot) are mobile-first.".into());
    }
    // Fixed multi-column grids without responsive override are a
    // common collapse pattern. Catch `grid-template-columns:` with
    // two-or-more `1fr` repeated.
    if html.contains("grid-template-columns: 1fr 1fr") && !html.contains("@media") {
        warnings.push("multi-column grid declared without any `@media` override — will overflow at mobile widths.".into());
    }
    // Check that declared fonts actually appear in the @import URL.
    for fam in [display, body].iter().flatten() {
        if !is_system_family(fam) {
            let fam_plus = fam.replace(' ', "+");
            if !html.contains(&fam_plus) {
                warnings.push(format!(
                    "declared font `{fam}` not present in any fonts.googleapis.com link — will fall back to system stack."
                ));
            }
        }
    }
    warnings
}

// ---------------------------------------------------------------------------
// Lucide path fingerprint recovery
// ---------------------------------------------------------------------------

/// (lucide-slug, signature). Signature is the FIRST `<path d="..."/>` content
/// (or `<rect`/`<circle`/`<line` prefix) — a unique-enough opening fragment
/// that downstream LLM output rarely mutates. Add more entries as we ship
/// additional Lucide icons in the asset-standards skill body.
const LUCIDE_FINGERPRINTS: &[(&str, &str)] = &[
    ("arrow-right", "M5 12h14"),
    ("arrow-up-right", "M7 7h10v10"),
    ("arrow-left", "m12 19-7-7 7-7"),
    ("arrow-down", "M12 5v14"),
    ("arrow-up", "m5 12 7-7 7 7"),
    ("mail", "M22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"),
    ("mail", "m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"),
    ("phone", "M22 16.92v3a2 2 0 0 1-2.18 2"),
    ("map-pin", "M20 10c0 4.993-5.539 10.193-7.399 11.799"),
    ("map-pin", "M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0"),
    ("home", "M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"),
    ("home", "m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2"),
    ("house", "M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8"),
    ("hammer", "m15 12-8.373 8.373"),
    ("ruler", "M21.3 8.7 8.7 21.3a2.41 2.41 0 0 1-3.4 0"),
    ("flask-conical", "M14 2v6a2 2 0 0 0 .245.96l5.51 10.08"),
    ("chart-line", "M3 3v16a2 2 0 0 0 2 2h16"),
    ("chart-line", "M3 3v18h18"),
    ("chevron-right", "m9 18 6-6-6-6"),
    ("chevron-left", "m15 18-6-6 6-6"),
    ("chevron-down", "m6 9 6 6 6-6"),
    ("chevron-up", "m18 15-6-6-6 6"),
    ("triangle-alert", "m21.73 18-8-14a2 2 0 0 0-3.48 0"),
    ("layout-dashboard", ""), // detected via rect-block heuristic below
    ("user-plus", "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"),
    ("users", "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"),
    ("user", "M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"),
    ("settings", "M19.14 12.94c.04-.3.06-.61.06-.94"),
    ("settings", "M12.22 2h-.44a2 2 0 0 0-2 2v.18"),
    ("search", "m21 21-4.34-4.34"),
    ("search", "M21 21l-4.35-4.35"),
    ("menu", "M4 12h16"),
    ("menu", "M3 12h18"),
    ("x", "M18 6 6 18"),
    ("check", "M20 6 9 17l-5-5"),
    ("plus", "M5 12h14"),
    ("minus", "M5 12h14"),
    ("calendar", "M8 2v4"),
    ("clock", "M12 6v6l4 2"),
    ("heart", "M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3"),
    ("star", "M11.525 2.295a.53.53 0 0 1 .95 0"),
    ("download", "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"),
    ("upload", "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"),
    ("trash", "M3 6h18"),
    ("edit", "M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"),
    ("file", "M14.5 22H5a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h7l5 5v3.5"),
    ("folder", "M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13"),
    ("globe", "M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0"),
    ("eye", "M2.062 12.348a1 1 0 0 1 0-.696 10.75 10.75 0 0 1 19.876 0"),
    ("link", "M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"),
    ("external-link", "M15 3h6v6"),
    ("log-out", "M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"),
    ("log-in", "M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"),
    ("filter", "M3 6h18l-7 12v6l-4-2v-4z"),
    ("filter", "M2 6h20l-9 12v6l-2-2v-4z"),
    ("share", "M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8"),
    ("info", "M12 16v-4"),
    ("info", "M12 8h.01"),
    ("zap", "M4 14a1 1 0 0 1-.78-1.63l9.9-10.2a.5.5 0 0 1 .86.46l-1.92 6.02A1 1 0 0 0 13 10h7"),
    ("circle", "M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z"),
    ("square", "M21 16V8a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v8"),
    ("activity", "M22 12h-2.48a2 2 0 0 0-1.93 1.46l-2.35 8.36a.25.25 0 0 1-.48"),
    ("activity", "M22 12h-4l-3 9L9 3l-3 9H2"),
    ("git-branch", "M6 3v12"),
    ("terminal", "m4 17 6-6-6-6"),
    ("database", "M5 3a9 3 0 0 1 14 0"),
    ("database", "M3 5a9 3 0 0 0 18 0"),
    ("cloud", "M17.5 19H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9"),
    ("lock", "M5 11h14"),
];

/// Walk every `<svg ...>...</svg>` block. If its first path / shape opens
/// with a known Lucide fingerprint AND the opening `<svg` lacks
/// `class="lucide lucide-...`, inject the class. Returns the rewritten
/// HTML plus the list of (deduplicated) icon slugs we recovered.
pub fn recover_lucide_classes(html: &str) -> (String, Vec<String>) {
    let mut out = String::with_capacity(html.len() + 256);
    let mut recovered: Vec<String> = Vec::new();
    let mut cursor = 0usize;
    while let Some(rel) = html[cursor..].find("<svg") {
        let svg_start = cursor + rel;
        out.push_str(&html[cursor..svg_start]);
        // Find end of opening tag.
        let open_end = match html[svg_start..].find('>') {
            Some(i) => svg_start + i + 1,
            None => {
                // Malformed; flush rest.
                out.push_str(&html[svg_start..]);
                return (out, dedup(&recovered));
            }
        };
        // Find </svg>.
        let close = match html[open_end..].find("</svg>") {
            Some(i) => open_end + i,
            None => {
                out.push_str(&html[svg_start..]);
                return (out, dedup(&recovered));
            }
        };
        let opening = &html[svg_start..open_end];
        let body = &html[open_end..close];

        let already_labelled = opening.contains("class=\"lucide lucide-")
            || opening.contains("class='lucide lucide-");

        let detected = if already_labelled {
            None
        } else {
            detect_lucide_by_fingerprint(body)
        };

        match detected {
            Some(slug) => {
                let rewritten = inject_class_attr(opening, &format!("lucide lucide-{slug}"));
                out.push_str(&rewritten);
                out.push_str(body);
                out.push_str("</svg>");
                recovered.push(slug);
            }
            None => {
                out.push_str(&html[svg_start..close + 6]);
            }
        }
        cursor = close + 6; // past `</svg>`
    }
    out.push_str(&html[cursor..]);
    (out, dedup(&recovered))
}

fn detect_lucide_by_fingerprint(svg_body: &str) -> Option<String> {
    // Find the first `d="...` (or `d='...`) opening — that's the d-attribute
    // of the first <path>. Some Lucide icons open with <rect> instead.
    if let Some(idx) = svg_body.find("d=\"") {
        let after = &svg_body[idx + 3..];
        let end = after.find('"').unwrap_or(after.len()).min(120);
        let needle = &after[..end];
        for (slug, fp) in LUCIDE_FINGERPRINTS {
            if fp.is_empty() {
                continue;
            }
            if needle.starts_with(fp) {
                return Some((*slug).to_string());
            }
        }
    }
    // layout-dashboard heuristic — exactly 4 <rect> blocks, two at x="3" and
    // two at x="14", which uniquely identifies Lucide's layout-dashboard.
    let rects: Vec<&str> = svg_body.matches("<rect ").collect();
    if rects.len() == 4
        && svg_body.matches("x=\"3\"").count() == 2
        && svg_body.matches("x=\"14\"").count() == 2
    {
        return Some("layout-dashboard".into());
    }
    None
}

fn inject_class_attr(opening_tag: &str, new_class: &str) -> String {
    // If `class="...` is already there, prepend new_class inside the quotes.
    if let Some(idx) = opening_tag.find("class=\"") {
        let after = &opening_tag[idx + 7..];
        let mut out = String::with_capacity(opening_tag.len() + new_class.len() + 4);
        out.push_str(&opening_tag[..idx + 7]);
        out.push_str(new_class);
        out.push(' ');
        out.push_str(after);
        return out;
    }
    // Otherwise insert `class="..."` just before the closing `>` (or self-close).
    let close = if opening_tag.ends_with("/>") {
        opening_tag.len() - 2
    } else if opening_tag.ends_with('>') {
        opening_tag.len() - 1
    } else {
        opening_tag.len()
    };
    let mut out = String::with_capacity(opening_tag.len() + new_class.len() + 12);
    out.push_str(&opening_tag[..close]);
    if !opening_tag[..close].ends_with(' ') {
        out.push(' ');
    }
    out.push_str(&format!("class=\"{new_class}\""));
    out.push_str(&opening_tag[close..]);
    out
}

fn dedup(v: &[String]) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for s in v {
        if seen.insert(s.clone()) {
            out.push(s.clone());
        }
    }
    out.sort();
    out
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
                // YAML font value may be a single family ("Inter") or a
                // CSS-style stack ("Inter, Helvetica, sans-serif"). Take
                // the FIRST family (the primary intent) and drop the
                // fallback chain.
                let primary = value
                    .split(',')
                    .next()
                    .unwrap_or(&value)
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string();
                if in_display && display.is_none() {
                    display = Some(primary);
                } else if in_body && body.is_none() {
                    body = Some(primary);
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
    fn recover_lucide_class_for_unlabelled_chart_line() {
        let html = r##"<html><body>
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 3v16a2 2 0 0 0 2 2h16"/><path d="m19 9-5 5-4-4-3 3"/></svg>
</body></html>"##;
        let (out, recovered) = recover_lucide_classes(html);
        assert_eq!(recovered, vec!["chart-line".to_string()]);
        assert!(out.contains("class=\"lucide lucide-chart-line\""));
    }

    #[test]
    fn already_labelled_svg_is_left_alone() {
        let html = r##"<svg class="lucide lucide-arrow-right"><path d="M5 12h14"/><path d="m12 5 7 7-7 7"/></svg>"##;
        let (out, recovered) = recover_lucide_classes(html);
        assert!(recovered.is_empty(), "should not relabel already-labelled svg");
        assert_eq!(out, html);
    }

    #[test]
    fn unknown_svg_is_left_alone() {
        let html = r##"<svg viewBox="0 0 100 100"><circle cx="50" cy="50" r="40"/></svg>"##;
        let (out, recovered) = recover_lucide_classes(html);
        assert!(recovered.is_empty());
        assert_eq!(out, html);
    }

    #[test]
    fn class_attr_merged_with_existing() {
        let html = r##"<svg class="chart-svg"><path d="M3 3v16a2 2 0 0 0 2 2h16"/></svg>"##;
        let (out, recovered) = recover_lucide_classes(html);
        assert_eq!(recovered, vec!["chart-line".to_string()]);
        assert!(out.contains("class=\"lucide lucide-chart-line chart-svg\""));
    }

    #[test]
    fn fix_broken_img_replaces_empty_src_with_figure() {
        // Pass 50 alpha: external URLs are now placeholder-by-default
        // unless explicitly opted out with data-aphrodite-asset-verified.
        let html = r##"<body>
<img src="" alt="Seoul Dining Table">
<img src="https://real.cdn/photo.jpg" alt="real photo" data-aphrodite-asset-verified>
<img src="[photo: workshop interior]" alt="workshop">
</body>"##;
        let fixed = fix_broken_img_placeholders(html);
        // Empty-src img and bracket-prefixed img become figures
        assert!(fixed.contains("image-placeholder"));
        assert!(fixed.contains("[photo: Seoul Dining Table]"));
        assert!(fixed.contains("[photo: workshop]"));
        // Explicitly verified real img URL is preserved
        assert!(fixed.contains(r#"<img src="https://real.cdn/photo.jpg""#));
        // Should NOT have any <img src=""
        assert!(!fixed.contains(r#"<img src="""#));
    }

    #[test]
    fn strip_video_unverified_external_url() {
        let html = r#"<body><video src="https://cdn.example.com/promo.mp4" autoplay aria-label="제품 소개 영상"></video></body>"#;
        let out = strip_unverified_external_media(html);
        assert!(out.contains("image-placeholder"));
        assert!(out.contains("[video: 제품 소개 영상]"));
        assert!(!out.contains("cdn.example.com"));
    }

    #[test]
    fn strip_video_preserves_verified_external_url() {
        let html = r#"<body><video src="https://real.cdn/video.mp4" data-aphrodite-asset-verified></video></body>"#;
        let out = strip_unverified_external_media(html);
        assert!(out.contains("real.cdn"));
        assert!(!out.contains("image-placeholder"));
    }

    #[test]
    fn strip_audio_unverified_external_url() {
        let html = r#"<body><audio src="https://stream.example.com/podcast.mp3" title="에피소드 1"></audio></body>"#;
        let out = strip_unverified_external_media(html);
        assert!(out.contains("[audio: 에피소드 1]"));
        assert!(!out.contains("stream.example.com"));
    }

    #[test]
    fn fix_broken_img_replaces_unverified_external_url() {
        // Pass 50 Banchan critical #3: composer used an unsplash-style
        // URL that resolved → 1.77 MB hero image. Default-to-placeholder
        // unless verified.
        let html = r#"<body><img src="https://images.unsplash.com/photo-xyz.jpg" alt="bibimbap"></body>"#;
        let fixed = fix_broken_img_placeholders(html);
        assert!(fixed.contains("image-placeholder"));
        assert!(fixed.contains("[photo: bibimbap]"));
        assert!(!fixed.contains("images.unsplash.com"));
    }

    #[test]
    fn auto_fix_h1_downgrades_extras_to_h2() {
        let html = r##"<html><body>
<h1 class="hero">Page Title</h1>
<section><h1>First Project</h1><p>...</p></section>
<section><h1 id="p2">Second Project</h1><p>...</p></section>
</body></html>"##;
        let fixed = auto_fix_h1_count(html);
        // First h1 stays
        assert!(fixed.contains("<h1 class=\"hero\">Page Title</h1>"));
        // Others become h2 with the original attributes preserved
        assert!(fixed.contains("<h2>First Project</h2>"));
        assert!(fixed.contains("<h2 id=\"p2\">Second Project</h2>"));
        assert_eq!(fixed.matches("<h1").count(), 1);
        assert_eq!(fixed.matches("<h2").count(), 2);
    }

    #[test]
    fn cap_section_min_height_clamps_oversized_vh() {
        let css = ".hero { min-height: 80vh; padding: 20px; } .other { min-height: 30vh; }";
        let out = cap_vh_property(css, "min-height", 40);
        assert!(out.contains("min-height: 40vh"));
        assert!(out.contains("min-height: 30vh"), "values below the cap pass through");
        assert!(!out.contains("80vh"));
    }

    #[test]
    fn quality_score_counts_article_as_region() {
        // v1.0 RC.9: editorial pages use <article>+<main>+<aside>
        // instead of <section>. Should still hit semantic=100.
        let html = r##"<!doctype html><html lang="ko"><head><meta name="viewport" content="width=device-width, initial-scale=1"><style>@media (min-width: 768px) {}</style></head><body><nav>n</nav><h1>제목</h1><article>a1</article><article>a2</article><footer>f</footer></body></html>"##;
        let (axes, _score) = score_quality(html, &[]);
        assert_eq!(axes.semantic, 100, "article should count toward region budget");
    }

    #[test]
    fn quality_score_high_on_clean_production_page() {
        let html = r##"<!doctype html><html lang="ko"><head><meta name="viewport" content="width=device-width, initial-scale=1"><style>@media (min-width: 768px) {}</style></head><body><nav>n</nav><h1>안녕</h1><section>s1</section><section>s2</section><footer>f</footer></body></html>"##;
        let (axes, score) = score_quality(html, &[]);
        assert!(score >= 90, "expected ≥ 90, got {score} (axes: {axes:?})");
        assert_eq!(axes.mobile, 100);
        assert_eq!(axes.semantic, 100);
    }

    #[test]
    fn quality_score_low_when_critical_warnings() {
        let html = "<html><body><div>no semantic</div></body></html>";
        let warnings = vec![
            "no <h1>".to_string(),
            "`<html>` has no `lang` attribute".to_string(),
            "missing viewport meta".to_string(),
        ];
        let (_axes, score) = score_quality(html, &warnings);
        assert!(score < 70, "expected < 70, got {score}");
    }

    #[test]
    fn korean_shim_injected_when_hangul() {
        let html = r#"<html lang="en"><head></head><body>안녕</body></html>"#;
        let out = inject_korean_layout_shim(html);
        assert!(out.contains("aphrodite-ko-shim"));
        assert!(out.contains("word-break: keep-all"));
        // idempotent
        let out2 = inject_korean_layout_shim(&out);
        assert_eq!(out, out2);
    }

    #[test]
    fn korean_shim_skipped_on_english_only() {
        let html = "<html><body>Hello</body></html>";
        assert_eq!(inject_korean_layout_shim(html), html);
    }

    #[test]
    fn patch_lang_sets_ko_when_hangul_present() {
        let html = r#"<!doctype html><html lang="en"><body><h1>안녕하세요</h1></body></html>"#;
        let out = patch_lang_attribute(html);
        assert!(out.contains(r#"<html lang="ko">"#));
        assert!(!out.contains(r#"<html lang="en">"#));
    }

    #[test]
    fn patch_lang_injects_when_missing() {
        let html = "<!doctype html><html><body><h1>안녕하세요</h1></body></html>";
        let out = patch_lang_attribute(html);
        assert!(out.contains(r#"<html lang="ko">"#));
    }

    #[test]
    fn patch_lang_idempotent_when_already_ko() {
        let html = r#"<html lang="ko"><body>안녕</body></html>"#;
        assert_eq!(patch_lang_attribute(html), html);
    }

    #[test]
    fn patch_lang_no_change_for_english_only() {
        let html = r#"<html lang="en"><body>Hello world</body></html>"#;
        assert_eq!(patch_lang_attribute(html), html);
    }

    #[test]
    fn count_viewport_violations_catches_high_vh_and_calc() {
        let css = ".hero { min-height: 80vh; } .img { height: 100vh; } .x { height: calc(100vw * 1.25); } .ok { height: 30vh; }";
        assert_eq!(count_viewport_height_violations(css), 3);
    }

    #[test]
    fn clamp_calc_viewport_heights_adds_max_height() {
        let css = ".hero { height: calc(100vw * 1.25); overflow: hidden; }";
        let out = clamp_calc_viewport_heights(css, 720);
        assert!(out.contains("height: calc(100vw * 1.25);"));
        assert!(out.contains("max-height: 720px"));
    }

    #[test]
    fn clamp_calc_viewport_heights_idempotent_when_capped() {
        let css = ".hero { height: calc(100vw * 1.25); max-height: 640px; }";
        let out = clamp_calc_viewport_heights(css, 720);
        // Only one max-height in the output (didn't double-inject).
        assert_eq!(out.matches("max-height:").count(), 1);
    }

    #[test]
    fn ensure_placeholder_max_height_injects_when_missing() {
        let html = r#"<figure class="image-placeholder" style="aspect-ratio: 4/5; height: calc(100vw * 1.25);">[photo: hero]</figure>"#;
        let out = ensure_placeholder_max_height(html, 720);
        assert!(out.contains("max-height: 720px"));
    }

    #[test]
    fn ensure_placeholder_max_height_leaves_smaller_caps() {
        let html = r#"<figure class="image-placeholder" style="max-height: 600px;">[photo: x]</figure>"#;
        let out = ensure_placeholder_max_height(html, 720);
        assert!(out.contains("max-height: 600px"));
        assert!(!out.contains("max-height: 720px"));
    }

    #[test]
    fn cap_vh_property_handles_plain_height() {
        // Pass 43 surfaced: `.hero figure { height: 80vh }` was the real
        // culprit — the cap must also clamp bare `height:` declarations,
        // not just `min-height:`.
        let css = ".hero figure { height: 80vh; width: 100%; }";
        let out = cap_vh_property(css, "height", 60);
        assert!(out.contains("height: 60vh"));
        assert!(!out.contains("80vh"));
    }

    #[test]
    fn auto_fix_h1_promotes_first_h2_when_zero() {
        let html = "<body><h2 class=\"hero\">Hello</h2><h2>Section</h2></body>";
        let fixed = auto_fix_h1_count(html);
        assert_eq!(fixed.matches("<h1").count(), 1);
        assert!(fixed.contains("<h1 class=\"hero\">Hello</h1>"));
        // Second h2 untouched
        assert!(fixed.contains("<h2>Section</h2>"));
    }

    #[test]
    fn auto_fix_h1_idempotent_when_single() {
        let html = "<h1>One</h1><h2>Two</h2><h2>Three</h2>";
        assert_eq!(auto_fix_h1_count(html), html);
    }

    #[test]
    fn recovers_home_icon_by_fingerprint() {
        let html = r##"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/></svg>"##;
        let (out, recovered) = recover_lucide_classes(html);
        assert_eq!(recovered, vec!["home".to_string()]);
        assert!(out.contains("class=\"lucide lucide-home\""));
    }

    #[test]
    fn detects_layout_dashboard_by_rect_heuristic() {
        let html = r##"<svg viewBox="0 0 24 24">
            <rect width="7" height="9" x="3" y="3" rx="1"/>
            <rect width="7" height="5" x="14" y="3" rx="1"/>
            <rect width="7" height="9" x="14" y="12" rx="1"/>
            <rect width="7" height="5" x="3" y="16" rx="1"/>
        </svg>"##;
        let (out, recovered) = recover_lucide_classes(html);
        assert_eq!(recovered, vec!["layout-dashboard".to_string()]);
        assert!(out.contains("lucide-layout-dashboard"));
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

    #[test]
    fn extract_families_takes_first_when_stack_provided() {
        // Pass 42 surfaced: composer sometimes writes the full CSS-style
        // stack into the YAML family field. Harmonize must pluck the
        // primary intent (first family), not the whole string.
        let md = r##"---
typography:
  display:
    family: "Instrument Serif, Noto Serif KR, Georgia, serif"
  body:
    family: "Inter, 'Pretendard', system-ui, sans-serif"
---
"##;
        let (d, b) = extract_families_from_md(md);
        assert_eq!(d.as_deref(), Some("Instrument Serif"));
        assert_eq!(b.as_deref(), Some("Inter"));
    }
}
