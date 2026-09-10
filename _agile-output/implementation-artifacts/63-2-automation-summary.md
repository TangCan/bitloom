# Automation Summary — Story 63.2

## ATDD

- `cargo test -p bitloom --test fr122_official_style_chisel`
- Positive hierarchy; rejects FR111 alone / FR97 alone / mechanical; O1 package strip; O3 marker strip; O2 section-order swap; AD-27 revise stamp; FR97/FR111 docs still present

## Product path

- `emit_chisel_idiomatic_fr122` / `check_idiomatic_chisel_fr122` in `rhdl-firrtl`
- Ordered body sections for FR122 face; `package bitloom.generated`

## Out of scope

- Epic 63 NFR14 checkbox closeout / README deferred ledger (Story 63.3)
