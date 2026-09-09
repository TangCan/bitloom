# FR102 — Multi-view attribute full matrix

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

This page is the **FR102 completion surface** (Epic 45 / Story 45.3). It delivers
the documented multi-view attribute / surface **full matrix** with legal and
illegal combination gates. It is **beyond** FR78 / FR92 adapter templates alone.

Authority: NFR14 risk record
`_agile-output/implementation-artifacts/nfr14-risk-epic45-formal-equiv-dual-model.md`
(property-macro matrix inventory).

## Completion surface (nailed)

| Attribute / surface | Role | Enters HIR / `freeze`? | Matrix duty |
|---------------------|------|------------------------|-------------|
| `#[rhdl::functional_model]` | Host functional model (`cycle`) | **No** | Required row |
| `#[rhdl::abstraction]` | Untimed / transaction-shaped abstraction | **No** | Required row |
| `#[functional_state]` / `#[rhdl::functional_state]` | Soft / functional-side state on a module or host view | **No** (must not leak) | Required row + negative ATDD |
| `#[rhdl::bridge]` | Pin ↔ abstraction adapter | **No** | Required; covers legal combos with abstraction / both |
| `#[rhdl::both]` | Mixed fixture (RTL + handwritten view) | **No** | Required; mixed path |
| FR78 / FR92 adapter **templates** | Reusable handshake skeleton | N/A | **Supporting**; **alone ≠ FR102** |

Design crates depend only on **`bitloom-prelude`** (AD-6). Attributes are re-exported
via `bitloom_prelude::rhdl`.

### `functional_state` surface (field attribute)

Stable Rust has no field-only proc-macro. `#[functional_state]` /
`#[rhdl::functional_state]` is an **inert field attribute** recognized by:

- `#[rhdl::module]` — field remains on the Rust struct; **skipped** when registering
  HIR ports; attribute stripped on expand.
- HostView macros (`functional_model` / `abstraction` / `bridge` / `both`) — attribute
  stripped on expand so host soft fields stay ordinary Rust.

Soft fields **must not** appear in the FrozenHir port table (AD-5 / AD-18).

## Legal combinations

| Combination | Status | Notes |
|-------------|--------|-------|
| `functional_model` alone | Legal | Host `cycle` vs `tick` on `PortValues` |
| `abstraction` + `bridge` | Legal | FR29 handwritten path |
| `both` owning RTL + abstraction (+ optional bridge) | Legal | Mixed fixture |
| `module` ports + `functional_state` soft fields | Legal | Soft fields host-only |
| HostView type + `functional_state` fields | Legal | Soft state on host model |
| FR78 `start_wait_complete` inside bridge / host | Legal **supporting** | Not a substitute for this matrix |

## Illegal combinations (gates)

| Illegal pattern | Gate |
|-----------------|------|
| `functional_state` (or equiv.) appearing on the synthesizable / FrozenHir path as a port or HIR node | Implementation skips port registration; ATDD asserts soft names absent from FrozenHir |
| Claiming FR102 done with **only** FR78 / FR92 adapter templates | Docs + ATDD honesty guards — templates are **配套非充分** |
| Undocumented attribute that silently no-ops while claiming the matrix is complete | Matrix rows above are the only FR102 completion inventory |

## Recipe

```text
cargo test -p bitloom --test fr102_multiview_attribute_matrix
```

## Supporting — not sufficient

| Prior path | Role vs FR102 |
|------------|---------------|
| FR29 handwritten bridge / abstraction / both | Rows in this matrix (required) |
| FR78 `start_wait_complete` / FR92 adapter docs | **Supporting**; **alone ≠ FR102** |
| This page + `functional_state` surface + ATDD | **FR102 完成面** |

Do **not** claim adapter templates alone close FR102.

## Non-goals (this story)

- FR103 first-class IP dual-model completeness / Epic 45 closeout → [`fr103-ip-dual-model.md`](fr103-ip-dual-model.md) (**Story 45.4 / Epic 45 closed**)
- SystemC TLM-2.0 product → FR101 / Epic 46
- Allowing `functional_state` into HIR / `freeze` (forbidden unless NFR14 revises)

## Cross-links

| Doc | Role |
|-----|------|
| [`fr29-bridge-abstraction-both.md`](fr29-bridge-abstraction-both.md) | Handwritten HostView rows |
| [`fr78-bridge-adapter-closures.md`](fr78-bridge-adapter-closures.md) | Adapter template (supporting ≠ FR102) |
| [`fr92-shared-stimulus-adapter.md`](fr92-shared-stimulus-adapter.md) | Shared stimulus / adapter (supporting) |
| [`fr100-formal-equiv.md`](fr100-formal-equiv.md) | FR100 formal-equiv product (sibling) |
| NFR14 Epic 45 | `_agile-output/implementation-artifacts/nfr14-risk-epic45-formal-equiv-dual-model.md` |
