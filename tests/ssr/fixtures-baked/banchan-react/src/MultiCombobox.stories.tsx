import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { MultiCombobox } from "./MultiCombobox";

const meta: Meta<typeof MultiCombobox> = { component: MultiCombobox };
export default meta;
type Story = StoryObj<typeof MultiCombobox>;

export const Tags: Story = {
  render: () => {
    const [v, setV] = useState<string[]>(["seoul"]);
    return (
      <div style={{ width: 320 }}>
        <MultiCombobox
          value={v}
          onChange={setV}
          ariaLabel="배송 가능 지역"
          placeholder="지역 추가…"
          options={[
            { value: "seoul", label: "서울" },
            { value: "busan", label: "부산" },
            { value: "incheon", label: "인천" },
            { value: "daegu", label: "대구" },
            { value: "daejeon", label: "대전" },
            { value: "gwangju", label: "광주" },
          ]}
        />
      </div>
    );
  },
};
