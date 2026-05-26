"use client";

import { ReactNode, Children, cloneElement, isValidElement } from "react";
import { cn } from "./cn";

export interface RadioGroupProps {
  name: string;
  value: string;
  onChange: (next: string) => void;
  horizontal?: boolean;
  className?: string;
  children: ReactNode;
}

export function RadioGroup({ name, value, onChange, horizontal, className, children }: RadioGroupProps) {
  return (
    <div role="radiogroup" className={cn("aph-radio-group", horizontal && "aph-radio-group--horizontal", className)}>
      {Children.map(children, (child) => {
        if (!isValidElement(child)) return child;
        const props = child.props as Record<string, unknown>;
        return cloneElement(child as React.ReactElement<Record<string, unknown>>, {
          name,
          checked: props.value === value,
          onChange: (e: React.ChangeEvent<HTMLInputElement>) => onChange(e.target.value),
        });
      })}
    </div>
  );
}
