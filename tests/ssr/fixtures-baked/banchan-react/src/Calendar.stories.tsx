import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { Calendar } from "./Calendar";

const meta: Meta<typeof Calendar> = { component: Calendar };
export default meta;
type Story = StoryObj<typeof Calendar>;

export const Default: Story = {
  render: () => {
    const [d, setD] = useState(new Date());
    return <Calendar value={d} onChange={setD} />;
  },
};
