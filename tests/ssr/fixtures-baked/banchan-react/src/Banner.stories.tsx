import type { Meta, StoryObj } from "@storybook/react";
import { Banner } from "./Banner";

const meta: Meta<typeof Banner> = { component: Banner };
export default meta;
type Story = StoryObj<typeof Banner>;

export const Info: Story = { args: { tone: "info", dismissible: true, children: "6월 1일부터 새로운 배송 정책이 적용됩니다." } };
export const Warning: Story = { args: { tone: "warning", dismissible: true, children: "본인 인증을 완료해주세요." } };
export const Danger: Story = { args: { tone: "danger", children: "결제 처리 중 오류가 발생했습니다." } };
