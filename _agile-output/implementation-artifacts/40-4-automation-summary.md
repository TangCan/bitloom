# Automation Summary — Story 40.4

**Mode:** Expand after implementation (AD revise + FR93 unlock / FR94 / NFR41)  
**Date:** 2026-09-09

## Tests added / rewritten

| File | Role |
| ---- | ---- |
| `crates/bitloom/tests/fr94_ad_fr93_unlock.rs` | NEW — AD-5/25/27 + Deferred + README unlock + NFR41 |
| `crates/bitloom/tests/fr93_permanent_non_goals.rs` | REWRITE — historical overturn + FR mapping（不再要求当前「须新 PRD」锁） |

## Companion (must stay green)

- `fr94_prd_path_b_gate.rs` (40.2)
- `fr94_doc19_literal_green.rs` (40.3)

## Risk notes

| Risk | Severity | Mitigation |
| ---- | -------- | ---------- |
| Premature Epic 41–47 ready | High | sprint 仍 backlog；审查确认 |
| Half-finished HLS/TLM claimed done | High | docs only；实现仍属 41–47 |
| Old fr93 ATDD blocking Path B | Medium | rewritten under 40.4 |

## Out of scope retained

- No in-tree HLS / idiomatic Chisel / TLM / VIP / LSP **implementation**
- Epic 41+ remains backlog until each epic’s NFR14
