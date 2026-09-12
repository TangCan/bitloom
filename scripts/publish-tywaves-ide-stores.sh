#!/usr/bin/env bash
# Bitloom FR167 (b) — multi IDE-store publish helper (Open VSX + JetBrains).
# Retains FR134 G1 VS Code marketplace id; does not silent-Ok missing tokens (NFR75).
set -euo pipefail

ROOT="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)"
cd "$ROOT"

if [[ -n "${BITLOOM_IDE_STORE_PUBLISH_FORCE_MISSING:-}" ]]; then
  echo "bitloom.ide-store-missing-token: BITLOOM_IDE_STORE_PUBLISH_FORCE_MISSING set (refusing silent-Ok)" >&2
  exit 2
fi

missing=0
if [[ -z "${OVSX_PAT:-}" ]]; then
  echo "bitloom.ide-store-missing-token: OVSX_PAT unset (Open VSX)" >&2
  missing=1
fi
if [[ -z "${JETBRAINS_TOKEN:-}" ]]; then
  echo "bitloom.ide-store-missing-token: JETBRAINS_TOKEN unset (JetBrains)" >&2
  missing=1
fi
if [[ "$missing" -ne 0 ]]; then
  echo "bitloom.ide-store-missing-token: refusing silent-Ok publish (set tokens for live; see docs/fr167-chiselsim-ide-stores.md)" >&2
  exit 2
fi

MODE="${1:-dry-run}"
echo "bitloom.ide-stores: FR167 tokens present (OVSX_PAT + JETBRAINS_TOKEN)"
echo "bitloom.ide-stores: Open VSX id=surfer-project.surfer"
echo "bitloom.ide-stores: JetBrains id=org.surferproject.surfer"
echo "bitloom.ide-stores: VS Code G1 retained (surfer-project.surfer marketplace) — alone ≠ FR167"

if [[ "$MODE" == "dry-run" ]]; then
  echo "bitloom.ide-stores: dry-run OK (no network publish). Pass 'live' after packaging the extension/plugin."
  exit 0
fi

if [[ "$MODE" != "live" ]]; then
  echo "usage: $0 [dry-run|live]" >&2
  exit 2
fi

echo "bitloom.ide-stores: live mode requires packaged VSIX / JetBrains plugin artifacts (NFR75)." >&2
echo "bitloom.ide-stores: wire ovsx/vsce/jetbrains-cli here when artifacts are present; refusing empty live." >&2
exit 2
