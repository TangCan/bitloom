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

# FR175: broader CIRCT SV / ir-verilog lower (beyond FR169 fir+hw).
# Not part of default `just test`. Missing/mismatched firtool → non-zero (never silent skip).
circt-external-sv-check:
	bash scripts/circt-external-sv-check.sh

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

# FR179: floating CIRCT HEAD track beyond FR174 (document-pinned firtool-1.159.0; ≠ AD-9; ≠ FR174).
# Not part of default `just test`. Missing/mismatched floating pin → non-zero (never silent skip).
circt-floating-git-head-check:
	bash scripts/circt-floating-git-head-check.sh

# FR186: unbounded / live CIRCT tip beyond FR179 floating-track pin (≠ AD-9 product default).
circt-live-tip-check:
	bash scripts/circt-live-tip-check.sh

# FR165: Style Guide / linter deepen beyond FR130 (≠ FR138 alone; no Chisel HEAD Parser).
chisel-style-lint-check:
	bash scripts/chisel-style-lint-check.sh

# FR176: combined Style Guide + update-mainline ecosystem deepen (≠ FR165/FR170 alone).
# Not part of default `just test`. FORCE_MISSING / sub-gate fail → non-zero (never silent skip).
chisel-ecosystem-deepen-check:
	bash scripts/chisel-ecosystem-deepen-check.sh

# FR181: deeper Style Guide / linter beyond FR176 (≠ FR176/FR165/FR130 alone).
# Not part of default `just test`. FORCE_MISSING / sub-gate fail → non-zero (never silent skip).
chisel-style-linter-deepen-check:
	bash scripts/chisel-style-linter-deepen-check.sh

# FR188: community Style Guide pack beyond FR181 (≠ FR181/FR176/FR165 alone).
# Not part of default `just test`. FORCE_MISSING / sub-gate fail → non-zero (never silent skip).
chisel-style-guide-pack-check:
	bash scripts/chisel-style-guide-pack-check.sh

# FR144: surface-crate SemVer breakage gate (cargo-semver-checks).
# Not part of default `just test`. Missing tool → non-zero (never silent success).
semver-check:
	bash scripts/semver-check.sh

# Behavioral Scala/JVM -> RTL numeric matrix; missing tools fail.
chisel-numeric-check:
	bash scripts/chisel-numeric-check.sh

# FR198 / FR201 core: complete AXI system across direct/FIRRTL/Chisel RTL,
# direct-CSR second composition, bounded formal, synthesis and SemVer.
fr198-fr201-core-check:
	python3 _agile-output/test-artifacts/129-3-build-runner.py

# FR200: fresh upstream closure, real wrapper/model, allowlisted offline replay,
# precise P0 mutations in normal and optimized Python; missing tools are fatal.
fr200-external-ip-pilot-check:
	#!/usr/bin/env bash
	set -euo pipefail
	cargo build -p bitloom
	cargo test -p bitloom --bin cargo-bitloom p0_behavior_oracle_kills_empty_and_zero_output_wrappers -- --ignored --nocapture
	: "${BITLOOM_VERILATOR_ROOT:?set to an explicit v5.052 installation; see scripts/provision-phase24-verilator.py}"
	gate_root=$(mktemp -d /tmp/bitloom-fr200-gate.XXXXXX)
	python3 scripts/phase24-external-ip-pilot.py --work "$gate_root/pilot"
	python3 -O scripts/phase24-external-ip-pilot.py --work "$gate_root/pilot" --reuse
	python3 scripts/phase24-external-ip-automation-test.py --work "$gate_root/pilot" --self-test
	python3 -O scripts/phase24-external-ip-automation-test.py --work "$gate_root/pilot" --mode optimized --self-test
	python3 scripts/phase24-external-ip-upstream.py --lock "$gate_root/pilot/source.lock.json" --cache "$gate_root/pilot/cache" --verilator-root "$BITLOOM_VERILATOR_ROOT" --work "$gate_root/upstream" --evidence "$gate_root/upstream.json" --timeout 3600 --parallel-cases
	BITLOOM_LOCKED_BINARY_PATH=bitloom BITLOOM_EXTERNAL_IP_MANIFEST="$gate_root/pilot/source.json" BITLOOM_EXTERNAL_IP_LOCK="$gate_root/pilot/source.lock.json" BITLOOM_EXTERNAL_IP_CACHE="$gate_root/pilot/cache" cargo test -p bitloom --test fr200_external_ip_binding --test fr200_external_ip_behavior -- --ignored
