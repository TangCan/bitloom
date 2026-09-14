#!/usr/bin/env bash
# FR188 — Community Style Guide pack gate beyond FR181 (Bitloom).
# Epic 121 / NFR14. Beyond FR181 alone / FR176 alone / FR165 alone / FR130 alone.
#
# Combined predicate:
#   (1) FR181 chisel-style-linter-deepen-check (includes FR176 + FR181 markers)
#   (2) FR188 community Style Guide pack emit/check markers
#
# Never silent success.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DOCS="${ROOT}/docs/fr188-community-style-guide-pack.md"

echo "chisel-style-guide-pack-check: FR188 community Style Guide pack (Bitloom)"
echo "chisel-style-guide-pack-check: ≠ FR181 alone; ≠ FR176 alone; ≠ FR165 alone; ≠ FR130 alone"

if [[ "${BITLOOM_STYLE_GUIDE_PACK_FORCE_MISSING:-0}" == "1" ]]; then
  echo "error: community Style Guide pack unavailable (BITLOOM_STYLE_GUIDE_PACK_FORCE_MISSING=1)" >&2
  echo "error: refusing silent success (FR188 failure semantics)" >&2
  exit 1
fi

if [[ ! -f "$DOCS" ]]; then
  echo "error: missing FR188 product docs: $DOCS" >&2
  exit 1
fi
if ! grep -q "FR188" "$DOCS" || ! grep -q "style-guide-pack\|community-style-guide\|scalafmt-community" "$DOCS"; then
  echo "error: FR188 docs must declare community Style Guide pack contract" >&2
  exit 1
fi
if ! grep -q "FR181" "$DOCS"; then
  echo "error: FR188 docs must reference FR181 style-linter deepen gate" >&2
  exit 1
fi

echo "chisel-style-guide-pack-check: (1/2) FR181 style-linter deepen"
bash "${ROOT}/scripts/chisel-style-linter-deepen-check.sh"

echo "chisel-style-guide-pack-check: (2/2) FR188 community Style Guide pack emit/check"
cd "$ROOT"
if ! cargo run -q -p bitloom --example fr188_style_guide_pack_gate; then
  echo "error: FR188 style-guide-pack emit/check gate failed" >&2
  exit 1
fi

echo "chisel-style-guide-pack-check: OK"
