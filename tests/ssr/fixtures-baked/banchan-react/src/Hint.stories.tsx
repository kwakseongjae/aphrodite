import type { Meta, StoryObj } from "@storybook/react";
import { Hint } from "./Hint";

const meta: Meta<typeof Hint> = { component: Hint };
export default meta;
type Story = StoryObj<typeof Hint>;

export const Neutral: Story = { args: { children: "최대 100MB까지 업로드할 수 있습니다." } };
export const Warning: Story = { args: { tone: "warning", icon: "⚠️", children: "이 작업은 되돌릴 수 없습니다." } };
export const Danger: Story = { args: { tone: "danger", icon: "✕", children: "비밀번호가 일치하지 않습니다." } };
