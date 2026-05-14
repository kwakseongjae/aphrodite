//! `aphrodite wiki <subcommand>` — manage the design-reference wiki.

use aphrodite_core::wiki;
use serde_json::json;

pub fn list() -> serde_json::Value {
    use console::style;
    let _ = wiki::seed_bundled_wiki();
    let mut rows = Vec::new();
    for slug in wiki::list() {
        match wiki::load(&slug) {
            Ok(e) => {
                println!(
                    "  {}  {} {}",
                    style(format!("{:24}", slug)).bold().cyan(),
                    e.frontmatter.title,
                    style(format!("[{}]", e.frontmatter.tags.join(", "))).dim()
                );
                if !e.frontmatter.signature.is_empty() {
                    println!("    {}", style(&e.frontmatter.signature).dim());
                }
                rows.push(json!({
                    "slug": slug,
                    "title": e.frontmatter.title,
                    "url": e.frontmatter.url,
                    "tags": e.frontmatter.tags,
                    "signature": e.frontmatter.signature,
                }));
            }
            Err(e) => {
                eprintln!("  ✖ {slug}: {e}");
            }
        }
    }
    json!({ "kind": "wiki_list", "entries": rows })
}

pub fn show(slug: &str) -> anyhow::Result<serde_json::Value> {
    let _ = wiki::seed_bundled_wiki();
    let e = wiki::load(slug)
        .map_err(|e| anyhow::anyhow!("could not load wiki entry `{slug}`: {e}"))?;
    println!("# {}\n", e.frontmatter.title);
    if !e.frontmatter.url.is_empty() {
        println!("URL: {}", e.frontmatter.url);
    }
    if !e.frontmatter.signature.is_empty() {
        println!("Signature: {}", e.frontmatter.signature);
    }
    if !e.frontmatter.tags.is_empty() {
        println!("Tags: {}", e.frontmatter.tags.join(", "));
    }
    println!();
    println!("{}", e.body);
    Ok(json!({
        "kind": "wiki_show",
        "slug": e.slug,
        "frontmatter": {
            "title": e.frontmatter.title,
            "url": e.frontmatter.url,
            "tags": e.frontmatter.tags,
            "signature": e.frontmatter.signature,
            "ingested_at": e.frontmatter.ingested_at,
        },
        "body": e.body,
    }))
}

pub async fn add(
    url: String,
    slug_override: Option<String>,
    title: Option<String>,
    tags: Vec<String>,
    signature: Option<String>,
    body_text: Option<String>,
    body_from_file: Option<std::path::PathBuf>,
    overwrite: bool,
    auto_fetch: bool,
) -> anyhow::Result<serde_json::Value> {
    // Auto-fetch URL metadata if requested. Failures degrade gracefully —
    // a network hiccup writes a stub instead of bailing the whole command.
    let fetched: Option<wiki::UrlMetadata> = if auto_fetch {
        eprintln!("  → fetching {url} …");
        match aphrodite_generator::wiki_fetch::fetch_url_metadata(&url).await {
            Ok(md) => {
                eprintln!(
                    "  ✓ title={:?} desc={:?} palette={} hints",
                    md.title.as_deref(),
                    md.description.as_deref().map(|s| if s.len() > 60 { format!("{}…", &s[..60]) } else { s.to_string() }),
                    md.palette_hints.len()
                );
                Some(md)
            }
            Err(e) => {
                eprintln!("  ⚠ fetch failed: {e} — falling back to stub");
                None
            }
        }
    } else {
        None
    };

    let slug = slug_override.unwrap_or_else(|| wiki::slug_from_url(&url));
    if slug.is_empty() {
        anyhow::bail!("could not derive a slug from URL `{url}` — pass `--slug` explicitly");
    }
    let existing = wiki::load(&slug).ok();
    if existing.is_some() && !overwrite {
        anyhow::bail!(
            "wiki entry `{slug}` already exists — pass `--overwrite` to replace, or `--slug <new>` to add separately"
        );
    }

    let body = match (body_text, body_from_file) {
        (Some(t), _) => t,
        (None, Some(p)) => std::fs::read_to_string(&p)
            .map_err(|e| anyhow::anyhow!("could not read --body-from-file {}: {e}", p.display()))?,
        (None, None) => {
            // Read from stdin if it's piped AND non-empty; otherwise emit a
            // stub (enriched by --auto-fetch's extracted metadata when
            // present). An empty piped stdin (sub-shell with no input)
            // should not produce an empty body file.
            use std::io::{IsTerminal, Read};
            let mut stdin = std::io::stdin();
            let mut buf = String::new();
            if !stdin.is_terminal() {
                stdin.read_to_string(&mut buf)?;
            }
            if buf.trim().is_empty() {
                eprintln!("  → stub body will be written; edit the file manually to enrich");
                enriched_stub_body(&slug, &url, &title, &signature, &tags, fetched.as_ref())
            } else {
                buf
            }
        }
    };

    let title = title
        .or_else(|| fetched.as_ref().and_then(|m| m.title.clone()))
        .unwrap_or_else(|| slug.replace('-', " "));
    let final_signature = signature
        .or_else(|| fetched.as_ref().and_then(|m| m.description.clone()))
        .unwrap_or_default();
    let entry = aphrodite_core::wiki::WikiEntry {
        slug: slug.clone(),
        frontmatter: aphrodite_core::wiki::WikiFrontmatter {
            title,
            url,
            tags,
            signature: final_signature,
            ingested_at: aphrodite_core::taste::now_rfc3339()[..10].to_string(),
        },
        body,
        path: aphrodite_core::wiki::entry_path(&slug),
    };
    wiki::save(&entry).map_err(|e| anyhow::anyhow!("could not save wiki entry: {e}"))?;
    let path = aphrodite_core::wiki::entry_path(&entry.slug);
    eprintln!("  ✓ wrote {}", path.display());
    Ok(json!({
        "kind": "wiki_add",
        "slug": entry.slug,
        "path": path.to_string_lossy(),
        "frontmatter": {
            "title": entry.frontmatter.title,
            "url": entry.frontmatter.url,
            "tags": entry.frontmatter.tags,
            "signature": entry.frontmatter.signature,
            "ingested_at": entry.frontmatter.ingested_at,
        },
    }))
}

fn enriched_stub_body(
    slug: &str,
    url: &str,
    title: &Option<String>,
    signature: &Option<String>,
    tags: &[String],
    fetched: Option<&wiki::UrlMetadata>,
) -> String {
    let title_display = title
        .clone()
        .or_else(|| fetched.and_then(|m| m.title.clone()))
        .unwrap_or_else(|| slug.replace('-', " "));
    let sig_display = signature
        .clone()
        .or_else(|| fetched.and_then(|m| m.description.clone()))
        .unwrap_or_else(|| "(one-line distillation)".to_string());
    let tags_display = if tags.is_empty() { "(none)".to_string() } else { tags.join(", ") };

    let fetched_block = if let Some(m) = fetched {
        let mut s = String::from("\n## Auto-fetched signals (from URL)\n\n");
        if let Some(t) = &m.title {
            s.push_str(&format!("- Page title: {t}\n"));
        }
        if let Some(d) = &m.description {
            s.push_str(&format!("- Meta description: {d}\n"));
        }
        if let Some(og) = &m.og_image {
            s.push_str(&format!("- OG image: {og}\n"));
        }
        if !m.palette_hints.is_empty() {
            s.push_str(&format!(
                "- Palette hints (hex values from HTML): {}\n",
                m.palette_hints.join(", ")
            ));
        }
        if s.lines().count() <= 2 {
            String::new()
        } else {
            s.push('\n');
            s
        }
    } else {
        String::new()
    };

    format!(
        r##"# {title_display} — why it matters

(TODO: 2-3 sentences naming what this reference is and the design lesson it carries.)
{fetched_block}
## What to absorb

- (concrete pattern 1 — px values / ratios / weights)
- (concrete pattern 2)
- (concrete pattern 3)

## What NOT to copy

- (brand-specific element 1)
- (any thing that only works in their context, not yours)

## Reference fragments worth lifting

- (specific measurement worth quoting)
- (specific measurement worth quoting)

---

stubbed-from: `aphrodite wiki add {url}`
slug: {slug}
tags: {tags_display}
signature: {sig_display}
"##,
    )
}
