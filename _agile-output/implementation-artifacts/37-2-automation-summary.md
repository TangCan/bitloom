# Automation summary — Story 37.2

**Mode:** Expand after implementation (FR88 docs honesty)  
**Date:** 2026-09-09

## Coverage decision

Epic 37.2 is documentation-contract (+ docs ATDD). Existing ATDD
`crates/bitloom/tests/fr88_firtool_chisel_ops_honesty.rs` already guards:

- Pinned Chisel 7.14.0 ↔ firtool 1.155.0 in FR28
- `RHDL_FIRTOOL_PATH` + cache/`firtool ensure` + PATH distrust
- Public compilable ≠ idiomatic honesty + ports/hierarchy predicates
- README cross-link to FR28 ops/honesty (FR88)
- FR28/FR46 ban on handwritten maintainable claims
- Bitloom brand

**No additional automate tests** — would duplicate string gates without new risk surfaces. E2E / API / UI levels N/A (docs-only story).

## Harm / priority

| Risk | Level | Automation |
| ---- | ----- | ---------- |
| Missing override/cache ops → PATH firtool impersonates pin | High | Covered by ATDD |
| Mechanical Chisel read as idiomatic | High | Covered by ATDD honesty + ban tests |
| README drifts from FR28 ops | Medium | Covered by README cross-link test |

## Outcome

Automate step: **accept ATDD as sufficient guardrail suite**; no new files.
