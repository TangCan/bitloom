# Code review: Story 15.1

## Summary

PASS with notes. Feasibility spike + minimal `Lit`/`Eq`/`Mux` close FR60 without claiming FR56.

## Findings

- **Fixed in-story:** HIR lacked data-dependent select; added `AssignExpr::{Eq,Mux}` and builder APIs; sim/vlog/firrtl/chisel updated.
- **Accepted debt:** sequential `we` gating deferred (documented in `FEASIBILITY.md`); `begin_then` remains latch-only.
- **NFR24:** `rv32_feasibility` depends only on `bitloom-prelude` for design.

## Tests

`cargo test -p rv32_feasibility` — elaborate / tick FSM+mem / emit `.v` smoke.
