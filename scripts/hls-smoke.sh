#!/usr/bin/env bash
# HLS product-path smoke (FR35 / FR50 / AD-25).
# Always fails on error (never ignore). Backend pin: PandA Bambu 2024.10.
#
# Modes:
#   default CI: uses scripts/fixtures/bambu-ci-stub.sh via BITLOOM_BAMBU_PATH
#               (wiring + synthesizable .v artifact; not synthesis-quality HLS)
#   real Bambu: BITLOOM_HLS_USE_REAL=1 and BITLOOM_BAMBU_PATH pointing at
#               bambu-2024.10 (AppImage or install)
#
# Cache strategy (NFR14): real AppImage may be cached under
#   ${BITLOOM_HLS_CACHE:-$HOME/.cache/bitloom-hls}/bambu-2024.10.AppImage
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${ROOT}/target/hls-smoke"
BACKEND_VERSION="2024.10"
STUB="${ROOT}/scripts/fixtures/bambu-ci-stub.sh"

echo "hls-smoke: pinned backend=bambu version=${BACKEND_VERSION}"
echo "hls-smoke: cache_root=${BITLOOM_HLS_CACHE:-$HOME/.cache/bitloom-hls}"

rm -rf "$OUT"
mkdir -p "$OUT"

# Always-on unit/CLI coverage (missing backend must fail readably).
(
  cd "$ROOT"
  cargo test -p bitloom --test hls_smoke -- --nocapture
)

if [[ "${BITLOOM_HLS_USE_REAL:-0}" == "1" ]]; then
  if [[ -z "${BITLOOM_BAMBU_PATH:-}" ]]; then
    CACHE_ROOT="${BITLOOM_HLS_CACHE:-$HOME/.cache/bitloom-hls}"
    APPIMAGE="${CACHE_ROOT}/bambu-${BACKEND_VERSION}.AppImage"
    if [[ ! -x "$APPIMAGE" ]]; then
      echo "error: BITLOOM_HLS_USE_REAL=1 but bambu not found; set BITLOOM_BAMBU_PATH or place AppImage at $APPIMAGE" >&2
      exit 1
    fi
    export BITLOOM_BAMBU_PATH="$APPIMAGE"
  fi
  echo "hls-smoke: using real bambu at $BITLOOM_BAMBU_PATH"
else
  chmod +x "$STUB"
  export BITLOOM_BAMBU_PATH="$STUB"
  echo "hls-smoke: using CI stub at $STUB (set BITLOOM_HLS_USE_REAL=1 for real Bambu ${BACKEND_VERSION})"
fi

(
  cd "$ROOT"
  cargo run -q -p bitloom -- hls --function add --out-dir "$OUT"
)

# Require synthesizable RTL artifact
shopt -s nullglob
rtl_files=("$OUT"/*.v "$OUT"/*.sv)
if (( ${#rtl_files[@]} == 0 )); then
  echo "error: no .v/.sv under $OUT after hls run" >&2
  ls -la "$OUT" >&2 || true
  exit 1
fi

echo "hls-smoke: OK backend=bambu version=${BACKEND_VERSION} out=$OUT"
ls -la "$OUT"
