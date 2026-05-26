"use client";

import { ReactNode, ReactElement, useId } from "react";
import { cn } from "./cn";

export interface FormFieldProps {
  label?: ReactNode;
  hint?: ReactNode;
  error?: ReactNode;
  className?: string;
  /** A single form control (Input, Select, etc) — receives id+aria from the field. */
  children: ReactElement<{ id?: string; "aria-describedby"?: string; "aria-invalid"?: boolean }>;
}

export function FormField({ label, hint, error, className, children }: FormFieldProps) {
  const id = useId();
  const hintId = `${id}-hint`;
  const errorId = `${id}-error`;
  const describedBy = [hint ? hintId : null, error ? errorId : null].filter(Boolean).join(" ") || undefined;
  const child = {
    ...children,
    props: {
      ...children.props,
      id: children.props.id ?? id,
      "aria-describedby": describedBy,
      "aria-invalid": error ? true : undefined,
    },
  };
  return (
    <div className={cn("aph-form-field", className)}>
      {label && <label htmlFor={id} className="aph-form-field__label">{label}</label>}
      {child}
      {hint && !error && <span id={hintId} className="aph-form-field__hint">{hint}</span>}
      {error && <span id={errorId} className="aph-form-field__error" role="alert">{error}</span>}
    </div>
  );
}
