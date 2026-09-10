# Automation summary — Story 60.2

**Mode:** Expand after implementation (FR119 product path)  
**Date:** 2026-09-10

## Coverage decision

ATDD `crates/bitloom/tests/fr119_symbiyosys_smt_path.rs` already covers:

- Docs: SymbiYosys/`sby`, assume/assert, isolation vs FR100/FR112/FR85/FR107
- Script + Justfile first-class entry; FORCE_MISSING readable fail
- Pass/fail fixtures with assume+assert and BMC expect
- Optional live `sby` pass when installed
- NFR14 gate + FR100/FR112 surface regression guards

**No additional automate tests** beyond running FR100/FR112 ATDD in verification.

## Outcome

Automate step: **accept ATDD as sufficient**; no new files.
