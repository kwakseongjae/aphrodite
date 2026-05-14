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
