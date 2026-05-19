"use client";

import COMPONENTS from "../../components-registry";

// Page itself is a Client Component — it dot-accesses .demo on the
// registry which is also "use client". Crossing the RSC boundary that
// way would throw at runtime (Cannot access .demo on the server).
// SSR still happens — Next.js server-renders client components on
// initial request; that's exactly the layer Gate 1 needs to exercise.
export default function ComponentPage({ params }: { params: { name: string } }) {
  const entry = COMPONENTS[params.name];
  if (!entry) {
    return (
      <main>
        <h1>Unknown component: {params.name}</h1>
      </main>
    );
  }
  const Demo = entry.demo;
  return (
    <main>
      <h1 style={{ fontSize: 24, marginBottom: 24 }} data-component-name={params.name}>
        {params.name}
      </h1>
      <div data-testid="component-root" data-component={params.name}>
        <Demo />
      </div>
    </main>
  );
}
