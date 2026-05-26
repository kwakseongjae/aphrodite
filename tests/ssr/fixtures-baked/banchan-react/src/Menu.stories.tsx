import type { Meta, StoryObj } from "@storybook/react";
import { Menu, MenuItem } from "./Menu";

const meta: Meta<typeof Menu> = { component: Menu };
export default meta;
type Story = StoryObj<typeof Menu>;

export const Default: Story = {
  render: () => (
    <Menu ariaLabel="사용자 메뉴">
      <MenuItem>프로필</MenuItem>
      <MenuItem>설정</MenuItem>
      <MenuItem>도움말</MenuItem>
      <MenuItem>로그아웃</MenuItem>
    </Menu>
  ),
};
