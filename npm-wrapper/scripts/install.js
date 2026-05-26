#!/usr/bin/env node
/**
 * Aphrodite npm wrapper — postinstall.
 *
 * Detects host platform/arch and downloads the matching Rust binary
 * from GitHub Releases into ./bin/aphrodite (or aphrodite.exe on
 * Windows). The tarball NEVER ships the binary itself; this script
 * pulls it lazily on `npm install`.
 *
 * Explicit UX with progress bar (deliberately not silent — first-time
 * users on slow networks see what's happening and can diagnose
 * failures faster).
 */

const fs = require("node:fs");
const path = require("node:path");
const https = require("node:https");
const { pipeline } = require("node:stream/promises");
const { execSync } = require("node:child_process");

const pkg = require("../package.json");

// During development the package.json version is "0.0.0" — fall back
// to the env var so local CI can test against a specific release tag.
const VERSION = process.env.APHRODITE_AGENT_VERSION || pkg.version;
const REPO = "kwakseongjae/aphrodite";
const BIN_DIR = path.join(__dirname, "..", "bin");
const PLATFORM = process.platform; // 'darwin' | 'linux' | 'win32'
const ARCH = process.arch; // 'arm64' | 'x64'

// Map (platform, arch) → release-asset suffix produced by our
// .github/workflows/release.yml matrix.
const TARGETS = {
  "darwin-arm64": "aphrodite-darwin-arm64.tar.gz",
  "darwin-x64": "aphrodite-darwin-x86_64.tar.gz",
  "linux-x64": "aphrodite-linux-x86_64.tar.gz",
  "linux-arm64": "aphrodite-linux-arm64.tar.gz",
  "win32-x64": "aphrodite-windows-x86_64.zip",
};

function exitWith(msg, code = 1) {
  process.stderr.write(`\n[aphrodite] ${msg}\n`);
  process.exit(code);
}

function info(msg) {
  process.stdout.write(`[aphrodite] ${msg}\n`);
}

function unsupportedPlatform() {
  exitWith(
    `Unsupported platform ${PLATFORM}/${ARCH}.\n` +
    "Supported during beta: darwin-arm64, darwin-x64.\n" +
    "Coming for 1.0: linux-x64, linux-arm64, win32-x64.\n" +
    "If you're on something else, run `cargo install aphrodite-cli` from source.",
  );
}

async function downloadWithProgress(url, dest) {
  return new Promise((resolve, reject) => {
    const req = https.get(url, { headers: { "User-Agent": "aphrodite-installer" } }, (res) => {
      if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
        return downloadWithProgress(res.headers.location, dest).then(resolve, reject);
      }
      if (res.statusCode !== 200) {
        return reject(new Error(`HTTP ${res.statusCode} from ${url}`));
      }
      const total = parseInt(res.headers["content-length"] || "0", 10);
      let received = 0;
      let lastPercent = -1;
      res.on("data", (chunk) => {
        received += chunk.length;
        if (total > 0) {
          const pct = Math.floor((received / total) * 100);
          if (pct !== lastPercent && pct % 5 === 0) {
            process.stdout.write(`\r[aphrodite] downloading… ${pct}%`);
            lastPercent = pct;
          }
        }
      });
      pipeline(res, fs.createWriteStream(dest)).then(() => {
        process.stdout.write("\n");
        resolve();
      }, reject);
    });
    req.on("error", reject);
  });
}

async function main() {
  // CI / dry-install bail-out — let GH Actions / Vercel skip the
  // network step if they want.
  if (process.env.APHRODITE_AGENT_SKIP_INSTALL === "1") {
    info("APHRODITE_AGENT_SKIP_INSTALL=1 set; skipping binary download.");
    return;
  }

  const key = `${PLATFORM}-${ARCH}`;
  const asset = TARGETS[key];
  if (!asset) {
    return unsupportedPlatform();
  }

  // Reservation publish (version 0.0.0) has no binary on releases —
  // print a friendly message rather than crash.
  if (VERSION === "0.0.0") {
    info("This is a name-reservation publish (version 0.0.0); no binary to download.");
    info("Wait for a real beta release: 1.0.0-beta.1 or later.");
    return;
  }

  const url = `https://github.com/${REPO}/releases/download/agent-v${VERSION}/${asset}`;
  fs.mkdirSync(BIN_DIR, { recursive: true });
  const archivePath = path.join(BIN_DIR, asset);

  info(`Downloading aphrodite v${VERSION} for ${key} (${asset})`);
  info(`  from ${url}`);

  try {
    await downloadWithProgress(url, archivePath);
  } catch (e) {
    exitWith(
      `Download failed: ${e.message}\n` +
      `If you're behind a corporate proxy or air-gapped, install from source:\n` +
      `  cargo install aphrodite-cli`,
    );
  }

  // Extract: tar.gz on unix, zip on windows.
  try {
    if (asset.endsWith(".tar.gz")) {
      execSync(`tar -xzf "${archivePath}" -C "${BIN_DIR}"`, { stdio: "inherit" });
    } else if (asset.endsWith(".zip")) {
      // PowerShell Expand-Archive on Windows
      execSync(
        `powershell -NoLogo -NoProfile -Command "Expand-Archive -LiteralPath '${archivePath}' -DestinationPath '${BIN_DIR}' -Force"`,
        { stdio: "inherit" },
      );
    }
  } catch (e) {
    exitWith(`Failed to extract archive: ${e.message}`);
  }

  // Make the extracted binary executable on unix.
  const binName = PLATFORM === "win32" ? "aphrodite.exe" : "aphrodite";
  const binPath = path.join(BIN_DIR, binName);
  if (!fs.existsSync(binPath)) {
    exitWith(`Expected binary at ${binPath} after extraction — archive layout mismatch.`);
  }
  if (PLATFORM !== "win32") {
    fs.chmodSync(binPath, 0o755);
  }

  // Clean up the archive
  fs.unlinkSync(archivePath);
  info(`Ready. Run \`aphrodite --version\` to verify.`);
}

main().catch((e) => exitWith(`postinstall failed: ${e.stack || e.message}`));
