"use client";

import { ReactElement, cloneElement, useState } from "react";

export interface TooltipProps {
  content: string;
  children: ReactElement;
}

export function Tooltip({ content, children }: TooltipProps) {
  const [open, setOpen] = useState(false);
  return (
    <span className="aph-tooltip-wrap" onMouseEnter={() => setOpen(true)} onMouseLeave={() => setOpen(false)} onFocus={() => setOpen(true)} onBlur={() => setOpen(false)}>
      {cloneElement(children, { "aria-describedby": open ? "aph-tooltip-active" : undefined })}
      <span role="tooltip" id="aph-tooltip-active" className="aph-tooltip">{content}</span>
    </span>
  );
}
