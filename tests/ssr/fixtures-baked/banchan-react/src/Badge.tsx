"use client";

import { forwardRef, HTMLAttributes, ReactNode } from "react";
import { cn } from "./cn";

export interface BadgeProps extends HTMLAttributes<HTMLSpanElement> {
  /** Dot variant ignores `count` and renders an 8x8 indicator. */
  dot?: boolean;
  count?: number;
  max?: number;
  children?: ReactNode;
}

export const Badge = forwardRef<HTMLSpanElement, BadgeProps>(function Badge(
  { dot, count, max = 99, className, children, ...rest },
  ref,
) {
  if (dot) {
    return <span ref={ref} className={cn("aph-badge", "aph-badge--dot", className)} aria-hidden="true" {...rest} />;
  }
  const label = typeof count === "number" ? (count > max ? `${max}+` : `${count}`) : children;
  return (
    <span ref={ref} className={cn("aph-badge", className)} {...rest}>{label}</span>
  );
});
