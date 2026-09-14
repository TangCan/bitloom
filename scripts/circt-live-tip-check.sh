#!/usr/bin/env bash
# FR186 — unbounded / live CIRCT tip channel (beyond FR179 floating-track pin).
# Epic 119 / NFR14. Beyond FR179 document-pinned floating-track 1.159.0 alone.
#
# Selected shape (NFR14): **live tip** = resolve the latest GitHub `firtool-*`
# release tag (readable tip identity), cache under `firtool-live-tip`, channel ≠
# AD-9 product cache and ≠ FR179 `firtool-floating-head`. When the latest tip
# coincides with FR179/AD-9 version (today often 1.159.0), distinguish by **path**.
#
# Optional:
#   BITLOOM_FIRTOOL_LIVE_TIP_PATH=/dir/containing/firtool
#   BITLOOM_FIRTOOL_LIVE_TIP_VERSION=1.159.0   # skip API; pin resolved tip for offline/CI cache
#
# Honesty: ≠ product default pin · ≠ FR179 floating-track alone · ≠ PATH-random
# ≠ silent-Ok under BITLOOM_FIRTOOL_LIVE_TIP_FORCE_MISSING
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
AD9_PRODUCT_VERSION="1.159.0"
FR179_FLOATING_VERSION="1.159.0"
FR174_UNPAIRED_VERSION="1.156.0"
ASSET="firrtl-bin-linux-x64.tar.gz"
OUT_DIR="${BITLOOM_FIRTOOL_LIVE_TIP_OUT:-$ROOT/target/circt-live-tip-check}"
DOCS="${ROOT}/docs/fr186-unbounded-circt-tip.md"
FIR="${BITLOOM_FIRTOOL_LIVE_TIP_FIR:-$ROOT/crates/rhdl-firrtl/fixtures/fr170_chisel_head_parser.fir}"
API_URL="${BITLOOM_FIRTOOL_LIVE_TIP_API:-https://api.github.com/repos/llvm/circt/releases?per_page=30}"

echo "circt-live-tip-check: FR186 unbounded / live CIRCT tip gate (Bitloom)"
echo "circt-live-tip-check: channel=firtool-live-tip (≠ AD-9 product; ≠ FR179 floating-head)"

if [[ "${BITLOOM_FIRTOOL_LIVE_TIP_FORCE_MISSING:-0}" == "1" ]]; then
  echo "error: live tip firtool unavailable (BITLOOM_FIRTOOL_LIVE_TIP_FORCE_MISSING=1)" >&2
  echo "error: set BITLOOM_FIRTOOL_LIVE_TIP_PATH or allow live tip download; refusing silent success" >&2
  exit 1
fi

if [[ ! -f "$DOCS" ]]; then
  echo "error: missing FR186 product docs: $DOCS" >&2
  exit 1
fi
if ! grep -q "FR186" "$DOCS" || ! grep -qE "live tip|live-tip|无界" "$DOCS"; then
  echo "error: FR186 docs must describe unbounded / live tip channel" >&2
  exit 1
fi
if ! grep -q "FR179" "$DOCS" || ! grep -q "${FR179_FLOATING_VERSION}" "$DOCS"; then
  echo "error: FR186 docs must contrast FR179 floating-track ${FR179_FLOATING_VERSION}" >&2
  exit 1
fi
if ! grep -qE "live-tip|BITLOOM_FIRTOOL_LIVE_TIP_PATH|channel" "$DOCS"; then
  echo "error: FR186 docs must distinguish live-tip channel by path/cache" >&2
  exit 1
fi
if ! grep -qE "非产品默认|not product default|≠.*产品" "$DOCS"; then
  echo "error: FR186 docs must honesty-state tip is not the product default pin" >&2
  exit 1
fi

cache_root() {
  if [[ -n "${RHDL_FIRTOOL_CACHE:-}" ]]; then
    echo "${RHDL_FIRTOOL_CACHE}/live-tip"
    return
  fi
  if [[ -n "${XDG_CACHE_HOME:-}" ]]; then
    echo "${XDG_CACHE_HOME}/rhdl/firtool-live-tip"
    return
  fi
  echo "${HOME:-.}/.cache/rhdl/firtool-live-tip"
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

floating_cache_root() {
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

resolve_dir() {
  local d="$1"
  mkdir -p "$d"
  (cd "$d" && pwd -P)
}

resolve_latest_firtool_tag() {
  if [[ -n "${BITLOOM_FIRTOOL_LIVE_TIP_VERSION:-}" ]]; then
    echo "${BITLOOM_FIRTOOL_LIVE_TIP_VERSION#firtool-}"
    return 0
  fi
  local json tag
  json="$(curl -fsSL "$API_URL")"
  tag="$(
    python3 -c '
import json,sys,re
releases=json.load(sys.stdin)
pat=re.compile(r"^firtool-(\d+\.\d+\.\d+)$")
for r in releases:
  m=pat.match(r.get("tag_name") or "")
  if m:
    print(m.group(1))
    sys.exit(0)
sys.exit("no firtool-* release tag found")
' <<<"$json"
  )"
  echo "$tag"
}

ensure_live_tip_firtool() {
  if [[ -n "${BITLOOM_FIRTOOL_LIVE_TIP_PATH:-}" ]]; then
    local bin="${BITLOOM_FIRTOOL_LIVE_TIP_PATH%/}/firtool"
    if [[ ! -x "$bin" && ! -f "$bin" ]]; then
      echo "error: BITLOOM_FIRTOOL_LIVE_TIP_PATH=${BITLOOM_FIRTOOL_LIVE_TIP_PATH} does not contain firtool" >&2
      exit 1
    fi
    local ad9_root float_root bin_dir
    ad9_root="$(resolve_dir "$(ad9_product_cache_root)")"
    float_root="$(resolve_dir "$(floating_cache_root)")"
    bin_dir="$(cd "$(dirname "$bin")" && pwd -P)"
    case "$bin_dir" in
      "${ad9_root}"|"${ad9_root}"/*)
        echo "error: LIVE_TIP_PATH under AD-9 product cache (${ad9_root}); FR186 requires live-tip channel" >&2
        exit 1
        ;;
      "${float_root}"|"${float_root}"/*)
        echo "error: LIVE_TIP_PATH under FR179 floating-head cache (${float_root}); FR186 requires live-tip channel" >&2
        exit 1
        ;;
    esac
    echo "$bin"
    return 0
  fi

  local tip_ver ver_dir bin tarball sha_file release_base
  tip_ver="$(resolve_latest_firtool_tag)"
  echo "circt-live-tip-check: resolved tip identity firtool-${tip_ver}" >&2
  ver_dir="$(cache_root)/${tip_ver}"
  bin="${ver_dir}/bin/firtool"
  if [[ -f "$bin" ]]; then
    echo "$bin"
    return 0
  fi

  mkdir -p "$ver_dir"
  tarball="${ver_dir}/${ASSET}"
  sha_file="${tarball}.sha256"
  release_base="https://github.com/llvm/circt/releases/download/firtool-${tip_ver}"
  echo "circt-live-tip-check: downloading ${release_base}/${ASSET}" >&2
  curl -fsSL -o "$tarball" "${release_base}/${ASSET}"
  curl -fsSL -o "$sha_file" "${release_base}/${ASSET}.sha256"
  local expected actual
  expected="$(awk '{print $1; exit}' "$sha_file")"
  actual="$(sha256sum "$tarball" | awk '{print $1}')"
  if [[ "$actual" != "$expected" ]]; then
    rm -f "$tarball"
    echo "error: sha256 mismatch for live-tip firtool-${tip_ver}" >&2
    exit 1
  fi
  tar -xzf "$tarball" -C "$ver_dir"
  mkdir -p "${ver_dir}/bin"
  if [[ ! -f "$bin" ]]; then
    local found
    found="$(find "$ver_dir" -type f -name firtool | head -n 1 || true)"
    if [[ -z "$found" ]]; then
      echo "error: extracted live-tip firtool archive has no firtool binary" >&2
      exit 1
    fi
    cp "$found" "$bin"
    chmod +x "$bin"
  fi
  echo "$bin"
}

FIRTOOL_BIN="$(ensure_live_tip_firtool)"
echo "circt-live-tip-check: firtool=${FIRTOOL_BIN}"

AD9_ROOT="$(resolve_dir "$(ad9_product_cache_root)")"
FLOAT_ROOT="$(resolve_dir "$(floating_cache_root)")"
TIP_ROOT="$(resolve_dir "$(cache_root)")"
BIN_DIR="$(cd "$(dirname "$FIRTOOL_BIN")" && pwd -P)"
case "$BIN_DIR" in
  "${TIP_ROOT}"|"${TIP_ROOT}"/*) ;;
  *)
    if [[ -z "${BITLOOM_FIRTOOL_LIVE_TIP_PATH:-}" ]]; then
      echo "error: live-tip firtool not under live-tip cache (${TIP_ROOT}); got ${BIN_DIR}" >&2
      exit 1
    fi
    ;;
esac
case "$BIN_DIR" in
  "${AD9_ROOT}"|"${AD9_ROOT}"/*)
    echo "error: firtool under AD-9 product cache (${AD9_ROOT}); FR186 requires live-tip channel" >&2
    exit 1
    ;;
  "${FLOAT_ROOT}"|"${FLOAT_ROOT}"/*)
    echo "error: firtool under FR179 floating-head cache (${FLOAT_ROOT}); FR186 requires live-tip channel" >&2
    exit 1
    ;;
esac

VER_OUT="$("$FIRTOOL_BIN" --version 2>&1 || true)"
TIP_ID="$(echo "$VER_OUT" | head -n 1)"
echo "circt-live-tip-check: tip-identity=${TIP_ID}"
if [[ -z "$TIP_ID" ]]; then
  echo "error: live tip firtool produced empty --version identity" >&2
  exit 1
fi
if grep -q "${FR174_UNPAIRED_VERSION}" <<<"$VER_OUT"; then
  echo "error: firtool reports FR174 unpaired pin ${FR174_UNPAIRED_VERSION}; FR186 tip must be beyond FR174 alone" >&2
  exit 1
fi

if [[ ! -f "$FIR" ]]; then
  echo "error: missing FIR fixture: $FIR" >&2
  exit 1
fi

mkdir -p "$OUT_DIR"
STAMP="$OUT_DIR/fr186_live_tip.parse-ok"
echo "circt-live-tip-check: firtool -parse-only on $FIR"
if ! "$FIRTOOL_BIN" -parse-only "$FIR" >"$OUT_DIR/fr186_live_tip.parse.log" 2>&1; then
  echo "error: live-tip firtool -parse-only failed for $FIR" >&2
  cat "$OUT_DIR/fr186_live_tip.parse.log" >&2 || true
  exit 1
fi
{
  echo "FR186 unbounded / live CIRCT tip OK"
  echo "tip_identity=${TIP_ID}"
  echo "ad9_product=${AD9_PRODUCT_VERSION}"
  echo "fr179_floating=${FR179_FLOATING_VERSION}"
  echo "fr174_unpaired=${FR174_UNPAIRED_VERSION}"
  echo "channel=firtool-live-tip"
  echo "not_product_default=true"
  echo "fixture=$FIR"
} >"$STAMP"

echo "circt-live-tip-check: OK stamp=$STAMP"
