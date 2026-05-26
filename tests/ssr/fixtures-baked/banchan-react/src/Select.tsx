"use client";

import { forwardRef, SelectHTMLAttributes, ReactNode } from "react";
import { cn } from "./cn";

export interface SelectProps extends SelectHTMLAttributes<HTMLSelectElement> {
  error?: boolean;
  children: ReactNode;
}

export const Select = forwardRef<HTMLSelectElement, SelectProps>(function Select(
  { error, className, children, ...rest },
  ref,
) {
  return (
    <select
      ref={ref}
      className={cn("aph-select", error && "aph-input--error", className)}
      aria-invalid={error || undefined}
      {...rest}
    >
      {children}
    </select>
  );
});
