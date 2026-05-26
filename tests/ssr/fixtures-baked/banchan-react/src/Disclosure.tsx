"use client";

import { useState, ReactNode, useId } from "react";
import { cn } from "./cn";

export interface DisclosureProps {
  title: ReactNode;
  children: ReactNode;
  defaultOpen?: boolean;
  className?: string;
}

export function Disclosure({ title, children, defaultOpen = false, className }: DisclosureProps) {
  const [open, setOpen] = useState(defaultOpen);
  const id = useId();
  return (
    <div className={cn("aph-disclosure", className)}>
      <button type="button" aria-expanded={open} aria-controls={`${id}-body`} className="aph-disclosure__trigger" onClick={() => setOpen((o) => !o)}>
        <span>{title}</span>
        <span aria-hidden="true">{open ? "−" : "+"}</span>
      </button>
      {open && <div id={`${id}-body`} className="aph-disclosure__body">{children}</div>}
    </div>
  );
}
