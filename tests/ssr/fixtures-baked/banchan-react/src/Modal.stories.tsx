import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { Modal } from "./Modal";
import { Button } from "./Button";

const meta: Meta<typeof Modal> = { component: Modal };
export default meta;
type Story = StoryObj<typeof Modal>;

export const Default: Story = {
  render: () => {
    const [open, setOpen] = useState(false);
    return (
      <>
        <Button onClick={() => setOpen(true)}>모달 열기</Button>
        <Modal open={open} onClose={() => setOpen(false)} title="확인">
          <p>저장하시겠습니까?</p>
          <Button onClick={() => setOpen(false)}>닫기</Button>
        </Modal>
      </>
    );
  },
};
