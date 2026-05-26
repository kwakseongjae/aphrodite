import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { Toggle } from "./Toggle";

const meta: Meta<typeof Toggle> = { component: Toggle };
export default meta;
type Story = StoryObj<typeof Toggle>;

export const Bold: Story = {
  render: () => {
    const [on, setOn] = useState(false);
    return <Toggle pressed={on} onPressedChange={setOn}>B</Toggle>;
  },
};
