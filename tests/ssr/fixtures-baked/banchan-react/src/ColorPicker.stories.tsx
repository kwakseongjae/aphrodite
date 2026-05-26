import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { ColorPicker } from "./ColorPicker";

const meta: Meta<typeof ColorPicker> = { component: ColorPicker };
export default meta;
type Story = StoryObj<typeof ColorPicker>;

export const Default: Story = {
  render: () => {
    const [c, setC] = useState("#16a34a");
    return <ColorPicker value={c} onChange={setC} ariaLabel="Brand color" />;
  },
};
