#!/usr/bin/env bash
# FR164 — external CIRCT simulation gate (Bitloom).
# Epic 96 / NFR14. Beyond FR137 compile-only MVP (see docs/fr164-circt-external-sim-gate.md).
#
# ≠ FR137 compile alone.
# ≠ FR129 C1–C4 alone (in-tree Handshake markers).
# ≠ scripts/firtool-smoke.sh (silent-skip when missing).
#
# Resolve firtool via AD-9: RHDL_FIRTOOL_PATH or `cargo run -p bitloom -- firtool ensure`.
# Never treat bare PATH `firtool` as the success resolver.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EXPECTED_VERSION="1.155.0"
FIR="${BITLOOM_CIRCT_SIM_FIR:-$ROOT/crates/rhdl-firrtl/fixtures/fr137_external_circt_gate.fir}"
SIM_FIR="${BITLOOM_CIRCT_SIM_EXEC_FIR:-$ROOT/crates/rhdl-firrtl/fixtures/fr164_external_circt_sim_gate.fir}"
OUT_DIR="${BITLOOM_CIRCT_SIM_OUT:-$ROOT/target/circt-external-sim-check}"

echo "circt-external-sim-check: FR164 external CIRCT sim gate (Bitloom; ≠ FR137 compile alone)"

if [[ "${BITLOOM_CIRCT_SIM_FORCE_MISSING:-0}" == "1" ]]; then
  echo "error: external CIRCT firtool unavailable (BITLOOM_CIRCT_SIM_FORCE_MISSING=1)" >&2
  echo "error: install/ensure firtool-${EXPECTED_VERSION} via AD-9 (cargo bitloom firtool ensure) or RHDL_FIRTOOL_PATH; refusing silent success" >&2
  exit 1
fi

resolve_firtool() {
  if [[ -n "${RHDL_FIRTOOL_PATH:-}" ]]; then
    local bin="${RHDL_FIRTOOL_PATH%/}/firtool"
    if [[ ! -x "$bin" && ! -f "$bin" ]]; then
      echo "error: RHDL_FIRTOOL_PATH=${RHDL_FIRTOOL_PATH} does not contain a firtool binary" >&2
      echo "error: refusing silent success (FR164)" >&2
      exit 1
    fi
    echo "$bin"
    return 0
  fi
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
echo "circt-external-sim-check: firtool=${FIRTOOL_BIN}"

VER_OUT="$("$FIRTOOL_BIN" --version 2>&1 || true)"
echo "circt-external-sim-check: version-line=$(echo "$VER_OUT" | head -n 1)"
if ! grep -q "${EXPECTED_VERSION}" <<<"$VER_OUT"; then
  echo "error: firtool version mismatch: expected ${EXPECTED_VERSION}, got: ${VER_OUT}" >&2
  echo "error: AD-9 / FR164 require firtool-${EXPECTED_VERSION} (≠ CIRCT HEAD; ≠ PATH random); refusing silent success" >&2
  exit 1
fi

if [[ ! -f "$FIR" ]]; then
  echo "error: missing FIR compile fixture: $FIR" >&2
  exit 1
fi
if [[ ! -f "$SIM_FIR" ]]; then
  echo "error: missing FIR sim-exec fixture: $SIM_FIR" >&2
  exit 1
fi

mkdir -p "$OUT_DIR"
OUT_V="$OUT_DIR/fr164_external_circt_sim_gate.firtool.v"
echo "circt-external-sim-check: compiling $FIR -> $OUT_V (external firtool; prerequisite for sim gate)"
if ! "$FIRTOOL_BIN" "$FIR" -o="$OUT_V"; then
  echo "error: firtool compile failed for $FIR" >&2
  exit 1
fi
if [[ ! -f "$OUT_V" ]]; then
  echo "error: firtool did not write expected output $OUT_V" >&2
  exit 1
fi
if ! grep -q "Fr137Gate\|module" "$OUT_V"; then
  echo "error: firtool Verilog missing expected module markers" >&2
  exit 1
fi

echo "circt-external-sim-check: running cycle-sim execution predicate on $SIM_FIR (beyond FR137 compile alone)"
if ! cargo run -q -p bitloom --example fr164_circt_sim_gate -- "$SIM_FIR"; then
  echo "error: FR164 sim/execution predicate failed for $SIM_FIR" >&2
  exit 1
fi

echo "circt-external-sim-check: OK sim-gate firtool-${EXPECTED_VERSION} compile=$FIR sim=$SIM_FIR out=$OUT_V"
