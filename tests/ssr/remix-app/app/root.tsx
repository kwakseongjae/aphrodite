import { Links, Meta, Outlet, Scripts, ScrollRestoration } from "@remix-run/react";
import type { LinksFunction } from "@remix-run/node";
import stylesUrl from "@aphrodite-design/example-banchan/styles.css?url";

export const links: LinksFunction = () => [{ rel: "stylesheet", href: stylesUrl }];

export default function App() {
  return (
    <html lang="ko">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width,initial-scale=1" />
        <Meta />
        <Links />
      </head>
      <body data-variant="light" style={{ margin: 0, padding: 24, fontFamily: "Pretendard, Inter, system-ui, sans-serif" }}>
        <Outlet />
        <ScrollRestoration />
        <Scripts />
      </body>
    </html>
  );
}
