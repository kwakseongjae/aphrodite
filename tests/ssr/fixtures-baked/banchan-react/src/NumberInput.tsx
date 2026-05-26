"use client";

import { useId } from "react";
import { cn } from "./cn";

export interface NumberInputProps {
  value: number;
  onChange: (next: number) => void;
  min?: number;
  max?: number;
  step?: number;
  disabled?: boolean;
  className?: string;
  ariaLabel?: string;
}

export function NumberInput({ value, onChange, min, max, step = 1, disabled, className, ariaLabel }: NumberInputProps) {
  const id = useId();
  const clamp = (n: number) => {
    if (typeof min === "number" && n < min) return min;
    if (typeof max === "number" && n > max) return max;
    return n;
  };
  return (
    <div className={cn("aph-number-input", className)}>
      <button type="button" className="aph-number-input__btn" aria-label="감소" disabled={disabled || (typeof min === "number" && value <= min)} onClick={() => onChange(clamp(value - step))}>−</button>
      <input
        id={id}
        type="number"
        className="aph-number-input__field"
        value={value}
        onChange={(e) => onChange(clamp(Number(e.target.value)))}
        disabled={disabled}
        aria-label={ariaLabel}
      />
      <button type="button" className="aph-number-input__btn" aria-label="증가" disabled={disabled || (typeof max === "number" && value >= max)} onClick={() => onChange(clamp(value + step))}>+</button>
    </div>
  );
}
