"use client";

import { ReactNode, useState } from "react";
import { cn } from "./cn";

export interface BannerProps {
  tone?: "info" | "warning" | "danger";
  children: ReactNode;
  dismissible?: boolean;
  onDismiss?: () => void;
  className?: string;
}

export function Banner({ tone = "info", children, dismissible, onDismiss, className }: BannerProps) {
  const [hidden, setHidden] = useState(false);
  if (hidden) return null;
  return (
    <div role="status" className={cn("aph-banner", `aph-banner--${tone}`, className)}>
      <span>{children}</span>
      {dismissible && (
        <button type="button" className="aph-banner__dismiss" aria-label="닫기" onClick={() => { setHidden(true); onDismiss?.(); }}>×</button>
      )}
    </div>
  );
}
