# Automation Summary — Story 90.2

## ATDD

- `cargo test -p bitloom --test fr158_third_party_lcov_gui`

Locks: docs (genhtml / FR114 ≠ FR158 / missing tool / FR142 note); GenhtmlNotFound on empty PATH; fake genhtml → index.html; LcovMissing; CLI `--genhtml` in help.

## Expand

No host `lcov` package required for CI — success path uses a PATH stub.
