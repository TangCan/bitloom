# FR157 — Automatic FSM state-label extraction

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.  
**Status:** Epic 89 / Story **89.2** (implementation). Closeout → Story **89.3**.  
**NFR14:** `_agile-output/implementation-artifacts/nfr14-risk-epic89-auto-fsm-labels-fr157.md`.

## Contract (MVP)

| # | Item | Product evidence |
|---|------|------------------|
| **Input** | Design Rust source with `#[bitloom::fsm]` or `#[rhdl::fsm]` on a **unit-variant** `enum` | Macro + offline extract |
| **Output** | Label set (`FSM_ID` + variant names); registry lines `fsm:<id>:<label>` | [`FsmLabels`] / `extract_fsm_labels_from_source` |
| **Feed FR109** | `sim.register_fsm_states(E::FSM_ID, E::state_labels().iter().copied())` | Unchanged visit API |
| **Failure** | Missing annotation / empty variants / non-unit variants → explicit error | `compile_error!` or `FsmExtractError` |

**FR109 alone ≠ FR157.** FR109 still requires an explicit label set; FR157 is **how** that set is obtained from annotated source.

## Design annotation

```rust
use bitloom_prelude::bitloom;
use bitloom_prelude::FsmLabels;

#[bitloom::fsm(name = "demo")] // name optional; default = enum ident
enum Demo {
    Idle,
    Busy,
    Done,
}

assert_eq!(Demo::FSM_ID, "demo");
assert_eq!(Demo::state_labels(), &["Idle", "Busy", "Done"]);
```

Transitional alias: `#[rhdl::fsm]`.

## Offline extract (tooling / CLI package)

```rust
use bitloom::fsm_labels::extract_fsm_labels_from_source;

let sets = extract_fsm_labels_from_source(src)?;
// sets[0].to_registry_lines() → ["fsm:demo:Idle", ...]
```

Design crates **must not** depend on the `bitloom` CLI package (AD-6) — use prelude + `FsmLabels` at compile time.

## Non-goals (NFR71)

- Unlabeled / heuristic FSM inference from arbitrary RTL
- Waveform / VCD reverse engineering
- Third-party LCOV GUI (→ **FR158**)
- Replacing FR109 visit recording / coverage v3 dialect

## Cross-links

- FR109: [`fr109-fsm-state-visit-coverage.md`](fr109-fsm-state-visit-coverage.md)
- Phase 19: README / `deferred-work.md`

```text
cargo test -p bitloom --test fr157_auto_fsm_labels
```
