#!/usr/bin/env bash
# FR179 — floating CIRCT git HEAD (reproducible floating-track pin beyond FR174).
# Epic 112 / NFR14. Beyond FR174 document-pinned unpaired 1.156.0 / FR173 paired AD-9 alone.
#
# Selected shape (NFR14): document-pinned **floating-track** firtool-**1.159.0**
# (≠ AD-9 product *channel*; ≠ FR174 unpaired 1.156.0). After FR182, AD-9 product pin
# is also **1.159.0** — channels are distinguished by **cache/path**
# (`firtool-floating-head` / `BITLOOM_FIRTOOL_FLOATING_HEAD_PATH`), not version inequality.
# Optional override:
#   BITLOOM_FIRTOOL_FLOATING_HEAD_PATH=/dir/containing/firtool
# Must report EXPECTED_FLOATING_VERSION and must use the floating-head channel
# (not the default AD-9 product cache path).
#
# Honesty: this is a reproducible floating-track pin (readable version), not PATH-random
# and not FR186 live tip (see docs/fr186-unbounded-circt-tip.md; FR179 alone ≠ FR186).
#
# ≠ FR174 alone · ≠ FR173 alone · ≠ FR182 alone
# ≠ PATH-random firtool · ≠ silent-Ok under BITLOOM_FIRTOOL_FLOATING_HEAD_FORCE_MISSING
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
AD9_PRODUCT_VERSION="1.159.0"
FR174_UNPAIRED_VERSION="1.156.0"
EXPECTED_FLOATING_VERSION="1.159.0"
ASSET="firrtl-bin-linux-x64.tar.gz"
RELEASE_BASE="https://github.com/llvm/circt/releases/download/firtool-${EXPECTED_FLOATING_VERSION}"
OUT_DIR="${BITLOOM_FIRTOOL_FLOATING_HEAD_OUT:-$ROOT/target/circt-floating-git-head-check}"
DOCS="${ROOT}/docs/fr179-floating-circt-git-head.md"
FIR="${BITLOOM_FIRTOOL_FLOATING_HEAD_FIR:-$ROOT/crates/rhdl-firrtl/fixtures/fr170_chisel_head_parser.fir}"

echo "circt-floating-git-head-check: FR179 floating CIRCT HEAD track gate (Bitloom)"
echo "circt-floating-git-head-check: floating-track pin firtool-${EXPECTED_FLOATING_VERSION} (channel ≠ AD-9 product; ≠ FR174 ${FR174_UNPAIRED_VERSION})"

if [[ "${BITLOOM_FIRTOOL_FLOATING_HEAD_FORCE_MISSING:-0}" == "1" ]]; then
  echo "error: floating HEAD track firtool unavailable (BITLOOM_FIRTOOL_FLOATING_HEAD_FORCE_MISSING=1)" >&2
  echo "error: set BITLOOM_FIRTOOL_FLOATING_HEAD_PATH or allow download of firtool-${EXPECTED_FLOATING_VERSION}; refusing silent success" >&2
  exit 1
fi

if [[ ! -f "$DOCS" ]]; then
  echo "error: missing FR179 product docs: $DOCS" >&2
  exit 1
fi
if ! grep -q "FR179" "$DOCS" || ! grep -q "${EXPECTED_FLOATING_VERSION}" "$DOCS"; then
  echo "error: FR179 docs must pin floating-track firtool-${EXPECTED_FLOATING_VERSION}" >&2
  exit 1
fi
if ! grep -q "${AD9_PRODUCT_VERSION}" "$DOCS" || ! grep -q "${FR174_UNPAIRED_VERSION}" "$DOCS"; then
  echo "error: FR179 docs must contrast AD-9 ${AD9_PRODUCT_VERSION} and FR174 ${FR174_UNPAIRED_VERSION}" >&2
  exit 1
fi
if ! grep -qE "floating-head|BITLOOM_FIRTOOL_FLOATING_HEAD_PATH|channel" "$DOCS"; then
  echo "error: FR179 docs must distinguish floating-track channel by path/cache (not version alone when coinciding with AD-9)" >&2
  exit 1
fi

cache_root() {
  if [[ -n "${RHDL_FIRTOOL_CACHE:-}" ]]; then
    echo "${RHDL_FIRTOOL_CACHE}/floating-head"
    return
  fi
  if [[ -n "${XDG_CACHE_HOME:-}" ]]; then
    echo "${XDG_CACHE_HOME}/rhdl/firtool-floating-head"
    return
  fi
  echo "${HOME:-.}/.cache/rhdl/firtool-floating-head"
}

ad9_product_cache_root() {
  if [[ -n "${RHDL_FIRTOOL_CACHE:-}" ]]; then
    echo "${RHDL_FIRTOOL_CACHE}"
    return
  fi
  if [[ -n "${XDG_CACHE_HOME:-}" ]]; then
    echo "${XDG_CACHE_HOME}/rhdl/firtool"
    return
  fi
  echo "${HOME:-.}/.cache/rhdl/firtool"
}

# Resolve with pwd -P so HOME→/nvme_* symlink farms compare equal.
resolve_dir() {
  local d="$1"
  mkdir -p "$d"
  (cd "$d" && pwd -P)
}

ensure_floating_firtool() {
  if [[ -n "${BITLOOM_FIRTOOL_FLOATING_HEAD_PATH:-}" ]]; then
    local bin="${BITLOOM_FIRTOOL_FLOATING_HEAD_PATH%/}/firtool"
    if [[ ! -x "$bin" && ! -f "$bin" ]]; then
      echo "error: BITLOOM_FIRTOOL_FLOATING_HEAD_PATH=${BITLOOM_FIRTOOL_FLOATING_HEAD_PATH} does not contain firtool" >&2
      exit 1
    fi
    # Channel honesty: override path must not be the default AD-9 product cache tree.
    local ad9_root bin_dir
    ad9_root="$(resolve_dir "$(ad9_product_cache_root)")"
    bin_dir="$(cd "$(dirname "$bin")" && pwd -P)"
    case "$bin_dir" in
      "${ad9_root}"|"${ad9_root}"/*)
        echo "error: BITLOOM_FIRTOOL_FLOATING_HEAD_PATH resolves under AD-9 product cache (${ad9_root}); FR179 requires floating-head channel" >&2
        exit 1
        ;;
    esac
    echo "$bin"
    return 0
  fi

  local ver_dir bin tarball sha_file
  ver_dir="$(cache_root)/${EXPECTED_FLOATING_VERSION}"
  bin="${ver_dir}/bin/firtool"
  if [[ -f "$bin" ]]; then
    echo "$bin"
    return 0
  fi

  mkdir -p "$ver_dir"
  tarball="${ver_dir}/${ASSET}"
  sha_file="${tarball}.sha256"
  echo "circt-floating-git-head-check: downloading ${RELEASE_BASE}/${ASSET}" >&2
  curl -fsSL -o "$tarball" "${RELEASE_BASE}/${ASSET}"
  curl -fsSL -o "$sha_file" "${RELEASE_BASE}/${ASSET}.sha256"
  local expected actual
  expected="$(awk '{print $1; exit}' "$sha_file")"
  actual="$(sha256sum "$tarball" | awk '{print $1}')"
  if [[ "$actual" != "$expected" ]]; then
    rm -f "$tarball"
    echo "error: sha256 mismatch for floating-track firtool-${EXPECTED_FLOATING_VERSION}" >&2
    exit 1
  fi
  tar -xzf "$tarball" -C "$ver_dir"
  mkdir -p "${ver_dir}/bin"
  if [[ ! -f "$bin" ]]; then
    local found
    found="$(find "$ver_dir" -type f -name firtool | head -n 1 || true)"
    if [[ -z "$found" ]]; then
      echo "error: extracted floating-track firtool archive has no firtool binary" >&2
      exit 1
    fi
    cp "$found" "$bin"
    chmod +x "$bin"
  fi
  echo "$bin"
}

FIRTOOL_BIN="$(ensure_floating_firtool)"
echo "circt-floating-git-head-check: firtool=${FIRTOOL_BIN}"

# Channel check: binary must come from floating-head cache or FLOATING_HEAD_PATH (not AD-9 product cache).
AD9_ROOT="$(resolve_dir "$(ad9_product_cache_root)")"
FLOAT_ROOT="$(resolve_dir "$(cache_root)")"
BIN_DIR="$(cd "$(dirname "$FIRTOOL_BIN")" && pwd -P)"
case "$BIN_DIR" in
  "${FLOAT_ROOT}"|"${FLOAT_ROOT}"/*) ;;
  *)
    if [[ -z "${BITLOOM_FIRTOOL_FLOATING_HEAD_PATH:-}" ]]; then
      echo "error: floating-track firtool not under floating-head cache (${FLOAT_ROOT}); got ${BIN_DIR}" >&2
      exit 1
    fi
    ;;
esac
case "$BIN_DIR" in
  "${AD9_ROOT}"|"${AD9_ROOT}"/*)
    echo "error: firtool resolves under AD-9 product cache (${AD9_ROOT}); FR179 requires floating-head channel (path ≠ product cache even when version coincides)" >&2
    exit 1
    ;;
esac

VER_OUT="$("$FIRTOOL_BIN" --version 2>&1 || true)"
echo "circt-floating-git-head-check: version-line=$(echo "$VER_OUT" | head -n 1)"
if ! grep -q "${EXPECTED_FLOATING_VERSION}" <<<"$VER_OUT"; then
  echo "error: floating-track firtool version mismatch: expected ${EXPECTED_FLOATING_VERSION}, got: ${VER_OUT}" >&2
  echo "error: FR179 requires firtool-${EXPECTED_FLOATING_VERSION} (floating-head channel ≠ AD-9 product channel; ≠ FR174 ${FR174_UNPAIRED_VERSION}); refusing silent success" >&2
  exit 1
fi
# When AD-9 product pin coincides with floating-track version (FR182), do **not** reject on
# version equality — channels are distinguished by path/cache above.
if grep -q "${FR174_UNPAIRED_VERSION}" <<<"$VER_OUT"; then
  echo "error: firtool reports FR174 unpaired pin ${FR174_UNPAIRED_VERSION}; FR179 requires beyond FR174" >&2
  exit 1
fi

if [[ ! -f "$FIR" ]]; then
  echo "error: missing FIR fixture: $FIR" >&2
  exit 1
fi

mkdir -p "$OUT_DIR"
STAMP="$OUT_DIR/fr179_floating_git_head.parse-ok"
echo "circt-floating-git-head-check: firtool -parse-only on $FIR"
if ! "$FIRTOOL_BIN" -parse-only "$FIR" >"$OUT_DIR/fr179_floating_git_head.parse.log" 2>&1; then
  echo "error: floating-track firtool -parse-only failed for $FIR" >&2
  cat "$OUT_DIR/fr179_floating_git_head.parse.log" >&2 || true
  exit 1
fi
{
  echo "FR179 floating CIRCT HEAD track OK"
  echo "floating_track_pin=firtool-${EXPECTED_FLOATING_VERSION}"
  echo "ad9_product=${AD9_PRODUCT_VERSION}"
  echo "fr174_unpaired=${FR174_UNPAIRED_VERSION}"
  echo "channel=firtool-floating-head"
  echo "fixture=$FIR"
} >"$STAMP"

echo "circt-floating-git-head-check: OK stamp=$STAMP"
