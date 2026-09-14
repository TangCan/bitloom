---
title: '120.2 Handshake lower 加深实现与验收（FR187）'
type: 'feature'
created: '2026-09-14'
status: 'done'
route: 'oneshot'
baseline_commit: 'a460406 Story 120.1: Epic 120 NFR14 risk record for Handshake lower deepen (FR187).'
---

## Intent

实现超出 FR180 fork+join 的 Handshake lower 产品路径：`handshake.branch`+`handshake.merge`、AD-25（NFR90）修订、CLI/API、FORCE_MISSING 非零、`docs/fr187-*`。

## Tasks

- [x] T1: `CirctHandshakeLowerDeepen` + schedule/CLI + FORCE_MISSING
- [x] T2: AD-25 revise (NFR90)；更新 FR180 诚实指针 → FR187
- [x] T3: ATDD `fr187_handshake_lower_deepen`；`docs/fr187-handshake-lower-deepen.md`
- [x] T4: sprint 120.2 done；120.3 ready-for-dev
- [x] T5: code-review + automation-summary

## Testing

- `cargo test -p bitloom --test fr187_handshake_lower_deepen`
- `cargo clean && cargo fmt --all && just test`

## Dev Agent Record

### File List

- `crates/bitloom/src/hls.rs`
- `crates/bitloom/src/main.rs`
- `docs/fr187-handshake-lower-deepen.md`
- `docs/fr180-handshake-dialect-deepen.md`
- `ARCHITECTURE-SPINE.md` AD-25
- `crates/bitloom/tests/fr187_handshake_lower_deepen.rs`
- `120-2-*` / `sprint-status.yaml`
