import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { SegmentedControl } from "./SegmentedControl";

const meta: Meta<typeof SegmentedControl> = { component: SegmentedControl };
export default meta;
type Story = StoryObj<typeof SegmentedControl>;

export const Default: Story = {
  render: () => {
    const [v, setV] = useState<"all" | "active" | "done">("active");
    return (
      <SegmentedControl
        value={v}
        onChange={setV}
        options={[
          { value: "all", label: "전체" },
          { value: "active", label: "진행 중" },
          { value: "done", label: "완료" },
        ]}
      />
    );
  },
};
