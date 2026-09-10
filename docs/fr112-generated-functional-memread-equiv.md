# FR112 — Formal / dual-model depth (GeneratedFunctional MemRead ≡ tick)

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 54 / FR112 closed** (Story **54.3**). Branch **(B)** delivered in Story **54.2**.
Phase 12 FR100 F1-(i) and FR103 SyncFifo handwritten FL MVP remain closed (NFR44).
Branches (A) SymbiYosys and (C) more handwritten IP FL stay **deferred** (NFR47 — need new contract).

This page is the **FR112 completion surface** for Epic 54 **branch (B)**.
It deepens dual-model honesty **beyond** Phase 12 FR100 F1-(i) bounded exhaustive and
FR103 SyncFifo handwritten FL MVP.

## Selected deepen branch (NFR14)

| Branch | Status |
|--------|--------|
| **(B) GeneratedFunctional MemRead ≡ tick** | **FR112 face — closed** (this page) |
| (A) F1-(ii) SymbiYosys/SMT | **deferred** — needs new contract |
| (C) More first-class IP handwritten FL | **deferred** — needs new contract |

**Forbidden closes:** FR92 scoreboard alone; FR78/FR92 adapter template alone;
FR100 F1-(i) alone; FR103 SyncFifo MVP alone; ≠ FR107 SystemC AT; docs-only.

## Proof obligation

In-process [`GeneratedFunctional`](../crates/bitloom-sim/src/generate.rs) matches
cycle-accurate [`Sim::tick`](../crates/bitloom-sim/src/lib.rs) on a SyncReadMem
fixture (latency-1 read): same PortValues after documented stimuli via
`check_generated_bridge`.

- **Tooling:** in-tree Bitloom (`just test` / `cargo test`) — no SymbiYosys required.
- **Emitted crate** (`generate_functional_sim`) may still stub `MemRead` as `0`;
  the FR112 face is the **in-process** view used by `check_generated_bridge`.

## Recipe

```text
cargo test -p bitloom --test fr112_memread_equiv_tick
cargo test -p bitloom --test fr112_epic54_closeout
cargo test -p bitloom-sim --lib generate::tests::generated_functional_sync_read_mem_matches_tick
```

## Negative / readable fail

A deliberate FL that ignores MemRead latency (always drives `rdata=0`) must
`EquivStatus::Fail` with readable mismatches — must not silent-Ok as FR112.

## Cross-links

| Doc | Role |
|-----|------|
| [`fr100-formal-equiv.md`](fr100-formal-equiv.md) | Phase 12 F1-(i) MVP — still closed (NFR44); ≠ FR112 alone |
| [`fr103-ip-dual-model.md`](fr103-ip-dual-model.md) | SyncFifo handwritten FL MVP — still closed; ≠ FR112 alone |
| NFR14 | `_agile-output/implementation-artifacts/nfr14-risk-epic54-formal-dual-model-depth.md` |

## Non-goals (NFR47)

- F1-(ii) SymbiYosys/SMT product entry (branch A)
- Expanding handwritten FL beyond SyncFifo for more IP classes (branch C)
- Claiming FR100 F1-(i) or FR103 SyncFifo MVP alone closes FR112
