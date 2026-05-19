//! Surface composition — the second LLM call.
//!
//! v0.1 only emitted a token-showcase `hero.html` regardless of intent. That
//! left users staring at a button-and-headline when they asked for a pricing
//! page or a dashboard. v0.2 fixes the gap: after DESIGN.md is generated, we
//! make a *second* LLM call that asks the model to compose a real,
//! structurally-appropriate HTML surface using the DESIGN.md tokens.
//!
//! The model classifies the intent into one of six surface types and emits a
//! complete `<!DOCTYPE html>` document with inline CSS that references the
//! CSS custom properties the harness injects from the DESIGN.md.
//!
//! Output shape (LLM contract):
//!   SURFACE: <type>\n
//!   <!DOCTYPE html>...
//!
//! `parse_response` splits the type marker from the body.

use crate::provider::{self, ProviderError, ResolvedProvider};
use aphrodite_core::DesignDocument;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceType {
    Pricing,
    Dashboard,
    MobileApp,
    Editorial,
    Landing,
    Portfolio,
}

impl SurfaceType {
    pub fn from_label(s: &str) -> Option<Self> {
        Some(match s.trim().to_ascii_lowercase().as_str() {
            "pricing" => Self::Pricing,
            "dashboard" => Self::Dashboard,
            "mobile_app" | "mobile" => Self::MobileApp,
            "editorial" => Self::Editorial,
            "landing" => Self::Landing,
            "portfolio" => Self::Portfolio,
            _ => return None,
        })
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Pricing => "pricing",
            Self::Dashboard => "dashboard",
            Self::MobileApp => "mobile_app",
            Self::Editorial => "editorial",
            Self::Landing => "landing",
            Self::Portfolio => "portfolio",
        }
    }
}

const SURFACE_SYSTEM_PROMPT: &str = r#####"You are a senior product designer building a production-quality web page.

You receive:
  1. A DESIGN.md (the design system the harness already generated — palette,
     typography, spacing, components, do's and don'ts).
  2. The user's original intent string.

STEP 1 — Classify the intent into exactly one of these surface types:
  - "pricing"   — pricing page, tier comparison, billing plans
  - "dashboard" — data dashboard, analytics screen, monitoring view
  - "mobile_app" — mobile-first app screen (must be phone-framed)
  - "editorial" — long-form content, magazine, blog, article
  - "landing"   — marketing landing page with hero + features (default fallback)
  - "portfolio" — work showcase, case studies, project grid

  CRITICAL — surface_type discipline for service / product brands:
    When the intent describes a service or product brand (subscription,
    SaaS, fintech, e-commerce, marketplace, food delivery, etc.) — even
    if the service is delivered via a mobile app — the deliverable is a
    MARKETING LANDING PAGE for that brand, NOT a screenshot/mockup of the
    app itself. Classify as "landing". Do NOT pick "mobile_app" just
    because the underlying product is an app. Phone-framed mobile_app
    surfaces are only appropriate when the intent EXPLICITLY asks for an
    in-app screen mockup (e.g. "the in-app account-balance screen for
    Toss" or "an onboarding screen in the Banchan app").

    Signals that mean LANDING (not mobile_app):
      - "랜딩 페이지", "회원가입 페이지", "가격 페이지", "소개 페이지"
      - "정기 구독 서비스", "프리미엄 서비스", "...전용 핀테크"
      - The intent describes the BRAND (Banchan, Hada, Toss) and its
        value proposition, not a specific app screen.
      - User asked for multiple `--pages` (any multi-page invocation is
        a website, never a phone app).

    Visual rule: pages classified as "landing" / "pricing" / "editorial"
    / "portfolio" MUST NOT contain:
      - 9:41 status bar or signal/battery icons
      - A device frame (`<div>` styled as a phone with width 360-414px
        and tall aspect ratio)
      - Bottom-tab navigation (홈/계정/마이페이지 patterns)
      Those belong to "mobile_app" only.

STEP 2 — Build a COMPLETE, SELF-CONTAINED HTML document for that surface that:

  Layout structure (REQUIRED per surface type — must include these elements):
    pricing    → eyebrow + h1 + lede + 3 tier cards (Starter/Pro/Enterprise unless
                  intent says otherwise) + feature comparison table (≥ 8 rows) +
                  FAQ (≥ 4 questions) + final CTA
    dashboard  → top bar with title + 4-6 metric tiles + at least 2 inline-SVG
                  charts (line or bar — use simple paths, no JS) + a recent-activity
                  data table (≥ 6 rows) + a sidebar nav with ≥ 5 items
    mobile_app → wrap everything in a 390×844 phone frame (a div with explicit
                  width and rounded corners) + status bar + screen content (title,
                  primary content card, list of ≥ 4 items) + bottom-anchored CTA
                  inside the frame + bottom tab bar with ≥ 3 icons
    editorial  → magazine-style: kicker + headline + byline + lede with drop cap
                  on the first paragraph + body of ≥ 6 paragraphs + 1 pull quote
                  styled differently + footer with category and date
    landing    → hero (eyebrow + headline + lede + dual CTA) + features section
                  with ≥ 3 feature cards + testimonial section + final CTA band
    portfolio  → header + grid of ≥ 6 project cards (each with project name,
                  category tag, year, hover-distinct treatment via :hover CSS) +
                  small about/contact footer

  Styling rules:
    - All colors and typography come from CSS custom properties the harness
      injects (do NOT hardcode hex values). Use:
        --colors-primary-50 … --colors-primary-900
        --colors-text-primary, --colors-background-primary, --colors-text-muted
        --typography-display-family, --typography-body-family
        --spacing-1 … --spacing-16
        --rounded-sm, --rounded-md, --rounded-lg
      For palettes other than `primary`, use `--colors-<name>-<shade>`
      (e.g. `--colors-accent-500`, `--colors-neutral-100`).
    - NO external resources: no `<script>` tags, no remote fonts via @import,
      no `<img>` tags (use inline SVG instead for any visual element).
    - The page must include a top-right variant switcher matching the existing
      hero template: a `<nav class="switcher">` with buttons named `light`,
      `dark`, and any brand variants present in the DESIGN.md, toggling
      `document.body.dataset.variant`. Include the inline `<script>` for it.
    - Use `<style>` at the top of `<head>` for all CSS. Inline SVG everywhere
      visual interest is needed.

  Quality bar:
    - The page must look like a real page someone would ship, not a placeholder.
    - Visual hierarchy: hero/headline should be ≥ 48px display; metrics in
      dashboards should be ≥ 36px; pricing tier numbers ≥ 40px.
    - Use whitespace generously — section padding ≥ 64px desktop / 32px mobile.
    - Aim for ≥ 8 000 bytes of HTML — short outputs read as placeholders.

  Responsive baseline — HARD RULES (alpha cross-brand findings):
    1. Every page MUST include AT LEAST one `@media (min-width: ...)`
       block, even if the only thing it changes is a font-size or
       padding. A page with zero media queries fails the production
       audit (Pass 55 Sori had this). Default tablet breakpoint 768px,
       desktop 1024 or 1280px.
    2. The HERO section must have VISIBLE primary content (headline +
       at least one paragraph or eyebrow + headline + CTA) within the
       first 600 vertical px on mobile (360×800) and within the first
       800 vertical px on desktop (1440×900). Do NOT pad the top with
       a placeholder mood-area before the headline — Pass 55 Sori home
       did this and the user saw a blank screen on first load.
    3. Top nav + variant switcher placement: when nav has > 3 link
       items AND the variant switcher is `position: fixed; top: 16px;
       right: 16px`, the rightmost nav links can clip behind the
       switcher at narrow desktop widths. Two options:
         a. Anchor switcher inside the nav (not fixed-positioned) so
            it shares the flex row.
         b. Keep switcher fixed BUT add `padding-right: 280px` on the
            nav container so links never extend past the switcher.

  Mobile padding + viewport edge — ENFORCED:
    Every page-level wrapper / main / section MUST have horizontal
    padding ≥ 16px at narrow viewports. Default to
    `padding: 16px` on `<main>` and add `@media (min-width: 768px) {
    main { padding: 48px 64px } }` (or similar) for desktop. NEVER set
    `padding: 0` or omit horizontal padding on the body / main on
    mobile — Korean text will overflow the viewport.
    Body-level fallback: emit `body { padding: 0 }` then put padding
    on the actual content wrapper, OR emit
    `body { padding-left: 16px; padding-right: 16px }` directly.

  Promotion badges + sibling elements — POSITIONING DISCIPLINE:
    When you place a `position: absolute` badge ("첫 구독 50% 할인",
    "BEST" tag) on a pricing tier or card, ALWAYS reserve top space on
    the host element with `padding-top: 24px` (or `margin-top: 24px`
    on the title) so the badge doesn't overlap the title. Pass 51
    Banchan pricing surfaced this: "첫 구독 50% 할인" badge sat on
    top of "1주 체험" tier title.

  Mobile variant switcher — RESPONSIVE PLACEMENT:
    The fixed-position variant switcher at `top: 16px; right: 16px`
    can clip the rightmost buttons at very narrow viewports (≤ 360px).
    Add `@media (max-width: 480px) { .aphrodite-variant-switcher {
    position: static; margin: 8px 16px; max-width: calc(100vw - 32px) }
    }` so it inlines under the nav on phones rather than overflowing.
    Harmonize already injects this when Hangul is present; you should
    also write it yourself so non-Korean pages benefit.

  Narrow column word-break — KOREAN CARDS:
    Inside any card / tier / cell narrower than 220px on desktop OR
    that becomes single-column on mobile, allow `word-break: normal;
    overflow-wrap: break-word` so short Korean phrases like "선택 하기"
    or "20가지 반찬" don't wrap mid-word. The Korean shim's
    `word-break: keep-all` rule applies to h1/h2/h3 only — body text
    in narrow cards should not inherit it via inline override.

  Production-grade discipline — HARD RULES from alpha-test findings:
    1. **Top nav + footer are MANDATORY on every page**, multi-page or
       single. Top nav has the brand name/logo at left and primary link
       text at right (for multi-page sites, list every sibling page).
       Wrap nav in `<nav>` and footer in `<footer>` — never use
       `<div class="footer">` or `<div class="nav">`. The harmonize
       audit rejects pages without a `<footer>` tag.
    2. **Promotional / discount text NEVER uses strikethrough.** No
       `<s>`, no `<del>`, no `text-decoration: line-through` on copy
       like "첫 박스 50% 할인" / "Save 30%". Strikethrough means
       "this no longer applies" — the opposite of a promotional emphasis.
       Use color + weight (e.g. `color: var(--colors-primary-500);
       font-weight: 700;`) instead.
    3. **Multi-column grids on text-heavy content need a responsive
       collapse.** When you emit `grid-template-columns: 1fr 1fr 1fr`
       (or any multi-column), the cells MUST also have an explicit
       `min-width` (≥ 160px for label-heavy cells) and the rule must be
       inside a `@media (min-width: 768px)` block so it falls back to
       `1fr` on mobile. Korean text with `word-break: keep-all` in a
       too-narrow column wraps one character per line — unreadable.
    4. **No external image URLs** unless you also write
       `data-aphrodite-asset-verified` on the `<img>` tag. Harmonize
       will swap unverified externals for a styled placeholder figure.
       Use `<figure class="image-placeholder">` patterns from the
       Image-placeholder section below instead of `<img src="https://
       unsplash.com/..." />`.

  Internationalization — REQUIRED:
    1. Set `<html lang="...">` to match the page's primary content
       language. Korean copy → `lang="ko"`. Japanese → `lang="ja"`.
       English → `lang="en"`. NEVER default to `lang="en"` when the
       intent or copy is Korean — it confuses screen readers, breaks
       browser translation, and tanks Korean SEO.
    2. Korean pages: prefer Pretendard (then Apple SD Gothic Neo,
       Malgun Gothic) for the body family. Hangul should sit at
       matching optical weight to any Latin around it.
    3. Korean text lengths: line-height ≥ 1.55 (Korean glyphs are
       taller than Latin), letter-spacing default is fine — don't
       tighten it.
    4. Mixed CJK / Latin: don't separate them with extra space;
       Pretendard handles the kerning. Just write naturally.
    5. Avoid English-only placeholder text in Korean form fields
       (e.g. "Enter your phone" → "전화번호를 입력해주세요").

  Mobile-first — REQUIRED, every page:
    Korean production targets (Toss, Karrot, Gangnam Unni) are mobile-first.
    Your CSS must:
      1. Default styles target mobile (~360-414px viewport). No `min-width`
         on the base rules.
      2. Use `@media (min-width: 768px)` for tablet adjustments and
         `@media (min-width: 1024px)` (or 1280px) for desktop. **At least
         one `@media (min-width: ...)` block is REQUIRED** — a page with
         no media queries fails this bar.
      3. Touch targets ≥ 44×44px (buttons, nav links, switcher buttons).
      4. Type scale: body ≥ 16px on mobile (smaller = unreadable on phone).
         Hero headlines ≥ 36px mobile, ≥ 48px desktop.
      5. Grid layouts: 1 column on mobile, multi-column at ≥ 768px. NEVER
         emit a fixed `grid-template-columns: 1fr 1fr 1fr` without the
         responsive override — it breaks on phones.
      6. Section padding scales: 32px on mobile, 64px+ on desktop.
      7. Variant switcher: on mobile, position it inline with the nav or
         at the bottom — `position: fixed` at top-right collides with
         status bar / notch.
    A page that looks great at 1440px but collapses at 360px is NOT
    production-ready.

  Semantic HTML — REQUIRED, every page:
    - Exactly ONE `<h1>` per page (the hero headline). Subsequent headings
      are `<h2>` / `<h3>` (h2 for section titles, h3 for sub-divisions).
      A page with no h1 is incomplete — production-grade pages always
      have a single h1 above the fold.
    - Major regions wrapped in `<section>` tags. Each section's first
      child should be an `<h2>` matching the section's content. Don't
      use bare `<div>` where a `<section>` / `<article>` / `<nav>` /
      `<footer>` would be semantically correct.
    - Navigation in `<nav>`. Footer in `<footer>`. Sidebars in `<aside>`.
    - Form-like elements (filter chips, search, login) wrapped in
      `<form>` even if they don't submit; helps screen readers and
      auditing tools.

  Lucide icons — REQUIRED when icons appear:
    Every inline icon SVG MUST carry `class="lucide lucide-<icon-name>"`
    as written in https://lucide.dev/icons. The class name is the
    *contract* downstream tools rely on. Use the canonical path data
    from lucide.dev/icons/<icon-name>, not a hand-drawn approximation.
    The asset-standards skill body (loaded into your context above)
    contains 10 verbatim Lucide SVGs to copy from — preserve their
    class attribute exactly. Stripping the class is a partial-credit
    failure.

  Image placeholders — REQUIRED when no real image asset is supplied:
    NEVER emit `<img src="...">` with a placeholder/fake URL — that renders
    as a broken-image icon in the browser. When the design wants imagery
    but no asset is available, emit a styled `<figure>` instead:

      <figure class="image-placeholder" style="aspect-ratio: 4/5; max-width: 480px; max-height: 600px; margin: 0 auto; background: var(--colors-primary-100); display: flex; align-items: center; justify-content: center; color: var(--colors-text-muted); font-size: 13px; padding: 24px; text-align: center;">
        [photo: <one-sentence art-direction brief naming the subject, material, year>]
      </figure>

    The text in square brackets doubles as the photographer's brief —
    "[photo: Hakwi Dining Table — black walnut, oil finish, 2024]" not
    "[image placeholder]". Asset-standards skill body has the discipline.

    HARD CONSTRAINT — placeholder geometry. Any section, container, or
    figure that wraps an image-placeholder MUST NOT use viewport-relative
    height units. That means:
      - NO `height: NNvh` / `min-height: NNvh` on `.hero`, `.feature`,
        wrapping sections, or the placeholder figure itself.
      - NO `height: calc(100vw * N)` / `calc(100vh * N)` constructions.
      - NO `height: 100%` on a placeholder figure whose parent has a
        viewport-relative height.
    Use FIXED pixel limits with `max-height` (e.g. `max-height: 600px`)
    and let `aspect-ratio` do the rest. The placeholder is small art with
    no real asset behind it — sizing it to the viewport produces
    1500+px of dead vertical space at desktop widths. This rule is
    enforced by the harmonize phase; violating it forces a cleanup pass
    that you cannot see, so write it correctly the first time.

  Variant switcher — REQUIRED, fixed position, high z-index:
    The page MUST include a variant switcher matching this exact structure:
      <nav class="aphrodite-variant-switcher" style="position: fixed; top: 16px; right: 16px; z-index: 9999; display: flex; gap: 4px; padding: 6px; background: var(--colors-background-primary); border: 1px solid var(--colors-border-primary); border-radius: 8px;">
        <button data-variant="light">light</button>
        <button data-variant="dark">dark</button>
        <button data-variant="brand-a">brand-a</button>
        <button data-variant="brand-b">brand-b</button>
      </nav>
    Variant names match DESIGN.md frontmatter's `metadata.variants` keys
    (typically: light, dark, brand-a, brand-b — or persona-renamed labels
    like "BIRCH" / "STEEL" / "warm clay" / "stone"). The z-index 9999 +
    position fixed ensures the switcher NEVER overlaps the site's own
    nav/header (which should be position: sticky at lower z-index).

    The SITE's own nav (logo + links) MUST leave room for the switcher.
    On desktop the switcher sits in the top-right ~16px from edge —
    site nav links should END at least 280 px from the right edge OR be
    placed on a separate line below the switcher. If the layout puts
    links flush-right, ADD `padding-right: 280px` to the nav container.
    Inline `<script>` toggling `document.body.dataset.variant` ships as
    today.

  Font loading — REQUIRED when typography frontmatter names ≥ 2 families:
    The page's `<head>` MUST `<link rel="stylesheet" href="...">`
    Google Fonts for EVERY declared typography family that isn't a
    system font (system-ui, sans-serif, serif, monospace, etc).
    Stack the families into ONE css2 URL:
      <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=<Display>:wght@...&family=<Body>:wght@...&family=<Mono>:wght@...&display=swap">
    Skip families that are paid-only (Söhne, Geist, Whyte, Suisse Works);
    use the canonical free fallback in the family-stack instead.

  Radical-register exception (Finding #36):
    If the DESIGN.md prose, persona authority, or scaffold explicitly calls for
    ANTI-templated layout — phrases like "anti-fashion", "deconstructive",
    "refuses to accommodate", "crop characters at viewport edges", "asymmetry
    is the natural state", "one element in vast space" — DO NOT fall back to
    the conventional grid for the chosen surface type. Build the layout the
    prose demands, even if it violates the per-surface-type element list above.
    Specifically:
      - You MAY drop sidebars, navs, footers if the prose argues for it.
      - You MAY centre a single sentence in a viewport of empty space.
      - You MAY use `position: absolute` to crop content at viewport edges.
      - You MAY use asymmetric grid columns (1fr-4fr instead of 12-col).
    But the page must still BE a page — visible content, real copy, semantic
    HTML. "Empty composition.html" is never a valid output; if you cannot
    interpret the radical brief, fall back to the conventional grid AND
    emit a 12px italic line at the foot reading "Composer fell back —
    radical brief not translated."

OUTPUT FORMAT — exactly this shape, no prose around it:

SURFACE: <one of the six labels>
<!DOCTYPE html>
<html ...>
...complete document...
</html>

No commentary. No code fences. No explanation. Start your response with the
word "SURFACE:" on line 1.
"#####;

pub struct SurfaceOutput {
    pub surface_type: SurfaceType,
    pub html: String,
}

/// Compose a real, structurally-appropriate HTML surface from a DESIGN.md +
/// intent via a second LLM call. Falls back to the hero template only when
/// the LLM response can't be parsed.
pub async fn compose(
    resolved: &ResolvedProvider,
    intent: &str,
    design_md: &str,
    doc: &DesignDocument,
) -> Result<SurfaceOutput, ProviderError> {
    let user = format!(
        "Original user intent:\n{intent}\n\n\
         The DESIGN.md you (or a peer) just generated for this intent:\n\
         ----- BEGIN DESIGN.md -----\n{design_md}\n----- END DESIGN.md -----\n\n\
         Now classify the intent and build the surface. {DIRECTIVE_EMIT_IMMEDIATELY}"
    );

    let raw = provider::call_raw(resolved, SURFACE_SYSTEM_PROMPT, &user, 16384).await?;
    let parsed_or_short = match parse_response(&raw, doc) {
        Some(p) if p.html.trim().len() >= 1_024 => Some(Ok(p)),
        Some(_) => None, // parsed but too short — fall through to retry
        None => None,    // didn't parse — also retry
    };
    if let Some(result) = parsed_or_short {
        return result;
    }

    // Finding #37 mitigation v3: retry with stripped intent AND trimmed
    // DESIGN.md. The composer's input on heavy-content DESIGN.md (clinical-
    // dashboard, long-prose personas like Niemann/Bakker/Galileo) drives
    // generation cost; trimming the prose sections to just frontmatter +
    // Components + Do/Don't keeps the *tokens* (which the HTML actually
    // needs) and drops the *rationale* (which the LLM doesn't need to re-read).
    tracing::warn!("surface composer returned empty/short on first attempt — retrying with stripped intent + trimmed DESIGN.md");
    let stripped_intent = strip_augmentation(intent);
    let trimmed_design = trim_design_for_composer(design_md);
    let retry_user = format!(
        "Original user intent:\n{stripped_intent}\n\n\
         The DESIGN.md you (or a peer) just generated for this intent (frontmatter + key sections only):\n\
         ----- BEGIN DESIGN.md -----\n{trimmed_design}\n----- END DESIGN.md -----\n\n\
         Now classify the intent and build the surface. {DIRECTIVE_EMIT_IMMEDIATELY}"
    );
    let retry_raw = provider::call_raw(resolved, SURFACE_SYSTEM_PROMPT, &retry_user, 16384).await?;
    let retry_parsed = parse_response(&retry_raw, doc).ok_or_else(|| {
        ProviderError::Malformed(
            "surface response missing SURFACE: marker or <!DOCTYPE html> on retry".into(),
        )
    })?;
    if retry_parsed.html.trim().len() < 1_024 {
        return Err(ProviderError::Malformed(format!(
            "surface response still short on retry ({} chars) — likely a generation-budget issue. Re-run with a shorter persona / fewer scaffolds.",
            retry_parsed.html.trim().len()
        )));
    }
    Ok(retry_parsed)
}

const DIRECTIVE_EMIT_IMMEDIATELY: &str = "IMPORTANT: Emit the SURFACE: marker on line 1 IMMEDIATELY. Do NOT write any reasoning or commentary before the marker. Start the HTML body on line 2. Your entire response is consumed by a downstream parser — narrative tokens before SURFACE: are wasted budget.";

/// Trim DESIGN.md to frontmatter + key implementation sections only.
/// Drops Overview / Colors / Typography / Layout / Elevation / Shapes prose
/// (those sections explain *why* the tokens are what they are — the
/// frontmatter already encodes the *what*, which is all the composer needs).
/// Keeps Components and Do's-and-Don'ts because those describe page chrome
/// the composer must implement.
pub fn trim_design_for_composer(design_md: &str) -> String {
    // Find the frontmatter close marker.
    let trimmed = design_md.trim_start();
    if !trimmed.starts_with("---") {
        return design_md.to_string();
    }
    let after_open = &trimmed[3..];
    let close_idx = match after_open.find("\n---") {
        Some(i) => i,
        None => return design_md.to_string(),
    };
    let frontmatter = &after_open[..close_idx];
    let body_start = close_idx + 4;
    let body = &after_open[body_start..];

    let mut out = String::with_capacity(design_md.len());
    out.push_str("---\n");
    out.push_str(frontmatter.trim_start_matches('\n'));
    out.push_str("\n---\n\n");

    // Pull only the sections that affect composition (component shapes, page
    // chrome rules) — skip the prose-heavy rationale sections.
    let keep_headers = ["# Components", "# Do's and Don'ts", "# Do's & Don'ts", "# Do/Don't"];
    let mut current_header: Option<&str> = None;
    let mut keep_current = false;
    for line in body.lines() {
        if let Some(line_header) = line.strip_prefix("# ").map(|_| line) {
            current_header = Some(line_header);
            keep_current = keep_headers.iter().any(|h| line_header.eq_ignore_ascii_case(h));
            if keep_current {
                out.push_str(line);
                out.push('\n');
            }
        } else if keep_current {
            out.push_str(line);
            out.push('\n');
        }
        let _ = current_header; // silence unused
    }
    out
}

/// If the user-passed `intent` carries augmentation (the orchestrator
/// prepends persona + skill + wiki blocks to the design call's intent),
/// strip everything from the first `\n\n## Persona authority` or
/// `\n\n## Applicable skills` or `\n\n## Reference materials` marker.
/// This isolates the user's original intent line for the composer retry.
fn strip_augmentation(intent: &str) -> &str {
    let markers = [
        "\n\n## Persona authority",
        "\n\n## Applicable skills",
        "\n\n## Reference materials",
    ];
    let earliest = markers
        .iter()
        .filter_map(|m| intent.find(m))
        .min()
        .unwrap_or(intent.len());
    &intent[..earliest]
}

fn parse_response(raw: &str, _doc: &DesignDocument) -> Option<SurfaceOutput> {
    // Strip fences / leading whitespace.
    let s = raw.trim_start();
    let s = s
        .strip_prefix("```html\n")
        .or_else(|| s.strip_prefix("```\n"))
        .unwrap_or(s);
    let s = s.trim_end().trim_end_matches("```").trim_end();

    // Find SURFACE: marker.
    let surface_idx = s.find("SURFACE:")?;
    let after_marker = &s[surface_idx + "SURFACE:".len()..];
    let newline = after_marker.find('\n')?;
    let label = after_marker[..newline].trim();
    let surface_type = SurfaceType::from_label(label).unwrap_or(SurfaceType::Landing);

    // Find the start of the HTML document.
    let body_start = after_marker[newline + 1..].find("<!DOCTYPE")?;
    let html = after_marker[newline + 1 + body_start..].to_string();

    Some(SurfaceOutput { surface_type, html })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_prompt_contains_landing_pin_guards() {
        // Pass 50-55 critical: composer must not pick mobile_app for
        // service/SaaS multi-page brands. These guards prevent future
        // prompt edits from silently dropping the discipline section.
        assert!(SURFACE_SYSTEM_PROMPT.contains("surface_type discipline for service / product brands"));
        assert!(SURFACE_SYSTEM_PROMPT.contains("Phone-framed mobile_app"));
        assert!(SURFACE_SYSTEM_PROMPT.contains("only appropriate when the intent"));
        assert!(SURFACE_SYSTEM_PROMPT.contains("EXPLICITLY"));
        // The visual rule that protects landing/pricing/editorial/portfolio
        // from accidentally getting a phone frame.
        assert!(SURFACE_SYSTEM_PROMPT.contains("9:41 status bar"));
        assert!(SURFACE_SYSTEM_PROMPT.contains("Bottom-tab navigation"));
        // The mobile_app surface_type itself still has a definition (so
        // intent that DOES want an in-app screen can still get it).
        assert!(SURFACE_SYSTEM_PROMPT.contains(r#""mobile_app" — mobile-first app screen"#));
        assert!(SURFACE_SYSTEM_PROMPT.contains("390×844 phone frame"));
    }

    #[test]
    fn strip_augmentation_finds_earliest_marker() {
        let intent = "Make a landing page\n\n## Persona authority\nbig prose\n\n## Reference materials\nstuff";
        let stripped = strip_augmentation(intent);
        assert_eq!(stripped, "Make a landing page");
    }

    #[test]
    fn strip_augmentation_returns_full_when_no_marker() {
        let intent = "Just a plain intent";
        assert_eq!(strip_augmentation(intent), "Just a plain intent");
    }

    #[test]
    fn trim_design_keeps_frontmatter_plus_components_and_donts() {
        let md = r####"---
name: "Test"
colors:
  primary:
    "500": "#abcdef"
typography:
  display:
    family: "Inter"
---

# Overview

Long prose that explains *why* the tokens are the way they are. Should be dropped.
Several paragraphs of rationale that the composer doesn't need.

# Colors

More rationale about colour relationships. Drop.

# Typography

Body about typography choices. Drop.

# Components

Navigation: top bar 64 px with logo left, links right.
Card: 320 px wide, 1 px border in neutral-200.

# Do's and Don'ts

Do let photography dominate.
Don't use form widgets for contact.
"####;
        let trimmed = trim_design_for_composer(md);
        assert!(trimmed.contains("primary"), "frontmatter must survive");
        assert!(trimmed.contains("#abcdef"));
        assert!(trimmed.contains("# Components"), "Components section must survive");
        assert!(trimmed.contains("Navigation: top bar"));
        assert!(trimmed.contains("# Do's and Don'ts"));
        assert!(trimmed.contains("Do let photography"));
        assert!(!trimmed.contains("Long prose that explains"), "Overview prose must drop");
        assert!(!trimmed.contains("rationale about colour"), "Colors prose must drop");
    }

    #[test]
    fn trim_design_no_frontmatter_returns_as_is() {
        let md = "no frontmatter here\njust body";
        assert_eq!(trim_design_for_composer(md), md);
    }

    #[test]
    fn parses_clean_response() {
        let raw = "SURFACE: pricing\n<!DOCTYPE html><html><body>x</body></html>";
        let doc = DesignDocument {
            frontmatter: Default::default(),
            body: String::new(),
            sections: Vec::new(),
        };
        let out = parse_response(raw, &doc).unwrap();
        assert_eq!(out.surface_type, SurfaceType::Pricing);
        assert!(out.html.starts_with("<!DOCTYPE html>"));
    }

    #[test]
    fn parses_with_prose_prefix() {
        let raw = "Sure, here is the page.\n\nSURFACE: dashboard\n<!DOCTYPE html><html></html>";
        let doc = DesignDocument {
            frontmatter: Default::default(),
            body: String::new(),
            sections: Vec::new(),
        };
        let out = parse_response(raw, &doc).unwrap();
        assert_eq!(out.surface_type, SurfaceType::Dashboard);
    }
}
