import type { Meta, StoryObj } from "@storybook/react";
import { Kbd } from "./Kbd";

const meta: Meta<typeof Kbd> = { component: Kbd };
export default meta;
type Story = StoryObj<typeof Kbd>;

export const Shortcut: Story = {
  render: () => <span><Kbd>⌘</Kbd> + <Kbd>K</Kbd> 로 검색 열기</span>,
};
