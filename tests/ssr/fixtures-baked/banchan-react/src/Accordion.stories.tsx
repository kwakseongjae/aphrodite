import type { Meta, StoryObj } from "@storybook/react";
import { Accordion, AccordionItem } from "./Accordion";

const meta: Meta<typeof Accordion> = { component: Accordion };
export default meta;
type Story = StoryObj<typeof Accordion>;

export const Default: Story = {
  render: () => (
    <Accordion>
      <AccordionItem title="환불 정책이 어떻게 되나요?">7일 이내 전액 환불 가능합니다.</AccordionItem>
      <AccordionItem title="해외에서도 사용할 수 있나요?">현재 한국과 일본에서 사용 가능합니다.</AccordionItem>
      <AccordionItem title="문의는 어디로 하나요?">help@example.com 으로 문의 부탁드립니다.</AccordionItem>
    </Accordion>
  ),
};
