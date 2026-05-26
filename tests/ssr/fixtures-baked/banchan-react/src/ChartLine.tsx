"use client";

import { cn } from "./cn";

export interface ChartLineProps {
  points: Array<{ x: number | string; y: number }>;
  width?: number;
  height?: number;
  className?: string;
  showArea?: boolean;
  showDots?: boolean;
  ariaLabel?: string;
}

export function ChartLine({ points, width = 480, height = 200, className, showArea = true, showDots = true, ariaLabel = "Line chart" }: ChartLineProps) {
  if (points.length === 0) return null;
  const pad = 32;
  const ys = points.map((p) => p.y);
  const minY = Math.min(...ys, 0);
  const maxY = Math.max(...ys, 1);
  const stepX = (width - pad * 2) / Math.max(1, points.length - 1);
  const scaleY = (y: number) => height - pad - ((y - minY) / (maxY - minY || 1)) * (height - pad * 2);
  const coords = points.map((p, i) => ({ cx: pad + i * stepX, cy: scaleY(p.y) }));
  const path = coords.map((c, i) => `${i === 0 ? "M" : "L"} ${c.cx.toFixed(1)} ${c.cy.toFixed(1)}`).join(" ");
  const area = `${path} L ${coords[coords.length - 1].cx.toFixed(1)} ${(height - pad).toFixed(1)} L ${coords[0].cx.toFixed(1)} ${(height - pad).toFixed(1)} Z`;
  return (
    <svg className={cn("aph-chart", className)} viewBox={`0 0 ${width} ${height}`} role="img" aria-label={ariaLabel}>
      {showArea && <path className="aph-chart-line__area" d={area} />}
      <path className="aph-chart-line__path" d={path} />
      {showDots && coords.map((c, i) => <circle key={i} className="aph-chart-line__dot" cx={c.cx} cy={c.cy} r={3} />)}
      <line className="aph-chart__axis" x1={pad} x2={width - pad} y1={height - pad} y2={height - pad} />
    </svg>
  );
}
