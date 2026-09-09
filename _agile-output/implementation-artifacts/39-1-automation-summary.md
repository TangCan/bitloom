# Automation summary — Story 39.1

**Mode:** Expand after implementation (NFR14 doc gate)  
**Date:** 2026-09-09

## Coverage decision

Epic 39.1 is documentation-only (risk record + gate). Existing ATDD
`crates/bitloom/tests/nfr14_risk_epic39_ide_multiview.rs` already guards:

- NFR14 fields (a)–(d)
- Explicit FR91 branch **B** (Path B / explicit defer Bitloom LSP)
- Branch A (shallow LSP MVP) not delivered
- Forbidden SystemC TLM-2.0 claims
- Forbidden automatic FL≡RTL / formal-equivalence claims
- Forbidden hierarchy HTML counting as LSP done
- Forbidden undocumented half-baked LSP binary under branch B
- Gate on 39.2–39.4 ready
- FR90/FR91/FR92 / NFR39 / AD-5 / owner / NFR14-crates / bitloom-prelude / rust-analyzer

**No additional automate tests** — would duplicate the ATDD file test without new risk surfaces. E2E / API / UI levels N/A.

## Harm / priority

| Risk | Level | Automation |
| ---- | ----- | ---------- |
| Missing / hollow NFR14 record → premature 39.2 ready | High | Covered by ATDD |
| Unpicked FR91 branch / HTML≡LSP / TLM / formal claimed done | High | Covered by ATDD string gates |

## Outcome

Automate step: **accept ATDD as sufficient guardrail suite**; no new files.
