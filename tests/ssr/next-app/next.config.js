/** @type {import('next').NextConfig} */
const nextConfig = {
  // The fixture is consumed via `file:` link — transpilePackages tells
  // Next.js to push the symlinked source through its TS+SWC pipeline.
  transpilePackages: ["@aphrodite-design/example-banchan"],
  // Surface every console warning during build so hydration mismatches
  // become visible in CI logs.
  reactStrictMode: true,
};

module.exports = nextConfig;
