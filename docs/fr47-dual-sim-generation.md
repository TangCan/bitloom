# FR47 — dual simulator generation (leg 1: Rust functional sim)

Bitloom can **generate** a Rust functional-simulator crate from `FrozenHir` (AD-5 / PRD FR47).

## API

```rust
use bitloom_sim::{generate_functional_sim, GeneratedFunctional};

// In-process model (same semantics as the emitted crate):
let mut abs = GeneratedFunctional::from_hir(&hir);

// Write a standalone crate (Cargo.toml + lib + main) for `cargo test` / `cargo run`:
generate_functional_sim(&hir, out_dir)?;
```

Alias: `emit_functional_crate`.

## CLI

```bash
cargo bitloom gen-func --package counter_ports --out-dir target/bitloom-func-sim
cd target/bitloom-func-sim && cargo test && cargo run
```

The generator lives in the **toolchain** (`bitloom-sim` / `bitloom` CLI). Design crates continue to depend only on `bitloom-prelude`.

## Gold fixture

For a counter-style HIR (`count++`, `data_out = count`), after reset + three cycles the generated model yields `PortValues` with `data_out == 3`.

## Non-goals

- **Not** SystemC / TLM-2.0 (never contracted).
- Cycle-accurate artifact + bridge/compare → Story 21.4.
- FR30 product equiv on the **generated** path → Story 21.5 (handwritten equiv remains in `docs/fr30-dual-view-equiv.md`).
