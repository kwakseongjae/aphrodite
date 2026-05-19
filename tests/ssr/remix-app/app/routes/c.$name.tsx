import { useParams } from "@remix-run/react";
import COMPONENTS from "../components-registry";

export default function ComponentRoute() {
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
