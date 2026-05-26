import type { Meta, StoryObj } from "@storybook/react";
import { Disclosure } from "./Disclosure";

const meta: Meta<typeof Disclosure> = { component: Disclosure };
export default meta;
type Story = StoryObj<typeof Disclosure>;

export const Default: Story = {
  args: { title: "자세히 보기", children: <p>여기에 추가 정보가 들어갑니다.</p> },
};
