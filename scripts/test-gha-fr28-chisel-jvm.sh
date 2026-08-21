#!/usr/bin/env bash
# ATDD: Story 25.3 — GHA fr28-chisel-jvm job contract (static workflow asserts).
set -euo pipefail
ROOT="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)"
WF="$ROOT/.github/workflows/ci.yml"
FAILS=0

need() {
  local pat="$1" msg="$2"
  if ! grep -qE "$pat" "$WF"; then
    echo "FAIL: $msg"
    FAILS=$((FAILS + 1))
  else
    echo "PASS: $msg"
  fi
}

forbid() {
  local pat="$1" msg="$2"
  if grep -qE "$pat" "$WF"; then
    echo "FAIL: $msg"
    FAILS=$((FAILS + 1))
  else
    echo "PASS: $msg"
  fi
}

need 'fr28-chisel-jvm:' 'job fr28-chisel-jvm exists'
need 'java-version: ["'\'']?17' 'Temurin/Java 17'
need 'cache: sbt' 'sbt dependency cache'
need 'sbt/setup-sbt' 'setup-sbt action'
need 'chisel-fr28-compile-required' 'calls required compile script'
need 'timeout-minutes: 20' 'timeout pinned (~15–20)'
forbid 'BITLOOM_CHISEL_JVM_SKIP=' 'must not assign SKIP env in workflow'
forbid 'continue-on-error:\s*true' 'must not continue-on-error'

# Parallel: fr28 job must not need: [test]
if grep -A20 'fr28-chisel-jvm:' "$WF" | grep -q 'needs:'; then
  echo "FAIL: fr28-chisel-jvm should not depend on other jobs (Pattern A parallel)"
  FAILS=$((FAILS + 1))
else
  echo "PASS: fr28-chisel-jvm has no needs: (parallel)"
fi

if [[ "$FAILS" -ne 0 ]]; then
  echo "ATDD 25.3 failed: $FAILS"
  exit 1
fi
echo "ATDD OK: story 25.3 GHA contract"
