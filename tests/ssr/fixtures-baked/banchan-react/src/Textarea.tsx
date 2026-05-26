"use client";

import { forwardRef, TextareaHTMLAttributes } from "react";
import { cn } from "./cn";

export interface TextareaProps extends TextareaHTMLAttributes<HTMLTextAreaElement> {
  error?: boolean;
}

export const Textarea = forwardRef<HTMLTextAreaElement, TextareaProps>(function Textarea(
  { error, className, ...rest },
  ref,
) {
  return (
    <textarea
      ref={ref}
      className={cn("aph-textarea", error && "aph-input--error", className)}
      aria-invalid={error || undefined}
      {...rest}
    />
  );
});
