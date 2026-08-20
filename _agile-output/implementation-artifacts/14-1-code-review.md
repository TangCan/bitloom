# Code review — Story 14.1

**Verdict:** approve

## Findings
- Package rename `bitloom-sim` with `publish = true`; depends on registry-compatible `bitloom-hir`.
- Rust path `bitloom_sim`; C ABI symbols remain `rhdl_sim_*` (ABI stability).
- README documents `cargo add bitloom-sim --dev`; AD-6 backend set uses `bitloom-*` names.
- Design `[dependencies]` unchanged (prelude-only).

## Must not regress
- Never put `bitloom-sim` in design `[dependencies]`
- Lockstep version with other published `bitloom-*` crates
