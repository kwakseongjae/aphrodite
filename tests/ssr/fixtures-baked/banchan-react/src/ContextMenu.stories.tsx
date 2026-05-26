import type { Meta, StoryObj } from "@storybook/react";
import { ContextMenu } from "./ContextMenu";

const meta: Meta<typeof ContextMenu> = { component: ContextMenu };
export default meta;
type Story = StoryObj<typeof ContextMenu>;

export const Default: Story = {
  args: {
    trigger: <span style={{ padding: 20, border: "1px dashed", display: "inline-block" }}>우클릭해보세요</span>,
    items: [
      { label: "수정", onSelect: () => alert("수정") },
      { label: "복사", onSelect: () => alert("복사") },
      { label: "삭제", onSelect: () => alert("삭제") },
    ],
  },
};
