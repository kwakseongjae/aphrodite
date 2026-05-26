import type { Meta, StoryObj } from "@storybook/react";
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
