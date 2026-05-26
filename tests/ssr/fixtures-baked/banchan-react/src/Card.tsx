"use client";

import { forwardRef, HTMLAttributes, ReactNode } from "react";
import { cn } from "./cn";

export interface CardProps extends HTMLAttributes<HTMLDivElement> {
  padded?: boolean;
  children: ReactNode;
}

export const Card = forwardRef<HTMLDivElement, CardProps>(function Card(
  { padded = true, className, children, ...rest },
  ref,
) {
  return (
    <div ref={ref} className={cn("aph-card", padded && "aph-card--padded", className)} {...rest}>
      {children}
    </div>
  );
});
