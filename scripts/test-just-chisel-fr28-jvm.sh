#!/usr/bin/env bash
# ATDD: Story 25.2 — just chisel-fr28-jvm exists; just test stays Rust-only.
set -euo pipefail
ROOT="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)"
cd "$ROOT"
FAILS=0

if ! just --list 2>/dev/null | grep -q 'chisel-fr28-jvm'; then
  echo "FAIL: just list missing chisel-fr28-jvm"
  FAILS=$((FAILS + 1))
else
  echo "PASS: just chisel-fr28-jvm listed"
fi

if ! just --list 2>/dev/null | grep -q 'chisel-fr28-atdd'; then
  echo "FAIL: just list missing chisel-fr28-atdd"
  FAILS=$((FAILS + 1))
else
  echo "PASS: just chisel-fr28-atdd listed"
fi

# `just --show test` should be cargo test only (no chisel/jvm)
show="$(just --show test 2>/dev/null || true)"
if echo "$show" | grep -qiE 'chisel|jvm|java|sbt'; then
  echo "FAIL: just test recipe must stay Rust-only; got:"
  echo "$show"
  FAILS=$((FAILS + 1))
else
  echo "PASS: just test is Rust-only"
fi

# Recipe points at required script + golden
show_jvm="$(just --show chisel-fr28-jvm 2>/dev/null || true)"
if ! echo "$show_jvm" | grep -q 'chisel-fr28-compile-required'; then
  echo "FAIL: chisel-fr28-jvm must call compile-required"
  echo "$show_jvm"
  FAILS=$((FAILS + 1))
else
  echo "PASS: chisel-fr28-jvm calls required script"
fi

just chisel-fr28-atdd

if [[ "$FAILS" -ne 0 ]]; then
  echo "ATDD 25.2 failed: $FAILS"
  exit 1
fi
echo "ATDD OK: story 25.2 just contract"
