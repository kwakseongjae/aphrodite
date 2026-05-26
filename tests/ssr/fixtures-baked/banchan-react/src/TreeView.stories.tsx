import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { TreeView } from "./TreeView";

const meta: Meta<typeof TreeView> = { component: TreeView };
export default meta;
type Story = StoryObj<typeof TreeView>;

export const Categories: Story = {
  render: () => {
    const [sel, setSel] = useState<string>("rice");
    return (
      <TreeView
        selectedId={sel}
        onSelect={setSel}
        defaultExpanded={["main", "side"]}
        nodes={[
          {
            id: "main", label: "주요리",
            children: [
              { id: "rice", label: "밥류" },
              { id: "soup", label: "국·찌개" },
              { id: "noodle", label: "면류" },
            ],
          },
          {
            id: "side", label: "반찬",
            children: [
              { id: "kimchi", label: "김치류" },
              { id: "namul", label: "나물류" },
              { id: "jorim", label: "조림류" },
            ],
          },
          { id: "dessert", label: "디저트" },
        ]}
      />
    );
  },
};
