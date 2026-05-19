import { vitePlugin as remix } from "@remix-run/dev";
import { defineConfig } from "vite";
import tsconfigPaths from "vite-tsconfig-paths";

export default defineConfig({
  plugins: [remix(), tsconfigPaths()],
  server: { port: 3002, strictPort: true },
  // The "use client" directives in fixture components are RSC-specific
  // and harmless in Remix; remix doesn't use RSC. ssr:noExternal makes
  // Vite bundle the fixture through SSR rather than treating it as an
  // external package.
  ssr: {
    noExternal: ["@aphrodite-design/example-banchan"],
  },
});
