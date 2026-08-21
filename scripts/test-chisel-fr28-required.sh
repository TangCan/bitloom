#!/usr/bin/env bash
# ATDD (red→green): FR71 required Chisel compile script exit-code contract.
# Run: bash scripts/test-chisel-fr28-required.sh
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REQ="$ROOT/scripts/chisel-fr28-compile-required.sh"
GOLDEN="$ROOT/crates/rhdl-firrtl/testdata/fr28_golden_counter.scala"
FAILS=0

assert_nonzero() {
  local name="$1"
  shift
  set +e
  "$@" >/tmp/fr28-atdd.out 2>/tmp/fr28-atdd.err
  local ec=$?
  set -e
  if [[ "$ec" -eq 0 ]]; then
    echo "FAIL: $name expected non-zero, got 0"
    cat /tmp/fr28-atdd.err || true
    FAILS=$((FAILS + 1))
  else
    echo "PASS: $name (exit $ec)"
  fi
}

assert_zero() {
  local name="$1"
  shift
  set +e
  "$@" >/tmp/fr28-atdd.out 2>/tmp/fr28-atdd.err
  local ec=$?
  set -e
  if [[ "$ec" -ne 0 ]]; then
    echo "FAIL: $name expected 0, got $ec"
    cat /tmp/fr28-atdd.err || true
    FAILS=$((FAILS + 1))
  else
    echo "PASS: $name"
  fi
}

chmod +x "$REQ" "$ROOT/scripts/chisel-fr28-compile.sh"

# usage / missing file
assert_nonzero "missing-arg" /bin/bash "$REQ"
assert_nonzero "missing-file" /bin/bash "$REQ" "$ROOT/no-such-file.scala"

# no java on PATH → required must fail
EMPTY_BIN="$(mktemp -d)"
assert_nonzero "no-java" env PATH="$EMPTY_BIN" BITLOOM_REQUIRE_CHISEL_JVM=1 \
  /bin/bash "$ROOT/scripts/chisel-fr28-compile.sh" "$GOLDEN"
rmdir "$EMPTY_BIN" 2>/dev/null || true

# Java present but <17 → required must fail (common contributor machine)
if command -v java >/dev/null 2>&1; then
  major="$(java -version 2>&1 | head -n1 | sed -nE 's/.* version "([0-9]+).*/\1/p')"
  if [[ -n "${major:-}" ]] && (( major < 17 )); then
    assert_nonzero "java-too-old" env BITLOOM_REQUIRE_CHISEL_JVM=1 \
      /bin/bash "$ROOT/scripts/chisel-fr28-compile.sh" "$GOLDEN"
  fi
fi

# SKIP escape hatch → zero even under require wrapper if SKIP set first
assert_zero "skip-escape" env BITLOOM_CHISEL_JVM_SKIP=1 BITLOOM_REQUIRE_CHISEL_JVM=1 \
  /bin/bash "$ROOT/scripts/chisel-fr28-compile.sh" "$GOLDEN"

# If real toolchain present, golden must compile (optional but preferred)
if command -v java >/dev/null 2>&1 && command -v sbt >/dev/null 2>&1; then
  major="$(java -version 2>&1 | head -n1 | sed -nE 's/.* version "([0-9]+).*/\1/p')"
  if [[ "${major:-0}" -ge 17 ]]; then
    assert_zero "golden-compile" /bin/bash "$REQ" "$GOLDEN"
  else
    echo "SKIP golden-compile: Java $major < 17"
  fi
else
  echo "SKIP golden-compile: java/sbt not both on PATH (CI job 25.3 will install)"
fi

if [[ "$FAILS" -ne 0 ]]; then
  echo "ATDD failed: $FAILS assertion(s)"
  exit 1
fi
echo "ATDD OK: chisel-fr28 required contract"
