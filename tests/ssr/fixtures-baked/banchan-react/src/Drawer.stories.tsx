import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { Drawer } from "./Drawer";
import { Button } from "./Button";

const meta: Meta<typeof Drawer> = { component: Drawer };
export default meta;
type Story = StoryObj<typeof Drawer>;

export const Default: Story = {
  render: () => {
    const [open, setOpen] = useState(false);
    return (
      <>
        <Button onClick={() => setOpen(true)}>드로어 열기</Button>
        <Drawer open={open} onClose={() => setOpen(false)}>
          <h2>설정</h2>
          <p>드로어 안의 컨텐츠입니다.</p>
          <Button variant="secondary" onClick={() => setOpen(false)}>닫기</Button>
        </Drawer>
      </>
    );
  },
};
