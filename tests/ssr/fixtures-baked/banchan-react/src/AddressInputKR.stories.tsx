import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { AddressInputKR } from "./AddressInputKR";

const meta: Meta<typeof AddressInputKR> = { component: AddressInputKR };
export default meta;
type Story = StoryObj<typeof AddressInputKR>;

export const Default: Story = {
  render: () => {
    const [v, setV] = useState({ postcode: "", road: "", detail: "" });
    return (
      <div style={{ width: 360 }}>
        <AddressInputKR
          value={v}
          onChange={setV}
          onLookup={(apply) => {
            // wire Daum/Kakao postcode service here
            apply("04524", "서울특별시 중구 세종대로 110");
          }}
        />
      </div>
    );
  },
};
