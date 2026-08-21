#!/usr/bin/env bash
# CI wiring stub for Bitloom HLS (FR35 / FR50).
# Mimics a pinned Bambu binary: accepts a .c path and writes synthesizable Verilog.
# NOT a substitute for real PandA Bambu 2024.10 synthesis quality — see docs/fr35-hls.md.
set -euo pipefail
OUT_DIR="$(pwd)"
FN="add"
for arg in "$@"; do
  case "$arg" in
    --top-fname=*) FN="${arg#--top-fname=}" ;;
    *.c) BASE="$(basename "$arg" .c)"; FN="$BASE" ;;
  esac
done
# Prefer cwd from caller (cargo bitloom hls sets current_dir=out_dir).
V="${OUT_DIR}/${FN}.v"
cat >"$V" <<EOF
// bitloom HLS CI stub (not real Bambu); synthesizable smoke only
module ${FN} (
  input  wire [31:0] a,
  input  wire [31:0] b,
  output wire [31:0] y
);
  assign y = a + b;
endmodule
EOF
echo "bambu-ci-stub wrote $V"
