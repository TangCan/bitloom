# FR103 — First-class IP dual-model completeness

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

This page is the **FR103 completion surface** (Epic 45 / Story 45.4). For each
IP class in the NFR14 risk-record set, Bitloom delivers a **runnable functional
model** and a **cycle-accurate model** (or generation path), co-verified with
stimulus / equivalence paths. This is **beyond** FR78/FR92 adapter templates alone.

## Dual-model obligations (NFR14 set)

| IP class | Cycle model | Functional model | Co-verify path |
|----------|-------------|------------------|----------------|
| **FIFO** (`SyncFifo`) | `Elaboratable` → `Sim::tick` (+ `settle` for gated enables) | **Handwritten** [`SyncFifoFunctional`](../crates/bitloom-sim/src/ip_dual.rs) | `IpDualModelMatrix::verify_sync_fifo` |
| **UART** (`UartTx` / `UartRx`) | same | **Generated** `GeneratedFunctional` (FR47) | `IpDualModelMatrix::verify_generated_rst_compare` / `FormalEquivProduct` |
| **SPI** (`SpiMaster`) | same | Generated FL | same |
| **I2C** (`I2cMaster`) | same | Generated FL | same |
| **AXI4-Lite** (`Axi4LiteSlave`) | same | Generated FL | same |
| GPIO / Crc8Lut | — | — | **Not** in FR103 close set |

### Why SyncFifo is handwritten

FR103 nailed **handwritten** [`SyncFifoFunctional`] as the FIFO dual-model
completion face (architectural PortValues). SyncFifo contains `declare_mem`.
**FR112** deepens in-process **GeneratedFunctional MemRead ≡ tick** (SyncReadMem /
Mem) — see [`fr112-generated-functional-memread-equiv.md`](fr112-generated-functional-memread-equiv.md).
That deepen does **not** rewrite FR103: SyncFifo dual-model close remains the
documented handwritten FL (not generated-alone).

## Product API (`bitloom-sim`)

Design crates stay on **`bitloom-prelude` only**. Dual-model helpers live in the
toolchain crate `bitloom-sim`:

```rust
use bitloom_sim::IpDualModelMatrix;
use bitloom_prelude::ip::SyncFifo;
use bitloom_prelude::Elaboratable;

let matrix = IpDualModelMatrix::new();
assert!(matrix.verify_sync_fifo(SyncFifo::elaborate()?).is_pass());
assert!(matrix.verify_generated_rst_compare(UartTx::elaborate()?).is_pass());
```

| Path | Role vs FR103 |
|------|----------------|
| `IpDualModelMatrix` + ATDD | **FR103 完成面** |
| FR78 / FR92 adapter templates | **配套**；**alone ≠ FR103** |
| FR100 `FormalEquivProduct` | Formally co-verify FL≡tick (used for non-mem IPs) |
| FR102 attribute matrix | Orthogonal surface (Story 45.3) |

Deliberate mismatch → `EquivStatus::Fail` with readable `PortMismatch` diagnostics.

## Recipe

```text
cargo test -p bitloom --test fr103_ip_dual_model
cargo test -p bitloom --test fr103_epic45_closeout
```

## Epic 45 closeout

Closing FR103 with this page + ATDD also closes **Epic 45** (FR100 + FR102 + FR103):

- FR100 formal-equiv product — [`fr100-formal-equiv.md`](fr100-formal-equiv.md)
- FR102 multi-view attribute matrix — [`fr102-multiview-attribute-matrix.md`](fr102-multiview-attribute-matrix.md)
- FR103 IP dual-model — **this page**

**SystemC TLM-2.0 product path is Epic 46 / FR101** (revised AD-5; **Epic 46 closed** —
LT-only MVP). Closing Epic 45 does **not** by itself deliver TLM product (that is a
separate epic, now closed). Default TLM≡CA / automatic formal equivalence is **no
longer** a permanent non-goal (FR100 delivered). 「不承诺 SystemC TLM」is **no longer**
a product completion exclusion.

## Non-goals (this story)

- SystemC TLM-2.0 product → FR101 / Epic 46 (**closed** separately; LT-only; AT deferred)
- GPIO VIP (optional at FR98 G1; **near-VIP delivered as FR108 / Epic 50** — Story 50.3; commercial GPIO VIP still out of scope)
- Full-chip unbounded formal proof
- **FR112** GeneratedFunctional MemRead ≡ tick deepen → [`fr112-generated-functional-memread-equiv.md`](fr112-generated-functional-memread-equiv.md) (**Epic 54 closed** / Story 54.3; does not rewrite this FR103 SyncFifo handwritten face)

## Cross-links

| Doc | Role |
|-----|------|
| [`ip/README.md`](ip/README.md) | First-class IP index (FR98 near-VIP) |
| [`fr100-formal-equiv.md`](fr100-formal-equiv.md) | Formal FL≡RTL product |
| [`fr102-multiview-attribute-matrix.md`](fr102-multiview-attribute-matrix.md) | Attribute matrix |
| [`fr112-generated-functional-memread-equiv.md`](fr112-generated-functional-memread-equiv.md) | FR112 MemRead ≡ tick deepen |
| [`fr47-dual-sim-generation.md`](fr47-dual-sim-generation.md) | Generated FL / cycle crates |
| [`fr92-shared-stimulus-adapter.md`](fr92-shared-stimulus-adapter.md) | Shared stimulus (supporting) |
| NFR14 Epic 45 | `_agile-output/implementation-artifacts/nfr14-risk-epic45-formal-equiv-dual-model.md` |
