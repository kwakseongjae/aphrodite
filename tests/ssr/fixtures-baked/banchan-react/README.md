# Banchan — React component library

Auto-generated from an Aphrodite DESIGN.md. **Do not edit `src/` by hand** —
those files are regenerated on every `aphrodite create` / `aphrodite react`
run. Tokens and component primitives live here so your product
codebase can `npm i @aphrodite-design/banchan` and import directly.

## Quick start

```tsx
import { Button, Input, Card } from "@aphrodite-design/banchan";
import "@aphrodite-design/banchan/styles.css";

export function Page() {
  return (
    <Card>
      <Input label="이메일" placeholder="example@toss.im" />
      <Button variant="primary">계속하기</Button>
    </Card>
  );
}
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
