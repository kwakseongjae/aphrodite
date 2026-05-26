import type { Meta, StoryObj } from "@storybook/react";
import { Slider } from "./Slider";

const meta: Meta<typeof Slider> = { component: Slider, args: { min: 0, max: 100, defaultValue: 30 } };
export default meta;
type Story = StoryObj<typeof Slider>;

export const Default: Story = {};
