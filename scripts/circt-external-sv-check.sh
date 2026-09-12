#!/usr/bin/env bash
# FR175 — broader CIRCT/MLIR lower beyond FR169 (Bitloom).
# Epic 108 / NFR14. Beyond FR169 --ir-fir + --ir-hw + Verilog alone.
#
# Selected deepen: --ir-sv (SV dialect) + --ir-verilog (post-Verilog-lowering IR)
# at AD-9 firtool-1.158.0, then Verilog emit.
#
# ≠ FR169 alone · ≠ FR164 alone · ≠ FR137 alone · ≠ FR129 alone
# ≠ PATH-random · ≠ unpaired HEAD pin · ≠ silent-Ok under BITLOOM_CIRCT_SV_FORCE_MISSING
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EXPECTED_VERSION="1.158.0"
FIR="${BITLOOM_CIRCT_SV_FIR:-$ROOT/crates/rhdl-firrtl/fixtures/fr169_circt_mlir_allocation.fir}"
OUT_DIR="${BITLOOM_CIRCT_SV_OUT:-$ROOT/target/circt-external-sv-check}"
DOCS="${ROOT}/docs/fr175-broader-circt-mlir-sim.md"

echo "circt-external-sv-check: FR175 broader CIRCT SV / ir-verilog gate (Bitloom; ≠ FR169 alone)"
echo "circt-external-sv-check: AD-9 pin firtool-${EXPECTED_VERSION}"

if [[ "${BITLOOM_CIRCT_SV_FORCE_MISSING:-0}" == "1" ]]; then
  echo "error: external CIRCT firtool unavailable (BITLOOM_CIRCT_SV_FORCE_MISSING=1)" >&2
  echo "error: install/ensure firtool-${EXPECTED_VERSION} via AD-9 (cargo bitloom firtool ensure) or RHDL_FIRTOOL_PATH; refusing silent success" >&2
  exit 1
fi

if [[ ! -f "$DOCS" ]]; then
  echo "error: missing FR175 product docs: $DOCS" >&2
  exit 1
fi
if ! grep -q "FR175" "$DOCS" || ! grep -qE -- '--ir-sv|ir-sv' "$DOCS"; then
  echo "error: FR175 docs must pin --ir-sv deepen" >&2
  exit 1
fi
if ! grep -qE -- '--ir-verilog|ir-verilog' "$DOCS"; then
  echo "error: FR175 docs must pin --ir-verilog deepen" >&2
  exit 1
fi

resolve_firtool() {
  if [[ -n "${RHDL_FIRTOOL_PATH:-}" ]]; then
    local bin="${RHDL_FIRTOOL_PATH%/}/firtool"
    if [[ ! -x "$bin" && ! -f "$bin" ]]; then
      echo "error: RHDL_FIRTOOL_PATH=${RHDL_FIRTOOL_PATH} does not contain a firtool binary" >&2
      exit 1
    fi
    echo "$bin"
    return 0
  fi
  local ensured
  if ! ensured="$(cargo run -q -p bitloom -- firtool ensure 2>/dev/null)"; then
    echo "error: firtool ensure failed (AD-9 / firtool-${EXPECTED_VERSION})" >&2
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
echo "circt-external-sv-check: firtool=${FIRTOOL_BIN}"

VER_OUT="$("$FIRTOOL_BIN" --version 2>&1 || true)"
echo "circt-external-sv-check: version-line=$(echo "$VER_OUT" | head -n 1)"
if ! grep -q "${EXPECTED_VERSION}" <<<"$VER_OUT"; then
  echo "error: firtool version mismatch: expected ${EXPECTED_VERSION}, got: ${VER_OUT}" >&2
  echo "error: AD-9 / FR175 require firtool-${EXPECTED_VERSION}; refusing silent success" >&2
  exit 1
fi

if [[ ! -f "$FIR" ]]; then
  echo "error: missing FIR fixture: $FIR" >&2
  exit 1
fi

mkdir -p "$OUT_DIR"
OUT_SV="$OUT_DIR/fr175_sv.ir-sv.mlir"
OUT_IRV="$OUT_DIR/fr175_sv.ir-verilog.mlir"
OUT_V="$OUT_DIR/fr175_sv.firtool.v"

echo "circt-external-sv-check: deepen (1/3) --ir-sv -> $OUT_SV"
if ! "$FIRTOOL_BIN" "$FIR" --ir-sv -o="$OUT_SV"; then
  echo "error: firtool --ir-sv failed for $FIR" >&2
  exit 1
fi
if [[ ! -f "$OUT_SV" ]] || ! grep -qE 'sv\.|hw\.module' "$OUT_SV"; then
  echo "error: SV dialect IR missing expected markers in $OUT_SV" >&2
  exit 1
fi

echo "circt-external-sv-check: deepen (2/3) --ir-verilog -> $OUT_IRV"
if ! "$FIRTOOL_BIN" "$FIR" --ir-verilog -o="$OUT_IRV"; then
  echo "error: firtool --ir-verilog failed for $FIR" >&2
  exit 1
fi
if [[ ! -f "$OUT_IRV" ]] || ! grep -qE 'sv\.|hw\.|module' "$OUT_IRV"; then
  echo "error: ir-verilog IR missing expected markers in $OUT_IRV" >&2
  exit 1
fi

echo "circt-external-sv-check: deepen (3/3) Verilog emit -> $OUT_V"
if ! "$FIRTOOL_BIN" "$FIR" -o="$OUT_V"; then
  echo "error: firtool Verilog emit failed for $FIR" >&2
  exit 1
fi
if [[ ! -f "$OUT_V" ]] || ! grep -qE 'Fr169AllocGate|module' "$OUT_V"; then
  echo "error: Verilog missing expected module markers" >&2
  exit 1
fi

STAMP="$OUT_DIR/fr175_sv.parse-ok"
{
  echo "FR175 broader CIRCT SV / ir-verilog OK"
  echo "pin=firtool-${EXPECTED_VERSION}"
  echo "fixture=$FIR"
} >"$STAMP"

echo "circt-external-sv-check: OK stamp=$STAMP"
