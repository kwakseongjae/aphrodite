"use client";

import { ReactNode, Fragment } from "react";
import { cn } from "./cn";

export interface BreadcrumbProps {
  items: Array<{ label: ReactNode; href?: string }>;
  className?: string;
  separator?: ReactNode;
}

export function Breadcrumb({ items, className, separator = "/" }: BreadcrumbProps) {
  return (
    <nav aria-label="Breadcrumb" className={cn("aph-breadcrumb", className)}>
      {items.map((item, i) => {
        const last = i === items.length - 1;
        return (
          <Fragment key={i}>
            {item.href && !last ? (
              <a href={item.href}>{item.label}</a>
            ) : (
              <span className={last ? "aph-breadcrumb__current" : undefined} aria-current={last ? "page" : undefined}>{item.label}</span>
            )}
            {!last && <span className="aph-breadcrumb__sep" aria-hidden="true">{separator}</span>}
          </Fragment>
        );
      })}
    </nav>
  );
}
