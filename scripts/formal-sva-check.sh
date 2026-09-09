#!/usr/bin/env bash
# FR85 — non-toy formal path: real design → emit_sva → external checker.
# Documented entry: `just formal-sva-check` (not part of default `just test`).
#
# Behavior:
#   1. Export Counter SVA via `cargo run -p rhdl-formal --example export_fr85_counter`
#   2. Invoke an external checker:
#        - sby (SymbiYosys) if BITLOOM_FORMAL_CHECKER=sby or `sby` on PATH
#        - else verilator --lint-only --assert (default when available)
#   3. If no checker is available → exit non-zero with a clear error
#      (never silent success). Force with BITLOOM_FORMAL_FORCE_MISSING=1.
#
# Escape: none that turns missing-tool into success. This gate is honest.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${BITLOOM_FORMAL_OUT:-$ROOT/target/formal-sva}"
FIXTURE_COMMITTED="$ROOT/crates/rhdl-formal/fixtures/fr85_counter_sva.sv"

echo "formal-sva-check: FR85 external checker path (beyond toy string heuristics)"

mkdir -p "$OUT"
EXPORT_SV="$OUT/fr85_counter_sva.sv"

(
  cd "$ROOT"
  cargo run -q -p rhdl-formal --example export_fr85_counter -- "$EXPORT_SV"
)

if [[ ! -f "$EXPORT_SV" ]]; then
  echo "error: export did not write $EXPORT_SV" >&2
  exit 1
fi

# Keep committed fixture in sync when regenerating under the crate fixtures path.
if [[ "${BITLOOM_FORMAL_UPDATE_FIXTURE:-0}" == "1" ]]; then
  mkdir -p "$(dirname "$FIXTURE_COMMITTED")"
  cp "$EXPORT_SV" "$FIXTURE_COMMITTED"
  echo "formal-sva-check: updated committed fixture $FIXTURE_COMMITTED"
fi

if [[ ! -f "$FIXTURE_COMMITTED" ]]; then
  echo "error: missing committed fixture $FIXTURE_COMMITTED (run with BITLOOM_FORMAL_UPDATE_FIXTURE=1 once)" >&2
  exit 1
fi

# Sanity: exported content must look like SVA (structure), not rely on toy heuristic close.
if ! grep -q "assert property" "$EXPORT_SV"; then
  echo "error: exported SVA missing 'assert property': $EXPORT_SV" >&2
  exit 1
fi

pick_checker() {
  if [[ "${BITLOOM_FORMAL_FORCE_MISSING:-0}" == "1" ]]; then
    echo ""
    return
  fi
  local pref="${BITLOOM_FORMAL_CHECKER:-}"
  if [[ -n "$pref" ]]; then
    if command -v "$pref" >/dev/null 2>&1; then
      echo "$pref"
      return
    fi
    echo "error: BITLOOM_FORMAL_CHECKER=$pref not found on PATH" >&2
    exit 1
  fi
  if command -v sby >/dev/null 2>&1; then
    echo "sby"
    return
  fi
  if command -v verilator >/dev/null 2>&1; then
    echo "verilator"
    return
  fi
  echo ""
}

CHECKER="$(pick_checker)"
if [[ -z "$CHECKER" ]]; then
  echo "error: formal checker not found / unavailable (need verilator or sby on PATH)" >&2
  echo "error: install Verilator (verilator --lint-only --assert) or SymbiYosys (sby); refusing silent success" >&2
  exit 1
fi

echo "formal-sva-check: using checker=$CHECKER on $EXPORT_SV"

case "$CHECKER" in
  verilator)
    (
      cd "$OUT"
      # External tool parse/assert enablement — not in-repo toy string heuristics.
      verilator --lint-only --assert "$EXPORT_SV"
    )
    ;;
  sby)
    # Minimal documented hook: require operator to supply a .sby when using sby explicitly.
    SBY_FILE="${BITLOOM_FORMAL_SBY_FILE:-}"
    if [[ -z "$SBY_FILE" || ! -f "$SBY_FILE" ]]; then
      echo "error: sby selected but BITLOOM_FORMAL_SBY_FILE missing or not a file" >&2
      echo "error: set BITLOOM_FORMAL_SBY_FILE to a .sby that includes the exported SVA, or use verilator" >&2
      exit 1
    fi
    sby -f "$SBY_FILE"
    ;;
  *)
    echo "error: unsupported BITLOOM_FORMAL_CHECKER=$CHECKER (supported: verilator, sby)" >&2
    exit 1
    ;;
esac

echo "formal-sva-check: OK checker=$CHECKER export=$EXPORT_SV fixture=$FIXTURE_COMMITTED"
