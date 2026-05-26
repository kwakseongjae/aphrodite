import type { Meta, StoryObj } from "@storybook/react";
import { Toolbar } from "./Toolbar";
import { Button } from "./Button";

const meta: Meta<typeof Toolbar> = { component: Toolbar };
export default meta;
type Story = StoryObj<typeof Toolbar>;

export const Default: Story = {
  render: () => (
    <Toolbar ariaLabel="문서 편집">
      <Button variant="ghost" size="sm">굵게</Button>
      <Button variant="ghost" size="sm">기울임</Button>
      <Toolbar.Separator />
      <Button variant="ghost" size="sm">왼쪽 정렬</Button>
      <Button variant="ghost" size="sm">가운데</Button>
    </Toolbar>
  ),
};
