# FR100 — Automatic FL≡RTL / formal-equivalence product path

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

This page is the **FR100 completion surface** (Epic 45 / Story 45.2). It delivers
documented automatic random/compare **and** an in-tree bounded formal-equivalence
product entry. It is **beyond** FR92 shared-stimulus scoreboard alone.

## F1 tool branch (nailed)

**Selected: F1-(i) — in-tree formal / bounded prover API** (beyond “PortValues
random scoreboard only”).

- Not F1-(ii) external SymbiYosys/SMT-only for this MVP (CI/`just test` must
  reproduce without `sby`). FR85 remains the external SVA checker pattern; it is
  **not** the FR100 FL≡RTL close.
- Bounds: NFR14 F1–F5 in
  `_agile-output/implementation-artifacts/nfr14-risk-epic45-formal-equiv-dual-model.md`.

## Product API (`bitloom-sim`)

Design crates stay on **`bitloom-prelude` only**. Formal-equiv helpers live in the
toolchain crate `bitloom-sim`:

```rust
use bitloom_sim::FormalEquivProduct;

// F3 companion — automatic random/compare (reproducible seed). Supporting, not sufficient alone.
let random = FormalEquivProduct::new(0xC0FFEE, 8).with_boolean_ports(&["rst"]);
assert!(random.check_random_compare(hir.clone()).is_pass());

// F1-(i) product entry — exhaustive FL≡tick over alphabet^depth (beyond random sampling).
let formal = FormalEquivProduct::new(0, 0)
    .with_boolean_ports(&["rst"])
    .with_exhaustive_depth(3);
assert!(formal.check_bounded_exhaustive(hir).is_pass());
```

| Path | API | Role vs FR100 |
|------|-----|----------------|
| Automatic random/compare | `check_random_compare` | F3 **companion**; **非充分** alone (F5) |
| Bounded exhaustive formal | `check_bounded_exhaustive` | **F1-(i) product entry** |
| FR92 scoreboard | `SharedStimulusScoreboard` | Soft dependency / 配套；**不得单独**关闭 FR100 |

Deliberate mismatch → `EquivStatus::Fail { cycle, mismatches }` with readable
`PortMismatch` diagnostics (F2).

### Scope (F4)

MVP = doc-pinned module fixture scale (boolean alphabet × small depth). **Not**
full-chip unbounded SMT proof. Out-of-scope claims must not be marketed as FR100
green.

## Recipe

```text
cargo test -p bitloom --test fr100_formal_equiv_product
```

## FR92 / FR30 are supporting — not sufficient

| Prior path | Status for FR100 |
|------------|------------------|
| FR92 `SharedStimulusScoreboard` | 配套同刺激；**alone ≠ FR100** |
| FR30 bounded fixture checker | 前件；**alone ≠ FR100** |
| This page + `FormalEquivProduct` ATDD | **FR100 完成面** |

Do **not** claim FR92 alone closes FR100.

## Non-goals (historical — closed by later stories)

- FR102 attribute-macro full matrix → [`fr102-multiview-attribute-matrix.md`](fr102-multiview-attribute-matrix.md) (**Story 45.3 / closed**)
- FR103 first-class IP dual-model completeness / Epic 45 closeout → [`fr103-ip-dual-model.md`](fr103-ip-dual-model.md) (**Story 45.4 / Epic 45 closed**)
- SystemC TLM-2.0 product → FR101 / Epic 46 (**closed** — LT-only MVP)
- External SymbiYosys as the sole F1 close for this MVP

## Cross-links

| Doc | Role |
|-----|------|
| [`fr92-shared-stimulus-adapter.md`](fr92-shared-stimulus-adapter.md) | Shared stimulus (supporting) |
| [`fr30-dual-view-equiv.md`](fr30-dual-view-equiv.md) | Bounded PortValues checker precedent |
| [`fr47-dual-sim-generation.md`](fr47-dual-sim-generation.md) | Generated FL vs tick |
| [`fr112-generated-functional-memread-equiv.md`](fr112-generated-functional-memread-equiv.md) | FR112 deepen: MemRead ≡ tick (≠ F1-(i) alone) |
| [`fr39-formal-sva.md`](fr39-formal-sva.md) | SVA / FR85 external checker (≠ FR100) |
| NFR14 Epic 45 | `_agile-output/implementation-artifacts/nfr14-risk-epic45-formal-equiv-dual-model.md` |
