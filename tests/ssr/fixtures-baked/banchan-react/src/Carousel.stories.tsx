import type { Meta, StoryObj } from "@storybook/react";
import { Carousel } from "./Carousel";

const meta: Meta<typeof Carousel> = { component: Carousel };
export default meta;
type Story = StoryObj<typeof Carousel>;

export const Hero: Story = {
  render: () => (
    <div style={{ width: 600, height: 280 }}>
      <Carousel ariaLabel="홈 배너">
        {["#16a34a", "#3b82f6", "#dc2626"].map((bg, i) => (
          <div key={i} style={{ background: bg, height: 280, display: "flex", alignItems: "center", justifyContent: "center", color: "#fff", fontSize: 32, fontWeight: 700 }}>슬라이드 {i + 1}</div>
        ))}
      </Carousel>
    </div>
  ),
};
