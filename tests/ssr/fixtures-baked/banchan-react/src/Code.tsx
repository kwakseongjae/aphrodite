"use client";

import { ReactNode, HTMLAttributes } from "react";
import { cn } from "./cn";

export interface CodeProps extends HTMLAttributes<HTMLElement> {
  block?: boolean;
  children: ReactNode;
}

export function Code({ block, className, children, ...rest }: CodeProps) {
  if (block) {
    return (
      <pre className={cn("aph-code-block", className)} {...rest as HTMLAttributes<HTMLPreElement>}>
        <code>{children}</code>
      </pre>
    );
  }
  return <code className={cn("aph-code", className)} {...rest}>{children}</code>;
}
