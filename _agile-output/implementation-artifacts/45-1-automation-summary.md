# Automation summary — Story 45.1

**Mode:** Expand after implementation (NFR14 doc gate)  
**Date:** 2026-09-09

## Coverage decision

Epic 45.1 is documentation-only (risk record + gate). Existing ATDD
`crates/bitloom/tests/nfr14_risk_epic45_formal_equiv_dual_model.rs` already guards:

- NFR14 fields (a)–(d)
- Formal-equivalence product bounds (tool / proof obligation / fixture; F1–F5)
- Property-macro matrix inventory (`functional_model` / `abstraction` / `functional_state`)
- Dual-model first-class IP set (FIFO / UART / SPI / I2C / AXI)
- Forbidden: random co-sim scoreboard alone as formal-equivalence product
- Forbidden: close FR102/103 on template adapters only
- Gate on 45.2–45.4 ready
- Epic 45 / FR100–103 / owner NFR14+NFR40 / NFR14-crates / bitloom-prelude / Epic 40 gate / FR92 contrast

Companion: `fr99_epic44_closeout` and `fr98_epic43_closeout` sprint assertions now allow
`epic-45: in-progress` only when `45-1` is `done`.

**No additional automate tests** — would duplicate the ATDD file test without new risk surfaces.
E2E / API / UI levels N/A. FR100/102/103 implementation coverage belongs to 45.2–45.4.

## Harm / priority

| Risk | Level | Automation |
| ---- | ----- | ---------- |
| Missing / hollow NFR14 record → premature 45.2+ ready | High | Covered by ATDD |
| FR92 scoreboard / template adapter re-labeled as FR100/102/103 | High | Covered by ATDD string gates |
| Undefined IP set or macro matrix → silent scope shrink | High | Covered by ATDD |

## Outcome

Automate step: **accept ATDD as sufficient guardrail suite**; no new files.
