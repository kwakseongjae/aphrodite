"use client";

import { useState, useMemo, ReactNode, KeyboardEvent } from "react";
import { cn } from "./cn";

export interface CommandItem {
  id: string;
  label: ReactNode;
  /** Lower-cased search-key. Defaults to id. */
  search?: string;
  hint?: ReactNode;
  onSelect: () => void;
}

export interface CommandProps {
  items: CommandItem[];
  placeholder?: string;
  emptyMessage?: ReactNode;
  className?: string;
  ariaLabel?: string;
}

export function Command({ items, placeholder = "Search...", emptyMessage = "결과가 없습니다", className, ariaLabel = "Command" }: CommandProps) {
  const [q, setQ] = useState("");
  const [active, setActive] = useState(0);
  const filtered = useMemo(() => {
    if (!q) return items;
    const needle = q.toLowerCase();
    return items.filter((it) => (it.search ?? it.id).toLowerCase().includes(needle));
  }, [items, q]);
  const onKey = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "ArrowDown") { e.preventDefault(); setActive((a) => Math.min(filtered.length - 1, a + 1)); }
    if (e.key === "ArrowUp") { e.preventDefault(); setActive((a) => Math.max(0, a - 1)); }
    if (e.key === "Enter") { e.preventDefault(); filtered[active]?.onSelect(); }
  };
  return (
    <div role="combobox" aria-label={ariaLabel} aria-expanded="true" className={cn("aph-command-root", className)}>
      <input
        type="text"
        autoFocus
        value={q}
        onChange={(e) => { setQ(e.target.value); setActive(0); }}
        onKeyDown={onKey}
        placeholder={placeholder}
        className="aph-command-input"
        aria-label={placeholder}
      />
      <div className="aph-command-list" role="listbox">
        {filtered.length === 0 ? (
          <div className="aph-command-empty">{emptyMessage}</div>
        ) : (
          filtered.map((it, i) => (
            <div
              key={it.id}
              role="option"
              aria-selected={i === active}
              onMouseEnter={() => setActive(i)}
              onClick={() => it.onSelect()}
              className="aph-command-row"
            >
              <span style={{ flex: 1 }}>{it.label}</span>
              {it.hint && <span style={{ fontSize: 12, color: "var(--colors-text-muted)" }}>{it.hint}</span>}
            </div>
          ))
        )}
      </div>
    </div>
  );
}
