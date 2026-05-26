import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { PinInput } from "./PinInput";

const meta: Meta<typeof PinInput> = { component: PinInput };
export default meta;
type Story = StoryObj<typeof PinInput>;

export const OTP: Story = {
  render: () => {
    const [v, setV] = useState("");
    return <PinInput length={6} value={v} onChange={setV} />;
  },
};
