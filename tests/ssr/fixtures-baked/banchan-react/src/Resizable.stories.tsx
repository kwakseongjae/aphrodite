import type { Meta, StoryObj } from "@storybook/react";
import { Resizable } from "./Resizable";

const meta: Meta<typeof Resizable> = { component: Resizable };
export default meta;
type Story = StoryObj<typeof Resizable>;

export const TwoPane: Story = {
  render: () => (
    <div style={{ width: 600, height: 240 }}>
      <Resizable
        left={<div><h4 style={{ margin: 0 }}>사이드바</h4><p>여기에 폴더 목록 등을 넣습니다.</p></div>}
        right={<div><h4 style={{ margin: 0 }}>본문</h4><p>핸들을 좌우로 끌어 너비 조절.</p></div>}
      />
    </div>
  ),
};
