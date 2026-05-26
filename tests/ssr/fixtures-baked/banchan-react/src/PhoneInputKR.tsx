"use client";

import { forwardRef, InputHTMLAttributes, ChangeEvent } from "react";
import { cn } from "./cn";

/** Strip every non-digit and cap at 11 digits (010 prefix + 8). */
export function parseKoreanPhone(raw: string): string {
  return raw.replace(/\D/g, "").slice(0, 11);
}

/** Format a digit-string to 010-XXXX-XXXX (or partial). */
export function formatKoreanPhone(digits: string): string {
  const d = parseKoreanPhone(digits);
  if (d.length < 4) return d;
  if (d.length < 8) return `${d.slice(0, 3)}-${d.slice(3)}`;
  return `${d.slice(0, 3)}-${d.slice(3, 7)}-${d.slice(7)}`;
}

export interface PhoneInputKRProps extends Omit<InputHTMLAttributes<HTMLInputElement>, "type" | "value" | "onChange"> {
  value: string;
  /** Called with the raw 11-digit string (e.g. "01012345678"). */
  onChange: (digits: string) => void;
  error?: boolean;
}

/** Auto-formatting Korean mobile input — accepts paste of any digit/
 *  hyphen pattern and emits a clean 11-digit string back via onChange. */
export const PhoneInputKR = forwardRef<HTMLInputElement, PhoneInputKRProps>(function PhoneInputKR(
  { value, onChange, error, className, placeholder = "010-1234-5678", ...rest },
  ref,
) {
  const display = formatKoreanPhone(value);
  return (
    <input
      ref={ref}
      type="tel"
      inputMode="numeric"
      autoComplete="tel"
      maxLength={13}
      value={display}
      placeholder={placeholder}
      className={cn("aph-input", error && "aph-input--error", className)}
      aria-invalid={error || undefined}
      onChange={(e: ChangeEvent<HTMLInputElement>) => onChange(parseKoreanPhone(e.target.value))}
      {...rest}
    />
  );
});
