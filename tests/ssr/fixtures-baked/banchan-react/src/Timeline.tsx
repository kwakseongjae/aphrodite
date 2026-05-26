"use client";

import { ReactNode } from "react";
import { cn } from "./cn";

export interface TimelineProps {
  children: ReactNode;
  className?: string;
}

export interface TimelineItemProps {
  marker?: ReactNode;
  title: ReactNode;
  time?: ReactNode;
  children?: ReactNode;
}

export function Timeline({ children, className }: TimelineProps) {
  return <ol className={cn("aph-timeline", className)}>{children}</ol>;
}

export function TimelineItem({ marker, title, time, children }: TimelineItemProps) {
  return (
    <li className="aph-timeline-item">
      <span className="aph-timeline-item__bubble" aria-hidden="true">{marker ?? "•"}</span>
      <div className="aph-timeline-item__body">
        <span className="aph-timeline-item__title">{title}</span>
        {time && <span className="aph-timeline-item__time">{time}</span>}
        {children && <div>{children}</div>}
      </div>
    </li>
  );
}
