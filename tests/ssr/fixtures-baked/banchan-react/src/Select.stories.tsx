import type { Meta, StoryObj } from "@storybook/react";
import { Select } from "./Select";

const meta: Meta<typeof Select> = {
  component: Select,
  args: { children: <><option value="kr">대한민국</option><option value="jp">일본</option><option value="us">미국</option></> },
};
export default meta;
type Story = StoryObj<typeof Select>;

export const Default: Story = {};
