#!/usr/bin/env bash
# FR161 — formal-sby hygiene check (pins present + install script obedience).
# Runnable without sby on PATH. Missing/empty pins → non-zero (never silent-Ok).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PINS="${BITLOOM_SBY_PINS_FILE:-$ROOT/scripts/ci-sby-pins.env}"
INSTALL="$ROOT/scripts/ci-install-sby.sh"

echo "ci-sby-hygiene-check: FR161 formal-sby image/tooling hygiene (Bitloom)"

if [[ "${BITLOOM_SBY_HYGIENE_FORCE_FAIL:-0}" == "1" ]]; then
  echo "error: FR161 hygiene forced fail (BITLOOM_SBY_HYGIENE_FORCE_FAIL=1); refusing silent success" >&2
  exit 1
fi

if [[ ! -f "$PINS" ]]; then
  echo "error: FR161 pins file missing: $PINS" >&2
  exit 1
fi

# shellcheck disable=SC1090
source "$PINS"

missing=0
for key in SBY_GIT_URL SBY_GIT_REF SBY_GIT_SHA; do
  if [[ -z "${!key:-}" ]]; then
    echo "error: FR161 pin $key empty in $PINS" >&2
    missing=1
  fi
done
if [[ "$missing" -ne 0 ]]; then
  exit 1
fi

# SHA must look like a full git object name.
if [[ ! "$SBY_GIT_SHA" =~ ^[0-9a-fA-F]{40}$ ]]; then
  echo "error: FR161 SBY_GIT_SHA must be 40-char hex, got: $SBY_GIT_SHA" >&2
  exit 1
fi

if [[ ! -f "$INSTALL" ]]; then
  echo "error: FR161 expected install script missing: $INSTALL" >&2
  exit 1
fi

# Install script must source pins and clone the pinned ref (not floating HEAD alone).
if ! grep -q 'ci-sby-pins.env' "$INSTALL"; then
  echo "error: FR161 ci-install-sby.sh must reference ci-sby-pins.env" >&2
  exit 1
fi
if ! grep -q 'SBY_GIT_REF' "$INSTALL"; then
  echo "error: FR161 ci-install-sby.sh must use SBY_GIT_REF" >&2
  exit 1
fi
if ! grep -q 'SBY_GIT_SHA' "$INSTALL"; then
  echo "error: FR161 ci-install-sby.sh must verify SBY_GIT_SHA" >&2
  exit 1
fi
if ! grep -q -- '--branch' "$INSTALL" || ! grep -q 'SBY_GIT_REF' "$INSTALL"; then
  echo "error: FR161 ci-install-sby.sh must clone with --branch \"\$SBY_GIT_REF\"" >&2
  exit 1
fi
if grep -E 'git clone --depth 1 https://github.com/YosysHQ/sby\.git[[:space:]]' "$INSTALL" | grep -qv 'branch'; then
  echo "error: FR161 forbids unpinned floating sby clone URL without --branch pin" >&2
  exit 1
fi

echo "ci-sby-hygiene-check: ok pin ref=$SBY_GIT_REF sha=$SBY_GIT_SHA url=$SBY_GIT_URL"
echo "ci-sby-hygiene-check: install script obeys FR161 pins"
