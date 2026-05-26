import type { Meta, StoryObj } from "@storybook/react";
import { Avatar } from "./Avatar";

const meta: Meta<typeof Avatar> = { component: Avatar };
export default meta;
type Story = StoryObj<typeof Avatar>;

export const Initials: Story = { args: { initials: "JK", alt: "Jihyo Kim" } };
export const Small: Story = { args: { size: "sm", initials: "MJ" } };
export const Large: Story = { args: { size: "lg", initials: "SA" } };
