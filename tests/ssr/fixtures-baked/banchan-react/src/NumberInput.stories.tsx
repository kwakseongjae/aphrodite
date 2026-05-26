import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { NumberInput } from "./NumberInput";

const meta: Meta<typeof NumberInput> = { component: NumberInput };
export default meta;
type Story = StoryObj<typeof NumberInput>;

export const Quantity: Story = {
  render: () => {
    const [v, setV] = useState(1);
    return <NumberInput value={v} onChange={setV} min={0} max={99} ariaLabel="수량" />;
  },
};
