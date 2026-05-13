//! Banner — Aphrodite's first impression.
//!
//! The figure on the left is the Venus de Milo (Aphrodite of Melos) ASCII art
//! by H. P. Barmario (Morfina), 2014-05-23 — sourced from ascii.co.uk under
//! community attribution. The "mrf" signature within the art is preserved as
//! per ASCII Art Archive convention.
//!
//! Aphrodite layers stars, sea waves, a pedestal, and the wordmark/tagline
//! around the figure.

use console::style;

/// 24-line statue figure (Venus de Milo by H. P. Barmario "mrf").
const FIGURE: &[&str] = &[
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

/// Right-side text panel, line for line. Empty rows align with the figure's
/// blank decorative rows.
const RIGHT_PANEL: &[(&str, Style)] = &[
    ("",                                                            Style::None),
    ("",                                                            Style::None),
    ("",                                                            Style::None),
    ("                ✦",                                           Style::Star),
    ("            .       .",                                       Style::Star),
    ("        *               *",                                   Style::Star),
    ("",                                                            Style::None),
    ("A   P   H   R   O   D   I   T   E",                           Style::Wordmark),
    ("─────────────────────────────────",                           Style::Rule),
    ("",                                                            Style::None),
    ("the UI generation harness",                                   Style::Tagline),
    ("undeniably beautiful, every time",                            Style::Tagline),
    ("",                                                            Style::None),
    ("called by humans and AI agents alike,",                       Style::Dim),
    ("under a single DESIGN.md contract.",                          Style::Dim),
    ("",                                                            Style::None),
    ("open-source · Apache-2.0",                                    Style::Dim),
    ("",                                                            Style::None),
    ("art: Venus de Milo · H.P. Barmario (mrf)",                    Style::Attribution),
    ("",                                                            Style::None),
    ("",                                                            Style::None),
    ("",                                                            Style::None),
    ("",                                                            Style::None),
    ("",                                                            Style::None),
];

const WAVES: &[&str] = &[
    " ~ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ~",
    "  ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋ ≋",
];

#[derive(Copy, Clone)]
enum Style {
    None,
    Star,
    Wordmark,
    Rule,
    Tagline,
    Dim,
    Attribution,
}

fn paint(text: &str, kind: Style) -> String {
    match kind {
        Style::None => text.to_string(),
        Style::Star => style(text).cyan().dim().to_string(),
        Style::Wordmark => style(text).bold().magenta().to_string(),
        Style::Rule => style(text).magenta().dim().to_string(),
        Style::Tagline => style(text).italic().to_string(),
        Style::Dim => style(text).dim().to_string(),
        Style::Attribution => style(text).dim().italic().to_string(),
    }
}

pub fn print(version: &str) {
    eprintln!();
    let rows = FIGURE.len().max(RIGHT_PANEL.len());
    for i in 0..rows {
        let fig = FIGURE.get(i).copied().unwrap_or("                  ");
        let (right_text, right_style) = RIGHT_PANEL.get(i).copied().unwrap_or(("", Style::None));
        let right_text = if i == 9 {
            // Append version on the rule line for compactness.
            format!("{right_text}     v{version}")
        } else {
            right_text.to_string()
        };
        eprintln!(
            "    {}    {}",
            style(fig).magenta().dim(),
            paint(&right_text, right_style)
        );
    }
    for w in WAVES {
        eprintln!("    {}", style(w).blue().dim());
    }
    eprintln!();
}

/// Compact one-liner — used by `aphrodite version` -- actually we also use the
/// full banner there; this is a slot reserved for future inline contexts.
#[allow(dead_code)]
pub fn print_compact() {
    eprintln!(
        "{} {}",
        style("✦ aphrodite").bold().magenta(),
        style("· the UI generation harness").dim()
    );
}
