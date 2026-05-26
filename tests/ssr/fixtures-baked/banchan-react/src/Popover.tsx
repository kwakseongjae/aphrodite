"use client";

import { useState, useRef, useEffect, ReactNode, ReactElement, cloneElement } from "react";
import { cn } from "./cn";

export interface PopoverProps {
  trigger: ReactElement;
  children: ReactNode;
  className?: string;
}

export function Popover({ trigger, children, className }: PopoverProps) {
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLSpanElement>(null);
  useEffect(() => {
    if (!open) return;
    const onDoc = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) setOpen(false);
    };
    document.addEventListener("mousedown", onDoc);
    return () => document.removeEventListener("mousedown", onDoc);
  }, [open]);
  return (
    <span ref={ref} className="aph-popover-wrap">
      {cloneElement(trigger, { onClick: () => setOpen((o) => !o), "aria-expanded": open })}
      {open && <div className={cn("aph-popover", className)} role="dialog">{children}</div>}
    </span>
  );
}
