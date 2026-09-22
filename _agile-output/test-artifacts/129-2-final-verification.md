# Story 129.2 final verification

Date: 2026-09-22

## Seven-step result

- create-story: Story and build spec created, then restored to `in-progress` after the resume audit found unconditional PASS benches.
- ATDD: normal Python and `python -O` gates both GREEN.
- build: real AXI-Lite and bridge-free direct CSR hierarchies execute the four leaf peripherals.
- code review: four delegated layers timed out and were not called clean; fallback triage fixed two medium findings, deferred one ownership conflict to 129.3, and rejected one false finding.
- automate: isolated checkout, empty Cargo target, independent RTL root, Icarus/VVP behavior, VCD, and Yosys `check -assert` all PASS.
- regression: `cargo clean` removed 45,368 files / 13.6 GiB, `cargo fmt --all` completed, and the `cargo test --workspace` process launched by `just test` completed with exit code 0.
- commit: this evidence and the Story implementation are included in the single Story 129.2 commit.

## Behavioral evidence

| Topology | Directed + random transactions | Assertions | Frozen random budget |
| --- | ---: | ---: | ---: |
| AXI-Lite -> bridge -> decoder -> four leaves | 16,102 | 62,416 | 16 seeds x 1,000 |
| Direct CSR -> decoder -> four leaves | 16,098 | 60,406 | 16 seeds x 1,000 |

The final isolated replay reported `3 passed; 0 failed; 0 ignored`. Both topology simulations exited 0, emitted non-empty VCD files, and passed strict Yosys structure checks. The default-PATH negative control exited 101 with `required iverilog executable is missing`, proving that a missing required simulator is not skipped.

## Regression environment note

The outer Trae command session returned 1 after the successful Cargo process because the workspace sandbox rejected 1,013 file probes under `/proc/*/fd` and user-home Yosys history paths. Sandbox trace index 20603 records the Cargo process with `exit_code_nonzero: 0`; trace index 20609 records the separate file-block result. This is retained as an infrastructure warning, not represented as either a Cargo test failure or an unrestricted-environment PASS.

Ordinary workspace tests intentionally leave dedicated formal tests ignored. Those ignored tests are not counted as PASS and are outside Story 129.2's RTL behavior gate. FIRRTL/Chisel four-peripheral system closure remains assigned to Story 129.3.
