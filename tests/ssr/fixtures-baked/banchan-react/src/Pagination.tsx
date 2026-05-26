"use client";

import { cn } from "./cn";

export interface PaginationProps {
  page: number;
  pageCount: number;
  onChange: (page: number) => void;
  className?: string;
  /** Max number of page buttons shown. Default 7. */
  siblingCount?: number;
}

export function Pagination({ page, pageCount, onChange, className, siblingCount = 7 }: PaginationProps) {
  const pages: number[] = [];
  const half = Math.floor(siblingCount / 2);
  let start = Math.max(1, page - half);
  let end = Math.min(pageCount, start + siblingCount - 1);
  if (end - start + 1 < siblingCount) start = Math.max(1, end - siblingCount + 1);
  for (let i = start; i <= end; i++) pages.push(i);
  return (
    <nav aria-label="Pagination" className={cn("aph-pagination", className)}>
      <button type="button" className="aph-pagination__btn" disabled={page <= 1} onClick={() => onChange(page - 1)} aria-label="이전 페이지">‹</button>
      {pages.map((p) => (
        <button
          key={p}
          type="button"
          className="aph-pagination__btn"
          aria-current={p === page ? "page" : undefined}
          onClick={() => onChange(p)}
        >
          {p}
        </button>
      ))}
      <button type="button" className="aph-pagination__btn" disabled={page >= pageCount} onClick={() => onChange(page + 1)} aria-label="다음 페이지">›</button>
    </nav>
  );
}
