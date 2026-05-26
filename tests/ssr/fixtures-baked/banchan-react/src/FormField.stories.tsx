import type { Meta, StoryObj } from "@storybook/react";
import { FormField } from "./FormField";
import { Input } from "./Input";

const meta: Meta<typeof FormField> = { component: FormField };
export default meta;
type Story = StoryObj<typeof FormField>;

export const WithHint: Story = {
  args: {
    label: "이메일",
    hint: "회사 이메일을 사용해주세요",
    children: <Input placeholder="name@toss.im" />,
  },
};

export const WithError: Story = {
  args: {
    label: "비밀번호",
    error: "8자 이상이어야 합니다",
    children: <Input type="password" error defaultValue="abc" />,
  },
};
