import type { Meta, StoryObj } from "@storybook/react";
import { DescriptionList } from "./DescriptionList";

const meta: Meta<typeof DescriptionList> = { component: DescriptionList };
export default meta;
type Story = StoryObj<typeof DescriptionList>;

export const ProductSpecs: Story = {
  args: {
    items: [
      { label: "원산지", value: "국내산 (강원도 평창)" },
      { label: "유통기한", value: "냉장 보관 5일" },
      { label: "용량", value: "300g × 5팩" },
      { label: "보관 방법", value: "0~4°C 냉장" },
    ],
  },
};
