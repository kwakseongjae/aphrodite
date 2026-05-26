import type { Meta, StoryObj } from "@storybook/react";
import { ChartBar } from "./ChartBar";

const meta: Meta<typeof ChartBar> = { component: ChartBar };
export default meta;
type Story = StoryObj<typeof ChartBar>;

export const WeeklyOrders: Story = {
  args: {
    ariaLabel: "요일별 주문",
    bars: [
      { label: "월", value: 24 }, { label: "화", value: 18 }, { label: "수", value: 27 },
      { label: "목", value: 22 }, { label: "금", value: 35 }, { label: "토", value: 41 }, { label: "일", value: 38 },
    ],
  },
};
