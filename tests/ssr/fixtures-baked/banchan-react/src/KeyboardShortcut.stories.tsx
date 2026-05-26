import type { Meta, StoryObj } from "@storybook/react";
import { KeyboardShortcut } from "./KeyboardShortcut";

const meta: Meta<typeof KeyboardShortcut> = { component: KeyboardShortcut };
export default meta;
type Story = StoryObj<typeof KeyboardShortcut>;

export const Search: Story = { args: { label: "검색", keys: ["⌘", "K"] } };
export const Save: Story = { args: { label: "저장", keys: ["⌘", "S"] } };
