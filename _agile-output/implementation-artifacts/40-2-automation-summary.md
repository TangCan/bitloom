# Automation summary — Story 40.2

**Mode:** Expand after implementation (FR94 PRD / Correct Course gate)  
**Date:** 2026-09-09

## Coverage decision

Epic 40.2 is documentation + contract-gate ATDD. Existing suite
`crates/bitloom/tests/fr94_prd_path_b_gate.rs` already guards:

- Correct Course `status: approved` + Path B / FR93 overturn
- Addendum Phase 12 literal-green / FR94–105 / NFR40–43
- FR93 five-item → FR95–99(/100/101) map
- NFR42 claim discipline + FR87 historical (not sole product-done)
- Bitloom / `bitloom-*` brand + forbid `rhdl` publish
- prd amendment `phase12-literal-green-path-b`

Companion historical lock remains in `fr93_permanent_non_goals.rs` until Story 40.4.

**No additional automate tests** — would duplicate FR94 string gates without new risk surfaces. E2E / API / UI levels N/A.

## Harm / priority

| Risk | Level | Automation |
| ---- | ----- | ---------- |
| Hollow Phase 12 / missing FR93 overturn map | High | Covered by ATDD |
| FR87≡字面绿 / missing NFR42 | High | Covered by ATDD |
| Brand drift away from Bitloom | Medium | Covered by ATDD |
| Premature doc-19 / AD rewrite in this story | High | Scope review + separate stories |

## Outcome

Automate step: **accept ATDD as sufficient guardrail suite**; no new files.
