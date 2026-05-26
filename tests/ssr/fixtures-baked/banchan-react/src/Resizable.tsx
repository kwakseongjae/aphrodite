"use client";

import { useRef, ReactNode, useState, MouseEvent as ReactMouseEvent } from "react";
import { cn } from "./cn";

export interface ResizableProps {
  left: ReactNode;
  right: ReactNode;
  /** Initial left-pane width (px). */
  initialLeft?: number;
  /** Minimum left-pane width (px). */
  minLeft?: number;
  /** Minimum right-pane width (px). */
  minRight?: number;
  className?: string;
}

export function Resizable({ left, right, initialLeft = 280, minLeft = 120, minRight = 200, className }: ResizableProps) {
  const wrapRef = useRef<HTMLDivElement>(null);
  const [leftW, setLeftW] = useState(initialLeft);
  const onMouseDown = (_e: ReactMouseEvent<HTMLDivElement>) => {
    const wrap = wrapRef.current;
    if (!wrap) return;
    const rect = wrap.getBoundingClientRect();
    const onMove = (e: MouseEvent) => {
      const next = e.clientX - rect.left;
      const max = rect.width - minRight;
      setLeftW(Math.max(minLeft, Math.min(max, next)));
    };
    const onUp = () => {
      window.removeEventListener("mousemove", onMove);
      window.removeEventListener("mouseup", onUp);
    };
    window.addEventListener("mousemove", onMove);
    window.addEventListener("mouseup", onUp);
  };
  return (
    <div ref={wrapRef} className={cn("aph-resizable", className)}>
      <div className="aph-resizable__pane" style={{ width: leftW, flexShrink: 0 }}>{left}</div>
      <div role="separator" aria-orientation="vertical" className="aph-resizable__handle" onMouseDown={onMouseDown} />
      <div className="aph-resizable__pane" style={{ flex: 1 }}>{right}</div>
    </div>
  );
}
