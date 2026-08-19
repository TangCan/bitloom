# FR32 — interpreter vs compiled `tick`

`rhdl_sim::Sim::with_engine(hir, TickEngine)` selects how each cycle is evaluated:

| Name | Meaning |
|------|---------|
| `interpreter` (default) | Walk FrozenHir processes every `tick` |
| `compiled` | Linearize sequential then combinational assigns once at construction |

Both engines must emit the **same `PortValues` sequence** on the same stimulus (`interpreter_and_compiled_portvalues_match`).

```rust
use rhdl_sim::{Sim, TickEngine};
let mut sim = Sim::with_engine(hir, TickEngine::from_name("compiled").unwrap());
```

CLI listing (does not run a sim):

```bash
cargo run -p bitloom -- sim-engines
```
