# FR129 — CIRCT Handshake dialect / multi-clock elastic buffers

**Product:** Bitloom (`cargo bitloom` / `bitloom::hls`). Unrelated to `samitbasu/rhdl`.

**Status:** Product path + AD-25 revise delivered in Story **69.2** (Epic 69 close → Story **69.3**).

Beyond FR121 ready/valid alone: CIRCT Handshake dialect IR markers and multi-clock
elastic buffers.

## Contract (NFR14 C1–C4)

| # | Gate | Product evidence |
|---|------|------------------|
| **C1** | CIRCT Handshake dialect subset | schedule IR: `handshake.func` / `handshake.buffer` + `"fr129": true` |
| **C2** | Multi-clock elastic | `clock_domains ≥ 2` + `elastic_depth ≥ 1` |
| **C3** | API + CLI | `schedule_circt_handshake` / `meets_fr129_circt_handshake`; `--circt-handshake` |
| **C4** | AD-25 + ATDD | ARCHITECTURE-SPINE AD-25 Revised 2026-09-10 / FR129; this ATDD |

**FR95/96 alone ≠ FR129. FR110 alone ≠ FR129. FR121 ready/valid alone ≠ FR129. Docs-only ≠ FR129. Unrevised AD-25 ≠ FR129.**

## API

```rust
use bitloom::hls::{schedule_circt_handshake, HlsDataflowOp};

let art = schedule_circt_handshake("hs_circt", HlsDataflowOp::AddConst(1), 1, 2, 2)?;
assert!(art.schedule_ir.contains("\"fr129\": true"));
assert!(art.rtl_stub.contains("clk0") && art.rtl_stub.contains("clk1"));
```

CLI:

```text
cargo bitloom hls --circt-handshake --function hs_circt --dataflow add1 --channels 1 --clock-domains 2 --elastic-depth 2
```

FR121 path unchanged: `--handshake`.

## Cross-links

- FR121: [`fr121-handshake-default.md`](fr121-handshake-default.md)
- NFR14: `_agile-output/implementation-artifacts/nfr14-risk-epic69-circt-handshake.md`
- AD-25: ARCHITECTURE-SPINE (Story 69.2 revised for FR129)

## Non-goals (NFR55)

- Full CIRCT/MLIR lower suite
- Full allocation/binding commercial optimizer suite
- Arbitrary clock-topology auto synthesis

```text
cargo test -p bitloom --test fr129_circt_handshake
```
