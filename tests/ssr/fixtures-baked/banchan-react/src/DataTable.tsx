"use client";

import { useState, useMemo, ReactNode } from "react";
import { cn } from "./cn";

export interface DataTableColumn<T> {
  key: keyof T & string;
  header: ReactNode;
  /** When true, header is clickable and toggles asc → desc → none. */
  sortable?: boolean;
  /** Custom cell renderer. */
  cell?: (row: T) => ReactNode;
  align?: "left" | "right" | "center";
  width?: string;
}

export interface DataTableProps<T> {
  rows: T[];
  columns: DataTableColumn<T>[];
  /** Stable key fn for React keys + selection. Defaults to row index. */
  rowKey?: (row: T, i: number) => string | number;
  className?: string;
  emptyMessage?: ReactNode;
}

type SortDir = "asc" | "desc" | null;

export function DataTable<T>({ rows, columns, rowKey, className, emptyMessage = "데이터가 없습니다" }: DataTableProps<T>) {
  const [sortKey, setSortKey] = useState<string | null>(null);
  const [sortDir, setSortDir] = useState<SortDir>(null);
  const sorted = useMemo(() => {
    if (!sortKey || !sortDir) return rows;
    return [...rows].sort((a, b) => {
      const av = (a as unknown as Record<string, unknown>)[sortKey];
      const bv = (b as unknown as Record<string, unknown>)[sortKey];
      if (av == null && bv == null) return 0;
      if (av == null) return 1;
      if (bv == null) return -1;
      const cmp = av < bv ? -1 : av > bv ? 1 : 0;
      return sortDir === "asc" ? cmp : -cmp;
    });
  }, [rows, sortKey, sortDir]);
  const cycle = (key: string) => {
    if (sortKey !== key) { setSortKey(key); setSortDir("asc"); return; }
    if (sortDir === "asc") { setSortDir("desc"); return; }
    if (sortDir === "desc") { setSortKey(null); setSortDir(null); return; }
    setSortDir("asc");
  };
  return (
    <div className={cn("aph-table-wrap", className)}>
      <table className="aph-table">
        <thead>
          <tr>
            {columns.map((c) => {
              const sort = c.sortable && sortKey === c.key ? (sortDir === "asc" ? "ascending" : "descending") : c.sortable ? "none" : undefined;
              return (
                <th
                  key={c.key}
                  aria-sort={sort as React.AriaAttributes["aria-sort"]}
                  style={{ textAlign: c.align ?? "left", width: c.width }}
                  onClick={c.sortable ? () => cycle(c.key) : undefined}
                >
                  {c.header}
                  {c.sortable && (
                    <span className={cn("aph-table__sort-icon", sortKey === c.key && "aph-table__sort-icon--active")}>
                      {sortKey === c.key ? (sortDir === "asc" ? "▲" : "▼") : "⇅"}
                    </span>
                  )}
                </th>
              );
            })}
          </tr>
        </thead>
        <tbody>
          {sorted.length === 0 ? (
            <tr><td colSpan={columns.length} style={{ textAlign: "center", padding: "32px 16px", color: "var(--colors-text-muted)" }}>{emptyMessage}</td></tr>
          ) : (
            sorted.map((row, i) => (
              <tr key={rowKey ? rowKey(row, i) : i}>
                {columns.map((c) => (
                  <td key={c.key} style={{ textAlign: c.align ?? "left" }}>
                    {c.cell ? c.cell(row) : String((row as unknown as Record<string, unknown>)[c.key] ?? "")}
                  </td>
                ))}
              </tr>
            ))
          )}
        </tbody>
      </table>
    </div>
  );
}
