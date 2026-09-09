# Automation summary — Story 40.3

**Mode:** Expand after implementation (FR94 doc-19 literal-green rewrite)  
**Date:** 2026-09-09

## Coverage decision

Epic 40.3 is documentation + docs ATDD. Guardrail suite:

- **NEW** `crates/bitloom/tests/fr94_doc19_literal_green.rs` — header dual framing; P5/P6/P7 literal FR mapping; NFR42 checkbox discipline; README/deferred pointers
- **UPDATED** `crates/bitloom/tests/fr87_doc19_contract_green.rs` — Phase 11 contract-green as historical milestone only
- **Companion** `fr94_prd_path_b_gate.rs` (40.2) and `fr93_permanent_non_goals.rs` (until 40.4)

**No additional automate tests** — further string gates would duplicate without new risk surfaces. E2E / API / UI levels N/A.

## Harm / priority

| Risk | Level | Automation |
| ---- | ----- | ---------- |
| doc-19 still uses FR87 as current P5–P7 green | High | Covered by fr94_doc19_* |
| Missing FR95–105 checkbox map | High | Covered by P5/P6/P7 ATDD |
| NFR42 missing / forbid-literal residual as current discipline | High | Covered by NFR42 test |
| README/deferred still only point at contract-green as current | Medium | Covered by pointer test |
| Premature AD / FR93 README lock revocation | High | Scope review + Story 40.4 |

## Outcome

Automate step: **accept ATDD as sufficient guardrail suite**; no further new test files.
