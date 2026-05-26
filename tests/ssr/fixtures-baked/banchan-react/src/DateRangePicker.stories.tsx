import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { DateRangePicker } from "./DateRangePicker";

const meta: Meta<typeof DateRangePicker> = { component: DateRangePicker };
export default meta;
type Story = StoryObj<typeof DateRangePicker>;

export const Default: Story = {
  render: () => {
    const [v, setV] = useState({ start: "2026-05-01", end: "2026-05-31" });
    return <DateRangePicker value={v} onChange={setV} />;
  },
};
