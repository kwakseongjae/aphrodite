"use client";

import { ReactNode } from "react";
import { cn } from "./cn";

export interface ChipProps {
  children: ReactNode;
  onClose?: () => void;
  className?: string;
}

export function Chip({ children, onClose, className }: ChipProps) {
  return (
    <span className={cn("aph-chip", className)}>
      {children}
      {onClose && (
        <button type="button" className="aph-chip__close" aria-label="제거" onClick={onClose}>×</button>
      )}
    </span>
  );
}
