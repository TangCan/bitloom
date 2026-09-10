---
title: '51.2 FSM/state-visit 覆盖率记录器与夹具（FR109）'
type: 'feature'
created: '2026-09-10'
status: 'done'
baseline_commit: '1b204c5'
---

## Story

As a 验证工程师,
I want 仿真路径记录 FSM/state-visit 并产出可检查报告,
So that Phase 12 cropped 的 C3 面可验收。

## Tasks

- [x] `Coverage` state-visit + `Sim::{register_fsm_states,sample_state_visit}`
- [x] report v3 + FR109 marker; `parse_state_report`
- [x] docs/fr109 + fr105 C3 → FR109 pointer
- [x] ATDD `fr109_fsm_state_visit_coverage`
- [x] code-review / automate
- [x] sprint 51-2 done

## File List

- crates/bitloom-sim/src/coverage.rs
- crates/bitloom-sim/src/lib.rs
- crates/bitloom/tests/fr109_fsm_state_visit_coverage.rs
- crates/bitloom/tests/fr105_sim_coverage_ext.rs
- docs/fr109-fsm-state-visit-coverage.md
- docs/fr105-sim-coverage-ext.md
