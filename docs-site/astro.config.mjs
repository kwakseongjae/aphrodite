import { defineConfig } from "astro/config";
import react from "@astrojs/react";

export default defineConfig({
  integrations: [react()],
  site: "https://docs.aphrodite.dev",
  vite: {
    optimizeDeps: { include: ["@aphrodite-design/example-banchan"] },
    ssr: {
      // Bundle the fixture through Vite SSR — same trick as Remix app.
      noExternal: ["@aphrodite-design/example-banchan"],
    },
  },
});
