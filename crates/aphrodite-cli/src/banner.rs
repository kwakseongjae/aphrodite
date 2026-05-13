//! Banner — ASCII art shown by `aphrodite init` and `aphrodite version`.

use console::style;

const ART: &str = r#"
                       ✦
                    *       *
                .               .

              .::::::::::.
            .::''        '::.
           ::      ✦       ::
           ::   .·°°°·.    ::
            ':.°       °.:'
              ':._   _.:'
                 │ │
              ═══╪═╪═══
            ═══════════
          ═══════════════
        ≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋
       ≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋
"#;

pub fn print(version: &str) {
    let lines: Vec<&str> = ART.lines().collect();
    for line in &lines {
        eprintln!("{}", style(line).magenta().dim());
    }
    eprintln!();
    let title = "A   P   H   R   O   D   I   T   E";
    eprintln!("        {}", style(title).bold().magenta());
    eprintln!("        {}", style("─".repeat(33)).dim());
    eprintln!(
        "        {}  {}",
        style("the UI generation harness").italic().dim(),
        style(format!("· v{version}")).dim()
    );
    eprintln!(
        "        {}",
        style("undeniably beautiful, every time").italic().dim()
    );
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
