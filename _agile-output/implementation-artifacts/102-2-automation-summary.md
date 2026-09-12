# Automation Summary — Story 102.2

- ATDD: `crates/bitloom/tests/fr169_circt_mlir_allocation.rs`
- Product: `scripts/circt-external-alloc-check.sh` (`--ir-fir` + `--ir-hw` + Verilog)
- Fixture: `crates/rhdl-firrtl/fixtures/fr169_circt_mlir_allocation.fir`
- Just: `circt-external-alloc-check`; CI job `circt-external-alloc` (required)
- Docs: `docs/fr169-circt-mlir-allocation.md`
- Regression: `cargo fmt --all && just test`
