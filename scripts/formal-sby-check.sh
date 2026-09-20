#!/usr/bin/env bash
# FR119 — SymbiYosys (sby) formal product path (F1-(ii)).
# Documented entry: `just formal-sby-check` (not part of default `just test`).
#
# ≠ FR85 `formal-sva-check` (Verilator lint / optional sby hook).
# ≠ FR100 F1-(i) in-tree FormalEquivProduct.
# ≠ FR112 branch B MemRead≡tick.
# ≠ FR107 SystemC AT.
#
# Behavior:
#   1. Require SymbiYosys `sby` on PATH (or fail readably).
#   2. Print tool versions when available (NFR14 version record obligation).
#   3. Run committed .sby fixture (default: pass; BITLOOM_SBY_MODE=fail for readable fail).
#   4. Missing tool / BITLOOM_SBY_FORCE_MISSING=1 → exit non-zero (never silent success).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FIXTURE_DIR="${BITLOOM_SBY_FIXTURE_DIR:-$ROOT/crates/rhdl-formal/fixtures/fr119}"
MODE="${BITLOOM_SBY_MODE:-pass}"

echo "formal-sby-check: FR119 SymbiYosys product path (Bitloom; ≠ FR85/FR100/FR112-B/FR107)"

if [[ "${BITLOOM_SBY_FORCE_MISSING:-0}" == "1" ]]; then
  echo "error: SymbiYosys (sby) unavailable (BITLOOM_SBY_FORCE_MISSING=1)" >&2
  echo "error: install SymbiYosys (sby) + Yosys + an SMT engine (e.g. z3) to run FR119; refusing silent success" >&2
  exit 1
fi

if ! command -v sby >/dev/null 2>&1; then
  echo "error: SymbiYosys (sby) not found on PATH" >&2
  echo "error: install SymbiYosys (sby) + Yosys + an SMT engine (e.g. z3) to run FR119; refusing silent success" >&2
  exit 1
fi

echo "formal-sby-check: sby=$(command -v sby)"
if timeout --kill-after=2s 10s sby --version >/dev/null 2>&1; then
  timeout --kill-after=2s 10s sby --version 2>&1 | head -n 5 | sed 's/^/formal-sby-check: /' || true
elif timeout --kill-after=2s 10s sby -h >/dev/null 2>&1; then
  echo "formal-sby-check: sby present (no --version); see docs/fr119-symbiyosys-smt.md to record band"
fi
if command -v yosys >/dev/null 2>&1; then
  timeout --kill-after=2s 10s yosys -V 2>&1 | head -n 2 | sed 's/^/formal-sby-check: /' || true
else
  echo "formal-sby-check: warning: yosys not on PATH (sby may still locate it)" >&2
fi

# Preflight the committed engine name when the .sby asks for smtbmc z3.
if [[ "${BITLOOM_SBY_SKIP_ENGINE_PREFLIGHT:-0}" != "1" ]]; then
  if ! command -v z3 >/dev/null 2>&1; then
    echo "error: SMT engine z3 not found on PATH (committed fixtures use 'smtbmc z3')" >&2
    echo "error: install z3 (or retarget [engines] and set BITLOOM_SBY_SKIP_ENGINE_PREFLIGHT=1 only if you know what you are doing); refusing silent success" >&2
    exit 1
  fi
  echo "formal-sby-check: z3=$(command -v z3)"
  timeout --kill-after=2s 10s z3 -version 2>&1 | head -n 1 | sed 's/^/formal-sby-check: /' || true
fi

case "$MODE" in
  pass)
    SBY_FILE="$FIXTURE_DIR/fr119_pass.sby"
    ;;
  fail)
    SBY_FILE="$FIXTURE_DIR/fr119_fail.sby"
    ;;
  *)
    echo "error: unsupported BITLOOM_SBY_MODE=$MODE (supported: pass, fail)" >&2
    exit 1
    ;;
esac

if [[ ! -f "$SBY_FILE" ]]; then
  echo "error: missing FR119 sby fixture: $SBY_FILE" >&2
  exit 1
fi

# Sanity: fixtures must carry assume + assert obligations (NFR14).
SV_FILE="$FIXTURE_DIR/fr119_${MODE}.sv"
if [[ ! -f "$SV_FILE" ]]; then
  echo "error: missing FR119 SV fixture: $SV_FILE" >&2
  exit 1
fi
if ! grep -q "assume property" "$SV_FILE"; then
  echo "error: FR119 fixture missing assume property: $SV_FILE" >&2
  exit 1
fi
if ! grep -q "assert property" "$SV_FILE"; then
  echo "error: FR119 fixture missing assert property: $SV_FILE" >&2
  exit 1
fi

echo "formal-sby-check: running sby -f $SBY_FILE (mode=$MODE)"
(
  cd "$FIXTURE_DIR"
  timeout --kill-after=5s 180s sby -f "$(basename "$SBY_FILE")"
)

STATUS_FILE="$FIXTURE_DIR/fr119_${MODE}/status"
if [[ ! -f "$STATUS_FILE" ]]; then
  echo "error: missing actual SBY status: $STATUS_FILE" >&2
  exit 1
fi
read -r ACTUAL_STATUS _ < "$STATUS_FILE"
if [[ "$ACTUAL_STATUS" != "PASS" ]]; then
  echo "error: actual SBY status=$ACTUAL_STATUS mode=$MODE; see $STATUS_FILE" >&2
  exit 1
fi
if [[ "$MODE" == "fail" ]]; then
  echo "error: negative fixture unexpectedly PASS" >&2
  exit 1
fi
echo "formal-sby-check: OK mode=$MODE actual status=PASS fixture=$SBY_FILE"
