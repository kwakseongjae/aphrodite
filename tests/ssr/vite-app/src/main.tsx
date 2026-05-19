import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { BrowserRouter, Routes, Route, Link, useParams } from "react-router-dom";
import "@aphrodite-design/example-banchan/styles.css";
import COMPONENTS from "./components-registry";

function IndexPage() {
  const names = Object.keys(COMPONENTS).sort();
  return (
    <main>
      <h1>Aphrodite SSR matrix — Vite — {names.length} components</h1>
      <p>Each link below renders one component on its own route. Used by the Gate 1 Playwright smoke spec.</p>
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

function ComponentPage() {
  const { name = "" } = useParams<{ name: string }>();
  const entry = COMPONENTS[name];
  if (!entry) {
    return (
      <main>
        <h1>Unknown component: {name}</h1>
      </main>
    );
  }
  const Demo = entry.demo;
  return (
    <main>
      <h1 style={{ fontSize: 24, marginBottom: 24 }} data-component-name={name}>{name}</h1>
      <div data-testid="component-root" data-component={name}>
        <Demo />
      </div>
    </main>
  );
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<IndexPage />} />
        <Route path="/c/:name" element={<ComponentPage />} />
      </Routes>
    </BrowserRouter>
  </StrictMode>,
);
