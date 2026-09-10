---
title: '49.2 AT-style nb_transport 产品路径（FR107）'
type: 'feature'
created: '2026-09-10'
status: 'done'
baseline_commit: 'e85bc69'
---

## Story

As a 系统级验证工程师,
I want 可构建/可运行的 SystemC TLM AT 路径,
So that 超出 FR101 LT-only 的合同交付可验收。

## Tasks

- [x] `emit_systemc_tlm_at` + smoke + `gen-tlm-at`
- [x] AD-5 Phase 13 revision (NFR46)
- [x] docs/fr107-systemc-tlm-at.md
- [x] ATDD `fr107_systemc_tlm_at`
- [x] code-review / automate
- [x] sprint 49-2 done

## File List

- crates/bitloom-sim/src/systemc_tlm_at.rs
- crates/bitloom-sim/src/lib.rs
- crates/bitloom/src/main.rs
- docs/fr107-systemc-tlm-at.md
- docs/fr101-systemc-tlm.md (AT → FR107 pointer)
- ARCHITECTURE-SPINE AD-5
- crates/bitloom/tests/fr107_systemc_tlm_at.rs
