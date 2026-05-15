//! `aphrodite create "<intent>"` — autonomous creation harness entry point
//! (ADR 0004). Thin wrapper over `aphrodite_generator::orchestrator::run`.
//!
//! The orchestration logic lives in the generator crate so both CLI and
//! MCP surfaces share the same execution path.

use aphrodite_core::{personas, skills, wiki};
use serde_json::json;
use std::path::PathBuf;

pub async fn run(
    intent: String,
    max_turns: u32,
    satisfaction_threshold: f32,
    persona_slug: Option<String>,
    no_write: bool,
    repo: Option<PathBuf>,
) -> anyhow::Result<serde_json::Value> {
    aphrodite_generator::orchestrator::run(
        intent,
        max_turns,
        satisfaction_threshold,
        persona_slug,
        no_write,
        repo,
    )
    .await
}

/// Dry-run estimator — runs phase 1 retrieval (wiki + skills + persona) but
/// stops before any provider call. Reports what would be loaded and an
/// estimated LLM-call count + wall-clock based on rolling baselines.
pub fn estimate(
    intent: String,
    max_turns: u32,
    satisfaction_threshold: f32,
    persona_slug: Option<String>,
    _repo: Option<PathBuf>,
) -> anyhow::Result<serde_json::Value> {
    use console::style;
    // Materialise bundled seeds (idempotent) so the estimate matches a real run.
    let _ = personas::seed_bundled_personas();
    let _ = skills::seed_bundled_skills();
    let _ = wiki::seed_bundled_wiki();

    let persona_meta = if let Some(slug) = persona_slug.as_deref() {
        match personas::load(slug) {
            Ok(p) => Some(json!({
                "slug": slug,
                "name": p.frontmatter.name,
                "voice": p.frontmatter.voice,
            })),
            Err(e) => anyhow::bail!("unknown persona `{slug}`: {e}"),
        }
    } else {
        None
    };

    let intent_tags = skills::extract_intent_tags(&intent);
    let intent_tag_refs: Vec<&str> = intent_tags.iter().map(String::as_str).collect();
    let loaded_skills = skills::rank_for_intent(&intent_tag_refs, 3);
    let loaded_skill_slugs: Vec<String> = loaded_skills.iter().map(|s| s.slug.clone()).collect();
    let wiki_entries = wiki::query_by_tags(&intent_tag_refs, 3);
    let loaded_wiki_slugs: Vec<String> = wiki_entries.iter().map(|e| e.slug.clone()).collect();
    let og_image_urls: usize = wiki_entries
        .iter()
        .filter(|e| e.body.lines().any(|l| l.starts_with("- OG image:")))
        .count();

    // LLM call model:
    //   2 fixed (design + surface compose)
    //   per turn: 1 critic. if not satisfied: +1 refine +1 surface re-compose.
    // Optimistic = critic accepts on turn 1 → 2 + 1 = 3 calls
    // Pessimistic = every turn refines → 2 + 3 * max_turns
    let optimistic = 3u32;
    let pessimistic = 2 + 3 * max_turns;
    // Rolling baseline from dogfood: ~250 s for the optimistic 3-call path.
    let s_per_call = 80u32;
    let optimistic_s = optimistic * s_per_call;
    let pessimistic_s = pessimistic * s_per_call;

    println!("{}", style("Aphrodite — create estimate").bold().magenta());
    println!();
    println!("  {} {}", style("intent").dim(), intent);
    println!(
        "  {} {}",
        style("intent tags").dim(),
        if intent_tags.is_empty() { "(none)".into() } else { intent_tags.join(", ") }
    );
    if let Some(p) = persona_meta.as_ref() {
        println!(
            "  {} {} — {}",
            style("persona").dim(),
            p.get("name").and_then(|v| v.as_str()).unwrap_or(""),
            p.get("voice").and_then(|v| v.as_str()).unwrap_or("")
        );
    }
    println!(
        "  {} [{}]",
        style("skills").dim(),
        if loaded_skill_slugs.is_empty() { "(none)".to_string() } else { loaded_skill_slugs.join(", ") }
    );
    println!(
        "  {} [{}]",
        style("wiki refs").dim(),
        if loaded_wiki_slugs.is_empty() { "(none)".to_string() } else { loaded_wiki_slugs.join(", ") }
    );
    println!(
        "  {} {} entries carry og:image (will be fetched in phase 6)",
        style("assets").dim(),
        og_image_urls
    );
    println!();
    println!(
        "  {} {} – {} LLM calls",
        style("call budget").bold().yellow(),
        optimistic,
        pessimistic
    );
    println!(
        "  {} ~{}s – ~{}s ({}s typical critic-accept path)",
        style("wall clock").bold().yellow(),
        optimistic_s,
        pessimistic_s,
        optimistic_s
    );
    println!(
        "  {} max_turns={} threshold={:.2}",
        style("settings").dim(),
        max_turns,
        satisfaction_threshold
    );

    Ok(json!({
        "kind": "create_estimate",
        "intent": intent,
        "intent_tags": intent_tags,
        "persona": persona_meta,
        "loaded_skills": loaded_skill_slugs,
        "loaded_wiki": loaded_wiki_slugs,
        "og_image_count": og_image_urls,
        "estimate": {
            "llm_calls_min": optimistic,
            "llm_calls_max": pessimistic,
            "wall_clock_s_min": optimistic_s,
            "wall_clock_s_max": pessimistic_s,
            "wall_clock_s_typical": optimistic_s,
        },
        "settings": {
            "max_turns": max_turns,
            "satisfaction_threshold": satisfaction_threshold,
        },
    }))
}
