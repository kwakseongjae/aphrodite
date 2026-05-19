import { Link } from "@remix-run/react";
import COMPONENTS from "../components-registry";

export default function Index() {
  const names = Object.keys(COMPONENTS).sort();
  return (
    <main>
      <h1>Aphrodite SSR matrix — Remix — {names.length} components</h1>
      <ul style={{ columns: 3, padding: 0, listStyle: "none" }}>
        {names.map((n) => (
          <li key={n}>
            <Link to={`/c/${n}`}>{n}</Link>
          </li>
        ))}
      </ul>
    </main>
  );
}
