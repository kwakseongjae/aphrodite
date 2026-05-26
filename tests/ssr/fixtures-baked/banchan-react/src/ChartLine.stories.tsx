import type { Meta, StoryObj } from "@storybook/react";
import { ChartLine } from "./ChartLine";

const meta: Meta<typeof ChartLine> = { component: ChartLine };
export default meta;
type Story = StoryObj<typeof ChartLine>;

export const MonthlyRevenue: Story = {
  args: {
    ariaLabel: "월별 매출",
    points: [
      { x: "1월", y: 12 }, { x: "2월", y: 19 }, { x: "3월", y: 14 },
      { x: "4월", y: 22 }, { x: "5월", y: 27 }, { x: "6월", y: 31 },
    ],
  },
};
