"use client";

import { useState, useRef, useEffect, ReactNode } from "react";
import { cn } from "./cn";

export interface MultiComboboxOption {
  value: string;
  label: ReactNode;
  search?: string;
}

export interface MultiComboboxProps {
  options: MultiComboboxOption[];
  value: string[];
  onChange: (next: string[]) => void;
  placeholder?: string;
  className?: string;
  ariaLabel?: string;
}

export function MultiCombobox({ options, value, onChange, placeholder, className, ariaLabel }: MultiComboboxProps) {
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const wrapRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!open) return;
    const onDoc = (e: MouseEvent) => {
      if (wrapRef.current && !wrapRef.current.contains(e.target as Node)) setOpen(false);
    };
    document.addEventListener("mousedown", onDoc);
    return () => document.removeEventListener("mousedown", onDoc);
  }, [open]);
  const filtered = options.filter((o) => {
    if (value.includes(o.value)) return false;
    if (!query) return true;
    return (o.search ?? o.value).toLowerCase().includes(query.toLowerCase());
  });
  const remove = (v: string) => onChange(value.filter((x) => x !== v));
  const add = (v: string) => { onChange([...value, v]); setQuery(""); };
  return (
    <div ref={wrapRef} className={cn("aph-multicombo", className)}>
      <div className="aph-multicombo__field" role="combobox" aria-expanded={open} aria-label={ariaLabel} onClick={() => setOpen(true)}>
        {value.map((v) => {
          const opt = options.find((o) => o.value === v);
          return (
            <span key={v} className="aph-multicombo__chip">
              {opt?.label ?? v}
              <button type="button" className="aph-multicombo__chip-x" aria-label={`${opt?.label ?? v} 제거`} onClick={(e) => { e.stopPropagation(); remove(v); }}>×</button>
            </span>
          );
        })}
        <input
          className="aph-multicombo__input"
          value={query}
          onChange={(e) => { setQuery(e.target.value); setOpen(true); }}
          onFocus={() => setOpen(true)}
          placeholder={value.length === 0 ? placeholder : ""}
        />
      </div>
      {open && filtered.length > 0 && (
        <ul role="listbox" className="aph-multicombo__list">
          {filtered.map((o) => (
            <li
              key={o.value}
              role="option"
              aria-selected={false}
              className="aph-combobox__option"
              onMouseDown={(e) => { e.preventDefault(); add(o.value); }}
            >
              {o.label}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
