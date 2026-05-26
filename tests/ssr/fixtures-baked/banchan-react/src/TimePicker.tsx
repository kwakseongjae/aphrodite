"use client";

import { ChangeEvent } from "react";
import { cn } from "./cn";

export interface TimePickerProps {
  /** "HH:mm" 24-hour. */
  value: string;
  onChange: (hhmm: string) => void;
  minuteStep?: number;
  className?: string;
  ariaLabel?: string;
}

export function TimePicker({ value, onChange, minuteStep = 15, className, ariaLabel }: TimePickerProps) {
  const [hStr = "09", mStr = "00"] = value.split(":");
  const onH = (e: ChangeEvent<HTMLSelectElement>) => onChange(`${e.target.value}:${mStr}`);
  const onM = (e: ChangeEvent<HTMLSelectElement>) => onChange(`${hStr}:${e.target.value}`);
  const hours = Array.from({ length: 24 }, (_, i) => String(i).padStart(2, "0"));
  const minutes: string[] = [];
  for (let m = 0; m < 60; m += minuteStep) minutes.push(String(m).padStart(2, "0"));
  return (
    <div role="group" aria-label={ariaLabel} className={cn("aph-timepicker", className)}>
      <select value={hStr} onChange={onH} aria-label="시간">
        {hours.map((h) => <option key={h} value={h}>{h}</option>)}
      </select>
      <span aria-hidden="true">:</span>
      <select value={mStr} onChange={onM} aria-label="분">
        {minutes.map((m) => <option key={m} value={m}>{m}</option>)}
      </select>
    </div>
  );
}
