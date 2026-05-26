"use client";

import { ReactNode } from "react";
import { cn } from "./cn";

export interface StatProps {
  label: ReactNode;
  value: ReactNode;
  delta?: { value: ReactNode; direction: "up" | "down" };
  className?: string;
}

export function Stat({ label, value, delta, className }: StatProps) {
  return (
    <div className={cn("aph-stat", className)}>
      <span className="aph-stat__label">{label}</span>
      <span className="aph-stat__value">{value}</span>
      {delta && (
        <span className={cn("aph-stat__delta", `aph-stat__delta--${delta.direction}`)}>
          {delta.direction === "up" ? "▲" : "▼"} {delta.value}
        </span>
      )}
    </div>
  );
}
