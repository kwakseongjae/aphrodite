import type { Meta, StoryObj } from "@storybook/react";
import { HoverCard } from "./HoverCard";
import { Avatar } from "./Avatar";

const meta: Meta<typeof HoverCard> = { component: HoverCard };
export default meta;
type Story = StoryObj<typeof HoverCard>;

export const UserCard: Story = {
  render: () => (
    <HoverCard trigger={<Avatar initials="JK" alt="Jihyo Kim" />}>
      <div style={{ display: "flex", flexDirection: "column", gap: 4 }}>
        <strong>김지효</strong>
        <small>jihyo@toss.im</small>
        <small>마지막 활동: 5분 전</small>
      </div>
    </HoverCard>
  ),
};
