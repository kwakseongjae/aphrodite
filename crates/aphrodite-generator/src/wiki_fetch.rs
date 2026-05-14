//! Wiki URL fetcher — lives in the generator crate because core stays
//! HTTP-free and we already have a configured reqwest path here.

use aphrodite_core::wiki::{extract_metadata_from_html, UrlMetadata};
use std::time::Duration;

pub async fn fetch_url_metadata(url: &str) -> anyhow::Result<UrlMetadata> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(15))
        .user_agent("Aphrodite/0.3 (+wiki-ingest)")
        .build()?;
    let resp = client.get(url).send().await?;
    let status = resp.status();
    if !status.is_success() {
        anyhow::bail!("http {} for {url}", status.as_u16());
    }
    let body = resp.text().await?;
    Ok(extract_metadata_from_html(&body))
}

/// Download an image URL to bytes. Used by the create flow to materialise
/// wiki entry og_images into <project>/.aphrodite/assets/refs/.
pub async fn fetch_bytes(url: &str) -> anyhow::Result<(Vec<u8>, String)> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(60))
        .connect_timeout(Duration::from_secs(15))
        .user_agent("Aphrodite/0.3 (+asset-fetch)")
        .build()?;
    let resp = client.get(url).send().await?;
    if !resp.status().is_success() {
        anyhow::bail!("http {} for {url}", resp.status().as_u16());
    }
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    let ext = guess_image_ext(&content_type, url);
    let bytes = resp.bytes().await?.to_vec();
    Ok((bytes, ext))
}

fn guess_image_ext(content_type: &str, url: &str) -> String {
    let ct = content_type.to_ascii_lowercase();
    if ct.contains("png") { return "png".to_string(); }
    if ct.contains("jpeg") || ct.contains("jpg") { return "jpg".to_string(); }
    if ct.contains("webp") { return "webp".to_string(); }
    if ct.contains("svg") { return "svg".to_string(); }
    if ct.contains("gif") { return "gif".to_string(); }
    if ct.contains("avif") { return "avif".to_string(); }
    // Fall back to URL extension.
    let lower = url.to_ascii_lowercase();
    for e in &["png", "jpg", "jpeg", "webp", "svg", "gif", "avif"] {
        if lower.ends_with(&format!(".{e}")) {
            return if *e == "jpeg" { "jpg".into() } else { (*e).into() };
        }
    }
    "bin".to_string()
}
