import type { Meta, StoryObj } from "@storybook/react";
import { DataTable } from "./DataTable";

type Tx = { date: string; merchant: string; amount: number; status: string };
const rows: Tx[] = [
  { date: "2026-05-18", merchant: "스타벅스 강남점", amount: -6500, status: "완료" },
  { date: "2026-05-17", merchant: "월급", amount: 3200000, status: "완료" },
  { date: "2026-05-16", merchant: "GS25", amount: -3200, status: "완료" },
  { date: "2026-05-15", merchant: "쿠팡", amount: -42000, status: "대기" },
];

const meta: Meta<typeof DataTable<Tx>> = { component: DataTable };
export default meta;
type Story = StoryObj<typeof DataTable<Tx>>;

export const Transactions: Story = {
  args: {
    rows,
    columns: [
      { key: "date", header: "날짜", sortable: true },
      { key: "merchant", header: "내역", sortable: true },
      { key: "amount", header: "금액", sortable: true, align: "right", cell: (r) => (r.amount > 0 ? `+${r.amount.toLocaleString()}` : `${r.amount.toLocaleString()}`) + "원" },
      { key: "status", header: "상태" },
    ],
  },
};
