# Automation summary — Story 43.1

**Mode:** Expand after implementation (NFR14 doc gate)  
**Date:** 2026-09-09

## Coverage decision

Epic 43.1 is documentation-only (risk record + gate). Existing ATDD
`crates/bitloom/tests/nfr14_risk_epic43_vip_full_protocol_ip.rs` already guards:

- NFR14 fields (a)–(d)
- Per-class near-VIP / full-protocol acceptance items (UART/SPI/I2C/AXI + optional GPIO)
- Forbidden: one-class deepen claiming FR98 full green without explicit cut-down contract
- Forbidden: claim VIP without ATDD
- Gate on 43.2–43.5 ready
- Epic 43 / FR98 / owner NFR14+NFR40 / NFR14-crates / bitloom-prelude / Epic 40 gate / FR82+FR89 contrast

Companion: `fr97_epic42_closeout` sprint assertion now allows `epic-43: in-progress` only when `43-1` is `done`.

**No additional automate tests** — would duplicate the ATDD file test without new risk surfaces. E2E / API / UI levels N/A. IP implementation coverage belongs to 43.2–43.5.

## Harm / priority

| Risk | Level | Automation |
| ---- | ----- | ---------- |
| Missing / hollow NFR14 record → premature 43.2+ ready | High | Covered by ATDD |
| FR89 / single-class deepen re-labeled as FR98 | High | Covered by ATDD string gates |
| VIP claim without ATDD | High | Covered by ATDD |

## Outcome

Automate step: **accept ATDD as sufficient guardrail suite**; no new files.
