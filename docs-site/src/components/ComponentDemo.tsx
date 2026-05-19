/**
 * React island that mounts the actual demo from the canonical
 * banchan fixture. The components-registry symlink points at the
 * tests/ssr/next-app version — single source of truth across SSR
 * harness + docs site.
 */
import COMPONENTS from "./components-registry";

export default function ComponentDemo({ name }: { name: string }) {
  const entry = COMPONENTS[name];
  if (!entry) {
    return <p>(Unknown component: {name})</p>;
  }
  const Demo = entry.demo;
  return (
    <div data-testid="component-root" data-component={name}>
      <Demo />
    </div>
  );
}
