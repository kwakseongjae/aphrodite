//! Skill substrate — on-disk markdown workflows + per-skill usage tracker.
//!
//! See ADR 0004 (`docs/adr/0004-autonomous-creation-harness.md`).
//!
//! Layout:
//!   ~/.aphrodite/skills/<slug>/SKILL.md     — YAML frontmatter + procedural body
//!   ~/.aphrodite/skills/usage.json          — single JSON: per-slug usage record
//!   ~/.aphrodite/skills/_archive/<slug>/    — archived skills (curator destination)
//!
//! Distinction from `TastePreferences`:
//!   * TastePreferences = declarative facts that decay
//!     ("user prefers warm hues, +0.6 signal over 17 runs")
//!   * Skill                  = procedural workflow that patches
//!     ("for Korean editorial portfolios, do research → reference → 3-turn refine
//!      with Source Serif 4 as default display family")
//!
//! Mirrors Hermes' `tools/skill_usage.py` API at the call-site level.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const SKILL_FILENAME: &str = "SKILL.md";
const USAGE_FILENAME: &str = "usage.json";
const ARCHIVE_DIRNAME: &str = "_archive";

// ---------------------------------------------------------------------------
// Paths
// ---------------------------------------------------------------------------

pub fn skills_root() -> PathBuf {
    let home = std::env::var_os("HOME").unwrap_or_default();
    let mut p = PathBuf::from(home);
    p.push(".aphrodite");
    p.push("skills");
    let _ = fs::create_dir_all(&p);
    p
}

pub fn archive_root() -> PathBuf {
    let mut p = skills_root();
    p.push(ARCHIVE_DIRNAME);
    let _ = fs::create_dir_all(&p);
    p
}

pub fn usage_path() -> PathBuf {
    let mut p = skills_root();
    p.push(USAGE_FILENAME);
    p
}

pub fn skill_dir(slug: &str) -> PathBuf {
    let mut p = skills_root();
    p.push(slug);
    p
}

pub fn skill_md_path(slug: &str) -> PathBuf {
    let mut p = skill_dir(slug);
    p.push(SKILL_FILENAME);
    p
}

// ---------------------------------------------------------------------------
// SKILL.md model
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillFrontmatter {
    pub name: String,
    pub description: String,
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub related_skills: Vec<String>,
    /// True ⇒ skill was created by the orchestrator after a successful trajectory.
    /// False ⇒ user-authored or installed. Curator only touches `agent_created: true`.
    #[serde(default)]
    pub agent_created: bool,
    /// True ⇒ this skill is loaded for EVERY create invocation regardless of
    /// intent-tag overlap. Reserve for cross-cutting standards (asset sourcing,
    /// accessibility, AI-slop avoidance). Default false.
    #[serde(default)]
    pub default: bool,
}

fn default_version() -> String {
    "1.0.0".to_string()
}

#[derive(Debug, Clone)]
pub struct Skill {
    pub slug: String,
    pub frontmatter: SkillFrontmatter,
    pub body: String,
    pub path: PathBuf,
}

impl Skill {
    /// Render full SKILL.md as bytes-on-disk.
    pub fn render(&self) -> Result<String, SkillError> {
        let fm = serde_yaml::to_string(&self.frontmatter)
            .map_err(|e| SkillError::Format(e.to_string()))?;
        Ok(format!("---\n{}---\n\n{}", fm, self.body.trim_end()))
    }

    /// Single-line summary used to inject the skill into an orchestrator system prompt.
    pub fn as_scaffold_hint(&self) -> String {
        format!(
            "- [{}] {} (tags: {})",
            self.slug,
            self.frontmatter.description,
            if self.frontmatter.tags.is_empty() {
                "—".to_string()
            } else {
                self.frontmatter.tags.join(", ")
            }
        )
    }
}

#[derive(Debug)]
pub enum SkillError {
    NotFound(String),
    Io(io::Error),
    Format(String),
    NoFrontmatter,
}

impl std::fmt::Display for SkillError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkillError::NotFound(s) => write!(f, "skill not found: {s}"),
            SkillError::Io(e) => write!(f, "skill io error: {e}"),
            SkillError::Format(e) => write!(f, "skill format error: {e}"),
            SkillError::NoFrontmatter => write!(f, "skill missing YAML frontmatter"),
        }
    }
}

impl std::error::Error for SkillError {}

impl From<io::Error> for SkillError {
    fn from(e: io::Error) -> Self {
        SkillError::Io(e)
    }
}

// ---------------------------------------------------------------------------
// Parse / load / save
// ---------------------------------------------------------------------------

pub fn parse_skill(slug: &str, text: &str, path: PathBuf) -> Result<Skill, SkillError> {
    let trimmed = text.trim_start();
    if !trimmed.starts_with("---") {
        return Err(SkillError::NoFrontmatter);
    }
    let after = &trimmed[3..];
    let end = after
        .find("\n---")
        .ok_or(SkillError::NoFrontmatter)?;
    let fm_text = &after[..end];
    let body_start = end + 4;
    let body = after[body_start..].trim_start_matches('\n').to_string();
    let frontmatter: SkillFrontmatter =
        serde_yaml::from_str(fm_text).map_err(|e| SkillError::Format(e.to_string()))?;
    Ok(Skill {
        slug: slug.to_string(),
        frontmatter,
        body,
        path,
    })
}

pub fn load(slug: &str) -> Result<Skill, SkillError> {
    let path = skill_md_path(slug);
    if !path.exists() {
        return Err(SkillError::NotFound(slug.to_string()));
    }
    let text = fs::read_to_string(&path)?;
    parse_skill(slug, &text, path)
}

pub fn save(skill: &Skill) -> Result<(), SkillError> {
    let dir = skill_dir(&skill.slug);
    fs::create_dir_all(&dir)?;
    let rendered = skill.render()?;
    let target = skill_md_path(&skill.slug);
    fs::write(&target, rendered)?;
    Ok(())
}

/// List all live (non-archived) skills, sorted by slug.
pub fn list() -> Vec<String> {
    let root = skills_root();
    let mut out = Vec::new();
    if let Ok(entries) = fs::read_dir(&root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let name = match path.file_name().and_then(|s| s.to_str()) {
                Some(n) => n.to_string(),
                None => continue,
            };
            if name == ARCHIVE_DIRNAME {
                continue;
            }
            if path.join(SKILL_FILENAME).exists() {
                out.push(name);
            }
        }
    }
    out.sort();
    out
}

/// Move a skill directory to `_archive/`. Idempotent w.r.t. usage record (state ← Archived).
pub fn archive(slug: &str) -> Result<(), SkillError> {
    let src = skill_dir(slug);
    if !src.exists() {
        return Err(SkillError::NotFound(slug.to_string()));
    }
    let mut dst = archive_root();
    dst.push(slug);
    if dst.exists() {
        let _ = fs::remove_dir_all(&dst);
    }
    fs::rename(&src, &dst)?;
    let _ = mutate_usage(slug, |r| r.state = SkillState::Archived);
    Ok(())
}

/// Restore an archived skill.
pub fn restore(slug: &str) -> Result<(), SkillError> {
    let mut src = archive_root();
    src.push(slug);
    if !src.exists() {
        return Err(SkillError::NotFound(slug.to_string()));
    }
    let dst = skill_dir(slug);
    fs::rename(&src, &dst)?;
    let _ = mutate_usage(slug, |r| r.state = SkillState::Active);
    Ok(())
}

// ---------------------------------------------------------------------------
// Usage tracking
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SkillState {
    Active,
    Stale,
    Archived,
}

impl Default for SkillState {
    fn default() -> Self {
        SkillState::Active
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UsageRecord {
    #[serde(default)]
    pub view_count: u32,
    #[serde(default)]
    pub use_count: u32,
    #[serde(default)]
    pub patch_count: u32,
    #[serde(default)]
    pub last_view_at: Option<String>,
    #[serde(default)]
    pub last_use_at: Option<String>,
    #[serde(default)]
    pub last_patch_at: Option<String>,
    #[serde(default)]
    pub state: SkillState,
    #[serde(default)]
    pub pinned: bool,
    /// Free-form skill metadata — kept here, not in frontmatter, because it
    /// changes more often than the skill itself.
    #[serde(default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

impl UsageRecord {
    pub fn latest_activity_at(&self) -> Option<&str> {
        // Prefer use > patch > view as the "alive" signal.
        self.last_use_at
            .as_deref()
            .or(self.last_patch_at.as_deref())
            .or(self.last_view_at.as_deref())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UsageStore {
    #[serde(flatten)]
    pub records: BTreeMap<String, UsageRecord>,
}

pub fn load_usage() -> UsageStore {
    let path = usage_path();
    match fs::read_to_string(&path) {
        Ok(s) => serde_json::from_str(&s).unwrap_or_default(),
        Err(_) => UsageStore::default(),
    }
}

pub fn save_usage(store: &UsageStore) -> Result<(), SkillError> {
    let path = usage_path();
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    let text =
        serde_json::to_string_pretty(store).map_err(|e| SkillError::Format(e.to_string()))?;
    // atomic-ish write via temp + rename
    let tmp = parent.join(".usage.json.tmp");
    fs::write(&tmp, text)?;
    fs::rename(&tmp, &path)?;
    Ok(())
}

pub fn get_record(slug: &str) -> UsageRecord {
    load_usage()
        .records
        .get(slug)
        .cloned()
        .unwrap_or_default()
}

fn mutate_usage<F: FnOnce(&mut UsageRecord)>(slug: &str, f: F) -> Result<(), SkillError> {
    let mut store = load_usage();
    let entry = store.records.entry(slug.to_string()).or_default();
    f(entry);
    save_usage(&store)
}

pub fn bump_view(slug: &str) -> Result<(), SkillError> {
    let now = crate::taste::now_rfc3339();
    mutate_usage(slug, |r| {
        r.view_count = r.view_count.saturating_add(1);
        r.last_view_at = Some(now);
    })
}

pub fn bump_use(slug: &str) -> Result<(), SkillError> {
    let now = crate::taste::now_rfc3339();
    mutate_usage(slug, |r| {
        r.use_count = r.use_count.saturating_add(1);
        r.last_use_at = Some(now);
    })
}

pub fn bump_patch(slug: &str) -> Result<(), SkillError> {
    let now = crate::taste::now_rfc3339();
    mutate_usage(slug, |r| {
        r.patch_count = r.patch_count.saturating_add(1);
        r.last_patch_at = Some(now);
    })
}

pub fn set_pinned(slug: &str, pinned: bool) -> Result<(), SkillError> {
    mutate_usage(slug, |r| r.pinned = pinned)
}

pub fn set_state(slug: &str, state: SkillState) -> Result<(), SkillError> {
    mutate_usage(slug, |r| r.state = state)
}

// ---------------------------------------------------------------------------
// Bundled seed skills (embedded at compile time)
// ---------------------------------------------------------------------------

/// Skills shipped with the binary. On first orchestrator run we materialise
/// these into `~/.aphrodite/skills/` if the slug is absent. Idempotent.
const BUNDLED_SKILLS: &[(&str, &str)] = &[
    (
        "asset-standards",
        include_str!("../seed-skills/asset-standards/SKILL.md"),
    ),
    (
        "editorial-portfolio",
        include_str!("../seed-skills/editorial-portfolio/SKILL.md"),
    ),
];

/// Materialise bundled skills onto disk if they are not already present.
/// Returns the list of slugs that were newly written this call.
pub fn seed_bundled_skills() -> Vec<String> {
    let mut newly_written = Vec::new();
    for (slug, contents) in BUNDLED_SKILLS {
        let target = skill_md_path(slug);
        if target.exists() {
            continue;
        }
        if let Some(parent) = target.parent() {
            if fs::create_dir_all(parent).is_err() {
                continue;
            }
        }
        if fs::write(&target, contents).is_ok() {
            newly_written.push((*slug).to_string());
        }
    }
    newly_written
}

// ---------------------------------------------------------------------------
// Intent → tag extraction (cheap heuristic, no LLM)
// ---------------------------------------------------------------------------

/// Extract tag tokens from a free-form intent string. Conservative:
/// returns a small set of canonical tags the bundled / user-installed
/// skills are likely to declare. Empty result is fine — `rank_for_intent`
/// then still surfaces pinned skills.
pub fn extract_intent_tags(intent: &str) -> Vec<String> {
    let lower = intent.to_ascii_lowercase();
    let mut tags: Vec<String> = Vec::new();
    let candidates: &[(&[&str], &str)] = &[
        // surface kind
        (&["portfolio", "work grid", "selected work", "case studies"], "portfolio"),
        (&["pricing", "tier", "saas", "subscription"], "pricing"),
        (&["dashboard", "analytics", "metrics"], "dashboard"),
        (&["mobile app", "ios app", "android app", "phone app"], "mobile_app"),
        (&["landing", "marketing site", "hero page"], "landing"),
        (&["editorial", "magazine", "long-form", "journal"], "editorial"),
        // domain
        (&["furniture", "woodwork", "walnut", "oak", "joinery", "cabinet maker"], "furniture"),
        (&["architecture", "architect", "studio", "atelier"], "architecture"),
        (&["ceramic", "pottery", "stoneware"], "ceramics"),
        (&["photograph", "photog"], "photography"),
        // maker register
        (&["independent", "solo", "craftsperson", "maker", "artisan"], "solo-maker"),
        (&["studio practice", "small workshop"], "solo-maker"),
        // material/photo
        (&["photo-dominant", "image-led", "photography-led", "image dominant"], "photo-dominant"),
    ];
    for (keys, tag) in candidates {
        if keys.iter().any(|k| lower.contains(k)) {
            let t = (*tag).to_string();
            if !tags.contains(&t) {
                tags.push(t);
            }
        }
    }
    tags
}

/// Build a single text block ready to prepend to a system prompt — top-K
/// scaffolded skills joined as "## Skill: <name>\n<body>" sections. Truncates
/// per-skill body to keep total length bounded.
pub fn build_scaffold_block(skills: &[Skill], max_body_chars: usize) -> String {
    if skills.is_empty() {
        return String::new();
    }
    let mut out = String::from(
        "## Applicable skills (workflow scaffolds from prior runs — treat as concrete checklists, not suggestions)\n\n",
    );
    for s in skills {
        out.push_str(&format!(
            "### Skill: {} — {}\n\n",
            s.frontmatter.name, s.frontmatter.description
        ));
        if s.body.len() <= max_body_chars {
            out.push_str(&s.body);
        } else {
            let truncated: String = s.body.chars().take(max_body_chars).collect();
            out.push_str(&truncated);
            out.push_str("\n…[skill body truncated]\n");
        }
        out.push_str("\n\n---\n\n");
    }
    out
}

// ---------------------------------------------------------------------------
// Orchestrator-facing scaffolding hooks
// ---------------------------------------------------------------------------

/// Rank live skills by tag overlap with the incoming intent's tag set.
/// Skills marked `default: true` in frontmatter are ALWAYS included (they
/// encode cross-cutting standards like asset sourcing). Pinned skills float
/// to the top regardless of overlap. Otherwise tag overlap drives score.
pub fn rank_for_intent(intent_tags: &[&str], top_k: usize) -> Vec<Skill> {
    let usage = load_usage();
    let mut scored: Vec<(i32, Skill)> = Vec::new();
    for slug in list() {
        let skill = match load(&slug) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let rec = usage.records.get(&slug).cloned().unwrap_or_default();
        if rec.state == SkillState::Archived {
            continue;
        }
        let overlap = skill
            .frontmatter
            .tags
            .iter()
            .filter(|t| intent_tags.iter().any(|it| it.eq_ignore_ascii_case(t)))
            .count() as i32;
        let pin_bonus = if rec.pinned { 100 } else { 0 };
        let default_bonus = if skill.frontmatter.default { 80 } else { 0 };
        let use_bonus = (rec.use_count.min(20) as i32) / 4;
        let score = overlap * 10 + pin_bonus + default_bonus + use_bonus;
        if score > 0 || rec.pinned || skill.frontmatter.default {
            scored.push((score, skill));
        }
    }
    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored.into_iter().take(top_k).map(|(_, s)| s).collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Global mutex to serialize tests in this module — they all mutate the
    /// shared process env var HOME, so running them concurrently would race.
    static TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// Hermetic scratch dir for one test — holds the lock for its lifetime
    /// and points HOME at a tempdir.
    struct Scratch {
        _td: tempfile::TempDir,
        _guard: std::sync::MutexGuard<'static, ()>,
    }
    impl Scratch {
        fn new() -> Self {
            let guard = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
            let td = tempfile::tempdir().expect("tempdir");
            // SAFETY: HOME is mutated only while we hold TEST_LOCK; no
            // concurrent reader exists in this test module.
            unsafe { std::env::set_var("HOME", td.path()); }
            Self { _td: td, _guard: guard }
        }
    }

    fn fixture_skill(slug: &str, agent_created: bool) -> Skill {
        Skill {
            slug: slug.to_string(),
            frontmatter: SkillFrontmatter {
                name: slug.to_string(),
                description: format!("test skill {slug}"),
                version: "1.0.0".to_string(),
                tags: vec!["portfolio".into(), "test".into()],
                related_skills: vec![],
                agent_created,
                default: false,
            },
            body: "# Workflow\n\n1. Step one\n2. Step two\n".to_string(),
            path: skill_md_path(slug),
        }
    }

    #[test]
    fn round_trip_parse_render() {
        let _s = Scratch::new();
        let skill = fixture_skill("test-roundtrip", false);
        save(&skill).expect("save");
        let loaded = load("test-roundtrip").expect("load");
        assert_eq!(loaded.frontmatter.name, "test-roundtrip");
        assert!(loaded.body.contains("Step one"));
        assert_eq!(loaded.frontmatter.tags, vec!["portfolio", "test"]);
    }

    #[test]
    fn list_includes_live_skips_archive_dir() {
        let _s = Scratch::new();
        save(&fixture_skill("alpha", false)).unwrap();
        save(&fixture_skill("beta", false)).unwrap();
        let _ = archive_root(); // ensures _archive exists
        let listed = list();
        assert!(listed.contains(&"alpha".to_string()));
        assert!(listed.contains(&"beta".to_string()));
        assert!(!listed.contains(&ARCHIVE_DIRNAME.to_string()));
    }

    #[test]
    fn usage_counters_increment_and_persist() {
        let _s = Scratch::new();
        save(&fixture_skill("counters", true)).unwrap();
        bump_view("counters").unwrap();
        bump_use("counters").unwrap();
        bump_use("counters").unwrap();
        bump_patch("counters").unwrap();
        let r = get_record("counters");
        assert_eq!(r.view_count, 1);
        assert_eq!(r.use_count, 2);
        assert_eq!(r.patch_count, 1);
        assert!(r.last_use_at.is_some());
    }

    #[test]
    fn archive_then_restore() {
        let _s = Scratch::new();
        save(&fixture_skill("lifecycle", true)).unwrap();
        archive("lifecycle").unwrap();
        assert!(!list().contains(&"lifecycle".to_string()));
        assert_eq!(get_record("lifecycle").state, SkillState::Archived);

        restore("lifecycle").unwrap();
        assert!(list().contains(&"lifecycle".to_string()));
        assert_eq!(get_record("lifecycle").state, SkillState::Active);
    }

    #[test]
    fn rank_includes_default_skills_even_with_no_overlap() {
        let _s = Scratch::new();
        let mut default_skill = fixture_skill("asset-standards", false);
        default_skill.frontmatter.tags = vec!["completely-unrelated".into()];
        default_skill.frontmatter.default = true;
        save(&default_skill).unwrap();

        let mut domain_skill = fixture_skill("editorial", false);
        domain_skill.frontmatter.tags = vec!["editorial".into()];
        save(&domain_skill).unwrap();

        let ranked = rank_for_intent(&["editorial"], 5);
        let slugs: Vec<&str> = ranked.iter().map(|s| s.slug.as_str()).collect();
        // default skill must appear despite zero tag overlap
        assert!(slugs.contains(&"asset-standards"), "got: {:?}", slugs);
        assert!(slugs.contains(&"editorial"));
    }

    #[test]
    fn rank_by_tag_overlap_and_pin() {
        let _s = Scratch::new();
        let mut a = fixture_skill("editorial", true);
        a.frontmatter.tags = vec!["editorial".into(), "portfolio".into()];
        save(&a).unwrap();

        let mut b = fixture_skill("pricing", true);
        b.frontmatter.tags = vec!["pricing".into(), "saas".into()];
        save(&b).unwrap();

        let mut c = fixture_skill("pinned-misc", true);
        c.frontmatter.tags = vec!["unrelated".into()];
        save(&c).unwrap();
        set_pinned("pinned-misc", true).unwrap();

        let ranked = rank_for_intent(&["editorial", "portfolio"], 5);
        let slugs: Vec<&str> = ranked.iter().map(|s| s.slug.as_str()).collect();
        // pinned floats to top, editorial overlaps, pricing zero overlap excluded
        assert!(slugs.contains(&"editorial"));
        assert!(slugs.contains(&"pinned-misc"));
        assert!(!slugs.contains(&"pricing"));
    }

    #[test]
    fn intent_to_tags_extracts_canonical_tokens() {
        let tags = extract_intent_tags(
            "a portfolio site for an independent furniture maker working in solid walnut, based in Seoul",
        );
        assert!(tags.contains(&"portfolio".to_string()));
        assert!(tags.contains(&"furniture".to_string()));
        assert!(tags.contains(&"solo-maker".to_string()));
        // Should NOT pick up unrelated buckets:
        assert!(!tags.contains(&"pricing".to_string()));
        assert!(!tags.contains(&"dashboard".to_string()));
    }

    #[test]
    fn intent_to_tags_empty_for_generic_intent() {
        let tags = extract_intent_tags("a simple page about the weather");
        assert!(tags.is_empty(), "got tags: {:?}", tags);
    }

    #[test]
    fn seed_bundled_skills_is_idempotent() {
        let _s = Scratch::new();
        let first = seed_bundled_skills();
        assert!(first.contains(&"editorial-portfolio".to_string()));
        let again = seed_bundled_skills();
        assert!(again.is_empty(), "should not re-write existing skills, got: {:?}", again);
        // And it should be loadable.
        let s = load("editorial-portfolio").expect("load seeded skill");
        assert_eq!(s.frontmatter.name, "editorial-portfolio");
        assert!(s.frontmatter.tags.contains(&"portfolio".to_string()));
    }

    #[test]
    fn scaffold_block_renders_when_skills_present() {
        let _s = Scratch::new();
        save(&fixture_skill("alpha", true)).unwrap();
        let alpha = load("alpha").unwrap();
        let block = build_scaffold_block(&[alpha], 1_000);
        assert!(block.contains("Applicable skills"));
        assert!(block.contains("alpha"));
        assert!(block.contains("Step one"));
    }

    #[test]
    fn scaffold_block_empty_for_no_skills() {
        let block = build_scaffold_block(&[], 1_000);
        assert!(block.is_empty());
    }

    #[test]
    fn missing_frontmatter_rejected() {
        let _s = Scratch::new();
        let bad = "no frontmatter here at all\nbody only\n";
        let err = parse_skill("bad", bad, PathBuf::from("/tmp/none")).unwrap_err();
        assert!(matches!(err, SkillError::NoFrontmatter));
    }
}
