# Code Review — Story 35.2 (FR83 C ABI beyond Counter)

**Verdict: Approve**

**Date:** 2026-09-09  
**Scope:** `rhdl-cabi` DUT select + Adder fixture + docs + ATDD

## Summary

FR83 Option A is landed: documented second DUT `Adder`, `rhdl_sim_new_dut` generate/link select path, C/Rust fixtures proving non-Counter-only, and `rhdl_last_error` diagnosis. No blocking defects remain after harness lifetime fix.

## AC checklist

| AC | Result |
| --- | --- |
| Given 35.1; second DUT **or** generate/link path (FR83) | **pass** — `Adder` + `rhdl_sim_new_dut("Counter"\|"Adder")` + docs link recipe |
| Automated fixture proves not Counter-only | **pass** — `harness_adder.c`, `c_harness_adder_proves_not_counter_only`, unit goldens |
| Docs: symbols, lifetime, error modes | **pass** — `docs/fr33-c-abi.md` |
| Failures diagnose clearly (not silent success) | **pass** — unknown DUT → null + last_error; unknown port → 0 + last_error |

## Findings

1. **Fixed:** `harness_adder.c` previously printed `rhdl_last_error()` after `rhdl_sim_free`, which can invalidate the thread-local string. Checks now run before free.
2. **Non-blocking:** `rhdl_sim_get` still returns `0` for missing ports (ABI compat); diagnosis requires checking `rhdl_last_error` — documented.
3. **Non-blocking:** Arbitrary FrozenHir load-from-file via C is deferred (story deferred list); not required to close FR83 Option A.
4. **Non-blocking:** NFR14 Epic 35 close checklist FR83 marked Option A; FR84/85 remain open for 35.3–35.4.

## Verification run (targeted; no full `just test`)

- `cargo test -p rhdl-cabi` — 5 unit + 3 integration OK
- `cargo test -p bitloom --test fr83_c_abi_beyond_counter` — 4 OK
- `cargo test -p bitloom --test nfr14_risk_epic35_residual_partials` — OK

## testarch-automate

Existing ATDD + cabi unit/integration coverage is sufficient for FR83; no additional automate pass required.
