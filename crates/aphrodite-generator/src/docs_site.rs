//! v1.0 RC.6 docs site. Generates `docs/index.html` — a Material-UI /
//! Mantine-style single-page documentation site for the emitted React
//! component package. Designers / FE engineers landing on it see:
//!
//!   - Hero (project name + tagline + GitHub-style status badges)
//!   - Getting started (npm install / import / styles.css copy-paste)
//!   - Color token swatches + type scale (live from the resolved
//!     variants — 4-variant switcher works in the docs page itself)
//!   - One section per component: name, short description, live demo,
//!     copy-pasteable React snippet
//!   - Sticky left TOC that scrolls into each section
//!
//! Zero LLM cost, deterministic, written entirely from the resolved
//! Vec<Variant> + a static component-catalog table inside this module.

use aphrodite_core::variant::Variant;

#[derive(Clone, Copy)]
struct ComponentDocEntry {
    name: &'static str,
    /// Korean tagline shown under the heading.
    tagline: &'static str,
    /// HTML demo body (uses the same aph-* classes from styles.css).
    demo_html: &'static str,
    /// Plain-text JSX snippet shown in the copy-able code box.
    snippet: &'static str,
}

/// Build the single-file docs site. `project_name` is taken from
/// DESIGN.md `name:` field.
pub fn build_docs_index(variants: &[Variant], project_name: &str) -> String {
    let kebab = slug(project_name);
    let initial_variant = variants
        .first()
        .map(|v| v.kind.label())
        .unwrap_or_else(|| "light".to_string());

    let variant_buttons: String = variants
        .iter()
        .map(|v| format!(r#"<button data-variant="{0}" class="vs-btn">{0}</button>"#, v.kind.label()))
        .collect::<Vec<_>>()
        .join("");

    let color_keys: Vec<String> = {
        let mut keys: Vec<String> = variants
            .iter()
            .flat_map(|v| v.tokens.keys().cloned())
            .filter(|k| k.starts_with("colors."))
            .collect();
        keys.sort();
        keys.dedup();
        keys
    };
    let color_swatches: String = color_keys
        .iter()
        .map(|k| {
            let css_var = k.replace('.', "-");
            format!(r#"<div class="swatch"><div class="swatch-chip" style="background: var(--{css_var});"></div><code>{k}</code></div>"#)
        })
        .collect();

    let toc: String = catalog()
        .iter()
        .map(|c| format!(r##"<a href="#c-{name}">{name}</a>"##, name = c.name))
        .collect::<Vec<_>>()
        .join("");

    let component_sections: String = catalog()
        .iter()
        .map(|c| {
            let escaped_snippet = escape_html(c.snippet);
            format!(
                r##"<section class="comp" id="c-{name}">
  <div class="comp-head">
    <h3>{name}</h3>
    <p class="comp-tag">{tagline}</p>
  </div>
  <div class="comp-body">
    <div class="comp-demo">{demo}</div>
    <pre class="comp-snippet"><code>{snippet}</code></pre>
  </div>
</section>"##,
                name = c.name,
                tagline = c.tagline,
                demo = c.demo_html,
                snippet = escaped_snippet,
            )
        })
        .collect();

    let tokens_inline = crate::design_system::build_tokens_css(variants);

    format!(
        r##"<!doctype html>
<html lang="ko">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{project_name} — Aphrodite docs</title>
  <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Pretendard:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap">
  <style>
{tokens_inline}
    * {{ box-sizing: border-box; }}
    body {{
      margin: 0;
      font-family: 'Pretendard', Inter, system-ui, sans-serif;
      background: var(--colors-background-primary, #fff);
      color: var(--colors-text-primary, #111);
      line-height: 1.55;
      word-break: keep-all;
      overflow-wrap: anywhere;
    }}
    code {{ font-family: 'JetBrains Mono', ui-monospace, SFMono-Regular, monospace; font-size: 12px; }}
    a {{ color: var(--colors-primary-500); text-decoration: none; }}
    a:hover {{ text-decoration: underline; }}

    .layout {{ display: grid; grid-template-columns: 1fr; min-height: 100vh; }}
    @media (min-width: 900px) {{ .layout {{ grid-template-columns: 240px 1fr; }} }}

    .sidebar {{
      padding: 24px 20px; border-bottom: 1px solid var(--colors-border-primary);
      background: var(--colors-background-secondary, #f5f5f5);
    }}
    @media (min-width: 900px) {{
      .sidebar {{ position: sticky; top: 0; height: 100vh; overflow: auto; border-bottom: none; border-right: 1px solid var(--colors-border-primary); }}
    }}
    .sidebar h1 {{ font-size: 18px; margin: 0 0 4px; letter-spacing: -0.01em; }}
    .sidebar .pkg {{ font-size: 12px; color: var(--colors-text-muted); margin-bottom: 16px; }}
    .sidebar nav {{ display: flex; flex-direction: column; gap: 4px; font-size: 13px; }}
    .sidebar nav a {{ padding: 4px 8px; border-radius: 4px; color: var(--colors-text-primary); }}
    .sidebar nav a:hover {{ background: var(--colors-background-primary); text-decoration: none; }}
    .sidebar h4 {{ font-size: 11px; text-transform: uppercase; letter-spacing: 0.06em; color: var(--colors-text-muted); margin: 16px 0 8px; }}

    main {{ padding: 32px 20px 96px; max-width: 960px; }}
    @media (min-width: 900px) {{ main {{ padding: 48px 48px 96px; }} }}
    main > section + section {{ margin-top: 48px; }}

    .hero {{ display: flex; flex-direction: column; gap: 12px; padding: 32px 0 12px; }}
    .hero h2 {{ font-size: 32px; font-weight: 700; letter-spacing: -0.02em; margin: 0; }}
    @media (min-width: 768px) {{ .hero h2 {{ font-size: 44px; }} }}
    .hero p {{ font-size: 16px; color: var(--colors-text-muted); margin: 0; max-width: 60ch; }}

    .badge-row {{ display: flex; gap: 6px; flex-wrap: wrap; margin-top: 8px; }}
    .badge {{ display: inline-flex; padding: 4px 10px; border-radius: 999px; font-size: 12px; background: var(--colors-primary-50, #f5f5f5); color: var(--colors-primary-700, #111); font-weight: 500; }}

    .vs-bar {{ display: flex; gap: 4px; padding: 6px; background: var(--colors-background-secondary, #f5f5f5); border: 1px solid var(--colors-border-primary, #e5e5e5); border-radius: 8px; align-self: flex-start; }}
    .vs-btn {{ font: inherit; padding: 6px 12px; border: 1px solid transparent; background: transparent; color: var(--colors-text-primary, #111); border-radius: 6px; cursor: pointer; min-height: 32px; }}
    .vs-btn[aria-pressed="true"] {{ background: var(--colors-primary-500); color: var(--colors-background-primary); }}

    h2.section-title {{ font-size: 24px; font-weight: 700; margin: 0 0 16px; letter-spacing: -0.01em; }}

    pre.snippet, pre.comp-snippet {{
      background: var(--colors-background-secondary, #f5f5f5); border: 1px solid var(--colors-border-primary, #e5e5e5);
      padding: 14px 16px; border-radius: 8px; font-size: 12px; overflow-x: auto; margin: 12px 0 0;
    }}

    .swatch-grid {{ display: grid; grid-template-columns: 1fr 1fr; gap: 8px; }}
    @media (min-width: 900px) {{ .swatch-grid {{ grid-template-columns: repeat(3, 1fr); }} }}
    .swatch {{ display: flex; align-items: center; gap: 10px; padding: 8px 12px; border: 1px solid var(--colors-border-primary); border-radius: 6px; }}
    .swatch-chip {{ width: 28px; height: 28px; border-radius: 4px; flex-shrink: 0; border: 1px solid rgba(0,0,0,0.06); }}
    .swatch code {{ font-size: 11px; color: var(--colors-text-muted); }}

    .type-scale > * + * {{ margin-top: 8px; }}

    section.comp {{ border-top: 1px solid var(--colors-border-primary); padding-top: 24px; }}
    .comp-head h3 {{ font-size: 22px; margin: 0; }}
    .comp-tag {{ font-size: 14px; color: var(--colors-text-muted); margin: 4px 0 16px; }}
    .comp-body {{ display: grid; grid-template-columns: 1fr; gap: 12px; }}
    @media (min-width: 768px) {{ .comp-body {{ grid-template-columns: 3fr 2fr; align-items: start; }} }}
    .comp-demo {{ padding: 24px; border: 1px solid var(--colors-border-primary); border-radius: 8px; background: var(--colors-background-primary); display: flex; flex-wrap: wrap; gap: 12px; align-items: center; }}

    {EXTRA_COMPONENT_CSS}
  </style>
</head>
<body data-variant="{initial_variant}">
  <div class="layout">
    <aside class="sidebar">
      <h1>{project_name}</h1>
      <div class="pkg"><code>@aphrodite/{kebab}</code></div>
      <h4>Tokens</h4>
      <nav>
        <a href="#tokens">Color tokens</a>
        <a href="#types">Type scale</a>
        <a href="#install">Install</a>
      </nav>
      <h4>Components</h4>
      <nav>{toc}</nav>
    </aside>
    <main>
      <section class="hero">
        <h2>{project_name} 컴포넌트 라이브러리</h2>
        <p>Aphrodite가 DESIGN.md에서 자동 생성한 42개의 TypeScript React 컴포넌트.
        토큰 · ARIA · Storybook · npm 발행 워크플로우까지 포함합니다.</p>
        <div class="badge-row">
          <span class="badge">42 components</span>
          <span class="badge">4 variants</span>
          <span class="badge">TypeScript</span>
          <span class="badge">Storybook CSF3</span>
        </div>
        <nav class="vs-bar" aria-label="Theme variants" style="margin-top:16px;">{variant_buttons}</nav>
      </section>

      <section id="install">
        <h2 class="section-title">설치</h2>
        <pre class="snippet"><code>npm i @aphrodite/{kebab}
# or
pnpm add @aphrodite/{kebab}</code></pre>
        <pre class="snippet"><code>// app entry
import "@aphrodite/{kebab}/styles.css";
import {{ Button, Card, Input }} from "@aphrodite/{kebab}";</code></pre>
      </section>

      <section id="tokens">
        <h2 class="section-title">Color tokens</h2>
        <div class="swatch-grid">{color_swatches}</div>
      </section>

      <section id="types">
        <h2 class="section-title">Type scale</h2>
        <div class="type-scale">
          <div style="font-size: 44px; font-weight: 700; letter-spacing: -0.02em;">Display 44</div>
          <div style="font-size: 32px; font-weight: 700;">Headline 32</div>
          <div style="font-size: 22px; font-weight: 600;">Title 22</div>
          <div style="font-size: 18px; font-weight: 500;">Subtitle 18</div>
          <div style="font-size: 16px;">Body 16 — 한글 본문 가독성을 확인합니다.</div>
          <div style="font-size: 13px; color: var(--colors-text-muted);">Caption 13 — 보조 정보, 메타데이터.</div>
        </div>
      </section>

      <h2 class="section-title" style="margin-top:48px;">Components</h2>
      {component_sections}
    </main>
  </div>
  <script>
    (function () {{
      var btns = document.querySelectorAll('.vs-btn');
      function apply(v) {{
        document.body.setAttribute('data-variant', v);
        btns.forEach(function (b) {{ b.setAttribute('aria-pressed', b.getAttribute('data-variant') === v ? 'true' : 'false'); }});
      }}
      btns.forEach(function (b) {{ b.addEventListener('click', function () {{ apply(b.getAttribute('data-variant')); }}); }});
      apply(document.body.getAttribute('data-variant') || (btns[0] && btns[0].getAttribute('data-variant')));
    }})();
  </script>
</body>
</html>
"##,
        toc = toc,
    )
}

fn slug(s: &str) -> String {
    s.trim()
        .to_ascii_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

const EXTRA_COMPONENT_CSS: &str = r#"
/* Re-use the same component primitive classes from the React package so
   the docs demos render with the real CSS. We inline a compact subset
   here — the published styles.css is the source of truth at runtime. */
.aph-btn { font: inherit; min-height: 44px; padding: 12px 20px; border-radius: 8px; border: 1px solid transparent; cursor: pointer; font-weight: 600; font-size: 15px; display: inline-flex; align-items: center; justify-content: center; gap: 8px; }
.aph-btn--primary { background: var(--colors-primary-500); color: var(--colors-background-primary, #fff); }
.aph-btn--secondary { background: transparent; color: var(--colors-text-primary); border-color: var(--colors-border-primary); }
.aph-btn--ghost { background: transparent; color: var(--colors-text-primary); }
.aph-btn--danger { background: var(--colors-danger-500, #dc2626); color: #fff; }
.aph-input { font: inherit; min-height: 44px; padding: 12px 14px; border-radius: 8px; border: 1px solid var(--colors-border-primary); background: var(--colors-background-primary); color: var(--colors-text-primary); width: 100%; max-width: 320px; }
.aph-tag { display: inline-flex; align-items: center; padding: 4px 10px; border-radius: 999px; background: var(--colors-primary-50, #f5f5f5); color: var(--colors-primary-700, #111); font-size: 12px; font-weight: 500; }
.aph-tag--success { background: var(--colors-success-50, #ecfdf5); color: var(--colors-success-700, #047857); }
.aph-tag--warning { background: var(--colors-warning-50, #fffbeb); color: var(--colors-warning-700, #b45309); }
.aph-tag--danger { background: var(--colors-danger-50, #fef2f2); color: var(--colors-danger-700, #b91c1c); }
.aph-avatar { width: 40px; height: 40px; border-radius: 999px; background: var(--colors-primary-200, #ddd); display: inline-flex; align-items: center; justify-content: center; font-weight: 600; }
.aph-badge { display: inline-flex; align-items: center; min-width: 18px; height: 18px; padding: 0 6px; border-radius: 999px; background: var(--colors-danger-500, #dc2626); color: #fff; font-size: 11px; font-weight: 600; }
.aph-skeleton { background: var(--colors-background-secondary); border-radius: 6px; display: inline-block; }
.aph-spinner { width: 20px; height: 20px; border: 2px solid var(--colors-border-primary); border-top-color: var(--colors-primary-500); border-radius: 999px; animation: aph-spin 0.8s linear infinite; display: inline-block; }
@keyframes aph-spin { to { transform: rotate(360deg); } }
.aph-switch { position: relative; width: 44px; height: 24px; background: var(--colors-primary-500); border-radius: 999px; border: none; padding: 0; }
.aph-switch::after { content: ""; position: absolute; top: 2px; left: 22px; width: 20px; height: 20px; background: #fff; border-radius: 999px; }
.aph-segmented { display: inline-flex; padding: 4px; background: var(--colors-background-secondary); border-radius: 8px; gap: 2px; }
.aph-segmented__btn { padding: 8px 14px; border: none; background: transparent; border-radius: 6px; font: inherit; min-height: 36px; }
.aph-segmented__btn[aria-pressed="true"] { background: var(--colors-background-primary); }
.aph-stat__value { font-size: 24px; font-weight: 700; }
.aph-stat__delta--up { color: var(--colors-success-600, #16a34a); font-weight: 500; }
.aph-alert { padding: 12px 14px; border-radius: 8px; border: 1px solid var(--colors-primary-300, #93c5fd); background: var(--colors-primary-50, #eff6ff); color: var(--colors-primary-900, #1e3a8a); max-width: 360px; }
.aph-progress { width: 200px; height: 8px; background: var(--colors-border-primary); border-radius: 999px; overflow: hidden; }
.aph-progress__bar { height: 100%; background: var(--colors-primary-500); width: 60%; }
"#;

/// Static catalog of the 42 components with a Korean tagline, demo
/// HTML, and JSX snippet. Order matters — drives the TOC + section
/// layout.
fn catalog() -> &'static [ComponentDocEntry] {
    &[
        ComponentDocEntry { name: "Button", tagline: "기본 인터랙션 — 4 variant × 3 size × loading 상태.", demo_html: r#"<button class="aph-btn aph-btn--primary">시작하기</button><button class="aph-btn aph-btn--secondary">취소</button><button class="aph-btn aph-btn--ghost">더 보기</button><button class="aph-btn aph-btn--danger">탈퇴</button>"#, snippet: "<Button variant=\"primary\">시작하기</Button>\n<Button variant=\"secondary\">취소</Button>" },
        ComponentDocEntry { name: "Input", tagline: "텍스트 입력 — error 상태 + aria-invalid 자동 연결.", demo_html: r#"<input class="aph-input" placeholder="이메일을 입력해주세요" />"#, snippet: "<Input placeholder=\"이메일을 입력해주세요\" />\n<Input error placeholder=\"...\" />" },
        ComponentDocEntry { name: "Textarea", tagline: "여러 줄 입력 — resize 세로만.", demo_html: r#"<textarea class="aph-input" rows="3" placeholder="내용을 입력해주세요" style="min-height:96px;"></textarea>"#, snippet: "<Textarea placeholder=\"내용을 입력해주세요\" />" },
        ComponentDocEntry { name: "Select", tagline: "단일 선택 드롭다운.", demo_html: r#"<select class="aph-input" style="max-width:200px;"><option>대한민국 (+82)</option><option>일본 (+81)</option></select>"#, snippet: "<Select>\n  <option>대한민국</option>\n  <option>일본</option>\n</Select>" },
        ComponentDocEntry { name: "Checkbox", tagline: "다중 선택 + label 자동 wrap.", demo_html: r#"<label style="display:inline-flex;gap:8px;align-items:center;"><input type="checkbox" checked /> 이용약관에 동의합니다</label>"#, snippet: "<Checkbox label=\"이용약관에 동의합니다\" />" },
        ComponentDocEntry { name: "Radio", tagline: "단일 선택.", demo_html: r#"<label style="display:inline-flex;gap:8px;align-items:center;"><input type="radio" name="x" checked /> 개인</label>"#, snippet: "<Radio value=\"personal\" label=\"개인\" />" },
        ComponentDocEntry { name: "RadioGroup", tagline: "Radio 묶음 — name + value/onChange.", demo_html: r#"<div style="display:flex;gap:16px;"><label><input type="radio" name="g" checked /> 개인</label><label><input type="radio" name="g" /> 사업자</label></div>"#, snippet: "<RadioGroup name=\"membership\" value={v} onChange={setV}>\n  <Radio value=\"personal\" label=\"개인\" />\n  <Radio value=\"business\" label=\"사업자\" />\n</RadioGroup>" },
        ComponentDocEntry { name: "Switch", tagline: "ON/OFF 토글 — role=switch + aria-checked.", demo_html: r#"<button class="aph-switch" role="switch" aria-checked="true" aria-label="알림"></button>"#, snippet: "<Switch checked={on} onChange={setOn} label=\"알림\" />" },
        ComponentDocEntry { name: "SegmentedControl", tagline: "필터 탭 — 토스 스타일 pill.", demo_html: r#"<div class="aph-segmented"><button class="aph-segmented__btn">전체</button><button class="aph-segmented__btn" aria-pressed="true">진행 중</button><button class="aph-segmented__btn">완료</button></div>"#, snippet: "<SegmentedControl value={v} onChange={setV} options={[{value:'all',label:'전체'}, {value:'active',label:'진행 중'}]} />" },
        ComponentDocEntry { name: "Tag", tagline: "라벨 — neutral / success / warning / danger.", demo_html: r#"<span class="aph-tag">신규</span> <span class="aph-tag aph-tag--success">완료</span> <span class="aph-tag aph-tag--warning">대기</span> <span class="aph-tag aph-tag--danger">실패</span>"#, snippet: "<Tag tone=\"success\">완료</Tag>" },
        ComponentDocEntry { name: "Badge", tagline: "카운트 + dot 변형. 알림 뱃지에 적합.", demo_html: r#"<span class="aph-badge">3</span> <span class="aph-badge">99+</span>"#, snippet: "<Badge count={3} />\n<Badge count={124} max={99} />\n<Badge dot />" },
        ComponentDocEntry { name: "Avatar", tagline: "프로필 — initials 또는 src, 3 size.", demo_html: r#"<span class="aph-avatar">JK</span>"#, snippet: "<Avatar initials=\"JK\" alt=\"Jihyo Kim\" />" },
        ComponentDocEntry { name: "Card", tagline: "콘텐츠 컨테이너 — padded 옵션.", demo_html: r#"<div style="padding:16px;border:1px solid var(--colors-border-primary);border-radius:8px;max-width:280px;"><strong>월 정산 내역</strong><div style="font-size:13px;color:var(--colors-text-muted);">2026년 5월 1일 ~ 5월 31일</div></div>"#, snippet: "<Card>\n  <h4>월 정산 내역</h4>\n</Card>" },
        ComponentDocEntry { name: "Stat", tagline: "KPI 표시 — 라벨 + 값 + 증감 indicator.", demo_html: r#"<div><div style="font-size:13px;color:var(--colors-text-muted);">월 거래액</div><div class="aph-stat__value">₩ 12,456,789</div><div class="aph-stat__delta--up">▲ +8.4%</div></div>"#, snippet: "<Stat label=\"월 거래액\" value=\"₩ 12,456,789\" delta={{ value: '+8.4%', direction: 'up' }} />" },
        ComponentDocEntry { name: "Alert", tagline: "지속 표시 배너 — Toast과 다름. 4 tone.", demo_html: r#"<div class="aph-alert"><strong>안내</strong><div>본인 인증이 필요합니다.</div></div>"#, snippet: "<Alert tone=\"info\" title=\"안내\">본인 인증이 필요합니다.</Alert>" },
        ComponentDocEntry { name: "Toast", tagline: "임시 알림 — ToastProvider + useToast 훅.", demo_html: r#"<div style="background:var(--colors-text-primary);color:var(--colors-background-primary);padding:12px 18px;border-radius:8px;">저장되었습니다</div>"#, snippet: "const { push } = useToast();\npush({ message: '저장되었습니다', tone: 'success' });" },
        ComponentDocEntry { name: "Tooltip", tagline: "마우스 오버 도움말 — role=tooltip.", demo_html: r#"<span style="position:relative;"><button class="aph-btn aph-btn--secondary">저장</button><span style="position:absolute;bottom:calc(100% + 6px);left:0;background:var(--colors-text-primary);color:var(--colors-background-primary);padding:6px 10px;border-radius:6px;font-size:12px;">⌘S</span></span>"#, snippet: "<Tooltip content=\"⌘S\"><Button>저장</Button></Tooltip>" },
        ComponentDocEntry { name: "Popover", tagline: "클릭 → 열림. 외부 클릭으로 닫힘.", demo_html: r#"<button class="aph-btn aph-btn--secondary">필터 ▾</button>"#, snippet: "<Popover trigger={<Button>필터</Button>}>\n  <Filters />\n</Popover>" },
        ComponentDocEntry { name: "HoverCard", tagline: "호버 → 카드 노출. 프로필 미리보기 패턴.", demo_html: r#"<span class="aph-avatar">JK</span>"#, snippet: "<HoverCard trigger={<Avatar initials=\"JK\" />}>\n  <ProfileMini />\n</HoverCard>" },
        ComponentDocEntry { name: "Menu", tagline: "role=menu + MenuItem.", demo_html: r#"<div style="background:var(--colors-background-primary);border:1px solid var(--colors-border-primary);border-radius:8px;padding:4px;min-width:160px;"><button class="aph-btn aph-btn--ghost" style="width:100%;justify-content:flex-start;min-height:36px;padding:6px 10px;">프로필</button><button class="aph-btn aph-btn--ghost" style="width:100%;justify-content:flex-start;min-height:36px;padding:6px 10px;">로그아웃</button></div>"#, snippet: "<Menu ariaLabel=\"사용자 메뉴\">\n  <MenuItem>프로필</MenuItem>\n  <MenuItem>로그아웃</MenuItem>\n</Menu>" },
        ComponentDocEntry { name: "Tabs", tagline: "탭 + 패널 — 전체 ARIA tablist 와이어링.", demo_html: r#"<div style="border-bottom:1px solid var(--colors-border-primary);"><button class="aph-btn aph-btn--ghost" style="border-bottom:2px solid var(--colors-primary-500);border-radius:0;font-weight:600;">계정</button><button class="aph-btn aph-btn--ghost" style="border-radius:0;color:var(--colors-text-muted);">알림</button></div>"#, snippet: "<Tabs defaultValue=\"account\">\n  <TabList>\n    <Tab value=\"account\">계정</Tab>\n  </TabList>\n  <TabPanel value=\"account\">...</TabPanel>\n</Tabs>" },
        ComponentDocEntry { name: "Accordion", tagline: "FAQ / 접힘 콘텐츠.", demo_html: r#"<div style="border-bottom:1px solid var(--colors-border-primary);padding:16px 0;display:flex;justify-content:space-between;width:280px;"><span style="font-weight:500;">환불 정책이 어떻게 되나요?</span><span>▾</span></div>"#, snippet: "<Accordion>\n  <AccordionItem title=\"환불 정책이 어떻게 되나요?\">7일 이내 전액 환불</AccordionItem>\n</Accordion>" },
        ComponentDocEntry { name: "Breadcrumb", tagline: "경로 표시.", demo_html: r#"<nav style="display:flex;gap:6px;font-size:13px;color:var(--colors-text-muted);"><a>홈</a><span>/</span><a>투자</a><span>/</span><span style="color:var(--colors-text-primary);font-weight:500;">삼성전자</span></nav>"#, snippet: "<Breadcrumb items={[{label:'홈', href:'/'}, {label:'투자', href:'/invest'}, {label:'삼성전자'}]} />" },
        ComponentDocEntry { name: "Pagination", tagline: "페이지 네비게이션.", demo_html: r#"<div style="display:flex;gap:4px;"><button class="aph-btn aph-btn--secondary" style="min-width:36px;min-height:36px;padding:6px 10px;">‹</button><button class="aph-btn aph-btn--primary" style="min-width:36px;min-height:36px;padding:6px 10px;">3</button><button class="aph-btn aph-btn--secondary" style="min-width:36px;min-height:36px;padding:6px 10px;">4</button><button class="aph-btn aph-btn--secondary" style="min-width:36px;min-height:36px;padding:6px 10px;">›</button></div>"#, snippet: "<Pagination page={p} pageCount={20} onChange={setP} />" },
        ComponentDocEntry { name: "Modal", tagline: "중앙 다이얼로그 — Escape 닫기 + aria-modal.", demo_html: r#"<button class="aph-btn aph-btn--primary">모달 열기</button>"#, snippet: "<Modal open={open} onClose={close} title=\"확인\">\n  저장하시겠습니까?\n</Modal>" },
        ComponentDocEntry { name: "Drawer", tagline: "우측 패널 — 데스크톱 멀티콘텐츠.", demo_html: r#"<button class="aph-btn aph-btn--primary">드로어 열기</button>"#, snippet: "<Drawer open={open} onClose={close}>\n  <Settings />\n</Drawer>" },
        ComponentDocEntry { name: "Sheet", tagline: "바텀시트 — 모바일 우선.", demo_html: r#"<button class="aph-btn aph-btn--primary">바텀시트 열기</button>"#, snippet: "<Sheet open={open} onClose={close} title=\"계좌 선택\">\n  ...\n</Sheet>" },
        ComponentDocEntry { name: "PinInput", tagline: "OTP / 본인 인증 6자리 입력.", demo_html: r#"<div style="display:flex;gap:8px;">{1,2,3,4,5,6}.map ··· — pinput</div>"#, snippet: "<PinInput length={6} value={v} onChange={setV} />" },
        ComponentDocEntry { name: "NumberInput", tagline: "수량/금액 입력 — +/- 버튼.", demo_html: r#"<div style="display:inline-flex;border:1px solid var(--colors-border-primary);border-radius:8px;padding:4px;"><button class="aph-btn aph-btn--ghost" style="min-height:32px;width:32px;padding:0;">−</button><input class="aph-input" style="width:64px;text-align:center;border:none;min-height:32px;" value="2" /><button class="aph-btn aph-btn--ghost" style="min-height:32px;width:32px;padding:0;">+</button></div>"#, snippet: "<NumberInput value={qty} onChange={setQty} min={0} max={99} />" },
        ComponentDocEntry { name: "SearchInput", tagline: "검색 — clear 버튼 포함.", demo_html: r#"<input class="aph-input" type="search" placeholder="상품, 브랜드 검색" />"#, snippet: "<SearchInput value={q} onChange={setQ} placeholder=\"상품 검색\" />" },
        ComponentDocEntry { name: "FileUploader", tagline: "드래그앤드롭 — 키보드 fallback.", demo_html: r#"<div style="border:2px dashed var(--colors-border-primary);border-radius:12px;padding:32px 24px;text-align:center;color:var(--colors-text-muted);max-width:360px;">📎  파일을 끌어다 놓거나 클릭하여 업로드</div>"#, snippet: "<FileUploader multiple accept=\"image/*\" onFiles={handleFiles} />" },
        ComponentDocEntry { name: "DatePicker", tagline: "HTML5 date input — locale 자동.", demo_html: r#"<input class="aph-input" type="date" value="2026-05-18" style="max-width:200px;" />"#, snippet: "<DatePicker defaultValue=\"2026-05-18\" />" },
        ComponentDocEntry { name: "Combobox", tagline: "자동완성 — 한글 검색 키 지원.", demo_html: r#"<input class="aph-input" type="text" value="서울특별시" style="max-width:240px;" />"#, snippet: "<Combobox value={v} onChange={setV} options={options} placeholder=\"도시 검색\" />" },
        ComponentDocEntry { name: "Slider", tagline: "범위 입력 — accent-color 자동.", demo_html: r#"<input type="range" min="0" max="100" value="30" style="width:200px;accent-color:var(--colors-primary-500);" />"#, snippet: "<Slider min={0} max={100} defaultValue={30} />" },
        ComponentDocEntry { name: "ProgressBar", tagline: "진행률 — role=progressbar.", demo_html: r#"<div class="aph-progress"><div class="aph-progress__bar"></div></div>"#, snippet: "<ProgressBar value={60} max={100} label=\"업로드\" />" },
        ComponentDocEntry { name: "Stepper", tagline: "다단계 진행 — 본인 인증 → 계좌 등록 → 완료.", demo_html: r#"<div style="display:flex;align-items:center;gap:12px;"><span style="display:flex;gap:6px;align-items:center;"><span class="aph-avatar" style="width:24px;height:24px;background:var(--colors-primary-500);color:#fff;font-size:13px;">✓</span>본인 인증</span><span style="flex:1;width:40px;height:2px;background:var(--colors-border-primary);"></span><span style="display:flex;gap:6px;align-items:center;font-weight:600;"><span class="aph-avatar" style="width:24px;height:24px;background:var(--colors-primary-500);color:#fff;font-size:13px;">2</span>계좌 등록</span></div>"#, snippet: "<Stepper current={1} steps={[{label:'본인 인증'},{label:'계좌 등록'},{label:'완료'}]} />" },
        ComponentDocEntry { name: "Toolbar", tagline: "role=toolbar — 편집기 등.", demo_html: r#"<div style="display:flex;gap:4px;padding:6px;border:1px solid var(--colors-border-primary);border-radius:8px;"><button class="aph-btn aph-btn--ghost" style="min-height:32px;padding:6px 10px;">B</button><button class="aph-btn aph-btn--ghost" style="min-height:32px;padding:6px 10px;">I</button></div>"#, snippet: "<Toolbar ariaLabel=\"편집기\">\n  <Button variant=\"ghost\">굵게</Button>\n  <Toolbar.Separator />\n</Toolbar>" },
        ComponentDocEntry { name: "FormField", tagline: "label + hint + error 자동 ARIA 연결.", demo_html: r#"<div style="display:flex;flex-direction:column;gap:6px;max-width:280px;"><label style="font-size:13px;font-weight:500;">이메일</label><input class="aph-input" placeholder="name@toss.im" /><span style="font-size:12px;color:var(--colors-text-muted);">회사 이메일을 사용해주세요</span></div>"#, snippet: "<FormField label=\"이메일\" hint=\"회사 이메일\">\n  <Input placeholder=\"name@toss.im\" />\n</FormField>" },
        ComponentDocEntry { name: "Skeleton", tagline: "로딩 placeholder — shimmer.", demo_html: r#"<div style="display:flex;flex-direction:column;gap:8px;width:200px;"><span class="aph-skeleton" style="height:16px;width:80%;"></span><span class="aph-skeleton" style="height:16px;width:60%;"></span></div>"#, snippet: "<Skeleton width={240} height={16} />\n<Skeleton width={48} height={48} circle />" },
        ComponentDocEntry { name: "Spinner", tagline: "로딩 인디케이터 — role=status.", demo_html: r#"<span class="aph-spinner"></span>"#, snippet: "<Spinner size=\"sm\" />\n<Spinner label=\"불러오는 중\" />" },
        ComponentDocEntry { name: "Divider", tagline: "수평/수직 구분선.", demo_html: r#"<hr style="border:none;border-top:1px solid var(--colors-border-primary);width:200px;margin:8px 0;" />"#, snippet: "<Divider />\n<Divider orientation=\"vertical\" />" },
        ComponentDocEntry { name: "EmptyState", tagline: "결과 없음 / 빈 상태.", demo_html: r#"<div style="text-align:center;color:var(--colors-text-muted);padding:24px;"><div style="font-weight:600;font-size:16px;color:var(--colors-text-primary);">거래 내역이 없습니다</div><div style="font-size:13px;margin-top:4px;">이번 달은 아직 거래가 없네요.</div></div>"#, snippet: "<EmptyState title=\"거래 내역이 없습니다\" description=\"이번 달은 아직 거래가 없네요\" action={<Button>시작하기</Button>} />" },
        ComponentDocEntry { name: "DataTable", tagline: "정렬 가능한 표 — 컬럼 정의 + custom cell renderer.", demo_html: r#"<div style="border:1px solid var(--colors-border-primary);border-radius:8px;overflow:hidden;font-size:13px;"><table style="width:100%;border-collapse:collapse;"><thead><tr style="background:var(--colors-background-secondary);"><th style="padding:10px 14px;text-align:left;">날짜</th><th style="padding:10px 14px;text-align:left;">내역</th><th style="padding:10px 14px;text-align:right;">금액</th></tr></thead><tbody><tr><td style="padding:10px 14px;">5/17</td><td style="padding:10px 14px;">월급</td><td style="padding:10px 14px;text-align:right;color:var(--colors-success-600,#16a34a);">+3,200,000원</td></tr><tr><td style="padding:10px 14px;">5/18</td><td style="padding:10px 14px;">스타벅스</td><td style="padding:10px 14px;text-align:right;">-6,500원</td></tr></tbody></table></div>"#, snippet: "<DataTable rows={txs} columns={[{key:'date',header:'날짜',sortable:true}, ...]} />" },
        ComponentDocEntry { name: "Carousel", tagline: "슬라이더 — 화살표 + 도트, 자동 재생 옵션.", demo_html: r#"<div style="position:relative;width:320px;height:160px;border-radius:12px;background:var(--colors-primary-500);display:flex;align-items:center;justify-content:center;color:#fff;font-weight:700;font-size:20px;">슬라이드 1<button style="position:absolute;right:12px;top:50%;transform:translateY(-50%);width:32px;height:32px;border-radius:999px;background:#fff;border:none;">›</button></div>"#, snippet: "<Carousel autoplay={5000}>\n  <Slide1 /><Slide2 /><Slide3 />\n</Carousel>" },
        ComponentDocEntry { name: "Calendar", tagline: "월별 그리드 — 한글 요일, 오늘 강조, 선택 가능.", demo_html: r#"<div style="display:inline-flex;flex-direction:column;gap:6px;padding:10px;border:1px solid var(--colors-border-primary);border-radius:8px;font-size:12px;"><div style="display:flex;justify-content:space-between;font-weight:600;">2026년 5월</div><div style="display:grid;grid-template-columns:repeat(7,28px);gap:2px;text-align:center;color:var(--colors-text-muted);"><div>일</div><div>월</div><div>화</div><div>수</div><div>목</div><div>금</div><div>토</div></div></div>"#, snippet: "<Calendar value={date} onChange={setDate} />" },
        ComponentDocEntry { name: "Chip", tagline: "제거 가능한 태그.", demo_html: r#"<span class="aph-tag" style="padding-right:4px;">서울 <button style="border:none;background:transparent;padding:0 4px;cursor:pointer;">×</button></span>"#, snippet: "<Chip onClose={() => remove(id)}>서울</Chip>" },
        ComponentDocEntry { name: "Image", tagline: "fallback + lazy loading.", demo_html: r#"<span style="display:inline-block;width:120px;height:120px;background:var(--colors-background-secondary);border-radius:12px;display:inline-flex;align-items:center;justify-content:center;color:var(--colors-text-muted);font-size:12px;">상품 사진</span>"#, snippet: "<Image src=\"/product.jpg\" alt=\"상품\" width={120} height={120} rounded={12} />" },
        ComponentDocEntry { name: "Kbd", tagline: "키보드 단축키 표시.", demo_html: r#"<span><kbd style="padding:2px 6px;border:1px solid var(--colors-border-primary);background:var(--colors-background-secondary);border-radius:4px;font-family:monospace;font-size:11px;">⌘</kbd> + <kbd style="padding:2px 6px;border:1px solid var(--colors-border-primary);background:var(--colors-background-secondary);border-radius:4px;font-family:monospace;font-size:11px;">K</kbd></span>"#, snippet: "<Kbd>⌘</Kbd> + <Kbd>K</Kbd>" },
        ComponentDocEntry { name: "Code", tagline: "inline + block — block은 pre wrap.", demo_html: r#"<code style="background:var(--colors-background-secondary);padding:2px 6px;border-radius:4px;font-family:monospace;font-size:12px;">npm i @aphrodite/x</code>"#, snippet: "<Code>npm i</Code>\n<Code block>const x = 1;</Code>" },
        ComponentDocEntry { name: "Timeline", tagline: "주문 / 배송 / 진행 단계 표시.", demo_html: r#"<ol style="display:flex;flex-direction:column;gap:8px;padding:0;list-style:none;"><li style="display:flex;gap:12px;"><span style="width:24px;height:24px;border-radius:999px;background:var(--colors-primary-500);color:#fff;display:inline-flex;align-items:center;justify-content:center;font-size:11px;font-weight:600;">1</span><span><strong>주문 접수</strong><div style="font-size:12px;color:var(--colors-text-muted);">5/17 14:02</div></span></li></ol>"#, snippet: "<Timeline>\n  <TimelineItem marker=\"1\" title=\"주문 접수\" time=\"5/17 14:02\" />\n</Timeline>" },
        ComponentDocEntry { name: "Disclosure", tagline: "단일 펼치기/접기 — Accordion의 단순 버전.", demo_html: r#"<div style="padding:12px 14px;border:1px solid var(--colors-border-primary);border-radius:8px;display:flex;justify-content:space-between;align-items:center;width:240px;font-weight:500;">자세히 보기 <span>+</span></div>"#, snippet: "<Disclosure title=\"자세히 보기\">...</Disclosure>" },
        ComponentDocEntry { name: "ContextMenu", tagline: "우클릭 메뉴 — role=menu.", demo_html: r#"<span style="padding:16px;border:1px dashed var(--colors-border-primary);border-radius:6px;font-size:13px;color:var(--colors-text-muted);">영역에 우클릭 → 메뉴</span>"#, snippet: "<ContextMenu trigger={<TableRow />} items={[{label:'복사',onSelect:...}]} />" },
        ComponentDocEntry { name: "Command", tagline: "⌘K 명령 팔레트 — 키보드 nav, 필터.", demo_html: r#"<div style="background:var(--colors-background-primary);border:1px solid var(--colors-border-primary);border-radius:10px;width:280px;overflow:hidden;"><input style="width:100%;padding:10px 14px;border:none;border-bottom:1px solid var(--colors-border-primary);background:transparent;" placeholder="명령어 검색…" /><div style="padding:4px;font-size:13px;"><div style="padding:8px 12px;border-radius:6px;background:var(--colors-background-secondary);display:flex;justify-content:space-between;"><span>새 문서</span><span style="color:var(--colors-text-muted);">⌘N</span></div></div></div>"#, snippet: "<Command placeholder=\"검색…\" items={[{id:'new', label:'새 문서', hint:'⌘N', onSelect:...}]} />" },
        ComponentDocEntry { name: "Hint", tagline: "간단 안내 텍스트 — neutral / warning / danger.", demo_html: r#"<span style="font-size:12px;color:var(--colors-text-muted);">최대 100MB까지 업로드할 수 있습니다.</span>"#, snippet: "<Hint>최대 100MB까지 업로드 가능</Hint>" },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use aphrodite_core::variant::{Variant, VariantKind};
    use std::collections::BTreeMap;

    fn fixture() -> Vec<Variant> {
        let mut t = BTreeMap::new();
        t.insert("colors.primary.500".into(), "#16a34a".into());
        t.insert("colors.background.primary".into(), "#ffffff".into());
        vec![
            Variant { kind: VariantKind::Light, tokens: t.clone() },
            Variant { kind: VariantKind::Dark, tokens: t },
        ]
    }

    #[test]
    fn docs_index_contains_every_catalog_section() {
        let html = build_docs_index(&fixture(), "test");
        assert!(catalog().len() >= 42, "catalog should list at least 42 components");
        for c in catalog() {
            let anchor = format!(r#"id="c-{}""#, c.name);
            assert!(html.contains(&anchor), "missing section anchor: {anchor}");
        }
    }

    #[test]
    fn docs_index_has_install_section_and_variant_switcher() {
        let html = build_docs_index(&fixture(), "Test Project");
        assert!(html.contains("npm i @aphrodite/test-project"));
        assert!(html.contains(r#"<button data-variant="light""#));
        assert!(html.contains(r#"<button data-variant="dark""#));
    }

    #[test]
    fn docs_index_is_korean_lang() {
        let html = build_docs_index(&fixture(), "x");
        assert!(html.contains(r#"lang="ko""#));
    }
}
