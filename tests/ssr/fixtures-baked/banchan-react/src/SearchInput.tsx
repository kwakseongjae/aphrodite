"use client";

import { forwardRef, InputHTMLAttributes } from "react";
import { cn } from "./cn";

export interface SearchInputProps extends Omit<InputHTMLAttributes<HTMLInputElement>, "type" | "onChange"> {
  value: string;
  onChange: (v: string) => void;
  onClear?: () => void;
}

export const SearchInput = forwardRef<HTMLInputElement, SearchInputProps>(function SearchInput(
  { value, onChange, onClear, className, ...rest },
  ref,
) {
  const clear = () => {
    onChange("");
    onClear?.();
  };
  return (
    <div className={cn("aph-search", className)}>
      <input
        ref={ref}
        type="search"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        className="aph-input aph-search__input"
        {...rest}
      />
      {value && (
        <button type="button" className="aph-search__clear" aria-label="검색어 지우기" onClick={clear}>×</button>
      )}
    </div>
  );
});
