import type { Meta, StoryObj } from "@storybook/react";
import { Skeleton } from "./Skeleton";

const meta: Meta<typeof Skeleton> = { component: Skeleton };
export default meta;
type Story = StoryObj<typeof Skeleton>;

export const Line: Story = { args: { width: 240, height: 16 } };
export const Block: Story = { args: { width: 320, height: 120 } };
export const Circle: Story = { args: { width: 48, height: 48, circle: true } };
