import type { Meta, StoryObj } from "@storybook/react";
import { Popover } from "./Popover";
import { Button } from "./Button";

const meta: Meta<typeof Popover> = { component: Popover };
export default meta;
type Story = StoryObj<typeof Popover>;

export const Default: Story = {
  render: () => (
    <Popover trigger={<Button variant="secondary">필터</Button>}>
      <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
        <label><input type="checkbox" /> 진행 중</label>
        <label><input type="checkbox" /> 완료</label>
        <label><input type="checkbox" /> 취소</label>
      </div>
    </Popover>
  ),
};
