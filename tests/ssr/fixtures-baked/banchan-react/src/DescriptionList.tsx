"use client";

import { ReactNode } from "react";
import { cn } from "./cn";

export interface DescriptionListProps {
  items: Array<{ label: ReactNode; value: ReactNode }>;
  className?: string;
}

export function DescriptionList({ items, className }: DescriptionListProps) {
  return (
    <dl className={cn("aph-desc-list", className)}>
      {items.map((it, i) => (
        <span key={i} style={{ display: "contents" }}>
          <dt>{it.label}</dt>
          <dd>{it.value}</dd>
        </span>
      ))}
    </dl>
  );
}
