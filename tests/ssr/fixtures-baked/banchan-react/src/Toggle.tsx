"use client";

import { forwardRef, ButtonHTMLAttributes, ReactNode } from "react";
import { cn } from "./cn";

export interface ToggleProps extends Omit<ButtonHTMLAttributes<HTMLButtonElement>, "onChange"> {
  pressed: boolean;
  onPressedChange: (next: boolean) => void;
  children: ReactNode;
}

export const Toggle = forwardRef<HTMLButtonElement, ToggleProps>(function Toggle(
  { pressed, onPressedChange, className, children, ...rest },
  ref,
) {
  return (
    <button
      ref={ref}
      type="button"
      aria-pressed={pressed}
      onClick={() => onPressedChange(!pressed)}
      className={cn("aph-toggle", className)}
      {...rest}
    >
      {children}
    </button>
  );
});
