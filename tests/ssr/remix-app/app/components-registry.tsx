"use client";

/** Registry of every component in @aphrodite-design/example-banchan with
 *  a minimal demo wrapper. Per-component page renders the `demo` here.
 *  Used by Playwright spec to iterate all 70 components in one matrix. */
import { useState } from "react";
import * as Aph from "@aphrodite-design/example-banchan";

type Entry = { demo: () => JSX.Element };

const COMPONENTS: Record<string, Entry> = {
  // --- text + form primitives ---
  Button: { demo: () => <Aph.Button variant="primary">시작하기</Aph.Button> },
  Input: { demo: () => <Aph.Input placeholder="이메일" /> },
  Textarea: { demo: () => <Aph.Textarea placeholder="내용" /> },
  Select: { demo: () => <Aph.Select><option>대한민국</option></Aph.Select> },
  Checkbox: { demo: () => <Aph.Checkbox label="동의합니다" /> },
  Radio: { demo: () => <Aph.Radio label="개인" name="x" value="p" /> },
  RadioGroup: {
    demo: () => {
      const [v, setV] = useState("a");
      return <Aph.RadioGroup name="g" value={v} onChange={setV}><Aph.Radio value="a" label="A" /></Aph.RadioGroup>;
    },
  },
  Switch: {
    demo: () => {
      const [on, setOn] = useState(false);
      return <Aph.Switch checked={on} onChange={setOn} label="알림" />;
    },
  },
  Slider: { demo: () => <Aph.Slider min={0} max={100} defaultValue={30} /> },
  NumberInput: {
    demo: () => {
      const [n, setN] = useState(1);
      return <Aph.NumberInput value={n} onChange={setN} min={0} max={99} />;
    },
  },
  SearchInput: {
    demo: () => {
      const [q, setQ] = useState("");
      return <Aph.SearchInput value={q} onChange={setQ} placeholder="검색" />;
    },
  },
  FileUploader: { demo: () => <Aph.FileUploader onFiles={() => {}} /> },
  DatePicker: { demo: () => <Aph.DatePicker defaultValue="2026-05-19" /> },
  Combobox: {
    demo: () => {
      const [v, setV] = useState("seoul");
      return <Aph.Combobox value={v} onChange={setV} options={[{ value: "seoul", label: "서울" }]} />;
    },
  },
  PinInput: {
    demo: () => {
      const [v, setV] = useState("");
      return <Aph.PinInput length={6} value={v} onChange={setV} />;
    },
  },
  SegmentedControl: {
    demo: () => {
      const [v, setV] = useState<"a" | "b">("a");
      return <Aph.SegmentedControl value={v} onChange={setV} options={[{ value: "a" as const, label: "A" }, { value: "b" as const, label: "B" }]} />;
    },
  },
  FormField: {
    demo: () => (
      <Aph.FormField label="이메일">
        <Aph.Input placeholder="name@example.com" />
      </Aph.FormField>
    ),
  },
  PhoneInputKR: {
    demo: () => {
      const [v, setV] = useState("");
      return <Aph.PhoneInputKR value={v} onChange={setV} />;
    },
  },
  AddressInputKR: {
    demo: () => {
      const [v, setV] = useState({ postcode: "", road: "", detail: "" });
      return <Aph.AddressInputKR value={v} onChange={setV} />;
    },
  },
  ColorPicker: {
    demo: () => {
      const [c, setC] = useState("#16a34a");
      return <Aph.ColorPicker value={c} onChange={setC} />;
    },
  },
  TimePicker: {
    demo: () => {
      const [t, setT] = useState("09:30");
      return <Aph.TimePicker value={t} onChange={setT} />;
    },
  },
  DateRangePicker: {
    demo: () => {
      const [r, setR] = useState({ start: "2026-05-01", end: "2026-05-31" });
      return <Aph.DateRangePicker value={r} onChange={setR} />;
    },
  },
  MultiCombobox: {
    demo: () => {
      const [v, setV] = useState<string[]>([]);
      return <Aph.MultiCombobox value={v} onChange={setV} options={[{ value: "a", label: "A" }]} />;
    },
  },

  // --- display primitives ---
  Tag: { demo: () => <Aph.Tag tone="success">완료</Aph.Tag> },
  Badge: { demo: () => <Aph.Badge count={3} /> },
  Avatar: { demo: () => <Aph.Avatar initials="JK" alt="JK" /> },
  Chip: { demo: () => <Aph.Chip>서울</Aph.Chip> },
  Card: { demo: () => <Aph.Card>카드</Aph.Card> },
  Skeleton: { demo: () => <Aph.Skeleton width={120} height={16} /> },
  Spinner: { demo: () => <Aph.Spinner /> },
  ProgressBar: { demo: () => <Aph.ProgressBar value={60} max={100} /> },
  Stepper: { demo: () => <Aph.Stepper current={1} steps={[{ label: "1" }, { label: "2" }, { label: "3" }]} /> },
  Stat: { demo: () => <Aph.Stat label="매출" value="₩ 1,000" /> },
  Divider: { demo: () => <Aph.Divider /> },
  EmptyState: { demo: () => <Aph.EmptyState title="비어있음" /> },
  Kbd: { demo: () => <Aph.Kbd>K</Aph.Kbd> },
  KeyboardShortcut: { demo: () => <Aph.KeyboardShortcut keys={["⌘", "K"]} label="검색" /> },
  Code: { demo: () => <Aph.Code>npm i</Aph.Code> },
  CodeBlock: { demo: () => <Aph.CodeBlock>const x = 1;</Aph.CodeBlock> },
  Quote: { demo: () => <Aph.Quote cite="김민주">신선한 재료.</Aph.Quote> },
  Hint: { demo: () => <Aph.Hint>도움말</Aph.Hint> },
  Banner: { demo: () => <Aph.Banner tone="info">공지</Aph.Banner> },
  DescriptionList: { demo: () => <Aph.DescriptionList items={[{ label: "원산지", value: "한국" }]} /> },
  Image: { demo: () => <Aph.Image src="" alt="x" width={120} height={120} /> },

  // --- nav / disclosure ---
  Tabs: {
    demo: () => (
      <Aph.Tabs defaultValue="a">
        <Aph.TabList>
          <Aph.Tab value="a">A</Aph.Tab>
        </Aph.TabList>
        <Aph.TabPanel value="a">contents</Aph.TabPanel>
      </Aph.Tabs>
    ),
  },
  Accordion: {
    demo: () => (
      <Aph.Accordion>
        <Aph.AccordionItem title="질문">답변</Aph.AccordionItem>
      </Aph.Accordion>
    ),
  },
  Disclosure: { demo: () => <Aph.Disclosure title="더보기">내용</Aph.Disclosure> },
  Breadcrumb: { demo: () => <Aph.Breadcrumb items={[{ label: "홈" }]} /> },
  Pagination: {
    demo: () => {
      const [p, setP] = useState(1);
      return <Aph.Pagination page={p} pageCount={5} onChange={setP} />;
    },
  },
  Toolbar: { demo: () => <Aph.Toolbar><Aph.Button variant="ghost" size="sm">B</Aph.Button></Aph.Toolbar> },
  Menu: { demo: () => <Aph.Menu><Aph.MenuItem>항목</Aph.MenuItem></Aph.Menu> },
  TreeView: { demo: () => <Aph.TreeView nodes={[{ id: "1", label: "노드" }]} /> },
  Toggle: {
    demo: () => {
      const [on, setOn] = useState(false);
      return <Aph.Toggle pressed={on} onPressedChange={setOn}>B</Aph.Toggle>;
    },
  },

  // --- overlay ---
  // Open=true so SSR actually exercises the surface; closed overlays
  // return null and provide nothing to assert against.
  Modal: { demo: () => <Aph.Modal open={true} onClose={() => {}}>모달 본문</Aph.Modal> },
  Drawer: { demo: () => <Aph.Drawer open={true} onClose={() => {}}>드로어 본문</Aph.Drawer> },
  Sheet: { demo: () => <Aph.Sheet open={true} onClose={() => {}}>시트 본문</Aph.Sheet> },
  Tooltip: { demo: () => <Aph.Tooltip content="툴팁"><Aph.Button>호버</Aph.Button></Aph.Tooltip> },
  Popover: { demo: () => <Aph.Popover trigger={<Aph.Button>열기</Aph.Button>}>내용</Aph.Popover> },
  HoverCard: { demo: () => <Aph.HoverCard trigger={<Aph.Avatar initials="J" />}>profile</Aph.HoverCard> },
  ContextMenu: { demo: () => <Aph.ContextMenu trigger={<span>우클릭</span>} items={[{ label: "복사", onSelect: () => {} }]} /> },
  Command: { demo: () => <Aph.Command items={[{ id: "n", label: "새 문서", onSelect: () => {} }]} /> },
  // ToastProvider with a child Toast inline so the toast region renders something
  Toast: {
    demo: () => (
      <Aph.ToastProvider>
        <Aph.Toast id={1} message="저장되었습니다" tone="success" />
      </Aph.ToastProvider>
    ),
  },

  // --- data / motion ---
  Calendar: {
    demo: () => {
      const [d, setD] = useState(new Date("2026-05-19"));
      return <Aph.Calendar value={d} onChange={setD} />;
    },
  },
  Carousel: {
    demo: () => (
      <Aph.Carousel><div>slide 1</div></Aph.Carousel>
    ),
  },
  DataTable: {
    demo: () => (
      <Aph.DataTable
        rows={[{ a: "x" }] as Array<{ a: string }>}
        columns={[{ key: "a", header: "A" }]}
      />
    ),
  },
  ChartLine: { demo: () => <Aph.ChartLine points={[{ x: 1, y: 10 }, { x: 2, y: 20 }]} /> },
  ChartBar: { demo: () => <Aph.ChartBar bars={[{ label: "월", value: 24 }]} /> },
  Resizable: { demo: () => <div style={{ height: 120 }}><Aph.Resizable left={<div>L</div>} right={<div>R</div>} /></div> },
};

export default COMPONENTS;
