import type { Meta, StoryObj } from "@storybook/react";
import { Breadcrumb } from "./Breadcrumb";

const meta: Meta<typeof Breadcrumb> = { component: Breadcrumb };
export default meta;
type Story = StoryObj<typeof Breadcrumb>;

export const Default: Story = {
  args: {
    items: [
      { label: "홈", href: "/" },
      { label: "투자", href: "/invest" },
      { label: "삼성전자" },
    ],
  },
};
