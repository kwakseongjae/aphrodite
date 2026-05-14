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
    parse_design, personas, preferences, record_taste, resolve_variants, skills, wiki,
    validate_design, Caller, Invocation, SignalKind, Surface, WriteMode,
};

const REFERENCES_TOP_K: usize = 3;
use aphrodite_generator::{critic, harmonize, refine, surface};
use serde_json::json;
use std::path::PathBuf;
use std::time::Instant;

const MAX_SCAFFOLD_BODY_CHARS: usize = 4_000;
const SCAFFOLD_TOP_K: usize = 3;

pub async fn run(
    intent: String,
    max_turns: u32,
    satisfaction_threshold: f32,
    persona_slug: Option<String>,
    no_write: bool,
    repo: Option<PathBuf>,
) -> anyhow::Result<serde_json::Value> {
    let target = match repo {
        Some(p) => p,
        None => std::env::current_dir()?,
    };
    let target = target.canonicalize().unwrap_or(target);
    let resolved = aphrodite_generator::resolve_default_provider().ok_or_else(|| {
        anyhow::anyhow!("no provider credential reachable. Run `aphrodite doctor`.")
    })?;

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
                personas::as_system_prompt_block(&p)
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
    let output = aphrodite_generator::generate_with_user_intent(
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
            &resolved,
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

        if verdict.satisfied() {
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
        let new_composition = match surface::compose(&resolved, &intent, &new_md, &new_doc).await {
            Ok(out) => {
                llm_calls += 1;
                aphrodite_generator::hero::inject_variant_css(&out.html, &new_doc, &new_variants)
            }
            Err(e) => {
                eprintln!("    ⚠ surface re-compose failed: {e} — keeping prior composition");
                composition_html.clone()
            }
        };
        let new_hero = aphrodite_generator::hero::render(&new_doc, &new_variants)
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
    let final_report = validate_design(&final_doc, &final_variants);

    let (design_path, hero_path, composition_path) = if no_write {
        let out = target.join(".aphrodite").join("out");
        std::fs::create_dir_all(&out)?;
        let dp = out.join("DESIGN.md");
        let hp = out.join("hero.html");
        std::fs::write(&dp, &design_md)?;
        std::fs::write(&hp, &hero_html)?;
        let cp = if !composition_html.is_empty() {
            let cp = out.join("composition.html");
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
        let cp = if !composition_html.is_empty() {
            let cp = target.join("composition.html");
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
            "asset_create": "stub",
            "asset_manage": "stub",
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
        "harmonize": {
            "fonts_injected": harmonize_report.fonts_injected,
            "hero_typography_fixed": harmonize_report.hero_typography_fixed,
            "lucide_labels_recovered": harmonize_report.lucide_labels_recovered,
            "notes": harmonize_report.notes,
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
