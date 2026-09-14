#!/usr/bin/env bash
# FR181 — Deeper Style Guide / linter gate beyond FR176 (Bitloom).
# Epic 114 / NFR14. Beyond FR176 alone / FR165 alone / FR130 alone / FR138 alone.
#
# Combined predicate:
#   (1) FR176 chisel-ecosystem-deepen-check (includes FR165 + FR170 + FR176 markers)
#   (2) FR181 style-linter deepen emit/check markers
#
# Never silent success.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DOCS="${ROOT}/docs/fr181-deeper-style-guide-linter.md"

echo "chisel-style-linter-deepen-check: FR181 Style Guide / linter deepen (Bitloom)"
echo "chisel-style-linter-deepen-check: ≠ FR176 alone; ≠ FR165 alone; ≠ FR130 alone; ≠ FR138 alone"

if [[ "${BITLOOM_STYLE_LINTER_DEEPEN_FORCE_MISSING:-0}" == "1" ]]; then
  echo "error: Style Guide / linter deepen pack unavailable (BITLOOM_STYLE_LINTER_DEEPEN_FORCE_MISSING=1)" >&2
  echo "error: refusing silent success (FR181 failure semantics)" >&2
  exit 1
fi

if [[ ! -f "$DOCS" ]]; then
  echo "error: missing FR181 product docs: $DOCS" >&2
  exit 1
fi
if ! grep -q "FR181" "$DOCS" || ! grep -q "style-linter" "$DOCS"; then
  echo "error: FR181 docs must declare style-linter deepen contract" >&2
  exit 1
fi
if ! grep -q "FR176" "$DOCS"; then
  echo "error: FR181 docs must reference FR176 combined gate" >&2
  exit 1
fi

echo "chisel-style-linter-deepen-check: (1/2) FR176 ecosystem deepen"
bash "${ROOT}/scripts/chisel-ecosystem-deepen-check.sh"

echo "chisel-style-linter-deepen-check: (2/2) FR181 style-linter emit/check"
cd "$ROOT"
if ! cargo run -q -p bitloom --example fr181_style_linter_gate; then
  echo "error: FR181 style-linter emit/check gate failed" >&2
  exit 1
fi

echo "chisel-style-linter-deepen-check: OK"
