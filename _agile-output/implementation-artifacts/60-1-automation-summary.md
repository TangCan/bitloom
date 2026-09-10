# Automation summary — Story 60.1

**Mode:** Expand after implementation (NFR14 doc gate)  
**Date:** 2026-09-10

## Coverage decision

Epic 60.1 is documentation-only (risk record + gate). Existing ATDD
`crates/bitloom/tests/nfr14_risk_epic60_symbiyosys_smt.rs` already guards:

- NFR14 fields (a)–(d)
- Nailed SymbiYosys/`sby` binding; assume/assert; fixture/repro
- Forbidden: FR92 alone; FR100 F1-(i) alone; FR112-B alone; docs-only; FR107 confusion
- Missing-tool non-silent-success obligation
- Gate on 60.2–60.3 ready
- Owner NFR14 + NFR50 + NFR51; NFR48; NFR14-crates; bitloom-prelude; Bitloom brand
- Branch C deferred honesty

**No additional automate tests** — would duplicate the ATDD file test without new risk surfaces. E2E / API / UI levels N/A.

## Harm / priority

| Risk | Level | Automation |
| ---- | ----- | ---------- |
| Missing / hollow NFR14 → premature 60.2 ready | High | Covered by ATDD |
| Close FR119 with FR100 / FR112-B / FR92 alone | High | Covered by ATDD string gates |
| Docs-only / FR107 confusion / silent missing-tool | High | Covered by ATDD |

## Outcome

Automate step: **accept ATDD as sufficient guardrail suite**; no new files.
