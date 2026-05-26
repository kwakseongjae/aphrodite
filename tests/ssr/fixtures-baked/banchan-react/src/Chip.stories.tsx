import type { Meta, StoryObj } from "@storybook/react";
import { Chip } from "./Chip";

const meta: Meta<typeof Chip> = { component: Chip };
export default meta;
type Story = StoryObj<typeof Chip>;

export const Closable: Story = {
  args: { children: "서울", onClose: () => alert("removed") },
};
