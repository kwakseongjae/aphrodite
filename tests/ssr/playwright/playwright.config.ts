import { defineConfig, devices } from "@playwright/test";

/** Playwright config for the Gate 1 SSR matrix. Expects a Next.js
 *  server running on http://localhost:3000 (started via `pnpm start`
 *  in tests/ssr/next-app after `pnpm build`). The webServer block
 *  below auto-starts it for local runs; CI starts it explicitly. */
export default defineConfig({
  testDir: ".",
  timeout: 30_000,
  retries: 0,
  fullyParallel: true,
  reporter: [["list"], ["html", { open: "never", outputFolder: "playwright-report" }]],
  use: {
    baseURL: process.env.APH_SSR_BASE_URL ?? "http://localhost:3000",
    actionTimeout: 10_000,
  },
  projects: [
    { name: "chromium-desktop", use: { ...devices["Desktop Chrome"] } },
  ],
  // Auto-start the Next.js server when running locally. CI passes
  // APH_SSR_SKIP_WEBSERVER=1 because it manages the process itself.
  webServer: process.env.APH_SSR_SKIP_WEBSERVER
    ? undefined
    : {
        command: "cd ../next-app && pnpm start",
        url: "http://localhost:3000",
        reuseExistingServer: !process.env.CI,
        timeout: 60_000,
      },
});
