#!/usr/bin/env bash
# FR127 + FR161 — install SymbiYosys (sby) + Yosys + z3 for required CI.
# FR161: clone uses pinned ref/SHA from scripts/ci-sby-pins.env (not floating HEAD).
# Idempotent-ish: skips clone if sby already on PATH (pins still loaded/validated).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PINS="${BITLOOM_SBY_PINS_FILE:-$ROOT/scripts/ci-sby-pins.env}"

echo "ci-install-sby: Bitloom FR127/FR161 toolchain bootstrap"

if [[ ! -f "$PINS" ]]; then
  echo "error: FR161 pins file missing: $PINS" >&2
  echo "error: refusing silent install without hygiene pins" >&2
  exit 1
fi
# shellcheck disable=SC1090
source "$PINS"
if [[ -z "${SBY_GIT_URL:-}" || -z "${SBY_GIT_REF:-}" || -z "${SBY_GIT_SHA:-}" ]]; then
  echo "error: FR161 pins incomplete (need SBY_GIT_URL, SBY_GIT_REF, SBY_GIT_SHA) in $PINS" >&2
  exit 1
fi
echo "ci-install-sby: FR161 pin ref=$SBY_GIT_REF sha=$SBY_GIT_SHA"

if command -v sby >/dev/null 2>&1 && command -v yosys >/dev/null 2>&1 && command -v z3 >/dev/null 2>&1; then
  echo "ci-install-sby: sby/yosys/z3 already present (skip clone; pins recorded above)"
  sby -h >/dev/null 2>&1 || true
  yosys -V 2>&1 | head -n 1 || true
  z3 --version 2>&1 | head -n 1 || true
  exit 0
fi

export DEBIAN_FRONTEND=noninteractive
sudo apt-get update -y
sudo apt-get install -y yosys z3 python3 python3-pip git

# SymbiYosys (sby) — upstream YosysHQ at FR161 pin (not HEAD).
TMP="${RUNNER_TEMP:-/tmp}/bitloom-sby-src"
rm -rf "$TMP"
git clone --depth 1 --branch "$SBY_GIT_REF" "$SBY_GIT_URL" "$TMP"
ACTUAL_SHA="$(git -C "$TMP" rev-parse HEAD)"
if [[ "$ACTUAL_SHA" != "$SBY_GIT_SHA" ]]; then
  echo "error: FR161 sby SHA mismatch: expected $SBY_GIT_SHA got $ACTUAL_SHA (ref $SBY_GIT_REF)" >&2
  exit 1
fi
echo "ci-install-sby: cloned sby@$ACTUAL_SHA (matches pin)"

# sby is a Python driver; install into /usr/local/bin
sudo install -m 0755 "$TMP/sbysrc/sby.py" /usr/local/bin/sby
# Also ship support modules next to it when present
if [[ -d "$TMP/sbysrc" ]]; then
  sudo mkdir -p /usr/local/share/sby
  sudo cp -a "$TMP/sbysrc/." /usr/local/share/sby/
fi

command -v sby >/dev/null
command -v yosys >/dev/null
command -v z3 >/dev/null
echo "ci-install-sby: installed sby=$(command -v sby) yosys=$(command -v yosys) z3=$(command -v z3)"
