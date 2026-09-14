# Automation Summary — Story 119.2

## ATDD

- `cargo test -p bitloom --test fr186_unbounded_circt_tip`
- `just circt-live-tip-check` (CI job `circt-live-tip`)

## Expand

Required CI job mirrors the just recipe; FORCE_MISSING covered by ATDD.
