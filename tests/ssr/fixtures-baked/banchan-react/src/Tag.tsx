"use client";

import { forwardRef, HTMLAttributes, ReactNode } from "react";
import { cn } from "./cn";

export type TagTone = "neutral" | "success" | "warning" | "danger";

export interface TagProps extends HTMLAttributes<HTMLSpanElement> {
  tone?: TagTone;
  children: ReactNode;
}

export const Tag = forwardRef<HTMLSpanElement, TagProps>(function Tag(
  { tone = "neutral", className, children, ...rest },
  ref,
) {
  return (
    <span
      ref={ref}
      className={cn("aph-tag", tone !== "neutral" && `aph-tag--${tone}`, className)}
      {...rest}
    >
      {children}
    </span>
  );
});
