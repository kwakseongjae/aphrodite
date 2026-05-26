"use client";

import { cn } from "./cn";

export interface ProgressBarProps {
  value: number;
  max?: number;
  label?: string;
  className?: string;
}

export function ProgressBar({ value, max = 100, label, className }: ProgressBarProps) {
  const pct = Math.max(0, Math.min(100, (value / max) * 100));
  return (
    <div role="progressbar" aria-valuenow={value} aria-valuemin={0} aria-valuemax={max} aria-label={label} className={cn("aph-progress", className)}>
      <div className="aph-progress__bar" style={{ width: `${pct}%` }} />
    </div>
  );
}
