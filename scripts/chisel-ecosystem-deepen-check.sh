#!/usr/bin/env bash
# FR176 — Combined Parser / Chisel ecosystem deepen gate (Bitloom).
# Epic 109 / NFR14. Beyond FR170 alone / FR165 alone / FR138 alone / FR130 alone.
#
# Combined predicate:
#   (1) FR165 style-lint gate
#   (2) FR170 parser-head-migration-check @ AD-9 pin
#   (3) FR176 ecosystem emit/check markers
#
# Never silent success.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DOCS="${ROOT}/docs/fr176-deeper-parser-chisel-ecosystem.md"

echo "chisel-ecosystem-deepen-check: FR176 combined Style Guide + update-mainline ecosystem (Bitloom)"
echo "chisel-ecosystem-deepen-check: ≠ FR170 alone; ≠ FR165 alone; ≠ FR138 alone; ≠ FR130 alone"

if [[ "${BITLOOM_ECOSYSTEM_FORCE_MISSING:-0}" == "1" ]]; then
  echo "error: Chisel ecosystem deepen pack unavailable (BITLOOM_ECOSYSTEM_FORCE_MISSING=1)" >&2
  echo "error: refusing silent success (FR176 failure semantics)" >&2
  exit 1
fi

if [[ ! -f "$DOCS" ]]; then
  echo "error: missing FR176 product docs: $DOCS" >&2
  exit 1
fi
if ! grep -q "FR176" "$DOCS" || ! grep -q "ecosystem" "$DOCS"; then
  echo "error: FR176 docs must declare ecosystem deepen contract" >&2
  exit 1
fi
if ! grep -q "FR165" "$DOCS" || ! grep -q "FR170" "$DOCS"; then
  echo "error: FR176 docs must reference FR165 + FR170 combined gates" >&2
  exit 1
fi

echo "chisel-ecosystem-deepen-check: (1/3) FR165 style-lint"
bash "${ROOT}/scripts/chisel-style-lint-check.sh"

echo "chisel-ecosystem-deepen-check: (2/3) FR170 parser-head-migration @ AD-9"
bash "${ROOT}/scripts/parser-head-migration-check.sh"

echo "chisel-ecosystem-deepen-check: (3/3) FR176 ecosystem emit/check"
cd "$ROOT"
if ! cargo run -q -p bitloom --example fr176_ecosystem_gate; then
  echo "error: FR176 ecosystem emit/check gate failed" >&2
  exit 1
fi

echo "chisel-ecosystem-deepen-check: OK"
