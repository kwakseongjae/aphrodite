//! Skill curator — periodic stale/archive review of agent-created skills.
//!
//! Adapted from Hermes Agent's `agent/curator.py` (1781 lines, ours is smaller
//! by scope: we only do automatic state transitions, no LLM review pass yet).
//! Only touches skills with `agent_created: true` in frontmatter; user-installed
//! or bundled skills are never auto-mutated.
//!
//! Rules:
//!   - `stale_after_days` (default 30): if `latest_activity_at` older than this
//!     and skill is in `Active` state, transition to `Stale`.
//!   - `archive_after_days` (default 90): if older still, move skill to
//!     `~/.aphrodite/skills/_archive/<slug>/` and set state = `Archived`.
//!
//! Output: a `~/.aphrodite/curator/last-run.md` report + a JSON-friendly
//! `CuratorReport` returned to the caller. Idempotent — re-running on the
//! same day produces the same transitions.

use crate::skills::{self, SkillState, UsageRecord};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuratorConfig {
    pub stale_after_days: u32,
    pub archive_after_days: u32,
}

impl Default for CuratorConfig {
    fn default() -> Self {
        Self {
            stale_after_days: 30,
            archive_after_days: 90,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuratorTransition {
    pub slug: String,
    pub from_state: SkillState,
    pub to_state: SkillState,
    pub last_activity_at: Option<String>,
    pub days_inactive: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CuratorReport {
    pub config: CuratorConfig,
    pub now: String,
    pub dry_run: bool,
    pub examined: u32,
    pub skipped_non_agent_created: u32,
    pub stale_transitions: Vec<CuratorTransition>,
    pub archive_transitions: Vec<CuratorTransition>,
    pub already_stale_kept: u32,
    pub already_archived_kept: u32,
    pub active_kept: u32,
}

pub fn curator_root() -> PathBuf {
    let home = std::env::var_os("HOME").unwrap_or_default();
    let mut p = PathBuf::from(home);
    p.push(".aphrodite");
    p.push("curator");
    let _ = fs::create_dir_all(&p);
    p
}

pub fn report_path() -> PathBuf {
    let mut p = curator_root();
    p.push("last-run.md");
    p
}

/// Run one curator pass.
pub fn run(config: CuratorConfig, dry_run: bool) -> Result<CuratorReport, std::io::Error> {
    let now_str = crate::taste::now_rfc3339();
    let now_epoch = parse_rfc3339_epoch(&now_str).unwrap_or(0);
    let stale_secs = (config.stale_after_days as i64) * 86_400;
    let archive_secs = (config.archive_after_days as i64) * 86_400;

    let mut report = CuratorReport {
        config: config.clone(),
        now: now_str.clone(),
        dry_run,
        ..Default::default()
    };

    let usage_store = skills::load_usage();

    for slug in skills::list() {
        let skill = match skills::load(&slug) {
            Ok(s) => s,
            Err(_) => continue,
        };
        report.examined += 1;
        if !skill.frontmatter.agent_created {
            report.skipped_non_agent_created += 1;
            continue;
        }
        let rec: UsageRecord = usage_store.records.get(&slug).cloned().unwrap_or_default();
        if rec.pinned {
            report.active_kept += 1;
            continue;
        }
        match rec.state {
            SkillState::Archived => {
                report.already_archived_kept += 1;
                continue;
            }
            SkillState::Stale => {
                // Check whether stale → archive transition is due.
                let act = rec.latest_activity_at();
                let inactive = act
                    .and_then(|s| parse_rfc3339_epoch(s))
                    .map(|ts| now_epoch - ts);
                if let Some(secs) = inactive {
                    if secs >= archive_secs {
                        if !dry_run {
                            let _ = skills::archive(&slug);
                        }
                        report.archive_transitions.push(CuratorTransition {
                            slug: slug.clone(),
                            from_state: SkillState::Stale,
                            to_state: SkillState::Archived,
                            last_activity_at: act.map(String::from),
                            days_inactive: Some(secs / 86_400),
                        });
                        continue;
                    }
                }
                report.already_stale_kept += 1;
            }
            SkillState::Active => {
                let act = rec.latest_activity_at();
                let inactive = act
                    .and_then(|s| parse_rfc3339_epoch(s))
                    .map(|ts| now_epoch - ts);
                // No recorded activity → treat as inactive forever.
                let inactive_secs = inactive.unwrap_or(i64::MAX);
                if inactive_secs >= archive_secs {
                    // Skip the stale intermediate; go straight to archive.
                    if !dry_run {
                        let _ = skills::archive(&slug);
                    }
                    report.archive_transitions.push(CuratorTransition {
                        slug: slug.clone(),
                        from_state: SkillState::Active,
                        to_state: SkillState::Archived,
                        last_activity_at: act.map(String::from),
                        days_inactive: inactive.map(|s| s / 86_400),
                    });
                } else if inactive_secs >= stale_secs {
                    if !dry_run {
                        let _ = skills::set_state(&slug, SkillState::Stale);
                    }
                    report.stale_transitions.push(CuratorTransition {
                        slug: slug.clone(),
                        from_state: SkillState::Active,
                        to_state: SkillState::Stale,
                        last_activity_at: act.map(String::from),
                        days_inactive: inactive.map(|s| s / 86_400),
                    });
                } else {
                    report.active_kept += 1;
                }
            }
        }
    }

    // Write the human-readable report.
    let md = render_report_md(&report);
    if !dry_run {
        let _ = fs::write(report_path(), &md);
    }

    Ok(report)
}

fn render_report_md(r: &CuratorReport) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "# Curator run — {}{}\n\n",
        r.now,
        if r.dry_run { " (DRY-RUN)" } else { "" }
    ));
    out.push_str(&format!(
        "Examined: {} · skipped (non-agent-created): {}\n",
        r.examined, r.skipped_non_agent_created
    ));
    out.push_str(&format!(
        "Active kept: {} · already-stale kept: {} · already-archived kept: {}\n\n",
        r.active_kept, r.already_stale_kept, r.already_archived_kept
    ));
    if !r.stale_transitions.is_empty() {
        out.push_str("## Active → Stale\n");
        for t in &r.stale_transitions {
            out.push_str(&format!(
                "- `{}` (inactive {} days)\n",
                t.slug,
                t.days_inactive
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "never used".into())
            ));
        }
        out.push('\n');
    }
    if !r.archive_transitions.is_empty() {
        out.push_str("## → Archived\n");
        for t in &r.archive_transitions {
            out.push_str(&format!(
                "- `{}` (from {:?}, inactive {} days)\n",
                t.slug,
                t.from_state,
                t.days_inactive
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "never used".into())
            ));
        }
        out.push('\n');
    }
    out.push_str(&format!(
        "## Config\n\n- stale_after_days: {}\n- archive_after_days: {}\n",
        r.config.stale_after_days, r.config.archive_after_days
    ));
    out
}

/// RFC3339 → unix epoch parser using the `time` crate (already a core dep).
fn parse_rfc3339_epoch(s: &str) -> Option<i64> {
    let dt = time::OffsetDateTime::parse(s.trim(), &time::format_description::well_known::Rfc3339)
        .ok()?;
    Some(dt.unix_timestamp())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skills::{Skill, SkillFrontmatter};
    use std::path::PathBuf as PB;

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

    fn agent_skill(slug: &str, agent_created: bool) -> Skill {
        Skill {
            slug: slug.to_string(),
            frontmatter: SkillFrontmatter {
                name: slug.to_string(),
                description: "test".into(),
                version: "1.0.0".into(),
                tags: vec!["test".into()],
                related_skills: vec![],
                agent_created,
                default: false,
            },
            body: "# body".into(),
            path: skills::skill_md_path(slug),
        }
    }

    fn now_epoch() -> i64 {
        parse_rfc3339_epoch(&crate::taste::now_rfc3339()).unwrap()
    }
    fn rfc3339(epoch: i64) -> String {
        let dt = time::OffsetDateTime::from_unix_timestamp(epoch).unwrap();
        dt.format(&time::format_description::well_known::Rfc3339)
            .unwrap()
    }

    #[test]
    fn parse_and_reverse_roundtrip() {
        let s = "2026-05-15T12:34:56Z";
        let e = parse_rfc3339_epoch(s).unwrap();
        let back = rfc3339(e);
        assert_eq!(back, s);
    }

    #[test]
    fn fresh_agent_skill_stays_active() {
        let _s = Scratch::new();
        skills::save(&agent_skill("fresh", true)).unwrap();
        skills::bump_use("fresh").unwrap();
        let r = run(CuratorConfig::default(), true).unwrap();
        assert_eq!(r.examined, 1);
        assert_eq!(r.active_kept, 1);
        assert!(r.stale_transitions.is_empty());
        assert!(r.archive_transitions.is_empty());
    }

    #[test]
    fn non_agent_created_is_skipped() {
        let _s = Scratch::new();
        skills::save(&agent_skill("user-installed", false)).unwrap();
        let r = run(CuratorConfig::default(), true).unwrap();
        assert_eq!(r.examined, 1);
        assert_eq!(r.skipped_non_agent_created, 1);
        assert!(r.stale_transitions.is_empty());
    }

    #[test]
    fn never_used_agent_skill_archives_immediately_in_dry_run() {
        let _s = Scratch::new();
        skills::save(&agent_skill("never-touched", true)).unwrap();
        // No bump_use ever called — record has no latest_activity_at.
        let r = run(CuratorConfig::default(), true).unwrap();
        assert_eq!(r.archive_transitions.len(), 1);
        assert_eq!(r.archive_transitions[0].slug, "never-touched");
    }

    #[test]
    fn stale_after_30_days() {
        let _s = Scratch::new();
        skills::save(&agent_skill("aged", true)).unwrap();
        // Forge a usage record with an old last_use_at
        let now = now_epoch();
        let old = rfc3339(now - 40 * 86_400);
        let mut store = skills::load_usage();
        let rec = store.records.entry("aged".to_string()).or_default();
        rec.use_count = 5;
        rec.last_use_at = Some(old);
        skills::save_usage(&store).unwrap();

        let r = run(CuratorConfig::default(), true).unwrap();
        assert_eq!(r.stale_transitions.len(), 1);
        assert_eq!(r.stale_transitions[0].slug, "aged");
    }

    #[test]
    fn archive_after_90_days() {
        let _s = Scratch::new();
        skills::save(&agent_skill("ancient", true)).unwrap();
        let now = now_epoch();
        let very_old = rfc3339(now - 120 * 86_400);
        let mut store = skills::load_usage();
        let rec = store.records.entry("ancient".to_string()).or_default();
        rec.use_count = 1;
        rec.last_use_at = Some(very_old);
        skills::save_usage(&store).unwrap();

        let r = run(CuratorConfig::default(), false).unwrap(); // real run
        assert_eq!(r.archive_transitions.len(), 1);
        // skill should be moved to _archive
        let archived = skills::list();
        assert!(!archived.contains(&"ancient".to_string()), "should be archived");
        // Re-running is idempotent
        let r2 = run(CuratorConfig::default(), false).unwrap();
        assert!(r2.archive_transitions.is_empty());
        let _ = PB::from("dummy"); // silence unused-import
    }

    #[test]
    fn pinned_skill_never_touched() {
        let _s = Scratch::new();
        skills::save(&agent_skill("pinned-old", true)).unwrap();
        let now = now_epoch();
        let very_old = rfc3339(now - 200 * 86_400);
        let mut store = skills::load_usage();
        let rec = store.records.entry("pinned-old".to_string()).or_default();
        rec.pinned = true;
        rec.last_use_at = Some(very_old);
        skills::save_usage(&store).unwrap();

        let r = run(CuratorConfig::default(), true).unwrap();
        assert!(r.archive_transitions.is_empty());
        assert!(r.stale_transitions.is_empty());
        assert_eq!(r.active_kept, 1);
    }
}
