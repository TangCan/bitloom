#!/usr/bin/env bash
# FR137 — external CIRCT compile gate (Bitloom).
# Epic 76 / NFR14 E1–E4. Selected MVP = compile gate (sim deferred; see docs/fr137-external-circt-gate.md).
#
# ≠ FR129 C1–C4 alone (in-tree Handshake markers).
# ≠ FR121 / FR110 / FR95 / FR96 alone.
# ≠ scripts/firtool-smoke.sh (that path silent-skips when firtool is missing).
# ≠ FR127 formal-sby (SymbiYosys).
#
# Resolve firtool via AD-9: RHDL_FIRTOOL_PATH or `cargo run -p bitloom -- firtool ensure`.
# Never treat bare PATH `firtool` as the success resolver.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EXPECTED_VERSION="1.158.0"
FIR="${BITLOOM_CIRCT_FIR:-$ROOT/crates/rhdl-firrtl/fixtures/fr137_external_circt_gate.fir}"
OUT_DIR="${BITLOOM_CIRCT_OUT:-$ROOT/target/circt-external-check}"

echo "circt-external-check: FR137 external CIRCT compile gate (Bitloom; ≠ FR129/121/110/95/96 alone)"

if [[ "${BITLOOM_CIRCT_FORCE_MISSING:-0}" == "1" ]]; then
  echo "error: external CIRCT firtool unavailable (BITLOOM_CIRCT_FORCE_MISSING=1)" >&2
  echo "error: install/ensure firtool-${EXPECTED_VERSION} via AD-9 (cargo bitloom firtool ensure) or RHDL_FIRTOOL_PATH; refusing silent success" >&2
  exit 1
fi

resolve_firtool() {
  if [[ -n "${RHDL_FIRTOOL_PATH:-}" ]]; then
    local bin="${RHDL_FIRTOOL_PATH%/}/firtool"
    if [[ ! -x "$bin" && ! -f "$bin" ]]; then
      echo "error: RHDL_FIRTOOL_PATH=${RHDL_FIRTOOL_PATH} does not contain a firtool binary" >&2
      echo "error: refusing silent success (FR137 E3)" >&2
      exit 1
    fi
    echo "$bin"
    return 0
  fi
  # AD-9 managed cache / download — never `command -v firtool` as the primary path.
  local ensured
  if ! ensured="$(cargo run -q -p bitloom -- firtool ensure 2>/dev/null)"; then
    echo "error: firtool ensure failed (AD-9 / firtool-${EXPECTED_VERSION})" >&2
    echo "error: set RHDL_FIRTOOL_PATH or fix network/cache; refusing silent success" >&2
    exit 1
  fi
  ensured="$(echo "$ensured" | tail -n 1 | tr -d '\r')"
  if [[ -z "$ensured" || ! -f "$ensured" ]]; then
    echo "error: firtool ensure did not return a usable binary path" >&2
    exit 1
  fi
  echo "$ensured"
}

FIRTOOL_BIN="$(resolve_firtool)"
echo "circt-external-check: firtool=${FIRTOOL_BIN}"

VER_OUT="$("$FIRTOOL_BIN" --version 2>&1 || true)"
echo "circt-external-check: version-line=$(echo "$VER_OUT" | head -n 1)"
if ! grep -q "${EXPECTED_VERSION}" <<<"$VER_OUT"; then
  echo "error: firtool version mismatch: expected ${EXPECTED_VERSION}, got: ${VER_OUT}" >&2
  echo "error: AD-9 / FR137 require firtool-${EXPECTED_VERSION} (≠ CIRCT HEAD; ≠ PATH random); refusing silent success" >&2
  exit 1
fi

if [[ ! -f "$FIR" ]]; then
  echo "error: missing FIR fixture: $FIR" >&2
  exit 1
fi

mkdir -p "$OUT_DIR"
OUT_V="$OUT_DIR/fr137_external_circt_gate.firtool.v"
echo "circt-external-check: compiling $FIR -> $OUT_V"
if ! "$FIRTOOL_BIN" "$FIR" -o="$OUT_V"; then
  echo "error: firtool compile failed for $FIR" >&2
  exit 1
fi
if [[ ! -f "$OUT_V" ]]; then
  echo "error: firtool did not write expected output $OUT_V" >&2
  exit 1
fi

echo "circt-external-check: OK compile-gate firtool-${EXPECTED_VERSION} fixture=$FIR out=$OUT_V"
