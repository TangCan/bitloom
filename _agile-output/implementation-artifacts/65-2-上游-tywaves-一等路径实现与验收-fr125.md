---
title: '65.2 上游 Tywaves 一等路径实现与验收（FR125）'
type: 'feature'
created: '2026-09-10'
status: 'done'
route: 'oneshot'
baseline_commit: '19e0431'
review_loop_iteration: 0
context: []
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

交付 `cargo bitloom wave --tywaves`（T1–T4）：`wave.tywaves.json` + launch；`BITLOOM_TYWAVES_BIN` / FORCE_MISSING 失败语义；保留 FR117 工件。

</frozen-after-approval>

## Tasks / Subtasks

- [x] T1: `tywaves_wave_json` / launch + CLI `--tywaves`
- [x] T2: ATDD `fr125_upstream_tywaves`
- [x] T3: docs/fr125；code-review Approve

## Dev Agent Record

### File List

- `crates/rhdl-viz/src/lib.rs`
- `crates/bitloom/src/main.rs`
- `crates/bitloom/tests/fr125_upstream_tywaves.rs`
- `docs/fr125-upstream-tywaves.md`
