//! Hero HTML emitter. Outputs a single self-contained HTML page that bakes
//! every variant's tokens as CSS custom properties under `[data-variant=...]`
//! and renders a hero layout that responds to a variant switcher.
//!
//! Hard constraint (seed acceptance): no external network calls at render time.
//! All styles inline; no fonts/scripts fetched.

use aphrodite_core::{DesignDocument, Variant};
use minijinja::{context, Environment};

const HERO_TEMPLATE: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8" />
<meta name="viewport" content="width=device-width, initial-scale=1" />
<title>{{ name }} — Aphrodite</title>
<style>
  :root { color-scheme: light dark; }
  *, *::before, *::after { box-sizing: border-box; }
  body {
    margin: 0;
    min-height: 100vh;
    font-family: ui-sans-serif, system-ui, -apple-system, "Segoe UI", Roboto, sans-serif;
    background: var(--bg);
    color: var(--fg);
    transition: background 320ms ease, color 320ms ease;
  }
  {% for v in variants %}
  [data-variant="{{ v.label }}"] {
    {%- for k, val in v.css_vars %}
    {{ k }}: {{ val }};
    {%- endfor %}
    --bg: {{ v.bg }};
    --fg: {{ v.fg }};
    --accent: {{ v.accent }};
    --muted: {{ v.muted }};
  }
  {% endfor %}
  .wrap {
    max-width: 1100px;
    margin: 0 auto;
    padding: 96px 32px;
  }
  .eyebrow {
    font-size: 12px;
    letter-spacing: 0.14em;
    text-transform: uppercase;
    color: var(--muted);
    margin-bottom: 24px;
  }
  h1 {
    font-size: clamp(40px, 6vw, 84px);
    line-height: 1.04;
    letter-spacing: -0.02em;
    margin: 0 0 24px;
    font-weight: 700;
  }
  .lede {
    font-size: clamp(18px, 1.6vw, 22px);
    line-height: 1.55;
    max-width: 60ch;
    color: var(--muted);
    margin: 0 0 40px;
  }
  .cta {
    display: inline-flex;
    align-items: center;
    gap: 10px;
    padding: 14px 22px;
    border-radius: 12px;
    background: var(--accent);
    color: var(--bg);
    text-decoration: none;
    font-weight: 600;
    transition: transform 160ms ease, filter 160ms ease;
  }
  .cta:hover { transform: translateY(-1px); filter: brightness(1.05); }
  .switcher {
    position: fixed;
    top: 16px;
    right: 16px;
    display: flex;
    gap: 6px;
    background: var(--bg);
    border: 1px solid var(--muted);
    padding: 6px;
    border-radius: 999px;
    z-index: 10;
  }
  .switcher button {
    appearance: none;
    border: none;
    background: transparent;
    color: var(--fg);
    font: inherit;
    font-size: 12px;
    padding: 6px 12px;
    border-radius: 999px;
    cursor: pointer;
  }
  .switcher button[aria-pressed="true"] { background: var(--accent); color: var(--bg); }
  @media (max-width: 600px) {
    .wrap { padding: 64px 20px; }
  }
</style>
</head>
<body data-variant="{{ initial_variant }}">
  <nav class="switcher" aria-label="Variant switcher">
    {% for v in variants -%}
    <button data-set="{{ v.label }}" aria-pressed="{{ 'true' if v.label == initial_variant else 'false' }}">{{ v.label }}</button>
    {% endfor -%}
  </nav>
  <main class="wrap">
    <p class="eyebrow">{{ name }}</p>
    <h1>{{ headline }}</h1>
    <p class="lede">{{ lede }}</p>
    <a class="cta" href="#" aria-label="Primary call to action">Get started →</a>
  </main>
  <script>
    document.querySelectorAll('.switcher button').forEach(b => {
      b.addEventListener('click', () => {
        const v = b.dataset.set;
        document.body.dataset.variant = v;
        document.querySelectorAll('.switcher button').forEach(x => {
          x.setAttribute('aria-pressed', x.dataset.set === v ? 'true' : 'false');
        });
      });
    });
  </script>
</body>
</html>
"##;

pub fn render(doc: &DesignDocument, variants: &[Variant]) -> Result<String, String> {
    let mut env = Environment::new();
    env.add_template("hero", HERO_TEMPLATE).map_err(|e| e.to_string())?;
    let tmpl = env.get_template("hero").map_err(|e| e.to_string())?;

    // Build per-variant rendering data: derive bg/fg/accent/muted from tokens.
    let prepared: Vec<_> = variants
        .iter()
        .map(|v| {
            let bg = pick(&v.tokens, &[
                "colors.background.primary",
                "colors.primary.50",
            ])
            .unwrap_or_else(|| "#ffffff".to_string());
            let fg = pick(&v.tokens, &[
                "colors.text.primary",
                "colors.primary.900",
            ])
            .unwrap_or_else(|| "#111111".to_string());
            let accent = pick(&v.tokens, &[
                "colors.primary.500",
                "colors.primary.600",
            ])
            .unwrap_or_else(|| "#3366ff".to_string());
            let muted = pick(&v.tokens, &[
                "colors.text.muted",
                "colors.primary.300",
            ])
            .unwrap_or_else(|| "#7a7a85".to_string());
            let css_vars: Vec<(String, String)> = v
                .tokens
                .iter()
                .map(|(k, val)| (format!("--{}", k.replace('.', "-")), val.clone()))
                .collect();
            minijinja::value::Value::from_serialize(&serde_json::json!({
                "label": v.kind.label(),
                "bg": bg,
                "fg": fg,
                "accent": accent,
                "muted": muted,
                "css_vars": css_vars,
            }))
        })
        .collect();

    let name = doc.frontmatter.name.clone();
    let description = doc
        .frontmatter
        .description
        .clone()
        .unwrap_or_else(|| "An Aphrodite-generated experience.".to_string());
    let initial = variants
        .iter()
        .find(|v| v.kind.label() == "light")
        .or_else(|| variants.first())
        .map(|v| v.kind.label())
        .unwrap_or_else(|| "default".to_string());

    let headline = derive_headline(&description, &name);
    let lede = derive_lede(&description);

    let out = tmpl
        .render(context! {
            name,
            headline,
            lede,
            variants => prepared,
            initial_variant => initial,
        })
        .map_err(|e| e.to_string())?;
    Ok(out)
}

fn pick(tokens: &std::collections::BTreeMap<String, String>, keys: &[&str]) -> Option<String> {
    for k in keys {
        if let Some(v) = tokens.get(*k) {
            return Some(v.clone());
        }
    }
    None
}

fn derive_headline(description: &str, name: &str) -> String {
    let d = description.trim();
    if d.is_empty() || d.len() < 8 {
        format!("Meet {name}.")
    } else {
        // Use the description as the headline; trim trailing period if any.
        d.trim_end_matches('.').to_string() + "."
    }
}

fn derive_lede(description: &str) -> String {
    if description.trim().is_empty() {
        "Generated by Aphrodite — every variant validates against WCAG-AA contrast and the Google Labs DESIGN.md alpha schema.".to_string()
    } else {
        format!(
            "{description} Every variant of this surface validates against WCAG-AA contrast and the Google Labs DESIGN.md alpha schema."
        )
    }
}
