import type { Meta, StoryObj } from "@storybook/react";
import { EmptyState } from "./EmptyState";
import { Button } from "./Button";

const meta: Meta<typeof EmptyState> = { component: EmptyState };
export default meta;
type Story = StoryObj<typeof EmptyState>;

export const Default: Story = {
  args: {
    title: "거래 내역이 없습니다",
    description: "이번 달은 아직 거래가 없네요.",
    action: <Button>거래 시작하기</Button>,
  },
};
