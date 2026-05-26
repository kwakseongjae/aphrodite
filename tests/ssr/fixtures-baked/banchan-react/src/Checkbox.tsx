"use client";

import { forwardRef, InputHTMLAttributes, ReactNode } from "react";
import { cn } from "./cn";

export interface CheckboxProps extends Omit<InputHTMLAttributes<HTMLInputElement>, "type"> {
  label?: ReactNode;
}

export const Checkbox = forwardRef<HTMLInputElement, CheckboxProps>(function Checkbox(
  { label, className, disabled, ...rest },
  ref,
) {
  const input = <input ref={ref} type="checkbox" disabled={disabled} {...rest} />;
  if (!label) return input;
  return (
    <label className={cn("aph-checkbox", disabled && "aph-checkbox--disabled", className)}>
      {input}
      <span>{label}</span>
    </label>
  );
});
