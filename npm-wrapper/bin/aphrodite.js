#!/usr/bin/env node
/**
 * Thin shim: exec the platform-native aphrodite binary downloaded by
 * scripts/install.js, passing through argv + stdio. Exit code mirrors
 * the binary's exit code.
 */

const { spawnSync } = require("node:child_process");
const path = require("node:path");
const fs = require("node:fs");

const PLATFORM = process.platform;
const binName = PLATFORM === "win32" ? "aphrodite.exe" : "aphrodite";
const binPath = path.join(__dirname, binName);

if (!fs.existsSync(binPath)) {
  process.stderr.write(
    "[aphrodite] Binary not found at " + binPath + "\n" +
    "[aphrodite] Did the postinstall download fail? Try:\n" +
    "[aphrodite]   npm rebuild @aphrodite-design/aphrodite-agent\n" +
    "[aphrodite] Or install from source:\n" +
    "[aphrodite]   cargo install aphrodite-cli\n",
  );
  process.exit(127);
}

const result = spawnSync(binPath, process.argv.slice(2), {
  stdio: "inherit",
  windowsHide: false,
});
process.exit(result.status ?? 1);
