import type { Meta, StoryObj } from "@storybook/react";
import { Alert } from "./Alert";

const meta: Meta<typeof Alert> = { component: Alert };
export default meta;
type Story = StoryObj<typeof Alert>;

export const Info: Story = { args: { tone: "info", title: "안내", children: "본인 인증이 필요합니다." } };
export const Success: Story = { args: { tone: "success", title: "전송 완료", children: "이체가 완료되었습니다." } };
export const Warning: Story = { args: { tone: "warning", title: "주의", children: "잔액이 부족합니다." } };
export const Danger: Story = { args: { tone: "danger", title: "오류", children: "결제에 실패했습니다." } };
