# FR112 — Formal / dual-model depth (GeneratedFunctional MemRead ≡ tick)

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 54 / FR112 closed** (Story **54.3**). Branch **(B)** delivered in Story **54.2**.
Phase 12 FR100 F1-(i) and FR103 SyncFifo handwritten FL MVP remain closed (NFR44).
Branches (A) SymbiYosys → **FR119 / Epic 60** ([`fr119-symbiyosys-smt.md`](fr119-symbiyosys-smt.md));
(C) more handwritten IP FL stay **deferred** (NFR51 — need new contract).

This page is the **FR112 completion surface** for Epic 54 **branch (B)**.
It deepens dual-model honesty **beyond** Phase 12 FR100 F1-(i) bounded exhaustive and
FR103 SyncFifo handwritten FL MVP.

## Selected deepen branch (NFR14)

| Branch | Status |
|--------|--------|
| **(B) GeneratedFunctional MemRead ≡ tick** | **FR112 face — closed** (this page) |
| (A) F1-(ii) SymbiYosys/SMT | **FR119 / Epic 60** — [`fr119-symbiyosys-smt.md`](fr119-symbiyosys-smt.md) (≠ this page) |
| (C) More first-class IP handwritten FL | **deferred** — needs new contract (NFR51) |

**Forbidden closes:** FR92 scoreboard alone; FR78/FR92 adapter template alone;
FR100 F1-(i) alone; FR103 SyncFifo MVP alone; ≠ FR107 SystemC AT; docs-only.

## Proof obligation

In-process [`GeneratedFunctional`](../crates/bitloom-sim/src/generate.rs) matches
cycle-accurate [`Sim::tick`](../crates/bitloom-sim/src/lib.rs) on a SyncReadMem
fixture (latency-1 read): same PortValues after documented stimuli via
`check_generated_bridge`.

- **Tooling:** in-tree Bitloom (`just test` / `cargo test`) — no SymbiYosys required.
- **Emitted crate** MemRead full generate is **FR159** — see
  [`fr159-memread-full-emit.md`](fr159-memread-full-emit.md).
  The FR112 face remains the **in-process** view used by `check_generated_bridge`
  (FR112 alone ≠ FR159).

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
| [`fr119-symbiyosys-smt.md`](fr119-symbiyosys-smt.md) | FR119 / Epic 60 SymbiYosys path (former branch A; ≠ this page) |
| [`fr159-memread-full-emit.md`](fr159-memread-full-emit.md) | FR159 emitted MemRead full generate (≠ this FR112 page alone) |
| NFR14 | `_agile-output/implementation-artifacts/nfr14-risk-epic54-formal-dual-model-depth.md` |

## Non-goals (this FR112 page)

- F1-(ii) SymbiYosys/SMT product entry — **not on this page**; delivered under **FR119** ([`fr119-symbiyosys-smt.md`](fr119-symbiyosys-smt.md))
- Expanding handwritten FL beyond SyncFifo for more IP classes (branch C) — **deferred** (NFR51)
- Claiming FR100 F1-(i) or FR103 SyncFifo MVP alone closes FR112
