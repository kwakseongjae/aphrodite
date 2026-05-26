import type { Meta, StoryObj } from "@storybook/react";
import { ProgressBar } from "./ProgressBar";

const meta: Meta<typeof ProgressBar> = { component: ProgressBar };
export default meta;
type Story = StoryObj<typeof ProgressBar>;

export const ThirtyPercent: Story = { args: { value: 30, label: "업로드 진행률" } };
export const Complete: Story = { args: { value: 100, label: "완료" } };
