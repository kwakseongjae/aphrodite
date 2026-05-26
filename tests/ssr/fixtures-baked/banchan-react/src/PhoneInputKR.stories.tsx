import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { PhoneInputKR } from "./PhoneInputKR";

const meta: Meta<typeof PhoneInputKR> = { component: PhoneInputKR };
export default meta;
type Story = StoryObj<typeof PhoneInputKR>;

export const Default: Story = {
  render: () => {
    const [v, setV] = useState("");
    return (
      <div style={{ display: "flex", flexDirection: "column", gap: 8, width: 280 }}>
        <PhoneInputKR value={v} onChange={setV} />
        <small style={{ color: "var(--colors-text-muted)" }}>raw value: {v || "(empty)"}</small>
      </div>
    );
  },
};

export const Prefilled: Story = {
  render: () => {
    const [v, setV] = useState("01012345678");
    return <PhoneInputKR value={v} onChange={setV} />;
  },
};
