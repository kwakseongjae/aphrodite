"use client";

import { Fragment, ReactNode } from "react";
import { cn } from "./cn";

export interface StepperProps {
  steps: Array<{ label: ReactNode }>;
  current: number;
  className?: string;
}

export function Stepper({ steps, current, className }: StepperProps) {
  return (
    <ol className={cn("aph-stepper", className)}>
      {steps.map((s, i) => (
        <Fragment key={i}>
          <li
            className={cn(
              "aph-stepper-step",
              i === current && "aph-stepper-step--active",
              i < current && "aph-stepper-step--done",
            )}
            aria-current={i === current ? "step" : undefined}
          >
            <span className="aph-stepper-bubble">{i < current ? "✓" : i + 1}</span>
            <span>{s.label}</span>
          </li>
          {i < steps.length - 1 && <span className="aph-stepper-line" aria-hidden="true" />}
        </Fragment>
      ))}
    </ol>
  );
}
