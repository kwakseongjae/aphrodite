import type { Meta, StoryObj } from "@storybook/react";
import { Textarea } from "./Textarea";

const meta: Meta<typeof Textarea> = { component: Textarea, args: { placeholder: "내용을 입력해주세요" } };
export default meta;
type Story = StoryObj<typeof Textarea>;

export const Default: Story = {};
export const Error: Story = { args: { error: true, defaultValue: "에러" } };
