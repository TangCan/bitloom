# Code Review: Story 27.1 LUT/常量初值生成器闭包 API

**Verdict:** Approve

## Findings (introduced then fixed)

1. **Fixed:** `width > 64` was initially rejected for all Mem decls; narrowed to init path only so existing wide Mem without init stays valid.
2. **Accepted (non-blocking):** FIRRTL records init as `; mem-init …` comments (not dialect `mem` init); Verilog `initial` is the primary FR73 visibility surface — sufficient for AC “.v and/or FIRRTL”.
3. **Accepted (deferred):** Capturing hardware Signal/Reg diagnostics remain Story 27.3; comb/seq inlined closures remain Epic 28.

## AC Trace

| AC | Result |
| ---- | ------ |
| Documented `declare_*_with_init_fn` / `generate_mem_init` runs Fn in ElaborateSession | pass |
| Fixture: closure table visible in emit `.v` (+ FIRRTL comment) | pass (`fr73_mem_init_generator`) |
| FrozenHir has no closure residue (NFR36) | pass |
| AD-1/7/13; no rustc-time netlist extract | pass (session-only API) |
| No comb/seq inlined closures required | pass (deferred) |

## Verification

- `cargo test -p bitloom --test fr73_mem_init_generator`
- `cargo test -p bitloom-builder mem_`
- Related: `bitloom-vlog`, `bitloom-sim sync_read_mem`, `rhdl-firrtl --lib`

**Accept**
