# ATDD Checklist — Story 42.2

**Story:** 42-2-idiomatic-chisel-发射与验收-fr97  
**Phase:** red → green (FR97 idiomatic Chisel emit + acceptance)  
**Date:** 2026-09-09  
**Stack:** backend (Rust / cargo test)

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| Idiomatic emit meets naming/structure/readability | `fr97_idiomatic_emit_passes_check_on_hierarchy` | mechanical-only / no FR97 path |
| Automated style/structure assertions | same + `fr97_idiomatic_header_and_sections` | silent claim without predicates |
| Docs distinguish mechanical vs FR97 | `fr97_docs_distinguish_mechanical_vs_idiomatic` | doc-only re-label / no fr97 page |
| Negative: mechanical fails idiomatic check | `fr97_mechanical_emit_fails_idiomatic_check` | silent Ok on FR28 output |
| Brand Bitloom + AD-27 | docs test | wrong brand / missing AD cite |
| No 42.3 closeout | N/A (manual / sprint) | premature Epic 42 close |
