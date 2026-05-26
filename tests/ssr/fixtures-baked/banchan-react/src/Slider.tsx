"use client";

import { forwardRef, InputHTMLAttributes } from "react";
import { cn } from "./cn";

export interface SliderProps extends Omit<InputHTMLAttributes<HTMLInputElement>, "type"> {}

export const Slider = forwardRef<HTMLInputElement, SliderProps>(function Slider(
  { className, ...rest },
  ref,
) {
  return <input ref={ref} type="range" className={cn("aph-slider", className)} {...rest} />;
});
