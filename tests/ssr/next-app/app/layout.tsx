import type { ReactNode } from "react";
import "@aphrodite-design/example-banchan/styles.css";

export const metadata = {
  title: "Aphrodite SSR test — Next.js 14",
  description: "Gate 1 SSR test harness for every emitted component.",
};

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html lang="ko">
      <body data-variant="light" style={{ margin: 0, padding: 24, fontFamily: "Pretendard, Inter, system-ui, sans-serif" }}>
        {children}
      </body>
    </html>
  );
}
