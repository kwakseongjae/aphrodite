"use client";

import { forwardRef, InputHTMLAttributes } from "react";
import { cn } from "./cn";

export interface DatePickerProps extends Omit<InputHTMLAttributes<HTMLInputElement>, "type"> {
  error?: boolean;
}

/** Native HTML5 date input — picks up locale automatically. For custom
 *  calendar UI use a third-party (react-aria-components / @internationalized/date)
 *  paired with our tokens. */
export const DatePicker = forwardRef<HTMLInputElement, DatePickerProps>(function DatePicker(
  { error, className, ...rest },
  ref,
) {
  return (
    <div className="aph-date-input">
      <input
        ref={ref}
        type="date"
        className={cn("aph-input", error && "aph-input--error", className)}
        aria-invalid={error || undefined}
        {...rest}
      />
    </div>
  );
});
