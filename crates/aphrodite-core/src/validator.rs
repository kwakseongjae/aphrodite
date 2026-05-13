//! DESIGN.md validator: schema pass + WCAG-AA contrast.

use crate::{DesignDocument, Variant};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationReport {
    pub violations: Vec<ValidationViolation>,
}

impl ValidationReport {
    pub fn is_ok(&self) -> bool {
        self.violations.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationViolation {
    pub kind: ViolationKind,
    pub message: String,
    pub variant: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ViolationKind {
    MissingRequiredField,
    SectionOutOfOrder,
    DuplicateSection,
    UnresolvedTokenRef,
    PrimaryPaletteMissing,
    WcagAaContrast,
    InvalidColorHex,
}

pub fn validate(doc: &DesignDocument, variants: &[Variant]) -> ValidationReport {
    let mut violations = Vec::new();

    if doc.frontmatter.name.trim().is_empty() {
        violations.push(ValidationViolation {
            kind: ViolationKind::MissingRequiredField,
            message: "frontmatter.name is required".into(),
            variant: None,
        });
    }
    match &doc.frontmatter.colors {
        Some(c) if !c.primary.is_empty() => {}
        _ => violations.push(ValidationViolation {
            kind: ViolationKind::PrimaryPaletteMissing,
            message: "colors.primary palette must exist with at least one shade".into(),
            variant: None,
        }),
    }

    // Section ordering: any present section must be in canonical order, no duplicates.
    let mut seen = std::collections::HashSet::new();
    let mut last_order: i16 = -1;
    for s in &doc.sections {
        if !seen.insert(s.kind) {
            violations.push(ValidationViolation {
                kind: ViolationKind::DuplicateSection,
                message: format!("duplicate section: {:?}", s.kind),
                variant: None,
            });
        }
        let o = s.kind.order() as i16;
        if o < last_order && s.kind != crate::design::SectionKind::Unknown {
            violations.push(ValidationViolation {
                kind: ViolationKind::SectionOutOfOrder,
                message: format!("section {:?} appears out of canonical order", s.kind),
                variant: None,
            });
        }
        last_order = o.max(last_order);
    }

    // Hex validity + WCAG-AA across every variant.
    for v in variants {
        let label = v.kind.label();
        let mut tokens = v.tokens.clone();
        // Pull "background" + "foreground" if present; else use primary.500 as fg, primary.50 as bg.
        let fg = tokens
            .remove("colors.text.primary")
            .or_else(|| tokens.get("colors.primary.700").cloned())
            .or_else(|| tokens.get("colors.primary.500").cloned());
        let bg = tokens
            .remove("colors.background.primary")
            .or_else(|| tokens.get("colors.background.50").cloned())
            .or_else(|| tokens.get("colors.primary.50").cloned());

        for (k, value) in v.tokens.iter() {
            if k.starts_with("colors.") && !is_valid_hex(value) {
                violations.push(ValidationViolation {
                    kind: ViolationKind::InvalidColorHex,
                    message: format!("token `{k}` = `{value}` is not a valid `#RRGGBB` hex"),
                    variant: Some(label.clone()),
                });
            }
        }

        if let (Some(fg), Some(bg)) = (fg, bg) {
            if let (Some(f), Some(b)) = (parse_hex(&fg), parse_hex(&bg)) {
                let ratio = contrast_ratio(f, b);
                if ratio < 4.5 {
                    violations.push(ValidationViolation {
                        kind: ViolationKind::WcagAaContrast,
                        message: format!(
                            "WCAG-AA fail: fg {fg} vs bg {bg} contrast {ratio:.2}:1 < 4.5:1"
                        ),
                        variant: Some(label.clone()),
                    });
                }
            }
        }
    }

    ValidationReport { violations }
}

fn is_valid_hex(s: &str) -> bool {
    let s = s.trim();
    s.len() == 7 && s.starts_with('#') && s[1..].chars().all(|c| c.is_ascii_hexdigit())
}

fn parse_hex(s: &str) -> Option<(u8, u8, u8)> {
    let s = s.trim();
    if !is_valid_hex(s) {
        return None;
    }
    let r = u8::from_str_radix(&s[1..3], 16).ok()?;
    let g = u8::from_str_radix(&s[3..5], 16).ok()?;
    let b = u8::from_str_radix(&s[5..7], 16).ok()?;
    Some((r, g, b))
}

/// WCAG 2.1 relative luminance + contrast ratio.
pub fn contrast_ratio(fg: (u8, u8, u8), bg: (u8, u8, u8)) -> f64 {
    let l1 = relative_luminance(fg);
    let l2 = relative_luminance(bg);
    let (light, dark) = if l1 >= l2 { (l1, l2) } else { (l2, l1) };
    (light + 0.05) / (dark + 0.05)
}

fn relative_luminance((r, g, b): (u8, u8, u8)) -> f64 {
    fn lin(c: u8) -> f64 {
        let s = c as f64 / 255.0;
        if s <= 0.03928 {
            s / 12.92
        } else {
            ((s + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * lin(r) + 0.7152 * lin(g) + 0.0722 * lin(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_on_white_is_21_to_1() {
        let r = contrast_ratio((0, 0, 0), (255, 255, 255));
        assert!((r - 21.0).abs() < 0.001, "got {r}");
    }

    #[test]
    fn white_on_white_is_1_to_1() {
        let r = contrast_ratio((255, 255, 255), (255, 255, 255));
        assert!((r - 1.0).abs() < 0.001);
    }

    #[test]
    fn rejects_invalid_hex() {
        let v = crate::Variant {
            kind: crate::VariantKind::Light,
            tokens: std::collections::BTreeMap::from([(
                "colors.primary.500".to_string(),
                "not-a-hex".to_string(),
            )]),
        };
        let doc = crate::DesignDocument {
            frontmatter: crate::design::Frontmatter {
                name: "Test".into(),
                colors: Some(crate::design::ColorTokens {
                    primary: std::collections::BTreeMap::from([("500".into(), "#000000".into())]),
                    others: Default::default(),
                }),
                ..Default::default()
            },
            body: String::new(),
            sections: Vec::new(),
        };
        let r = validate(&doc, &[v]);
        assert!(r.violations.iter().any(|v| v.kind == ViolationKind::InvalidColorHex));
    }

    #[test]
    fn flags_low_contrast() {
        // light grey on white: ratio ~1.6 — should fail WCAG-AA.
        let v = crate::Variant {
            kind: crate::VariantKind::Light,
            tokens: std::collections::BTreeMap::from([
                ("colors.background.primary".to_string(), "#ffffff".to_string()),
                ("colors.text.primary".to_string(), "#cccccc".to_string()),
            ]),
        };
        let doc = crate::DesignDocument {
            frontmatter: crate::design::Frontmatter {
                name: "Test".into(),
                colors: Some(crate::design::ColorTokens {
                    primary: std::collections::BTreeMap::from([("500".into(), "#000000".into())]),
                    others: Default::default(),
                }),
                ..Default::default()
            },
            body: String::new(),
            sections: Vec::new(),
        };
        let r = validate(&doc, &[v]);
        assert!(r.violations.iter().any(|v| v.kind == ViolationKind::WcagAaContrast));
    }
}
