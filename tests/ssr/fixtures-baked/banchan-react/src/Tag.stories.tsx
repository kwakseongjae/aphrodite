import type { Meta, StoryObj } from "@storybook/react";
import { Tag } from "./Tag";

const meta: Meta<typeof Tag> = { component: Tag, args: { children: "신규" } };
export default meta;
type Story = StoryObj<typeof Tag>;

export const Neutral: Story = { args: { tone: "neutral" } };
export const Success: Story = { args: { tone: "success", children: "완료" } };
export const Warning: Story = { args: { tone: "warning", children: "주의" } };
export const Danger: Story = { args: { tone: "danger", children: "실패" } };
