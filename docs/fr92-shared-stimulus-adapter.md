# FR92 — Multi-view shared stimulus + bridge adapter template

**Wave D / P7 contract (FR87):** functional-sim path and cycle-accurate `tick`
(and/or historical `build-sim` wrappers) share one stimulus / scoreboard fixture;
a bridge adapter **template** is documented and reusable. Brand: **Bitloom**.

This page is the FR92 product contract. It **reuses** FR47 dual-sim generation and
FR78 `start_wait_complete` — it does **not** invent a second uncorrelated
simulation semantics.

## Shared stimulus / scoreboard

API skeleton (`bitloom-sim`, toolchain / test dependency — design crates stay on
`bitloom-prelude` only):

```rust
use bitloom_sim::{SharedStimulusScoreboard, reset_then_run};

let board = SharedStimulusScoreboard::from_stimuli(reset_then_run(4));
// Same Vec<PortValues> drives generated functional view and Sim::tick:
assert!(board.check_generated(hir).is_pass());
```

| Side | Path | Role |
|------|------|------|
| Functional | FR47 `GeneratedFunctional` / handwritten `AbstractionView` | Consume shared stimuli |
| Cycle-accurate | FrozenHir `Sim::tick` (FR47 `generate_cycle_accurate_sim` / host tick) | Consume **the same** stimuli |
| Scoreboard | `SharedStimulusScoreboard` | Holds the shared vector; compares PortValues (AD-17) |

Deliberate mismatch on the same stimuli must **fail** (FR30 spirit). Recipe:

```text
cargo test -p bitloom --test fr92_shared_stimulus_adapter
```

Prior art (technical precedent, not a second product): Story 30.3
`fr78_fr47_dual_view_coverify` — FR78 template-recorded stimuli × FR47 bridge.

Historical `cargo … build-sim --kind …` remains a deferred CLI wrapper around
these libraries ([`docs/fr40-cli-verbs.md`](fr40-cli-verbs.md)); FR92 acceptance
is the shared-stimulus **path**, not the verb matrix.

## Bridge adapter template

Reusable host handshake: **`start_wait_complete`** (FR78 / Cap-R-65).

- Docs: [`docs/fr78-bridge-adapter-closures.md`](fr78-bridge-adapter-closures.md)
- Prelude: `bitloom_prelude::{StartWaitComplete, start_wait_complete}`
- Tutorial: [`docs/tutorials/bridge-half.md`](tutorials/bridge-half.md)

Closures stay on the **host / bridge** stack; cycle-accurate `tick` sees ordinary
pins / `PortValues` only (NFR36 / AD-18). FR92 does not replace FR78 — it names
the Wave D requirement that the adapter template + shared stimulus path exist
together for P7 green.

## Explicit non-goals (AD-5 / NFR38)

**FR92 does not claim and must not be closed as:**

- Automatic formal **FL≡RTL** (or default TLM≡CA) product / proof suite
- **SystemC TLM-2.0** product path (`emit_tlm` / TLM libraries)

Consistency remains random / contrast / PortValues scoreboard tests (AD-5), not
a silent formal-equivalence product.

## Cross-links

| Doc | Role |
|-----|------|
| [`fr47-dual-sim-generation.md`](fr47-dual-sim-generation.md) | Generated functional + cycle crates |
| [`fr78-bridge-adapter-closures.md`](fr78-bridge-adapter-closures.md) | Adapter template |
| [`fr29-bridge-abstraction-both.md`](fr29-bridge-abstraction-both.md) | Handwritten multi-view |
| [`fr30-dual-view-equiv.md`](fr30-dual-view-equiv.md) | Equiv / mismatch spirit |
| [`docs/requirements/19. 实施路线图.md`](requirements/19.%20实施路线图.md) §19.9 | P7 合同绿 |
| NFR14 Epic 39 | `_agile-output/implementation-artifacts/nfr14-risk-epic39-ide-multiview.md` |
