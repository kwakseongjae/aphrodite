import type { Meta, StoryObj } from "@storybook/react";
import { ToastProvider, useToast } from "./Toast";
import { Button } from "./Button";

const meta: Meta<typeof ToastProvider> = { component: ToastProvider };
export default meta;
type Story = StoryObj<typeof ToastProvider>;

function Demo() {
  const { push } = useToast();
  return (
    <div style={{ display: "flex", gap: 8 }}>
      <Button onClick={() => push({ message: "저장되었습니다" })}>기본</Button>
      <Button variant="secondary" onClick={() => push({ message: "전송 완료", tone: "success" })}>성공</Button>
      <Button variant="danger" onClick={() => push({ message: "오류가 발생했습니다", tone: "danger" })}>위험</Button>
    </div>
  );
}

export const Default: Story = {
  render: () => <ToastProvider><Demo /></ToastProvider>,
};
