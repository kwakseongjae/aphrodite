"use client";

import { cn } from "./cn";

export interface ChartBarProps {
  bars: Array<{ label: string; value: number }>;
  width?: number;
  height?: number;
  className?: string;
  ariaLabel?: string;
}

export function ChartBar({ bars, width = 480, height = 200, className, ariaLabel = "Bar chart" }: ChartBarProps) {
  if (bars.length === 0) return null;
  const pad = 32;
  const maxV = Math.max(...bars.map((b) => b.value), 1);
  const barW = (width - pad * 2) / bars.length * 0.7;
  const gap = (width - pad * 2) / bars.length * 0.3;
  return (
    <svg className={cn("aph-chart", className)} viewBox={`0 0 ${width} ${height}`} role="img" aria-label={ariaLabel}>
      {bars.map((b, i) => {
        const x = pad + i * (barW + gap) + gap / 2;
        const h = (b.value / maxV) * (height - pad * 2);
        const y = height - pad - h;
        return (
          <g key={i}>
            <rect className="aph-chart-bar__bar" x={x} y={y} width={barW} height={h} rx={2}>
              <title>{b.label}: {b.value}</title>
            </rect>
            <text className="aph-chart__label" x={x + barW / 2} y={height - pad + 14} textAnchor="middle">{b.label}</text>
          </g>
        );
      })}
      <line className="aph-chart__axis" x1={pad} x2={width - pad} y1={height - pad} y2={height - pad} />
    </svg>
  );
}
