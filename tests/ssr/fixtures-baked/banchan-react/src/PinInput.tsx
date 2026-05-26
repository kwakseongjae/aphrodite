"use client";

import { useRef, ChangeEvent, KeyboardEvent } from "react";
import { cn } from "./cn";

export interface PinInputProps {
  length?: number;
  value: string;
  onChange: (next: string) => void;
  type?: "number" | "text";
  className?: string;
  ariaLabel?: string;
}

export function PinInput({ length = 6, value, onChange, type = "number", className, ariaLabel = "OTP" }: PinInputProps) {
  const refs = useRef<Array<HTMLInputElement | null>>([]);
  const digits = Array.from({ length }, (_, i) => value[i] ?? "");
  const set = (i: number, v: string) => {
    const next = (value.slice(0, i) + v + value.slice(i + 1)).slice(0, length);
    onChange(next);
  };
  const onInput = (i: number) => (e: ChangeEvent<HTMLInputElement>) => {
    const v = (type === "number" ? e.target.value.replace(/\D/g, "") : e.target.value).slice(-1);
    set(i, v);
    if (v && i < length - 1) refs.current[i + 1]?.focus();
  };
  const onKey = (i: number) => (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Backspace" && !digits[i] && i > 0) refs.current[i - 1]?.focus();
    if (e.key === "ArrowLeft" && i > 0) refs.current[i - 1]?.focus();
    if (e.key === "ArrowRight" && i < length - 1) refs.current[i + 1]?.focus();
  };
  return (
    <div role="group" aria-label={ariaLabel} className={cn("aph-pin", className)}>
      {digits.map((d, i) => (
        <input
          key={i}
          ref={(el) => { refs.current[i] = el; }}
          inputMode={type === "number" ? "numeric" : "text"}
          maxLength={1}
          value={d}
          onChange={onInput(i)}
          onKeyDown={onKey(i)}
          className="aph-pin__digit"
          aria-label={`Digit ${i + 1}`}
        />
      ))}
    </div>
  );
}
