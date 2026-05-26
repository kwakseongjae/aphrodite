//! Pattern extractor — looks at a generated DESIGN.md and pulls observable
//! patterns into a `TastePreferences::Observation` so the next feedback event
//! can apply them as weights.

use aphrodite_core::preferences::Observation;

pub fn observe(design_md: &str) -> Observation {
    Observation {
        hue_family: hue_family(design_md),
        display_font: extract_font(design_md, "display"),
        body_font: extract_font(design_md, "body"),
        density: density_from_spacing(design_md),
        serif_vs_sans: serif_vs_sans_from_display(design_md),
        accent_intensity: accent_intensity(design_md),
    }
}

fn extract_font(md: &str, slot: &str) -> Option<String> {
    let key = format!("\n  {slot}:");
    let idx = md.find(&key)?;
    let chunk = &md[idx..(idx + 200).min(md.len())];
    let family_key = "family:";
    let fidx = chunk.find(family_key)?;
    let line = &chunk[fidx + family_key.len()..];
    let end = line.find('\n').unwrap_or(line.len());
    let value = line[..end].trim().trim_matches(&['"', '\''][..]);
    // Take first family before any comma.
    let primary = value.split(',').next().unwrap_or(value).trim().trim_matches(&['"', '\''][..]);
    if primary.is_empty() {
        None
    } else {
        Some(primary.to_string())
    }
}

/// Pull the primary 500 hex and bucket it into a hue family by HSL hue + chroma.
fn hue_family(md: &str) -> Option<String> {
    let hex = extract_primary_500(md)?;
    let (r, g, b) = parse_hex(&hex)?;
    let (h, s, l) = rgb_to_hsl(r, g, b);
    if s < 0.15 {
        return Some("neutral".to_string());
    }
    let family = match h {
        h if h < 15.0 || h >= 340.0 => "red",
        h if h < 45.0 => "warm-orange",
        h if h < 70.0 => "amber",
        h if h < 90.0 => "olive",
        h if h < 150.0 => "green",
        h if h < 190.0 => "teal",
        h if h < 230.0 => "blue",
        h if h < 270.0 => "violet",
        h if h < 310.0 => "magenta",
        _ => "rose",
    };
    let modifier = if l < 0.30 { "deep " }
                   else if l > 0.75 { "pastel " }
                   else if s > 0.65 { "saturated " }
                   else { "" };
    Some(format!("{modifier}{family}"))
}

fn extract_primary_500(md: &str) -> Option<String> {
    let key = "primary:";
    let idx = md.find(key)?;
    let rest = &md[idx..];
    let m = regex_lite_500(rest)?;
    Some(m)
}

/// Tiny regex-free '500: "#xxxxxx"' finder.
fn regex_lite_500(s: &str) -> Option<String> {
    let key = "\"500\"";
    let i = s.find(key).or_else(|| s.find("'500'"))?;
    let after = &s[i + key.len()..];
    let colon = after.find(':')?;
    let after = &after[colon + 1..];
    let q = after.find(|c| c == '"' || c == '\'')?;
    let rest = &after[q + 1..];
    let end = rest.find(|c| c == '"' || c == '\'')?;
    Some(rest[..end].to_string())
}

fn parse_hex(s: &str) -> Option<(u8, u8, u8)> {
    let s = s.trim().trim_start_matches('#');
    if s.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&s[0..2], 16).ok()?;
    let g = u8::from_str_radix(&s[2..4], 16).ok()?;
    let b = u8::from_str_radix(&s[4..6], 16).ok()?;
    Some((r, g, b))
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    if (max - min).abs() < 0.0001 {
        return (0.0, 0.0, l);
    }
    let d = max - min;
    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
    let h = if max == r {
        ((g - b) / d) + if g < b { 6.0 } else { 0.0 }
    } else if max == g {
        ((b - r) / d) + 2.0
    } else {
        ((r - g) / d) + 4.0
    } * 60.0;
    (h.rem_euclid(360.0), s, l)
}

fn density_from_spacing(md: &str) -> Option<f32> {
    // Density signal = (max px / token_count) blended with absolute max.
    // Generous designs tend to have BOTH a big top spacing (96-128px) and a
    // few tokens (6-8); dense designs have many tokens packed under 32px.
    // Falls back to a small magnitude rather than None so the signal always
    // contributes something — better than missing.
    let idx = md.find("\nspacing:")?;
    let chunk = &md[idx..(idx + 600).min(md.len())];
    let mut max_px = 0u32;
    let mut min_px = u32::MAX;
    let mut count = 0u32;
    for line in chunk.lines() {
        let trimmed = line.trim_start();
        // expect lines like:  "8": "32px"
        if !trimmed.starts_with('"') { continue; }
        if let Some(q) = trimmed.find(':') {
            let value_part = trimmed[q + 1..].trim();
            if let Some(start) = value_part.find('"') {
                let after = &value_part[start + 1..];
                if let Some(end) = after.find('"') {
                    let val = &after[..end];
                    if let Some(num) = val.strip_suffix("px") {
                        if let Ok(n) = num.parse::<u32>() {
                            max_px = max_px.max(n);
                            min_px = min_px.min(n);
                            count += 1;
                        }
                    }
                }
            }
        }
        // Stop at next top-level key.
        if !line.starts_with(' ') && !line.starts_with('"') && trimmed.contains(':') && count > 0 {
            break;
        }
    }
    if count == 0 {
        return None;
    }
    let spread = max_px.saturating_sub(if min_px == u32::MAX { 0 } else { min_px });

    // Bucketing:
    //   max ≥ 96    → -0.5 generous
    //   max ≥ 64    → -0.25 lean generous
    //   max ≥ 48 and spread ≥ 40 → -0.15 lean generous
    //   max ≥ 32    → 0.0  neutral
    //   max < 32 and count ≥ 6 → +0.3 dense
    //   default     → +0.1 slight dense lean
    let v = if max_px >= 96 { -0.5 }
            else if max_px >= 64 { -0.25 }
            else if max_px >= 48 && spread >= 40 { -0.15 }
            else if max_px >= 32 { 0.0 }
            else if count >= 6 { 0.3 }
            else { 0.1 };
    Some(v)
}

fn serif_vs_sans_from_display(md: &str) -> Option<f32> {
    let font = extract_font(md, "display")?.to_lowercase();
    let serif_markers = ["serif", "garamond", "caslon", "didot", "playfair", "fraunces",
                         "newsreader", "instrument", "merriweather", "lora", "bodoni", "tiempos",
                         "noto serif", "minion", "georgia", "times", "luminance", "souvenir", "명조"];
    let sans_markers = ["sans", "inter", "helvetica", "neue", "dm sans", "söhne",
                        "geist", "nunito", "open sans", "roboto", "futura", "avenir",
                        "noto sans", "archivo", "uncut", "satoshi", "manrope"];
    let mono_markers = ["mono", "berkeley", "jetbrains", "commit", "fira code", "iosevka", "menlo", "monaco"];
    let mut v: f32 = 0.0;
    for m in &serif_markers { if font.contains(m) { v -= 1.0; } }
    for m in &sans_markers { if font.contains(m) { v += 1.0; } }
    for m in &mono_markers { if font.contains(m) { v += 0.5; } }
    if v.abs() < 0.5 { None } else { Some(v.clamp(-1.0, 1.0)) }
}

fn accent_intensity(md: &str) -> Option<f32> {
    let hex = extract_primary_500(md)?;
    let (r, g, b) = parse_hex(&hex)?;
    let (_, s, l) = rgb_to_hsl(r, g, b);
    // High chroma + mid lightness = bold (positive). Low chroma or extreme L = restrained.
    let near_mid = 1.0 - (l - 0.5).abs() * 2.0;
    let raw = s * near_mid;
    // Map [0, 1] → [-0.7, +0.7]
    Some((raw - 0.4) * 1.4)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hue_family_warm_brown() {
        // #b85a2e — warm brown like our Hangul Pass 2 output
        assert!(hue_family("primary:\n  \"500\": \"#b85a2e\"").unwrap().contains("orange"));
    }
    #[test]
    fn density_generous() {
        // Realistic DESIGN.md shape: `spacing:` is a nested key, so it is
        // preceded by a newline (the parser anchors on `\nspacing:`).
        let md = "tokens:\nspacing:\n  \"1\": \"4px\"\n  \"8\": \"32px\"\n  \"16\": \"128px\"\nnext: x";
        let v = density_from_spacing(md).unwrap();
        // 128px top spacing → most-generous bucket (-0.5).
        assert!(v <= -0.5);
    }
    #[test]
    fn serif_detected() {
        // Realistic DESIGN.md shape: `display:` sits under `fonts:` at 2-space
        // indent, so it is preceded by a newline (parser anchors on `\n  display:`).
        let md = "fonts:\n  display:\n    family: \"'Newsreader', serif\"\n  body:\n    family: \"DM Sans\"";
        assert!(serif_vs_sans_from_display(md).unwrap() < 0.0);
    }
}
