"use client";

import { ReactNode } from "react";
import { cn } from "./cn";

export interface QuoteProps {
  children: ReactNode;
  cite?: ReactNode;
  className?: string;
}

export function Quote({ children, cite, className }: QuoteProps) {
  return (
    <blockquote className={cn("aph-quote", className)}>
      {children}
      {cite && <cite className="aph-quote__cite">— {cite}</cite>}
    </blockquote>
  );
}
