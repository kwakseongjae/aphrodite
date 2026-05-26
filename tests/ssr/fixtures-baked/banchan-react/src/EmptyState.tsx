"use client";

import { ReactNode } from "react";
import { cn } from "./cn";

export interface EmptyStateProps {
  title: ReactNode;
  description?: ReactNode;
  action?: ReactNode;
  icon?: ReactNode;
  className?: string;
}

export function EmptyState({ title, description, action, icon, className }: EmptyStateProps) {
  return (
    <div role="status" className={cn("aph-empty", className)}>
      {icon}
      <div className="aph-empty__title">{title}</div>
      {description && <div>{description}</div>}
      {action}
    </div>
  );
}
