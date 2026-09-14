#!/usr/bin/env bash
# FR138 — Parser restore product path (Bitloom).
# Epic 77 / NFR14 P1–P4. Product-equivalent API: BitloomFirrtlParser.parse
# ≡ historical Scala firrtl.Parser.parse / Parser.parse (removed; chipsalliance/chisel#4899).
# Implementation: AD-9 firtool-1.159.0 -parse-only (CIRCT-era substitute).
# Pairing: Chisel 7.15.0 ↔ firtool-1.159.0 (AD-9 Stack).
#
# ≠ FR130 Style Guide alone (S3 Parser not restored for that FR).
# ≠ FR122 / FR111 / FR97 alone.
# ≠ docs-only.
#
# Resolve firtool via AD-9: RHDL_FIRTOOL_PATH or `cargo run -p bitloom -- firtool ensure`.
# Never treat bare PATH `firtool` as the success resolver.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EXPECTED_FIRTOOL_VERSION="1.159.0"
EXPECTED_CHISEL_VERSION="7.15.0"
FIR="${BITLOOM_PARSER_FIR:-$ROOT/crates/rhdl-firrtl/fixtures/fr138_parser_restore.fir}"
OUT_DIR="${BITLOOM_PARSER_OUT:-$ROOT/target/parser-restore-check}"

echo "parser-restore-check: FR138 BitloomFirrtlParser.parse gate (Bitloom; ≡ Parser.parse; ≠ FR130/122/111/97 alone)"
echo "parser-restore-check: pairing Chisel ${EXPECTED_CHISEL_VERSION} ↔ firtool-${EXPECTED_FIRTOOL_VERSION}"

if [[ "${BITLOOM_PARSER_FORCE_MISSING:-0}" == "1" ]]; then
  echo "error: Parser / BitloomFirrtlParser.parse unavailable (BITLOOM_PARSER_FORCE_MISSING=1)" >&2
  echo "error: install/ensure firtool-${EXPECTED_FIRTOOL_VERSION} via AD-9 (cargo bitloom firtool ensure) or RHDL_FIRTOOL_PATH; refusing silent success" >&2
  exit 1
fi

resolve_firtool() {
  if [[ -n "${RHDL_FIRTOOL_PATH:-}" ]]; then
    local bin="${RHDL_FIRTOOL_PATH%/}/firtool"
    if [[ ! -x "$bin" && ! -f "$bin" ]]; then
      echo "error: RHDL_FIRTOOL_PATH=${RHDL_FIRTOOL_PATH} does not contain a firtool binary" >&2
      echo "error: Parser product path unavailable; refusing silent success (FR138 P3)" >&2
      exit 1
    fi
    echo "$bin"
    return 0
  fi
  local ensured
  if ! ensured="$(cargo run -q -p bitloom -- firtool ensure 2>/dev/null)"; then
    echo "error: firtool ensure failed (AD-9 / firtool-${EXPECTED_FIRTOOL_VERSION})" >&2
    echo "error: BitloomFirrtlParser.parse / Parser equivalent unavailable; refusing silent success" >&2
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
echo "parser-restore-check: firtool=${FIRTOOL_BIN}"

VER_OUT="$("$FIRTOOL_BIN" --version 2>&1 || true)"
echo "parser-restore-check: version-line=$(echo "$VER_OUT" | head -n 1)"
if ! grep -q "${EXPECTED_FIRTOOL_VERSION}" <<<"$VER_OUT"; then
  echo "error: firtool version mismatch: expected ${EXPECTED_FIRTOOL_VERSION}, got: ${VER_OUT}" >&2
  echo "error: AD-9 / FR138 require firtool-${EXPECTED_FIRTOOL_VERSION} paired with Chisel ${EXPECTED_CHISEL_VERSION} (≠ CIRCT HEAD; ≠ PATH random); refusing silent success" >&2
  exit 1
fi

if [[ ! -f "$FIR" ]]; then
  echo "error: missing FIR fixture: $FIR" >&2
  exit 1
fi

mkdir -p "$OUT_DIR"
STAMP="$OUT_DIR/fr138_parser_restore.parse-ok"
echo "parser-restore-check: BitloomFirrtlParser.parse via firtool -parse-only on $FIR"
if ! "$FIRTOOL_BIN" -parse-only "$FIR" >"$OUT_DIR/fr138_parser_restore.parse.log" 2>&1; then
  echo "error: BitloomFirrtlParser.parse / firtool -parse-only failed for $FIR" >&2
  cat "$OUT_DIR/fr138_parser_restore.parse.log" >&2 || true
  exit 1
fi
{
  echo "FR138 BitloomFirrtlParser.parse OK"
  echo "equivalent_to=firrtl.Parser.parse / Parser.parse"
  echo "chisel=${EXPECTED_CHISEL_VERSION}"
  echo "firtool=${EXPECTED_FIRTOOL_VERSION}"
  echo "fixture=$FIR"
} >"$STAMP"

echo "parser-restore-check: OK BitloomFirrtlParser.parse firtool-${EXPECTED_FIRTOOL_VERSION} Chisel-${EXPECTED_CHISEL_VERSION} fixture=$FIR stamp=$STAMP"
