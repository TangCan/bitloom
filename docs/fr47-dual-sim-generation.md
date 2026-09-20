# FR47 — dual simulator generation

Bitloom can **generate** both a Rust functional simulator and a cycle-accurate tick wrapper from `FrozenHir` (AD-5 / PRD FR47). Comparison uses **PortValues** only.

## Leg 1 — functional sim

### API

```rust
use bitloom_sim::{generate_functional_sim, GeneratedFunctional};

let mut abs = GeneratedFunctional::from_hir(&hir);
generate_functional_sim(&hir, out_dir)?;
```

Alias: `emit_functional_crate`.

### CLI

```bash
cargo bitloom gen-func --package counter_ports --out-dir target/bitloom-func-sim
cd target/bitloom-func-sim && cargo test && cargo run
```

## Leg 2 — cycle-accurate + bridge/compare

Cycle-accurate artifact = FrozenHir → `Sim::tick` wrapper crate (not SystemC).

### API

```rust
use bitloom_sim::{
    generate_cycle_accurate_sim, check_generated_bridge, reset_then_run, CycleAccurateSim,
};

generate_cycle_accurate_sim(&hir, out_dir)?;
let status = check_generated_bridge(hir, reset_then_run(3));
assert!(status.is_pass());
```

Deliberate mismatch (wrong functional view) must fail — paving Story 21.5 / FR30:

```rust
let status = check_generated_bridge_with(hir, &mut wrong_abs, reset_then_run(1));
assert!(!status.is_pass());
```

### CLI

```bash
cargo bitloom gen-cycle --package counter_ports --out-dir target/bitloom-cycle-sim
cd target/bitloom-cycle-sim && cargo test && cargo run
```

## Constraints

- Generator lives in the **toolchain** (`bitloom-sim` / `bitloom` CLI). Design crates depend only on `bitloom-prelude`.
- **Not** SystemC / TLM-2.0. SystemC TLM-2.0 product path is **FR101** — see [`fr101-systemc-tlm.md`](fr101-systemc-tlm.md) (**Epic 46 closed**; LT-only MVP; revised **AD-5**).
- **FR30 / P3:** product acceptance uses `check_functional_equiv_generated` on the generated path (see `docs/fr30-dual-view-equiv.md`). Handwritten equiv may coexist.
- **FR78 × FR47 (Story 30.3–30.4):** bridge-adapter `start_wait_complete` templates may drive PortValues stimuli into this same generated-path bridge — see `docs/fr78-bridge-adapter-closures.md`, UJ「桥接半程」[`docs/tutorials/bridge-half.md`](tutorials/bridge-half.md), and `cargo test -p bitloom --test fr78_fr47_dual_view_coverify`. **FR47 ≠ FR78** (sim-crate generation ≠ host handshake template). Not SystemC TLM.
- **FR92 (Wave D / Story 39.4):** product contract for **shared stimulus / scoreboard** + adapter template over this path — see [`docs/fr92-shared-stimulus-adapter.md`](fr92-shared-stimulus-adapter.md) and `SharedStimulusScoreboard`. Does **not** claim automatic formal FL≡RTL / SystemC TLM-2.0.
- **Current subset (2026-09-20 maintenance):** flat **single module without instances**. Cycle and functional generation both support the documented `Mem`/`SyncReadMem` subset and their emitted crates are actually compiled and exercised. Hierarchy is explicitly rejected by both generation entries; in-process infallible constructors panic clearly, and CLI wave/coverage return readable errors. No first-module fallback. See [boundary evidence](backend-boundary-evidence-2026-09-20.md) for read-stage/reset/enable/collision limits. Historical FR47 MVP excluded Mem; subsequent Mem implementation and the 2026-09-20 boundary regressions supersede that restriction, without delivering hierarchical simulation.
