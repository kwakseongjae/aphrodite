import type { Meta, StoryObj } from "@storybook/react";
import { Badge } from "./Badge";

const meta: Meta<typeof Badge> = { component: Badge };
export default meta;
type Story = StoryObj<typeof Badge>;

export const Count: Story = { args: { count: 3 } };
export const Overflow: Story = { args: { count: 124, max: 99 } };
export const Dot: Story = { args: { dot: true } };
