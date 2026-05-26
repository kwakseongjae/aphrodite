"use client";

import { useId } from "react";
import { cn } from "./cn";

export interface DateRange {
  start: string;
  end: string;
}

export interface DateRangePickerProps {
  value: DateRange;
  onChange: (next: DateRange) => void;
  min?: string;
  max?: string;
  className?: string;
  labelStart?: string;
  labelEnd?: string;
}

export function DateRangePicker({ value, onChange, min, max, className, labelStart = "시작", labelEnd = "종료" }: DateRangePickerProps) {
  const id = useId();
  return (
    <div className={cn("aph-daterange", className)}>
      <input
        type="date"
        id={`${id}-start`}
        aria-label={labelStart}
        className="aph-input"
        style={{ minHeight: 40, padding: "8px 10px" }}
        value={value.start}
        max={value.end || max}
        min={min}
        onChange={(e) => onChange({ ...value, start: e.target.value })}
      />
      <span className="aph-daterange__sep" aria-hidden="true">—</span>
      <input
        type="date"
        id={`${id}-end`}
        aria-label={labelEnd}
        className="aph-input"
        style={{ minHeight: 40, padding: "8px 10px" }}
        value={value.end}
        min={value.start || min}
        max={max}
        onChange={(e) => onChange({ ...value, end: e.target.value })}
      />
    </div>
  );
}
