import type { Meta, StoryObj } from "@storybook/react";
import { Command } from "./Command";

const meta: Meta<typeof Command> = { component: Command };
export default meta;
type Story = StoryObj<typeof Command>;

export const Default: Story = {
  args: {
    placeholder: "명령어 또는 검색어 입력...",
    items: [
      { id: "new-doc", label: "새 문서", hint: "⌘N", onSelect: () => {} },
      { id: "search", label: "검색", hint: "⌘K", onSelect: () => {} },
      { id: "settings", label: "설정", hint: "⌘,", onSelect: () => {} },
      { id: "logout", label: "로그아웃", onSelect: () => {} },
    ],
  },
};
