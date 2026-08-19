# FR30 — dual-view equivalence (functional vs `tick`)

`rhdl_sim::check_functional_equiv` drives the **same stimulus** through:

1. cycle-accurate `Sim::tick` on a `FrozenHir`
2. a handwritten `AbstractionView::cycle`

It compares **only** `PortValues`. Matching traces return `EquivStatus::Pass`; the first mismatch returns `Fail`.

This is a **bounded checker**, not an SMT solver. Exhaustiveness is the fixture's stimulus set (see `reset_then_run`).

```rust
let status = rhdl_sim::check_functional_equiv(hir, &mut abs, rhdl_sim::reset_then_run(8));
assert!(status.is_pass());
```

A deliberately wrong abstraction must fail (Story 8.2 AC).
