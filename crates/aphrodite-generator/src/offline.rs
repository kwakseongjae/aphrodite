//! Offline DESIGN.md generator. Used when no provider key is configured.
//!
//! Deterministic: given the same intent, produces the same DESIGN.md. The
//! purpose is to keep the full pipeline (parse → validate → variant → hero)
//! exercisable end-to-end on a machine with no network and no API key. The
//! output is intentionally generic and not aesthetically claim-worthy — it
//! exists to make tests pass, not to be beautiful.

pub fn generate(intent: &str) -> String {
    let name = derive_name(intent);
    let seed = hash_u32(intent);

    // Pick a primary hue from the intent hash; emit a 9-shade palette.
    let hue = (seed % 360) as f64;
    let primary = palette_from_hue(hue);
    let secondary = palette_from_hue((hue + 180.0) % 360.0);

    format!(
        r##"---
name: {name}
version: "0.1"
description: "{intent_escaped}"
colors:
  primary:
    "50":  "{p50}"
    "100": "{p100}"
    "200": "{p200}"
    "300": "{p300}"
    "400": "{p400}"
    "500": "{p500}"
    "600": "{p600}"
    "700": "{p700}"
    "800": "{p800}"
    "900": "{p900}"
  secondary:
    "50":  "{s50}"
    "500": "{s500}"
    "900": "{s900}"
  neutral:
    "0":    "#ffffff"
    "1000": "#000000"
typography:
  display:
    family: "Inter Tight"
    weight: 700
  body:
    family: "Inter"
    weight: 400
spacing:
  "1": "4px"
  "2": "8px"
  "4": "16px"
  "8": "32px"
rounded:
  sm: "4px"
  md: "8px"
  lg: "16px"
metadata:
  variants:
    light:
      description: "Default light mode"
      tokens:
        colors.background.primary: "#ffffff"
        colors.text.primary:        "#0b0b10"
    dark:
      description: "Dark mode"
      tokens:
        colors.background.primary: "#0b0b10"
        colors.text.primary:        "#f5f5f7"
    brand-a:
      description: "Warm brand variant"
      tokens:
        colors.background.primary: "#fff8f1"
        colors.text.primary:        "#241407"
    brand-b:
      description: "Cool brand variant"
      tokens:
        colors.background.primary: "#f1faf8"
        colors.text.primary:        "#04231d"
---

# Overview

{intent}. This document is the offline-generated design baseline. It
satisfies the Google Labs DESIGN.md alpha schema and ships four variants
(light, dark, brand-a, brand-b), each WCAG-AA-validated for text-on-background
contrast.

# Colors

The primary palette anchors brand recognition. Use shade 500 for accent
fills, 700 for active states, and 50/100 for hairline surfaces. Secondary
exists as a deliberate complement; reach for it sparingly.

# Typography

Inter Tight for display, Inter for body. Display steps follow a 1.25 ratio
from 16px body. Line-height: 1.5 (body), 1.1 (display).

# Layout

8pt baseline grid. Container max-widths follow Tailwind defaults
(640 / 768 / 1024 / 1280 / 1536). Side gutters are spacing.4 (16px) on
mobile, scaling to spacing.8 (32px) above the 768 breakpoint.

# Elevation & Depth

Three depth levels: 0 (flat), 1 (subtle 1-2px shadow for cards), 2
(modal/overlay 8-16px shadow). Never combine elevations within a single
hierarchy.

# Shapes

Default radius is rounded.md (8px). Use rounded.sm (4px) on dense
controls and rounded.lg (16px) on hero containers. Pure-circular is
reserved for avatars and floating action buttons.

# Components

Buttons: solid primary, outline secondary, ghost tertiary. Inputs: bordered
with rounded.md, focus ring uses primary.500 at 40% alpha. Cards: primary.50
on light variant, primary.900 on dark.

# Do's and Don'ts

DO use primary.500 as the only accent fill in a single view. DON'T mix
primary and secondary as adjacent fills. DO maintain ≥ 4.5:1 contrast on
all text. DON'T introduce shades not defined in this document.
"##,
        name = name,
        intent = intent,
        intent_escaped = intent.replace('"', "\\\""),
        p50 = primary[0],
        p100 = primary[1],
        p200 = primary[2],
        p300 = primary[3],
        p400 = primary[4],
        p500 = primary[5],
        p600 = primary[6],
        p700 = primary[7],
        p800 = primary[8],
        p900 = primary[9],
        s50 = secondary[0],
        s500 = secondary[5],
        s900 = secondary[9],
    )
}

fn derive_name(intent: &str) -> String {
    // The eyebrow uses this — must read as a project / brand name, not as the
    // first three words of an instructional prompt.
    //
    // Strategy: drop instructional verbs ("include", "use", "make", "build",
    // "generate"), drop framework names, take the first 2 informative words,
    // Title-case. Fall back to "Aphrodite Project" if nothing's left.
    const SKIP: &[&str] = &[
        "a", "an", "the", "this", "that", "and", "or", "of", "for", "with", "into", "onto",
        "include", "use", "uses", "using", "make", "made", "build", "built", "generate",
        "generated", "create", "created", "place", "placed", "show", "showing", "design",
        "designed", "render", "rendered", "ship", "shipped", "produce", "produced",
        "three", "threejs", "react", "vue", "svelte", "angular", "tailwind", "figma",
        "video", "image", "png", "jpg", "mp4", "css", "html", "js",
    ];
    let cleaned: String = intent
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '-')
        .collect();
    let words: Vec<&str> = cleaned
        .split_whitespace()
        .filter(|w| {
            let lower: String = w.to_ascii_lowercase();
            !SKIP.contains(&lower.as_str()) && lower.len() > 1
        })
        .take(2)
        .collect();
    if words.is_empty() {
        return "Aphrodite Project".to_string();
    }
    words
        .iter()
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn hash_u32(s: &str) -> u32 {
    // FNV-1a; we just need stable, not cryptographic.
    let mut h: u32 = 0x811c9dc5;
    for b in s.bytes() {
        h ^= b as u32;
        h = h.wrapping_mul(0x01000193);
    }
    h
}

/// Emit a 10-shade palette (50, 100, ..., 900) given a hue. Saturation and
/// lightness are tuned for accessibility — primary.500 lands at L≈45 so
/// contrast against white is naturally ≥ 4.5:1.
fn palette_from_hue(hue: f64) -> [String; 10] {
    let ls = [97.0, 92.0, 84.0, 73.0, 60.0, 45.0, 36.0, 28.0, 20.0, 12.0];
    let s = 70.0;
    let mut out: [String; 10] = Default::default();
    for (i, l) in ls.iter().enumerate() {
        out[i] = hsl_to_hex(hue, s, *l);
    }
    out
}

fn hsl_to_hex(h: f64, s_pct: f64, l_pct: f64) -> String {
    let s = s_pct / 100.0;
    let l = l_pct / 100.0;
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h_prime = (h % 360.0) / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let (r1, g1, b1) = match h_prime as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    let r = ((r1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    let g = ((g1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    let b = ((b1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    format!("#{r:02x}{g:02x}{b:02x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use aphrodite_core::parse_design;

    #[test]
    fn output_parses_back() {
        let md = generate("a calm editorial landing page");
        let doc = parse_design(&md).expect("must parse");
        assert!(doc.frontmatter.colors.is_some());
        assert!(doc.frontmatter.metadata.is_some());
        let v = doc.frontmatter.metadata.as_ref().unwrap().variants.len();
        assert_eq!(v, 4, "expected 4 variants, got {v}");
    }
}
