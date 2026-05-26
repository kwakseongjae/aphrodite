import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { Combobox } from "./Combobox";

const meta: Meta<typeof Combobox> = { component: Combobox };
export default meta;
type Story = StoryObj<typeof Combobox>;

export const Cities: Story = {
  render: () => {
    const [v, setV] = useState("seoul");
    return (
      <div style={{ width: 280 }}>
        <Combobox
          value={v}
          onChange={setV}
          ariaLabel="도시 선택"
          placeholder="도시 검색…"
          options={[
            { value: "seoul", label: "서울특별시" },
            { value: "busan", label: "부산광역시" },
            { value: "incheon", label: "인천광역시" },
            { value: "daegu", label: "대구광역시" },
            { value: "daejeon", label: "대전광역시" },
            { value: "gwangju", label: "광주광역시" },
            { value: "ulsan", label: "울산광역시" },
            { value: "jeju", label: "제주특별자치도" },
          ]}
        />
      </div>
    );
  },
};
