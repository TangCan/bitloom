# FR159 — MemRead stub → full functional-sim emit

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 91 / FR159 closed** (Story **91.3**). Implementation Story **91.2**.
Phase 12 FR112 in-process `GeneratedFunctional` MemRead≡tick remains closed (NFR68).

## Selected face (NFR14)

Upgrade `generate_functional_sim` so the **emitted** functional-sim crate implements
real `MemRead` / `MemWrite` with **SyncReadMem latency-1**, aligned with
`GeneratedFunctional` / `Sim::tick` — no longer stubs `MemRead` as `0`.

| Layer | Role |
|-------|------|
| **FR112** | In-process MemRead≡tick — **still closed**; alone ≠ FR159 |
| **FR159** | Emitted crate MemRead full generate — **this page** |

**Forbidden closes:** stub-alone (`MemRead` → `0`); FR112 alone; docs-only.

## Proof obligation

1. Emitted `src/lib.rs` includes `eval_mem_read` + `pending_mem_reads` (SyncReadMem).
2. SyncReadMem fixture: write then read → `rdata` is `0` then `0xAB` under
   `cargo test` on the generated crate (gold
   `gold_sync_read_mem_latency1_write_then_read`).
3. In-process FR112 bridge remains green.

## Recipe

```text
cargo test -p bitloom-sim --lib generate::tests::emit_sync_read_mem_not_stubbed_and_cargo_tests
cargo test -p bitloom --test fr159_memread_full_emit
cargo test -p bitloom --test fr112_memread_equiv_tick
```

## Cross-links

| Doc | Role |
|-----|------|
| [`fr112-generated-functional-memread-equiv.md`](fr112-generated-functional-memread-equiv.md) | FR112 in-process face (still closed; ≠ FR159 alone) |
| [`fr47-dual-sim-generation.md`](fr47-dual-sim-generation.md) | FR47 dual sim generation |
| NFR14 | `_agile-output/implementation-artifacts/nfr14-risk-epic91-memread-full-emit-fr159.md` |

## Non-goals

- Multi-port / banked mem arbitration
- Async-only deepen beyond SyncReadMem MVP
- Verilog/FIRRTL mem semantics deepen
- Claiming NFR59 “fully cleared” before FR157–FR165 all close
