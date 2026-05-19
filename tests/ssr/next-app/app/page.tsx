import Link from "next/link";
import COMPONENTS from "./components-registry";

export default function IndexPage() {
  const names = Object.keys(COMPONENTS).sort();
  return (
    <main>
      <h1>Aphrodite SSR matrix — {names.length} components</h1>
      <p>Each link below renders one component on its own server-rendered page. Used by the Gate 1 Playwright smoke spec.</p>
      <ul style={{ columns: 3, padding: 0, listStyle: "none" }}>
        {names.map((n) => (
          <li key={n}>
            <Link href={`/c/${n}`}>{n}</Link>
          </li>
        ))}
      </ul>
    </main>
  );
}
