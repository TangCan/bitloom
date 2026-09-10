# FR110 — In-tree HLS commercial depth

**Product:** Bitloom (`cargo bitloom` / `bitloom::hls`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 52 / FR110 closed** (Story **52.3**). Q1+Q2 depth path delivered in Story **52.2**. FR95/96 MVP remains closed (NFR44). Undeclared full HLS suite still needs a new contract (NFR47).

## Contract (NFR14 Q1–Q2)

| # | Gate | Product evidence |
|---|------|------------------|
| **Q1** | `pipeline_stages >= 2` | Schedule IR field + ATDD |
| **Q2** | Initiation interval `ii` | Schedule IR `"ii"` (+ `initiation_interval`) |
| **Marker** | `fr110: true` | Only when Q1+Q2 met |

**FR95 loop-unroll / `in-tree-mvp` alone ≠ FR110. Bambu stub alone ≠ FR110. Docs-only ≠ FR110.**

## API

```rust
use bitloom::hls::{schedule_in_tree_fr110, HlsDataflowOp};

let art = schedule_in_tree_fr110("pipe_add1", HlsDataflowOp::AddConst(1), 1, 2)?;
assert!(art.schedule_ir.contains("\"fr110\": true"));
assert!(art.schedule_ir.contains("\"ii\": 1"));
assert!(art.schedule_ir.contains("\"pipeline_stages\": 2"));
```

CLI (still dissolves `--dataflow` first — FR96):

```text
cargo bitloom hls --in-tree --pipeline --ii 1 --stages 2 --function pipe_add1 --dataflow add1
```

MVP loop-unroll path unchanged: `--in-tree` without `--pipeline`.

## Cross-links

- MVP FR95/96: [`fr35-hls.md`](fr35-hls.md)
- NFR14: `_agile-output/implementation-artifacts/nfr14-risk-epic52-hls-commercial-depth.md`
- AD-25: ARCHITECTURE-SPINE (Story 52.3 may revise)

## Non-goals (NFR47 → Phase 14)

- Full commercial HLS compiler suite
- Handshake / dynamic dataflow as default synthesizable semantics → **FR121** / [`fr121-handshake-default.md`](fr121-handshake-default.md) (not FR110)
- Closing FR110 via Bambu stub green alone

```text
cargo test -p bitloom --test fr110_hls_commercial_depth
cargo test -p bitloom --test fr110_epic52_closeout
cargo test -p bitloom --test fr95_in_tree_hls_schedule
```
