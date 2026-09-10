# FR121 — Handshake / dynamic dataflow default synthesizable

**Product:** Bitloom (`cargo bitloom` / `bitloom::hls`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 62 / FR121 closed** (Story **62.3**). Product path + AD-25 revise delivered in Story **62.2**. FR95/96 MVP and FR110 Q1+Q2 remain closed (NFR48). Full allocation/binding suite still needs a new contract (NFR51).

## Contract (NFR14 H1–H4)

| # | Gate | Product evidence |
|---|------|------------------|
| **H1** | Handshake / dynamic DF as documented **default synthesizable** semantics | ready/valid channels; CLI `--handshake` |
| **H2** | AD-18 dissolve before schedule | `schedule_handshake_from_transform`; capturing → readable Err |
| **H3** | Emit / acceptance | schedule IR `fr121` + `handshake` + `semantics=handshake-dynamic-df`; RTL ready/valid ports |
| **H4** | Failure semantics | `channels=0` / capturing fail readably; no silent `fr121` on FR95/FR110 alone |
| **AD** | Revise AD-25 | ARCHITECTURE-SPINE AD-25 Revised 2026-09-10 / FR121 |

**FR95 loop-unroll / `in-tree-mvp` alone ≠ FR121. FR110 Q1+Q2 alone ≠ FR121. Docs-only ≠ FR121. Unrevised AD-25 ≠ FR121.**

## API

```rust
use bitloom::hls::{schedule_handshake_default, schedule_handshake_from_transform, HlsDataflowOp};

let art = schedule_handshake_default("hs_add1", HlsDataflowOp::AddConst(1), 1)?;
assert!(art.schedule_ir.contains("\"fr121\": true"));
assert!(art.rtl_stub.contains("valid") && art.rtl_stub.contains("ready"));

let art2 = schedule_handshake_from_transform("hs_xor", &[], 1, || HlsDataflowOp::XorConst(0xa5))?;
```

CLI:

```text
cargo bitloom hls --handshake --function hs_add1 --dataflow add1 --channels 1
```

Static FR95/FR110 paths unchanged: `--in-tree` / `--in-tree --pipeline`.

## Cross-links

- MVP FR95/96: [`fr35-hls.md`](fr35-hls.md)
- Commercial depth FR110: [`fr110-hls-commercial-depth.md`](fr110-hls-commercial-depth.md)
- NFR14: `_agile-output/implementation-artifacts/nfr14-risk-epic62-handshake-default.md`
- AD-25: ARCHITECTURE-SPINE (Story 62.2 revised for FR121)

## Non-goals (NFR51 → Phase 15)

- Full CIRCT Handshake dialect / multi-clock elastic → **FR129** / [`fr129-circt-handshake.md`](fr129-circt-handshake.md) (**Epic 69 closed**)
- Full allocation/binding commercial HLS optimizer suite
- Forcing Handshake as the only legal path (FR95/FR110 must remain regressable)

```text
cargo test -p bitloom --test fr121_handshake_default
cargo test -p bitloom --test fr121_epic62_closeout
```
