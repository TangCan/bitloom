#!/usr/bin/env bash
# FR28 Chisel Scala compile under pinned Chisel 7.14.0 (AD-9).
#
# Modes:
#   BITLOOM_REQUIRE_CHISEL_JVM=1  — missing Java≥17 / sbt / compile fail → non-zero (FR71)
#   BITLOOM_CHISEL_JVM_SKIP=1    — escape hatch: skip with exit 0 (NOT for default CI; NFR34)
#   (default / optional)         — missing toolchain → skip exit 0 (legacy local convenience)
#
# Prefer scripts/chisel-fr28-compile-required.sh for CI and just chisel-fr28-jvm.
# Pin: Chisel 7.14.0 ↔ firtool 1.155.0. Never silent-downgrade FR28 to best-effort.
set -euo pipefail
ROOT="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)"
SCALA="${1:-}"
REQUIRE="${BITLOOM_REQUIRE_CHISEL_JVM:-0}"
SKIP="${BITLOOM_CHISEL_JVM_SKIP:-0}"

fail_or_skip() {
  local msg="$1"
  if [[ "$SKIP" == "1" ]]; then
    echo "BITLOOM_CHISEL_JVM_SKIP=1: $msg (escape hatch; not default CI)" >&2
    exit 0
  fi
  if [[ "$REQUIRE" == "1" ]]; then
    echo "error: $msg" >&2
    exit 1
  fi
  echo "$msg (optional path; set BITLOOM_REQUIRE_CHISEL_JVM=1 to fail)"
  exit 0
}

if [[ -z "$SCALA" ]]; then
  echo "usage: $0 <generated.scala>" >&2
  exit 2
fi
if [[ ! -f "$SCALA" ]]; then
  echo "error: file not found: $SCALA" >&2
  exit 2
fi

if [[ "$SKIP" == "1" ]]; then
  echo "BITLOOM_CHISEL_JVM_SKIP=1: skipping Chisel compile (escape hatch; not default CI)"
  exit 0
fi

need_java_major=17
if ! command -v java >/dev/null 2>&1; then
  fail_or_skip "java not found (need Java >= ${need_java_major} + sbt)"
fi
java_ver="$(java -version 2>&1 | head -n1 || true)"
major="$(sed -nE 's/.* version "([0-9]+)(\.[0-9]+)*.*/\1/p' <<<"$java_ver")"
if [[ "$major" == "1" ]]; then
  major="$(sed -nE 's/.* version "1\.([0-9]+).*/\1/p' <<<"$java_ver")"
fi
if [[ -z "${major:-}" ]] || (( major < need_java_major )); then
  fail_or_skip "Java ${major:-unknown} < ${need_java_major} (detected: $java_ver)"
fi

if ! command -v sbt >/dev/null 2>&1; then
  fail_or_skip "sbt not found (Java ${major} OK; install sbt for Chisel plugin compile)"
fi

WORKDIR="$ROOT/target/chisel-fr28-compile"
rm -rf "$WORKDIR"
mkdir -p "$WORKDIR/src/main/scala" "$WORKDIR/project"
cp "$SCALA" "$WORKDIR/src/main/scala/"

# Minimal build.sbt: matching chisel + chisel-plugin (CrossVersion.full), Chisel 7.14.0.
cat >"$WORKDIR/build.sbt" <<'EOF'
scalaVersion := "2.13.16"
libraryDependencies += "org.chipsalliance" %% "chisel" % "7.14.0"
addCompilerPlugin("org.chipsalliance" % "chisel-plugin" % "7.14.0" cross CrossVersion.full)
EOF

# Pin sbt launcher for reproducible CI (optional; system sbt also OK).
cat >"$WORKDIR/project/build.properties" <<'EOF'
sbt.version=1.10.11
EOF

echo "compiling under Chisel 7.14.0 in $WORKDIR (require=$REQUIRE) ..."
(
  cd "$WORKDIR"
  sbt -batch compile
)
echo "FR28 Chisel compile OK (Chisel 7.14.0)"
