//! v1.0 React/TSX export. Aphrodite v0.6 emitted an HTML preview;
//! production-grade FE teams need actual `.tsx` files they can `npm i`.
//! This module deterministically generates a publishable npm package
//! from the resolved DESIGN.md + variants:
//!
//! ```text
//!   react/
//!     package.json
//!     tsconfig.json
//!     src/
//!       index.ts
//!       tokens.ts
//!       Button.tsx
//!       Input.tsx
//!       Tag.tsx
//!       Avatar.tsx
//!       Card.tsx
//!       Modal.tsx
//!       Drawer.tsx
//!       Skeleton.tsx
//!       FormField.tsx
//! ```
//!
//! Components are typed React.forwardRef function components with
//! variant / size / state props matching DESIGN.md tokens. Zero LLM
//! cost; one-pass deterministic generation. The point: a Toss/Karrot FE
//! engineer can clone the run, `cd react && npm publish` (or vendor it
//! into their monorepo), and `import { Button } from "@org/aphrodite"`.

use aphrodite_core::variant::Variant;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct ReactPackage {
    /// Map of relative file path → file contents.
    pub files: BTreeMap<String, String>,
}

impl ReactPackage {
    pub fn write_to(&self, root: &std::path::Path) -> std::io::Result<()> {
        for (rel, contents) in &self.files {
            let full = root.join(rel);
            if let Some(parent) = full.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&full, contents)?;
        }
        Ok(())
    }
}

pub fn build(variants: &[Variant], project_name: &str) -> ReactPackage {
    let mut files: BTreeMap<String, String> = BTreeMap::new();
    let kebab = slug(project_name);
    files.insert("package.json".into(), build_package_json(&kebab));
    files.insert("tsconfig.json".into(), build_tsconfig());
    files.insert("README.md".into(), build_readme(project_name, &kebab));
    files.insert("src/tokens.ts".into(), build_tokens_ts(variants));
    files.insert("src/cn.ts".into(), build_cn_helper());
    files.insert("src/Button.tsx".into(), BUTTON_TSX.into());
    files.insert("src/Input.tsx".into(), INPUT_TSX.into());
    files.insert("src/Tag.tsx".into(), TAG_TSX.into());
    files.insert("src/Avatar.tsx".into(), AVATAR_TSX.into());
    files.insert("src/Card.tsx".into(), CARD_TSX.into());
    files.insert("src/Modal.tsx".into(), MODAL_TSX.into());
    files.insert("src/Drawer.tsx".into(), DRAWER_TSX.into());
    files.insert("src/Skeleton.tsx".into(), SKELETON_TSX.into());
    files.insert("src/FormField.tsx".into(), FORM_FIELD_TSX.into());
    files.insert("src/Switch.tsx".into(), SWITCH_TSX.into());
    files.insert("src/Badge.tsx".into(), BADGE_TSX.into());
    files.insert("src/Spinner.tsx".into(), SPINNER_TSX.into());
    files.insert("src/index.ts".into(), build_index_ts());
    files.insert("src/styles.css".into(), build_styles_css(variants));
    ReactPackage { files }
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

fn build_package_json(kebab: &str) -> String {
    format!(
        r#"{{
  "name": "@aphrodite/{kebab}",
  "version": "0.1.0",
  "description": "Auto-generated React component library from an Aphrodite design contract.",
  "main": "dist/index.js",
  "module": "dist/index.mjs",
  "types": "dist/index.d.ts",
  "files": ["dist", "src", "README.md"],
  "exports": {{
    ".": {{
      "types": "./dist/index.d.ts",
      "import": "./dist/index.mjs",
      "require": "./dist/index.js"
    }},
    "./styles.css": "./src/styles.css"
  }},
  "scripts": {{
    "build": "tsup src/index.ts --format esm,cjs --dts",
    "typecheck": "tsc --noEmit"
  }},
  "peerDependencies": {{
    "react": ">=18",
    "react-dom": ">=18"
  }},
  "devDependencies": {{
    "@types/react": "^18",
    "tsup": "^8",
    "typescript": "^5"
  }},
  "sideEffects": ["**/*.css"],
  "license": "MIT"
}}
"#
    )
}

fn build_tsconfig() -> String {
    r#"{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "jsx": "react-jsx",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "declaration": true,
    "outDir": "dist",
    "lib": ["DOM", "DOM.Iterable", "ES2020"]
  },
  "include": ["src"]
}
"#
    .into()
}

fn build_readme(name: &str, kebab: &str) -> String {
    format!(
        r#"# {name} — React component library

Auto-generated from an Aphrodite DESIGN.md. **Do not edit `src/` by hand** —
those files are regenerated on every `aphrodite create` / `aphrodite react`
run. Tokens and component primitives live here so your product
codebase can `npm i @aphrodite/{kebab}` and import directly.

## Quick start

```tsx
import {{ Button, Input, Card }} from "@aphrodite/{kebab}";
import "@aphrodite/{kebab}/styles.css";

export function Page() {{
  return (
    <Card>
      <Input label="이메일" placeholder="example@toss.im" />
      <Button variant="primary">계속하기</Button>
    </Card>
  );
}}
```

## What you get

- TypeScript components (`forwardRef`, fully typed props)
- Variant / size / state props matching the DESIGN.md token contract
- `tokens.ts` exporting the resolved palette / spacing / typography as
  typed constants — usable in styled-components, Emotion, vanilla CSS
  vars (consume via `styles.css`)
- 12 primitive components: Button, Input, Tag, Avatar, Card, Modal,
  Drawer, Skeleton, FormField, Switch, Badge, Spinner.

## Versioning

Bump `package.json` version manually when you ship a breaking change
to DESIGN.md. The generated code is deterministic — same DESIGN.md +
same Aphrodite version → byte-identical output.
"#
    )
}

fn build_tokens_ts(variants: &[Variant]) -> String {
    let mut out = String::from(
        "/* Auto-generated — Aphrodite token export. Source: DESIGN.md */\n\n",
    );
    out.push_str("export type VariantName =");
    for (i, v) in variants.iter().enumerate() {
        out.push_str(&format!(
            "{} \"{}\"",
            if i == 0 { "" } else { " |" },
            v.kind.label()
        ));
    }
    out.push_str(";\n\n");
    out.push_str("export const tokens = {\n");
    for v in variants {
        out.push_str(&format!("  \"{}\": {{\n", v.kind.label()));
        for (k, val) in &v.tokens {
            out.push_str(&format!("    \"{}\": \"{}\",\n", k, val.replace('"', "\\\"")));
        }
        out.push_str("  },\n");
    }
    out.push_str("} as const;\n\n");
    out.push_str("export type TokenKey = keyof typeof tokens[\"");
    let first_label = variants
        .first()
        .map(|v| v.kind.label())
        .unwrap_or_else(|| "light".to_string());
    out.push_str(&first_label);
    out.push_str("\"];\n");
    out
}

fn build_cn_helper() -> String {
    r#"/* Auto-generated. Class-name composition helper. */

export function cn(...classes: Array<string | false | null | undefined>): string {
  return classes.filter(Boolean).join(" ");
}
"#
    .into()
}

fn build_index_ts() -> String {
    r#"/* Auto-generated barrel — re-export every primitive + tokens. */

export { Button } from "./Button";
export type { ButtonProps } from "./Button";
export { Input } from "./Input";
export type { InputProps } from "./Input";
export { Tag } from "./Tag";
export type { TagProps } from "./Tag";
export { Avatar } from "./Avatar";
export type { AvatarProps } from "./Avatar";
export { Card } from "./Card";
export type { CardProps } from "./Card";
export { Modal } from "./Modal";
export type { ModalProps } from "./Modal";
export { Drawer } from "./Drawer";
export type { DrawerProps } from "./Drawer";
export { Skeleton } from "./Skeleton";
export type { SkeletonProps } from "./Skeleton";
export { FormField } from "./FormField";
export type { FormFieldProps } from "./FormField";
export { Switch } from "./Switch";
export type { SwitchProps } from "./Switch";
export { Badge } from "./Badge";
export type { BadgeProps } from "./Badge";
export { Spinner } from "./Spinner";
export type { SpinnerProps } from "./Spinner";
export { tokens } from "./tokens";
export type { VariantName, TokenKey } from "./tokens";
export { cn } from "./cn";
"#
    .into()
}

fn build_styles_css(variants: &[Variant]) -> String {
    // Same generator as design_system::build_tokens_css but with
    // additional primitive component classes for the React layer to
    // attach to. Keeps styling out of the .tsx files so consumers can
    // override via CSS without ejecting.
    let mut out = String::from("/* Aphrodite component library — base styles. */\n\n");
    for v in variants {
        out.push_str(&format!("body[data-variant=\"{}\"] {{\n", v.kind.label()));
        for (k, val) in &v.tokens {
            out.push_str(&format!("  --{}: {};\n", k.replace('.', "-"), val));
        }
        out.push_str("}\n\n");
    }
    out.push_str(BASE_STYLES);
    out
}

const BASE_STYLES: &str = r#"
/* primitives */
.aph-btn { font: inherit; min-height: 44px; padding: 12px 20px; border-radius: 8px; border: 1px solid transparent; cursor: pointer; font-weight: 600; font-size: 15px; line-height: 1; display: inline-flex; align-items: center; justify-content: center; gap: 8px; }
.aph-btn--primary { background: var(--colors-primary-500); color: var(--colors-background-primary, #fff); }
.aph-btn--primary:hover { background: var(--colors-primary-600, #000); }
.aph-btn--secondary { background: transparent; color: var(--colors-text-primary); border-color: var(--colors-border-primary); }
.aph-btn--ghost { background: transparent; color: var(--colors-text-primary); }
.aph-btn--danger { background: var(--colors-danger-500, #dc2626); color: #fff; }
.aph-btn--sm { min-height: 36px; padding: 8px 14px; font-size: 14px; }
.aph-btn--lg { min-height: 52px; padding: 16px 24px; font-size: 17px; }
.aph-btn[disabled] { opacity: 0.5; cursor: not-allowed; }

.aph-input { font: inherit; min-height: 44px; padding: 12px 14px; border-radius: 8px; border: 1px solid var(--colors-border-primary); background: var(--colors-background-primary); color: var(--colors-text-primary); width: 100%; }
.aph-input:focus { outline: 2px solid var(--colors-primary-500); outline-offset: 1px; }
.aph-input--error { border-color: var(--colors-danger-500, #dc2626); }

.aph-tag { display: inline-flex; align-items: center; padding: 4px 10px; border-radius: 999px; background: var(--colors-primary-50, #f5f5f5); color: var(--colors-primary-700, #111); font-size: 12px; font-weight: 500; }
.aph-tag--success { background: var(--colors-success-50, #ecfdf5); color: var(--colors-success-700, #047857); }
.aph-tag--warning { background: var(--colors-warning-50, #fffbeb); color: var(--colors-warning-700, #b45309); }
.aph-tag--danger { background: var(--colors-danger-50, #fef2f2); color: var(--colors-danger-700, #b91c1c); }

.aph-avatar { width: 40px; height: 40px; border-radius: 999px; background: var(--colors-primary-200, #ddd); display: inline-flex; align-items: center; justify-content: center; font-weight: 600; color: var(--colors-text-primary, #111); flex-shrink: 0; }
.aph-avatar--sm { width: 32px; height: 32px; font-size: 13px; }
.aph-avatar--lg { width: 56px; height: 56px; font-size: 18px; }

.aph-card { padding: 16px; border: 1px solid var(--colors-border-primary, #e5e5e5); border-radius: 8px; background: var(--colors-background-primary, #fff); }
.aph-card--padded { padding: 24px; }

.aph-modal-backdrop { position: fixed; inset: 0; background: rgba(0,0,0,0.4); display: flex; align-items: center; justify-content: center; padding: 16px; z-index: 1000; }
.aph-modal { background: var(--colors-background-primary, #fff); color: var(--colors-text-primary, #111); border-radius: 12px; padding: 24px; max-width: 480px; width: 100%; max-height: 90vh; overflow: auto; }

.aph-drawer-backdrop { position: fixed; inset: 0; background: rgba(0,0,0,0.4); z-index: 1000; }
.aph-drawer { position: fixed; top: 0; right: 0; bottom: 0; width: min(420px, 100%); background: var(--colors-background-primary, #fff); padding: 24px; box-shadow: -8px 0 24px rgba(0,0,0,0.1); z-index: 1001; overflow: auto; }

.aph-skeleton { background: linear-gradient(90deg, var(--colors-background-secondary, #f5f5f5) 0%, var(--colors-border-primary, #e5e5e5) 50%, var(--colors-background-secondary, #f5f5f5) 100%); background-size: 200% 100%; border-radius: 6px; min-height: 16px; animation: aph-skeleton-shimmer 1.5s ease-in-out infinite; }
@keyframes aph-skeleton-shimmer { 0% { background-position: 200% 0; } 100% { background-position: -200% 0; } }

.aph-form-field { display: flex; flex-direction: column; gap: 6px; }
.aph-form-field__label { font-size: 13px; font-weight: 500; color: var(--colors-text-primary); }
.aph-form-field__hint { font-size: 12px; color: var(--colors-text-muted, #666); }
.aph-form-field__error { font-size: 12px; color: var(--colors-danger-500, #dc2626); }

.aph-switch { position: relative; width: 44px; height: 24px; background: var(--colors-border-primary); border-radius: 999px; cursor: pointer; transition: background 0.15s ease; flex-shrink: 0; border: none; padding: 0; }
.aph-switch[aria-checked="true"] { background: var(--colors-primary-500); }
.aph-switch::after { content: ""; position: absolute; top: 2px; left: 2px; width: 20px; height: 20px; background: #fff; border-radius: 999px; transition: transform 0.15s ease; }
.aph-switch[aria-checked="true"]::after { transform: translateX(20px); }

.aph-badge { display: inline-flex; align-items: center; min-width: 18px; height: 18px; padding: 0 6px; border-radius: 999px; background: var(--colors-danger-500, #dc2626); color: #fff; font-size: 11px; font-weight: 600; line-height: 1; }
.aph-badge--dot { width: 8px; height: 8px; min-width: 8px; padding: 0; }

.aph-spinner { width: 24px; height: 24px; border: 2px solid var(--colors-border-primary); border-top-color: var(--colors-primary-500); border-radius: 999px; animation: aph-spin 0.8s linear infinite; display: inline-block; }
@keyframes aph-spin { to { transform: rotate(360deg); } }
.aph-spinner--sm { width: 16px; height: 16px; border-width: 1.5px; }
.aph-spinner--lg { width: 32px; height: 32px; border-width: 3px; }
"#;

// ---- Component .tsx bodies ----

const BUTTON_TSX: &str = r#"import { forwardRef, ButtonHTMLAttributes, ReactNode } from "react";
import { cn } from "./cn";

export type ButtonVariant = "primary" | "secondary" | "ghost" | "danger";
export type ButtonSize = "sm" | "md" | "lg";

export interface ButtonProps extends Omit<ButtonHTMLAttributes<HTMLButtonElement>, "size"> {
  variant?: ButtonVariant;
  size?: ButtonSize;
  loading?: boolean;
  leftIcon?: ReactNode;
  rightIcon?: ReactNode;
}

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(function Button(
  { variant = "primary", size = "md", loading = false, leftIcon, rightIcon, disabled, children, className, ...rest },
  ref,
) {
  return (
    <button
      ref={ref}
      className={cn(
        "aph-btn",
        `aph-btn--${variant}`,
        size !== "md" && `aph-btn--${size}`,
        className,
      )}
      disabled={disabled || loading}
      aria-busy={loading || undefined}
      {...rest}
    >
      {leftIcon}
      {loading ? "…" : children}
      {rightIcon}
    </button>
  );
});
"#;

const INPUT_TSX: &str = r#"import { forwardRef, InputHTMLAttributes } from "react";
import { cn } from "./cn";

export interface InputProps extends InputHTMLAttributes<HTMLInputElement> {
  error?: boolean;
}

export const Input = forwardRef<HTMLInputElement, InputProps>(function Input(
  { error, className, ...rest },
  ref,
) {
  return (
    <input
      ref={ref}
      className={cn("aph-input", error && "aph-input--error", className)}
      aria-invalid={error || undefined}
      {...rest}
    />
  );
});
"#;

const TAG_TSX: &str = r#"import { forwardRef, HTMLAttributes, ReactNode } from "react";
import { cn } from "./cn";

export type TagTone = "neutral" | "success" | "warning" | "danger";

export interface TagProps extends HTMLAttributes<HTMLSpanElement> {
  tone?: TagTone;
  children: ReactNode;
}

export const Tag = forwardRef<HTMLSpanElement, TagProps>(function Tag(
  { tone = "neutral", className, children, ...rest },
  ref,
) {
  return (
    <span
      ref={ref}
      className={cn("aph-tag", tone !== "neutral" && `aph-tag--${tone}`, className)}
      {...rest}
    >
      {children}
    </span>
  );
});
"#;

const AVATAR_TSX: &str = r#"import { forwardRef, HTMLAttributes, ReactNode } from "react";
import { cn } from "./cn";

export type AvatarSize = "sm" | "md" | "lg";

export interface AvatarProps extends HTMLAttributes<HTMLSpanElement> {
  size?: AvatarSize;
  initials?: string;
  src?: string;
  alt?: string;
  children?: ReactNode;
}

export const Avatar = forwardRef<HTMLSpanElement, AvatarProps>(function Avatar(
  { size = "md", initials, src, alt, className, children, ...rest },
  ref,
) {
  if (src) {
    return (
      <span
        ref={ref}
        className={cn("aph-avatar", size !== "md" && `aph-avatar--${size}`, className)}
        {...rest}
      >
        <img src={src} alt={alt ?? ""} style={{ width: "100%", height: "100%", borderRadius: "999px", objectFit: "cover" }} />
      </span>
    );
  }
  return (
    <span
      ref={ref}
      className={cn("aph-avatar", size !== "md" && `aph-avatar--${size}`, className)}
      aria-label={alt}
      {...rest}
    >
      {initials ?? children}
    </span>
  );
});
"#;

const CARD_TSX: &str = r#"import { forwardRef, HTMLAttributes, ReactNode } from "react";
import { cn } from "./cn";

export interface CardProps extends HTMLAttributes<HTMLDivElement> {
  padded?: boolean;
  children: ReactNode;
}

export const Card = forwardRef<HTMLDivElement, CardProps>(function Card(
  { padded = true, className, children, ...rest },
  ref,
) {
  return (
    <div ref={ref} className={cn("aph-card", padded && "aph-card--padded", className)} {...rest}>
      {children}
    </div>
  );
});
"#;

const MODAL_TSX: &str = r#"import { useEffect, ReactNode } from "react";
import { cn } from "./cn";

export interface ModalProps {
  open: boolean;
  onClose: () => void;
  title?: ReactNode;
  children: ReactNode;
  className?: string;
}

export function Modal({ open, onClose, title, children, className }: ModalProps) {
  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  }, [open, onClose]);
  if (!open) return null;
  return (
    <div className="aph-modal-backdrop" onMouseDown={(e) => { if (e.target === e.currentTarget) onClose(); }}>
      <div
        role="dialog"
        aria-modal="true"
        aria-label={typeof title === "string" ? title : undefined}
        className={cn("aph-modal", className)}
      >
        {title && <h2 style={{ marginTop: 0 }}>{title}</h2>}
        {children}
      </div>
    </div>
  );
}
"#;

const DRAWER_TSX: &str = r#"import { useEffect, ReactNode } from "react";
import { cn } from "./cn";

export interface DrawerProps {
  open: boolean;
  onClose: () => void;
  children: ReactNode;
  className?: string;
}

export function Drawer({ open, onClose, children, className }: DrawerProps) {
  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  }, [open, onClose]);
  if (!open) return null;
  return (
    <>
      <div className="aph-drawer-backdrop" onClick={onClose} aria-hidden="true" />
      <aside role="dialog" aria-modal="true" className={cn("aph-drawer", className)}>
        {children}
      </aside>
    </>
  );
}
"#;

const SKELETON_TSX: &str = r#"import { CSSProperties } from "react";
import { cn } from "./cn";

export interface SkeletonProps {
  width?: number | string;
  height?: number | string;
  circle?: boolean;
  className?: string;
  style?: CSSProperties;
}

export function Skeleton({ width, height, circle, className, style }: SkeletonProps) {
  return (
    <span
      aria-hidden="true"
      className={cn("aph-skeleton", className)}
      style={{
        display: "inline-block",
        width: width ?? "100%",
        height: height ?? 16,
        borderRadius: circle ? "999px" : undefined,
        ...style,
      }}
    />
  );
}
"#;

const FORM_FIELD_TSX: &str = r#"import { ReactNode, ReactElement, useId } from "react";
import { cn } from "./cn";

export interface FormFieldProps {
  label?: ReactNode;
  hint?: ReactNode;
  error?: ReactNode;
  className?: string;
  /** A single form control (Input, Select, etc) — receives id+aria from the field. */
  children: ReactElement<{ id?: string; "aria-describedby"?: string; "aria-invalid"?: boolean }>;
}

export function FormField({ label, hint, error, className, children }: FormFieldProps) {
  const id = useId();
  const hintId = `${id}-hint`;
  const errorId = `${id}-error`;
  const describedBy = [hint ? hintId : null, error ? errorId : null].filter(Boolean).join(" ") || undefined;
  const child = {
    ...children,
    props: {
      ...children.props,
      id: children.props.id ?? id,
      "aria-describedby": describedBy,
      "aria-invalid": error ? true : undefined,
    },
  };
  return (
    <div className={cn("aph-form-field", className)}>
      {label && <label htmlFor={id} className="aph-form-field__label">{label}</label>}
      {child}
      {hint && !error && <span id={hintId} className="aph-form-field__hint">{hint}</span>}
      {error && <span id={errorId} className="aph-form-field__error" role="alert">{error}</span>}
    </div>
  );
}
"#;

const SWITCH_TSX: &str = r#"import { forwardRef, ButtonHTMLAttributes } from "react";
import { cn } from "./cn";

export interface SwitchProps extends Omit<ButtonHTMLAttributes<HTMLButtonElement>, "onChange" | "value"> {
  checked: boolean;
  onChange: (next: boolean) => void;
  label?: string;
}

export const Switch = forwardRef<HTMLButtonElement, SwitchProps>(function Switch(
  { checked, onChange, label, className, disabled, ...rest },
  ref,
) {
  return (
    <button
      ref={ref}
      type="button"
      role="switch"
      aria-checked={checked}
      aria-label={label}
      disabled={disabled}
      onClick={() => !disabled && onChange(!checked)}
      className={cn("aph-switch", className)}
      {...rest}
    />
  );
});
"#;

const BADGE_TSX: &str = r#"import { forwardRef, HTMLAttributes, ReactNode } from "react";
import { cn } from "./cn";

export interface BadgeProps extends HTMLAttributes<HTMLSpanElement> {
  /** Dot variant ignores `count` and renders an 8x8 indicator. */
  dot?: boolean;
  count?: number;
  max?: number;
  children?: ReactNode;
}

export const Badge = forwardRef<HTMLSpanElement, BadgeProps>(function Badge(
  { dot, count, max = 99, className, children, ...rest },
  ref,
) {
  if (dot) {
    return <span ref={ref} className={cn("aph-badge", "aph-badge--dot", className)} aria-hidden="true" {...rest} />;
  }
  const label = typeof count === "number" ? (count > max ? `${max}+` : `${count}`) : children;
  return (
    <span ref={ref} className={cn("aph-badge", className)} {...rest}>{label}</span>
  );
});
"#;

const SPINNER_TSX: &str = r#"import { HTMLAttributes } from "react";
import { cn } from "./cn";

export type SpinnerSize = "sm" | "md" | "lg";

export interface SpinnerProps extends HTMLAttributes<HTMLSpanElement> {
  size?: SpinnerSize;
  label?: string;
}

export function Spinner({ size = "md", label = "Loading", className, ...rest }: SpinnerProps) {
  return (
    <span
      role="status"
      aria-label={label}
      className={cn("aph-spinner", size !== "md" && `aph-spinner--${size}`, className)}
      {...rest}
    />
  );
}
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use aphrodite_core::variant::{Variant, VariantKind};
    use std::collections::BTreeMap;

    fn fixture() -> Vec<Variant> {
        let mut t = BTreeMap::new();
        t.insert("colors.primary.500".into(), "#16a34a".into());
        t.insert("colors.background.primary".into(), "#ffffff".into());
        t.insert("spacing.4".into(), "16px".into());
        vec![
            Variant { kind: VariantKind::Light, tokens: t.clone() },
            Variant { kind: VariantKind::Dark, tokens: t },
        ]
    }

    #[test]
    fn build_produces_expected_file_set() {
        let pkg = build(&fixture(), "test-project");
        let must_have = [
            "package.json",
            "tsconfig.json",
            "README.md",
            "src/tokens.ts",
            "src/index.ts",
            "src/cn.ts",
            "src/Button.tsx",
            "src/Input.tsx",
            "src/Tag.tsx",
            "src/Avatar.tsx",
            "src/Card.tsx",
            "src/Modal.tsx",
            "src/Drawer.tsx",
            "src/Skeleton.tsx",
            "src/FormField.tsx",
            "src/Switch.tsx",
            "src/Badge.tsx",
            "src/Spinner.tsx",
            "src/styles.css",
        ];
        for p in must_have {
            assert!(pkg.files.contains_key(p), "missing file: {p}");
        }
    }

    #[test]
    fn package_json_has_react_peer_dep_and_exports_map() {
        let pkg = build(&fixture(), "test");
        let pkgjson = pkg.files.get("package.json").unwrap();
        assert!(pkgjson.contains("\"react\": \">=18\""));
        assert!(pkgjson.contains("\"./styles.css\""));
        assert!(pkgjson.contains("\"name\": \"@aphrodite/test\""));
    }

    #[test]
    fn tokens_ts_has_variant_union_and_const_object() {
        let pkg = build(&fixture(), "x");
        let ts = pkg.files.get("src/tokens.ts").unwrap();
        assert!(ts.contains("export type VariantName"));
        assert!(ts.contains("\"light\""));
        assert!(ts.contains("\"dark\""));
        assert!(ts.contains("export const tokens"));
        assert!(ts.contains("colors.primary.500"));
    }

    #[test]
    fn index_ts_re_exports_every_component() {
        let pkg = build(&fixture(), "x");
        let idx = pkg.files.get("src/index.ts").unwrap();
        for c in [
            "Button", "Input", "Tag", "Avatar", "Card",
            "Modal", "Drawer", "Skeleton", "FormField",
            "Switch", "Badge", "Spinner",
        ] {
            assert!(idx.contains(c), "barrel missing component: {c}");
        }
    }
}
