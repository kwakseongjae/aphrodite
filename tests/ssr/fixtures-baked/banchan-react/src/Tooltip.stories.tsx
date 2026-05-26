import type { Meta, StoryObj } from "@storybook/react";
import { Tooltip } from "./Tooltip";
import { Button } from "./Button";

const meta: Meta<typeof Tooltip> = { component: Tooltip };
export default meta;
type Story = StoryObj<typeof Tooltip>;

export const Default: Story = {
  render: () => <Tooltip content="저장 (⌘S)"><Button>저장</Button></Tooltip>,
};
