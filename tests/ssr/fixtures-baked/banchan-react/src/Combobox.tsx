"use client";

import { useState, useRef, useEffect, ReactNode, useId } from "react";
import { cn } from "./cn";

export interface ComboboxOption {
  value: string;
  label: ReactNode;
  /** Lower-cased search-key. Defaults to the value. */
  search?: string;
}

export interface ComboboxProps {
  options: ComboboxOption[];
  value: string;
  onChange: (v: string) => void;
  placeholder?: string;
  className?: string;
  ariaLabel?: string;
}

export function Combobox({ options, value, onChange, placeholder, className, ariaLabel }: ComboboxProps) {
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const wrapRef = useRef<HTMLDivElement>(null);
  const id = useId();
  useEffect(() => {
    if (!open) return;
    const onDoc = (e: MouseEvent) => {
      if (wrapRef.current && !wrapRef.current.contains(e.target as Node)) setOpen(false);
    };
    document.addEventListener("mousedown", onDoc);
    return () => document.removeEventListener("mousedown", onDoc);
  }, [open]);
  const filtered = options.filter((o) => {
    if (!query) return true;
    const haystack = (o.search ?? o.value).toLowerCase();
    return haystack.includes(query.toLowerCase());
  });
  const current = options.find((o) => o.value === value);
  return (
    <div ref={wrapRef} className={cn("aph-combobox", className)}>
      <input
        type="text"
        className="aph-input"
        role="combobox"
        aria-expanded={open}
        aria-controls={`${id}-list`}
        aria-label={ariaLabel}
        placeholder={placeholder}
        value={open ? query : (typeof current?.label === "string" ? current.label : value)}
        onFocus={() => setOpen(true)}
        onChange={(e) => { setQuery(e.target.value); setOpen(true); }}
      />
      {open && filtered.length > 0 && (
        <ul role="listbox" id={`${id}-list`} className="aph-combobox__list">
          {filtered.map((o) => (
            <li
              key={o.value}
              role="option"
              aria-selected={o.value === value}
              className="aph-combobox__option"
              onMouseDown={(e) => { e.preventDefault(); onChange(o.value); setOpen(false); setQuery(""); }}
            >
              {o.label}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
