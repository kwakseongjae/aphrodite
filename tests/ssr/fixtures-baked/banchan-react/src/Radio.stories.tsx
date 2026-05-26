import type { Meta, StoryObj } from "@storybook/react";
import { Radio } from "./Radio";

const meta: Meta<typeof Radio> = { component: Radio, args: { label: "개인", name: "membership", value: "personal" } };
export default meta;
type Story = StoryObj<typeof Radio>;

export const Default: Story = {};
