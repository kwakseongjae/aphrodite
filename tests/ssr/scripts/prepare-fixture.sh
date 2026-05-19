#!/usr/bin/env bash
# Regenerate the canonical Banchan @aphrodite-design/example-banchan
# package at tests/ssr/fixtures/banchan-react/. The Next.js / Vite /
# Remix apps reference this directory via `file:` in their package.json.
#
# Idempotent. Safe to re-run after a `react_export.rs` change.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
FIXTURE_ROOT="$REPO_ROOT/tests/ssr/fixtures"
BANCHAN_DIR="$FIXTURE_ROOT/banchan-source"

# Intent for the canonical fixture. Pinned to the same Pass-54
# Banchan brief used in cross-brand alpha sweeps.
INTENT="한국 프리미엄 가정식 반찬 정기 구독 서비스 'Banchan' — 매주 신선한 제철 재료로 만든 5가지 반찬을 집까지 배송. 1주/2주/4주 정기권. 첫 박스 50% 할인."

mkdir -p "$BANCHAN_DIR"

if [[ ! -f "$BANCHAN_DIR/DESIGN.md" ]]; then
  echo "[fixture] Running aphrodite create — this will take ~10 minutes…"
  cd "$BANCHAN_DIR"
  aphrodite create "$INTENT" --persona dieter-rams
else
  echo "[fixture] Reusing existing DESIGN.md — re-emitting react/ only"
  cd "$BANCHAN_DIR"
  aphrodite react
fi

# Move the emitted react/ to the canonical fixture path used by
# every test app's package.json.
rm -rf "$FIXTURE_ROOT/banchan-react"
mv "$BANCHAN_DIR/react" "$FIXTURE_ROOT/banchan-react"

echo "[fixture] Ready at: $FIXTURE_ROOT/banchan-react"
echo "[fixture] Components: $(ls $FIXTURE_ROOT/banchan-react/src/*.tsx | grep -v stories | wc -l | xargs)"
