"use client";

import { ChangeEvent } from "react";
import { cn } from "./cn";

export interface ColorPickerProps {
  value: string;
  onChange: (hex: string) => void;
  className?: string;
  ariaLabel?: string;
}

export function ColorPicker({ value, onChange, className, ariaLabel = "Color" }: ColorPickerProps) {
  const onSwatch = (e: ChangeEvent<HTMLInputElement>) => onChange(e.target.value);
  const onText = (e: ChangeEvent<HTMLInputElement>) => {
    const v = e.target.value.trim();
    if (/^#[0-9a-fA-F]{6}$/.test(v)) onChange(v);
    else if (/^#[0-9a-fA-F]{3}$/.test(v)) {
      const r = v[1], g = v[2], b = v[3];
      onChange(`#${r}${r}${g}${g}${b}${b}`);
    }
  };
  return (
    <div className={cn("aph-colorpicker", className)}>
      <input
        type="color"
        className="aph-colorpicker__swatch"
        aria-label={`${ariaLabel} swatch`}
        value={value}
        onChange={onSwatch}
      />
      <input
        type="text"
        className="aph-colorpicker__input"
        aria-label={`${ariaLabel} hex`}
        defaultValue={value}
        onBlur={onText}
        spellCheck={false}
      />
    </div>
  );
}
