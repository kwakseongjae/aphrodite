//! v0.6 design-system handoff. Emits three production-grade artifacts
//! from the resolved DESIGN.md + variants — zero LLM calls, fully
//! deterministic. The point is: a Toss/Karrot designer should be able to
//! open the `components.html` preview, and a frontend engineer should be
//! able to `@import "tokens.css"` (or `import tokens from "tokens.json"`)
//! and have everything wired without re-typing tokens by hand.

use aphrodite_core::variant::Variant;
use std::collections::BTreeMap;

/// Build a `tokens.css` file containing CSS custom properties for every
/// variant. Scoped to `body[data-variant=...]` so a single import covers
/// the entire theme set.
pub fn build_tokens_css(variants: &[Variant]) -> String {
    let mut out = String::from(
        "/* aphrodite tokens — auto-generated. Edit DESIGN.md instead. */\n\n",
    );
    for v in variants {
        out.push_str(&format!("body[data-variant=\"{}\"] {{\n", v.kind.label()));
        for (k, val) in &v.tokens {
            out.push_str(&format!("  --{}: {};\n", k.replace('.', "-"), val));
        }
        out.push_str("}\n\n");
    }
    out
}

/// Build a `tokens.json` file in Style-Dictionary-compatible shape.
/// Nested keys (`colors.primary.500`) become nested objects so SD
/// transformers can pick them up directly.
pub fn build_tokens_json(variants: &[Variant]) -> String {
    let mut root = serde_json::Map::new();
    for v in variants {
        let mut node = serde_json::Map::new();
        for (k, val) in &v.tokens {
            insert_nested(&mut node, k, val);
        }
        root.insert(v.kind.label().to_string(), serde_json::Value::Object(node));
    }
    serde_json::to_string_pretty(&serde_json::Value::Object(root)).unwrap_or_default()
}

fn insert_nested(node: &mut serde_json::Map<String, serde_json::Value>, key: &str, value: &str) {
    let parts: Vec<&str> = key.split('.').collect();
    if parts.is_empty() {
        return;
    }
    let (last, rest) = parts.split_last().unwrap();
    let mut cur = node;
    for p in rest {
        let entry = cur
            .entry(p.to_string())
            .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));
        if !entry.is_object() {
            // collision (e.g. someone declared `colors.primary` AND
            // `colors.primary.500`) — overwrite with object.
            *entry = serde_json::Value::Object(serde_json::Map::new());
        }
        cur = entry.as_object_mut().unwrap();
    }
    cur.insert(
        last.to_string(),
        serde_json::Value::Object(
            [
                ("value".to_string(), serde_json::Value::String(value.to_string())),
                ("type".to_string(), serde_json::Value::String(infer_type(value))),
            ]
            .into_iter()
            .collect(),
        ),
    );
}

fn infer_type(value: &str) -> String {
    let v = value.trim();
    if v.starts_with('#') || v.starts_with("rgb") || v.starts_with("hsl") {
        "color".into()
    } else if v.ends_with("px") || v.ends_with("rem") || v.ends_with("em") {
        "dimension".into()
    } else if v.ends_with("ms") || v.ends_with("s") {
        "duration".into()
    } else {
        "string".into()
    }
}

/// Build a `components.html` preview page — Storybook-style. Shows token
/// swatches, type scale, and primitive components (Button / Input /
/// Card / Tag / Avatar) rendered with the design tokens. Designers
/// inspect this for visual QA; frontend engineers reverse-engineer the
/// HTML snippets directly.
pub fn build_components_html(variants: &[Variant]) -> String {
    let project_name = "Aphrodite Components";
    // Collect every unique color token across variants so the swatches
    // section covers the full palette.
    let mut color_keys: Vec<String> = variants
        .iter()
        .flat_map(|v| v.tokens.keys().cloned())
        .filter(|k| k.starts_with("colors."))
        .collect();
    color_keys.sort();
    color_keys.dedup();

    let initial_variant = variants
        .first()
        .map(|v| v.kind.label().to_string())
        .unwrap_or_else(|| "light".into());

    // The variant-switcher buttons.
    let switcher_buttons: String = variants
        .iter()
        .map(|v| {
            format!(
                r#"<button data-variant="{0}" class="vs-btn">{0}</button>"#,
                v.kind.label()
            )
        })
        .collect::<Vec<_>>()
        .join("");

    // Color swatch grid (one cell per token, picks up the value from
    // the active variant's CSS var).
    let color_swatches: String = color_keys
        .iter()
        .map(|k| {
            let css_var = k.replace('.', "-");
            format!(
                r#"<div class="swatch"><div class="swatch-chip" style="background: var(--{css_var});"></div><div class="swatch-meta"><div class="swatch-name">{k}</div><div class="swatch-var">var(--{css_var})</div></div></div>"#
            )
        })
        .collect();

    // Inline the tokens.css output so the preview page is fully
    // self-contained — designers can email it.
    let tokens_inline = build_tokens_css(variants);

    format!(
        r##"<!doctype html>
<html lang="ko">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{project_name}</title>
  <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Pretendard:wght@400;500;600;700&display=swap">
  <style>
{tokens_inline}
    * {{ box-sizing: border-box; }}
    body {{
      margin: 0;
      font-family: 'Pretendard', Inter, system-ui, sans-serif;
      background: var(--colors-background-primary, #fff);
      color: var(--colors-text-primary, #111);
      line-height: 1.5;
    }}
    .page {{ max-width: 1200px; margin: 0 auto; padding: 32px 24px 96px; }}
    @media (min-width: 768px) {{
      .page {{ padding: 64px 48px 128px; }}
    }}
    h1, h2, h3 {{ margin: 0; }}
    h1 {{ font-size: 36px; font-weight: 700; letter-spacing: -0.02em; }}
    @media (min-width: 768px) {{ h1 {{ font-size: 48px; }} }}
    h2 {{ font-size: 24px; font-weight: 600; margin-top: 48px; margin-bottom: 24px; letter-spacing: -0.01em; }}
    .lede {{ font-size: 16px; color: var(--colors-text-muted, #666); margin-top: 8px; }}
    .switcher {{ display: flex; gap: 4px; padding: 6px; background: var(--colors-background-secondary, #f5f5f5); border: 1px solid var(--colors-border-primary, #e5e5e5); border-radius: 8px; }}
    .vs-btn {{ font: inherit; padding: 6px 12px; border: 1px solid transparent; background: transparent; color: var(--colors-text-primary, #111); border-radius: 6px; cursor: pointer; min-height: 32px; }}
    .vs-btn[aria-pressed="true"] {{ background: var(--colors-primary-500, #111); color: var(--colors-background-primary, #fff); }}
    .top-bar {{ display: flex; justify-content: space-between; align-items: flex-start; flex-wrap: wrap; gap: 16px; }}
    .swatch-grid {{ display: grid; grid-template-columns: 1fr; gap: 12px; }}
    @media (min-width: 600px) {{ .swatch-grid {{ grid-template-columns: 1fr 1fr; }} }}
    @media (min-width: 900px) {{ .swatch-grid {{ grid-template-columns: 1fr 1fr 1fr; }} }}
    .swatch {{ display: flex; align-items: center; gap: 12px; padding: 12px; border: 1px solid var(--colors-border-primary, #e5e5e5); border-radius: 6px; background: var(--colors-background-primary, #fff); }}
    .swatch-chip {{ width: 48px; height: 48px; border-radius: 6px; border: 1px solid rgba(0,0,0,0.05); flex-shrink: 0; }}
    .swatch-meta {{ font-size: 12px; min-width: 0; overflow: hidden; }}
    .swatch-name {{ font-weight: 500; color: var(--colors-text-primary, #111); }}
    .swatch-var {{ color: var(--colors-text-muted, #666); font-family: ui-monospace, SFMono-Regular, monospace; font-size: 11px; }}
    .type-scale > * + * {{ margin-top: 12px; }}
    .components-grid {{ display: grid; grid-template-columns: 1fr; gap: 24px; }}
    @media (min-width: 768px) {{ .components-grid {{ grid-template-columns: 1fr 1fr; }} }}
    .comp-card {{ padding: 24px; border: 1px solid var(--colors-border-primary, #e5e5e5); border-radius: 8px; background: var(--colors-background-primary, #fff); }}
    .comp-card > h3 {{ font-size: 13px; font-weight: 500; color: var(--colors-text-muted, #666); text-transform: uppercase; letter-spacing: 0.06em; margin-bottom: 16px; }}
    .demo-stack > * + * {{ margin-top: 12px; }}
    .demo-row {{ display: flex; gap: 8px; flex-wrap: wrap; align-items: center; }}
    /* Primitive components — Button, Input, Tag, Card, Avatar */
    .btn {{ font: inherit; min-height: 44px; padding: 12px 20px; border-radius: 8px; border: 1px solid transparent; cursor: pointer; font-weight: 600; font-size: 15px; }}
    .btn-primary {{ background: var(--colors-primary-500, #111); color: var(--colors-background-primary, #fff); }}
    .btn-secondary {{ background: transparent; color: var(--colors-text-primary, #111); border-color: var(--colors-border-primary, #e5e5e5); }}
    .btn-ghost {{ background: transparent; color: var(--colors-text-primary, #111); }}
    .input {{ font: inherit; min-height: 44px; padding: 12px 14px; border-radius: 8px; border: 1px solid var(--colors-border-primary, #e5e5e5); background: var(--colors-background-primary, #fff); color: var(--colors-text-primary, #111); width: 100%; }}
    .tag {{ display: inline-flex; align-items: center; padding: 4px 10px; border-radius: 999px; background: var(--colors-primary-50, #f5f5f5); color: var(--colors-primary-700, #111); font-size: 12px; font-weight: 500; }}
    .avatar {{ width: 40px; height: 40px; border-radius: 999px; background: var(--colors-primary-200, #ddd); display: inline-flex; align-items: center; justify-content: center; font-weight: 600; color: var(--colors-text-primary, #111); }}
    .card-demo {{ padding: 16px; border: 1px solid var(--colors-border-primary, #e5e5e5); border-radius: 8px; background: var(--colors-background-primary, #fff); }}
    .card-demo h4 {{ margin: 0 0 8px; font-size: 16px; }}
    .card-demo p {{ margin: 0; color: var(--colors-text-muted, #666); font-size: 14px; }}
    pre.snippet {{ background: var(--colors-background-secondary, #f5f5f5); padding: 12px 14px; border-radius: 6px; font-size: 12px; overflow-x: auto; margin: 12px 0 0; }}
  </style>
</head>
<body data-variant="{initial_variant}">
  <div class="page">
    <div class="top-bar">
      <div>
        <h1>{project_name}</h1>
        <p class="lede">Auto-generated component preview. Tokens are live — switch the variant on the right to inspect each theme.</p>
      </div>
      <nav class="switcher" aria-label="Theme variants">{switcher_buttons}</nav>
    </div>

    <h2>Color tokens</h2>
    <div class="swatch-grid">{color_swatches}</div>

    <h2>Type scale</h2>
    <div class="type-scale">
      <div style="font-size: 48px; font-weight: 700; letter-spacing: -0.02em;">Display 48</div>
      <div style="font-size: 36px; font-weight: 700; letter-spacing: -0.02em;">Headline 36</div>
      <div style="font-size: 24px; font-weight: 600;">Title 24</div>
      <div style="font-size: 18px; font-weight: 500;">Subtitle 18</div>
      <div style="font-size: 16px;">Body 16 — 본문에 한글이 들어왔을 때 가독성을 확인합니다.</div>
      <div style="font-size: 14px; color: var(--colors-text-muted, #666);">Caption 14 — 보조 정보, 메타데이터.</div>
    </div>

    <h2>Components</h2>
    <div class="components-grid">
      <div class="comp-card">
        <h3>Button</h3>
        <div class="demo-stack">
          <div class="demo-row">
            <button class="btn btn-primary">Primary</button>
            <button class="btn btn-secondary">Secondary</button>
            <button class="btn btn-ghost">Ghost</button>
          </div>
          <pre class="snippet">&lt;button class="btn btn-primary"&gt;Primary&lt;/button&gt;</pre>
        </div>
      </div>

      <div class="comp-card">
        <h3>Input</h3>
        <div class="demo-stack">
          <input class="input" type="text" placeholder="이메일을 입력하세요" aria-label="Email">
          <input class="input" type="password" placeholder="비밀번호 (8자 이상)" aria-label="Password">
          <pre class="snippet">&lt;input class="input" placeholder="..."&gt;</pre>
        </div>
      </div>

      <div class="comp-card">
        <h3>Tag</h3>
        <div class="demo-stack">
          <div class="demo-row">
            <span class="tag">New</span>
            <span class="tag">베타</span>
            <span class="tag">Promotion</span>
          </div>
          <pre class="snippet">&lt;span class="tag"&gt;New&lt;/span&gt;</pre>
        </div>
      </div>

      <div class="comp-card">
        <h3>Avatar</h3>
        <div class="demo-stack">
          <div class="demo-row">
            <span class="avatar" aria-label="Hyejin">H</span>
            <span class="avatar" aria-label="Min Jun">MJ</span>
            <span class="avatar" aria-label="Soo Ah">SA</span>
          </div>
          <pre class="snippet">&lt;span class="avatar"&gt;H&lt;/span&gt;</pre>
        </div>
      </div>

      <div class="comp-card" style="grid-column: 1 / -1;">
        <h3>Card</h3>
        <div class="demo-stack">
          <div class="card-demo">
            <h4>월 정산 내역</h4>
            <p>2026년 5월 1일 ~ 5월 31일</p>
          </div>
          <pre class="snippet">&lt;div class="card-demo"&gt;...&lt;/div&gt;</pre>
        </div>
      </div>
    </div>
  </div>
  <script>
    (function() {{
      var buttons = document.querySelectorAll('.vs-btn');
      function applyVariant(name) {{
        document.body.setAttribute('data-variant', name);
        buttons.forEach(function(b) {{
          b.setAttribute('aria-pressed', b.getAttribute('data-variant') === name ? 'true' : 'false');
        }});
      }}
      buttons.forEach(function(b) {{
        b.addEventListener('click', function() {{ applyVariant(b.getAttribute('data-variant')); }});
      }});
      applyVariant(document.body.getAttribute('data-variant') || buttons[0] && buttons[0].getAttribute('data-variant'));
    }})();
  </script>
</body>
</html>
"##,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use aphrodite_core::variant::{Variant, VariantKind};

    fn fixture_variants() -> Vec<Variant> {
        let mut t1 = BTreeMap::new();
        t1.insert("colors.primary.500".into(), "#0070f3".into());
        t1.insert("colors.background.primary".into(), "#ffffff".into());
        t1.insert("spacing.4".into(), "16px".into());
        let mut t2 = BTreeMap::new();
        t2.insert("colors.primary.500".into(), "#3b82f6".into());
        t2.insert("colors.background.primary".into(), "#0b0b0b".into());
        vec![
            Variant { kind: VariantKind::Light, tokens: t1 },
            Variant { kind: VariantKind::Dark, tokens: t2 },
        ]
    }

    #[test]
    fn tokens_css_scopes_to_body_data_variant() {
        let css = build_tokens_css(&fixture_variants());
        assert!(css.contains(r#"body[data-variant="light"]"#));
        assert!(css.contains(r#"body[data-variant="dark"]"#));
        assert!(css.contains("--colors-primary-500: #0070f3"));
    }

    #[test]
    fn tokens_json_nests_dotted_keys_and_tags_type() {
        let json = build_tokens_json(&fixture_variants());
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        let primary500 = &v["light"]["colors"]["primary"]["500"];
        assert_eq!(primary500["value"], "#0070f3");
        assert_eq!(primary500["type"], "color");
        let space = &v["light"]["spacing"]["4"];
        assert_eq!(space["value"], "16px");
        assert_eq!(space["type"], "dimension");
    }

    #[test]
    fn components_html_contains_all_variants_and_swatches() {
        let html = build_components_html(&fixture_variants());
        assert!(html.contains(r#"data-variant="light""#));
        assert!(html.contains(r#"data-variant="dark""#));
        assert!(html.contains("Color tokens"));
        assert!(html.contains("Type scale"));
        assert!(html.contains("Components"));
        assert!(html.contains("var(--colors-primary-500)"));
        // mobile-first responsive present
        assert!(html.contains("@media (min-width: 768px)"));
    }
}
