#!/usr/bin/env bash
# FR71 required path: fail if JDK/sbt missing or sbt compile fails.
# Do not set BITLOOM_CHISEL_JVM_SKIP in default CI.
set -euo pipefail
ROOT="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)"
export BITLOOM_REQUIRE_CHISEL_JVM=1
# Explicitly ignore skip unless caller forces it (documented escape only).
exec /bin/bash "$ROOT/scripts/chisel-fr28-compile.sh" "$@"
