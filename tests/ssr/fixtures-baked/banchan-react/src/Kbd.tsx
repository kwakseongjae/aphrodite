"use client";

import { ReactNode } from "react";
import { cn } from "./cn";

export interface KbdProps {
  children: ReactNode;
  className?: string;
}

export function Kbd({ children, className }: KbdProps) {
  return <kbd className={cn("aph-kbd", className)}>{children}</kbd>;
}
