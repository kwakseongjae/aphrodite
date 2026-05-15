//! Design-reference wiki — Karpathy LLM-Wiki pattern adapted for Aphrodite.
//!
//! Compiles design references (sites / type families / colour systems / past
//! work) once and keeps them current as a compounding markdown KB. At
//! `aphrodite create` time we query the wiki by intent tags and inject the
//! top-K entries into the design call as concrete prior art — replacing the
//! LLM's habit of imagining stock references with verified ones.
//!
//! Source: <https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f>
//! and the Hermes Agent `llm-wiki` skill packaging of the same pattern.
//!
//! Layout: `~/.aphrodite/wiki/<slug>.md` (flat, single-level).
//! Frontmatter: `title`, `url`, `tags`, `signature`, `ingested_at`.
//! Body: free-form prose — why this reference is useful, what to absorb,
//! what to NOT copy.

use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiFrontmatter {
    pub title: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub tags: Vec<String>,
    /// One-line stylistic distillation — what this reference *is*.
    #[serde(default)]
    pub signature: String,
    #[serde(default)]
    pub ingested_at: String,
}

#[derive(Debug, Clone)]
pub struct WikiEntry {
    pub slug: String,
    pub frontmatter: WikiFrontmatter,
    pub body: String,
    pub path: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum WikiError {
    #[error("wiki io: {0}")]
    Io(#[from] io::Error),
    #[error("wiki format: {0}")]
    Format(String),
    #[error("wiki entry missing frontmatter")]
    NoFrontmatter,
}

pub fn wiki_root() -> PathBuf {
    let home = std::env::var_os("HOME").unwrap_or_default();
    let mut p = PathBuf::from(home);
    p.push(".aphrodite");
    p.push("wiki");
    let _ = fs::create_dir_all(&p);
    p
}

pub fn entry_path(slug: &str) -> PathBuf {
    let mut p = wiki_root();
    p.push(format!("{slug}.md"));
    p
}

pub fn parse_entry(slug: &str, text: &str, path: PathBuf) -> Result<WikiEntry, WikiError> {
    let trimmed = text.trim_start();
    if !trimmed.starts_with("---") {
        return Err(WikiError::NoFrontmatter);
    }
    let after = &trimmed[3..];
    let end = after.find("\n---").ok_or(WikiError::NoFrontmatter)?;
    let fm: WikiFrontmatter =
        serde_yaml::from_str(&after[..end]).map_err(|e| WikiError::Format(e.to_string()))?;
    let body = after[end + 4..].trim_start_matches('\n').to_string();
    Ok(WikiEntry { slug: slug.to_string(), frontmatter: fm, body, path })
}

pub fn load(slug: &str) -> Result<WikiEntry, WikiError> {
    let path = entry_path(slug);
    let text = fs::read_to_string(&path)?;
    parse_entry(slug, &text, path)
}

/// Persist a new (or updated) wiki entry. Overwrites if `slug` already exists.
pub fn save(entry: &WikiEntry) -> Result<(), WikiError> {
    let fm = serde_yaml::to_string(&entry.frontmatter)
        .map_err(|e| WikiError::Format(e.to_string()))?;
    let text = format!("---\n{fm}---\n\n{}\n", entry.body.trim_end());
    let path = entry_path(&entry.slug);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, text)?;
    Ok(())
}

/// Derive a slug from a URL: lowercase host without TLD/subdomain noise.
/// `https://www.apartamentomagazine.com/about` → `apartamentomagazine`.
pub fn slug_from_url(url: &str) -> String {
    let host = url
        .split("://")
        .nth(1)
        .unwrap_or(url)
        .split('/')
        .next()
        .unwrap_or(url);
    let host = host.trim_start_matches("www.");
    // Drop the final label (TLD). For `linear.app` → `linear`,
    // `apartamentomagazine.com` → `apartamentomagazine`,
    // `blog.muji.com` → `blog-muji`.
    let labels: Vec<&str> = host.split('.').collect();
    let core = if labels.len() >= 2 {
        labels[..labels.len() - 1].join("-")
    } else {
        host.to_string()
    };
    core.to_ascii_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

// ---------------------------------------------------------------------------
// URL metadata fetch — Pass 18 auto-ingest helper
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
pub struct UrlMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub og_image: Option<String>,
    /// Colour hex values discovered in the HTML (`#abc`, `#abcdef`). De-duplicated,
    /// capped at 8, ordered by appearance.
    pub palette_hints: Vec<String>,
}

/// Pure HTML→metadata extraction. The fetch itself lives in the CLI crate
/// (which already has reqwest); keeping core pure avoids dragging async
/// HTTP into the shared invariants crate.
pub fn extract_metadata_from_html(html: &str) -> UrlMetadata {
    let title = extract_first_match(html, "<title", "</title>")
        .map(|s| html_decode(&s.trim().replace('\n', " ")));
    let description = extract_meta(html, "description").or_else(|| extract_og(html, "description"));
    let og_image = extract_og(html, "image");
    let palette_hints = extract_palette_hints(html, 8);
    UrlMetadata {
        title,
        description,
        og_image,
        palette_hints,
    }
}

fn extract_first_match(html: &str, open_prefix: &str, close: &str) -> Option<String> {
    let i = html.to_ascii_lowercase().find(open_prefix)?;
    let after_open_tag = html[i..].find('>')? + i + 1;
    let close_idx = html.to_ascii_lowercase()[after_open_tag..].find(close)?;
    Some(html[after_open_tag..after_open_tag + close_idx].to_string())
}

fn extract_meta(html: &str, name: &str) -> Option<String> {
    let lower = html.to_ascii_lowercase();
    let needle = format!("name=\"{name}\"");
    let i = lower.find(&needle)?;
    // Find content="..." in the surrounding tag — look backwards and forwards
    // for the enclosing <meta ...> tag.
    let tag_start = lower[..i].rfind("<meta").unwrap_or(i);
    let tag_end = lower[i..].find('>').map(|x| i + x).unwrap_or(html.len());
    let tag_text = &html[tag_start..tag_end];
    pick_attribute(tag_text, "content").map(|s| html_decode(&s))
}

fn extract_og(html: &str, property_suffix: &str) -> Option<String> {
    let lower = html.to_ascii_lowercase();
    let needle = format!("property=\"og:{property_suffix}\"");
    let i = lower.find(&needle)?;
    let tag_start = lower[..i].rfind("<meta").unwrap_or(i);
    let tag_end = lower[i..].find('>').map(|x| i + x).unwrap_or(html.len());
    let tag_text = &html[tag_start..tag_end];
    pick_attribute(tag_text, "content").map(|s| html_decode(&s))
}

fn pick_attribute(tag: &str, attr: &str) -> Option<String> {
    let needle_dq = format!("{attr}=\"");
    if let Some(i) = tag.find(&needle_dq) {
        let after = &tag[i + needle_dq.len()..];
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

fn html_decode(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
}

/// Find hex colour values in the HTML body. Captures both `#abc` and
/// `#abcdef`. De-duplicated, ordered by first appearance, capped at `max`.
pub fn extract_palette_hints(html: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = html.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'#' && i + 4 <= bytes.len() {
            // Try 6-hex first, then 3-hex
            let try6 = i + 1 + 6 <= bytes.len()
                && (i + 7 == bytes.len()
                    || !bytes[i + 7].is_ascii_hexdigit())
                && bytes[i + 1..i + 7].iter().all(|b| b.is_ascii_hexdigit());
            if try6 {
                let hex = format!("#{}", &html[i + 1..i + 7].to_ascii_lowercase());
                if !out.contains(&hex) {
                    out.push(hex);
                    if out.len() >= max {
                        return out;
                    }
                }
                i += 7;
                continue;
            }
            let try3 = i + 1 + 3 <= bytes.len()
                && (i + 4 == bytes.len() || !bytes[i + 4].is_ascii_hexdigit())
                && bytes[i + 1..i + 4].iter().all(|b| b.is_ascii_hexdigit());
            if try3 {
                let hex = format!("#{}", &html[i + 1..i + 4].to_ascii_lowercase());
                if !out.contains(&hex) {
                    out.push(hex);
                    if out.len() >= max {
                        return out;
                    }
                }
                i += 4;
                continue;
            }
        }
        i += 1;
    }
    out
}

pub fn list() -> Vec<String> {
    let mut out = Vec::new();
    if let Ok(entries) = fs::read_dir(wiki_root()) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    out.push(stem.to_string());
                }
            }
        }
    }
    out.sort();
    out
}

const BUNDLED_WIKI: &[(&str, &str)] = &[
    ("pretendard", include_str!("../seed-wiki/pretendard.md")),
    ("muji-website", include_str!("../seed-wiki/muji-website.md")),
    ("apartamento", include_str!("../seed-wiki/apartamento.md")),
    ("pentagram", include_str!("../seed-wiki/pentagram.md")),
    ("linear-app", include_str!("../seed-wiki/linear-app.md")),
    ("are-na", include_str!("../seed-wiki/are-na.md")),
    ("naver-papago", include_str!("../seed-wiki/naver-papago.md")),
    ("vercel", include_str!("../seed-wiki/vercel.md")),
    ("nyt-magazine", include_str!("../seed-wiki/nyt-magazine.md")),
    ("pitchfork", include_str!("../seed-wiki/pitchfork.md")),
    (
        "teenage-engineering",
        include_str!("../seed-wiki/teenage-engineering.md"),
    ),
    ("figma-site", include_str!("../seed-wiki/figma-site.md")),
    (
        "aesop-website",
        include_str!("../seed-wiki/aesop-website.md"),
    ),
];

pub fn seed_bundled_wiki() -> Vec<String> {
    let mut newly = Vec::new();
    for (slug, contents) in BUNDLED_WIKI {
        let target = entry_path(slug);
        // Same self-healing pattern as personas — overwrite an on-disk entry
        // only if it fails to parse. User edits to a parseable entry are
        // preserved.
        if target.exists() {
            if let Ok(text) = fs::read_to_string(&target) {
                if parse_entry(slug, &text, target.clone()).is_ok() {
                    continue;
                }
            } else {
                continue;
            }
        }
        if let Some(parent) = target.parent() {
            if fs::create_dir_all(parent).is_err() {
                continue;
            }
        }
        if fs::write(&target, contents).is_ok() {
            newly.push((*slug).to_string());
        }
    }
    newly
}

/// Rank wiki entries by tag overlap with the incoming intent's tag set.
/// Returns top_k entries with score > 0.
pub fn query_by_tags(intent_tags: &[&str], top_k: usize) -> Vec<WikiEntry> {
    let mut scored: Vec<(i32, WikiEntry)> = Vec::new();
    for slug in list() {
        let entry = match load(&slug) {
            Ok(e) => e,
            Err(_) => continue,
        };
        let overlap = entry
            .frontmatter
            .tags
            .iter()
            .filter(|t| intent_tags.iter().any(|it| it.eq_ignore_ascii_case(t)))
            .count() as i32;
        if overlap > 0 {
            scored.push((overlap, entry));
        }
    }
    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored.into_iter().take(top_k).map(|(_, e)| e).collect()
}

/// Render a set of wiki entries as a single text block for prompt injection.
pub fn render_references_block(entries: &[WikiEntry]) -> String {
    if entries.is_empty() {
        return String::new();
    }
    let mut out = String::from("## Reference materials (curated design wiki — absorb the signal, do NOT copy verbatim)\n\n");
    for e in entries {
        out.push_str(&format!("### {} — {}\n", e.frontmatter.title, e.frontmatter.signature));
        if !e.frontmatter.url.is_empty() {
            out.push_str(&format!("Source: {}\n\n", e.frontmatter.url));
        }
        // Cap body at 1500 chars per entry — references should be terse.
        if e.body.len() <= 1500 {
            out.push_str(e.body.trim());
        } else {
            let trunc: String = e.body.chars().take(1500).collect();
            out.push_str(trunc.trim());
            out.push_str("\n…[truncated]");
        }
        out.push_str("\n\n---\n\n");
    }
    out.push_str("These are *prior art*. Pull their compositional moves and avoid the obvious traps. The output must remain specific to the user's intent — do not regurgitate any of these brands.\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Scratch {
        _td: tempfile::TempDir,
        _g: std::sync::MutexGuard<'static, ()>,
    }
    impl Scratch {
        fn new() -> Self {
            let g = crate::test_lock::GLOBAL.lock().unwrap_or_else(|e| e.into_inner());
            let td = tempfile::tempdir().unwrap();
            unsafe { std::env::set_var("HOME", td.path()); }
            Self { _td: td, _g: g }
        }
    }

    #[test]
    fn seed_then_load_then_query() {
        let _s = Scratch::new();
        let newly = seed_bundled_wiki();
        assert!(newly.len() >= 5, "expected >=5 seeded wiki entries, got {newly:?}");
        // Load one and verify shape
        let pretendard = load("pretendard").unwrap();
        assert!(!pretendard.frontmatter.title.is_empty());
        assert!(!pretendard.frontmatter.tags.is_empty());
        assert!(!pretendard.body.trim().is_empty());
        // Query
        let results = query_by_tags(&["typography", "korean"], 3);
        assert!(!results.is_empty(), "query should match pretendard");
        // Re-seed is no-op
        let again = seed_bundled_wiki();
        assert!(again.is_empty());
    }

    #[test]
    fn references_block_renders() {
        let _s = Scratch::new();
        seed_bundled_wiki();
        let entries = query_by_tags(&["editorial", "portfolio"], 2);
        let block = render_references_block(&entries);
        assert!(block.contains("Reference materials"));
        assert!(block.contains("Source:"));
    }

    #[test]
    fn empty_block_for_no_results() {
        assert!(render_references_block(&[]).is_empty());
    }

    #[test]
    fn slug_from_url_strips_www_and_tld() {
        assert_eq!(slug_from_url("https://www.apartamentomagazine.com/"), "apartamentomagazine");
        assert_eq!(slug_from_url("https://linear.app"), "linear");
        assert_eq!(slug_from_url("https://www.muji.com/store"), "muji");
        assert_eq!(slug_from_url("https://blog.muji.com/recommend"), "blog-muji");
    }

    #[test]
    fn extracts_title_and_description() {
        let html = r#"<!doctype html><html><head>
<title>Example — A Web Page</title>
<meta name="description" content="A description of the page that should be picked up." />
<meta property="og:image" content="https://example.com/banner.png">
</head><body>
<p>Hello.</p>
</body></html>"#;
        let md = extract_metadata_from_html(html);
        assert_eq!(md.title.as_deref(), Some("Example — A Web Page"));
        assert_eq!(
            md.description.as_deref(),
            Some("A description of the page that should be picked up.")
        );
        assert_eq!(md.og_image.as_deref(), Some("https://example.com/banner.png"));
    }

    #[test]
    fn falls_back_to_og_description() {
        let html = r#"<html><head>
<title>X</title>
<meta property="og:description" content="OG fallback desc" />
</head></html>"#;
        let md = extract_metadata_from_html(html);
        assert_eq!(md.description.as_deref(), Some("OG fallback desc"));
    }

    #[test]
    fn palette_hints_dedup_and_cap() {
        let html = r#"<style>.a{color:#fff}.b{color:#FFF}.c{color:#abc}.d{color:#123456}.e{background:#777}</style>"#;
        let hints = extract_palette_hints(html, 8);
        // #fff and #FFF dedup to one (lowercased)
        assert!(hints.contains(&"#fff".to_string()));
        assert!(hints.contains(&"#abc".to_string()));
        assert!(hints.contains(&"#123456".to_string()));
        assert!(hints.contains(&"#777".to_string()));
        assert_eq!(hints.iter().filter(|s| *s == "#fff").count(), 1);
    }

    #[test]
    fn save_round_trip() {
        let _s = Scratch::new();
        let entry = WikiEntry {
            slug: "test-entry".to_string(),
            frontmatter: WikiFrontmatter {
                title: "Test entry".into(),
                url: "https://example.com".into(),
                tags: vec!["test".into(), "portfolio".into()],
                signature: "a test fixture".into(),
                ingested_at: "2026-05-14".into(),
            },
            body: "# Test\n\nWhat to absorb: nothing, it's a test.\n".into(),
            path: entry_path("test-entry"),
        };
        save(&entry).unwrap();
        let loaded = load("test-entry").unwrap();
        assert_eq!(loaded.frontmatter.title, "Test entry");
        assert_eq!(loaded.frontmatter.tags, vec!["test", "portfolio"]);
        assert!(loaded.body.contains("nothing, it's a test"));
    }
}
