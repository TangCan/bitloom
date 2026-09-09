# Automation summary — Story 42.1

**Mode:** Expand after implementation (NFR14 doc gate)  
**Date:** 2026-09-09

## Coverage decision

Epic 42.1 is documentation-only (risk record + gate). Existing ATDD
`crates/bitloom/tests/nfr14_risk_epic42_idiomatic_chisel.rs` already guards:

- NFR14 fields (a)–(d)
- Idiomatic acceptance criteria (naming / structure / readability / official style subset)
- FIRRTL→Scala Circuit official non-support + abandoned Parser (#4899)
- Forbidden: docs-only re-label of mechanical emit as idiomatic / FR97
- Forbidden: require upstream Parser restore without alternate contract
- Gate on 42.2–42.3 ready
- AD-27 / Epic 42 / FR97 / owner NFR14+NFR41 / NFR14-crates / bitloom-prelude / Epic 40 gate / FR28|FR46 contrast

**No additional automate tests** — would duplicate the ATDD file test without new risk surfaces. E2E / API / UI levels N/A.

## Harm / priority

| Risk | Level | Automation |
| ---- | ----- | ---------- |
| Missing / hollow NFR14 record → premature 42.2 ready | High | Covered by ATDD |
| Mechanical emit re-labeled idiomatic via docs | High | Covered by ATDD string gates |
| Wait-on-upstream Parser as FR97 completion | High | Covered by ATDD |

## Outcome

Automate step: **accept ATDD as sufficient guardrail suite**; no new files.
