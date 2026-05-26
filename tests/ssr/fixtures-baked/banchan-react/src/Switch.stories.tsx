import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { Switch } from "./Switch";

const meta: Meta<typeof Switch> = { component: Switch };
export default meta;
type Story = StoryObj<typeof Switch>;

export const Default: Story = {
  render: () => {
    const [on, setOn] = useState(true);
    return <Switch checked={on} onChange={setOn} label="알림 받기" />;
  },
};
