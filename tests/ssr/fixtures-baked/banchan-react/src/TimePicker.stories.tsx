import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { TimePicker } from "./TimePicker";

const meta: Meta<typeof TimePicker> = { component: TimePicker };
export default meta;
type Story = StoryObj<typeof TimePicker>;

export const Default: Story = {
  render: () => {
    const [t, setT] = useState("09:30");
    return <TimePicker value={t} onChange={setT} ariaLabel="배송 희망 시간" />;
  },
};
