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
- **Not** SystemC / TLM-2.0.
- **FR30 / P3:** product acceptance uses `check_functional_equiv_generated` on the generated path (see `docs/fr30-dual-view-equiv.md`). Handwritten equiv may coexist.
- **MVP subset (locked):** flat **single top module** only. Cycle-accurate emit **rejects** hierarchy instances and memories; functional generate takes `modules.first()`. Hierarchy / `Mem` cycle-accurate generation is a **future story** — do not silently expand the subset without updating this page, `language-surface.md`, and `deferred-work.md`.
