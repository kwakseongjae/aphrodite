import type { Meta, StoryObj } from "@storybook/react";
import { CodeBlock } from "./CodeBlock";

const meta: Meta<typeof CodeBlock> = { component: CodeBlock };
export default meta;
type Story = StoryObj<typeof CodeBlock>;

export const TypeScript: Story = {
  args: {
    language: "tsx",
    children: "import { Button } from \"@aphrodite-design/x\";\n\nexport default function Page() {\n  return <Button>시작하기</Button>;\n}",
  },
};
