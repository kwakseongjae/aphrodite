import type { Meta, StoryObj } from "@storybook/react";
import { Image } from "./Image";

const meta: Meta<typeof Image> = { component: Image };
export default meta;
type Story = StoryObj<typeof Image>;

export const Fallback: Story = { args: { width: 200, height: 200, src: "nope.jpg", alt: "상품 사진", rounded: 12 } };
