import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { Sheet } from "./Sheet";
import { Button } from "./Button";

const meta: Meta<typeof Sheet> = { component: Sheet };
export default meta;
type Story = StoryObj<typeof Sheet>;

export const Default: Story = {
  render: () => {
    const [open, setOpen] = useState(false);
    return (
      <>
        <Button onClick={() => setOpen(true)}>바텀시트 열기</Button>
        <Sheet open={open} onClose={() => setOpen(false)} title="계좌 선택">
          <p>출금하실 계좌를 선택해주세요.</p>
          <Button onClick={() => setOpen(false)}>완료</Button>
        </Sheet>
      </>
    );
  },
};
