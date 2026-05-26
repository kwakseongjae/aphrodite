import type { Meta, StoryObj } from "@storybook/react";
import { Quote } from "./Quote";

const meta: Meta<typeof Quote> = { component: Quote };
export default meta;
type Story = StoryObj<typeof Quote>;

export const Editorial: Story = {
  args: {
    children: "신선한 제철 재료로 만든 진정한 한국 음식을 집까지 즐기다.",
    cite: "김민주, 서울",
  },
};
