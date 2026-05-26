"use client";

import { useState, ReactNode, useId } from "react";
import { cn } from "./cn";

export interface AccordionProps {
  children: ReactNode;
  className?: string;
  /** Allow multiple items open at once. */
  multiple?: boolean;
}

export interface AccordionItemProps {
  title: ReactNode;
  children: ReactNode;
  defaultOpen?: boolean;
}

export function Accordion({ children, className }: AccordionProps) {
  return <div className={cn("aph-accordion", className)}>{children}</div>;
}

export function AccordionItem({ title, children, defaultOpen = false }: AccordionItemProps) {
  const [open, setOpen] = useState(defaultOpen);
  const id = useId();
  return (
    <div className="aph-accordion-item">
      <button
        type="button"
        className="aph-accordion-trigger"
        aria-expanded={open}
        aria-controls={`${id}-content`}
        id={`${id}-trigger`}
        onClick={() => setOpen((o) => !o)}
      >
        <span>{title}</span>
        <span className="aph-accordion-chevron" aria-hidden="true">▾</span>
      </button>
      {open && (
        <div role="region" id={`${id}-content`} aria-labelledby={`${id}-trigger`} className="aph-accordion-content">
          {children}
        </div>
      )}
    </div>
  );
}
