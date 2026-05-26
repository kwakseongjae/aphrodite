"use client";

import { ReactNode, ReactElement } from "react";
import { cn } from "./cn";

export interface HoverCardProps {
  trigger: ReactElement;
  children: ReactNode;
  className?: string;
}

export function HoverCard({ trigger, children, className }: HoverCardProps) {
  return (
    <span className="aph-hover-wrap" tabIndex={0}>
      {trigger}
      <span role="tooltip" className={cn("aph-hover-card", className)}>{children}</span>
    </span>
  );
}
