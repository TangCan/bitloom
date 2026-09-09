# CDC 深度 — FR79 DoubleFlop / SyncFIFO follow-along

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

This walkthrough is the **UJ-level「CDC 深度」** close-out for Epic 31 / FR79
(AD-29): synthesizable synchronizer RTL + documented tick goldens — **not** the
Epic 7 / FR52 minimal contract (`mark_cdc_bridge` only).

Design crates depend only on **`bitloom-prelude`**.

## Historical minimal vs true RTL (NFR37)

| Contract | What it proves | Fixture |
|----------|----------------|---------|
| **FR52 / Epic 7** | phantom `ClockDomain` + illegal CDC → `E0220` + `mark_cdc_bridge` | `examples/clockdomain_skel` |
| **FR79 / AD-29** | emit **recognisable sync/FIFO RTL** + per-domain tick goldens | `examples/doubleflop_skel` · `examples/syncfifo_skel` |

`DoubleFlop` / `SyncFIFO` may still be **zero-sized Rust types**; FR79 depth is
judged by **elaborate → emit netlist + tick**, not by `size_of`. Do **not** close
FR79 with FR52/`done` talk alone.

## Fixture A — DoubleFlop goldens (Story 31.2)

```bash
cargo test -p bitloom --test fr79_doubleflop_rtl
cargo test -p doubleflop_skel
```

Expect: two-stage `sync_ff*` regs in `.v`; `dout` follows after
`LATENCY_DST_TICKS = 2` destination-domain ticks (`Sim::tick` MVP); illegal
unsync cross-domain still `rhdl::E0220`.

Docs: [`docs/fr79-doubleflop-cdc.md`](../fr79-doubleflop-cdc.md).

## Fixture B — SyncFIFO goldens (Story 31.3)

```bash
cargo test -p bitloom --test fr79_syncfifo_rtl
cargo test -p syncfifo_skel
```

Expect: mem + gray-pointer DoubleFlop sync + `full`/`empty`; documented
DEPTH=4 / WIDTH=8; cross-domain write→read under ptr-sync latency; ≠
`ip::SyncFifo` (FR82).

Docs: [`docs/fr79-syncfifo-cdc.md`](../fr79-syncfifo-cdc.md).

## Contrast —「仅 ZST」/ FR52 bridge (testable)

```bash
cargo test -p clockdomain_skel
```

`clockdomain_skel` uses `mark_cdc_bridge` **without** emitting `sync_ff*` —
that is the **historical minimal** path. Close-out ATDD asserts FR79 elaborates
produce real regs/mem while the FR52 emit does **not**.

## Close-out ATDD + contributor recipe

Documented Epic 31 depth matrix (Story 31.4):

```bash
cargo test -p bitloom --test fr79_cdc_depth_closeout
cargo test -p bitloom --test fr79_doubleflop_rtl --test fr79_syncfifo_rtl
```

Full workspace (contributors): `just test` (or `cargo test --workspace`).
This story does **not** require `cargo clean && just test` as a gate.

## Cross-links

| Topic | Doc |
|-------|-----|
| DoubleFlop true RTL | [`docs/fr79-doubleflop-cdc.md`](../fr79-doubleflop-cdc.md) |
| SyncFIFO true RTL | [`docs/fr79-syncfifo-cdc.md`](../fr79-syncfifo-cdc.md) |
| Language surface (CAP-11 / FR79) | `_agile-output/specs/spec-rhdl/language-surface.md` |
| Risk gate / Epic 31 close | `nfr14-risk-epic31-cdc-true-rtl.md` |
| FR52 product skel | `examples/clockdomain_skel` |

## Out of scope here

- Silicon metastability / MTBF sign-off (documented non-contract).
- Dual physical `wr_clk`/`rd_clk` product matrix; arbitrary DEPTH/WIDTH.
- Confusing language-level `SyncFIFO` with `ip::SyncFifo` (FR82).
