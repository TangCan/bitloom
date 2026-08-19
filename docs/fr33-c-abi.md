# FR33 — C ABI / cdylib simulation

`rhdl-cabi` is a `cdylib` (plus `rlib` for Rust tests) exporting:

| Symbol | Role |
|--------|------|
| `rhdl_sim_new` / `rhdl_sim_free` | Opaque handle |
| `rhdl_sim_set` / `rhdl_sim_tick` / `rhdl_sim_get` | Cycle-accurate `tick` |
| `rhdl_abs_cycle` / `rhdl_abs_get` | Handwritten abstraction |

Header: `crates/rhdl-cabi/include/rhdl_cabi.h`. Harness: `crates/rhdl-cabi/tests/harness.c`.

Golden: reset pulse then 3 ticks → `data_out == 3` on **both** views (same as `rhdl_cabi::rust_golden_data_out()`).
