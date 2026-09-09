# Automation summary — Story 37.1

**Mode:** Expand after implementation (NFR14 doc gate)  
**Date:** 2026-09-09

## Coverage decision

Epic 37.1 is documentation-only (risk record + gate). Existing ATDD
`crates/bitloom/tests/nfr14_risk_epic37_interop_hls.rs` already guards:

- NFR14 fields (a)–(d)
- firtool/Chisel pin drift (7.14.0 ↔ 1.155.0)
- Mechanical Chisel mistaken for idiomatic
- Nightly real Bambu vs stub default
- Forbidden silent firtool bump
- Forbidden stub CI as HLS quality
- Forbidden `continue-on-error` masking failures
- Gate on 37.2–37.3 ready
- NFR12 / NFR39 / owner / NFR14-crates disambiguation

**No additional automate tests** — would duplicate the ATDD file test without new risk surfaces. E2E / API / UI levels N/A.

## Harm / priority

| Risk | Level | Automation |
| ---- | ----- | ---------- |
| Missing / hollow NFR14 record → premature 37.2 ready | High | Covered by ATDD |
| Silent firtool bump / stub-as-HLS / continue-on-error | High | Covered by ATDD string gates |

## Outcome

Automate step: **accept ATDD as sufficient guardrail suite**; no new files.
