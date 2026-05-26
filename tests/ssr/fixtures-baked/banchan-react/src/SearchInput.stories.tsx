import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { SearchInput } from "./SearchInput";

const meta: Meta<typeof SearchInput> = { component: SearchInput };
export default meta;
type Story = StoryObj<typeof SearchInput>;

export const Default: Story = {
  render: () => {
    const [q, setQ] = useState("");
    return <SearchInput value={q} onChange={setQ} placeholder="상품, 브랜드, 카테고리 검색" />;
  },
};
