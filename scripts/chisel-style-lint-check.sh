#!/usr/bin/env bash
# FR165 — Style Guide / linter deepen product path (Bitloom).
# Epic 97 / NFR14. Beyond FR130 Style Guide markers; ≠ FR138 Parser alone.
# Does not migrate arbitrary Chisel HEAD Parser (NFR71). AD-27 not revised (NFR70).
#
# Resolve via cargo example emit+check. Never silent success.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "chisel-style-lint-check: FR165 Style Guide / linter deepen (Bitloom; ≠ FR130/FR138 alone)"

if [[ "${BITLOOM_STYLE_LINT_FORCE_MISSING:-0}" == "1" ]]; then
  echo "error: Style Guide / linter pack unavailable (BITLOOM_STYLE_LINT_FORCE_MISSING=1)" >&2
  echo "error: refusing silent success (FR165 failure semantics)" >&2
  exit 1
fi

cd "$ROOT"
if ! cargo run -q -p bitloom --example fr165_style_lint_gate; then
  echo "error: FR165 style lint gate failed (emit/check)" >&2
  exit 1
fi

echo "chisel-style-lint-check: OK"
