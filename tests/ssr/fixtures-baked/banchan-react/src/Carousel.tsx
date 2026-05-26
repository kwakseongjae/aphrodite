"use client";

import { useState, ReactNode, Children, useEffect } from "react";
import { cn } from "./cn";

export interface CarouselProps {
  children: ReactNode;
  autoplay?: number;
  showArrows?: boolean;
  showDots?: boolean;
  className?: string;
  ariaLabel?: string;
}

export function Carousel({ children, autoplay, showArrows = true, showDots = true, className, ariaLabel }: CarouselProps) {
  const slides = Children.toArray(children);
  const [idx, setIdx] = useState(0);
  const go = (next: number) => setIdx((next + slides.length) % slides.length);
  useEffect(() => {
    if (!autoplay) return;
    const t = setInterval(() => go(idx + 1), autoplay);
    return () => clearInterval(t);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [autoplay, idx, slides.length]);
  return (
    <div role="region" aria-roledescription="carousel" aria-label={ariaLabel} className={cn("aph-carousel", className)}>
      <div className="aph-carousel__track" style={{ transform: `translateX(-${idx * 100}%)` }}>
        {slides.map((s, i) => (
          <div key={i} className="aph-carousel__slide" aria-hidden={i !== idx}>{s}</div>
        ))}
      </div>
      {showArrows && slides.length > 1 && (
        <>
          <button type="button" className="aph-carousel__btn aph-carousel__btn--prev" onClick={() => go(idx - 1)} aria-label="이전">‹</button>
          <button type="button" className="aph-carousel__btn aph-carousel__btn--next" onClick={() => go(idx + 1)} aria-label="다음">›</button>
        </>
      )}
      {showDots && slides.length > 1 && (
        <div className="aph-carousel__dots" role="tablist">
          {slides.map((_, i) => (
            <button key={i} type="button" role="tab" aria-current={i === idx} className="aph-carousel__dot" onClick={() => setIdx(i)} aria-label={`슬라이드 ${i + 1}`} />
          ))}
        </div>
      )}
    </div>
  );
}
