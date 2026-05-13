//! Banner — the "wow" first impression.
//!
//! Design choices, after rejecting photographic half-blocks:
//!   - Identity, not imagery. `♀` (Venus glyph) + an `APHRODITE` block
//!     wordmark in a pink→magenta→gold gradient. Aphrodite's colors.
//!   - Charmbracelet-style rounded panels (lipgloss aesthetic) for the
//!     tagline, version, and license.
//!   - A short type-in animation (≤ 400 ms total) on `init` and `version`
//!     when the terminal is a TTY and supports truecolor. Otherwise we
//!     print the same content statically — same wow shape, no motion.
//!   - Speed is part of wow. Total banner time ≤ 0.5 s. Other subcommands
//!     don't show a banner; they use a single `♀ aphrodite …` accent line.

use console::style;
use std::io::{IsTerminal, Write};
use std::time::Duration;

/// Three-row APHRODITE wordmark. Hand-designed (Unicode block half-cells), 37 cols.
const WORDMARK: [&str; 3] = [
    "█▀█ █▀█ █ █ █▀█ █▀█ █▀▄ █ ▀█▀ █▀▀",
    "█▀█ █▀▀ █▀█ █▀▄ █ █ █ █ █  █  █▀▀",
    "▀ ▀ ▀   ▀ ▀ ▀ ▀ ▀▀▀ ▀▀  ▀  ▀  ▀▀▀",
];

const GLYPH: &str = "♀";

/// Sparkle row above the wordmark (cosmetic).
const SPARKLES: &str = "  ·  ⋆  ·       ·    ✦    ·       ·  ⋆  ·";

/// Three-stop gradient: rose-pink → magenta → gold.
const STOP_PINK: (u8, u8, u8) = (245, 182, 194);
const STOP_MAGENTA: (u8, u8, u8) = (216, 71, 184);
const STOP_GOLD: (u8, u8, u8) = (245, 212, 114);

fn lerp(a: (u8, u8, u8), b: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
    let m = |x: u8, y: u8| ((x as f32) * (1.0 - t) + (y as f32) * t).round() as u8;
    (m(a.0, b.0), m(a.1, b.1), m(a.2, b.2))
}

fn gradient(col: usize, total: usize) -> (u8, u8, u8) {
    let t = if total <= 1 { 0.0 } else { col as f32 / (total - 1) as f32 };
    if t < 0.5 {
        lerp(STOP_PINK, STOP_MAGENTA, t * 2.0)
    } else {
        lerp(STOP_MAGENTA, STOP_GOLD, (t - 0.5) * 2.0)
    }
}

/// Paint a row with a per-column gradient. Spaces are emitted bare so they
/// inherit the parent terminal background.
fn paint_gradient(row: &str, total_cols: usize) -> String {
    let mut out = String::new();
    for (i, ch) in row.chars().enumerate() {
        if ch == ' ' {
            out.push(' ');
            continue;
        }
        let (r, g, b) = gradient(i, total_cols);
        out.push_str(&format!("\x1b[38;2;{r};{g};{b}m{ch}"));
    }
    out.push_str("\x1b[0m");
    out
}

/// Truecolor capability sniff. Returns false for `NO_COLOR`, dumb terminals,
/// or pipes (no TTY).
fn supports_truecolor() -> bool {
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if !std::io::stderr().is_terminal() {
        return false;
    }
    if let Ok(t) = std::env::var("COLORTERM") {
        let t = t.to_ascii_lowercase();
        if t == "truecolor" || t == "24bit" {
            return true;
        }
    }
    if let Ok(p) = std::env::var("TERM_PROGRAM") {
        if matches!(
            p.as_str(),
            "iTerm.app" | "Apple_Terminal" | "WarpTerminal" | "ghostty" | "vscode" | "Hyper"
        ) {
            return true;
        }
    }
    std::env::var("TERM")
        .map(|t| t.contains("256color") || t.contains("truecolor") || t.contains("kitty"))
        .unwrap_or(false)
}

fn animation_enabled() -> bool {
    // Animation requires a TTY *and* truecolor *and* the user not opting out.
    if std::env::var_os("APHRODITE_NO_ANIM").is_some() {
        return false;
    }
    supports_truecolor()
}

pub fn print(version: &str) {
    if animation_enabled() {
        animate(version);
    } else if supports_truecolor() {
        print_static_gradient(version);
    } else {
        print_static_plain(version);
    }
}

/// Charmbracelet-style rounded panel. Width is the *interior* width.
fn render_panel(lines: &[(&str, console::Style)], width: usize) -> Vec<String> {
    let top = format!("╭{}╮", "─".repeat(width + 2));
    let bot = format!("╰{}╯", "─".repeat(width + 2));
    let mut out = vec![style(&top).color256(213).dim().to_string()];
    for (text, s) in lines {
        let visible_len = text.chars().count();
        let padding = width.saturating_sub(visible_len);
        out.push(format!(
            "{} {}{} {}",
            style("│").color256(213).dim(),
            s.clone().apply_to(text),
            " ".repeat(padding),
            style("│").color256(213).dim(),
        ));
    }
    out.push(style(&bot).color256(213).dim().to_string());
    out
}

fn panel_lines(version: &str) -> Vec<String> {
    let v = format!("v{version}");
    render_panel(
        &[
            ("♀  the UI generation harness", console::Style::new().bold()),
            ("   undeniably beautiful, every time", console::Style::new().italic()),
            ("", console::Style::new()),
            (
                &format!("   {v}  ·  Apache-2.0  ·  open-source"),
                console::Style::new().dim(),
            ),
        ]
        .iter()
        .map(|(s, st)| (*s, st.clone()))
        .collect::<Vec<_>>()
        .as_slice(),
        38,
    )
}

fn print_glyph_centered() {
    // Big gold ♀ centered above the wordmark; the wordmark is 37 cols wide.
    let pad = " ".repeat(17);
    eprintln!("{}{}", pad, style(GLYPH).color256(220).bold());
}

fn print_sparkles_centered() {
    let (r, g, b) = STOP_PINK;
    eprintln!("\x1b[38;2;{r};{g};{b}m{SPARKLES}\x1b[0m");
}

fn animate(version: &str) {
    let stderr = std::io::stderr();
    let mut out = stderr.lock();
    let total_cols = WORDMARK[0].chars().count();

    // 1) Sparkles (instant)
    eprintln!();
    print_sparkles_centered();

    // 2) Glyph appears (instant)
    print_glyph_centered();

    // 3) Wordmark types in (left-to-right, 4 cols at a time)
    eprintln!();
    // Reserve three blank rows we'll overwrite in place.
    eprintln!();
    eprintln!();
    eprintln!();

    let step = 4usize;
    let mut col = step;
    while col < total_cols + step {
        let visible = col.min(total_cols);
        // Move cursor up 3 lines to the start of the reserved wordmark area.
        let _ = write!(out, "\x1b[3F");
        for row in WORDMARK {
            let prefix: String = row.chars().take(visible).collect();
            let _ = writeln!(out, "\x1b[K{}", paint_gradient(&prefix, total_cols));
        }
        let _ = out.flush();
        std::thread::sleep(Duration::from_millis(14));
        col += step;
    }

    // 4) Rounded panel slides in (one row per ~22ms)
    eprintln!();
    for line in panel_lines(version) {
        eprintln!("{line}");
        std::thread::sleep(Duration::from_millis(22));
    }
    eprintln!();
}

fn print_static_gradient(version: &str) {
    let total = WORDMARK[0].chars().count();
    eprintln!();
    print_sparkles_centered();
    print_glyph_centered();
    eprintln!();
    for row in WORDMARK {
        eprintln!("{}", paint_gradient(row, total));
    }
    eprintln!();
    for line in panel_lines(version) {
        eprintln!("{line}");
    }
    eprintln!();
}

fn print_static_plain(version: &str) {
    eprintln!();
    eprintln!("{SPARKLES}");
    eprintln!("                 {GLYPH}");
    eprintln!();
    for row in WORDMARK {
        eprintln!("{row}");
    }
    eprintln!();
    eprintln!("┌──────────────────────────────────────────┐");
    eprintln!("│  ♀  the UI generation harness            │");
    eprintln!("│     undeniably beautiful, every time     │");
    eprintln!("│                                          │");
    eprintln!("│     v{version}  ·  Apache-2.0              │");
    eprintln!("└──────────────────────────────────────────┘");
    eprintln!();
}

#[allow(dead_code)]
pub fn print_compact() {
    eprintln!(
        "{} {}",
        style(format!("{GLYPH} aphrodite")).color256(213).bold(),
        style("· the UI generation harness").dim()
    );
}
