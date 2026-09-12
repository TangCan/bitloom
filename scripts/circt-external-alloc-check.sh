#!/usr/bin/env bash
# FR169 — external CIRCT multi-lower / MLIR dialect allocation gate (Bitloom).
# Epic 102 / NFR14 option (A). Beyond FR164 sim and FR137 compile (see docs/fr169-*).
#
# ≠ FR164 sim alone.
# ≠ FR137 compile alone.
# ≠ FR129 C1–C4 alone (in-tree Handshake markers).
# ≠ firtool bump (option B / AD-9 revise) — this gate stays on firtool-1.158.0.
# ≠ scripts/firtool-smoke.sh (silent-skip when missing).
#
# Product path: multi-lower emits FIR dialect IR + HW dialect IR + Verilog from the
# same pinned firtool — dialect allocation / multi-lower evidence beyond cycle-sim.
#
# Resolve firtool via AD-9: RHDL_FIRTOOL_PATH or `cargo run -p bitloom -- firtool ensure`.
# Never treat bare PATH `firtool` as the success resolver.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EXPECTED_VERSION="1.158.0"
FIR="${BITLOOM_CIRCT_ALLOC_FIR:-$ROOT/crates/rhdl-firrtl/fixtures/fr169_circt_mlir_allocation.fir}"
OUT_DIR="${BITLOOM_CIRCT_ALLOC_OUT:-$ROOT/target/circt-external-alloc-check}"

echo "circt-external-alloc-check: FR169 external CIRCT multi-lower / MLIR allocation gate (Bitloom; ≠ FR164 alone)"

if [[ "${BITLOOM_CIRCT_ALLOC_FORCE_MISSING:-0}" == "1" ]]; then
  echo "error: external CIRCT firtool unavailable (BITLOOM_CIRCT_ALLOC_FORCE_MISSING=1)" >&2
  echo "error: install/ensure firtool-${EXPECTED_VERSION} via AD-9 (cargo bitloom firtool ensure) or RHDL_FIRTOOL_PATH; refusing silent success" >&2
  exit 1
fi

resolve_firtool() {
  if [[ -n "${RHDL_FIRTOOL_PATH:-}" ]]; then
    local bin="${RHDL_FIRTOOL_PATH%/}/firtool"
    if [[ ! -x "$bin" && ! -f "$bin" ]]; then
      echo "error: RHDL_FIRTOOL_PATH=${RHDL_FIRTOOL_PATH} does not contain a firtool binary" >&2
      echo "error: refusing silent success (FR169)" >&2
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
echo "circt-external-alloc-check: firtool=${FIRTOOL_BIN}"

VER_OUT="$("$FIRTOOL_BIN" --version 2>&1 || true)"
echo "circt-external-alloc-check: version-line=$(echo "$VER_OUT" | head -n 1)"
if ! grep -q "${EXPECTED_VERSION}" <<<"$VER_OUT"; then
  echo "error: firtool version mismatch: expected ${EXPECTED_VERSION}, got: ${VER_OUT}" >&2
  echo "error: AD-9 / FR169 require firtool-${EXPECTED_VERSION} (≠ CIRCT HEAD; ≠ PATH random); refusing silent success" >&2
  exit 1
fi

if [[ ! -f "$FIR" ]]; then
  echo "error: missing FIR allocation fixture: $FIR" >&2
  exit 1
fi

mkdir -p "$OUT_DIR"
OUT_FIR_IR="$OUT_DIR/fr169_alloc.ir-fir.mlir"
OUT_HW_IR="$OUT_DIR/fr169_alloc.ir-hw.mlir"
OUT_V="$OUT_DIR/fr169_alloc.firtool.v"

echo "circt-external-alloc-check: multi-lower (1/3) --ir-fir -> $OUT_FIR_IR"
if ! "$FIRTOOL_BIN" "$FIR" --ir-fir -o="$OUT_FIR_IR"; then
  echo "error: firtool --ir-fir failed for $FIR" >&2
  exit 1
fi
if [[ ! -f "$OUT_FIR_IR" ]]; then
  echo "error: firtool did not write $OUT_FIR_IR" >&2
  exit 1
fi
if ! grep -qE 'firrtl\.|module' "$OUT_FIR_IR"; then
  echo "error: FIR dialect IR missing expected markers in $OUT_FIR_IR" >&2
  exit 1
fi

echo "circt-external-alloc-check: multi-lower (2/3) --ir-hw -> $OUT_HW_IR (HW dialect allocation)"
if ! "$FIRTOOL_BIN" "$FIR" --ir-hw -o="$OUT_HW_IR"; then
  echo "error: firtool --ir-hw failed for $FIR" >&2
  exit 1
fi
if [[ ! -f "$OUT_HW_IR" ]]; then
  echo "error: firtool did not write $OUT_HW_IR" >&2
  exit 1
fi
if ! grep -q 'hw.module' "$OUT_HW_IR"; then
  echo "error: HW dialect IR missing hw.module allocation markers in $OUT_HW_IR" >&2
  exit 1
fi
if ! grep -q 'Fr169AllocGate' "$OUT_HW_IR"; then
  echo "error: HW dialect IR missing Fr169AllocGate module" >&2
  exit 1
fi

echo "circt-external-alloc-check: multi-lower (3/3) Verilog emit -> $OUT_V"
if ! "$FIRTOOL_BIN" "$FIR" -o="$OUT_V"; then
  echo "error: firtool Verilog emit failed for $FIR" >&2
  exit 1
fi
if [[ ! -f "$OUT_V" ]]; then
  echo "error: firtool did not write $OUT_V" >&2
  exit 1
fi
if ! grep -q 'Fr169AllocGate\|module' "$OUT_V"; then
  echo "error: Verilog missing expected module markers" >&2
  exit 1
fi

echo "circt-external-alloc-check: OK multi-lower firtool-${EXPECTED_VERSION} fir=$FIR fir_ir=$OUT_FIR_IR hw_ir=$OUT_HW_IR v=$OUT_V"
