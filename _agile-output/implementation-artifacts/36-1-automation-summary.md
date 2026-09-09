# Automation summary — Story 36.1

**Mode:** Expand after implementation (NFR14 doc gate)  
**Date:** 2026-09-09

## Coverage decision

Epic 36.1 is documentation-only (risk record + gate). Existing ATDD
`crates/bitloom/tests/nfr14_risk_epic36_contract_green.rs` already guards:

- NFR14 fields (a)–(d)
- P5–P7 rewrite / no FR46–FR86 rollback
- ①C relationship language
- Forbidden literal seven-stage green without docs
- Forbidden permanent non-goals marked done
- Gate on 36.2–36.3 ready
- NFR38 / owner / NFR14-crates disambiguation

**No additional automate tests** — would duplicate the ATDD file test without new risk surfaces. E2E / API / UI levels N/A.

## Harm / priority

| Risk | Level | Automation |
| ---- | ----- | ---------- |
| Missing / hollow NFR14 record → premature 36.2 ready | High | Covered by ATDD |
| Silent claim of literal seven-stage green | High | Covered by ATDD string gates |

## Outcome

Automate step: **accept ATDD as sufficient guardrail suite**; no new files.
