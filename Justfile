test:
	cargo test --workspace

fmt:
	cargo fmt --all

check:
	cargo fmt --all -- --check
	cargo test --workspace

firtool-smoke fir:
	bash scripts/firtool-smoke.sh {{fir}}

# FR35/FR50 HLS product-path smoke (CI stub by default; BITLOOM_HLS_USE_REAL=1 for Bambu 2024.10).
hls-smoke:
	bash scripts/hls-smoke.sh

# FR71 / NFR34: required Chisel JVM compile of the FR28 golden fixture (not part of `just test`).
# Needs Java ≥ 17 + sbt. Escape hatch: BITLOOM_CHISEL_JVM_SKIP=1 (never set in default CI).
chisel-fr28-jvm:
	bash scripts/chisel-fr28-compile-required.sh crates/rhdl-firrtl/testdata/fr28_golden_counter.scala

# FR81 Path A (optional): Mem SyncReadMem contract fixture under the same NFR12 compile path.
# Does NOT replace FR71 / `chisel-fr28-jvm` (counter golden remains the required gate).
chisel-fr81-mem-jvm:
	bash scripts/chisel-fr28-compile-required.sh crates/rhdl-firrtl/testdata/fr81_path_a_sync_read_mem.scala

# Exit-code ATDD for the required compile contract (no JDK install required for most asserts).
chisel-fr28-atdd:
	bash scripts/test-chisel-fr28-required.sh

# FR85: real design → emit_sva → external checker (verilator or sby).
# Not part of default `just test`. Missing checker → non-zero (never silent success).
formal-sva-check:
	bash scripts/formal-sva-check.sh

# FR119: SymbiYosys (sby) formal product path (F1-(ii); ≠ FR85 / FR100 / FR112-B).
# Not part of default `just test`. Missing sby → non-zero (never silent success).
formal-sby-check:
	bash scripts/formal-sby-check.sh

# FR137: external CIRCT compile gate (firtool-1.155.0 / AD-9; ≠ FR129 in-tree alone).
# Not part of default `just test`. Missing/mismatched firtool → non-zero (never silent skip).
circt-external-check:
	bash scripts/circt-external-check.sh

# FR164: external CIRCT sim gate (beyond FR137 compile-only).
circt-external-sim-check:
	bash scripts/circt-external-sim-check.sh

# FR169: external CIRCT multi-lower / MLIR allocation (beyond FR164 sim; AD-9 pin).
circt-external-alloc-check:
	bash scripts/circt-external-alloc-check.sh

# FR138: Parser restore product path (BitloomFirrtlParser.parse ≡ Parser.parse; AD-9 firtool -parse-only).
# Not part of default `just test`. Missing/mismatched firtool → non-zero (never silent skip).
parser-restore-check:
	bash scripts/parser-restore-check.sh

# FR170: Chisel update-mainline / HEAD Parser migration (beyond FR138; AD-27 revise; AD-9 pin unchanged).
parser-head-migration-check:
	bash scripts/parser-head-migration-check.sh

# FR174: unpaired CIRCT/firtool document-pinned mainline (≠ AD-9 product pin; ≠ FR170 alone).
# Not part of default `just test`. Missing/mismatched HEAD pin → non-zero (never silent skip).
circt-unpaired-head-check:
	bash scripts/circt-unpaired-head-check.sh

# FR165: Style Guide / linter deepen beyond FR130 (≠ FR138 alone; no Chisel HEAD Parser).
chisel-style-lint-check:
	bash scripts/chisel-style-lint-check.sh

# FR144: surface-crate SemVer breakage gate (cargo-semver-checks).
# Not part of default `just test`. Missing tool → non-zero (never silent success).
semver-check:
	bash scripts/semver-check.sh
