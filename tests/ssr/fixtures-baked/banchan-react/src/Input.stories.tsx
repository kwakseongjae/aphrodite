import type { Meta, StoryObj } from "@storybook/react";
import { Input } from "./Input";

const meta: Meta<typeof Input> = { component: Input, args: { placeholder: "이메일을 입력해주세요" } };
export default meta;
type Story = StoryObj<typeof Input>;

export const Default: Story = {};
export const Error: Story = { args: { error: true, defaultValue: "잘못된 이메일" } };
export const Disabled: Story = { args: { disabled: true, defaultValue: "user@toss.im" } };
