# FR30 — dual-view equivalence (functional vs `tick`)

Bitloom compares a **functional** view and cycle-accurate `Sim::tick` on the **same stimulus**, using **only** `PortValues`. Matching traces return `EquivStatus::Pass`; the first mismatch returns `Fail`.

This is a **bounded checker**, not an SMT solver. Exhaustiveness is the fixture's stimulus set (see `reset_then_run`).

## P3 acceptance = generated path

For Phase-3 / FR47 closure, product acceptance is the **generated** path:

```rust
use bitloom_sim::{check_functional_equiv_generated, reset_then_run};

let status = check_functional_equiv_generated(hir, reset_then_run(8));
assert!(status.is_pass());
```

`check_functional_equiv_generated` drives `GeneratedFunctional` (FR47 leg 1) against `tick` (same as `check_generated_bridge`). A deliberately wrong functional view must fail (SM-7).

Handwritten `AbstractionView` / `check_functional_equiv` **may coexist** (Epic 8 / FR29) but are **not** the P3 gate:

```rust
// Still valid for handwritten fixtures — not sufficient alone for P3 FR30+FR47.
let status = bitloom_sim::check_functional_equiv(hir, &mut abs, bitloom_sim::reset_then_run(8));
```

See also `docs/fr47-dual-sim-generation.md`.
