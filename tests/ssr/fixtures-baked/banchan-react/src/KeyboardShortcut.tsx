"use client";

import { ReactNode } from "react";
import { Kbd } from "./Kbd";
import { cn } from "./cn";

export interface KeyboardShortcutProps {
  keys: ReactNode[];
  label?: ReactNode;
  className?: string;
}

export function KeyboardShortcut({ keys, label, className }: KeyboardShortcutProps) {
  return (
    <span className={cn("aph-kshortcut", className)}>
      {label && <span>{label}</span>}
      <span className="aph-kshortcut__keys">
        {keys.map((k, i) => (
          <span key={i}>
            {i > 0 && <span aria-hidden="true"> + </span>}
            <Kbd>{k}</Kbd>
          </span>
        ))}
      </span>
    </span>
  );
}
