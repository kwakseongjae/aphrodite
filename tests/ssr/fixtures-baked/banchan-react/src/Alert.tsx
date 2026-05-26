"use client";

import { ReactNode } from "react";
import { cn } from "./cn";

export type AlertTone = "info" | "success" | "warning" | "danger";

export interface AlertProps {
  tone?: AlertTone;
  title?: ReactNode;
  children?: ReactNode;
  icon?: ReactNode;
  className?: string;
}

export function Alert({ tone = "info", title, children, icon, className }: AlertProps) {
  return (
    <div role="alert" className={cn("aph-alert", `aph-alert--${tone}`, className)}>
      {icon && <span aria-hidden="true">{icon}</span>}
      <div>
        {title && <div className="aph-alert__title">{title}</div>}
        {children && <div>{children}</div>}
      </div>
    </div>
  );
}
