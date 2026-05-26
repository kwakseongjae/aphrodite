#!/usr/bin/env bash
#
# Aphrodite emit smoke — runs the beta-tester acceptance pipeline against a
# generated React component package (roadmap beta criteria 3-6):
#
#   3. npm install      (no peer-dep conflicts, no missing-binary errors)
#   4. npm run build     (tsup ESM + CJS + DTS)
#   5. npm run typecheck (tsc --noEmit)
#   6. npm test          (Vitest + JSDOM smoke)
#
# This is the script a beta tester (or you) runs to prove an emit is healthy.
# It does NOT need an API key when pointed at an existing emit or DESIGN.md;
# only `--create` calls the LLM.
#
# Usage:
#   scripts/emit-smoke.sh <react-package-dir>
#       Validate an already-emitted package (a dir containing package.json).
#
#   scripts/emit-smoke.sh --react <repo-dir>
#       Run `aphrodite react` on <repo-dir> (must contain DESIGN.md, offline,
#       no key), then validate the resulting <repo-dir>/react/.
#
#   scripts/emit-smoke.sh --create "<intent>" [work-dir]
#       Run `aphrodite create` (NEEDS an API key) into a fresh work-dir
#       (default: a mktemp dir), then validate work-dir/react/.
#
# Binary resolution: $APHRODITE_BIN, else `aphrodite` on PATH, else
# ./target/release/aphrodite.
set -euo pipefail

bold() { printf '\033[1m%s\033[0m\n' "$1"; }
green() { printf '\033[32m%s\033[0m\n' "$1"; }
red() { printf '\033[31m%s\033[0m\n' "$1"; }

resolve_bin() {
  if [[ -n "${APHRODITE_BIN:-}" ]]; then echo "$APHRODITE_BIN"; return; fi
  if command -v aphrodite >/dev/null 2>&1; then echo "aphrodite"; return; fi
  if [[ -x "./target/release/aphrodite" ]]; then echo "./target/release/aphrodite"; return; fi
  red "aphrodite binary not found. Set APHRODITE_BIN, put it on PATH, or build it."
  exit 127
}

PKG_DIR=""

case "${1:-}" in
  --react)
    repo="${2:?--react needs a repo dir containing DESIGN.md}"
    [[ -f "$repo/DESIGN.md" ]] || { red "no DESIGN.md in $repo"; exit 1; }
    BIN="$(resolve_bin)"
    bold "▶ aphrodite react --repo $repo (offline emit)"
    "$BIN" react --repo "$repo"
    PKG_DIR="$repo/react"
    ;;
  --create)
    intent="${2:?--create needs an intent string}"
    work="${3:-$(mktemp -d)}"
    mkdir -p "$work"
    BIN="$(resolve_bin)"
    bold "▶ aphrodite create (needs API key) → $work"
    ( cd "$work" && git init -q . && "$BIN" create "$intent" )
    PKG_DIR="$work/react"
    ;;
  "" | -h | --help)
    sed -n '2,30p' "$0" | sed 's/^# \{0,1\}//'
    exit 0
    ;;
  *)
    PKG_DIR="$1"
    ;;
esac

[[ -f "$PKG_DIR/package.json" ]] || { red "no package.json in $PKG_DIR — not an emit"; exit 1; }
bold "▶ validating emit at $PKG_DIR"
cd "$PKG_DIR"

run_step() {
  local label="$1"; shift
  bold "── $label"
  if "$@"; then green "   ✓ $label"; else red "   ✗ $label FAILED"; exit 1; fi
}

run_step "3. npm install" npm install --no-audit --no-fund
run_step "4. npm run build" npm run build
run_step "5. npm run typecheck" npm run typecheck
run_step "6. npm test" npm test

echo
green "✓ emit smoke PASSED — install + build + typecheck + test all green ($PKG_DIR)"
