//! `aphrodite design <intent>` — the v0.1 vertical slice.

use aphrodite_core::{
    record_taste, validate_design, Caller, Invocation, SignalKind, Surface, WriteMode,
};
#[allow(unused_imports)]
use aphrodite_core::DesignDocument;
use serde_json::json;
use std::path::{Path, PathBuf};

pub async fn run(
    intent: String,
    no_write: bool,
    repo: Option<PathBuf>,
    is_redesign: bool,
    require_llm: bool,
) -> anyhow::Result<serde_json::Value> {
    let target_repo = match repo {
        Some(p) => p,
        None => std::env::current_dir()?,
    };
    let target_repo = target_repo.canonicalize().unwrap_or(target_repo);

    let write_mode = if no_write { WriteMode::ArtifactOnly } else { WriteMode::Commit };
    let invocation = Invocation {
        id: uuid::Uuid::new_v4().to_string(),
        caller: Caller::Human,
        surface: Surface::Cli,
        intent: intent.clone(),
        target_repo: target_repo.clone(),
        write_mode,
    };

    // `--require-llm` short-circuits *before* generation if we'd be falling
    // back to offline. Agents that must have a real provider call set this.
    if require_llm && aphrodite_generator::resolve_default_provider().is_none() {
        anyhow::bail!(
            "--require-llm: no provider credential is reachable. Run `aphrodite doctor` for details."
        );
    }

    // Generator resolves provider internally (z.ai → Anthropic → OpenRouter → offline).
    let output = aphrodite_generator::generate(&invocation).await?;

    // Validate.
    let report = validate_design(&output.design_doc, &output.variants);

    // Choose write destination.
    let (design_path, hero_path, committed, dest_label) = if no_write {
        let out = target_repo.join(".aphrodite").join("out");
        std::fs::create_dir_all(&out)?;
        let dp = out.join("DESIGN.md");
        let hp = out.join("hero.html");
        std::fs::write(&dp, &output.design_md)?;
        std::fs::write(&hp, &output.hero_html)?;
        (dp, hp, false, "artifact-only".to_string())
    } else {
        let dp = target_repo.join("DESIGN.md");
        let hp = target_repo.join("hero.html");
        std::fs::write(&dp, &output.design_md)?;
        std::fs::write(&hp, &output.hero_html)?;
        let committed = try_git_commit(&target_repo, &[&dp, &hp], &intent).is_ok();
        (dp, hp, committed, "repo-root".to_string())
    };

    // Record taste event.
    let signal = if is_redesign { SignalKind::Regenerate } else { SignalKind::Accept };
    let _ = record_taste(
        &invocation,
        signal,
        json!({
            "intent": intent,
            "design_path": design_path.to_string_lossy(),
            "provider": output.provider_used,
        }),
    );

    Ok(json!({
        "kind": "design",
        "invocation_id": invocation.id,
        "output": {
            "provider_used": output.provider_used,
            "model_used": output.model_used,
            "files": [
                design_path.to_string_lossy(),
                hero_path.to_string_lossy(),
            ],
            "design_path": design_path.to_string_lossy(),
            "hero_path": hero_path.to_string_lossy(),
            "committed": committed,
            "destination": dest_label,
            "variants": output.variants.iter().map(|v| v.kind.label()).collect::<Vec<_>>(),
            "validation": {
                "ok": report.is_ok(),
                "violations": report.violations,
            },
            "warnings": output.warnings,
        },
    }))
}

fn try_git_commit(repo: &Path, files: &[&Path], intent: &str) -> anyhow::Result<()> {
    // Only attempt commit if `repo/.git` exists.
    if !repo.join(".git").exists() {
        anyhow::bail!("not a git repo");
    }
    use std::process::{Command, Stdio};
    for f in files {
        let _ = Command::new("git")
            .arg("-C")
            .arg(repo)
            .arg("add")
            .arg(f)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    let msg = format!("Aphrodite: design for \"{}\"", truncate(intent, 60));
    let status = Command::new("git")
        .arg("-C")
        .arg(repo)
        .arg("commit")
        .arg("-m")
        .arg(&msg)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if !status.success() {
        anyhow::bail!("git commit failed");
    }
    Ok(())
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(max - 1).collect();
        out.push('…');
        out
    }
}
