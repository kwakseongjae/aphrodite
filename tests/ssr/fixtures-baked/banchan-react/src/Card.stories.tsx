import type { Meta, StoryObj } from "@storybook/react";
import { Card } from "./Card";

const meta: Meta<typeof Card> = { component: Card };
export default meta;
type Story = StoryObj<typeof Card>;

export const Default: Story = {
  args: { children: <><h3 style={{ margin: 0 }}>월 정산 내역</h3><p style={{ marginTop: 8 }}>2026년 5월 1일 ~ 5월 31일</p></> },
};
