# Automation Summary — Story 76.1

## ATDD

- `cargo test -p bitloom --test nfr14_risk_epic76_external_circt_compile_sim_gate`
- Locks NFR14 a–d, external CIRCT tool version/channel (firtool-1.155.0), E1–E4 compile/sim gate predicates, missing-tool non-zero readable failure, FR129 C1–C4 boundary, bans FR95 / FR96 / FR110 / FR121 / FR129 C1–C4 alone / docs-only / continue-on-error, NFR14/56/58/59 owners, gate 76.2–76.3

## Regression guards

- None required beyond new ATDD (FR129 closeout does not freeze Epic 76)

## Out of scope

- No external CIRCT compile/sim gate implementation; no deferred/docs closeout; do not mark 76.2 ready; do not start Story 76.2
