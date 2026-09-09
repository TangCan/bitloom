# FR78 — Bridge adapter closure templates (`start_wait_complete`)

Reusable **host/bridge** helpers that map a transaction-shaped start action onto a
cycle-accurate start→wait→complete handshake (Cap-R-65). Public surface:
`bitloom_prelude::{StartWaitComplete, start_wait_complete}`.

Design crates depend only on **`bitloom-prelude`**. SystemC TLM-2.0 is **not** a
product contract (AD-5 / AD-17); comparison stays on `PortValues`.

## API

### Trait form

```rust
use bitloom_prelude::StartWaitComplete;

impl StartWaitComplete for MyBridge {
    fn is_busy(&self) -> bool { /* e.g. read tx_busy pin */ }
    fn tick(&mut self) { /* drive cycle-accurate sim / settle pins */ }

    // start_wait_complete is provided by default:
    // start_fn → one commit tick → while is_busy { tick }
}

bridge.start_wait_complete(|s| {
    s.tx_data = byte;
    s.tx_start = true;
});
```

The default method is a **documented equivalent** of the requirements sketch
`start_fn; while busy { tick }`: it always performs one tick after `start_fn` so
registered `busy` can assert before the wait loop (common for RTL handshakes).

### Free-function form (documented equivalent)

```rust
use bitloom_prelude::start_wait_complete;

start_wait_complete(
    &mut host,
    |h| h.tick(),
    |h| h.is_busy(),
    |h| {
        h.tx_data = byte;
        h.tx_start = true;
    },
);
```

## View boundary (Cap-R-66 / Cap-R-67 / NFR36)

| Path | Closures | What the cycle-accurate side sees |
|------|----------|-----------------------------------|
| **Functional / bridge host** | Free Rust closures OK (Cap-R-66/67) | N/A — host only |
| **After template expand** | Closure finished on host stack | Ordinary signals / `PortValues` only |
| **`Sim::tick` / FrozenHir / emit** | **No** closure objects (NFR36) | Pins, regs, nets — never `Fn` |

**Do not** treat this template as:

- Elaborate-time **generator** closures (FR73 / Cap-R-53)
- **SynthesizableClosure** comb/seq inline (FR74 / FR75)
- **FR47** sim-crate generators (dual-view codegen ≠ FR78)
- Permission to capture into cycle-accurate `tick` (FR16 / AD-18 / NFR35 still reject)

Risk gate: `_agile-output/implementation-artifacts/nfr14-risk-epic30-bridge-adapter-closures.md`.

## Relation to FR29 / FR47

| Capability | Role |
|------------|------|
| FR29 handwritten `#[bridge]` / `HostView` | Host markers; never enter HIR |
| FR78 `start_wait_complete` | Reusable handshake **inside** host/bridge code |
| FR47 `generate_*` / `check_generated_bridge` | Dual-view **crate** generation + compare (Epic 21+;联验 → Story 30.3) |

## ATDD

`cargo test -p bitloom --test fr78_bridge_adapter_start_wait_complete`
