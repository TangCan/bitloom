# FR33 / FR83 — C ABI / cdylib simulation

`rhdl-cabi` is a `cdylib` (plus `rlib` for Rust tests) exporting a **documented generate/link path**: build the crate, link `librhdl_cabi`, include `rhdl_cabi.h`, select a DUT by name.

> **Brand:** public product is **Bitloom**; C symbol prefix remains `rhdl_*` for ABI stability. Unrelated to `samitbasu/rhdl`.  
> **AD-5:** this is the Rust/C shared-library sim path — **not** SystemC TLM-2.0.

## Exported symbols

| Symbol | Role |
|--------|------|
| `rhdl_sim_new` | Opaque handle for legacy **Counter** DUT (FR33) |
| `rhdl_sim_new_dut` | Opaque handle for documented DUT name: `"Counter"` \| `"Adder"` (**FR83**) |
| `rhdl_last_error` | Thread-local last error C string, or `NULL` if none |
| `rhdl_sim_free` | Release handle |
| `rhdl_sim_set` / `rhdl_sim_tick` / `rhdl_sim_get` | Cycle-accurate `tick` |
| `rhdl_abs_cycle` / `rhdl_abs_get` | Handwritten abstraction view |

Header: `crates/rhdl-cabi/include/rhdl_cabi.h`.

## Documented DUTs (FR83 Option A)

| Name | Ports (u8 unless noted) | Golden |
|------|-------------------------|--------|
| `Counter` | `clk`, `rst`, `data_in` → `data_out` | reset pulse then 3 ticks → `data_out == 3` (both views) |
| `Adder` | `clk`, `rst`, `a`, `b` → `sum` | `a=5`, `b=7` → `sum == 12` (both views) |

Harnesses:

- Counter: `crates/rhdl-cabi/tests/harness.c` (`rhdl_sim_new`)
- Adder: `crates/rhdl-cabi/tests/harness_adder.c` (`rhdl_sim_new_dut("Adder")`) — **proves not Counter-only**

Rust goldens: `rhdl_cabi::rust_golden_data_out()` / `rhdl_cabi::rust_golden_adder_sum()`.

## Lifetime / ownership

1. Create with `rhdl_sim_new` or `rhdl_sim_new_dut` → non-null `Handle*` on success.
2. Use only that pointer with `rhdl_sim_*` / `rhdl_abs_*`.
3. Free exactly once with `rhdl_sim_free`. Do not use after free.
4. `rhdl_last_error()` returns a pointer owned by the library for **this thread**; it is invalidated by the next `rhdl_*` call that sets or clears the error. Copy the string if you need it longer.

## Error modes (must not silent-succeed)

| Failure | Result | Diagnosis |
|---------|--------|-----------|
| Unknown DUT name | `rhdl_sim_new_dut` → `NULL` | `rhdl_last_error()` mentions `unknown DUT` and the name |
| Null / non-UTF-8 DUT name | `NULL` | `rhdl_last_error()` set |
| Null handle on set/tick/get/abs | no-op / get returns `0` | `rhdl_last_error()` set (e.g. `null handle`) |
| Unknown / unset port on get | returns `0` | `rhdl_last_error()` set — **do not treat `0` alone as success**; check `rhdl_last_error()` after suspicious gets |

Successful APIs clear the last error (so a prior failure does not linger across a good call).

## Link path (generate / consume)

```text
cargo build -p rhdl-cabi
cc -I crates/rhdl-cabi/include harness.c \
   -L target/debug -lrhdl_cabi -Wl,-rpath,$PWD/target/debug
```

Same link line works for any documented DUT selected at runtime via `rhdl_sim_new_dut`.
