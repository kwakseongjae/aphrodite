import type { Meta, StoryObj } from "@storybook/react";
import { Code } from "./Code";

const meta: Meta<typeof Code> = { component: Code };
export default meta;
type Story = StoryObj<typeof Code>;

export const Inline: Story = { render: () => <span><Code>npm i @aphrodite-design/your-name</Code> 로 설치</span> };
export const Block: Story = { args: { block: true, children: "import { Button } from '@aphrodite-design/x';\n\n<Button>Hi</Button>" } };
