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

# Exit-code ATDD for the required compile contract (no JDK install required for most asserts).
chisel-fr28-atdd:
	bash scripts/test-chisel-fr28-required.sh
