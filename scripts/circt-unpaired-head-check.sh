#!/usr/bin/env bash
# FR174 — unpaired CIRCT/firtool HEAD (or document-pinned unpaired mainline) product path.
# Epic 107 / NFR14. Beyond FR170 update-mainline alone / FR173 paired AD-9 pin alone.
#
# Selected shape (NFR14): document-pinned **unpaired mainline** firtool-**1.156.0**
# (≠ AD-9 product pin firtool-1.158.0 ↔ Chisel 7.15.0). Optional override:
#   BITLOOM_FIRTOOL_HEAD_PATH=/dir/containing/firtool
# Must report version matching EXPECTED_HEAD_VERSION and must **not** equal AD-9 pin.
#
# ≠ FR170 alone · ≠ FR138 alone · ≠ FR165 alone · ≠ FR173 alone
# ≠ PATH-random firtool · ≠ silent-Ok under BITLOOM_FIRTOOL_HEAD_FORCE_MISSING
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
AD9_PRODUCT_VERSION="1.158.0"
EXPECTED_HEAD_VERSION="1.156.0"
ASSET="firrtl-bin-linux-x64.tar.gz"
RELEASE_BASE="https://github.com/llvm/circt/releases/download/firtool-${EXPECTED_HEAD_VERSION}"
OUT_DIR="${BITLOOM_FIRTOOL_HEAD_OUT:-$ROOT/target/circt-unpaired-head-check}"
DOCS="${ROOT}/docs/fr174-unpaired-head.md"
FIR="${BITLOOM_FIRTOOL_HEAD_FIR:-$ROOT/crates/rhdl-firrtl/fixtures/fr170_chisel_head_parser.fir}"

echo "circt-unpaired-head-check: FR174 unpaired firtool mainline gate (Bitloom)"
echo "circt-unpaired-head-check: document pin firtool-${EXPECTED_HEAD_VERSION} (≠ AD-9 product ${AD9_PRODUCT_VERSION})"

if [[ "${BITLOOM_FIRTOOL_HEAD_FORCE_MISSING:-0}" == "1" ]]; then
  echo "error: unpaired HEAD / document-pinned mainline firtool unavailable (BITLOOM_FIRTOOL_HEAD_FORCE_MISSING=1)" >&2
  echo "error: set BITLOOM_FIRTOOL_HEAD_PATH or allow download of firtool-${EXPECTED_HEAD_VERSION}; refusing silent success" >&2
  exit 1
fi

if [[ ! -f "$DOCS" ]]; then
  echo "error: missing FR174 product docs: $DOCS" >&2
  exit 1
fi
if ! grep -q "FR174" "$DOCS" || ! grep -q "${EXPECTED_HEAD_VERSION}" "$DOCS"; then
  echo "error: FR174 docs must pin unpaired firtool-${EXPECTED_HEAD_VERSION}" >&2
  exit 1
fi
if ! grep -q "${AD9_PRODUCT_VERSION}" "$DOCS"; then
  echo "error: FR174 docs must contrast AD-9 product pin ${AD9_PRODUCT_VERSION}" >&2
  exit 1
fi

cache_root() {
  if [[ -n "${RHDL_FIRTOOL_CACHE:-}" ]]; then
    echo "${RHDL_FIRTOOL_CACHE}/head"
    return
  fi
  if [[ -n "${XDG_CACHE_HOME:-}" ]]; then
    echo "${XDG_CACHE_HOME}/rhdl/firtool-head"
    return
  fi
  echo "${HOME:-.}/.cache/rhdl/firtool-head"
}

ensure_head_firtool() {
  if [[ -n "${BITLOOM_FIRTOOL_HEAD_PATH:-}" ]]; then
    local bin="${BITLOOM_FIRTOOL_HEAD_PATH%/}/firtool"
    if [[ ! -x "$bin" && ! -f "$bin" ]]; then
      echo "error: BITLOOM_FIRTOOL_HEAD_PATH=${BITLOOM_FIRTOOL_HEAD_PATH} does not contain firtool" >&2
      exit 1
    fi
    echo "$bin"
    return 0
  fi

  local ver_dir bin tarball sha_file
  ver_dir="$(cache_root)/${EXPECTED_HEAD_VERSION}"
  bin="${ver_dir}/bin/firtool"
  if [[ -f "$bin" ]]; then
    echo "$bin"
    return 0
  fi

  mkdir -p "$ver_dir"
  tarball="${ver_dir}/${ASSET}"
  sha_file="${tarball}.sha256"
  echo "circt-unpaired-head-check: downloading ${RELEASE_BASE}/${ASSET}" >&2
  curl -fsSL -o "$tarball" "${RELEASE_BASE}/${ASSET}"
  curl -fsSL -o "$sha_file" "${RELEASE_BASE}/${ASSET}.sha256"
  local expected actual
  expected="$(awk '{print $1; exit}' "$sha_file")"
  actual="$(sha256sum "$tarball" | awk '{print $1}')"
  if [[ "$actual" != "$expected" ]]; then
    rm -f "$tarball"
    echo "error: sha256 mismatch for unpaired firtool-${EXPECTED_HEAD_VERSION}" >&2
    exit 1
  fi
  tar -xzf "$tarball" -C "$ver_dir"
  mkdir -p "${ver_dir}/bin"
  if [[ ! -f "$bin" ]]; then
    local found
    found="$(find "$ver_dir" -type f -name firtool | head -n 1 || true)"
    if [[ -z "$found" ]]; then
      echo "error: extracted unpaired firtool archive has no firtool binary" >&2
      exit 1
    fi
    cp "$found" "$bin"
    chmod +x "$bin"
  fi
  echo "$bin"
}

FIRTOOL_BIN="$(ensure_head_firtool)"
echo "circt-unpaired-head-check: firtool=${FIRTOOL_BIN}"

VER_OUT="$("$FIRTOOL_BIN" --version 2>&1 || true)"
echo "circt-unpaired-head-check: version-line=$(echo "$VER_OUT" | head -n 1)"
if ! grep -q "${EXPECTED_HEAD_VERSION}" <<<"$VER_OUT"; then
  echo "error: unpaired firtool version mismatch: expected ${EXPECTED_HEAD_VERSION}, got: ${VER_OUT}" >&2
  echo "error: FR174 requires document-pinned firtool-${EXPECTED_HEAD_VERSION} (≠ AD-9 ${AD9_PRODUCT_VERSION}; ≠ PATH random); refusing silent success" >&2
  exit 1
fi
if grep -q "${AD9_PRODUCT_VERSION}" <<<"$VER_OUT"; then
  echo "error: firtool reports AD-9 product pin ${AD9_PRODUCT_VERSION}; FR174 requires unpaired ≠ product pin" >&2
  exit 1
fi

if [[ ! -f "$FIR" ]]; then
  echo "error: missing FIR fixture: $FIR" >&2
  exit 1
fi

mkdir -p "$OUT_DIR"
STAMP="$OUT_DIR/fr174_unpaired_head.parse-ok"
echo "circt-unpaired-head-check: firtool -parse-only on $FIR"
if ! "$FIRTOOL_BIN" -parse-only "$FIR" >"$OUT_DIR/fr174_unpaired_head.parse.log" 2>&1; then
  echo "error: unpaired firtool -parse-only failed for $FIR" >&2
  cat "$OUT_DIR/fr174_unpaired_head.parse.log" >&2 || true
  exit 1
fi
{
  echo "FR174 unpaired firtool mainline OK"
  echo "unpaired pin=firtool-${EXPECTED_HEAD_VERSION}"
  echo "ad9_product=${AD9_PRODUCT_VERSION}"
  echo "fixture=$FIR"
} >"$STAMP"

echo "circt-unpaired-head-check: OK stamp=$STAMP"
