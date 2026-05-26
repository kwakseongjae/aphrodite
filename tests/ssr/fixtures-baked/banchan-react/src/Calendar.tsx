"use client";

import { useState } from "react";
import { cn } from "./cn";

export interface CalendarProps {
  value?: Date;
  onChange?: (d: Date) => void;
  /** ISO yyyy-mm-dd strings to disable. */
  disabledDates?: string[];
  locale?: string;
  className?: string;
}

const KOREAN_DOW = ["일", "월", "화", "수", "목", "금", "토"];

export function Calendar({ value, onChange, disabledDates = [], locale = "ko-KR", className }: CalendarProps) {
  const initial = value ?? new Date();
  const [view, setView] = useState({ year: initial.getFullYear(), month: initial.getMonth() });
  const today = new Date();
  const todayIso = isoDate(today);
  const selectedIso = value ? isoDate(value) : null;

  const first = new Date(view.year, view.month, 1);
  const startDow = first.getDay();
  const daysInMonth = new Date(view.year, view.month + 1, 0).getDate();
  const prevMonthDays = new Date(view.year, view.month, 0).getDate();

  const cells: Array<{ d: number; otherMonth: boolean; iso: string; date: Date }> = [];
  for (let i = startDow - 1; i >= 0; i--) {
    const day = prevMonthDays - i;
    const date = new Date(view.year, view.month - 1, day);
    cells.push({ d: day, otherMonth: true, iso: isoDate(date), date });
  }
  for (let d = 1; d <= daysInMonth; d++) {
    const date = new Date(view.year, view.month, d);
    cells.push({ d, otherMonth: false, iso: isoDate(date), date });
  }
  while (cells.length % 7 !== 0) {
    const d = cells.length - (startDow + daysInMonth) + 1;
    const date = new Date(view.year, view.month + 1, d);
    cells.push({ d, otherMonth: true, iso: isoDate(date), date });
  }

  const monthLabel = new Date(view.year, view.month, 1).toLocaleDateString(locale, { year: "numeric", month: "long" });

  return (
    <div className={cn("aph-calendar", className)} role="grid" aria-label={monthLabel}>
      <div className="aph-calendar__head">
        <button type="button" className="aph-calendar__nav" aria-label="이전 달" onClick={() => setView(({ year, month }) => month === 0 ? { year: year - 1, month: 11 } : { year, month: month - 1 })}>‹</button>
        <span className="aph-calendar__month">{monthLabel}</span>
        <button type="button" className="aph-calendar__nav" aria-label="다음 달" onClick={() => setView(({ year, month }) => month === 11 ? { year: year + 1, month: 0 } : { year, month: month + 1 })}>›</button>
      </div>
      <div className="aph-calendar__grid">
        {KOREAN_DOW.map((d) => <div key={d} className="aph-calendar__dow">{d}</div>)}
        {cells.map((c, i) => {
          const disabled = disabledDates.includes(c.iso);
          const selected = selectedIso === c.iso;
          const isToday = todayIso === c.iso;
          return (
            <button
              key={i}
              type="button"
              role="gridcell"
              aria-selected={selected}
              aria-current={isToday ? "date" : undefined}
              disabled={disabled}
              className={cn(
                "aph-calendar__cell",
                c.otherMonth && "aph-calendar__cell--other",
                isToday && "aph-calendar__cell--today",
                selected && "aph-calendar__cell--selected",
              )}
              onClick={() => !disabled && onChange?.(c.date)}
            >
              {c.d}
            </button>
          );
        })}
      </div>
    </div>
  );
}

function isoDate(d: Date): string {
  const y = d.getFullYear();
  const m = String(d.getMonth() + 1).padStart(2, "0");
  const day = String(d.getDate()).padStart(2, "0");
  return `${y}-${m}-${day}`;
}
