# Automation summary — Story 38.1

**Mode:** Expand after implementation (NFR14 doc gate)  
**Date:** 2026-09-09

## Coverage decision

Epic 38.1 is documentation-only (risk record + gate). Existing ATDD
`crates/bitloom/tests/nfr14_risk_epic38_uarttx_deepen.rs` already guards:

- NFR14 fields (a)–(d)
- Explicit deepen branch **A** (programmable baud)
- Branch B (minimal RX) not delivered
- FR82 baseline contrast (8N1 / baud=clk) + UartTx target
- Forbidden full-duplex / VIP / full-protocol claims
- Forbidden silent expansion to SPI / I2C / AXI
- Gate on 38.2–38.3 ready
- NFR39 / owner / NFR14-crates disambiguation / bitloom-prelude boundary

**No additional automate tests** — would duplicate the ATDD file test without new risk surfaces. E2E / API / UI levels N/A.

## Harm / priority

| Risk | Level | Automation |
| ---- | ----- | ---------- |
| Missing / hollow NFR14 record → premature 38.2 ready | High | Covered by ATDD |
| Unpicked branch / VIP / multi-IP expand claimed done | High | Covered by ATDD string gates |

## Outcome

Automate step: **accept ATDD as sufficient guardrail suite**; no new files.
