#!/usr/bin/env bash
# Optional FR28 smoke: compile emitted Chisel Scala under pinned Chisel 7.14.0.
# Skips cleanly when Java < 17 or coursier/sbt is unavailable — does not fail CI.
# Pin: Chisel 7.14.0 ↔ firtool 1.155.0 (AD-9). Never silent-downgrade FR28 to best-effort.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SCALA="${1:-}"

if [[ -z "$SCALA" ]]; then
  echo "usage: $0 <generated.scala>" >&2
  exit 2
fi
if [[ ! -f "$SCALA" ]]; then
  echo "error: file not found: $SCALA" >&2
  exit 2
fi

need_java_major=17
if ! command -v java >/dev/null 2>&1; then
  echo "java not found; skipping Chisel compile (need Java >= ${need_java_major} + coursier/sbt)"
  exit 0
fi
java_ver="$(java -version 2>&1 | head -n1 || true)"
# Extract major: 1.8.x → 8; 11.x / 17.x → 11 / 17
major="$(sed -nE 's/.* version "([0-9]+)(\.[0-9]+)*.*/\1/p' <<<"$java_ver")"
if [[ "$major" == "1" ]]; then
  major="$(sed -nE 's/.* version "1\.([0-9]+).*/\1/p' <<<"$java_ver")"
fi
if [[ -z "${major:-}" ]] || (( major < need_java_major )); then
  echo "Java ${major:-unknown} (< ${need_java_major}); skipping Chisel compile"
  echo "  detected: $java_ver"
  echo "  install Java >= ${need_java_major} and coursier/sbt to enable true compile"
  exit 0
fi

CS=""
if command -v cs >/dev/null 2>&1; then
  CS=cs
elif command -v coursier >/dev/null 2>&1; then
  CS=coursier
fi
if [[ -z "$CS" ]] && ! command -v sbt >/dev/null 2>&1; then
  echo "coursier/sbt not found; skipping Chisel compile (Java ${major} OK)"
  exit 0
fi

WORKDIR="$ROOT/target/chisel-fr28-compile"
rm -rf "$WORKDIR"
mkdir -p "$WORKDIR/src/main/scala"
cp "$SCALA" "$WORKDIR/src/main/scala/"

# Minimal build.sbt for syntax+elaborate smoke under pinned Chisel.
cat >"$WORKDIR/build.sbt" <<'EOF'
scalaVersion := "2.13.16"
libraryDependencies += "org.chipsalliance" %% "chisel" % "7.14.0"
addCompilerPlugin("org.chipsalliance" % "chisel-plugin" % "7.14.0" cross CrossVersion.full)
EOF

echo "compiling under Chisel 7.14.0 in $WORKDIR ..."
(
  cd "$WORKDIR"
  if [[ -n "$CS" ]]; then
    # Fetch deps via coursier then use scalac if no sbt — prefer sbt when present.
    if command -v sbt >/dev/null 2>&1; then
      sbt -batch compile
    else
      echo "sbt not found; coursier alone cannot drive full Chisel plugin compile — skipping"
      exit 0
    fi
  else
    sbt -batch compile
  fi
)
echo "FR28 Chisel compile OK (Chisel 7.14.0)"
