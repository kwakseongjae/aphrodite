"use client";

import { CSSProperties } from "react";
import { cn } from "./cn";

export interface SkeletonProps {
  width?: number | string;
  height?: number | string;
  circle?: boolean;
  className?: string;
  style?: CSSProperties;
}

export function Skeleton({ width, height, circle, className, style }: SkeletonProps) {
  return (
    <span
      aria-hidden="true"
      className={cn("aph-skeleton", className)}
      style={{
        display: "inline-block",
        width: width ?? "100%",
        height: height ?? 16,
        borderRadius: circle ? "999px" : undefined,
        ...style,
      }}
    />
  );
}
