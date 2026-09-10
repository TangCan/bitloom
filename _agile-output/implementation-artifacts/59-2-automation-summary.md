# Automation summary — Story 59.2

**Mode:** Expand after implementation (FR118 syn-scan)  
**Date:** 2026-09-10

## Coverage decision

ATDD `crates/bitloom/tests/fr118_syn_scan_design_root_discovery.rs` already covers:

- Docs contract + bans (DesignFixture / metadata alone / shallow)
- No-metadata syn-scan positive discover + full elaborate Pass
- Fail root readable diagnostics
- Shallow does not finish
- FR113 metadata + FR99 DesignFixture regression
- didSave prefers syn-scan
- NFR14 gate still selects syn-scan

Unit coverage in `bitloom-lsp` `discover::tests` covers attribute parsing for `bitloom::top` / `rhdl::top`.

**No additional automate tests** beyond ATDD — would duplicate risk surfaces. E2E IDE plugin N/A.

## Harm / priority

| Risk | Level | Automation |
| ---- | ----- | ---------- |
| Metadata-only close pretending FR118 | High | ATDD bans + no-metadata fixture |
| Shallow finish silent green | High | ATDD shallow assertion |
| Break FR99/FR113 | High | Regression tests in same file |

## Outcome

Automate step: **accept ATDD + discover unit tests as sufficient**; no new files.
