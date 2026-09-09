# Automation Summary — Story 42.2

**Mode:** Expand after implementation (FR97 idiomatic Chisel)  
**Date:** 2026-09-09

## Assessment

ATDD `crates/bitloom/tests/fr97_idiomatic_chisel.rs` already guards:

- Idiomatic emit + `check_idiomatic_chisel` Ok on hierarchy fixture
- Naming / Module+IO / instance structure / FR97 header / Bitloom / pin
- HIR-driven section markers (`registers` / `instances` / `logic`)
- Mechanical `emit_chisel` fails idiomatic check (E0904)
- Mutilated idiomatic (stripped body sections) fails E0904
- Docs: `fr97-idiomatic-chisel.md` vs fr28 mechanical honesty + FR97 cross-link

No additional TEA expansion required beyond ATDD for this MVP.

## Risk residual

| Risk | Level | Coverage |
| ---- | ---- | ---- |
| Mechanical re-labeled FR97 | High | Covered (header + check + ATDD) |
| IO-only section fake pass | High | Covered (HIR-driven markers + mutilation test) |
| Per-module scoped port check | Medium | Deferred |
| Empty circuit FR97 claim | Low | Deferred |

## Decision

**ATDD sufficient** — no new automate suite; keep `just test` / fr28 / fr88 regressions green.
