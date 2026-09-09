# Automation summary — Story 40.1

**Mode:** Expand after implementation (NFR14 doc gate)  
**Date:** 2026-09-09

## Coverage decision

Epic 40.1 is documentation-only (risk record + gate). Existing ATDD
`crates/bitloom/tests/nfr14_risk_epic40_literal_path_b.rs` already guards:

- NFR14 fields (a)–(d)
- B1 / Path B forced choice vs research advising against literal full green
- FR93 five-item overturn scope (HLS / idiomatic / formal·TLM / VIP / LSP)
- NFR40 multi-year / high-maintenance
- AD-5 / AD-25 / AD-27 sync list
- Forbidden: open 41–47 before FR94 / Epic 40 gate
- Forbidden: FR87 contract-green claiming literal Path B done
- Forbidden: silent half-baked LSP/HLS as literal bars
- Gate on 40.2–40.4 ready
- FR94 / Epic 40 / owner NFR14+NFR40–43 / NFR14-crates / bitloom-prelude

**No additional automate tests** — would duplicate the ATDD file test without new risk surfaces. E2E / API / UI levels N/A.

## Harm / priority

| Risk | Level | Automation |
| ---- | ----- | ---------- |
| Missing / hollow NFR14 record → premature 40.2 ready | High | Covered by ATDD |
| FR87≡字面绿 / 未合 FR94 开 41–47 / 半成品 LSP·HLS | High | Covered by ATDD string gates |
| B1 without research-conflict disclosure | High | Covered by ATDD |

## Outcome

Automate step: **accept ATDD as sufficient guardrail suite**; no new files.
