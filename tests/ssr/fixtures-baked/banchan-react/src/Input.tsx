"use client";

import { forwardRef, InputHTMLAttributes } from "react";
import { cn } from "./cn";

export interface InputProps extends InputHTMLAttributes<HTMLInputElement> {
  error?: boolean;
}

export const Input = forwardRef<HTMLInputElement, InputProps>(function Input(
  { error, className, ...rest },
  ref,
) {
  return (
    <input
      ref={ref}
      className={cn("aph-input", error && "aph-input--error", className)}
      aria-invalid={error || undefined}
      {...rest}
    />
  );
});
