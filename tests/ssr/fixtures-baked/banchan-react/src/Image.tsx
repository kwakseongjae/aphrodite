"use client";

import { useState, ImgHTMLAttributes } from "react";
import { cn } from "./cn";

export interface ImageProps extends ImgHTMLAttributes<HTMLImageElement> {
  fallback?: string;
  width?: number;
  height?: number;
  rounded?: boolean | number;
}

export function Image({ src, fallback, width, height, rounded, alt = "", className, ...rest }: ImageProps) {
  const [errored, setErrored] = useState(false);
  const radius = typeof rounded === "number" ? rounded : rounded ? 999 : 0;
  return (
    <span
      className={cn("aph-image", className)}
      style={{ width, height, borderRadius: radius }}
    >
      {!errored && src ? (
        <img src={src} alt={alt} loading="lazy" onError={() => setErrored(true)} {...rest} />
      ) : (
        <span className="aph-image__fallback">{fallback ?? (alt || "이미지를 불러올 수 없습니다")}</span>
      )}
    </span>
  );
}
