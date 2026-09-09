# Automation summary — Story 41.1

**Mode:** Expand after implementation (NFR14 doc gate)  
**Date:** 2026-09-09

## Coverage decision

Epic 41.1 is documentation-only (risk record + gate). Existing ATDD
`crates/bitloom/tests/nfr14_risk_epic41_in_tree_hls.rs` already guards:

- NFR14 fields (a)–(d)
- In-tree vs external (Bambu / FR35 / FR86) coexistence
- Scheduling/allocation crate scope boundary
- FR96 ↔ AD-18 dissolve rules
- Forbidden: docs-only claim of in-tree HLS delivered
- Forbidden: stub / `BITLOOM_HLS_USE_REAL` as FR95 done
- Forbidden: silent dynamic dataflow / Handshake defaults
- Gate on 41.2–41.4 ready
- AD-25 / Epic 41 / owner NFR14+NFR41 / NFR14-crates / bitloom-prelude / Epic 40 gate

**No additional automate tests** — would duplicate the ATDD file test without new risk surfaces. E2E / API / UI levels N/A.

## Harm / priority

| Risk | Level | Automation |
| ---- | ----- | ---------- |
| Missing / hollow NFR14 record → premature 41.2 ready | High | Covered by ATDD |
| External stub path steal FR95 | High | Covered by ATDD string gates |
| Silent Handshake / dynamic dataflow default | High | Covered by ATDD |

## Outcome

Automate step: **accept ATDD as sufficient guardrail suite**; no new files.
