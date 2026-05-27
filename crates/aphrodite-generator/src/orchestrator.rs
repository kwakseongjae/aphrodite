//! `aphrodite create "<intent>"` — autonomous creation harness entry point
//! (ADR 0004 / Phase 4 prototype).
//!
//! Internal loop:
//!   1. design   — initial generation (same as `aphrodite design`)
//!   2. critique — self-critic identifies weakest axis or declares satisfaction
//!   3. refine   — apply the critic's proposed delta
//!   4. goto 2 until satisfied or `--max-turns` exhausted
//!   5. emit final artifacts + structured turn-by-turn report
//!
//! Future phases (research, asset, harmonizer, skillify) are stubbed at the
//! report-shape level: the JSON output includes `phases: { ... }` so callers
//! can already key off them while the implementations land later.

use aphrodite_core::{
    assets, parse_design, personas, preferences, record_taste, resolve_variants, skills, wiki,
    validate_design, Caller, Invocation, SignalKind, Surface, WriteMode,
};

const REFERENCES_TOP_K: usize = 3;
use crate::{critic, harmonize, refine, surface};
use serde_json::json;
use std::path::PathBuf;
use std::time::Instant;

// Finding #37 mitigation: per-skill scaffold cap reduced from 4000 → 2500.
// At top_k=3 this caps the scaffold section at ~7500 chars total. Reduces
// inference time on long-context inputs without losing "What to absorb /
// What NOT to copy" structure.
const MAX_SCAFFOLD_BODY_CHARS: usize = 2_500;
const SCAFFOLD_TOP_K: usize = 3;
/// Cap on the full persona system-prompt block. Frontmatter principles/
/// rejects/prefers/cjk_strategy always survive (they render first); the
/// trailing "Additional notes from your own writing" gets truncated when
/// the block exceeds this cap.
const MAX_PERSONA_BLOCK_CHARS: usize = 4_000;

pub async fn run(
    intent: String,
    max_turns: u32,
    satisfaction_threshold: f32,
    persona_slug: Option<String>,
    no_write: bool,
    repo: Option<PathBuf>,
    pages: Vec<String>,
) -> anyhow::Result<serde_json::Value> {
    let target = match repo {
        Some(p) => p,
        None => std::env::current_dir()?,
    };
    let target = target.canonicalize().unwrap_or(target);
    let resolved = crate::resolve_default_provider().ok_or_else(|| {
        anyhow::anyhow!(
            "no LLM provider configured — `aphrodite create` needs one (offline mode covers `design` only).\n  \
             Fastest path: `aphrodite init` (2-min guided setup), or set APHRODITE_ZAI_API_KEY in your shell.\n  \
             Diagnose anytime with `aphrodite doctor`. Docs: https://docs.aphrodite.dev/getting-started"
        )
    })?;
    // Per-call model overrides (e.g. ~/.aphrodite/config.toml's
    // [providers.zai] composer_model = "glm-4.5-air"). Falls back to the
    // base model when unset. Used by surface composer and critic to spend
    // a faster tier on translation/critique tasks while keeping the design
    // call on the quality model.
    let composer_resolved = match crate::composer_model_override() {
        Some(m) => {
            eprintln!("● using composer model override: {m}");
            resolved.with_model(m)
        }
        None => resolved.clone(),
    };
    let critic_resolved = match crate::critic_model_override() {
        Some(m) => {
            eprintln!("● using critic model override: {m}");
            resolved.with_model(m)
        }
        None => resolved.clone(),
    };

    let write_mode = if no_write {
        WriteMode::ArtifactOnly
    } else {
        WriteMode::Commit
    };
    let invocation = Invocation {
        id: uuid::Uuid::new_v4().to_string(),
        caller: Caller::Human,
        surface: Surface::Cli,
        intent: intent.clone(),
        target_repo: target.clone(),
        write_mode,
    };

    let started = Instant::now();
    let mut llm_calls = 0u32;

    // ---- Persona authority (optional, outranks generic skill scaffolds) -----
    let _ = personas::seed_bundled_personas();
    let persona_block = if let Some(slug) = persona_slug.as_deref() {
        match personas::load(slug) {
            Ok(p) => {
                eprintln!("● persona / channeling: {} ({})", p.frontmatter.name, slug);
                let full = personas::as_system_prompt_block(&p);
                if full.len() <= MAX_PERSONA_BLOCK_CHARS {
                    full
                } else {
                    let mut truncated: String =
                        full.chars().take(MAX_PERSONA_BLOCK_CHARS).collect();
                    truncated.push_str(
                        "\n…[persona body truncated to fit context window; principles + cjk_strategy preserved above]\n",
                    );
                    truncated
                }
            }
            Err(e) => {
                anyhow::bail!(
                    "unknown persona `{slug}`: {e}. Run `aphrodite personas` to list installed personas."
                );
            }
        }
    } else {
        String::new()
    };

    // ---- Phase 1 (stub) / Phase 2 (taste) / Phase 8a (skill loading) --------
    // Skill loading is the missing piece Pass 8 surfaced (Finding #27).
    // We seed bundled skills, rank by intent-derived tags, and prepare a
    // scaffold block to inject into both the critic's system prompt and the
    // design call's intent.
    let newly_seeded = skills::seed_bundled_skills();
    if !newly_seeded.is_empty() {
        eprintln!(
            "● phase 8a / seeded bundled skills: {}",
            newly_seeded.join(", ")
        );
    }
    let intent_tags = skills::extract_intent_tags(&intent);
    let intent_tag_refs: Vec<&str> = intent_tags.iter().map(String::as_str).collect();
    let loaded_skills = skills::rank_for_intent(&intent_tag_refs, SCAFFOLD_TOP_K);
    let loaded_slugs: Vec<String> = loaded_skills.iter().map(|s| s.slug.clone()).collect();
    if !loaded_skills.is_empty() {
        eprintln!(
            "● phase 1 / loaded skill scaffolds: [{}] for tags [{}]",
            loaded_slugs.join(", "),
            intent_tags.join(", ")
        );
        for slug in &loaded_slugs {
            let _ = skills::bump_view(slug);
        }
    } else if !intent_tags.is_empty() {
        eprintln!(
            "● phase 1 / no installed skills match tags [{}] — judging in vacuum",
            intent_tags.join(", ")
        );
    }
    let scaffold_block = skills::build_scaffold_block(&loaded_skills, MAX_SCAFFOLD_BODY_CHARS);

    // ---- Phase 1: research — Karpathy LLM-Wiki design-reference query ------
    let newly_seeded_wiki = wiki::seed_bundled_wiki();
    if !newly_seeded_wiki.is_empty() {
        eprintln!(
            "● phase 1 / seeded design wiki: {}",
            newly_seeded_wiki.join(", ")
        );
    }
    let wiki_entries = wiki::query_by_tags(&intent_tag_refs, REFERENCES_TOP_K);
    let wiki_slugs: Vec<String> = wiki_entries.iter().map(|e| e.slug.clone()).collect();
    if !wiki_entries.is_empty() {
        eprintln!(
            "● phase 1 / loaded reference materials: [{}]",
            wiki_slugs.join(", ")
        );
    } else if !intent_tag_refs.is_empty() {
        eprintln!(
            "● phase 1 / no wiki entries match tags [{}] — generator works without prior art",
            intent_tags.join(", ")
        );
    }
    let references_block = wiki::render_references_block(&wiki_entries);

    // ---- Phase 6: asset management — materialise wiki og_images ------------
    // Best-effort: fetch each loaded wiki entry's og_image into
    // <project>/.aphrodite/assets/refs/. Failures are silent — refs are
    // a convenience, never blocking.
    let mut materialised_refs: Vec<String> = Vec::new();
    for e in &wiki_entries {
        // og_image is not part of WikiFrontmatter today; we look for it via
        // a generic "extract og_image from body" heuristic. Skip if absent.
        let og = e
            .body
            .lines()
            .find(|l| l.starts_with("- OG image:"))
            .and_then(|l| l.split_whitespace().last())
            .map(|s| s.to_string());
        if let Some(url) = og {
            if url.starts_with("http") {
                match crate::wiki_fetch::fetch_bytes(&url).await {
                    Ok((bytes, ext)) => {
                        let dir = assets::refs_dir(&target);
                        match assets::dedupe_into(&dir, &bytes, &ext) {
                            Ok(p) => {
                                materialised_refs.push(p.to_string_lossy().to_string());
                            }
                            Err(err) => eprintln!("  ⚠ asset dedupe failed for {url}: {err}"),
                        }
                    }
                    Err(err) => {
                        eprintln!("  ⚠ ref fetch failed for {url}: {err}");
                    }
                }
            }
        }
    }
    if !materialised_refs.is_empty() {
        eprintln!(
            "● phase 6 / asset manage: materialised {} reference image(s) into .aphrodite/assets/refs/",
            materialised_refs.len()
        );
    }

    // Combine persona + scaffold + references. Persona first (final authority),
    // then references (concrete prior art), then scaffold (generic checklists).
    let augmentation = {
        let mut parts: Vec<String> = Vec::new();
        if !persona_block.is_empty() {
            parts.push(persona_block.clone());
        }
        if !references_block.is_empty() {
            parts.push(references_block.clone());
        }
        if !scaffold_block.is_empty() {
            parts.push(scaffold_block.clone());
        }
        if parts.is_empty() {
            String::new()
        } else {
            parts.join("\n---\n\n")
        }
    };
    let intent_for_design = if augmentation.is_empty() {
        intent.clone()
    } else {
        format!("{intent}\n\n{augmentation}")
    };
    // Multi-page invocations are ALWAYS web landing pages, never phone-app
    // mockups. Pass 52 Banchan home + Pass 53 Hada home both regressed to
    // surface_type=mobile_app despite explicit landing intent — composer
    // saw "정기 구독 서비스" / "프리랜서 핀테크" and assumed app screen. This
    // pin overrides surface-type detection upstream of the composer call.
    let intent_for_design = if pages.len() > 1 {
        format!(
            "{intent_for_design}\n\n\
             SURFACE-TYPE PIN: this is a multi-page website. Every page MUST be \
             classified as 'landing' / 'pricing' / 'editorial' / 'portfolio' / \
             'dashboard' — NEVER 'mobile_app'. Do not render a phone frame, a 9:41 \
             status bar, or a bottom-tab navigation. Use a desktop+mobile responsive \
             web layout with a top nav and a `<footer>` element.\n\
             Sibling pages this site will have: {siblings}.",
            siblings = pages.join(", "),
        )
    } else {
        intent_for_design
    };
    // The critic also sees both: persona as the authority, scaffold as context.
    let critic_context = augmentation.clone();
    // Replace the invocation's intent for this run so the generator's
    // prefs+intent prompt picks up the scaffold automatically.
    let invocation_for_design = Invocation {
        intent: intent_for_design,
        ..invocation.clone()
    };

    // ---- Phase 3: initial design --------------------------------------------
    eprintln!("● phase 3 / design …");
    let phase3_start = Instant::now();
    let output = crate::generate_with_user_intent(
        &invocation_for_design,
        Some(&resolved),
        Some(&intent), // scan user's original intent for out-of-scope phrases
    )
    .await?;
    llm_calls += 2; // design + surface compose
    let mut design_md = output.design_md;
    let mut composition_html = output.composition_html;
    let mut hero_html = output.hero_html;
    eprintln!(
        "  ✓ design in {:.1}s (provider={}, model={})",
        phase3_start.elapsed().as_secs_f32(),
        output.provider_used,
        output.model_used
    );

    // Guard (Finding #3 → #1): the initial surface compose can occasionally
    // return an empty/stub composition — more likely on a cheaper composer
    // model. If we enter the critic loop with a missing surface, the critic
    // burns its whole first turn just asking for the file to exist (~13 min
    // of wasted wall-clock at observed z.ai latencies in the 1.0.0-beta.1
    // zero-to-one run). Detect a too-small composition and re-compose once
    // with the composer model before the loop, so turn 1 critiques a real
    // surface instead of recovering from a blank one.
    if composition_html.trim().len() < 1000 {
        eprintln!(
            "  ⚠ initial composition too small ({} bytes) — re-composing before critic loop",
            composition_html.trim().len()
        );
        let variants0 = resolve_variants(&output.design_doc);
        match surface::compose(&composer_resolved, &intent, &design_md, &output.design_doc).await {
            Ok(out) if out.html.trim().len() >= 1000 => {
                composition_html =
                    crate::hero::inject_variant_css(&out.html, &output.design_doc, &variants0);
                llm_calls += 1;
                eprintln!("  ✓ recovered composition ({} bytes)", composition_html.len());
            }
            Ok(_) => eprintln!("  ⚠ re-compose still too small — proceeding; critic will guide"),
            Err(e) => eprintln!("  ⚠ re-compose failed: {e} — proceeding"),
        }
    }

    // ---- Phase 4: self-critic refinement loop -------------------------------
    eprintln!("● phase 4 / self-critic refinement (max_turns={max_turns}, threshold={satisfaction_threshold:.2})");
    let prefs = preferences::load(&target);
    let pref_hint = prefs.as_prompt_hint();
    let mut turns: Vec<serde_json::Value> = Vec::new();
    let mut prior_deltas: Vec<String> = Vec::new();
    let mut final_satisfaction = 0.0f32;
    let mut stop_reason = "unknown".to_string();

    for turn in 1..=max_turns {
        let t_start = Instant::now();
        eprintln!("  → turn {turn} / critique …");
        let verdict_result = critic::critique(
            &critic_resolved,
            &intent,
            &design_md,
            &composition_html,
            &prior_deltas,
            &pref_hint,
            &critic_context,
        )
        .await;
        llm_calls += 1;
        let verdict = match verdict_result {
            Ok(v) => v,
            Err(e) => {
                eprintln!("    ⚠ critic failed: {e} — stopping loop");
                stop_reason = format!("critic_error: {e}");
                turns.push(json!({
                    "turn": turn,
                    "phase": "critique",
                    "error": e.to_string(),
                }));
                break;
            }
        };
        eprintln!(
            "    satisfaction={:.2}  axis={}  rationale={:?}",
            verdict.satisfaction,
            verdict.weakest_axis.as_deref().unwrap_or("—"),
            verdict.rationale
        );
        final_satisfaction = verdict.satisfaction;

        // Loop-exit condition: consult the CLI-supplied threshold rather
        // than the hardcoded `verdict.satisfied()` constant. Earlier the
        // CLI flag was only logged, not enforced — so lowering --threshold
        // had no effect and every run consumed all `max_turns`.
        let loop_done = verdict.satisfaction >= satisfaction_threshold
            || verdict.proposed_delta.is_none();
        if loop_done {
            turns.push(json!({
                "turn": turn,
                "phase": "critique",
                "satisfaction": verdict.satisfaction,
                "weakest_axis": verdict.weakest_axis,
                "rationale": verdict.rationale,
                "proposed_delta": null,
                "duration_s": t_start.elapsed().as_secs_f32(),
            }));
            stop_reason = if verdict.satisfaction >= satisfaction_threshold {
                "satisfaction_threshold_met".to_string()
            } else {
                "critic_emitted_no_delta".to_string()
            };
            break;
        }
        let delta = verdict
            .proposed_delta
            .clone()
            .expect("satisfied() guarantees Some when not satisfied");

        eprintln!("  → turn {turn} / refine: {}", short(&delta, 90));
        let refine_start = Instant::now();
        let new_md = match refine::refine(&resolved, &design_md, &delta).await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("    ⚠ refine failed: {e} — stopping loop");
                stop_reason = format!("refine_error: {e}");
                turns.push(json!({
                    "turn": turn,
                    "phase": "refine",
                    "delta": delta,
                    "error": e.to_string(),
                }));
                break;
            }
        };
        llm_calls += 1;
        let new_doc = match parse_design(&new_md) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("    ⚠ refined DESIGN.md failed to parse: {e} — keeping prior turn");
                stop_reason = format!("refine_parse_error: {e}");
                turns.push(json!({
                    "turn": turn,
                    "phase": "refine",
                    "delta": delta,
                    "error": e.to_string(),
                }));
                break;
            }
        };
        let new_variants = resolve_variants(&new_doc);
        let new_composition = match surface::compose(&composer_resolved, &intent, &new_md, &new_doc).await {
            Ok(out) => {
                llm_calls += 1;
                crate::hero::inject_variant_css(&out.html, &new_doc, &new_variants)
            }
            Err(e) => {
                eprintln!("    ⚠ surface re-compose failed: {e} — keeping prior composition");
                composition_html.clone()
            }
        };
        let new_hero = crate::hero::render(&new_doc, &new_variants)
            .map_err(|e| anyhow::anyhow!("hero render: {e}"))?;
        let report = validate_design(&new_doc, &new_variants);

        design_md = new_md;
        composition_html = new_composition;
        hero_html = new_hero;
        prior_deltas.push(delta.clone());

        turns.push(json!({
            "turn": turn,
            "phase": "refine",
            "satisfaction_before": verdict.satisfaction,
            "weakest_axis": verdict.weakest_axis,
            "rationale": verdict.rationale,
            "delta": delta,
            "validation_ok": report.is_ok(),
            "violations": report.violations.len(),
            "duration_s": refine_start.elapsed().as_secs_f32(),
        }));

        if turn == max_turns {
            stop_reason = "max_turns_reached".to_string();
        }
    }

    // ---- Phase 7: harmonize -------------------------------------------------
    // Closes Findings #24 (Google Fonts @import missing) and #26 (hero ignores
    // typography tokens). Pure post-processing, no LLM call.
    let final_doc = parse_design(&design_md)?;
    let final_variants = resolve_variants(&final_doc);
    let (composition_html_h, hero_html_h, harmonize_report) =
        harmonize::harmonize(&composition_html, &hero_html, &design_md, &final_doc);
    composition_html = composition_html_h;
    hero_html = hero_html_h;
    let mut harmonize_bits: Vec<String> = Vec::new();
    if !harmonize_report.fonts_injected.is_empty() {
        harmonize_bits.push(format!(
            "injected fonts [{}]",
            harmonize_report.fonts_injected.join(", ")
        ));
    }
    if harmonize_report.hero_typography_fixed {
        harmonize_bits.push("hero typography hooked up".to_string());
    }
    if !harmonize_report.lucide_labels_recovered.is_empty() {
        harmonize_bits.push(format!(
            "recovered Lucide labels [{}]",
            harmonize_report.lucide_labels_recovered.join(", ")
        ));
    }
    if !harmonize_bits.is_empty() {
        eprintln!("● phase 7 / harmonize: {}", harmonize_bits.join("; "));
    }
    if !harmonize_report.quality_warnings.is_empty() {
        eprintln!("● phase 7 / quality audit — {} concern(s):", harmonize_report.quality_warnings.len());
        for w in &harmonize_report.quality_warnings {
            eprintln!("    ⚠ {w}");
        }
    }
    let qa = &harmonize_report.quality_axes;
    let score_emoji = if harmonize_report.quality_score >= 90 {
        "✓"
    } else if harmonize_report.quality_score >= 75 {
        "•"
    } else {
        "⚠"
    };
    eprintln!(
        "● Aphrodite Quality Score: {} {}/100  (a11y={} mobile={} perf={} semantic={})",
        score_emoji,
        harmonize_report.quality_score,
        qa.a11y,
        qa.mobile,
        qa.performance,
        qa.semantic
    );
    let final_report = validate_design(&final_doc, &final_variants);

    // Composition write rule:
    //   - If composer succeeded AND the rendered HTML body has ≥ 1 KB of real
    //     content, write composition.html.
    //   - Otherwise skip the write — a 200-byte file containing only the
    //     Google Fonts <link> tags (harmonize's only contribution to an
    //     otherwise-empty composer output) is worse than no file: it implies
    //     to the user that composition succeeded.
    let composition_writable =
        composition_html.trim().len() >= 1_024 && composition_html.contains("<body");
    if !composition_writable && !composition_html.is_empty() {
        eprintln!(
            "● composition.html skipped: only {} bytes after harmonize (composer call likely failed). DESIGN.md + hero.html still emitted.",
            composition_html.trim().len()
        );
    }
    let (design_path, hero_path, composition_path) = if no_write {
        let out = target.join(".aphrodite").join("out");
        std::fs::create_dir_all(&out)?;
        let dp = out.join("DESIGN.md");
        let hp = out.join("hero.html");
        std::fs::write(&dp, &design_md)?;
        std::fs::write(&hp, &hero_html)?;
        let cp = if composition_writable {
            let cp = out.join(primary_page_filename(&pages));
            std::fs::write(&cp, &composition_html)?;
            Some(cp)
        } else {
            None
        };
        (dp, hp, cp)
    } else {
        let dp = target.join("DESIGN.md");
        let hp = target.join("hero.html");
        std::fs::write(&dp, &design_md)?;
        std::fs::write(&hp, &hero_html)?;
        let cp = if composition_writable {
            let cp = target.join(primary_page_filename(&pages));
            std::fs::write(&cp, &composition_html)?;
            Some(cp)
        } else {
            None
        };
        // Persist intent for future refine calls.
        let _ = std::fs::create_dir_all(target.join(".aphrodite"));
        let _ = std::fs::write(target.join(".aphrodite").join("intent.txt"), &intent);
        // Single auto-commit summarising the whole create run.
        if target.join(".git").exists() {
            use std::process::{Command, Stdio};
            let mut paths = vec![dp.clone(), hp.clone()];
            if let Some(c) = cp.as_ref() {
                paths.push(c.clone());
            }
            for f in &paths {
                let _ = Command::new("git")
                    .arg("-C")
                    .arg(&target)
                    .arg("add")
                    .arg(f)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();
            }
            let _ = Command::new("git")
                .arg("-C")
                .arg(&target)
                .arg("commit")
                .arg("-m")
                .arg(format!(
                    "Aphrodite create: {} ({} turns, satisfaction {:.2})",
                    short(&intent, 50),
                    prior_deltas.len(),
                    final_satisfaction
                ))
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
        (dp, hp, cp)
    };

    // Phase 8.4: secondary-page composition loop (v0.5 multi-page).
    // If `pages` lists more than one slug, re-call the composer for each
    // additional slug using the SAME DESIGN.md + variants. We skip the
    // expensive design / critic / refine phases — those happen once for
    // the primary page. Each secondary page goes through harmonize and
    // gets its own HTML file in the target directory.
    let mut secondary_pages_written: Vec<std::path::PathBuf> = Vec::new();
    if pages.len() > 1 && !no_write {
        for slug in pages.iter().skip(1) {
            let slug = sanitize_page_slug(slug);
            if slug.is_empty() {
                continue;
            }
            let page_intent = format!(
                "{intent} — additional page: `{slug}`. \
                 IMPORTANT — this is a desktop+mobile RESPONSIVE WEB PAGE for a multi-page website, \
                 NOT a phone-app mockup. Do NOT wrap content in a phone frame. Do NOT render a \
                 mobile status bar (9:41, signal bars) or a bottom-tab nav bar — those belong only \
                 in `mobile_app` surface type which is not appropriate here. \
                 Reuse the existing DESIGN.md tokens, components, top nav (with links to ALL sibling \
                 pages including this one), footer (use `<footer>` tag NOT `<div class=\"footer\">`), \
                 and variant switcher. Compose the page content (hero + body + footer) for the \
                 `{slug}` page only. The chrome (nav + footer + switcher) must visually match the \
                 primary page so this works as a section of the same multi-page site. \
                 Sibling pages: {siblings}.",
                siblings = pages.join(", "),
            );
            eprintln!("● phase 8.4 / secondary page `{slug}` — composing");
            let page_compose = surface::compose(
                &composer_resolved,
                &page_intent,
                &design_md,
                &final_doc,
            )
            .await;
            match page_compose {
                Ok(out) => {
                    // Pass 50 surfaced: secondary pages were not getting
                    // the variant CSS injected, so their `data-variant`
                    // switcher buttons fell back to the unscoped rule
                    // and re-introduced the "dark button invisible" bug.
                    let page_with_variants = crate::hero::inject_variant_css(&out.html, &final_doc, &final_variants);
                    let (page_html_h, _, _) = harmonize::harmonize(
                        &page_with_variants,
                        &hero_html,
                        &design_md,
                        &final_doc,
                    );
                    if page_html_h.trim().len() >= 1_024 && page_html_h.contains("<body") {
                        let page_path = target.join(format!("{slug}.html"));
                        if std::fs::write(&page_path, &page_html_h).is_ok() {
                            secondary_pages_written.push(page_path);
                        }
                    } else {
                        eprintln!("    ⚠ secondary page `{slug}` produced < 1 KB — skipped write");
                    }
                }
                Err(e) => {
                    eprintln!("    ⚠ secondary page `{slug}` failed: {e}");
                }
            }
        }
        if !secondary_pages_written.is_empty() {
            let _ = write_sitemap(&target, composition_path.as_deref(), &secondary_pages_written);
        }
    }

    // Phase 8.5: visual capture across three viewports (mobile / tablet /
    // desktop). Korean production targets are mobile-first, so a single
    // 1440px shot lies — it can hide a layout that collapses at 360px.
    // Failures are silent; a missing screenshot is never a run failure.
    let screenshot_paths = if let Some(comp_path) = composition_path.as_ref() {
        capture_screenshots(comp_path)
    } else {
        Vec::new()
    };
    if !screenshot_paths.is_empty() {
        eprintln!(
            "● phase 8.5 / visual capture: {} viewport(s) — {}",
            screenshot_paths.len(),
            screenshot_paths
                .iter()
                .map(|p| p.file_name().map(|f| f.to_string_lossy().into_owned()).unwrap_or_default())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    // Also capture secondary pages — each one gets the 3-viewport set
    // so the gallery shows real mobile/tablet/desktop evidence.
    for sp in &secondary_pages_written {
        let n = capture_screenshots(sp).len();
        if n > 0 {
            eprintln!(
                "● phase 8.5 / visual capture (secondary `{}`): {n} viewport(s)",
                sp.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default()
            );
        }
    }

    // Phase 8.7: design-system handoff. Emit tokens.css, tokens.json
    // (Style-Dictionary shape), and components.html — the artifacts a
    // designer / FE engineer at a Korean production shop would need to
    // adopt this run as the source of truth. Zero LLM cost; fully
    // deterministic from the resolved variants.
    if !no_write {
        let ds_target: &std::path::Path = &target;
        let css_path = ds_target.join("tokens.css");
        let json_path = ds_target.join("tokens.json");
        let comp_path = ds_target.join("components.html");
        let _ = std::fs::write(&css_path, crate::design_system::build_tokens_css(&final_variants));
        let _ = std::fs::write(&json_path, crate::design_system::build_tokens_json(&final_variants));
        let _ = std::fs::write(&comp_path, crate::design_system::build_components_html(&final_variants));
        // Tokens Studio JSON for Figma — designers import via the
        // Tokens Studio plugin (free, no Enterprise plan needed).
        let figma_path = ds_target.join("tokens.figma.json");
        let _ = std::fs::write(&figma_path, crate::figma_sync::build_tokens_studio_json(&final_variants));
        // v1.0 RC.6 docs site — material-ui style single-page docs.
        let docs_dir = ds_target.join("docs");
        let _ = std::fs::create_dir_all(&docs_dir);
        let project_name = if final_doc.frontmatter.name.is_empty() {
            "aphrodite-design"
        } else {
            final_doc.frontmatter.name.as_str()
        };
        let _ = std::fs::write(
            docs_dir.join("index.html"),
            crate::docs_site::build_docs_index(&final_variants, project_name),
        );
        eprintln!(
            "● phase 8.7 / design-system handoff: tokens.css, tokens.json, tokens.figma.json, components.html, docs/"
        );
        // Capture the components.html preview at 3 viewports too.
        let _ = capture_screenshots(&comp_path);
        // v1.0 React package: publishable npm package with typed
        // primitives. Lives in `react/` so it doesn't clash with any
        // existing project files. FE engineers can `cd react && npm
        // publish` (or vendor into a monorepo) immediately.
        let react_root = ds_target.join("react");
        let project_name = final_doc.frontmatter.name.as_str();
        let project_name = if project_name.is_empty() { "aphrodite-design" } else { project_name };
        let pkg = crate::react_export::build(&final_variants, project_name);
        if pkg.write_to(&react_root).is_ok() {
            eprintln!(
                "● phase 8.7 / react package: {} files in react/ ({} components + tokens.ts + package.json)",
                pkg.files.len(),
                pkg.files.keys().filter(|k| k.ends_with(".tsx")).count()
            );
        }
    }

    // Phase 8.6: optional external audits. If the user has installed
    // `lighthouse` (npm i -g lighthouse) and/or `axe` (npm i -g
    // @axe-core/cli) globally, run them against the composition and
    // surface category scores. Both are completely optional — a missing
    // binary is not a warning, just a no-op. Future v0.8 phase will
    // bundle these as managed deps.
    if let Some(comp_path) = composition_path.as_ref() {
        let external_audits = run_external_audits(comp_path);
        for line in &external_audits {
            eprintln!("● phase 8.6 / external audit: {line}");
        }
    }

    // Single Accept (or Regenerate if no turns made it satisfied) taste event
    // covering the create run as a whole. Per-turn events would double-count.
    let signal = if final_satisfaction >= satisfaction_threshold && prior_deltas.is_empty() {
        SignalKind::Accept
    } else {
        SignalKind::Regenerate
    };
    let _ = record_taste(
        &invocation,
        signal,
        json!({
            "kind": "create",
            "intent": intent,
            "turns": prior_deltas.len(),
            "satisfaction": final_satisfaction,
            "provider": output.provider_used,
        }),
    );

    // Bump use-counter on every scaffold-loaded skill that survived the run.
    // A successful run is evidence the skill is useful; curator (future) will
    // weight by use_count vs view_count.
    if final_satisfaction >= satisfaction_threshold {
        for slug in &loaded_slugs {
            let _ = skills::bump_use(slug);
        }
    }

    // ---- Phase 8b: skillify proposal ----------------------------------------
    // We DO NOT auto-write a new skill. We surface a `proposed_skill` block
    // in the JSON. The user can manually `aphrodite skill propose <slug>` to
    // accept (verb to be added later). Avoids auto-creating skill bloat from
    // shallow trajectories. ADR 0004 §"v0.3 ships skills without curator".
    let proposed_skill = if prior_deltas.len() >= 3 && final_satisfaction >= satisfaction_threshold
    {
        let slug = slugify(&intent, 6);
        Some(json!({
            "slug": slug,
            "rationale": format!(
                "Trajectory was non-trivial ({} refines, satisfaction {:.2}); workflow worth preserving for future runs.",
                prior_deltas.len(),
                final_satisfaction
            ),
            "draft_frontmatter": {
                "name": slug,
                "description": short(&intent, 100),
                "version": "1.0.0",
                "tags": intent_tags,
                "agent_created": true,
            },
            "captured_deltas": prior_deltas,
        }))
    } else {
        None
    };

    let total_s = started.elapsed().as_secs_f32();
    eprintln!(
        "● done: {} turns, satisfaction={:.2}, total={:.1}s, llm_calls={}",
        prior_deltas.len(),
        final_satisfaction,
        total_s,
        llm_calls
    );

    Ok(json!({
        "kind": "create",
        "invocation_id": invocation.id,
        "intent": intent,
        "output": {
            "design_path": design_path.to_string_lossy(),
            "hero_path": hero_path.to_string_lossy(),
            "composition_path": composition_path.as_ref().map(|p| p.to_string_lossy().to_string()),
            "validation": {
                "ok": final_report.is_ok(),
                "violations": final_report.violations,
            },
            "warnings": output.warnings,
        },
        "loop": {
            "turns_completed": prior_deltas.len(),
            "max_turns": max_turns,
            "final_satisfaction": final_satisfaction,
            "satisfaction_threshold": satisfaction_threshold,
            "stop_reason": stop_reason,
            "deltas": prior_deltas,
            "turn_log": turns,
        },
        "phases": {
            "research": if wiki_slugs.is_empty() { "no_matching_wiki_entries" } else { "applied" },
            "skill_loading": if loaded_slugs.is_empty() {
                if intent_tags.is_empty() { "no_intent_tags" } else { "no_matching_skills" }
            } else { "applied" },
            "taste_application": if pref_hint.is_empty() { "no_prior_signal" } else { "applied" },
            "design": "done",
            "self_critic_refine": "done",
            "asset_create": "done",
            "asset_manage": if materialised_refs.is_empty() { "noop" } else { "applied" },
            "harmonize": if harmonize_report.fonts_injected.is_empty() && !harmonize_report.hero_typography_fixed {
                "noop"
            } else { "applied" },
            "skillify": if proposed_skill.is_some() { "proposed" } else { "below_threshold" },
        },
        "skills": {
            "intent_tags": intent_tags,
            "loaded": loaded_slugs,
            "newly_seeded": newly_seeded,
            "proposed_new": proposed_skill,
        },
        "persona": persona_slug,
        "research": {
            "wiki_entries_loaded": wiki_slugs,
            "newly_seeded_wiki": newly_seeded_wiki,
        },
        "assets": {
            "materialised_refs": materialised_refs,
        },
        "harmonize": {
            "fonts_injected": harmonize_report.fonts_injected,
            "hero_typography_fixed": harmonize_report.hero_typography_fixed,
            "lucide_labels_recovered": harmonize_report.lucide_labels_recovered,
            "notes": harmonize_report.notes,
            "quality_warnings": harmonize_report.quality_warnings,
        },
        "quality_score": harmonize_report.quality_score,
        "quality_axes": {
            "a11y": harmonize_report.quality_axes.a11y,
            "mobile": harmonize_report.quality_axes.mobile,
            "performance": harmonize_report.quality_axes.performance,
            "semantic": harmonize_report.quality_axes.semantic,
        },
        "telemetry": {
            "llm_calls": llm_calls,
            "wall_clock_s": total_s,
            "provider": output.provider_used,
            "model": output.model_used,
        },
    }))
}

fn short(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(max - 1).collect();
        out.push('…');
        out
    }
}

/// Render composition.html across mobile / tablet / desktop viewports.
/// Returns paths of successfully written screenshots — empty Vec means
/// no Chrome binary was found or every capture failed.
///
/// Honours `APHRODITE_NO_SCREENSHOT=1` for runs that don't want it
/// (e.g. CI without a display server).
fn capture_screenshots(composition_path: &std::path::Path) -> Vec<std::path::PathBuf> {
    if std::env::var_os("APHRODITE_NO_SCREENSHOT").is_some() {
        return Vec::new();
    }
    let chrome_candidates = [
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/Applications/Chromium.app/Contents/MacOS/Chromium",
        "/Applications/Brave Browser.app/Contents/MacOS/Brave Browser",
    ];
    let Some(chrome) = chrome_candidates.iter().find(|p| std::path::Path::new(p).exists()) else {
        return Vec::new();
    };
    // (suffix, width, height). 360x800 ≈ Galaxy S23, 768x1024 = iPad,
    // 1440x1800 = standard laptop. Heights are tall enough to capture
    // the hero + first 2-3 sections — full page scroll is not the goal
    // here; layout collapse detection is.
    let viewports = [
        ("mobile", 360u32, 800u32),
        ("tablet", 768, 1024),
        ("desktop", 1440, 1800),
    ];
    let file_url = format!("file://{}", composition_path.display());
    let stem = composition_path.with_extension("");
    let mut out = Vec::new();
    for (suffix, w, h) in viewports {
        let png_path = stem.with_file_name(format!(
            "{}-{suffix}.png",
            stem.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| "composition".into())
        ));
        let status = std::process::Command::new(chrome)
            .arg("--headless")
            .arg("--disable-gpu")
            .arg("--no-sandbox")
            .arg(format!("--screenshot={}", png_path.display()))
            .arg(format!("--window-size={w},{h}"))
            .arg(&file_url)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
        if let Ok(s) = status {
            if s.success() && png_path.exists() {
                out.push(png_path);
            }
        }
    }
    // Also write a canonical composition.png aliased to the desktop
    // capture, so downstream tooling that expects the legacy file
    // path still works.
    if let Some(desktop_png) = out.iter().find(|p| {
        p.file_name()
            .map(|n| n.to_string_lossy().contains("-desktop"))
            .unwrap_or(false)
    }) {
        let canonical = composition_path.with_extension("png");
        if std::fs::copy(desktop_png, &canonical).is_ok() {
            out.push(canonical);
        }
    }
    out
}

/// Filename for the primary page. `pages[0]` if provided, else
/// "composition.html" for backward compatibility with single-page runs.
fn primary_page_filename(pages: &[String]) -> String {
    pages
        .first()
        .map(|s| sanitize_page_slug(s))
        .filter(|s| !s.is_empty())
        .map(|s| format!("{s}.html"))
        .unwrap_or_else(|| "composition.html".into())
}

/// Lowercase + ascii-letters / digits / hyphens / underscores only.
/// Rejects path separators and traversal patterns.
fn sanitize_page_slug(s: &str) -> String {
    s.trim()
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect()
}

/// Emit a minimal sitemap.xml listing every written page so external
/// crawlers + the harness gallery view can enumerate the multi-page
/// site without filesystem traversal.
fn write_sitemap(
    target: &std::path::Path,
    primary: Option<&std::path::Path>,
    secondaries: &[std::path::PathBuf],
) -> std::io::Result<()> {
    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );
    let push_loc = |xml: &mut String, p: &std::path::Path| {
        let fname = p.file_name().and_then(|s| s.to_str()).unwrap_or_default();
        xml.push_str(&format!("  <url><loc>{fname}</loc></url>\n"));
    };
    if let Some(p) = primary {
        push_loc(&mut xml, p);
    }
    for p in secondaries {
        push_loc(&mut xml, p);
    }
    xml.push_str("</urlset>\n");
    std::fs::write(target.join("sitemap.xml"), xml)
}

/// Optional external audits: lighthouse + axe. Both pulled from PATH;
/// silent no-op if absent. Returns one-line summary strings for the
/// audits that ran.
fn run_external_audits(composition_path: &std::path::Path) -> Vec<String> {
    use std::process::{Command, Stdio};
    let file_url = format!("file://{}", composition_path.display());
    let mut summaries = Vec::new();

    // Lighthouse — only run if installed globally (npx --no-install
    // returns immediately when the package isn't local).
    if Command::new("lighthouse")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        let json_path = composition_path.with_extension("lighthouse.json");
        let ok = Command::new("lighthouse")
            .arg(&file_url)
            .arg("--quiet")
            .arg("--chrome-flags=--headless --no-sandbox")
            .arg("--output=json")
            .arg(format!("--output-path={}", json_path.display()))
            .arg("--only-categories=performance,accessibility,best-practices,seo")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if ok {
            if let Ok(raw) = std::fs::read_to_string(&json_path) {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&raw) {
                    let cats = &v["categories"];
                    let score = |k: &str| -> u32 {
                        (cats[k]["score"].as_f64().unwrap_or(0.0) * 100.0) as u32
                    };
                    summaries.push(format!(
                        "lighthouse perf={} a11y={} bp={} seo={}",
                        score("performance"),
                        score("accessibility"),
                        score("best-practices"),
                        score("seo")
                    ));
                }
            }
        }
    }

    // axe via @axe-core/cli — same opt-in pattern.
    if Command::new("axe")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        let out = Command::new("axe")
            .arg(&file_url)
            .arg("--no-reporter")
            .arg("--stdout")
            .output();
        if let Ok(o) = out {
            let stdout = String::from_utf8_lossy(&o.stdout);
            let violations = stdout.lines().filter(|l| l.contains("violation")).count();
            summaries.push(format!("axe {violations} violation(s)"));
        }
    }

    summaries
}

/// Cheap slug-from-intent: take the first `word_cap` significant words,
/// lowercase + hyphenate. Stopwords stripped. Falls back to "agent-created-skill"
/// if nothing remains.
fn slugify(intent: &str, word_cap: usize) -> String {
    const STOP: &[&str] = &[
        "a", "an", "the", "for", "of", "to", "with", "in", "on", "and", "or", "site", "page",
    ];
    let words: Vec<String> = intent
        .to_ascii_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty() && !STOP.contains(w))
        .take(word_cap)
        .map(|s| s.to_string())
        .collect();
    if words.is_empty() {
        "agent-created-skill".to_string()
    } else {
        words.join("-")
    }
}
