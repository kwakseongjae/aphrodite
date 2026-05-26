"use client";

import { ReactNode, ButtonHTMLAttributes, forwardRef } from "react";
import { cn } from "./cn";

export interface MenuProps {
  children: ReactNode;
  className?: string;
  ariaLabel?: string;
}

export interface MenuItemProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  children: ReactNode;
}

export function Menu({ children, className, ariaLabel }: MenuProps) {
  return (
    <div role="menu" aria-label={ariaLabel} className={cn("aph-menu", className)}>
      {children}
    </div>
  );
}

export const MenuItem = forwardRef<HTMLButtonElement, MenuItemProps>(function MenuItem(
  { children, className, ...rest },
  ref,
) {
  return (
    <button ref={ref} type="button" role="menuitem" className={cn("aph-menu-item", className)} {...rest}>
      {children}
    </button>
  );
});
