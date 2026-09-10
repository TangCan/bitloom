#!/usr/bin/env bash
# FR127 — install SymbiYosys (sby) + Yosys + z3 for required CI.
# Idempotent-ish: skips clone if sby already on PATH.
set -euo pipefail
echo "ci-install-sby: Bitloom FR127 toolchain bootstrap"

if command -v sby >/dev/null 2>&1 && command -v yosys >/dev/null 2>&1 && command -v z3 >/dev/null 2>&1; then
  echo "ci-install-sby: sby/yosys/z3 already present"
  sby -h >/dev/null 2>&1 || true
  yosys -V 2>&1 | head -n 1 || true
  z3 --version 2>&1 | head -n 1 || true
  exit 0
fi

export DEBIAN_FRONTEND=noninteractive
sudo apt-get update -y
sudo apt-get install -y yosys z3 python3 python3-pip git

# SymbiYosys (sby) — upstream YosysHQ
TMP="${RUNNER_TEMP:-/tmp}/bitloom-sby-src"
rm -rf "$TMP"
git clone --depth 1 https://github.com/YosysHQ/sby.git "$TMP"
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
