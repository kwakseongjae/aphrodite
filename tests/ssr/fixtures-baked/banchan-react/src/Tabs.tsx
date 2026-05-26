"use client";

import { createContext, useContext, useState, useId, ReactNode } from "react";
import { cn } from "./cn";

type TabsCtx = { value: string; setValue: (v: string) => void; baseId: string };
const Ctx = createContext<TabsCtx | null>(null);

export interface TabsProps {
  defaultValue: string;
  value?: string;
  onValueChange?: (v: string) => void;
  children: ReactNode;
  className?: string;
}

export function Tabs({ defaultValue, value, onValueChange, children, className }: TabsProps) {
  const [internal, setInternal] = useState(defaultValue);
  const current = value ?? internal;
  const setCurrent = (v: string) => {
    if (value === undefined) setInternal(v);
    onValueChange?.(v);
  };
  const baseId = useId();
  return (
    <div className={cn("aph-tabs", className)}>
      <Ctx.Provider value={{ value: current, setValue: setCurrent, baseId }}>{children}</Ctx.Provider>
    </div>
  );
}

export function TabList({ children }: { children: ReactNode }) {
  return <div role="tablist" className="aph-tab-list">{children}</div>;
}

export function Tab({ value, children }: { value: string; children: ReactNode }) {
  const ctx = useContext(Ctx);
  if (!ctx) throw new Error("Tab must be inside <Tabs>");
  const selected = ctx.value === value;
  return (
    <button
      type="button"
      role="tab"
      id={`${ctx.baseId}-tab-${value}`}
      aria-controls={`${ctx.baseId}-panel-${value}`}
      aria-selected={selected}
      tabIndex={selected ? 0 : -1}
      onClick={() => ctx.setValue(value)}
      className="aph-tab"
    >
      {children}
    </button>
  );
}

export function TabPanel({ value, children }: { value: string; children: ReactNode }) {
  const ctx = useContext(Ctx);
  if (!ctx) throw new Error("TabPanel must be inside <Tabs>");
  if (ctx.value !== value) return null;
  return (
    <div role="tabpanel" id={`${ctx.baseId}-panel-${value}`} aria-labelledby={`${ctx.baseId}-tab-${value}`} className="aph-tab-panel">
      {children}
    </div>
  );
}
