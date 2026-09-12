#!/usr/bin/env bash
# FR170 — Chisel update-mainline / HEAD Parser migration gate (Bitloom).
# Epic 103 / NFR14. Beyond FR138 BitloomFirrtlParser.parse restore alone.
#
# Selected shape: document-pinned **update-mainline** Parser product path
# (Chisel 7.14.0 ↔ firtool-1.155.0 / AD-9 unchanged — Epic 102 option B deferred).
# Migration delta vs FR138: accept FIRRTL **version 6.0.0** mainline dialect
# (AD-3 exchange header) via BitloomFirrtlParser.parseUpdateMainline ≡ parse.
#
# ≠ FR138 alone (v4 fixture / parser-restore-check alone).
# ≠ FR165 Style Guide/linter alone.
# ≠ FR130 alone.
# ≠ unpaired CIRCT HEAD / PATH-random firtool (AD-9 not revised this batch).
#
# Resolve firtool via AD-9: RHDL_FIRTOOL_PATH or `cargo bitloom firtool ensure`.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EXPECTED_FIRTOOL_VERSION="1.155.0"
EXPECTED_CHISEL_UPDATE_MAINLINE="7.14.0"
FIR="${BITLOOM_PARSER_HEAD_FIR:-$ROOT/crates/rhdl-firrtl/fixtures/fr170_chisel_head_parser.fir}"
OUT_DIR="${BITLOOM_PARSER_HEAD_OUT:-$ROOT/target/parser-head-migration-check}"
DOCS="${ROOT}/docs/fr170-chisel-head-parser.md"

echo "parser-head-migration-check: FR170 BitloomFirrtlParser.parseUpdateMainline gate (Bitloom; ≠ FR138 alone)"
echo "parser-head-migration-check: update-mainline pin Chisel ${EXPECTED_CHISEL_UPDATE_MAINLINE} ↔ firtool-${EXPECTED_FIRTOOL_VERSION}"

if [[ "${BITLOOM_PARSER_HEAD_FORCE_MISSING:-0}" == "1" ]]; then
  echo "error: update-mainline Parser / BitloomFirrtlParser.parseUpdateMainline unavailable (BITLOOM_PARSER_HEAD_FORCE_MISSING=1)" >&2
  echo "error: install/ensure firtool-${EXPECTED_FIRTOOL_VERSION} via AD-9 (cargo bitloom firtool ensure) or RHDL_FIRTOOL_PATH; refusing silent success" >&2
  exit 1
fi

if [[ ! -f "$DOCS" ]]; then
  echo "error: missing FR170 product docs: $DOCS" >&2
  exit 1
fi
if ! grep -q "update-mainline" "$DOCS" || ! grep -q "${EXPECTED_CHISEL_UPDATE_MAINLINE}" "$DOCS"; then
  echo "error: FR170 docs must pin update-mainline Chisel ${EXPECTED_CHISEL_UPDATE_MAINLINE}" >&2
  exit 1
fi
if ! grep -q "FIRRTL version 6.0.0\|version 6.0.0" "$DOCS"; then
  echo "error: FR170 docs must declare FIRRTL 6.0.0 mainline dialect migration vs FR138" >&2
  exit 1
fi

resolve_firtool() {
  if [[ -n "${RHDL_FIRTOOL_PATH:-}" ]]; then
    local bin="${RHDL_FIRTOOL_PATH%/}/firtool"
    if [[ ! -x "$bin" && ! -f "$bin" ]]; then
      echo "error: RHDL_FIRTOOL_PATH=${RHDL_FIRTOOL_PATH} does not contain a firtool binary" >&2
      echo "error: refusing silent success (FR170)" >&2
      exit 1
    fi
    echo "$bin"
    return 0
  fi
  local ensured
  if ! ensured="$(cargo run -q -p bitloom -- firtool ensure 2>/dev/null)"; then
    echo "error: firtool ensure failed (AD-9 / firtool-${EXPECTED_FIRTOOL_VERSION})" >&2
    echo "error: BitloomFirrtlParser.parseUpdateMainline unavailable; refusing silent success" >&2
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
echo "parser-head-migration-check: firtool=${FIRTOOL_BIN}"

VER_OUT="$("$FIRTOOL_BIN" --version 2>&1 || true)"
echo "parser-head-migration-check: version-line=$(echo "$VER_OUT" | head -n 1)"
if ! grep -q "${EXPECTED_FIRTOOL_VERSION}" <<<"$VER_OUT"; then
  echo "error: firtool version mismatch: expected ${EXPECTED_FIRTOOL_VERSION}, got: ${VER_OUT}" >&2
  echo "error: AD-9 / FR170 require firtool-${EXPECTED_FIRTOOL_VERSION} (≠ CIRCT HEAD; ≠ PATH random); refusing silent success" >&2
  exit 1
fi

if [[ ! -f "$FIR" ]]; then
  echo "error: missing FR170 FIR fixture: $FIR" >&2
  exit 1
fi
if ! grep -q 'FIRRTL version 6.0.0' "$FIR"; then
  echo "error: FR170 fixture must be FIRRTL version 6.0.0 (mainline dialect vs FR138 v4)" >&2
  exit 1
fi

mkdir -p "$OUT_DIR"
STAMP="$OUT_DIR/fr170_head_parser.parse-ok"
echo "parser-head-migration-check: BitloomFirrtlParser.parseUpdateMainline via firtool -parse-only on $FIR"
if ! "$FIRTOOL_BIN" -parse-only "$FIR" >"$OUT_DIR/fr170_head_parser.parse.log" 2>&1; then
  echo "error: BitloomFirrtlParser.parseUpdateMainline / firtool -parse-only failed for $FIR" >&2
  cat "$OUT_DIR/fr170_head_parser.parse.log" >&2 || true
  exit 1
fi
{
  echo "FR170 BitloomFirrtlParser.parseUpdateMainline OK"
  echo "update-mainline Chisel ${EXPECTED_CHISEL_UPDATE_MAINLINE} ↔ firtool-${EXPECTED_FIRTOOL_VERSION}"
  echo "fixture=$FIR"
  echo "dialect=FIRRTL version 6.0.0"
} >"$STAMP"

echo "parser-head-migration-check: OK stamp=$STAMP"
