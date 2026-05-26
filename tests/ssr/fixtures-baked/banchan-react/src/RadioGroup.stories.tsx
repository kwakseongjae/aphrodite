import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { RadioGroup } from "./RadioGroup";
import { Radio } from "./Radio";

const meta: Meta<typeof RadioGroup> = { component: RadioGroup };
export default meta;
type Story = StoryObj<typeof RadioGroup>;

export const Default: Story = {
  render: () => {
    const [v, setV] = useState("personal");
    return (
      <RadioGroup name="membership" value={v} onChange={setV}>
        <Radio value="personal" label="개인" />
        <Radio value="business" label="사업자" />
        <Radio value="foreign" label="외국인" />
      </RadioGroup>
    );
  },
};
