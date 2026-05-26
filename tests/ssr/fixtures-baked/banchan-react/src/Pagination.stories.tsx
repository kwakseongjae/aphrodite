import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { Pagination } from "./Pagination";

const meta: Meta<typeof Pagination> = { component: Pagination };
export default meta;
type Story = StoryObj<typeof Pagination>;

export const Default: Story = {
  render: () => {
    const [p, setP] = useState(3);
    return <Pagination page={p} pageCount={20} onChange={setP} />;
  },
};
