"use client";

import { ReactNode } from "react";
import { cn } from "./cn";

export interface ToolbarProps {
  children: ReactNode;
  className?: string;
  ariaLabel?: string;
}

export function Toolbar({ children, className, ariaLabel }: ToolbarProps) {
  return (
    <div role="toolbar" aria-label={ariaLabel} className={cn("aph-toolbar", className)}>
      {children}
    </div>
  );
}

Toolbar.Separator = function ToolbarSeparator() {
  return <span role="separator" aria-orientation="vertical" className="aph-toolbar__sep" />;
};
