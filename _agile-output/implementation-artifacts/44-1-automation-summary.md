# Automation summary — Story 44.1

**Mode:** Expand after implementation (NFR14 doc gate)  
**Date:** 2026-09-09

## Coverage decision

Epic 44.1 is documentation-only (risk record + gate). Existing ATDD
`crates/bitloom/tests/nfr14_risk_epic44_full_elaborate_lsp.rs` already guards:

- NFR14 fields (a)–(d)
- Keystroke full-elaborate performance / scope bounds (P1–P6)
- Division of labor with host rust-analyzer / FR90
- Forbidden: half-baked language-server binary as done
- Forbidden: HTML visualization counts as LSP
- Forbidden: close FR99 on shallow diagnostics only
- Gate on 44.2–44.4 ready
- Epic 44 / FR99 / owner NFR14+NFR40 / NFR14-crates / bitloom-prelude / Epic 40 gate / FR91 Path B contrast

Companion: `fr98_epic43_closeout` sprint assertion now allows `epic-44: in-progress` only when `44-1` is `done`.

**No additional automate tests** — would duplicate the ATDD file test without new risk surfaces. E2E / API / UI levels N/A. LSP implementation coverage belongs to 44.2–44.4.

## Harm / priority

| Risk | Level | Automation |
| ---- | ----- | ---------- |
| Missing / hollow NFR14 record → premature 44.2+ ready | High | Covered by ATDD |
| HTML / rust-analyzer / shallow diagnostics re-labeled as FR99 | High | Covered by ATDD string gates |
| Half-baked language-server binary claimed done | High | Covered by ATDD |

## Outcome

Automate step: **accept ATDD as sufficient guardrail suite**; no new files.
