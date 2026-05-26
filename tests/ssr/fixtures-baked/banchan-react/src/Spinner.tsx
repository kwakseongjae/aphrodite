"use client";

import { HTMLAttributes } from "react";
import { cn } from "./cn";

export type SpinnerSize = "sm" | "md" | "lg";

export interface SpinnerProps extends HTMLAttributes<HTMLSpanElement> {
  size?: SpinnerSize;
  label?: string;
}

export function Spinner({ size = "md", label = "Loading", className, ...rest }: SpinnerProps) {
  return (
    <span
      role="status"
      aria-label={label}
      className={cn("aph-spinner", size !== "md" && `aph-spinner--${size}`, className)}
      {...rest}
    />
  );
}
