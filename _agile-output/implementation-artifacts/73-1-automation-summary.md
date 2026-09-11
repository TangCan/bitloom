# Automation Summary — Story 73.1

## ATDD

- `cargo test -p bitloom --test nfr14_risk_epic73_upstream_tywaves_gui_ide`
- Locks NFR14 a–d, G1–G4 GUI/IDE depth shape (version/channel, metadata, failure, acceptance), FR125 T1–T4 boundary, bans FR104 / FR114 / FR117 / FR125 alone / docs-only, NFR14/56/59 owners, gate 73.2–73.3

## Regression guards

- None required beyond new ATDD (FR125 closeout does not freeze Epic 73)

## Out of scope

- No upstream GUI/IDE plugin implementation; no deferred/docs closeout; do not mark 73.2 ready; do not start Story 73.2
