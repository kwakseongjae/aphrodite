//! `aphrodite` CLI. Thin pretty layer over a JSON contract — `--json` short-
//! circuits the pretty renderer for agent callers.

use clap::{Parser, Subcommand};
use serde_json::json;
use std::path::PathBuf;

mod assets_cmd;
mod banner;
mod create_cmd;
mod curator_cmd;
mod design_cmd;
mod feedback_cmd;
mod gallery_cmd;
mod init_cmd;
mod log_cmd;
mod preview_cmd;
mod refine_cmd;
mod setup_cmd;
mod wiki_cmd;

#[derive(Parser)]
#[command(name = "aphrodite", version, about = "Aphrodite — UI generation harness")]
struct Cli {
    /// Emit raw JSON instead of the pretty terminal output.
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// First-run guided wizard — banner, provider, plan, model, key.
    Init,

    /// Print the ASCII banner + current config summary.
    Version,

    /// Non-interactive setup (env-var driven; kept for CI).
    Setup,

    /// Generate a DESIGN.md + hero HTML from an intent string.
    Design {
        intent: String,

        /// Skip the git commit; emit artifacts to `./.aphrodite/out/` instead.
        #[arg(long)]
        no_write: bool,

        /// Target repo (defaults to current working directory).
        #[arg(long)]
        repo: Option<PathBuf>,

        /// Refuse to fall back to offline if a real provider was intended.
        /// Useful for CI and agent calls where silent downgrade is unacceptable.
        #[arg(long)]
        require_llm: bool,
    },

    /// Regenerate with implicit "didn't like that one" signal recorded.
    Redesign {
        intent: String,

        #[arg(long)]
        no_write: bool,

        #[arg(long)]
        repo: Option<PathBuf>,

        #[arg(long)]
        require_llm: bool,
    },

    /// Show configured providers (without revealing key material).
    Auth {
        #[command(subcommand)]
        sub: AuthSub,
    },

    /// Run all health checks (config + keychain + env + reachability).
    Doctor,

    /// Print the v0.1 capability matrix — what Aphrodite can and cannot do.
    Capabilities,

    /// Rebuild the design-system handoff (tokens.css, tokens.json,
    /// components.html) from the DESIGN.md in the target directory.
    /// Useful when you've manually edited DESIGN.md and want to refresh
    /// the engineering artifacts without re-running the full create flow.
    Components {
        #[arg(long)]
        repo: Option<PathBuf>,
    },

    /// Generate a publishable React component package (`react/` directory
    /// with package.json + tsconfig + 30 typed primitive .tsx files +
    /// 30 CSF3 Storybook stories + tokens.ts) from the DESIGN.md in the
    /// target directory. The package can be `npm publish`-ed or vendored
    /// into a monorepo.
    React {
        #[arg(long)]
        repo: Option<PathBuf>,
    },

    /// Rename the emitted `react/` package to a new npm scope/name and
    /// (optionally) publish it. Used by adopters who want to ship their
    /// generated brand under their own `@org/name` instead of the
    /// canonical `@aphrodite-design/<brand>`.
    ///
    /// Examples:
    ///   aphrodite publish --as @toss/design-kit --dry-run
    ///   aphrodite publish --as @karrot/ui
    Publish {
        /// Target npm name, e.g. `@toss/design-kit` or `toss-design-kit`.
        #[arg(long)]
        r#as: String,
        /// Don't actually call `npm publish` — just print the changes.
        #[arg(long, default_value_t = false)]
        dry_run: bool,
        /// Override the source directory (defaults to ./react/).
        #[arg(long)]
        react_dir: Option<PathBuf>,
        /// Pass `--tag <tag>` through to npm publish (e.g. `beta`).
        #[arg(long)]
        npm_tag: Option<String>,
    },

    /// Run a cross-brand alpha sweep: takes a JSON file of intents,
    /// runs each through `aphrodite create`, aggregates Quality Score +
    /// audit warnings + telemetry, writes `eval-report.json` and prints
    /// a summary table. Useful for CI gating and pre-release regression
    /// testing across many brand archetypes.
    ///
    /// Spec file shape: array of `{ name, intent, persona?, pages? }`.
    Eval {
        /// Path to the spec JSON.
        spec: PathBuf,
        /// Root directory for per-run output subdirectories.
        #[arg(long, default_value = "./eval-out")]
        out: PathBuf,
        #[arg(long, default_value_t = 2)]
        max_turns: u32,
        #[arg(long, default_value_t = 0.78)]
        threshold: f32,
    },

    /// Rebuild the single-file docs site (`docs/index.html`) — Material-
    /// UI style component documentation with TOC sidebar, install
    /// snippet, color tokens, type scale, and a section per component.
    Docs {
        #[arg(long)]
        repo: Option<PathBuf>,
    },

    /// Figma Variables sync. `export` writes `tokens.figma.json` (Tokens
    /// Studio format) from DESIGN.md. `pull <file_key>` fetches the
    /// linked Figma file's local variables (requires APHRODITE_FIGMA_TOKEN
    /// env) and prints a diff vs DESIGN.md.
    Figma {
        #[command(subcommand)]
        sub: FigmaSub,
    },

    /// Visual regression: compare the screenshots in `<baseline>` against
    /// the screenshots in `<current>` (defaults to cwd). Emits a JSON
    /// summary + per-file verdict (identical / minor / changed) so CI
    /// can gate on the changed count. Falls back to file-size delta when
    /// ImageMagick's `compare` binary isn't on PATH.
    Diff {
        baseline: PathBuf,
        current: Option<PathBuf>,
        /// Size-delta percentage above which a pair is flagged as
        /// Changed (when ImageMagick is unavailable). Default 2.5.
        #[arg(long, default_value_t = 2.5)]
        threshold: f32,
        /// Also write ImageMagick heat-map PNGs alongside current files.
        #[arg(long, default_value_t = false)]
        write_heatmaps: bool,
    },

    /// Build a single-file gallery.html that previews every run subdirectory
    /// under the given path. Each run becomes a card with intent text,
    /// metrics, palette swatches, and an iframe of the composition (or hero
    /// fallback). Drop the output anywhere; it's self-contained.
    Gallery {
        /// Directory containing run-XX/ subdirectories (each with intent.txt,
        /// result.json, DESIGN.md, composition.html / hero.html).
        dir: PathBuf,
    },

    /// Record strong positive feedback on the latest design in the current
    /// repo. Boosts the observable patterns (hue family, fonts, density,
    /// type style, accent intensity) in your accumulated TastePreferences
    /// so future designs lean toward what worked.
    Love {
        #[arg(long)]
        repo: Option<PathBuf>,
    },

    /// Record strong negative feedback. Decays the same patterns so future
    /// designs avoid this direction.
    Hate {
        #[arg(long)]
        repo: Option<PathBuf>,
    },

    /// Show accumulated taste preferences (global + project merged).
    Prefs {
        #[arg(long)]
        repo: Option<PathBuf>,
    },

    /// Refine the current DESIGN.md with a delta instruction (multi-turn).
    /// Reads DESIGN.md from cwd, sends it as context with your change request,
    /// writes the revised DESIGN.md + re-composes the surface. Records a
    /// Regenerate taste signal so successive refinements feed the loop.
    Refine {
        /// One-sentence change request, e.g. "make the palette cooler" or
        /// "increase the spacing tokens significantly".
        change: String,
        #[arg(long)]
        no_write: bool,
        #[arg(long)]
        repo: Option<PathBuf>,
    },

    /// Autonomous creation: design → self-critic refine loop → final.
    /// ADR 0004 Phase 4 prototype. Caller hands over intent; harness
    /// internalises the multi-turn refinement (no external delta authoring).
    Create {
        intent: String,
        /// Cap on internal critic→refine iterations. Each iteration costs
        /// ~3 LLM calls (critic + refine + surface re-compose).
        #[arg(long, default_value_t = 2)]
        max_turns: u32,
        /// Stop when critic's satisfaction estimate reaches this value.
        /// Calibrated against rolling Pass 30-44 data: critic settles into
        /// 0.70-0.80 with constructive minor critique; 0.85 forced wasted
        /// refines that didn't change the verdict on the next turn.
        #[arg(long, default_value_t = 0.78)]
        satisfaction_threshold: f32,
        /// Invoke a design-authority persona (e.g. `dieter-rams`, `tadao-ando`,
        /// `rei-kawakubo`, `ettore-sottsass`, `kenya-hara`, `massimo-vignelli`,
        /// `galileo-galilei`). Persona principles outrank generic skill scaffolds.
        #[arg(long)]
        persona: Option<String>,
        /// Preview phase-1 retrieval + estimated LLM call count + wall-clock
        /// estimate without making any provider calls. Useful for sanity-checking
        /// what the harness will do before incurring cost.
        #[arg(long)]
        estimate: bool,
        #[arg(long)]
        no_write: bool,
        #[arg(long)]
        repo: Option<PathBuf>,
        /// Multi-page output. Comma-separated page slugs (e.g.
        /// "home,pricing,about"). The first slug runs the full create
        /// flow; subsequent slugs reuse the resulting DESIGN.md and only
        /// re-run the composition phase with a page-specific brief. Each
        /// page writes to `<slug>.html` and the harness emits a
        /// `sitemap.xml` listing them. Default is a single page named
        /// `composition`.
        #[arg(long, value_delimiter = ',')]
        pages: Vec<String>,
    },

    /// List installed personas (bundled + user-installed). Use `--persona <slug>`
    /// on `aphrodite create` to invoke one.
    Personas,

    /// Design-reference wiki (Karpathy LLM-Wiki pattern). Compounding KB of
    /// well-designed references that `aphrodite create` queries by tag overlap.
    Wiki {
        #[command(subcommand)]
        sub: WikiSub,
    },

    /// Inspect / clean per-project assets (`<project>/.aphrodite/assets/`).
    /// refs/ holds wiki reference images materialised at create time;
    /// uploads/ is reserved for user-supplied images and is never auto-cleaned.
    Assets {
        #[command(subcommand)]
        sub: AssetsSub,
    },

    /// Periodic review of agent-created skills — transition stale (30 days
    /// default) and archive (90 days default). Hermes-pattern. Only touches
    /// `agent_created: true` skills; user-installed or bundled skills are
    /// never auto-mutated.
    Curator {
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        stale_after_days: Option<u32>,
        #[arg(long)]
        archive_after_days: Option<u32>,
    },

    /// Show recent Aphrodite create/refine/design commits in the target repo.
    /// Each line is colour-coded by kind and dated by relative time.
    Log {
        #[arg(long, default_value_t = 10)]
        n: usize,
        #[arg(long)]
        repo: Option<PathBuf>,
    },

    /// Roll back the last N Aphrodite auto-commits. Dry-run by default —
    /// pass --yes to actually `git reset --hard`. Refuses to drop any
    /// non-Aphrodite commits in the range so user work is never lost.
    /// `git reflog` recovers the prior HEAD if needed.
    Undo {
        #[arg(long, default_value_t = 1)]
        n: usize,
        #[arg(long)]
        yes: bool,
        #[arg(long)]
        repo: Option<PathBuf>,
    },

    /// Open the current run's composition.html (or hero.html fallback) in
    /// the default browser. Closes the "no visual editor" pain — single
    /// keystroke to see your output.
    Preview {
        #[arg(long)]
        repo: Option<PathBuf>,
        /// Override which file to open (defaults to composition.html →
        /// hero.html → .aphrodite/out/composition.html).
        #[arg(long)]
        file: Option<String>,
    },
}

#[derive(Subcommand)]
enum AssetsSub {
    /// List asset files by category with sizes.
    List {
        #[arg(long)]
        repo: Option<PathBuf>,
    },
    /// Remove generated assets (refs/, icons/). Spares uploads/.
    Clean {
        #[arg(long)]
        repo: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum FigmaSub {
    /// Write `tokens.figma.json` (Tokens Studio plugin format) in the
    /// target directory. Designers import via Tokens Studio.
    Export {
        #[arg(long)]
        repo: Option<PathBuf>,
    },
    /// Fetch a Figma file's local variables and diff vs DESIGN.md.
    /// Requires APHRODITE_FIGMA_TOKEN (or FIGMA_TOKEN) env var.
    /// `file_key` is the segment after `figma.com/file/` in the URL.
    Pull {
        file_key: String,
        #[arg(long)]
        repo: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum WikiSub {
    /// List installed wiki entries (bundled + user-added).
    List,
    /// Print one entry's frontmatter + body.
    Show { slug: String },
    /// Add a new wiki entry. Body comes from --body, --body-from-file, or
    /// piped stdin. If none, a stub is written for manual editing.
    Add {
        /// URL of the reference site / system / artefact.
        url: String,
        /// Override the auto-derived slug.
        #[arg(long)]
        slug: Option<String>,
        /// Human-readable title (defaults to slug-as-words; --auto-fetch
        /// overrides with the page's <title> when no manual title given).
        #[arg(long)]
        title: Option<String>,
        /// Comma-separated tags for intent matching.
        #[arg(long, value_delimiter = ',', num_args = 0..)]
        tags: Vec<String>,
        /// One-line stylistic distillation. --auto-fetch falls back to
        /// the page's meta description / og:description when omitted.
        #[arg(long)]
        signature: Option<String>,
        /// Inline body text.
        #[arg(long)]
        body: Option<String>,
        /// Read body from this file path.
        #[arg(long, value_name = "PATH")]
        body_from_file: Option<PathBuf>,
        /// Replace an existing entry with the same slug.
        #[arg(long)]
        overwrite: bool,
        /// HTTP GET the URL and extract title / meta description / palette
        /// hints. Failures degrade gracefully — a stub is still written.
        #[arg(long)]
        auto_fetch: bool,
    },
}

#[derive(Subcommand)]
enum AuthSub {
    /// List configured providers.
    Status,
    /// Store an API key for a provider in the OS keychain.
    Set {
        provider: String,
        /// Read key from this env var instead of prompting.
        #[arg(long)]
        from_env: Option<String>,
        /// Read key from stdin (e.g. `pbpaste | aphrodite auth set zai --from-stdin`).
        /// Bypasses the hidden prompt entirely — most reliable across terminals.
        #[arg(long)]
        from_stdin: bool,
        /// Read key from a file path. File is read whole, then deleted unless
        /// `--keep-file` is also set.
        #[arg(long, value_name = "PATH")]
        from_file: Option<PathBuf>,
        /// With `--from-file`: don't delete the source file after reading.
        #[arg(long)]
        keep_file: bool,
    },
    /// Remove a provider's stored credential.
    Remove { provider: String },
    /// Verify that a provider's stored key is readable (round-trip test).
    Verify { provider: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .try_init();

    let payload = match cli.command {
        Command::Init => init_cmd::run().await?,
        Command::Version => version_summary(cli.json)?,
        Command::Setup => setup_cmd::run().await?,
        Command::Design { intent, no_write, repo, require_llm } => {
            design_cmd::run(intent, no_write, repo, false, require_llm).await?
        }
        Command::Redesign { intent, no_write, repo, require_llm } => {
            design_cmd::run(intent, no_write, repo, true, require_llm).await?
        }
        Command::Auth { sub } => match sub {
            AuthSub::Status => auth_status(),
            AuthSub::Set { provider, from_env, from_stdin, from_file, keep_file } => {
                auth_set(&provider, from_env.as_deref(), from_stdin, from_file.as_deref(), keep_file)?
            }
            AuthSub::Remove { provider } => auth_remove(&provider),
            AuthSub::Verify { provider } => auth_verify(&provider),
        },
        Command::Doctor => doctor(),
        Command::Capabilities => capabilities(),
        Command::Components { repo } => {
            let target = repo.unwrap_or_else(|| std::env::current_dir().expect("cwd"));
            let design_path = target.join("DESIGN.md");
            let design_md = std::fs::read_to_string(&design_path)
                .map_err(|e| anyhow::anyhow!("cannot read {}: {e}", design_path.display()))?;
            let doc = aphrodite_core::parse_design(&design_md)
                .map_err(|e| anyhow::anyhow!("parse DESIGN.md: {e}"))?;
            let variants = aphrodite_core::resolve_variants(&doc);
            let css = aphrodite_generator::design_system::build_tokens_css(&variants);
            let json = aphrodite_generator::design_system::build_tokens_json(&variants);
            let html = aphrodite_generator::design_system::build_components_html(&variants);
            std::fs::write(target.join("tokens.css"), &css)?;
            std::fs::write(target.join("tokens.json"), &json)?;
            std::fs::write(target.join("components.html"), &html)?;
            println!("wrote tokens.css, tokens.json, components.html in {}", target.display());
            json!({ "kind": "components", "path": target.to_string_lossy() })
        }
        Command::Publish { r#as, dry_run, react_dir, npm_tag } => {
            use std::process::Command as SysCmd;
            let react_root = react_dir.unwrap_or_else(|| std::env::current_dir().expect("cwd").join("react"));
            if !react_root.join("package.json").exists() {
                anyhow::bail!("no react/package.json found at {}. Run `aphrodite react` first.", react_root.display());
            }

            // Parse the target name. Accept @scope/name or bare name.
            let target = r#as.trim();
            let (target_scope, target_short) = if let Some(stripped) = target.strip_prefix('@') {
                let (scope, rest) = stripped.split_once('/').ok_or_else(|| anyhow::anyhow!("scoped name must be @scope/name"))?;
                (format!("@{scope}"), rest.to_string())
            } else {
                (String::new(), target.to_string())
            };
            let full_target = if target_scope.is_empty() { target_short.clone() } else { format!("{target_scope}/{target_short}") };

            // Read the current package.json
            let pkg_path = react_root.join("package.json");
            let pkg_text = std::fs::read_to_string(&pkg_path)?;
            let mut pkg: serde_json::Value = serde_json::from_str(&pkg_text)?;
            let current_name = pkg["name"].as_str().unwrap_or("").to_string();

            // Files to rewrite. Substitute the OLD package name (whatever it is) → full_target.
            let mut changed: Vec<(PathBuf, usize)> = Vec::new();

            // 1. package.json: name field
            pkg["name"] = serde_json::Value::String(full_target.clone());
            if !dry_run {
                std::fs::write(&pkg_path, serde_json::to_string_pretty(&pkg)?)?;
            }
            changed.push((pkg_path.clone(), 1));

            // 2. README + CHANGELOG: substring substitution of the old name.
            let extras = [react_root.join("README.md"), react_root.join("CHANGELOG.md")];
            for f in &extras {
                if !f.exists() { continue; }
                let body = std::fs::read_to_string(f)?;
                let new_body = body.replace(&current_name, &full_target);
                let count = body.matches(&current_name).count();
                if count > 0 {
                    if !dry_run { std::fs::write(f, &new_body)?; }
                    changed.push((f.clone(), count));
                }
            }

            // Print summary
            println!("aphrodite publish --rename");
            println!("  source : {} ({})", react_root.display(), current_name);
            println!("  target : {full_target}");
            println!("  changes:");
            for (p, n) in &changed {
                let rel = p.strip_prefix(&react_root).unwrap_or(p);
                println!("    {} ({n} substitution{})", rel.display(), if *n == 1 {""} else {"s"});
            }

            if dry_run {
                println!();
                println!("--dry-run: no files written, no npm publish.");
                json!({"kind":"publish","dry_run":true,"target":full_target,"source":current_name,"changes":changed.len()})
            } else {
                println!();
                let mut args = vec!["publish".to_string(), "--access".to_string(), "public".to_string()];
                if let Some(tag) = &npm_tag {
                    args.push("--tag".into());
                    args.push(tag.clone());
                }
                println!("running: npm {}", args.join(" "));
                let status = SysCmd::new("npm")
                    .args(&args)
                    .current_dir(&react_root)
                    .status()?;
                if !status.success() {
                    anyhow::bail!("npm publish exited with status {status}");
                }
                json!({"kind":"publish","dry_run":false,"target":full_target,"source":current_name,"changes":changed.len()})
            }
        }
        Command::React { repo } => {
            let target = repo.unwrap_or_else(|| std::env::current_dir().expect("cwd"));
            let design_path = target.join("DESIGN.md");
            let design_md = std::fs::read_to_string(&design_path)
                .map_err(|e| anyhow::anyhow!("cannot read {}: {e}", design_path.display()))?;
            let doc = aphrodite_core::parse_design(&design_md)
                .map_err(|e| anyhow::anyhow!("parse DESIGN.md: {e}"))?;
            let variants = aphrodite_core::resolve_variants(&doc);
            let project_name = if doc.frontmatter.name.is_empty() {
                "aphrodite-design"
            } else {
                doc.frontmatter.name.as_str()
            };
            let pkg = aphrodite_generator::react_export::build(&variants, project_name);
            let react_root = target.join("react");
            pkg.write_to(&react_root)?;
            let tsx_count = pkg.files.keys().filter(|k| k.ends_with(".tsx")).count();
            println!(
                "wrote react/ package — {} files, {} components, in {}",
                pkg.files.len(),
                tsx_count,
                react_root.display()
            );
            json!({
                "kind": "react",
                "path": react_root.to_string_lossy(),
                "components": tsx_count,
                "files": pkg.files.len()
            })
        }
        Command::Eval { spec, out, max_turns, threshold } => {
            let specs = aphrodite_generator::eval_sweep::parse_spec_file(&spec)?;
            std::fs::create_dir_all(&out)?;
            println!("aphrodite eval — {} intent(s) → {}", specs.len(), out.display());
            let mut results = Vec::with_capacity(specs.len());
            for (i, s) in specs.iter().enumerate() {
                println!("[{}/{}] {} ({} pages) — running…", i + 1, specs.len(), s.name, if s.pages.is_empty() { 1 } else { s.pages.len() });
                let r = aphrodite_generator::eval_sweep::run_one(s, &out, max_turns, threshold).await;
                println!("  {}", aphrodite_generator::eval_sweep::format_row(&r));
                results.push(r);
            }
            let summary = aphrodite_generator::eval_sweep::summarise(&results);
            let report = aphrodite_generator::eval_sweep::EvalReport { summary: summary.clone(), results };
            let report_path = out.join("eval-report.json");
            std::fs::write(&report_path, serde_json::to_string_pretty(&report)?)?;
            println!();
            println!("=== summary ===");
            println!("total={} ok={} warning={} failed={}", summary.total, summary.ok, summary.warning, summary.failed);
            println!("mean quality = {:.1}/100 (min {}, max {})", summary.mean_quality, summary.min_quality, summary.max_quality);
            println!("total wall = {:.0}s  total llm calls = {}", summary.total_wall_clock_s, summary.total_llm_calls);
            println!("wrote {}", report_path.display());
            json!({
                "kind": "eval",
                "summary": serde_json::to_value(&report.summary)?,
                "report_path": report_path.to_string_lossy(),
            })
        }
        Command::Docs { repo } => {
            let target = repo.unwrap_or_else(|| std::env::current_dir().expect("cwd"));
            let design_md = std::fs::read_to_string(target.join("DESIGN.md"))
                .map_err(|e| anyhow::anyhow!("cannot read DESIGN.md: {e}"))?;
            let doc = aphrodite_core::parse_design(&design_md)
                .map_err(|e| anyhow::anyhow!("parse DESIGN.md: {e}"))?;
            let variants = aphrodite_core::resolve_variants(&doc);
            let project_name = if doc.frontmatter.name.is_empty() {
                "aphrodite-design"
            } else {
                doc.frontmatter.name.as_str()
            };
            let docs_dir = target.join("docs");
            std::fs::create_dir_all(&docs_dir)?;
            let html = aphrodite_generator::docs_site::build_docs_index(&variants, project_name);
            let out = docs_dir.join("index.html");
            std::fs::write(&out, &html)?;
            println!("wrote {} ({} variants, 42 component sections)", out.display(), variants.len());
            json!({ "kind": "docs", "path": out.to_string_lossy() })
        }
        Command::Figma { sub } => match sub {
            FigmaSub::Export { repo } => {
                let target = repo.unwrap_or_else(|| std::env::current_dir().expect("cwd"));
                let design_md = std::fs::read_to_string(target.join("DESIGN.md"))
                    .map_err(|e| anyhow::anyhow!("cannot read DESIGN.md: {e}"))?;
                let doc = aphrodite_core::parse_design(&design_md)
                    .map_err(|e| anyhow::anyhow!("parse DESIGN.md: {e}"))?;
                let variants = aphrodite_core::resolve_variants(&doc);
                let json = aphrodite_generator::figma_sync::build_tokens_studio_json(&variants);
                let out_path = target.join("tokens.figma.json");
                std::fs::write(&out_path, &json)?;
                println!("wrote {} ({} variants, Tokens Studio format)", out_path.display(), variants.len());
                json!({ "kind": "figma_export", "path": out_path.to_string_lossy(), "variants": variants.len() })
            }
            FigmaSub::Pull { file_key, repo } => {
                let target = repo.unwrap_or_else(|| std::env::current_dir().expect("cwd"));
                let design_md = std::fs::read_to_string(target.join("DESIGN.md"))
                    .map_err(|e| anyhow::anyhow!("cannot read DESIGN.md: {e}"))?;
                let doc = aphrodite_core::parse_design(&design_md)
                    .map_err(|e| anyhow::anyhow!("parse DESIGN.md: {e}"))?;
                let variants = aphrodite_core::resolve_variants(&doc);
                let figma_vars = aphrodite_generator::figma_sync::pull_figma_variables(&file_key).await
                    .map_err(|e| anyhow::anyhow!("figma pull: {e}"))?;
                let d = aphrodite_generator::figma_sync::diff(&variants, &figma_vars);
                println!(
                    "figma sync — matched={} only_design={} only_figma={} mismatched={}",
                    d.matched.len(), d.only_in_design.len(), d.only_in_figma.len(), d.value_mismatches.len()
                );
                for m in &d.value_mismatches {
                    println!("  MISMATCH {}  design={}  figma={}", m.key, m.design_value, m.figma_value);
                }
                for k in d.only_in_design.iter().take(20) {
                    println!("  DESIGN-ONLY  {k}");
                }
                if d.only_in_design.len() > 20 { println!("  ... + {} more", d.only_in_design.len() - 20); }
                for k in d.only_in_figma.iter().take(20) {
                    println!("  FIGMA-ONLY   {k}");
                }
                serde_json::to_value(&d)?
            }
        }
        Command::Diff { baseline, current, threshold, write_heatmaps } => {
            let current = current.unwrap_or_else(|| std::env::current_dir().expect("cwd"));
            let report = aphrodite_generator::visual_diff::diff_dirs(&baseline, &current, threshold, write_heatmaps)?;
            let s = &report.summary;
            println!(
                "visual diff — identical={} minor={} changed={} only_baseline={} only_current={}",
                s.identical, s.minor, s.changed, s.only_baseline, s.only_current
            );
            for p in &report.pairs {
                if !matches!(p.verdict, aphrodite_generator::visual_diff::DiffVerdict::Identical) {
                    println!(
                        "  {:?}  {}  baseline={}B current={}B Δ={:.1}%{}",
                        p.verdict, p.file, p.baseline_bytes, p.current_bytes, p.size_delta_pct,
                        p.imagemagick_diff_pct.map(|x| format!(" im={:.2}%", x)).unwrap_or_default()
                    );
                }
            }
            for f in &report.only_in_baseline { println!("  REMOVED  {f}"); }
            for f in &report.only_in_current { println!("  ADDED    {f}"); }
            serde_json::to_value(&report)?
        }
        Command::Gallery { dir } => {
            let out = gallery_cmd::build(&dir)?;
            println!("✓ wrote {}", out.display());
            json!({ "kind": "gallery", "path": out.to_string_lossy() })
        }
        Command::Love { repo } => feedback_cmd::run(0.30, "love", repo)?,
        Command::Hate { repo } => feedback_cmd::run(-0.30, "hate", repo)?,
        Command::Prefs { repo } => feedback_cmd::show(repo)?,
        Command::Refine { change, no_write, repo } => refine_cmd::run(change, no_write, repo).await?,
        Command::Create { intent, max_turns, satisfaction_threshold, persona, estimate, no_write, repo, pages } => {
            if estimate {
                create_cmd::estimate(intent, max_turns, satisfaction_threshold, persona, repo)?
            } else {
                create_cmd::run(intent, max_turns, satisfaction_threshold, persona, no_write, repo, pages).await?
            }
        }
        Command::Personas => personas_list(),
        Command::Wiki { sub } => match sub {
            WikiSub::List => wiki_cmd::list(),
            WikiSub::Show { slug } => wiki_cmd::show(&slug)?,
            WikiSub::Add { url, slug, title, tags, signature, body, body_from_file, overwrite, auto_fetch } => {
                wiki_cmd::add(url, slug, title, tags, signature, body, body_from_file, overwrite, auto_fetch).await?
            }
        },
        Command::Assets { sub } => match sub {
            AssetsSub::List { repo } => assets_cmd::list(repo)?,
            AssetsSub::Clean { repo } => assets_cmd::clean(repo)?,
        },
        Command::Curator { dry_run, stale_after_days, archive_after_days } => {
            curator_cmd::run(dry_run, stale_after_days, archive_after_days)?
        }
        Command::Log { n, repo } => log_cmd::show_log(repo, n)?,
        Command::Undo { n, yes, repo } => log_cmd::undo(repo, n, yes)?,
        Command::Preview { repo, file } => preview_cmd::run(repo, file)?,
    };

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else {
        render_pretty(&payload);
    }
    Ok(())
}

const ALL_PROVIDERS: &[&str] = &["zai", "anthropic", "openrouter", "openai", "moonshot", "gemini"];

fn version_summary(_json: bool) -> anyhow::Result<serde_json::Value> {
    banner::print(env!("CARGO_PKG_VERSION"));
    let cfg = aphrodite_core::config::load();
    let active = cfg.default_provider.clone();
    let configured: Vec<_> = ALL_PROVIDERS
        .iter()
        .filter(|p| aphrodite_keyring::fetch(p).is_ok())
        .map(|s| s.to_string())
        .collect();
    if let Some(p) = active.as_deref() {
        let pc = cfg.providers.get(p);
        eprintln!(
            "  active provider : {} {} model={} plan={}",
            console::style(p).bold().yellow(),
            console::style("·").dim(),
            console::style(pc.and_then(|c| c.model.as_deref()).unwrap_or("(default)")).cyan(),
            console::style(pc.and_then(|c| c.plan.as_deref()).unwrap_or("(default)")).cyan()
        );
    } else {
        eprintln!(
            "  active provider : {}  (run `aphrodite init` to pick one)",
            console::style("offline").dim()
        );
    }
    eprintln!(
        "  configured keys : {}",
        if configured.is_empty() { "(none)".into() } else { configured.join(", ") }
    );
    Ok(json!({
        "kind": "version",
        "version": env!("CARGO_PKG_VERSION"),
        "active_provider": active,
        "configured": configured,
    }))
}

fn auth_status() -> serde_json::Value {
    let configured: Vec<_> = ALL_PROVIDERS
        .iter()
        .map(|p| {
            let present = aphrodite_keyring::fetch(p).is_ok();
            json!({ "provider": p, "configured": present })
        })
        .collect();
    json!({ "kind": "auth_status", "providers": configured })
}

fn auth_set(
    provider: &str,
    from_env: Option<&str>,
    from_stdin: bool,
    from_file: Option<&std::path::Path>,
    keep_file: bool,
) -> anyhow::Result<serde_json::Value> {
    if !ALL_PROVIDERS.contains(&provider) {
        anyhow::bail!("unknown provider `{provider}`; supported: {}", ALL_PROVIDERS.join(", "));
    }
    let raw = if let Some(name) = from_env {
        std::env::var(name).map_err(|_| anyhow::anyhow!("env var {name} unset"))?
    } else if from_stdin {
        use std::io::Read;
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        buf
    } else if let Some(path) = from_file {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("could not read --from-file {}: {e}", path.display()))?;
        if !keep_file {
            // Best-effort delete; warn if it fails but don't abort — the user
            // explicitly handed us this file.
            if let Err(e) = std::fs::remove_file(path) {
                eprintln!("  ⚠ could not delete source file after read: {e}");
            } else {
                eprintln!("  ✓ source file deleted: {}", path.display());
            }
        }
        contents
    } else {
        rpassword::prompt_password(format!("API key for {provider} (paste, hidden): "))?
    };
    let key = sanitize_secret(&raw);
    eprintln!("Captured {} characters (after cleanup).", key.chars().count());
    if key.is_empty() {
        anyhow::bail!(
            "empty key — nothing stored. If the hidden prompt isn't working in your terminal, try one of:\n  pbpaste | aphrodite auth set {provider} --from-stdin\n  echo \"<key>\" > /tmp/k && aphrodite auth set {provider} --from-file /tmp/k\n  APHRODITE_{}_API_KEY=<key> aphrodite auth set {provider} --from-env APHRODITE_{}_API_KEY",
            provider.to_ascii_uppercase(),
            provider.to_ascii_uppercase()
        );
    }
    // `store()` now verifies the round-trip internally; if it returns Ok the
    // key is in the keychain *and* readable.
    aphrodite_keyring::store(provider, &key).map_err(|e| anyhow::anyhow!(
        "Keychain write or readback failed: {e}.\n\
         On macOS this usually means the Keychain Access dialog was denied or dismissed.\n\
         Steps:\n  1. Open Keychain Access app\n  2. Search for `aphrodite`\n  3. Delete any existing entry (or right-click → Get Info → Access Control → Always allow)\n  4. Run `aphrodite auth set {provider}` again — this time click `Always Allow` when prompted.\n  Or skip the keychain entirely: `export APHRODITE_{}_API_KEY=...` in your shell rc.",
        provider.to_ascii_uppercase()
    ))?;
    Ok(json!({ "kind": "auth_set", "provider": provider, "stored": true, "verified": true, "key_chars": key.chars().count() }))
}

/// Strip bracketed-paste wrappers (`\x1b[200~ … \x1b[201~`), CR/LF, NULs, and
/// surrounding whitespace from a captured secret. This is the single biggest
/// silent failure mode for keys pasted into hidden prompts on macOS — the
/// terminal emits the wrappers, rpassword captures them verbatim, and the
/// resulting "key" is invalid to every API. We never print the key, only its
/// post-sanitisation length.
fn sanitize_secret(raw: &str) -> String {
    let mut s = raw.to_string();
    // Bracketed-paste open/close markers.
    s = s.replace("\u{1b}[200~", "").replace("\u{1b}[201~", "");
    // Lone ESC, then any stray newlines and NULs.
    s = s.replace('\u{1b}', "");
    s.retain(|c| c != '\r' && c != '\n' && c != '\0');
    s.trim().to_string()
}

fn auth_verify(provider: &str) -> serde_json::Value {
    use console::style;
    if !ALL_PROVIDERS.contains(&provider) {
        return json!({ "kind": "auth_verify", "provider": provider, "ok": false, "reason": "unknown provider" });
    }
    match aphrodite_keyring::fetch(provider) {
        Ok(k) => {
            eprintln!(
                "  {} {} present in OS keychain ({} chars)",
                style("✓").green(),
                provider,
                k.chars().count()
            );
            json!({ "kind": "auth_verify", "provider": provider, "ok": true, "key_chars": k.chars().count() })
        }
        Err(e) => {
            eprintln!(
                "  {} {} NOT readable from keychain: {}",
                style("✖").red(),
                provider,
                e
            );
            eprintln!(
                "  {} Try `aphrodite auth set {}` again, and on macOS click `Always Allow` if prompted.",
                style("→").dim(),
                provider
            );
            json!({ "kind": "auth_verify", "provider": provider, "ok": false, "error": e.to_string() })
        }
    }
}

fn auth_remove(provider: &str) -> serde_json::Value {
    let removed = aphrodite_keyring::delete(provider).is_ok();
    json!({ "kind": "auth_remove", "provider": provider, "removed": removed })
}

fn doctor() -> serde_json::Value {
    use console::style;
    let cfg = aphrodite_core::config::load();
    let mut issues: Vec<serde_json::Value> = Vec::new();
    let mut checks: Vec<serde_json::Value> = Vec::new();

    // Check 1: config existence + parseability.
    let cfg_path = aphrodite_core::config::config_path();
    let cfg_present = cfg_path.exists();
    eprintln!(
        "  {} config file ({})",
        if cfg_present { style("✓").green() } else { style("○").dim() },
        style(cfg_path.display()).dim()
    );
    checks.push(json!({ "name": "config_file_present", "ok": cfg_present }));

    // Check 2: default provider declared.
    let default = cfg.default_provider.clone();
    let has_default = default.is_some();
    eprintln!(
        "  {} default provider: {}",
        if has_default { style("✓").green() } else { style("○").dim() },
        default.as_deref().unwrap_or("(none — will use offline)")
    );

    // Check 3: keychain readback for the default provider.
    let mut keychain_ok = false;
    if let Some(p) = default.as_deref() {
        match aphrodite_keyring::fetch(p) {
            Ok(k) => {
                keychain_ok = !k.trim().is_empty();
                if keychain_ok {
                    eprintln!(
                        "  {} keychain entry for `{}` readable ({} chars)",
                        style("✓").green(),
                        p,
                        k.chars().count()
                    );
                } else {
                    eprintln!(
                        "  {} keychain entry for `{}` exists but is empty",
                        style("✖").red(),
                        p
                    );
                    issues.push(json!({
                        "code": "keychain_empty",
                        "fix": format!("aphrodite auth set {p}")
                    }));
                }
            }
            Err(e) => {
                eprintln!(
                    "  {} keychain entry for `{}` not readable: {}",
                    style("✖").red(),
                    p,
                    e
                );
                issues.push(json!({
                    "code": "keychain_unreadable",
                    "fix": format!("aphrodite auth set {p} — or on macOS check Keychain Access for 'Always Allow' on the aphrodite binary"),
                    "raw": e.to_string(),
                }));
            }
        }
    }
    checks.push(json!({ "name": "default_keychain_readable", "ok": keychain_ok }));

    // Check 4: env-var fallback for the default provider.
    let mut env_ok = false;
    if let Some(p) = default.as_deref() {
        let env_keys: &[&str] = match p {
            "zai" => &["APHRODITE_ZAI_API_KEY", "ZAI_API_KEY", "GLM_API_KEY"],
            "anthropic" => &["APHRODITE_ANTHROPIC_API_KEY", "ANTHROPIC_API_KEY"],
            "openrouter" => &["APHRODITE_OPENROUTER_API_KEY", "OPENROUTER_API_KEY"],
            _ => &[],
        };
        for name in env_keys {
            if std::env::var(name).map(|v| !v.trim().is_empty()).unwrap_or(false) {
                eprintln!("  {} env fallback present: {}", style("✓").green(), name);
                env_ok = true;
                break;
            }
        }
        if !env_ok {
            eprintln!(
                "  {} no env fallback for `{}` (tried: {})",
                style("○").dim(),
                p,
                env_keys.join(", ")
            );
        }
    }
    checks.push(json!({ "name": "env_fallback_present", "ok": env_ok }));

    // Verdict.
    let credential_ok = keychain_ok || env_ok;
    let verdict = if credential_ok {
        eprintln!("  {} ready for real provider calls", style("●").green());
        "healthy"
    } else if has_default {
        eprintln!(
            "  {} config intends `{}` but no credential is reachable — calls will fall back to offline",
            style("⚠").yellow(),
            default.as_deref().unwrap_or("?")
        );
        "degraded_offline_only"
    } else {
        eprintln!(
            "  {} no provider configured — running offline by design",
            style("○").dim()
        );
        "offline_by_design"
    };

    json!({
        "kind": "doctor",
        "verdict": verdict,
        "checks": checks,
        "issues": issues,
        "default_provider": default,
    })
}

fn capabilities() -> serde_json::Value {
    use console::style;
    let cap = json!({
        "kind": "capabilities",
        "version": env!("CARGO_PKG_VERSION"),
        "in_scope": {
            "design.modes": ["light", "dark", "brand"],
            "design.outputs": ["DESIGN.md (Google Labs alpha)", "hero.html (self-contained, no external network)"],
            "providers.api_key": ["zai", "anthropic", "openrouter"],
            "providers.offline_fallback": true,
            "validation": ["schema (Google Labs alpha)", "WCAG-AA contrast across all variants"],
            "taste_loop": "implicit (Regenerate signals bias next call)",
            "write_modes": ["commit (default)", "artifact_only"],
        },
        "out_of_scope_v01": [
            "image generation / asset fetching",
            "motion / video (HyperFrames adapter lands in v0.2)",
            "3D scenes / three.js / Blender (v0.3)",
            "Figma / Sketch round-trip (v0.2)",
            "explicit aesthetic jury (implicit signals only in v0.1)",
            "OAuth flows for any provider (v0.2; API-key only at v0.1)",
        ],
    });

    println!("{}", style("Aphrodite — v0.1 capabilities").bold().magenta());
    println!();
    println!("  {}", style("In scope:").bold());
    println!("    • design / redesign / validate / auth_status MCP tools");
    println!("    • 4 variants per DESIGN.md (light, dark, brand-a, brand-b)");
    println!("    • WCAG-AA contrast gate, schema gate");
    println!("    • Providers: z.ai GLM (API key), Anthropic (API key), OpenRouter (API key)");
    println!("    • Offline deterministic fallback (no network, no cost)");
    println!("    • Implicit taste loop — `redesign` shifts subsequent palettes");
    println!("    • Direct-commit by default; `--no-write` for artifact-only mode");
    println!();
    println!("  {}", style("Out of v0.1 scope (will surface as warnings if asked):").dim());
    println!("    • image generation, asset fetching");
    println!("    • motion / video (HyperFrames adapter, v0.2)");
    println!("    • 3D / three.js / Blender (v0.3)");
    println!("    • Figma round-trip (v0.2)");
    println!("    • OAuth (v0.2)");
    println!();
    cap
}

fn personas_list() -> serde_json::Value {
    use console::style;
    // Seed bundled before listing.
    let _ = aphrodite_core::personas::seed_bundled_personas();
    let mut entries = Vec::new();
    for slug in aphrodite_core::personas::list() {
        if let Ok(p) = aphrodite_core::personas::load(&slug) {
            println!(
                "  {}  {} {}",
                style(format!("{:18}", slug)).bold().cyan(),
                p.frontmatter.name,
                style(format!("({})", p.frontmatter.era)).dim()
            );
            if !p.frontmatter.voice.is_empty() {
                println!("    {}", style(&p.frontmatter.voice).dim());
            }
            entries.push(json!({
                "slug": slug,
                "name": p.frontmatter.name,
                "era": p.frontmatter.era,
                "voice": p.frontmatter.voice,
                "when_to_invoke": p.frontmatter.when_to_invoke,
            }));
        }
    }
    json!({ "kind": "personas", "personas": entries })
}

fn render_pretty(payload: &serde_json::Value) {
    use console::style;
    let kind = payload.get("kind").and_then(|v| v.as_str()).unwrap_or("");
    match kind {
        "auth_status" => {
            println!("{}", style("Aphrodite — auth status").bold().cyan());
            if let Some(arr) = payload.get("providers").and_then(|v| v.as_array()) {
                for p in arr {
                    let name = p.get("provider").and_then(|v| v.as_str()).unwrap_or("?");
                    let ok = p.get("configured").and_then(|v| v.as_bool()).unwrap_or(false);
                    let badge = if ok {
                        style("● configured").green()
                    } else {
                        style("○ not set    ").dim()
                    };
                    println!("  {badge}  {name}");
                }
            }
        }
        "setup" => {
            println!("{}", style("Aphrodite — setup").bold().cyan());
            if let Some(msg) = payload.get("message").and_then(|v| v.as_str()) {
                println!("  {msg}");
            }
        }
        "design" => {
            println!("{}", style("Aphrodite — design").bold().magenta());
            if let Some(out) = payload.get("output").and_then(|v| v.as_object()) {
                let provider = out.get("provider_used").and_then(|v| v.as_str()).unwrap_or("?");
                println!("  Provider     : {}", style(provider).yellow());
                if let Some(files) = out.get("files").and_then(|v| v.as_array()) {
                    println!("  Written      :");
                    for f in files {
                        if let Some(p) = f.as_str() {
                            println!("    • {p}");
                        }
                    }
                }
                if let Some(committed) = out.get("committed").and_then(|v| v.as_bool()) {
                    let badge = if committed {
                        style("yes").green()
                    } else {
                        style("no (artifact-only)").dim()
                    };
                    println!("  Committed    : {badge}");
                }
                if let Some(validation) = out.get("validation").and_then(|v| v.as_object()) {
                    let ok = validation.get("ok").and_then(|v| v.as_bool()).unwrap_or(false);
                    let n = validation
                        .get("violations")
                        .and_then(|v| v.as_array())
                        .map(|a| a.len())
                        .unwrap_or(0);
                    if ok {
                        println!("  Validation   : {}", style("PASS").green());
                    } else {
                        println!(
                            "  Validation   : {} ({n} violation{})",
                            style("FAIL").red(),
                            if n == 1 { "" } else { "s" }
                        );
                        if let Some(arr) = validation.get("violations").and_then(|v| v.as_array()) {
                            for v in arr.iter().take(5) {
                                println!("    - {}", v.get("message").and_then(|x| x.as_str()).unwrap_or("?"));
                            }
                        }
                    }
                }
                if let Some(warnings) = out.get("warnings").and_then(|v| v.as_array()) {
                    if !warnings.is_empty() {
                        println!("  Warnings     :");
                        for w in warnings {
                            let kind = w.get("kind").and_then(|v| v.as_str()).unwrap_or("?");
                            let msg = w.get("message").and_then(|v| v.as_str()).unwrap_or("");
                            println!("    {} {} — {}", style("⚠").yellow(), style(kind).yellow(), msg);
                            if let Some(hint) = w.get("hint").and_then(|v| v.as_str()) {
                                println!("      {} {}", style("→").dim(), style(hint).dim());
                            }
                        }
                    }
                }
                if let Some(hero) = out.get("hero_path").and_then(|v| v.as_str()) {
                    println!("\n  {} {}", style("Open in browser:").dim(), style(format!("file://{hero}")).underlined());
                }
            }
        }
        _ => {
            println!("{}", serde_json::to_string_pretty(payload).unwrap_or_default());
        }
    }
}
