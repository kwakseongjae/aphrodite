"use client";

import { useState, useEffect, useRef, ReactNode, MouseEvent as ReactMouseEvent } from "react";
import { cn } from "./cn";

export interface ContextMenuProps {
  trigger: ReactNode;
  items: Array<{ label: ReactNode; onSelect: () => void; disabled?: boolean }>;
  className?: string;
}

export function ContextMenu({ trigger, items, className }: ContextMenuProps) {
  const [pos, setPos] = useState<{ x: number; y: number } | null>(null);
  const ref = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!pos) return;
    const onDoc = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) setPos(null);
    };
    const onKey = (e: KeyboardEvent) => { if (e.key === "Escape") setPos(null); };
    document.addEventListener("mousedown", onDoc);
    document.addEventListener("keydown", onKey);
    return () => { document.removeEventListener("mousedown", onDoc); document.removeEventListener("keydown", onKey); };
  }, [pos]);
  const onTrigger = (e: ReactMouseEvent) => {
    e.preventDefault();
    setPos({ x: e.clientX, y: e.clientY });
  };
  return (
    <>
      <span onContextMenu={onTrigger}>{trigger}</span>
      {pos && (
        <div ref={ref} role="menu" className={cn("aph-context-menu", className)} style={{ left: pos.x, top: pos.y }}>
          {items.map((it, i) => (
            <button key={i} type="button" role="menuitem" className="aph-menu-item" disabled={it.disabled} onClick={() => { it.onSelect(); setPos(null); }}>
              {it.label}
            </button>
          ))}
        </div>
      )}
    </>
  );
}
