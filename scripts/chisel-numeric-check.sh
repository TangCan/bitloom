#!/usr/bin/env bash
# Strict behavioral gate: the independent Rust matrix -> Scala/JVM -> RTL -> Icarus.
set -euo pipefail
ROOT="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)"
cd "$ROOT"
for tool in java sbt iverilog vvp; do
  command -v "$tool" >/dev/null || { echo "error: Chisel numeric gate requires $tool" >&2; exit 1; }
done
: "${RHDL_FIRTOOL_PATH:?set RHDL_FIRTOOL_PATH to firtool 1.159.0 directory}"
version="$("$RHDL_FIRTOOL_PATH/firtool" --version)"
printf '%s\n' "$version"
grep -q 'firtool-1.159.0' <<<"$version" || { echo 'error: expected firtool-1.159.0' >&2; exit 1; }
WORK="${BITLOOM_CHISEL_WORK:-$ROOT/target/chisel-numeric}"
mkdir -p "$WORK"
WORK="$(cd "$WORK" && pwd)"
rm -rf "$WORK/cases" "$WORK/src"
mkdir -p "$WORK/cases" "$WORK/src/main/scala" "$WORK/project"
export BITLOOM_CHISEL_CASES="$WORK/cases" BITLOOM_REQUIRE_RTL=1 BITLOOM_REQUIRE_FIRRTL=1
cargo test -p bitloom --test simulator_bit_vectors -- --nocapture
cat > "$WORK/build.sbt" <<'SBT'
scalaVersion := "2.13.18"
libraryDependencies += "org.chipsalliance" %% "chisel" % "7.15.0"
addCompilerPlugin("org.chipsalliance" % "chisel-plugin" % "7.15.0" cross CrossVersion.full)
SBT
printf 'sbt.version=1.10.11\n' > "$WORK/project/build.properties"
printf 'object NumericMain extends App {\n' > "$WORK/src/main/scala/NumericMain.scala"
for dir in "$WORK"/cases/*; do
  name="$(basename "$dir")"
  top="$(cat "$dir/top.txt")"
  cp "$dir/Design.scala" "$WORK/src/main/scala/$name.scala"
  printf '  circt.stage.ChiselStage.emitSystemVerilogFile(new %s.%s, args = Array("--target-dir", "cases/%s"), firtoolOpts = Array("--disable-all-randomization", "--lowering-options=disallowLocalVariables"))\n' "$name" "$top" "$name" >> "$WORK/src/main/scala/NumericMain.scala"
done
printf '}\n' >> "$WORK/src/main/scala/NumericMain.scala"
export CHISEL_FIRTOOL_PATH="$RHDL_FIRTOOL_PATH"
(cd "$WORK" && sbt -batch 'runMain NumericMain')
for dir in "$WORK"/cases/*; do
  echo "Chisel numeric RTL: $(basename "$dir")"
  (cd "$dir" && iverilog -g2012 -I . -s tb -o run -f filelist.f tb.v && vvp run)
done
echo "PASS: Chisel 7.15.0 / Scala 2.13.18 / sbt 1.10.11 / firtool 1.159.0 numeric RTL matrix"
