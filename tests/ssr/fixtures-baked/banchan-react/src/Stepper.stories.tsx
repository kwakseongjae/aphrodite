import type { Meta, StoryObj } from "@storybook/react";
import { Stepper } from "./Stepper";

const meta: Meta<typeof Stepper> = { component: Stepper };
export default meta;
type Story = StoryObj<typeof Stepper>;

export const InProgress: Story = {
  args: {
    current: 1,
    steps: [{ label: "본인 인증" }, { label: "계좌 등록" }, { label: "완료" }],
  },
};
