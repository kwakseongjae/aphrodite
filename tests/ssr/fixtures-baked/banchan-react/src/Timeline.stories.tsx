import type { Meta, StoryObj } from "@storybook/react";
import { Timeline, TimelineItem } from "./Timeline";

const meta: Meta<typeof Timeline> = { component: Timeline };
export default meta;
type Story = StoryObj<typeof Timeline>;

export const OrderHistory: Story = {
  render: () => (
    <Timeline>
      <TimelineItem marker="1" title="주문 접수" time="5/17 14:02">결제가 완료되었습니다.</TimelineItem>
      <TimelineItem marker="2" title="상품 준비" time="5/18 09:14" />
      <TimelineItem marker="3" title="배송 시작" time="5/18 11:38" />
    </Timeline>
  ),
};
