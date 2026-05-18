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
    /// Files in this set are user-editable and preserved on subsequent
    /// `aphrodite react` runs (only seeded the first time).
    const PRESERVE_IF_EXISTS: &'static [&'static str] = &["CHANGELOG.md"];

    pub fn write_to(&self, root: &std::path::Path) -> std::io::Result<()> {
        for (rel, contents) in &self.files {
            let full = root.join(rel);
            if let Some(parent) = full.parent() {
                std::fs::create_dir_all(parent)?;
            }
            if Self::PRESERVE_IF_EXISTS.contains(&rel.as_str()) && full.exists() {
                continue;
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
    // v1.0 RC additional primitives — 18 more to reach Toss/Karrot
    // coverage. Form controls, navigation, feedback, overlays.
    files.insert("src/Textarea.tsx".into(), TEXTAREA_TSX.into());
    files.insert("src/Select.tsx".into(), SELECT_TSX.into());
    files.insert("src/Checkbox.tsx".into(), CHECKBOX_TSX.into());
    files.insert("src/Radio.tsx".into(), RADIO_TSX.into());
    files.insert("src/RadioGroup.tsx".into(), RADIO_GROUP_TSX.into());
    files.insert("src/Tabs.tsx".into(), TABS_TSX.into());
    files.insert("src/Accordion.tsx".into(), ACCORDION_TSX.into());
    files.insert("src/Toast.tsx".into(), TOAST_TSX.into());
    files.insert("src/Tooltip.tsx".into(), TOOLTIP_TSX.into());
    files.insert("src/Popover.tsx".into(), POPOVER_TSX.into());
    files.insert("src/Menu.tsx".into(), MENU_TSX.into());
    files.insert("src/ProgressBar.tsx".into(), PROGRESS_BAR_TSX.into());
    files.insert("src/Stepper.tsx".into(), STEPPER_TSX.into());
    files.insert("src/Slider.tsx".into(), SLIDER_TSX.into());
    files.insert("src/Breadcrumb.tsx".into(), BREADCRUMB_TSX.into());
    files.insert("src/Pagination.tsx".into(), PAGINATION_TSX.into());
    files.insert("src/Divider.tsx".into(), DIVIDER_TSX.into());
    files.insert("src/EmptyState.tsx".into(), EMPTY_STATE_TSX.into());
    // v1.0 RC.5: 12 advanced primitives bringing total to 42. Priorities
    // chosen for Korean fintech / marketplace product surface area.
    files.insert("src/SegmentedControl.tsx".into(), SEGMENTED_CONTROL_TSX.into());
    files.insert("src/PinInput.tsx".into(), PIN_INPUT_TSX.into());
    files.insert("src/NumberInput.tsx".into(), NUMBER_INPUT_TSX.into());
    files.insert("src/SearchInput.tsx".into(), SEARCH_INPUT_TSX.into());
    files.insert("src/FileUploader.tsx".into(), FILE_UPLOADER_TSX.into());
    files.insert("src/DatePicker.tsx".into(), DATE_PICKER_TSX.into());
    files.insert("src/Combobox.tsx".into(), COMBOBOX_TSX.into());
    files.insert("src/Sheet.tsx".into(), SHEET_TSX.into());
    files.insert("src/Alert.tsx".into(), ALERT_TSX.into());
    files.insert("src/Stat.tsx".into(), STAT_TSX.into());
    files.insert("src/Toolbar.tsx".into(), TOOLBAR_TSX.into());
    files.insert("src/HoverCard.tsx".into(), HOVER_CARD_TSX.into());
    // v1.0 RC.7: 12 advanced primitives — DataTable, Carousel, Calendar,
    // and supporting ones for 0-1 brand pages (Chip, Image, KBD, Code,
    // Timeline, Disclosure, ContextMenu, Command).
    files.insert("src/DataTable.tsx".into(), DATA_TABLE_TSX.into());
    files.insert("src/Carousel.tsx".into(), CAROUSEL_TSX.into());
    files.insert("src/Calendar.tsx".into(), CALENDAR_TSX.into());
    files.insert("src/Chip.tsx".into(), CHIP_TSX.into());
    files.insert("src/Image.tsx".into(), IMAGE_TSX.into());
    files.insert("src/Kbd.tsx".into(), KBD_TSX.into());
    files.insert("src/Code.tsx".into(), CODE_TSX.into());
    files.insert("src/Timeline.tsx".into(), TIMELINE_TSX.into());
    files.insert("src/Disclosure.tsx".into(), DISCLOSURE_TSX.into());
    files.insert("src/ContextMenu.tsx".into(), CONTEXT_MENU_TSX.into());
    files.insert("src/Command.tsx".into(), COMMAND_TSX.into());
    files.insert("src/Hint.tsx".into(), HINT_TSX.into());
    files.insert("src/PhoneInputKR.tsx".into(), PHONE_INPUT_KR_TSX.into());
    files.insert("src/index.ts".into(), build_index_ts());
    files.insert("src/styles.css".into(), build_styles_css(variants));
    files.insert(".npmignore".into(), build_npmignore());
    files.insert(".github/workflows/publish.yml".into(), build_publish_workflow());
    files.insert("CHANGELOG.md".into(), build_changelog_seed(&kebab));
    // Storybook stories (CSF3 format) for each component. Optional —
    // consumer Storybook config picks them up automatically.
    for (name, body) in [
        ("Button", BUTTON_STORIES),
        ("Input", INPUT_STORIES),
        ("Tag", TAG_STORIES),
        ("Avatar", AVATAR_STORIES),
        ("Card", CARD_STORIES),
        ("Modal", MODAL_STORIES),
        ("Drawer", DRAWER_STORIES),
        ("Skeleton", SKELETON_STORIES),
        ("FormField", FORM_FIELD_STORIES),
        ("Switch", SWITCH_STORIES),
        ("Badge", BADGE_STORIES),
        ("Spinner", SPINNER_STORIES),
        ("Textarea", TEXTAREA_STORIES),
        ("Select", SELECT_STORIES),
        ("Checkbox", CHECKBOX_STORIES),
        ("Radio", RADIO_STORIES),
        ("RadioGroup", RADIO_GROUP_STORIES),
        ("Tabs", TABS_STORIES),
        ("Accordion", ACCORDION_STORIES),
        ("Toast", TOAST_STORIES),
        ("Tooltip", TOOLTIP_STORIES),
        ("Popover", POPOVER_STORIES),
        ("Menu", MENU_STORIES),
        ("ProgressBar", PROGRESS_BAR_STORIES),
        ("Stepper", STEPPER_STORIES),
        ("Slider", SLIDER_STORIES),
        ("Breadcrumb", BREADCRUMB_STORIES),
        ("Pagination", PAGINATION_STORIES),
        ("Divider", DIVIDER_STORIES),
        ("EmptyState", EMPTY_STATE_STORIES),
        ("SegmentedControl", SEGMENTED_CONTROL_STORIES),
        ("PinInput", PIN_INPUT_STORIES),
        ("NumberInput", NUMBER_INPUT_STORIES),
        ("SearchInput", SEARCH_INPUT_STORIES),
        ("FileUploader", FILE_UPLOADER_STORIES),
        ("DatePicker", DATE_PICKER_STORIES),
        ("Combobox", COMBOBOX_STORIES),
        ("Sheet", SHEET_STORIES),
        ("Alert", ALERT_STORIES),
        ("Stat", STAT_STORIES),
        ("Toolbar", TOOLBAR_STORIES),
        ("HoverCard", HOVER_CARD_STORIES),
        ("PhoneInputKR", PHONE_INPUT_KR_STORIES),
        ("DataTable", DATA_TABLE_STORIES),
        ("Carousel", CAROUSEL_STORIES),
        ("Calendar", CALENDAR_STORIES),
        ("Chip", CHIP_STORIES),
        ("Image", IMAGE_STORIES),
        ("Kbd", KBD_STORIES),
        ("Code", CODE_STORIES),
        ("Timeline", TIMELINE_STORIES),
        ("Disclosure", DISCLOSURE_STORIES),
        ("ContextMenu", CONTEXT_MENU_STORIES),
        ("Command", COMMAND_STORIES),
        ("Hint", HINT_STORIES),
    ] {
        files.insert(format!("src/{name}.stories.tsx"), body.into());
    }
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
- 30 primitive components: Button, Input, Textarea, Select, Checkbox,
  Radio, RadioGroup, Tag, Badge, Avatar, Card, Modal, Drawer, Popover,
  Tooltip, Menu, Tabs, Accordion, Toast (+ ToastProvider/useToast),
  ProgressBar, Stepper, Slider, Breadcrumb, Pagination, Divider,
  EmptyState, Skeleton, FormField, Switch, Spinner.

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

fn build_npmignore() -> String {
    r#"# auto-generated by aphrodite — do not edit
*.stories.tsx
.github/
src/**/*.test.*
node_modules/
.DS_Store
"#
    .into()
}

fn build_publish_workflow() -> String {
    r#"# auto-generated by aphrodite — do not edit
# Push a tag matching v*.*.* to publish this package to npm.
# Requires NPM_TOKEN repo secret.
name: publish
on:
  push:
    tags:
      - "v*.*.*"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: "20"
          registry-url: "https://registry.npmjs.org"
      - run: npm ci || npm install
      - run: npm run build
      - run: npm publish --access public
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
"#
    .into()
}

fn build_changelog_seed(name: &str) -> String {
    format!(
        r#"# Changelog — @aphrodite/{name}

This file is auto-generated on first emit. Append entries by hand
when you cut a version — `aphrodite react` will preserve subsequent
edits because writes are idempotent only on the build artifacts under
`src/`, not on this file.

## 0.1.0 — initial

- 30 typed React primitives
- 4 brand variants via `body[data-variant]` scope
- Storybook CSF3 stories per component
"#
    )
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
export { Textarea } from "./Textarea";
export type { TextareaProps } from "./Textarea";
export { Select } from "./Select";
export type { SelectProps } from "./Select";
export { Checkbox } from "./Checkbox";
export type { CheckboxProps } from "./Checkbox";
export { Radio } from "./Radio";
export type { RadioProps } from "./Radio";
export { RadioGroup } from "./RadioGroup";
export type { RadioGroupProps } from "./RadioGroup";
export { Tabs, TabList, Tab, TabPanel } from "./Tabs";
export type { TabsProps } from "./Tabs";
export { Accordion, AccordionItem } from "./Accordion";
export type { AccordionProps, AccordionItemProps } from "./Accordion";
export { Toast, ToastProvider, useToast } from "./Toast";
export type { ToastProps } from "./Toast";
export { Tooltip } from "./Tooltip";
export type { TooltipProps } from "./Tooltip";
export { Popover } from "./Popover";
export type { PopoverProps } from "./Popover";
export { Menu, MenuItem } from "./Menu";
export type { MenuProps, MenuItemProps } from "./Menu";
export { ProgressBar } from "./ProgressBar";
export type { ProgressBarProps } from "./ProgressBar";
export { Stepper } from "./Stepper";
export type { StepperProps } from "./Stepper";
export { Slider } from "./Slider";
export type { SliderProps } from "./Slider";
export { Breadcrumb } from "./Breadcrumb";
export type { BreadcrumbProps } from "./Breadcrumb";
export { Pagination } from "./Pagination";
export type { PaginationProps } from "./Pagination";
export { Divider } from "./Divider";
export type { DividerProps } from "./Divider";
export { EmptyState } from "./EmptyState";
export type { EmptyStateProps } from "./EmptyState";
export { SegmentedControl } from "./SegmentedControl";
export type { SegmentedControlProps } from "./SegmentedControl";
export { PinInput } from "./PinInput";
export type { PinInputProps } from "./PinInput";
export { NumberInput } from "./NumberInput";
export type { NumberInputProps } from "./NumberInput";
export { SearchInput } from "./SearchInput";
export type { SearchInputProps } from "./SearchInput";
export { FileUploader } from "./FileUploader";
export type { FileUploaderProps } from "./FileUploader";
export { DatePicker } from "./DatePicker";
export type { DatePickerProps } from "./DatePicker";
export { Combobox } from "./Combobox";
export type { ComboboxProps, ComboboxOption } from "./Combobox";
export { Sheet } from "./Sheet";
export type { SheetProps } from "./Sheet";
export { Alert } from "./Alert";
export type { AlertProps } from "./Alert";
export { Stat } from "./Stat";
export type { StatProps } from "./Stat";
export { Toolbar } from "./Toolbar";
export type { ToolbarProps } from "./Toolbar";
export { HoverCard } from "./HoverCard";
export type { HoverCardProps } from "./HoverCard";
export { PhoneInputKR, formatKoreanPhone, parseKoreanPhone } from "./PhoneInputKR";
export type { PhoneInputKRProps } from "./PhoneInputKR";
export { DataTable } from "./DataTable";
export type { DataTableProps, DataTableColumn } from "./DataTable";
export { Carousel } from "./Carousel";
export type { CarouselProps } from "./Carousel";
export { Calendar } from "./Calendar";
export type { CalendarProps } from "./Calendar";
export { Chip } from "./Chip";
export type { ChipProps } from "./Chip";
export { Image } from "./Image";
export type { ImageProps } from "./Image";
export { Kbd } from "./Kbd";
export type { KbdProps } from "./Kbd";
export { Code } from "./Code";
export type { CodeProps } from "./Code";
export { Timeline, TimelineItem } from "./Timeline";
export type { TimelineProps, TimelineItemProps } from "./Timeline";
export { Disclosure } from "./Disclosure";
export type { DisclosureProps } from "./Disclosure";
export { ContextMenu } from "./ContextMenu";
export type { ContextMenuProps } from "./ContextMenu";
export { Command } from "./Command";
export type { CommandProps, CommandItem } from "./Command";
export { Hint } from "./Hint";
export type { HintProps } from "./Hint";
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

/* extended primitives */
.aph-textarea { font: inherit; min-height: 96px; padding: 12px 14px; border-radius: 8px; border: 1px solid var(--colors-border-primary); background: var(--colors-background-primary); color: var(--colors-text-primary); width: 100%; resize: vertical; line-height: 1.5; }
.aph-textarea:focus { outline: 2px solid var(--colors-primary-500); outline-offset: 1px; }

.aph-select { font: inherit; min-height: 44px; padding: 12px 36px 12px 14px; border-radius: 8px; border: 1px solid var(--colors-border-primary); background: var(--colors-background-primary); color: var(--colors-text-primary); width: 100%; appearance: none; background-image: linear-gradient(45deg, transparent 50%, currentColor 50%), linear-gradient(135deg, currentColor 50%, transparent 50%); background-position: calc(100% - 18px) 50%, calc(100% - 13px) 50%; background-size: 5px 5px; background-repeat: no-repeat; }

.aph-checkbox { display: inline-flex; align-items: center; gap: 8px; cursor: pointer; user-select: none; }
.aph-checkbox input { width: 20px; height: 20px; accent-color: var(--colors-primary-500); cursor: pointer; }
.aph-checkbox--disabled { opacity: 0.5; cursor: not-allowed; }

.aph-radio { display: inline-flex; align-items: center; gap: 8px; cursor: pointer; user-select: none; }
.aph-radio input { width: 20px; height: 20px; accent-color: var(--colors-primary-500); cursor: pointer; }
.aph-radio-group { display: flex; flex-direction: column; gap: 8px; }
.aph-radio-group--horizontal { flex-direction: row; flex-wrap: wrap; gap: 16px; }

.aph-tabs { display: flex; flex-direction: column; }
.aph-tab-list { display: flex; gap: 4px; border-bottom: 1px solid var(--colors-border-primary); }
.aph-tab { font: inherit; padding: 12px 16px; background: transparent; border: none; cursor: pointer; color: var(--colors-text-muted); border-bottom: 2px solid transparent; margin-bottom: -1px; font-weight: 500; min-height: 44px; }
.aph-tab[aria-selected="true"] { color: var(--colors-text-primary); border-bottom-color: var(--colors-primary-500); }
.aph-tab-panel { padding: 24px 0; }

.aph-accordion { display: flex; flex-direction: column; }
.aph-accordion-item { border-bottom: 1px solid var(--colors-border-primary); }
.aph-accordion-trigger { font: inherit; width: 100%; text-align: left; padding: 16px 0; background: transparent; border: none; cursor: pointer; display: flex; justify-content: space-between; align-items: center; color: var(--colors-text-primary); font-weight: 500; min-height: 44px; }
.aph-accordion-content { padding: 0 0 16px; color: var(--colors-text-muted); }
.aph-accordion-trigger[aria-expanded="true"] .aph-accordion-chevron { transform: rotate(180deg); }
.aph-accordion-chevron { transition: transform 0.15s ease; }

.aph-toast-region { position: fixed; bottom: 24px; left: 50%; transform: translateX(-50%); display: flex; flex-direction: column; gap: 8px; z-index: 9999; pointer-events: none; }
.aph-toast { background: var(--colors-text-primary); color: var(--colors-background-primary); padding: 12px 18px; border-radius: 8px; font-size: 14px; pointer-events: auto; box-shadow: 0 8px 24px rgba(0,0,0,0.18); min-width: 240px; max-width: 80vw; }
.aph-toast--success { background: var(--colors-success-600, #16a34a); color: #fff; }
.aph-toast--danger { background: var(--colors-danger-600, #dc2626); color: #fff; }

.aph-tooltip-wrap { position: relative; display: inline-flex; }
.aph-tooltip { position: absolute; bottom: calc(100% + 6px); left: 50%; transform: translateX(-50%); background: var(--colors-text-primary); color: var(--colors-background-primary); padding: 6px 10px; border-radius: 6px; font-size: 12px; white-space: nowrap; pointer-events: none; opacity: 0; transition: opacity 0.1s ease; }
.aph-tooltip-wrap:hover .aph-tooltip, .aph-tooltip-wrap:focus-within .aph-tooltip { opacity: 1; }

.aph-popover-wrap { position: relative; display: inline-block; }
.aph-popover { position: absolute; top: calc(100% + 6px); left: 0; min-width: 200px; background: var(--colors-background-primary); border: 1px solid var(--colors-border-primary); border-radius: 8px; box-shadow: 0 8px 24px rgba(0,0,0,0.08); padding: 12px; z-index: 100; }

.aph-menu { background: var(--colors-background-primary); border: 1px solid var(--colors-border-primary); border-radius: 8px; box-shadow: 0 8px 24px rgba(0,0,0,0.08); padding: 4px; min-width: 200px; }
.aph-menu-item { font: inherit; width: 100%; text-align: left; background: transparent; border: none; cursor: pointer; padding: 10px 12px; border-radius: 4px; color: var(--colors-text-primary); display: flex; align-items: center; gap: 8px; min-height: 40px; }
.aph-menu-item:hover, .aph-menu-item:focus { background: var(--colors-background-secondary); outline: none; }
.aph-menu-item[disabled] { opacity: 0.5; cursor: not-allowed; }

.aph-progress { width: 100%; height: 8px; background: var(--colors-border-primary); border-radius: 999px; overflow: hidden; }
.aph-progress__bar { height: 100%; background: var(--colors-primary-500); transition: width 0.3s ease; }

.aph-stepper { display: flex; align-items: center; gap: 12px; }
.aph-stepper-step { display: flex; align-items: center; gap: 8px; color: var(--colors-text-muted); }
.aph-stepper-step--active { color: var(--colors-text-primary); font-weight: 600; }
.aph-stepper-step--done { color: var(--colors-primary-500); }
.aph-stepper-bubble { width: 24px; height: 24px; border-radius: 999px; background: var(--colors-border-primary); display: inline-flex; align-items: center; justify-content: center; font-size: 13px; font-weight: 600; }
.aph-stepper-step--active .aph-stepper-bubble { background: var(--colors-primary-500); color: #fff; }
.aph-stepper-step--done .aph-stepper-bubble { background: var(--colors-primary-500); color: #fff; }
.aph-stepper-line { flex: 1; height: 2px; background: var(--colors-border-primary); }

.aph-slider { width: 100%; accent-color: var(--colors-primary-500); }

.aph-breadcrumb { display: flex; align-items: center; gap: 6px; font-size: 13px; color: var(--colors-text-muted); flex-wrap: wrap; }
.aph-breadcrumb a { color: var(--colors-text-muted); text-decoration: none; }
.aph-breadcrumb a:hover { color: var(--colors-text-primary); text-decoration: underline; }
.aph-breadcrumb__current { color: var(--colors-text-primary); font-weight: 500; }
.aph-breadcrumb__sep { color: var(--colors-text-muted); }

.aph-pagination { display: flex; align-items: center; gap: 4px; flex-wrap: wrap; }
.aph-pagination__btn { font: inherit; min-width: 36px; min-height: 36px; padding: 6px 10px; border-radius: 6px; border: 1px solid var(--colors-border-primary); background: var(--colors-background-primary); color: var(--colors-text-primary); cursor: pointer; }
.aph-pagination__btn[aria-current="page"] { background: var(--colors-primary-500); color: var(--colors-background-primary); border-color: var(--colors-primary-500); }
.aph-pagination__btn[disabled] { opacity: 0.4; cursor: not-allowed; }

.aph-divider { border: none; border-top: 1px solid var(--colors-border-primary); margin: 16px 0; }
.aph-divider--vertical { border-top: none; border-left: 1px solid var(--colors-border-primary); height: 100%; width: 1px; margin: 0 16px; display: inline-block; }

.aph-empty { display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 48px 24px; text-align: center; gap: 12px; color: var(--colors-text-muted); }
.aph-empty__title { color: var(--colors-text-primary); font-weight: 600; font-size: 18px; }

/* advanced primitives (RC.5) */
.aph-segmented { display: inline-flex; padding: 4px; background: var(--colors-background-secondary); border-radius: 8px; gap: 2px; }
.aph-segmented__btn { font: inherit; padding: 8px 14px; border: none; background: transparent; cursor: pointer; border-radius: 6px; color: var(--colors-text-muted); min-height: 40px; font-weight: 500; }
.aph-segmented__btn[aria-pressed="true"] { background: var(--colors-background-primary); color: var(--colors-text-primary); box-shadow: 0 1px 3px rgba(0,0,0,0.06); }

.aph-pin { display: inline-flex; gap: 8px; }
.aph-pin__digit { font: inherit; width: 44px; height: 52px; text-align: center; font-size: 20px; font-weight: 600; border-radius: 8px; border: 1px solid var(--colors-border-primary); background: var(--colors-background-primary); color: var(--colors-text-primary); }
.aph-pin__digit:focus { outline: 2px solid var(--colors-primary-500); outline-offset: 1px; }

.aph-number-input { display: inline-flex; align-items: center; gap: 4px; border: 1px solid var(--colors-border-primary); border-radius: 8px; padding: 4px; background: var(--colors-background-primary); }
.aph-number-input__btn { font: inherit; width: 32px; height: 32px; border: none; background: transparent; cursor: pointer; border-radius: 4px; color: var(--colors-text-primary); }
.aph-number-input__btn:hover { background: var(--colors-background-secondary); }
.aph-number-input__btn[disabled] { opacity: 0.4; cursor: not-allowed; }
.aph-number-input__field { font: inherit; width: 64px; text-align: center; border: none; background: transparent; color: var(--colors-text-primary); min-height: 32px; }
.aph-number-input__field:focus { outline: none; }

.aph-search { position: relative; display: inline-block; width: 100%; }
.aph-search__input { padding-right: 36px; }
.aph-search__clear { position: absolute; right: 8px; top: 50%; transform: translateY(-50%); border: none; background: transparent; cursor: pointer; padding: 4px 8px; color: var(--colors-text-muted); font-size: 16px; }

.aph-uploader { border: 2px dashed var(--colors-border-primary); border-radius: 12px; padding: 32px 24px; text-align: center; cursor: pointer; transition: border-color 0.15s ease, background 0.15s ease; display: flex; flex-direction: column; align-items: center; gap: 8px; color: var(--colors-text-muted); }
.aph-uploader:hover, .aph-uploader[data-drag="true"] { border-color: var(--colors-primary-500); background: var(--colors-background-secondary); color: var(--colors-text-primary); }
.aph-uploader[data-disabled="true"] { opacity: 0.5; pointer-events: none; }

.aph-date-input { position: relative; }
.aph-date-input input { padding-right: 36px; }

.aph-combobox { position: relative; }
.aph-combobox__list { position: absolute; top: calc(100% + 4px); left: 0; right: 0; max-height: 240px; overflow: auto; background: var(--colors-background-primary); border: 1px solid var(--colors-border-primary); border-radius: 8px; box-shadow: 0 8px 24px rgba(0,0,0,0.08); z-index: 100; padding: 4px; }
.aph-combobox__option { padding: 8px 10px; cursor: pointer; border-radius: 4px; min-height: 36px; display: flex; align-items: center; }
.aph-combobox__option[aria-selected="true"], .aph-combobox__option:hover { background: var(--colors-background-secondary); }

.aph-sheet-backdrop { position: fixed; inset: 0; background: rgba(0,0,0,0.4); z-index: 1000; }
.aph-sheet { position: fixed; bottom: 0; left: 0; right: 0; background: var(--colors-background-primary); color: var(--colors-text-primary); border-top-left-radius: 16px; border-top-right-radius: 16px; padding: 24px 20px; max-height: 90vh; overflow: auto; z-index: 1001; }
.aph-sheet__handle { width: 36px; height: 4px; background: var(--colors-border-primary); border-radius: 999px; margin: -8px auto 16px; }

.aph-alert { display: flex; gap: 12px; padding: 14px 16px; border-radius: 8px; border: 1px solid var(--colors-border-primary); background: var(--colors-background-secondary); align-items: flex-start; }
.aph-alert--info { border-color: var(--colors-primary-300); background: var(--colors-primary-50); color: var(--colors-primary-900); }
.aph-alert--success { border-color: var(--colors-success-300, #6ee7b7); background: var(--colors-success-50, #ecfdf5); color: var(--colors-success-900, #064e3b); }
.aph-alert--warning { border-color: var(--colors-warning-300, #fcd34d); background: var(--colors-warning-50, #fffbeb); color: var(--colors-warning-900, #78350f); }
.aph-alert--danger { border-color: var(--colors-danger-300, #fca5a5); background: var(--colors-danger-50, #fef2f2); color: var(--colors-danger-900, #7f1d1d); }
.aph-alert__title { font-weight: 600; margin: 0; }

.aph-stat { display: flex; flex-direction: column; gap: 4px; }
.aph-stat__label { font-size: 13px; color: var(--colors-text-muted); }
.aph-stat__value { font-size: 28px; font-weight: 700; color: var(--colors-text-primary); letter-spacing: -0.02em; }
.aph-stat__delta { font-size: 13px; font-weight: 500; }
.aph-stat__delta--up { color: var(--colors-success-600, #16a34a); }
.aph-stat__delta--down { color: var(--colors-danger-600, #dc2626); }

.aph-toolbar { display: flex; align-items: center; gap: 4px; padding: 6px; background: var(--colors-background-primary); border: 1px solid var(--colors-border-primary); border-radius: 8px; flex-wrap: wrap; }
.aph-toolbar__sep { width: 1px; height: 24px; background: var(--colors-border-primary); margin: 0 4px; }

.aph-hover-wrap { position: relative; display: inline-block; }
.aph-hover-card { position: absolute; bottom: calc(100% + 8px); left: 0; min-width: 240px; background: var(--colors-background-primary); border: 1px solid var(--colors-border-primary); border-radius: 8px; box-shadow: 0 8px 24px rgba(0,0,0,0.08); padding: 14px; opacity: 0; pointer-events: none; transition: opacity 0.12s ease; z-index: 100; }
.aph-hover-wrap:hover .aph-hover-card, .aph-hover-wrap:focus-within .aph-hover-card { opacity: 1; pointer-events: auto; }

/* advanced primitives (RC.7) */
.aph-table-wrap { width: 100%; overflow-x: auto; border: 1px solid var(--colors-border-primary); border-radius: 8px; }
.aph-table { width: 100%; border-collapse: collapse; font-size: 14px; }
.aph-table th, .aph-table td { padding: 12px 16px; text-align: left; border-bottom: 1px solid var(--colors-border-primary); }
.aph-table th { background: var(--colors-background-secondary); font-weight: 600; color: var(--colors-text-primary); font-size: 13px; }
.aph-table th[aria-sort] { cursor: pointer; user-select: none; }
.aph-table th[aria-sort]:hover { background: var(--colors-border-primary); }
.aph-table tr:last-child td { border-bottom: none; }
.aph-table tr:hover td { background: var(--colors-background-secondary); }
.aph-table__sort-icon { font-size: 11px; margin-left: 4px; color: var(--colors-text-muted); }
.aph-table__sort-icon--active { color: var(--colors-primary-500); }

.aph-carousel { position: relative; overflow: hidden; border-radius: 12px; }
.aph-carousel__track { display: flex; transition: transform 0.3s ease; }
.aph-carousel__slide { flex: 0 0 100%; }
.aph-carousel__btn { position: absolute; top: 50%; transform: translateY(-50%); width: 40px; height: 40px; border-radius: 999px; background: var(--colors-background-primary); border: 1px solid var(--colors-border-primary); cursor: pointer; display: inline-flex; align-items: center; justify-content: center; font-size: 18px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }
.aph-carousel__btn--prev { left: 12px; }
.aph-carousel__btn--next { right: 12px; }
.aph-carousel__dots { position: absolute; bottom: 12px; left: 50%; transform: translateX(-50%); display: flex; gap: 6px; }
.aph-carousel__dot { width: 8px; height: 8px; border-radius: 999px; background: rgba(255,255,255,0.6); border: none; padding: 0; cursor: pointer; }
.aph-carousel__dot[aria-current="true"] { background: #fff; transform: scale(1.3); }

.aph-calendar { display: inline-flex; flex-direction: column; gap: 8px; padding: 12px; background: var(--colors-background-primary); border: 1px solid var(--colors-border-primary); border-radius: 8px; font-size: 14px; }
.aph-calendar__head { display: flex; justify-content: space-between; align-items: center; padding: 0 4px; }
.aph-calendar__month { font-weight: 600; }
.aph-calendar__nav { background: transparent; border: none; cursor: pointer; padding: 4px 8px; font-size: 16px; color: var(--colors-text-primary); border-radius: 4px; }
.aph-calendar__nav:hover { background: var(--colors-background-secondary); }
.aph-calendar__grid { display: grid; grid-template-columns: repeat(7, 36px); gap: 2px; }
.aph-calendar__dow { text-align: center; font-size: 11px; color: var(--colors-text-muted); padding: 4px 0; font-weight: 500; }
.aph-calendar__cell { width: 36px; height: 36px; border: none; background: transparent; border-radius: 999px; cursor: pointer; font: inherit; color: var(--colors-text-primary); }
.aph-calendar__cell:hover:not([disabled]) { background: var(--colors-background-secondary); }
.aph-calendar__cell--other { color: var(--colors-text-muted); opacity: 0.4; }
.aph-calendar__cell--today { font-weight: 700; color: var(--colors-primary-500); }
.aph-calendar__cell--selected { background: var(--colors-primary-500); color: #fff; font-weight: 600; }
.aph-calendar__cell--selected:hover:not([disabled]) { background: var(--colors-primary-600, var(--colors-primary-500)); }

.aph-chip { display: inline-flex; align-items: center; gap: 6px; padding: 4px 4px 4px 10px; border-radius: 999px; background: var(--colors-background-secondary); color: var(--colors-text-primary); font-size: 13px; min-height: 28px; }
.aph-chip__close { width: 20px; height: 20px; border-radius: 999px; border: none; background: transparent; cursor: pointer; display: inline-flex; align-items: center; justify-content: center; color: var(--colors-text-muted); font-size: 14px; line-height: 1; }
.aph-chip__close:hover { background: var(--colors-border-primary); color: var(--colors-text-primary); }

.aph-image { display: inline-block; position: relative; overflow: hidden; background: var(--colors-background-secondary); }
.aph-image img { display: block; width: 100%; height: 100%; object-fit: cover; }
.aph-image__fallback { position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; color: var(--colors-text-muted); font-size: 12px; }

.aph-kbd { display: inline-flex; align-items: center; padding: 2px 6px; border-radius: 4px; border: 1px solid var(--colors-border-primary); background: var(--colors-background-secondary); font-family: ui-monospace, SFMono-Regular, monospace; font-size: 11px; font-weight: 600; color: var(--colors-text-primary); min-width: 18px; height: 20px; justify-content: center; }

.aph-code { font-family: ui-monospace, SFMono-Regular, monospace; font-size: 12px; background: var(--colors-background-secondary); padding: 2px 6px; border-radius: 4px; color: var(--colors-text-primary); }
.aph-code-block { background: var(--colors-background-secondary); padding: 14px 16px; border-radius: 8px; overflow-x: auto; font-family: ui-monospace, SFMono-Regular, monospace; font-size: 12px; }

.aph-timeline { display: flex; flex-direction: column; gap: 0; }
.aph-timeline-item { display: flex; gap: 16px; position: relative; padding-bottom: 24px; }
.aph-timeline-item:not(:last-child)::before { content: ""; position: absolute; left: 11px; top: 24px; bottom: 0; width: 2px; background: var(--colors-border-primary); }
.aph-timeline-item__bubble { width: 24px; height: 24px; border-radius: 999px; background: var(--colors-primary-500); color: #fff; display: inline-flex; align-items: center; justify-content: center; font-size: 12px; font-weight: 600; flex-shrink: 0; z-index: 1; }
.aph-timeline-item__body { display: flex; flex-direction: column; gap: 4px; padding-top: 2px; }
.aph-timeline-item__title { font-weight: 600; color: var(--colors-text-primary); }
.aph-timeline-item__time { font-size: 12px; color: var(--colors-text-muted); }

.aph-disclosure { padding: 12px 16px; border: 1px solid var(--colors-border-primary); border-radius: 8px; background: var(--colors-background-primary); }
.aph-disclosure__trigger { width: 100%; background: transparent; border: none; cursor: pointer; padding: 0; font: inherit; text-align: left; display: flex; justify-content: space-between; align-items: center; color: var(--colors-text-primary); font-weight: 500; min-height: 32px; }
.aph-disclosure__body { padding-top: 8px; color: var(--colors-text-muted); }

.aph-context-menu { position: fixed; background: var(--colors-background-primary); border: 1px solid var(--colors-border-primary); border-radius: 8px; box-shadow: 0 8px 24px rgba(0,0,0,0.12); padding: 4px; min-width: 180px; z-index: 1000; }

.aph-command-root { background: var(--colors-background-primary); border: 1px solid var(--colors-border-primary); border-radius: 12px; box-shadow: 0 16px 48px rgba(0,0,0,0.2); width: 100%; max-width: 540px; overflow: hidden; }
.aph-command-input { width: 100%; padding: 14px 18px; border: none; border-bottom: 1px solid var(--colors-border-primary); font: inherit; background: transparent; color: var(--colors-text-primary); }
.aph-command-input:focus { outline: none; }
.aph-command-list { max-height: 320px; overflow: auto; padding: 4px; }
.aph-command-empty { padding: 24px; text-align: center; color: var(--colors-text-muted); font-size: 13px; }
.aph-command-row { display: flex; align-items: center; gap: 8px; padding: 10px 12px; border-radius: 6px; cursor: pointer; min-height: 40px; }
.aph-command-row[aria-selected="true"] { background: var(--colors-background-secondary); }

.aph-hint { font-size: 12px; color: var(--colors-text-muted); display: inline-flex; align-items: flex-start; gap: 4px; }
.aph-hint--warning { color: var(--colors-warning-700, #b45309); }
.aph-hint--danger { color: var(--colors-danger-700, #b91c1c); }
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

// ---- Extended primitives (v1.0 RC component expansion) ----

const TEXTAREA_TSX: &str = r#"import { forwardRef, TextareaHTMLAttributes } from "react";
import { cn } from "./cn";

export interface TextareaProps extends TextareaHTMLAttributes<HTMLTextAreaElement> {
  error?: boolean;
}

export const Textarea = forwardRef<HTMLTextAreaElement, TextareaProps>(function Textarea(
  { error, className, ...rest },
  ref,
) {
  return (
    <textarea
      ref={ref}
      className={cn("aph-textarea", error && "aph-input--error", className)}
      aria-invalid={error || undefined}
      {...rest}
    />
  );
});
"#;

const SELECT_TSX: &str = r#"import { forwardRef, SelectHTMLAttributes, ReactNode } from "react";
import { cn } from "./cn";

export interface SelectProps extends SelectHTMLAttributes<HTMLSelectElement> {
  error?: boolean;
  children: ReactNode;
}

export const Select = forwardRef<HTMLSelectElement, SelectProps>(function Select(
  { error, className, children, ...rest },
  ref,
) {
  return (
    <select
      ref={ref}
      className={cn("aph-select", error && "aph-input--error", className)}
      aria-invalid={error || undefined}
      {...rest}
    >
      {children}
    </select>
  );
});
"#;

const CHECKBOX_TSX: &str = r#"import { forwardRef, InputHTMLAttributes, ReactNode } from "react";
import { cn } from "./cn";

export interface CheckboxProps extends Omit<InputHTMLAttributes<HTMLInputElement>, "type"> {
  label?: ReactNode;
}

export const Checkbox = forwardRef<HTMLInputElement, CheckboxProps>(function Checkbox(
  { label, className, disabled, ...rest },
  ref,
) {
  const input = <input ref={ref} type="checkbox" disabled={disabled} {...rest} />;
  if (!label) return input;
  return (
    <label className={cn("aph-checkbox", disabled && "aph-checkbox--disabled", className)}>
      {input}
      <span>{label}</span>
    </label>
  );
});
"#;

const RADIO_TSX: &str = r#"import { forwardRef, InputHTMLAttributes, ReactNode } from "react";
import { cn } from "./cn";

export interface RadioProps extends Omit<InputHTMLAttributes<HTMLInputElement>, "type"> {
  label?: ReactNode;
}

export const Radio = forwardRef<HTMLInputElement, RadioProps>(function Radio(
  { label, className, disabled, ...rest },
  ref,
) {
  const input = <input ref={ref} type="radio" disabled={disabled} {...rest} />;
  if (!label) return input;
  return (
    <label className={cn("aph-radio", className)}>
      {input}
      <span>{label}</span>
    </label>
  );
});
"#;

const RADIO_GROUP_TSX: &str = r#"import { ReactNode, Children, cloneElement, isValidElement } from "react";
import { cn } from "./cn";

export interface RadioGroupProps {
  name: string;
  value: string;
  onChange: (next: string) => void;
  horizontal?: boolean;
  className?: string;
  children: ReactNode;
}

export function RadioGroup({ name, value, onChange, horizontal, className, children }: RadioGroupProps) {
  return (
    <div role="radiogroup" className={cn("aph-radio-group", horizontal && "aph-radio-group--horizontal", className)}>
      {Children.map(children, (child) => {
        if (!isValidElement(child)) return child;
        const props = child.props as Record<string, unknown>;
        return cloneElement(child as React.ReactElement<Record<string, unknown>>, {
          name,
          checked: props.value === value,
          onChange: (e: React.ChangeEvent<HTMLInputElement>) => onChange(e.target.value),
        });
      })}
    </div>
  );
}
"#;

const TABS_TSX: &str = r#"import { createContext, useContext, useState, useId, ReactNode } from "react";
import { cn } from "./cn";

type TabsCtx = { value: string; setValue: (v: string) => void; baseId: string };
const Ctx = createContext<TabsCtx | null>(null);

export interface TabsProps {
  defaultValue: string;
  value?: string;
  onValueChange?: (v: string) => void;
  children: ReactNode;
  className?: string;
}

export function Tabs({ defaultValue, value, onValueChange, children, className }: TabsProps) {
  const [internal, setInternal] = useState(defaultValue);
  const current = value ?? internal;
  const setCurrent = (v: string) => {
    if (value === undefined) setInternal(v);
    onValueChange?.(v);
  };
  const baseId = useId();
  return (
    <div className={cn("aph-tabs", className)}>
      <Ctx.Provider value={{ value: current, setValue: setCurrent, baseId }}>{children}</Ctx.Provider>
    </div>
  );
}

export function TabList({ children }: { children: ReactNode }) {
  return <div role="tablist" className="aph-tab-list">{children}</div>;
}

export function Tab({ value, children }: { value: string; children: ReactNode }) {
  const ctx = useContext(Ctx);
  if (!ctx) throw new Error("Tab must be inside <Tabs>");
  const selected = ctx.value === value;
  return (
    <button
      type="button"
      role="tab"
      id={`${ctx.baseId}-tab-${value}`}
      aria-controls={`${ctx.baseId}-panel-${value}`}
      aria-selected={selected}
      tabIndex={selected ? 0 : -1}
      onClick={() => ctx.setValue(value)}
      className="aph-tab"
    >
      {children}
    </button>
  );
}

export function TabPanel({ value, children }: { value: string; children: ReactNode }) {
  const ctx = useContext(Ctx);
  if (!ctx) throw new Error("TabPanel must be inside <Tabs>");
  if (ctx.value !== value) return null;
  return (
    <div role="tabpanel" id={`${ctx.baseId}-panel-${value}`} aria-labelledby={`${ctx.baseId}-tab-${value}`} className="aph-tab-panel">
      {children}
    </div>
  );
}
"#;

const ACCORDION_TSX: &str = r#"import { useState, ReactNode, useId } from "react";
import { cn } from "./cn";

export interface AccordionProps {
  children: ReactNode;
  className?: string;
  /** Allow multiple items open at once. */
  multiple?: boolean;
}

export interface AccordionItemProps {
  title: ReactNode;
  children: ReactNode;
  defaultOpen?: boolean;
}

export function Accordion({ children, className }: AccordionProps) {
  return <div className={cn("aph-accordion", className)}>{children}</div>;
}

export function AccordionItem({ title, children, defaultOpen = false }: AccordionItemProps) {
  const [open, setOpen] = useState(defaultOpen);
  const id = useId();
  return (
    <div className="aph-accordion-item">
      <button
        type="button"
        className="aph-accordion-trigger"
        aria-expanded={open}
        aria-controls={`${id}-content`}
        id={`${id}-trigger`}
        onClick={() => setOpen((o) => !o)}
      >
        <span>{title}</span>
        <span className="aph-accordion-chevron" aria-hidden="true">▾</span>
      </button>
      {open && (
        <div role="region" id={`${id}-content`} aria-labelledby={`${id}-trigger`} className="aph-accordion-content">
          {children}
        </div>
      )}
    </div>
  );
}
"#;

const TOAST_TSX: &str = r#"import { createContext, useContext, useState, useCallback, ReactNode, useEffect } from "react";
import { cn } from "./cn";

export interface ToastProps {
  id: number;
  message: ReactNode;
  tone?: "neutral" | "success" | "danger";
  duration?: number;
}

type ToastCtx = {
  toasts: ToastProps[];
  push: (t: Omit<ToastProps, "id">) => void;
  dismiss: (id: number) => void;
};

const Ctx = createContext<ToastCtx | null>(null);

export function ToastProvider({ children }: { children: ReactNode }) {
  const [toasts, setToasts] = useState<ToastProps[]>([]);
  const dismiss = useCallback((id: number) => setToasts((xs) => xs.filter((t) => t.id !== id)), []);
  const push = useCallback((t: Omit<ToastProps, "id">) => {
    const id = Date.now() + Math.random();
    setToasts((xs) => [...xs, { ...t, id }]);
    const duration = t.duration ?? 3000;
    if (duration > 0) setTimeout(() => dismiss(id), duration);
  }, [dismiss]);
  return (
    <Ctx.Provider value={{ toasts, push, dismiss }}>
      {children}
      <div role="region" aria-live="polite" aria-label="Notifications" className="aph-toast-region">
        {toasts.map((t) => <Toast key={t.id} {...t} />)}
      </div>
    </Ctx.Provider>
  );
}

export function useToast() {
  const ctx = useContext(Ctx);
  if (!ctx) throw new Error("useToast must be inside <ToastProvider>");
  return ctx;
}

export function Toast({ message, tone = "neutral" }: ToastProps) {
  return <div className={cn("aph-toast", tone !== "neutral" && `aph-toast--${tone}`)}>{message}</div>;
}
"#;

const TOOLTIP_TSX: &str = r#"import { ReactElement, cloneElement, useState } from "react";

export interface TooltipProps {
  content: string;
  children: ReactElement;
}

export function Tooltip({ content, children }: TooltipProps) {
  const [open, setOpen] = useState(false);
  return (
    <span className="aph-tooltip-wrap" onMouseEnter={() => setOpen(true)} onMouseLeave={() => setOpen(false)} onFocus={() => setOpen(true)} onBlur={() => setOpen(false)}>
      {cloneElement(children, { "aria-describedby": open ? "aph-tooltip-active" : undefined })}
      <span role="tooltip" id="aph-tooltip-active" className="aph-tooltip">{content}</span>
    </span>
  );
}
"#;

const POPOVER_TSX: &str = r#"import { useState, useRef, useEffect, ReactNode, ReactElement, cloneElement } from "react";
import { cn } from "./cn";

export interface PopoverProps {
  trigger: ReactElement;
  children: ReactNode;
  className?: string;
}

export function Popover({ trigger, children, className }: PopoverProps) {
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLSpanElement>(null);
  useEffect(() => {
    if (!open) return;
    const onDoc = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) setOpen(false);
    };
    document.addEventListener("mousedown", onDoc);
    return () => document.removeEventListener("mousedown", onDoc);
  }, [open]);
  return (
    <span ref={ref} className="aph-popover-wrap">
      {cloneElement(trigger, { onClick: () => setOpen((o) => !o), "aria-expanded": open })}
      {open && <div className={cn("aph-popover", className)} role="dialog">{children}</div>}
    </span>
  );
}
"#;

const MENU_TSX: &str = r#"import { ReactNode, ButtonHTMLAttributes, forwardRef } from "react";
import { cn } from "./cn";

export interface MenuProps {
  children: ReactNode;
  className?: string;
  ariaLabel?: string;
}

export interface MenuItemProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  children: ReactNode;
}

export function Menu({ children, className, ariaLabel }: MenuProps) {
  return (
    <div role="menu" aria-label={ariaLabel} className={cn("aph-menu", className)}>
      {children}
    </div>
  );
}

export const MenuItem = forwardRef<HTMLButtonElement, MenuItemProps>(function MenuItem(
  { children, className, ...rest },
  ref,
) {
  return (
    <button ref={ref} type="button" role="menuitem" className={cn("aph-menu-item", className)} {...rest}>
      {children}
    </button>
  );
});
"#;

const PROGRESS_BAR_TSX: &str = r#"import { cn } from "./cn";

export interface ProgressBarProps {
  value: number;
  max?: number;
  label?: string;
  className?: string;
}

export function ProgressBar({ value, max = 100, label, className }: ProgressBarProps) {
  const pct = Math.max(0, Math.min(100, (value / max) * 100));
  return (
    <div role="progressbar" aria-valuenow={value} aria-valuemin={0} aria-valuemax={max} aria-label={label} className={cn("aph-progress", className)}>
      <div className="aph-progress__bar" style={{ width: `${pct}%` }} />
    </div>
  );
}
"#;

const STEPPER_TSX: &str = r#"import { Fragment, ReactNode } from "react";
import { cn } from "./cn";

export interface StepperProps {
  steps: Array<{ label: ReactNode }>;
  current: number;
  className?: string;
}

export function Stepper({ steps, current, className }: StepperProps) {
  return (
    <ol className={cn("aph-stepper", className)}>
      {steps.map((s, i) => (
        <Fragment key={i}>
          <li
            className={cn(
              "aph-stepper-step",
              i === current && "aph-stepper-step--active",
              i < current && "aph-stepper-step--done",
            )}
            aria-current={i === current ? "step" : undefined}
          >
            <span className="aph-stepper-bubble">{i < current ? "✓" : i + 1}</span>
            <span>{s.label}</span>
          </li>
          {i < steps.length - 1 && <span className="aph-stepper-line" aria-hidden="true" />}
        </Fragment>
      ))}
    </ol>
  );
}
"#;

const SLIDER_TSX: &str = r#"import { forwardRef, InputHTMLAttributes } from "react";
import { cn } from "./cn";

export interface SliderProps extends Omit<InputHTMLAttributes<HTMLInputElement>, "type"> {}

export const Slider = forwardRef<HTMLInputElement, SliderProps>(function Slider(
  { className, ...rest },
  ref,
) {
  return <input ref={ref} type="range" className={cn("aph-slider", className)} {...rest} />;
});
"#;

const BREADCRUMB_TSX: &str = r#"import { ReactNode, Fragment } from "react";
import { cn } from "./cn";

export interface BreadcrumbProps {
  items: Array<{ label: ReactNode; href?: string }>;
  className?: string;
  separator?: ReactNode;
}

export function Breadcrumb({ items, className, separator = "/" }: BreadcrumbProps) {
  return (
    <nav aria-label="Breadcrumb" className={cn("aph-breadcrumb", className)}>
      {items.map((item, i) => {
        const last = i === items.length - 1;
        return (
          <Fragment key={i}>
            {item.href && !last ? (
              <a href={item.href}>{item.label}</a>
            ) : (
              <span className={last ? "aph-breadcrumb__current" : undefined} aria-current={last ? "page" : undefined}>{item.label}</span>
            )}
            {!last && <span className="aph-breadcrumb__sep" aria-hidden="true">{separator}</span>}
          </Fragment>
        );
      })}
    </nav>
  );
}
"#;

const PAGINATION_TSX: &str = r#"import { cn } from "./cn";

export interface PaginationProps {
  page: number;
  pageCount: number;
  onChange: (page: number) => void;
  className?: string;
  /** Max number of page buttons shown. Default 7. */
  siblingCount?: number;
}

export function Pagination({ page, pageCount, onChange, className, siblingCount = 7 }: PaginationProps) {
  const pages: number[] = [];
  const half = Math.floor(siblingCount / 2);
  let start = Math.max(1, page - half);
  let end = Math.min(pageCount, start + siblingCount - 1);
  if (end - start + 1 < siblingCount) start = Math.max(1, end - siblingCount + 1);
  for (let i = start; i <= end; i++) pages.push(i);
  return (
    <nav aria-label="Pagination" className={cn("aph-pagination", className)}>
      <button type="button" className="aph-pagination__btn" disabled={page <= 1} onClick={() => onChange(page - 1)} aria-label="이전 페이지">‹</button>
      {pages.map((p) => (
        <button
          key={p}
          type="button"
          className="aph-pagination__btn"
          aria-current={p === page ? "page" : undefined}
          onClick={() => onChange(p)}
        >
          {p}
        </button>
      ))}
      <button type="button" className="aph-pagination__btn" disabled={page >= pageCount} onClick={() => onChange(page + 1)} aria-label="다음 페이지">›</button>
    </nav>
  );
}
"#;

const DIVIDER_TSX: &str = r#"import { cn } from "./cn";

export interface DividerProps {
  orientation?: "horizontal" | "vertical";
  className?: string;
}

export function Divider({ orientation = "horizontal", className }: DividerProps) {
  return <hr role="separator" aria-orientation={orientation} className={cn("aph-divider", orientation === "vertical" && "aph-divider--vertical", className)} />;
}
"#;

const EMPTY_STATE_TSX: &str = r#"import { ReactNode } from "react";
import { cn } from "./cn";

export interface EmptyStateProps {
  title: ReactNode;
  description?: ReactNode;
  action?: ReactNode;
  icon?: ReactNode;
  className?: string;
}

export function EmptyState({ title, description, action, icon, className }: EmptyStateProps) {
  return (
    <div role="status" className={cn("aph-empty", className)}>
      {icon}
      <div className="aph-empty__title">{title}</div>
      {description && <div>{description}</div>}
      {action}
    </div>
  );
}
"#;

// ---- Storybook stories (CSF3) ----

const BUTTON_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Button } from "./Button";

const meta: Meta<typeof Button> = { component: Button, args: { children: "확인" } };
export default meta;
type Story = StoryObj<typeof Button>;

export const Primary: Story = { args: { variant: "primary" } };
export const Secondary: Story = { args: { variant: "secondary" } };
export const Ghost: Story = { args: { variant: "ghost" } };
export const Danger: Story = { args: { variant: "danger" } };
export const Small: Story = { args: { variant: "primary", size: "sm" } };
export const Large: Story = { args: { variant: "primary", size: "lg" } };
export const Loading: Story = { args: { variant: "primary", loading: true } };
export const Disabled: Story = { args: { variant: "primary", disabled: true } };
"#;

const INPUT_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Input } from "./Input";

const meta: Meta<typeof Input> = { component: Input, args: { placeholder: "이메일을 입력해주세요" } };
export default meta;
type Story = StoryObj<typeof Input>;

export const Default: Story = {};
export const Error: Story = { args: { error: true, defaultValue: "잘못된 이메일" } };
export const Disabled: Story = { args: { disabled: true, defaultValue: "user@toss.im" } };
"#;

const TAG_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Tag } from "./Tag";

const meta: Meta<typeof Tag> = { component: Tag, args: { children: "신규" } };
export default meta;
type Story = StoryObj<typeof Tag>;

export const Neutral: Story = { args: { tone: "neutral" } };
export const Success: Story = { args: { tone: "success", children: "완료" } };
export const Warning: Story = { args: { tone: "warning", children: "주의" } };
export const Danger: Story = { args: { tone: "danger", children: "실패" } };
"#;

const AVATAR_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Avatar } from "./Avatar";

const meta: Meta<typeof Avatar> = { component: Avatar };
export default meta;
type Story = StoryObj<typeof Avatar>;

export const Initials: Story = { args: { initials: "JK", alt: "Jihyo Kim" } };
export const Small: Story = { args: { size: "sm", initials: "MJ" } };
export const Large: Story = { args: { size: "lg", initials: "SA" } };
"#;

const CARD_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Card } from "./Card";

const meta: Meta<typeof Card> = { component: Card };
export default meta;
type Story = StoryObj<typeof Card>;

export const Default: Story = {
  args: { children: <><h3 style={{ margin: 0 }}>월 정산 내역</h3><p style={{ marginTop: 8 }}>2026년 5월 1일 ~ 5월 31일</p></> },
};
"#;

const MODAL_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { Modal } from "./Modal";
import { Button } from "./Button";

const meta: Meta<typeof Modal> = { component: Modal };
export default meta;
type Story = StoryObj<typeof Modal>;

export const Default: Story = {
  render: () => {
    const [open, setOpen] = useState(false);
    return (
      <>
        <Button onClick={() => setOpen(true)}>모달 열기</Button>
        <Modal open={open} onClose={() => setOpen(false)} title="확인">
          <p>저장하시겠습니까?</p>
          <Button onClick={() => setOpen(false)}>닫기</Button>
        </Modal>
      </>
    );
  },
};
"#;

const DRAWER_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { Drawer } from "./Drawer";
import { Button } from "./Button";

const meta: Meta<typeof Drawer> = { component: Drawer };
export default meta;
type Story = StoryObj<typeof Drawer>;

export const Default: Story = {
  render: () => {
    const [open, setOpen] = useState(false);
    return (
      <>
        <Button onClick={() => setOpen(true)}>드로어 열기</Button>
        <Drawer open={open} onClose={() => setOpen(false)}>
          <h2>설정</h2>
          <p>드로어 안의 컨텐츠입니다.</p>
          <Button variant="secondary" onClick={() => setOpen(false)}>닫기</Button>
        </Drawer>
      </>
    );
  },
};
"#;

const SKELETON_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Skeleton } from "./Skeleton";

const meta: Meta<typeof Skeleton> = { component: Skeleton };
export default meta;
type Story = StoryObj<typeof Skeleton>;

export const Line: Story = { args: { width: 240, height: 16 } };
export const Block: Story = { args: { width: 320, height: 120 } };
export const Circle: Story = { args: { width: 48, height: 48, circle: true } };
"#;

const FORM_FIELD_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { FormField } from "./FormField";
import { Input } from "./Input";

const meta: Meta<typeof FormField> = { component: FormField };
export default meta;
type Story = StoryObj<typeof FormField>;

export const WithHint: Story = {
  args: {
    label: "이메일",
    hint: "회사 이메일을 사용해주세요",
    children: <Input placeholder="name@toss.im" />,
  },
};

export const WithError: Story = {
  args: {
    label: "비밀번호",
    error: "8자 이상이어야 합니다",
    children: <Input type="password" error defaultValue="abc" />,
  },
};
"#;

const SWITCH_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { Switch } from "./Switch";

const meta: Meta<typeof Switch> = { component: Switch };
export default meta;
type Story = StoryObj<typeof Switch>;

export const Default: Story = {
  render: () => {
    const [on, setOn] = useState(true);
    return <Switch checked={on} onChange={setOn} label="알림 받기" />;
  },
};
"#;

const BADGE_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Badge } from "./Badge";

const meta: Meta<typeof Badge> = { component: Badge };
export default meta;
type Story = StoryObj<typeof Badge>;

export const Count: Story = { args: { count: 3 } };
export const Overflow: Story = { args: { count: 124, max: 99 } };
export const Dot: Story = { args: { dot: true } };
"#;

const SPINNER_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Spinner } from "./Spinner";

const meta: Meta<typeof Spinner> = { component: Spinner };
export default meta;
type Story = StoryObj<typeof Spinner>;

export const Small: Story = { args: { size: "sm" } };
export const Medium: Story = { args: { size: "md" } };
export const Large: Story = { args: { size: "lg" } };
"#;

// ---- RC.5 advanced primitives ----

const SEGMENTED_CONTROL_TSX: &str = r#"import { ReactNode } from "react";
import { cn } from "./cn";

export interface SegmentedControlProps<T extends string> {
  options: Array<{ value: T; label: ReactNode }>;
  value: T;
  onChange: (next: T) => void;
  className?: string;
  ariaLabel?: string;
}

export function SegmentedControl<T extends string>({ options, value, onChange, className, ariaLabel }: SegmentedControlProps<T>) {
  return (
    <div role="radiogroup" aria-label={ariaLabel} className={cn("aph-segmented", className)}>
      {options.map((o) => (
        <button
          key={o.value}
          type="button"
          role="radio"
          aria-pressed={value === o.value}
          aria-checked={value === o.value}
          onClick={() => onChange(o.value)}
          className="aph-segmented__btn"
        >
          {o.label}
        </button>
      ))}
    </div>
  );
}
"#;

const PIN_INPUT_TSX: &str = r#"import { useRef, ChangeEvent, KeyboardEvent } from "react";
import { cn } from "./cn";

export interface PinInputProps {
  length?: number;
  value: string;
  onChange: (next: string) => void;
  type?: "number" | "text";
  className?: string;
  ariaLabel?: string;
}

export function PinInput({ length = 6, value, onChange, type = "number", className, ariaLabel = "OTP" }: PinInputProps) {
  const refs = useRef<Array<HTMLInputElement | null>>([]);
  const digits = Array.from({ length }, (_, i) => value[i] ?? "");
  const set = (i: number, v: string) => {
    const next = (value.slice(0, i) + v + value.slice(i + 1)).slice(0, length);
    onChange(next);
  };
  const onInput = (i: number) => (e: ChangeEvent<HTMLInputElement>) => {
    const v = (type === "number" ? e.target.value.replace(/\D/g, "") : e.target.value).slice(-1);
    set(i, v);
    if (v && i < length - 1) refs.current[i + 1]?.focus();
  };
  const onKey = (i: number) => (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Backspace" && !digits[i] && i > 0) refs.current[i - 1]?.focus();
    if (e.key === "ArrowLeft" && i > 0) refs.current[i - 1]?.focus();
    if (e.key === "ArrowRight" && i < length - 1) refs.current[i + 1]?.focus();
  };
  return (
    <div role="group" aria-label={ariaLabel} className={cn("aph-pin", className)}>
      {digits.map((d, i) => (
        <input
          key={i}
          ref={(el) => { refs.current[i] = el; }}
          inputMode={type === "number" ? "numeric" : "text"}
          maxLength={1}
          value={d}
          onChange={onInput(i)}
          onKeyDown={onKey(i)}
          className="aph-pin__digit"
          aria-label={`Digit ${i + 1}`}
        />
      ))}
    </div>
  );
}
"#;

const NUMBER_INPUT_TSX: &str = r#"import { useId } from "react";
import { cn } from "./cn";

export interface NumberInputProps {
  value: number;
  onChange: (next: number) => void;
  min?: number;
  max?: number;
  step?: number;
  disabled?: boolean;
  className?: string;
  ariaLabel?: string;
}

export function NumberInput({ value, onChange, min, max, step = 1, disabled, className, ariaLabel }: NumberInputProps) {
  const id = useId();
  const clamp = (n: number) => {
    if (typeof min === "number" && n < min) return min;
    if (typeof max === "number" && n > max) return max;
    return n;
  };
  return (
    <div className={cn("aph-number-input", className)}>
      <button type="button" className="aph-number-input__btn" aria-label="감소" disabled={disabled || (typeof min === "number" && value <= min)} onClick={() => onChange(clamp(value - step))}>−</button>
      <input
        id={id}
        type="number"
        className="aph-number-input__field"
        value={value}
        onChange={(e) => onChange(clamp(Number(e.target.value)))}
        disabled={disabled}
        aria-label={ariaLabel}
      />
      <button type="button" className="aph-number-input__btn" aria-label="증가" disabled={disabled || (typeof max === "number" && value >= max)} onClick={() => onChange(clamp(value + step))}>+</button>
    </div>
  );
}
"#;

const SEARCH_INPUT_TSX: &str = r#"import { forwardRef, InputHTMLAttributes } from "react";
import { cn } from "./cn";

export interface SearchInputProps extends Omit<InputHTMLAttributes<HTMLInputElement>, "type" | "onChange"> {
  value: string;
  onChange: (v: string) => void;
  onClear?: () => void;
}

export const SearchInput = forwardRef<HTMLInputElement, SearchInputProps>(function SearchInput(
  { value, onChange, onClear, className, ...rest },
  ref,
) {
  const clear = () => {
    onChange("");
    onClear?.();
  };
  return (
    <div className={cn("aph-search", className)}>
      <input
        ref={ref}
        type="search"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        className="aph-input aph-search__input"
        {...rest}
      />
      {value && (
        <button type="button" className="aph-search__clear" aria-label="검색어 지우기" onClick={clear}>×</button>
      )}
    </div>
  );
});
"#;

const FILE_UPLOADER_TSX: &str = r#"import { useState, useRef, ReactNode, DragEvent, ChangeEvent } from "react";
import { cn } from "./cn";

export interface FileUploaderProps {
  onFiles: (files: File[]) => void;
  accept?: string;
  multiple?: boolean;
  disabled?: boolean;
  className?: string;
  children?: ReactNode;
}

export function FileUploader({ onFiles, accept, multiple, disabled, className, children }: FileUploaderProps) {
  const inputRef = useRef<HTMLInputElement>(null);
  const [drag, setDrag] = useState(false);
  const handle = (files: FileList | null) => {
    if (!files || files.length === 0) return;
    onFiles(Array.from(files));
  };
  const onDrop = (e: DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setDrag(false);
    handle(e.dataTransfer.files);
  };
  return (
    <div
      role="button"
      tabIndex={disabled ? -1 : 0}
      data-drag={drag}
      data-disabled={disabled || undefined}
      className={cn("aph-uploader", className)}
      onClick={() => !disabled && inputRef.current?.click()}
      onKeyDown={(e) => { if ((e.key === "Enter" || e.key === " ") && !disabled) inputRef.current?.click(); }}
      onDragOver={(e) => { e.preventDefault(); setDrag(true); }}
      onDragLeave={() => setDrag(false)}
      onDrop={onDrop}
    >
      <input
        ref={inputRef}
        type="file"
        accept={accept}
        multiple={multiple}
        style={{ display: "none" }}
        onChange={(e: ChangeEvent<HTMLInputElement>) => handle(e.target.files)}
      />
      {children ?? (
        <>
          <span style={{ fontSize: 24 }}>📎</span>
          <span><strong>파일을 끌어다 놓거나 클릭하여 업로드</strong></span>
        </>
      )}
    </div>
  );
}
"#;

const DATE_PICKER_TSX: &str = r#"import { forwardRef, InputHTMLAttributes } from "react";
import { cn } from "./cn";

export interface DatePickerProps extends Omit<InputHTMLAttributes<HTMLInputElement>, "type"> {
  error?: boolean;
}

/** Native HTML5 date input — picks up locale automatically. For custom
 *  calendar UI use a third-party (react-aria-components / @internationalized/date)
 *  paired with our tokens. */
export const DatePicker = forwardRef<HTMLInputElement, DatePickerProps>(function DatePicker(
  { error, className, ...rest },
  ref,
) {
  return (
    <div className="aph-date-input">
      <input
        ref={ref}
        type="date"
        className={cn("aph-input", error && "aph-input--error", className)}
        aria-invalid={error || undefined}
        {...rest}
      />
    </div>
  );
});
"#;

const COMBOBOX_TSX: &str = r#"import { useState, useRef, useEffect, ReactNode, useId } from "react";
import { cn } from "./cn";

export interface ComboboxOption {
  value: string;
  label: ReactNode;
  /** Lower-cased search-key. Defaults to the value. */
  search?: string;
}

export interface ComboboxProps {
  options: ComboboxOption[];
  value: string;
  onChange: (v: string) => void;
  placeholder?: string;
  className?: string;
  ariaLabel?: string;
}

export function Combobox({ options, value, onChange, placeholder, className, ariaLabel }: ComboboxProps) {
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const wrapRef = useRef<HTMLDivElement>(null);
  const id = useId();
  useEffect(() => {
    if (!open) return;
    const onDoc = (e: MouseEvent) => {
      if (wrapRef.current && !wrapRef.current.contains(e.target as Node)) setOpen(false);
    };
    document.addEventListener("mousedown", onDoc);
    return () => document.removeEventListener("mousedown", onDoc);
  }, [open]);
  const filtered = options.filter((o) => {
    if (!query) return true;
    const haystack = (o.search ?? o.value).toLowerCase();
    return haystack.includes(query.toLowerCase());
  });
  const current = options.find((o) => o.value === value);
  return (
    <div ref={wrapRef} className={cn("aph-combobox", className)}>
      <input
        type="text"
        className="aph-input"
        role="combobox"
        aria-expanded={open}
        aria-controls={`${id}-list`}
        aria-label={ariaLabel}
        placeholder={placeholder}
        value={open ? query : (typeof current?.label === "string" ? current.label : value)}
        onFocus={() => setOpen(true)}
        onChange={(e) => { setQuery(e.target.value); setOpen(true); }}
      />
      {open && filtered.length > 0 && (
        <ul role="listbox" id={`${id}-list`} className="aph-combobox__list">
          {filtered.map((o) => (
            <li
              key={o.value}
              role="option"
              aria-selected={o.value === value}
              className="aph-combobox__option"
              onMouseDown={(e) => { e.preventDefault(); onChange(o.value); setOpen(false); setQuery(""); }}
            >
              {o.label}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
"#;

const SHEET_TSX: &str = r#"import { useEffect, ReactNode } from "react";
import { cn } from "./cn";

export interface SheetProps {
  open: boolean;
  onClose: () => void;
  title?: ReactNode;
  children: ReactNode;
  className?: string;
}

/** Mobile-first bottom sheet. Use Drawer for desktop right-side panels. */
export function Sheet({ open, onClose, title, children, className }: SheetProps) {
  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => { if (e.key === "Escape") onClose(); };
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  }, [open, onClose]);
  if (!open) return null;
  return (
    <>
      <div className="aph-sheet-backdrop" onClick={onClose} aria-hidden="true" />
      <div role="dialog" aria-modal="true" aria-label={typeof title === "string" ? title : undefined} className={cn("aph-sheet", className)}>
        <div className="aph-sheet__handle" aria-hidden="true" />
        {title && <h2 style={{ margin: "0 0 12px" }}>{title}</h2>}
        {children}
      </div>
    </>
  );
}
"#;

const ALERT_TSX: &str = r#"import { ReactNode } from "react";
import { cn } from "./cn";

export type AlertTone = "info" | "success" | "warning" | "danger";

export interface AlertProps {
  tone?: AlertTone;
  title?: ReactNode;
  children?: ReactNode;
  icon?: ReactNode;
  className?: string;
}

export function Alert({ tone = "info", title, children, icon, className }: AlertProps) {
  return (
    <div role="alert" className={cn("aph-alert", `aph-alert--${tone}`, className)}>
      {icon && <span aria-hidden="true">{icon}</span>}
      <div>
        {title && <div className="aph-alert__title">{title}</div>}
        {children && <div>{children}</div>}
      </div>
    </div>
  );
}
"#;

const STAT_TSX: &str = r#"import { ReactNode } from "react";
import { cn } from "./cn";

export interface StatProps {
  label: ReactNode;
  value: ReactNode;
  delta?: { value: ReactNode; direction: "up" | "down" };
  className?: string;
}

export function Stat({ label, value, delta, className }: StatProps) {
  return (
    <div className={cn("aph-stat", className)}>
      <span className="aph-stat__label">{label}</span>
      <span className="aph-stat__value">{value}</span>
      {delta && (
        <span className={cn("aph-stat__delta", `aph-stat__delta--${delta.direction}`)}>
          {delta.direction === "up" ? "▲" : "▼"} {delta.value}
        </span>
      )}
    </div>
  );
}
"#;

const TOOLBAR_TSX: &str = r#"import { ReactNode } from "react";
import { cn } from "./cn";

export interface ToolbarProps {
  children: ReactNode;
  className?: string;
  ariaLabel?: string;
}

export function Toolbar({ children, className, ariaLabel }: ToolbarProps) {
  return (
    <div role="toolbar" aria-label={ariaLabel} className={cn("aph-toolbar", className)}>
      {children}
    </div>
  );
}

Toolbar.Separator = function ToolbarSeparator() {
  return <span role="separator" aria-orientation="vertical" className="aph-toolbar__sep" />;
};
"#;

const HOVER_CARD_TSX: &str = r#"import { ReactNode, ReactElement } from "react";
import { cn } from "./cn";

export interface HoverCardProps {
  trigger: ReactElement;
  children: ReactNode;
  className?: string;
}

export function HoverCard({ trigger, children, className }: HoverCardProps) {
  return (
    <span className="aph-hover-wrap" tabIndex={0}>
      {trigger}
      <span role="tooltip" className={cn("aph-hover-card", className)}>{children}</span>
    </span>
  );
}
"#;

// ---- RC.7 advanced primitives ----

const DATA_TABLE_TSX: &str = r#"import { useState, useMemo, ReactNode } from "react";
import { cn } from "./cn";

export interface DataTableColumn<T> {
  key: keyof T & string;
  header: ReactNode;
  /** When true, header is clickable and toggles asc → desc → none. */
  sortable?: boolean;
  /** Custom cell renderer. */
  cell?: (row: T) => ReactNode;
  align?: "left" | "right" | "center";
  width?: string;
}

export interface DataTableProps<T> {
  rows: T[];
  columns: DataTableColumn<T>[];
  /** Stable key fn for React keys + selection. Defaults to row index. */
  rowKey?: (row: T, i: number) => string | number;
  className?: string;
  emptyMessage?: ReactNode;
}

type SortDir = "asc" | "desc" | null;

export function DataTable<T>({ rows, columns, rowKey, className, emptyMessage = "데이터가 없습니다" }: DataTableProps<T>) {
  const [sortKey, setSortKey] = useState<string | null>(null);
  const [sortDir, setSortDir] = useState<SortDir>(null);
  const sorted = useMemo(() => {
    if (!sortKey || !sortDir) return rows;
    return [...rows].sort((a, b) => {
      const av = (a as unknown as Record<string, unknown>)[sortKey];
      const bv = (b as unknown as Record<string, unknown>)[sortKey];
      if (av == null && bv == null) return 0;
      if (av == null) return 1;
      if (bv == null) return -1;
      const cmp = av < bv ? -1 : av > bv ? 1 : 0;
      return sortDir === "asc" ? cmp : -cmp;
    });
  }, [rows, sortKey, sortDir]);
  const cycle = (key: string) => {
    if (sortKey !== key) { setSortKey(key); setSortDir("asc"); return; }
    if (sortDir === "asc") { setSortDir("desc"); return; }
    if (sortDir === "desc") { setSortKey(null); setSortDir(null); return; }
    setSortDir("asc");
  };
  return (
    <div className={cn("aph-table-wrap", className)}>
      <table className="aph-table">
        <thead>
          <tr>
            {columns.map((c) => {
              const sort = c.sortable && sortKey === c.key ? (sortDir === "asc" ? "ascending" : "descending") : c.sortable ? "none" : undefined;
              return (
                <th
                  key={c.key}
                  aria-sort={sort as React.AriaAttributes["aria-sort"]}
                  style={{ textAlign: c.align ?? "left", width: c.width }}
                  onClick={c.sortable ? () => cycle(c.key) : undefined}
                >
                  {c.header}
                  {c.sortable && (
                    <span className={cn("aph-table__sort-icon", sortKey === c.key && "aph-table__sort-icon--active")}>
                      {sortKey === c.key ? (sortDir === "asc" ? "▲" : "▼") : "⇅"}
                    </span>
                  )}
                </th>
              );
            })}
          </tr>
        </thead>
        <tbody>
          {sorted.length === 0 ? (
            <tr><td colSpan={columns.length} style={{ textAlign: "center", padding: "32px 16px", color: "var(--colors-text-muted)" }}>{emptyMessage}</td></tr>
          ) : (
            sorted.map((row, i) => (
              <tr key={rowKey ? rowKey(row, i) : i}>
                {columns.map((c) => (
                  <td key={c.key} style={{ textAlign: c.align ?? "left" }}>
                    {c.cell ? c.cell(row) : String((row as unknown as Record<string, unknown>)[c.key] ?? "")}
                  </td>
                ))}
              </tr>
            ))
          )}
        </tbody>
      </table>
    </div>
  );
}
"#;

const CAROUSEL_TSX: &str = r#"import { useState, ReactNode, Children, useEffect } from "react";
import { cn } from "./cn";

export interface CarouselProps {
  children: ReactNode;
  autoplay?: number;
  showArrows?: boolean;
  showDots?: boolean;
  className?: string;
  ariaLabel?: string;
}

export function Carousel({ children, autoplay, showArrows = true, showDots = true, className, ariaLabel }: CarouselProps) {
  const slides = Children.toArray(children);
  const [idx, setIdx] = useState(0);
  const go = (next: number) => setIdx((next + slides.length) % slides.length);
  useEffect(() => {
    if (!autoplay) return;
    const t = setInterval(() => go(idx + 1), autoplay);
    return () => clearInterval(t);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [autoplay, idx, slides.length]);
  return (
    <div role="region" aria-roledescription="carousel" aria-label={ariaLabel} className={cn("aph-carousel", className)}>
      <div className="aph-carousel__track" style={{ transform: `translateX(-${idx * 100}%)` }}>
        {slides.map((s, i) => (
          <div key={i} className="aph-carousel__slide" aria-hidden={i !== idx}>{s}</div>
        ))}
      </div>
      {showArrows && slides.length > 1 && (
        <>
          <button type="button" className="aph-carousel__btn aph-carousel__btn--prev" onClick={() => go(idx - 1)} aria-label="이전">‹</button>
          <button type="button" className="aph-carousel__btn aph-carousel__btn--next" onClick={() => go(idx + 1)} aria-label="다음">›</button>
        </>
      )}
      {showDots && slides.length > 1 && (
        <div className="aph-carousel__dots" role="tablist">
          {slides.map((_, i) => (
            <button key={i} type="button" role="tab" aria-current={i === idx} className="aph-carousel__dot" onClick={() => setIdx(i)} aria-label={`슬라이드 ${i + 1}`} />
          ))}
        </div>
      )}
    </div>
  );
}
"#;

const CALENDAR_TSX: &str = r#"import { useState } from "react";
import { cn } from "./cn";

export interface CalendarProps {
  value?: Date;
  onChange?: (d: Date) => void;
  /** ISO yyyy-mm-dd strings to disable. */
  disabledDates?: string[];
  locale?: string;
  className?: string;
}

const KOREAN_DOW = ["일", "월", "화", "수", "목", "금", "토"];

export function Calendar({ value, onChange, disabledDates = [], locale = "ko-KR", className }: CalendarProps) {
  const initial = value ?? new Date();
  const [view, setView] = useState({ year: initial.getFullYear(), month: initial.getMonth() });
  const today = new Date();
  const todayIso = isoDate(today);
  const selectedIso = value ? isoDate(value) : null;

  const first = new Date(view.year, view.month, 1);
  const startDow = first.getDay();
  const daysInMonth = new Date(view.year, view.month + 1, 0).getDate();
  const prevMonthDays = new Date(view.year, view.month, 0).getDate();

  const cells: Array<{ d: number; otherMonth: boolean; iso: string; date: Date }> = [];
  for (let i = startDow - 1; i >= 0; i--) {
    const day = prevMonthDays - i;
    const date = new Date(view.year, view.month - 1, day);
    cells.push({ d: day, otherMonth: true, iso: isoDate(date), date });
  }
  for (let d = 1; d <= daysInMonth; d++) {
    const date = new Date(view.year, view.month, d);
    cells.push({ d, otherMonth: false, iso: isoDate(date), date });
  }
  while (cells.length % 7 !== 0) {
    const d = cells.length - (startDow + daysInMonth) + 1;
    const date = new Date(view.year, view.month + 1, d);
    cells.push({ d, otherMonth: true, iso: isoDate(date), date });
  }

  const monthLabel = new Date(view.year, view.month, 1).toLocaleDateString(locale, { year: "numeric", month: "long" });

  return (
    <div className={cn("aph-calendar", className)} role="grid" aria-label={monthLabel}>
      <div className="aph-calendar__head">
        <button type="button" className="aph-calendar__nav" aria-label="이전 달" onClick={() => setView(({ year, month }) => month === 0 ? { year: year - 1, month: 11 } : { year, month: month - 1 })}>‹</button>
        <span className="aph-calendar__month">{monthLabel}</span>
        <button type="button" className="aph-calendar__nav" aria-label="다음 달" onClick={() => setView(({ year, month }) => month === 11 ? { year: year + 1, month: 0 } : { year, month: month + 1 })}>›</button>
      </div>
      <div className="aph-calendar__grid">
        {KOREAN_DOW.map((d) => <div key={d} className="aph-calendar__dow">{d}</div>)}
        {cells.map((c, i) => {
          const disabled = disabledDates.includes(c.iso);
          const selected = selectedIso === c.iso;
          const isToday = todayIso === c.iso;
          return (
            <button
              key={i}
              type="button"
              role="gridcell"
              aria-selected={selected}
              aria-current={isToday ? "date" : undefined}
              disabled={disabled}
              className={cn(
                "aph-calendar__cell",
                c.otherMonth && "aph-calendar__cell--other",
                isToday && "aph-calendar__cell--today",
                selected && "aph-calendar__cell--selected",
              )}
              onClick={() => !disabled && onChange?.(c.date)}
            >
              {c.d}
            </button>
          );
        })}
      </div>
    </div>
  );
}

function isoDate(d: Date): string {
  const y = d.getFullYear();
  const m = String(d.getMonth() + 1).padStart(2, "0");
  const day = String(d.getDate()).padStart(2, "0");
  return `${y}-${m}-${day}`;
}
"#;

const CHIP_TSX: &str = r#"import { ReactNode } from "react";
import { cn } from "./cn";

export interface ChipProps {
  children: ReactNode;
  onClose?: () => void;
  className?: string;
}

export function Chip({ children, onClose, className }: ChipProps) {
  return (
    <span className={cn("aph-chip", className)}>
      {children}
      {onClose && (
        <button type="button" className="aph-chip__close" aria-label="제거" onClick={onClose}>×</button>
      )}
    </span>
  );
}
"#;

const IMAGE_TSX: &str = r#"import { useState, ImgHTMLAttributes } from "react";
import { cn } from "./cn";

export interface ImageProps extends ImgHTMLAttributes<HTMLImageElement> {
  fallback?: string;
  width?: number;
  height?: number;
  rounded?: boolean | number;
}

export function Image({ src, fallback, width, height, rounded, alt = "", className, ...rest }: ImageProps) {
  const [errored, setErrored] = useState(false);
  const radius = typeof rounded === "number" ? rounded : rounded ? 999 : 0;
  return (
    <span
      className={cn("aph-image", className)}
      style={{ width, height, borderRadius: radius }}
    >
      {!errored && src ? (
        <img src={src} alt={alt} loading="lazy" onError={() => setErrored(true)} {...rest} />
      ) : (
        <span className="aph-image__fallback">{fallback ?? (alt || "이미지를 불러올 수 없습니다")}</span>
      )}
    </span>
  );
}
"#;

const KBD_TSX: &str = r#"import { ReactNode } from "react";
import { cn } from "./cn";

export interface KbdProps {
  children: ReactNode;
  className?: string;
}

export function Kbd({ children, className }: KbdProps) {
  return <kbd className={cn("aph-kbd", className)}>{children}</kbd>;
}
"#;

const CODE_TSX: &str = r#"import { ReactNode, HTMLAttributes } from "react";
import { cn } from "./cn";

export interface CodeProps extends HTMLAttributes<HTMLElement> {
  block?: boolean;
  children: ReactNode;
}

export function Code({ block, className, children, ...rest }: CodeProps) {
  if (block) {
    return (
      <pre className={cn("aph-code-block", className)} {...rest as HTMLAttributes<HTMLPreElement>}>
        <code>{children}</code>
      </pre>
    );
  }
  return <code className={cn("aph-code", className)} {...rest}>{children}</code>;
}
"#;

const TIMELINE_TSX: &str = r#"import { ReactNode } from "react";
import { cn } from "./cn";

export interface TimelineProps {
  children: ReactNode;
  className?: string;
}

export interface TimelineItemProps {
  marker?: ReactNode;
  title: ReactNode;
  time?: ReactNode;
  children?: ReactNode;
}

export function Timeline({ children, className }: TimelineProps) {
  return <ol className={cn("aph-timeline", className)}>{children}</ol>;
}

export function TimelineItem({ marker, title, time, children }: TimelineItemProps) {
  return (
    <li className="aph-timeline-item">
      <span className="aph-timeline-item__bubble" aria-hidden="true">{marker ?? "•"}</span>
      <div className="aph-timeline-item__body">
        <span className="aph-timeline-item__title">{title}</span>
        {time && <span className="aph-timeline-item__time">{time}</span>}
        {children && <div>{children}</div>}
      </div>
    </li>
  );
}
"#;

const DISCLOSURE_TSX: &str = r#"import { useState, ReactNode, useId } from "react";
import { cn } from "./cn";

export interface DisclosureProps {
  title: ReactNode;
  children: ReactNode;
  defaultOpen?: boolean;
  className?: string;
}

export function Disclosure({ title, children, defaultOpen = false, className }: DisclosureProps) {
  const [open, setOpen] = useState(defaultOpen);
  const id = useId();
  return (
    <div className={cn("aph-disclosure", className)}>
      <button type="button" aria-expanded={open} aria-controls={`${id}-body`} className="aph-disclosure__trigger" onClick={() => setOpen((o) => !o)}>
        <span>{title}</span>
        <span aria-hidden="true">{open ? "−" : "+"}</span>
      </button>
      {open && <div id={`${id}-body`} className="aph-disclosure__body">{children}</div>}
    </div>
  );
}
"#;

const CONTEXT_MENU_TSX: &str = r#"import { useState, useEffect, useRef, ReactNode, MouseEvent as ReactMouseEvent } from "react";
import { cn } from "./cn";

export interface ContextMenuProps {
  trigger: ReactNode;
  items: Array<{ label: ReactNode; onSelect: () => void; disabled?: boolean }>;
  className?: string;
}

export function ContextMenu({ trigger, items, className }: ContextMenuProps) {
  const [pos, setPos] = useState<{ x: number; y: number } | null>(null);
  const ref = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!pos) return;
    const onDoc = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) setPos(null);
    };
    const onKey = (e: KeyboardEvent) => { if (e.key === "Escape") setPos(null); };
    document.addEventListener("mousedown", onDoc);
    document.addEventListener("keydown", onKey);
    return () => { document.removeEventListener("mousedown", onDoc); document.removeEventListener("keydown", onKey); };
  }, [pos]);
  const onTrigger = (e: ReactMouseEvent) => {
    e.preventDefault();
    setPos({ x: e.clientX, y: e.clientY });
  };
  return (
    <>
      <span onContextMenu={onTrigger}>{trigger}</span>
      {pos && (
        <div ref={ref} role="menu" className={cn("aph-context-menu", className)} style={{ left: pos.x, top: pos.y }}>
          {items.map((it, i) => (
            <button key={i} type="button" role="menuitem" className="aph-menu-item" disabled={it.disabled} onClick={() => { it.onSelect(); setPos(null); }}>
              {it.label}
            </button>
          ))}
        </div>
      )}
    </>
  );
}
"#;

const COMMAND_TSX: &str = r#"import { useState, useMemo, ReactNode, KeyboardEvent } from "react";
import { cn } from "./cn";

export interface CommandItem {
  id: string;
  label: ReactNode;
  /** Lower-cased search-key. Defaults to id. */
  search?: string;
  hint?: ReactNode;
  onSelect: () => void;
}

export interface CommandProps {
  items: CommandItem[];
  placeholder?: string;
  emptyMessage?: ReactNode;
  className?: string;
  ariaLabel?: string;
}

export function Command({ items, placeholder = "Search...", emptyMessage = "결과가 없습니다", className, ariaLabel = "Command" }: CommandProps) {
  const [q, setQ] = useState("");
  const [active, setActive] = useState(0);
  const filtered = useMemo(() => {
    if (!q) return items;
    const needle = q.toLowerCase();
    return items.filter((it) => (it.search ?? it.id).toLowerCase().includes(needle));
  }, [items, q]);
  const onKey = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "ArrowDown") { e.preventDefault(); setActive((a) => Math.min(filtered.length - 1, a + 1)); }
    if (e.key === "ArrowUp") { e.preventDefault(); setActive((a) => Math.max(0, a - 1)); }
    if (e.key === "Enter") { e.preventDefault(); filtered[active]?.onSelect(); }
  };
  return (
    <div role="combobox" aria-label={ariaLabel} aria-expanded="true" className={cn("aph-command-root", className)}>
      <input
        type="text"
        autoFocus
        value={q}
        onChange={(e) => { setQ(e.target.value); setActive(0); }}
        onKeyDown={onKey}
        placeholder={placeholder}
        className="aph-command-input"
        aria-label={placeholder}
      />
      <div className="aph-command-list" role="listbox">
        {filtered.length === 0 ? (
          <div className="aph-command-empty">{emptyMessage}</div>
        ) : (
          filtered.map((it, i) => (
            <div
              key={it.id}
              role="option"
              aria-selected={i === active}
              onMouseEnter={() => setActive(i)}
              onClick={() => it.onSelect()}
              className="aph-command-row"
            >
              <span style={{ flex: 1 }}>{it.label}</span>
              {it.hint && <span style={{ fontSize: 12, color: "var(--colors-text-muted)" }}>{it.hint}</span>}
            </div>
          ))
        )}
      </div>
    </div>
  );
}
"#;

const HINT_TSX: &str = r#"import { ReactNode } from "react";
import { cn } from "./cn";

export interface HintProps {
  tone?: "neutral" | "warning" | "danger";
  icon?: ReactNode;
  children: ReactNode;
  className?: string;
}

export function Hint({ tone = "neutral", icon, children, className }: HintProps) {
  return (
    <span className={cn("aph-hint", tone !== "neutral" && `aph-hint--${tone}`, className)}>
      {icon && <span aria-hidden="true">{icon}</span>}
      <span>{children}</span>
    </span>
  );
}
"#;

// ---- Korean phone input (locale-specific RC.8) ----

const PHONE_INPUT_KR_TSX: &str = r#"import { forwardRef, InputHTMLAttributes, ChangeEvent } from "react";
import { cn } from "./cn";

/** Strip every non-digit and cap at 11 digits (010 prefix + 8). */
export function parseKoreanPhone(raw: string): string {
  return raw.replace(/\D/g, "").slice(0, 11);
}

/** Format a digit-string to 010-XXXX-XXXX (or partial). */
export function formatKoreanPhone(digits: string): string {
  const d = parseKoreanPhone(digits);
  if (d.length < 4) return d;
  if (d.length < 8) return `${d.slice(0, 3)}-${d.slice(3)}`;
  return `${d.slice(0, 3)}-${d.slice(3, 7)}-${d.slice(7)}`;
}

export interface PhoneInputKRProps extends Omit<InputHTMLAttributes<HTMLInputElement>, "type" | "value" | "onChange"> {
  value: string;
  /** Called with the raw 11-digit string (e.g. "01012345678"). */
  onChange: (digits: string) => void;
  error?: boolean;
}

/** Auto-formatting Korean mobile input — accepts paste of any digit/
 *  hyphen pattern and emits a clean 11-digit string back via onChange. */
export const PhoneInputKR = forwardRef<HTMLInputElement, PhoneInputKRProps>(function PhoneInputKR(
  { value, onChange, error, className, placeholder = "010-1234-5678", ...rest },
  ref,
) {
  const display = formatKoreanPhone(value);
  return (
    <input
      ref={ref}
      type="tel"
      inputMode="numeric"
      autoComplete="tel"
      maxLength={13}
      value={display}
      placeholder={placeholder}
      className={cn("aph-input", error && "aph-input--error", className)}
      aria-invalid={error || undefined}
      onChange={(e: ChangeEvent<HTMLInputElement>) => onChange(parseKoreanPhone(e.target.value))}
      {...rest}
    />
  );
});
"#;

const PHONE_INPUT_KR_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { PhoneInputKR } from "./PhoneInputKR";

const meta: Meta<typeof PhoneInputKR> = { component: PhoneInputKR };
export default meta;
type Story = StoryObj<typeof PhoneInputKR>;

export const Default: Story = {
  render: () => {
    const [v, setV] = useState("");
    return (
      <div style={{ display: "flex", flexDirection: "column", gap: 8, width: 280 }}>
        <PhoneInputKR value={v} onChange={setV} />
        <small style={{ color: "var(--colors-text-muted)" }}>raw value: {v || "(empty)"}</small>
      </div>
    );
  },
};

export const Prefilled: Story = {
  render: () => {
    const [v, setV] = useState("01012345678");
    return <PhoneInputKR value={v} onChange={setV} />;
  },
};
"#;

// ---- Stories for RC.7 advanced primitives ----

const DATA_TABLE_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { DataTable } from "./DataTable";

type Tx = { date: string; merchant: string; amount: number; status: string };
const rows: Tx[] = [
  { date: "2026-05-18", merchant: "스타벅스 강남점", amount: -6500, status: "완료" },
  { date: "2026-05-17", merchant: "월급", amount: 3200000, status: "완료" },
  { date: "2026-05-16", merchant: "GS25", amount: -3200, status: "완료" },
  { date: "2026-05-15", merchant: "쿠팡", amount: -42000, status: "대기" },
];

const meta: Meta<typeof DataTable<Tx>> = { component: DataTable };
export default meta;
type Story = StoryObj<typeof DataTable<Tx>>;

export const Transactions: Story = {
  args: {
    rows,
    columns: [
      { key: "date", header: "날짜", sortable: true },
      { key: "merchant", header: "내역", sortable: true },
      { key: "amount", header: "금액", sortable: true, align: "right", cell: (r) => (r.amount > 0 ? `+${r.amount.toLocaleString()}` : `${r.amount.toLocaleString()}`) + "원" },
      { key: "status", header: "상태" },
    ],
  },
};
"#;

const CAROUSEL_STORIES: &str = r###"import type { Meta, StoryObj } from "@storybook/react";
import { Carousel } from "./Carousel";

const meta: Meta<typeof Carousel> = { component: Carousel };
export default meta;
type Story = StoryObj<typeof Carousel>;

export const Hero: Story = {
  render: () => (
    <div style={{ width: 600, height: 280 }}>
      <Carousel ariaLabel="홈 배너">
        {["#16a34a", "#3b82f6", "#dc2626"].map((bg, i) => (
          <div key={i} style={{ background: bg, height: 280, display: "flex", alignItems: "center", justifyContent: "center", color: "#fff", fontSize: 32, fontWeight: 700 }}>슬라이드 {i + 1}</div>
        ))}
      </Carousel>
    </div>
  ),
};
"###;

const CALENDAR_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { Calendar } from "./Calendar";

const meta: Meta<typeof Calendar> = { component: Calendar };
export default meta;
type Story = StoryObj<typeof Calendar>;

export const Default: Story = {
  render: () => {
    const [d, setD] = useState(new Date());
    return <Calendar value={d} onChange={setD} />;
  },
};
"#;

const CHIP_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Chip } from "./Chip";

const meta: Meta<typeof Chip> = { component: Chip };
export default meta;
type Story = StoryObj<typeof Chip>;

export const Closable: Story = {
  args: { children: "서울", onClose: () => alert("removed") },
};
"#;

const IMAGE_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Image } from "./Image";

const meta: Meta<typeof Image> = { component: Image };
export default meta;
type Story = StoryObj<typeof Image>;

export const Fallback: Story = { args: { width: 200, height: 200, src: "nope.jpg", alt: "상품 사진", rounded: 12 } };
"#;

const KBD_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Kbd } from "./Kbd";

const meta: Meta<typeof Kbd> = { component: Kbd };
export default meta;
type Story = StoryObj<typeof Kbd>;

export const Shortcut: Story = {
  render: () => <span><Kbd>⌘</Kbd> + <Kbd>K</Kbd> 로 검색 열기</span>,
};
"#;

const CODE_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Code } from "./Code";

const meta: Meta<typeof Code> = { component: Code };
export default meta;
type Story = StoryObj<typeof Code>;

export const Inline: Story = { render: () => <span><Code>npm i @aphrodite/your-name</Code> 로 설치</span> };
export const Block: Story = { args: { block: true, children: "import { Button } from '@aphrodite/x';\n\n<Button>Hi</Button>" } };
"#;

const TIMELINE_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Timeline, TimelineItem } from "./Timeline";

const meta: Meta<typeof Timeline> = { component: Timeline };
export default meta;
type Story = StoryObj<typeof Timeline>;

export const OrderHistory: Story = {
  render: () => (
    <Timeline>
      <TimelineItem marker="1" title="주문 접수" time="5/17 14:02">결제가 완료되었습니다.</TimelineItem>
      <TimelineItem marker="2" title="상품 준비" time="5/18 09:14" />
      <TimelineItem marker="3" title="배송 시작" time="5/18 11:38" />
    </Timeline>
  ),
};
"#;

const DISCLOSURE_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Disclosure } from "./Disclosure";

const meta: Meta<typeof Disclosure> = { component: Disclosure };
export default meta;
type Story = StoryObj<typeof Disclosure>;

export const Default: Story = {
  args: { title: "자세히 보기", children: <p>여기에 추가 정보가 들어갑니다.</p> },
};
"#;

const CONTEXT_MENU_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { ContextMenu } from "./ContextMenu";

const meta: Meta<typeof ContextMenu> = { component: ContextMenu };
export default meta;
type Story = StoryObj<typeof ContextMenu>;

export const Default: Story = {
  args: {
    trigger: <span style={{ padding: 20, border: "1px dashed", display: "inline-block" }}>우클릭해보세요</span>,
    items: [
      { label: "수정", onSelect: () => alert("수정") },
      { label: "복사", onSelect: () => alert("복사") },
      { label: "삭제", onSelect: () => alert("삭제") },
    ],
  },
};
"#;

const COMMAND_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Command } from "./Command";

const meta: Meta<typeof Command> = { component: Command };
export default meta;
type Story = StoryObj<typeof Command>;

export const Default: Story = {
  args: {
    placeholder: "명령어 또는 검색어 입력...",
    items: [
      { id: "new-doc", label: "새 문서", hint: "⌘N", onSelect: () => {} },
      { id: "search", label: "검색", hint: "⌘K", onSelect: () => {} },
      { id: "settings", label: "설정", hint: "⌘,", onSelect: () => {} },
      { id: "logout", label: "로그아웃", onSelect: () => {} },
    ],
  },
};
"#;

const HINT_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Hint } from "./Hint";

const meta: Meta<typeof Hint> = { component: Hint };
export default meta;
type Story = StoryObj<typeof Hint>;

export const Neutral: Story = { args: { children: "최대 100MB까지 업로드할 수 있습니다." } };
export const Warning: Story = { args: { tone: "warning", icon: "⚠️", children: "이 작업은 되돌릴 수 없습니다." } };
export const Danger: Story = { args: { tone: "danger", icon: "✕", children: "비밀번호가 일치하지 않습니다." } };
"#;

// ---- Stories for RC.5 advanced primitives ----

const SEGMENTED_CONTROL_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { SegmentedControl } from "./SegmentedControl";

const meta: Meta<typeof SegmentedControl> = { component: SegmentedControl };
export default meta;
type Story = StoryObj<typeof SegmentedControl>;

export const Default: Story = {
  render: () => {
    const [v, setV] = useState<"all" | "active" | "done">("active");
    return (
      <SegmentedControl
        value={v}
        onChange={setV}
        options={[
          { value: "all", label: "전체" },
          { value: "active", label: "진행 중" },
          { value: "done", label: "완료" },
        ]}
      />
    );
  },
};
"#;

const PIN_INPUT_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { PinInput } from "./PinInput";

const meta: Meta<typeof PinInput> = { component: PinInput };
export default meta;
type Story = StoryObj<typeof PinInput>;

export const OTP: Story = {
  render: () => {
    const [v, setV] = useState("");
    return <PinInput length={6} value={v} onChange={setV} />;
  },
};
"#;

const NUMBER_INPUT_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { NumberInput } from "./NumberInput";

const meta: Meta<typeof NumberInput> = { component: NumberInput };
export default meta;
type Story = StoryObj<typeof NumberInput>;

export const Quantity: Story = {
  render: () => {
    const [v, setV] = useState(1);
    return <NumberInput value={v} onChange={setV} min={0} max={99} ariaLabel="수량" />;
  },
};
"#;

const SEARCH_INPUT_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { SearchInput } from "./SearchInput";

const meta: Meta<typeof SearchInput> = { component: SearchInput };
export default meta;
type Story = StoryObj<typeof SearchInput>;

export const Default: Story = {
  render: () => {
    const [q, setQ] = useState("");
    return <SearchInput value={q} onChange={setQ} placeholder="상품, 브랜드, 카테고리 검색" />;
  },
};
"#;

const FILE_UPLOADER_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { FileUploader } from "./FileUploader";

const meta: Meta<typeof FileUploader> = { component: FileUploader };
export default meta;
type Story = StoryObj<typeof FileUploader>;

export const Default: Story = {
  render: () => {
    const [files, setFiles] = useState<string[]>([]);
    return (
      <div style={{ width: 360 }}>
        <FileUploader multiple accept="image/*" onFiles={(fs) => setFiles(fs.map((f) => f.name))} />
        {files.length > 0 && <ul>{files.map((f) => <li key={f}>{f}</li>)}</ul>}
      </div>
    );
  },
};
"#;

const DATE_PICKER_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { DatePicker } from "./DatePicker";

const meta: Meta<typeof DatePicker> = { component: DatePicker };
export default meta;
type Story = StoryObj<typeof DatePicker>;

export const Default: Story = { args: { defaultValue: "2026-05-18" } };
"#;

const COMBOBOX_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { Combobox } from "./Combobox";

const meta: Meta<typeof Combobox> = { component: Combobox };
export default meta;
type Story = StoryObj<typeof Combobox>;

export const Cities: Story = {
  render: () => {
    const [v, setV] = useState("seoul");
    return (
      <div style={{ width: 280 }}>
        <Combobox
          value={v}
          onChange={setV}
          ariaLabel="도시 선택"
          placeholder="도시 검색…"
          options={[
            { value: "seoul", label: "서울특별시" },
            { value: "busan", label: "부산광역시" },
            { value: "incheon", label: "인천광역시" },
            { value: "daegu", label: "대구광역시" },
            { value: "daejeon", label: "대전광역시" },
            { value: "gwangju", label: "광주광역시" },
            { value: "ulsan", label: "울산광역시" },
            { value: "jeju", label: "제주특별자치도" },
          ]}
        />
      </div>
    );
  },
};
"#;

const SHEET_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { Sheet } from "./Sheet";
import { Button } from "./Button";

const meta: Meta<typeof Sheet> = { component: Sheet };
export default meta;
type Story = StoryObj<typeof Sheet>;

export const Default: Story = {
  render: () => {
    const [open, setOpen] = useState(false);
    return (
      <>
        <Button onClick={() => setOpen(true)}>바텀시트 열기</Button>
        <Sheet open={open} onClose={() => setOpen(false)} title="계좌 선택">
          <p>출금하실 계좌를 선택해주세요.</p>
          <Button onClick={() => setOpen(false)}>완료</Button>
        </Sheet>
      </>
    );
  },
};
"#;

const ALERT_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Alert } from "./Alert";

const meta: Meta<typeof Alert> = { component: Alert };
export default meta;
type Story = StoryObj<typeof Alert>;

export const Info: Story = { args: { tone: "info", title: "안내", children: "본인 인증이 필요합니다." } };
export const Success: Story = { args: { tone: "success", title: "전송 완료", children: "이체가 완료되었습니다." } };
export const Warning: Story = { args: { tone: "warning", title: "주의", children: "잔액이 부족합니다." } };
export const Danger: Story = { args: { tone: "danger", title: "오류", children: "결제에 실패했습니다." } };
"#;

const STAT_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Stat } from "./Stat";

const meta: Meta<typeof Stat> = { component: Stat };
export default meta;
type Story = StoryObj<typeof Stat>;

export const Default: Story = {
  args: {
    label: "월 거래액",
    value: "₩ 12,456,789",
    delta: { value: "+8.4%", direction: "up" },
  },
};
"#;

const TOOLBAR_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Toolbar } from "./Toolbar";
import { Button } from "./Button";

const meta: Meta<typeof Toolbar> = { component: Toolbar };
export default meta;
type Story = StoryObj<typeof Toolbar>;

export const Default: Story = {
  render: () => (
    <Toolbar ariaLabel="문서 편집">
      <Button variant="ghost" size="sm">굵게</Button>
      <Button variant="ghost" size="sm">기울임</Button>
      <Toolbar.Separator />
      <Button variant="ghost" size="sm">왼쪽 정렬</Button>
      <Button variant="ghost" size="sm">가운데</Button>
    </Toolbar>
  ),
};
"#;

const HOVER_CARD_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { HoverCard } from "./HoverCard";
import { Avatar } from "./Avatar";

const meta: Meta<typeof HoverCard> = { component: HoverCard };
export default meta;
type Story = StoryObj<typeof HoverCard>;

export const UserCard: Story = {
  render: () => (
    <HoverCard trigger={<Avatar initials="JK" alt="Jihyo Kim" />}>
      <div style={{ display: "flex", flexDirection: "column", gap: 4 }}>
        <strong>김지효</strong>
        <small>jihyo@toss.im</small>
        <small>마지막 활동: 5분 전</small>
      </div>
    </HoverCard>
  ),
};
"#;

// ---- Stories for extended primitives ----

const TEXTAREA_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Textarea } from "./Textarea";

const meta: Meta<typeof Textarea> = { component: Textarea, args: { placeholder: "내용을 입력해주세요" } };
export default meta;
type Story = StoryObj<typeof Textarea>;

export const Default: Story = {};
export const Error: Story = { args: { error: true, defaultValue: "에러" } };
"#;

const SELECT_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Select } from "./Select";

const meta: Meta<typeof Select> = {
  component: Select,
  args: { children: <><option value="kr">대한민국</option><option value="jp">일본</option><option value="us">미국</option></> },
};
export default meta;
type Story = StoryObj<typeof Select>;

export const Default: Story = {};
"#;

const CHECKBOX_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Checkbox } from "./Checkbox";

const meta: Meta<typeof Checkbox> = { component: Checkbox, args: { label: "이용약관에 동의합니다" } };
export default meta;
type Story = StoryObj<typeof Checkbox>;

export const Default: Story = {};
export const Checked: Story = { args: { defaultChecked: true } };
export const Disabled: Story = { args: { disabled: true } };
"#;

const RADIO_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Radio } from "./Radio";

const meta: Meta<typeof Radio> = { component: Radio, args: { label: "개인", name: "membership", value: "personal" } };
export default meta;
type Story = StoryObj<typeof Radio>;

export const Default: Story = {};
"#;

const RADIO_GROUP_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { RadioGroup } from "./RadioGroup";
import { Radio } from "./Radio";

const meta: Meta<typeof RadioGroup> = { component: RadioGroup };
export default meta;
type Story = StoryObj<typeof RadioGroup>;

export const Default: Story = {
  render: () => {
    const [v, setV] = useState("personal");
    return (
      <RadioGroup name="membership" value={v} onChange={setV}>
        <Radio value="personal" label="개인" />
        <Radio value="business" label="사업자" />
        <Radio value="foreign" label="외국인" />
      </RadioGroup>
    );
  },
};
"#;

const TABS_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Tabs, TabList, Tab, TabPanel } from "./Tabs";

const meta: Meta<typeof Tabs> = { component: Tabs };
export default meta;
type Story = StoryObj<typeof Tabs>;

export const Default: Story = {
  render: () => (
    <Tabs defaultValue="account">
      <TabList>
        <Tab value="account">계정</Tab>
        <Tab value="notifications">알림</Tab>
        <Tab value="security">보안</Tab>
      </TabList>
      <TabPanel value="account">계정 정보 내용</TabPanel>
      <TabPanel value="notifications">알림 설정 내용</TabPanel>
      <TabPanel value="security">보안 설정 내용</TabPanel>
    </Tabs>
  ),
};
"#;

const ACCORDION_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Accordion, AccordionItem } from "./Accordion";

const meta: Meta<typeof Accordion> = { component: Accordion };
export default meta;
type Story = StoryObj<typeof Accordion>;

export const Default: Story = {
  render: () => (
    <Accordion>
      <AccordionItem title="환불 정책이 어떻게 되나요?">7일 이내 전액 환불 가능합니다.</AccordionItem>
      <AccordionItem title="해외에서도 사용할 수 있나요?">현재 한국과 일본에서 사용 가능합니다.</AccordionItem>
      <AccordionItem title="문의는 어디로 하나요?">help@example.com 으로 문의 부탁드립니다.</AccordionItem>
    </Accordion>
  ),
};
"#;

const TOAST_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { ToastProvider, useToast } from "./Toast";
import { Button } from "./Button";

const meta: Meta<typeof ToastProvider> = { component: ToastProvider };
export default meta;
type Story = StoryObj<typeof ToastProvider>;

function Demo() {
  const { push } = useToast();
  return (
    <div style={{ display: "flex", gap: 8 }}>
      <Button onClick={() => push({ message: "저장되었습니다" })}>기본</Button>
      <Button variant="secondary" onClick={() => push({ message: "전송 완료", tone: "success" })}>성공</Button>
      <Button variant="danger" onClick={() => push({ message: "오류가 발생했습니다", tone: "danger" })}>위험</Button>
    </div>
  );
}

export const Default: Story = {
  render: () => <ToastProvider><Demo /></ToastProvider>,
};
"#;

const TOOLTIP_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Tooltip } from "./Tooltip";
import { Button } from "./Button";

const meta: Meta<typeof Tooltip> = { component: Tooltip };
export default meta;
type Story = StoryObj<typeof Tooltip>;

export const Default: Story = {
  render: () => <Tooltip content="저장 (⌘S)"><Button>저장</Button></Tooltip>,
};
"#;

const POPOVER_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Popover } from "./Popover";
import { Button } from "./Button";

const meta: Meta<typeof Popover> = { component: Popover };
export default meta;
type Story = StoryObj<typeof Popover>;

export const Default: Story = {
  render: () => (
    <Popover trigger={<Button variant="secondary">필터</Button>}>
      <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
        <label><input type="checkbox" /> 진행 중</label>
        <label><input type="checkbox" /> 완료</label>
        <label><input type="checkbox" /> 취소</label>
      </div>
    </Popover>
  ),
};
"#;

const MENU_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Menu, MenuItem } from "./Menu";

const meta: Meta<typeof Menu> = { component: Menu };
export default meta;
type Story = StoryObj<typeof Menu>;

export const Default: Story = {
  render: () => (
    <Menu ariaLabel="사용자 메뉴">
      <MenuItem>프로필</MenuItem>
      <MenuItem>설정</MenuItem>
      <MenuItem>도움말</MenuItem>
      <MenuItem>로그아웃</MenuItem>
    </Menu>
  ),
};
"#;

const PROGRESS_BAR_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { ProgressBar } from "./ProgressBar";

const meta: Meta<typeof ProgressBar> = { component: ProgressBar };
export default meta;
type Story = StoryObj<typeof ProgressBar>;

export const ThirtyPercent: Story = { args: { value: 30, label: "업로드 진행률" } };
export const Complete: Story = { args: { value: 100, label: "완료" } };
"#;

const STEPPER_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Stepper } from "./Stepper";

const meta: Meta<typeof Stepper> = { component: Stepper };
export default meta;
type Story = StoryObj<typeof Stepper>;

export const InProgress: Story = {
  args: {
    current: 1,
    steps: [{ label: "본인 인증" }, { label: "계좌 등록" }, { label: "완료" }],
  },
};
"#;

const SLIDER_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Slider } from "./Slider";

const meta: Meta<typeof Slider> = { component: Slider, args: { min: 0, max: 100, defaultValue: 30 } };
export default meta;
type Story = StoryObj<typeof Slider>;

export const Default: Story = {};
"#;

const BREADCRUMB_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Breadcrumb } from "./Breadcrumb";

const meta: Meta<typeof Breadcrumb> = { component: Breadcrumb };
export default meta;
type Story = StoryObj<typeof Breadcrumb>;

export const Default: Story = {
  args: {
    items: [
      { label: "홈", href: "/" },
      { label: "투자", href: "/invest" },
      { label: "삼성전자" },
    ],
  },
};
"#;

const PAGINATION_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { Pagination } from "./Pagination";

const meta: Meta<typeof Pagination> = { component: Pagination };
export default meta;
type Story = StoryObj<typeof Pagination>;

export const Default: Story = {
  render: () => {
    const [p, setP] = useState(3);
    return <Pagination page={p} pageCount={20} onChange={setP} />;
  },
};
"#;

const DIVIDER_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { Divider } from "./Divider";

const meta: Meta<typeof Divider> = { component: Divider };
export default meta;
type Story = StoryObj<typeof Divider>;

export const Horizontal: Story = {};
export const Vertical: Story = { args: { orientation: "vertical" } };
"#;

const EMPTY_STATE_STORIES: &str = r#"import type { Meta, StoryObj } from "@storybook/react";
import { EmptyState } from "./EmptyState";
import { Button } from "./Button";

const meta: Meta<typeof EmptyState> = { component: EmptyState };
export default meta;
type Story = StoryObj<typeof EmptyState>;

export const Default: Story = {
  args: {
    title: "거래 내역이 없습니다",
    description: "이번 달은 아직 거래가 없네요.",
    action: <Button>거래 시작하기</Button>,
  },
};
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
    fn stories_files_emitted_for_every_component() {
        let pkg = build(&fixture(), "x");
        for c in [
            "Button", "Input", "Tag", "Avatar", "Card", "Modal", "Drawer", "Skeleton",
            "FormField", "Switch", "Badge", "Spinner",
            "Textarea", "Select", "Checkbox", "Radio", "RadioGroup", "Tabs", "Accordion",
            "Toast", "Tooltip", "Popover", "Menu", "ProgressBar", "Stepper", "Slider",
            "Breadcrumb", "Pagination", "Divider", "EmptyState",
        ] {
            let key = format!("src/{c}.stories.tsx");
            assert!(pkg.files.contains_key(&key), "missing stories file: {key}");
            let body = &pkg.files[&key];
            assert!(body.contains("@storybook/react"), "{c} story missing storybook import");
        }
    }

    #[test]
    fn full_component_count_is_55() {
        let pkg = build(&fixture(), "x");
        let tsx_count = pkg.files.keys()
            .filter(|k| k.ends_with(".tsx") && !k.ends_with(".stories.tsx"))
            .count();
        assert_eq!(tsx_count, 55, "expected 55 component .tsx files, got {tsx_count}");
        let story_count = pkg.files.keys().filter(|k| k.ends_with(".stories.tsx")).count();
        assert_eq!(story_count, 55, "expected 55 story files, got {story_count}");
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
