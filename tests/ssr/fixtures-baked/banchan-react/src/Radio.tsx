"use client";

import { forwardRef, InputHTMLAttributes, ReactNode } from "react";
import { cn } from "./cn";

export interface RadioProps extends Omit<InputHTMLAttributes<HTMLInputElement>, "type"> {
  label?: ReactNode;
}

export const Radio = forwardRef<HTMLInputElement, RadioProps>(function Radio(
  { label, className, disabled, ...rest },
  ref,
) {
  const input = <input ref={ref} type="radio" disabled={disabled} {...rest} />;
  if (!label) return input;
  return (
    <label className={cn("aph-radio", className)}>
      {input}
      <span>{label}</span>
    </label>
  );
});
