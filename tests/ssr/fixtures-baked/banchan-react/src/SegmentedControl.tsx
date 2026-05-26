"use client";

import { ReactNode } from "react";
import { cn } from "./cn";

export interface SegmentedControlProps<T extends string> {
  options: Array<{ value: T; label: ReactNode }>;
  value: T;
  onChange: (next: T) => void;
  className?: string;
  ariaLabel?: string;
}

export function SegmentedControl<T extends string>({ options, value, onChange, className, ariaLabel }: SegmentedControlProps<T>) {
  return (
    <div role="radiogroup" aria-label={ariaLabel} className={cn("aph-segmented", className)}>
      {options.map((o) => (
        <button
          key={o.value}
          type="button"
          role="radio"
          aria-pressed={value === o.value}
          aria-checked={value === o.value}
          onClick={() => onChange(o.value)}
          className="aph-segmented__btn"
        >
          {o.label}
        </button>
      ))}
    </div>
  );
}
