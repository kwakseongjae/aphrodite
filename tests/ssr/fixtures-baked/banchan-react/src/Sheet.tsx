"use client";

import { useEffect, ReactNode } from "react";
import { cn } from "./cn";

export interface SheetProps {
  open: boolean;
  onClose: () => void;
  title?: ReactNode;
  children: ReactNode;
  className?: string;
}

/** Mobile-first bottom sheet. Use Drawer for desktop right-side panels. */
export function Sheet({ open, onClose, title, children, className }: SheetProps) {
  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => { if (e.key === "Escape") onClose(); };
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  }, [open, onClose]);
  if (!open) return null;
  return (
    <>
      <div className="aph-sheet-backdrop" onClick={onClose} aria-hidden="true" />
      <div role="dialog" aria-modal="true" aria-label={typeof title === "string" ? title : undefined} className={cn("aph-sheet", className)}>
        <div className="aph-sheet__handle" aria-hidden="true" />
        {title && <h2 style={{ margin: "0 0 12px" }}>{title}</h2>}
        {children}
      </div>
    </>
  );
}
