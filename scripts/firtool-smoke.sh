#!/usr/bin/env bash
# Optional smoke: run pinned firtool-1.155.0 on an exported .fir (AD-9).
# Skips cleanly when firtool is not installed — does not fail CI by default.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FIR="${1:-}"
if [[ -z "$FIR" ]]; then
  echo "usage: $0 <file.fir>" >&2
  exit 2
fi
if ! command -v firtool >/dev/null 2>&1; then
  echo "firtool not found; skipping smoke (install firtool 1.155.0 to enable)"
  exit 0
fi
ver="$(firtool --version 2>&1 || true)"
if ! grep -q '1\.155\.0' <<<"$ver"; then
  echo "warning: expected firtool 1.155.0, got: $ver" >&2
fi
out="$ROOT/target/firtool-smoke"
mkdir -p "$out"
# Never overwrite phase-1 <abi>.v — write firtool SV beside it.
firtool "$FIR" -o="$out/smoke.firtool.v"
echo "wrote $out/smoke.firtool.v"
