"use client";

import { cn } from "./cn";

export interface DividerProps {
  orientation?: "horizontal" | "vertical";
  className?: string;
}

export function Divider({ orientation = "horizontal", className }: DividerProps) {
  return <hr role="separator" aria-orientation={orientation} className={cn("aph-divider", orientation === "vertical" && "aph-divider--vertical", className)} />;
}
