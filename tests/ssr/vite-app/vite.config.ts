import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  server: {
    port: 3001,
    strictPort: true,
  },
  preview: {
    port: 3001,
    strictPort: true,
  },
  // The fixture is consumed via `file:` link; Vite handles TS in
  // node_modules through esbuild by default. No extra config needed.
  optimizeDeps: {
    include: ["@aphrodite-design/example-banchan"],
  },
});
