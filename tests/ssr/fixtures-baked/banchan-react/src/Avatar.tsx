"use client";

import { forwardRef, HTMLAttributes, ReactNode } from "react";
import { cn } from "./cn";

export type AvatarSize = "sm" | "md" | "lg";

export interface AvatarProps extends HTMLAttributes<HTMLSpanElement> {
  size?: AvatarSize;
  initials?: string;
  src?: string;
  alt?: string;
  children?: ReactNode;
}

export const Avatar = forwardRef<HTMLSpanElement, AvatarProps>(function Avatar(
  { size = "md", initials, src, alt, className, children, ...rest },
  ref,
) {
  if (src) {
    return (
      <span
        ref={ref}
        className={cn("aph-avatar", size !== "md" && `aph-avatar--${size}`, className)}
        {...rest}
      >
        <img src={src} alt={alt ?? ""} style={{ width: "100%", height: "100%", borderRadius: "999px", objectFit: "cover" }} />
      </span>
    );
  }
  return (
    <span
      ref={ref}
      className={cn("aph-avatar", size !== "md" && `aph-avatar--${size}`, className)}
      aria-label={alt}
      {...rest}
    >
      {initials ?? children}
    </span>
  );
});
