//! `aphrodite gallery <DIR>` — build a single-file gallery.html that previews
//! every run subdirectory under DIR. Each run renders as a card with intent
//! text, metrics, palette swatches, and an iframe of the actual output
//! (`composition.html` preferred, falls back to `hero.html`).

use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

const CARD_CSS: &str = r##"
* { box-sizing: border-box; }
body {
  margin: 0;
  font-family: ui-sans-serif, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  background: #0e0e10; color: #e8e8ea; padding: 24px;
}
header { margin-bottom: 32px; }
h1 { margin: 0 0 8px; letter-spacing: -0.01em; }
.sub { color: #8a8a93; margin: 0 0 6px; }
.grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(420px, 1fr)); gap: 24px; }
.card { background: #18181b; border: 1px solid #27272a; border-radius: 16px; overflow: hidden; display: flex; flex-direction: column; }
.card h2 { font-size: 15px; margin: 0; padding: 14px 18px; background: #1f1f23; border-bottom: 1px solid #27272a; display: flex; align-items: center; justify-content: space-between; gap: 8px; }
.card h2 .slug { color: #a78bfa; font-weight: 600; }
.badge { font-size: 11px; padding: 2px 8px; border-radius: 999px; font-weight: 500; letter-spacing: 0.02em; }
.badge.ok { background: #14532d; color: #86efac; }
.badge.fail { background: #7f1d1d; color: #fca5a5; }
.badge.parse { background: #422006; color: #fcd34d; }
.surface-tag { background: #1e1b4b; color: #a5b4fc; font-size: 10px; padding: 2px 7px; border-radius: 999px; margin-left: 6px; }
.intent { padding: 14px 18px; font-size: 13px; line-height: 1.55; color: #d4d4d8; border-bottom: 1px solid #27272a; white-space: pre-wrap; word-break: break-word; }
.intent::before { content: "INTENT"; display: block; font-size: 10px; letter-spacing: 0.16em; color: #71717a; margin-bottom: 6px; }
.meta { padding: 12px 18px; display: grid; grid-template-columns: auto 1fr; gap: 4px 14px; font-size: 12px; border-bottom: 1px solid #27272a; }
.meta dt { color: #71717a; } .meta dd { margin: 0; color: #e4e4e7; font-family: ui-monospace, "SF Mono", monospace; font-size: 12px; }
.swatches { display: flex; gap: 2px; flex-wrap: wrap; padding: 12px 18px; border-bottom: 1px solid #27272a; }
.swatch { width: 36px; height: 36px; border-radius: 6px; position: relative; cursor: pointer; }
.swatch::after { content: attr(data-hex); position: absolute; bottom: -22px; left: 50%; transform: translateX(-50%); font-size: 10px; font-family: ui-monospace, monospace; white-space: nowrap; color: #71717a; opacity: 0; transition: opacity 120ms; pointer-events: none; }
.swatch:hover::after { opacity: 1; }
.preview { background: #fff; height: 420px; position: relative; overflow: hidden; }
.preview iframe { border: 0; width: 250%; height: 250%; transform: scale(0.4); transform-origin: top left; pointer-events: none; }
.preview-missing { display: flex; align-items: center; justify-content: center; height: 100%; color: #71717a; font-size: 13px; }
.links { padding: 10px 18px; display: flex; gap: 12px; font-size: 12px; background: #18181b; align-items: center; }
.links a { color: #a78bfa; text-decoration: none; } .links a:hover { text-decoration: underline; }
.pill { color: #71717a; margin-left: auto; }
"##;

pub fn build(dir: &Path) -> anyhow::Result<PathBuf> {
    if !dir.is_dir() {
        anyhow::bail!("not a directory: {}", dir.display());
    }
    let mut entries: Vec<_> = fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.path())
        .collect();
    entries.sort();
    if entries.is_empty() {
        anyhow::bail!("no run subdirectories under {}", dir.display());
    }

    let title = dir
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "Gallery".to_string());
    let mut cards = String::new();
    for run_dir in &entries {
        cards.push_str(&card(run_dir));
        cards.push('\n');
    }
    let html = format!(
        "<!DOCTYPE html>\n<html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><title>{title}</title><style>{CARD_CSS}</style></head><body>\n<header><h1>{title}</h1><p class=\"sub\">{n} runs · each card embeds the actual composition.html (or hero.html fallback) scaled · click <em>open ↗</em> for full size</p></header><main class=\"grid\">\n{cards}\n</main></body></html>",
        n = entries.len()
    );
    let out = dir.join("gallery.html");
    fs::write(&out, html)?;
    Ok(out)
}

fn card(run_dir: &Path) -> String {
    let slug = run_dir.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    let intent = fs::read_to_string(run_dir.join("intent.txt"))
        .unwrap_or_else(|_| slug.replace('-', " "))
        .trim()
        .to_string();
    let result_path = run_dir.join("result.json");
    let md_path = run_dir.join("DESIGN.md");
    let composition_path = run_dir.join("composition.html");
    let hero_path = run_dir.join("hero.html");

    let mut provider = "—".to_string();
    let mut model = "—".to_string();
    let mut valid: Option<bool> = None;
    let mut warnings = 0usize;
    let mut surface = String::new();
    if let Ok(s) = fs::read_to_string(&result_path) {
        if !s.is_empty() {
            if let Ok(v) = serde_json::from_str::<Value>(&s) {
                let o = v.get("output").cloned().unwrap_or_default();
                provider = o.get("provider_used").and_then(|v| v.as_str()).unwrap_or("—").to_string();
                model = o.get("model_used").and_then(|v| v.as_str()).unwrap_or("—").to_string();
                valid = o.get("validation").and_then(|v| v.get("ok")).and_then(|v| v.as_bool());
                warnings = o.get("warnings").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
                surface = o.get("surface_type").and_then(|v| v.as_str()).unwrap_or("").to_string();
            }
        }
    }
    let duration = fs::read_to_string(run_dir.join("duration.txt")).map(|s| s.trim().to_string() + "s").unwrap_or_else(|_| "—".to_string());

    let mut palette = Vec::new();
    let mut display_font = "—".to_string();
    let mut body_font = "—".to_string();
    if let Ok(md) = fs::read_to_string(&md_path) {
        if let Some(start) = md.find("primary:") {
            let chunk = &md[start..start.saturating_add(1200).min(md.len())];
            for cap in chunk.split('"').filter(|s| s.starts_with('#') && s.len() == 7) {
                palette.push(cap.to_string());
                if palette.len() >= 10 { break; }
            }
        }
        if let Some(pos) = md.find("display:") {
            let chunk = &md[pos..pos.saturating_add(120).min(md.len())];
            if let Some(line) = chunk.lines().nth(1) {
                if let Some(idx) = line.find("family:") {
                    display_font = line[idx + 7..].trim().trim_matches(&['"', '\''][..]).to_string();
                }
            }
        }
        if let Some(pos) = md.find("body:") {
            let chunk = &md[pos..pos.saturating_add(120).min(md.len())];
            if let Some(line) = chunk.lines().nth(1) {
                if let Some(idx) = line.find("family:") {
                    body_font = line[idx + 7..].trim().trim_matches(&['"', '\''][..]).to_string();
                }
            }
        }
    }

    let badge = match valid {
        Some(true) => r#"<span class="badge ok">valid</span>"#.to_string(),
        Some(false) => r#"<span class="badge fail">WCAG-AA fail</span>"#.to_string(),
        None => r#"<span class="badge parse">parse fail</span>"#.to_string(),
    };
    let surface_chip = if !surface.is_empty() {
        format!(r#"<span class="surface-tag">{surface}</span>"#)
    } else { String::new() };
    let swatches: String = if palette.is_empty() {
        r#"<div style="color:#71717a;font-size:12px">no palette</div>"#.to_string()
    } else {
        palette.iter().map(|h| format!(r#"<div class="swatch" style="background:{h}" data-hex="{h}"></div>"#)).collect::<Vec<_>>().join("")
    };
    let (preview, primary_link_label) = if composition_path.exists() {
        let label = if surface.is_empty() { "composition.html".to_string() } else { format!("{surface}.html") };
        (format!(r#"<iframe src="{}/composition.html" sandbox="allow-same-origin allow-scripts"></iframe>"#, slug), label)
    } else if hero_path.exists() {
        (format!(r#"<iframe src="{}/hero.html" sandbox="allow-same-origin allow-scripts"></iframe>"#, slug), "hero.html".to_string())
    } else {
        (r#"<div class="preview-missing">no rendered output (run failed)</div>"#.to_string(), "—".to_string())
    };

    let mut links = Vec::new();
    if md_path.exists() { links.push(format!(r#"<a href="{slug}/DESIGN.md">DESIGN.md</a>"#)); }
    if composition_path.exists() { links.push(format!(r#"<a href="{slug}/composition.html" target="_blank">open {primary_link_label} ↗</a>"#)); }
    else if hero_path.exists() { links.push(format!(r#"<a href="{slug}/hero.html" target="_blank">open hero ↗</a>"#)); }
    if run_dir.join("intent.txt").exists() { links.push(format!(r#"<a href="{slug}/intent.txt">intent.txt</a>"#)); }
    if result_path.exists() && fs::metadata(&result_path).map(|m| m.len() > 0).unwrap_or(false) {
        links.push(format!(r#"<a href="{slug}/result.json">result.json</a>"#));
    }

    format!(
        r#"<article class="card">
  <h2><span class="slug">{slug}{surface_chip}</span>{badge}</h2>
  <div class="intent">{intent}</div>
  <dl class="meta">
    <dt>provider</dt><dd>{provider}</dd>
    <dt>model</dt><dd>{model}</dd>
    <dt>time</dt><dd>{duration}</dd>
    <dt>display</dt><dd>{display_font}</dd>
    <dt>body</dt><dd>{body_font}</dd>
    <dt>warnings</dt><dd>{warnings}</dd>
  </dl>
  <div class="swatches">{swatches}</div>
  <div class="preview">{preview}</div>
  <div class="links">{links_html}<span class="pill">{slug}</span></div>
</article>"#,
        intent = html_escape(&intent),
        display_font = html_escape(&display_font),
        body_font = html_escape(&body_font),
        links_html = links.join(" &nbsp;·&nbsp; "),
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}
