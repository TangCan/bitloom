# FR78 — Bridge adapter closure templates (`start_wait_complete`)

Reusable **host/bridge** helpers that map a transaction-shaped start action onto a
cycle-accurate start→wait→complete handshake (Cap-R-65). Public surface:
`bitloom_prelude::{StartWaitComplete, start_wait_complete}`.

Design crates depend only on **`bitloom-prelude`**. SystemC TLM-2.0 is **not** a
product contract (AD-5 / AD-17); comparison stays on `PortValues`.

**UJ「桥接半程」跟练：** [`docs/tutorials/bridge-half.md`](tutorials/bridge-half.md)
（Story 30.4 文档收口）。

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

Risk gate: `_agile-output/implementation-artifacts/nfr14-risk-epic30-bridge-adapter-closures.md`.

## Terminology disambiguation (Story 30.4)

Aligned with Wave 0 decision table §3 — **four rows must not share one done definition**:

| Term | Meaning | Not |
|------|---------|-----|
| **生成器闭包** (Epic 27 / FR73) | Elaborate-time non-capturing `Fn` dissolved to Mem/ROM/factory HIR before freeze | This bridge template; FR47 crates |
| **FR47 sim generators** | Toolchain generates functional / cycle-accurate **sim crates** from FrozenHir | User `Fn` expanding hardware; FR78 |
| **Phase 7「闭环」** | Overview close-out (Epic 19–24); English *closure* = closed loop | Controlled generic closures (Phase 9) |
| **本模板 / FR78** | Host `start_wait_complete` (or documented equivalent); free closures on host → ordinary signals on tick | FR73 / FR74–75 / FR47 / Phase 7 |

Also not this template: **SynthesizableClosure** comb/seq inline (Epic 28 / FR74–75);
permission to capture into cycle-accurate `tick` (FR16 / AD-18 / NFR35 still reject).

## Cross-links — Epic 27 / 28 / dual-view

| Capability | Role | Doc entry |
|------------|------|-----------|
| FR73 generator closures (Epic 27) | Elaborate-time Mem init / factory `Fn` → HIR before freeze | README「Elaborate-time Mem init」· [`fr22-construct-bar.md`](fr22-construct-bar.md) |
| FR74/FR75 synthesizable closures (Epic 28) | Comb/seq `inline_*_fn` under `SynthesizableClosure` | README「可综合闭包」· [`fr22-construct-bar.md`](fr22-construct-bar.md) |
| FR29 handwritten `#[bridge]` / `HostView` | Host markers; never enter HIR | [`fr29-bridge-abstraction-both.md`](fr29-bridge-abstraction-both.md) |
| FR78 `start_wait_complete` | Reusable handshake **inside** host/bridge code | This page · [`tutorials/bridge-half.md`](tutorials/bridge-half.md) |
| FR47 `generate_*` / bridge compare | Dual-view **crate** generation + PortValues compare | [`fr47-dual-sim-generation.md`](fr47-dual-sim-generation.md) |

**Story 30.3 co-verification:** template-recorded stimuli (free closures on the host) feed the FR47 generated-path bridge via `check_functional_equiv_generated` / `check_generated_bridge`. Matching views pass; a deliberate wrong abstraction fails (FR30 spirit). Do not invent a parallel transaction semantics.

## ATDD

- Template API: `cargo test -p bitloom --test fr78_bridge_adapter_start_wait_complete`
- FR47 dual-view co-verification (Story 30.3): `cargo test -p bitloom --test fr78_fr47_dual_view_coverify`
- Docs / UJ「桥接半程」收口 (Story 30.4): `cargo test -p bitloom --test fr78_bridge_half_followalong`
