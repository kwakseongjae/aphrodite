"use client";

import { useEffect, ReactNode } from "react";
import { cn } from "./cn";

export interface DrawerProps {
  open: boolean;
  onClose: () => void;
  children: ReactNode;
  className?: string;
}

export function Drawer({ open, onClose, children, className }: DrawerProps) {
  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  }, [open, onClose]);
  if (!open) return null;
  return (
    <>
      <div className="aph-drawer-backdrop" onClick={onClose} aria-hidden="true" />
      <aside role="dialog" aria-modal="true" className={cn("aph-drawer", className)}>
        {children}
      </aside>
    </>
  );
}
