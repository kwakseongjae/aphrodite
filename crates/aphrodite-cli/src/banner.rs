//! Banner — Aphrodite's first impression.
//!
//! Renders Botticelli's *The Birth of Venus* (1485, public domain) as a
//! truecolor half-block image (▀ with 24-bit FG/BG per cell), pre-rendered at
//! build time via `chafa --symbols half --colors=full` into
//! `banner_venus.ansi`. The image is the canonical Aphrodite myth — the
//! goddess of beauty *arriving*. Stars, a wordmark, tagline, and waves layer
//! around her in the same banner.
//!
//! Fallback: when truecolor isn't supported (e.g. `NO_COLOR`, dumb term,
//! piped to a file), we use the H. P. Barmario ("mrf") Venus de Milo ASCII
//! art instead. The mrf signature is preserved per ASCII Art Archive
//! convention.

use console::style;

/// Pre-rendered Botticelli — 32 rows × ~28 cols, truecolor half-blocks.
/// Generated once at build time so runtime has zero deps and ~instant cold start.
const VENUS_TRUECOLOR: &str = include_str!("banner_venus.ansi");

/// Fallback figure — H. P. Barmario (Morfina), 2014-05-23, via ascii.co.uk.
/// `mrf` signature kept inline per ASCII Art Archive convention.
const VENUS_ASCII: &[&str] = &[
    r"       %%%        ",
    r"      =====       ",
    r"     &%&%%%&      ",
    r"     %& < <%      ",
    r"      &___/       ",
    r"       \ |____    ",
    r"      .', ,  ()   ",
    r"     / -.  _)|    ",
    r"    |_(_.    |    ",
    r"    '-'\  )  |    ",
    r"    mrf )    |    ",
    r"       /  .  ).   ",
    r"      /    _. |   ",
    r"    /'---':.-'|   ",
    r"   (__.' /    /   ",
    r"    \   ( /  /    ",
    r"     \ /  _  |    ",
    r"      \  |  '|    ",
    r"      | . \  |    ",
    r"      |(     |    ",
    r"      |  \ \ |    ",
    r"       \  )\ |    ",
    r"     __)/ / \     ",
    "  --\"--(_.Ooo'-- ",
];

const RIGHT_PANEL: &[(&str, Accent)] = &[
    ("        ✦", Accent::Star),
    ("    .         .", Accent::Star),
    ("        *", Accent::Star),
    ("", Accent::None),
    ("A   P   H   R   O   D   I   T   E", Accent::Wordmark),
    ("─────────────────────────────────", Accent::Rule),
    ("", Accent::None),
    ("the UI generation harness", Accent::Tagline),
    ("undeniably beautiful, every time", Accent::Tagline),
    ("", Accent::None),
    ("called by humans and AI agents alike,", Accent::Dim),
    ("under a single DESIGN.md contract.", Accent::Dim),
    ("", Accent::None),
    ("open-source · Apache-2.0", Accent::Dim),
    ("", Accent::None),
    ("art:  Sandro Botticelli", Accent::Attribution),
    ("      The Birth of Venus, 1485", Accent::Attribution),
    ("      (public domain)", Accent::Attribution),
    ("", Accent::None),
];

const WAVES: &[&str] = &[
    " ~ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ~",
    "  ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋",
];

#[derive(Copy, Clone)]
enum Accent {
    None,
    Star,
    Wordmark,
    Rule,
    Tagline,
    Dim,
    Attribution,
}

fn paint(text: &str, kind: Accent) -> String {
    match kind {
        Accent::None => text.to_string(),
        Accent::Star => style(text).cyan().dim().to_string(),
        Accent::Wordmark => style(text).bold().magenta().to_string(),
        Accent::Rule => style(text).magenta().dim().to_string(),
        Accent::Tagline => style(text).italic().to_string(),
        Accent::Dim => style(text).dim().to_string(),
        Accent::Attribution => style(text).dim().italic().to_string(),
    }
}

/// True if the current terminal can render 24-bit color half-blocks.
fn supports_truecolor() -> bool {
    // Honor user opt-out first.
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if let Ok(t) = std::env::var("COLORTERM") {
        let t = t.to_ascii_lowercase();
        if t == "truecolor" || t == "24bit" {
            return true;
        }
    }
    // Most modern terminals on macOS/Linux advertise truecolor through TERM_PROGRAM.
    if let Ok(p) = std::env::var("TERM_PROGRAM") {
        matches!(
            p.as_str(),
            "iTerm.app" | "Apple_Terminal" | "WarpTerminal" | "ghostty" | "vscode" | "Hyper"
        )
    } else {
        // Fall back to TERM hints.
        std::env::var("TERM")
            .map(|t| t.contains("256color") || t.contains("truecolor") || t.contains("kitty"))
            .unwrap_or(false)
    }
}

pub fn print(version: &str) {
    if supports_truecolor() {
        print_truecolor(version);
    } else {
        print_ascii(version);
    }
}

fn print_truecolor(version: &str) {
    eprintln!();

    // Strip chafa's leading cursor-hide and trailing cursor-show so we don't
    // interfere with terminal state.
    let cleaned = VENUS_TRUECOLOR
        .trim_start_matches("\x1b[?25l")
        .trim_end_matches('\n')
        .trim_end_matches("\x1b[?25h");

    let figure_lines: Vec<&str> = cleaned.lines().collect();
    let rows = figure_lines.len().max(RIGHT_PANEL.len());

    for i in 0..rows {
        let fig = figure_lines.get(i).copied().unwrap_or("");
        let (right_text, accent) = RIGHT_PANEL.get(i).copied().unwrap_or(("", Accent::None));
        let right_text = if i == 5 {
            format!("{right_text}     v{version}")
        } else {
            right_text.to_string()
        };
        eprintln!("    {}    {}", fig, paint(&right_text, accent));
    }
    for w in WAVES {
        eprintln!("    {}", style(w).blue().dim());
    }
    eprintln!();
}

fn print_ascii(version: &str) {
    eprintln!();
    let rows = VENUS_ASCII.len().max(RIGHT_PANEL.len());
    for i in 0..rows {
        let fig = VENUS_ASCII.get(i).copied().unwrap_or("                  ");
        let (right_text, accent) = RIGHT_PANEL.get(i).copied().unwrap_or(("", Accent::None));
        let right_text = if i == 5 {
            format!("{right_text}     v{version}")
        } else {
            right_text.to_string()
        };
        eprintln!(
            "    {}    {}",
            style(fig).magenta().dim(),
            paint(&right_text, accent)
        );
    }
    for w in WAVES {
        eprintln!("    {}", style(w).blue().dim());
    }
    eprintln!();
}

#[allow(dead_code)]
pub fn print_compact() {
    eprintln!(
        "{} {}",
        style("✦ aphrodite").bold().magenta(),
        style("· the UI generation harness").dim()
    );
}
