"use client";

import { ReactNode } from "react";
import { cn } from "./cn";

export interface HintProps {
  tone?: "neutral" | "warning" | "danger";
  icon?: ReactNode;
  children: ReactNode;
  className?: string;
}

export function Hint({ tone = "neutral", icon, children, className }: HintProps) {
  return (
    <span className={cn("aph-hint", tone !== "neutral" && `aph-hint--${tone}`, className)}>
      {icon && <span aria-hidden="true">{icon}</span>}
      <span>{children}</span>
    </span>
  );
}
