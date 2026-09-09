# ATDD Checklist — Story 44.4

**Story:** 44-4-fr99-收口与撤销-lsp-非目标  
**Phase:** red → green (FR99 / Epic 44 docs+ATDD closeout; revoke Path B completion bar)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| NFR14 Epic 44 close boxes | `fr99_nfr14_epic44_close_conditions_checked` | unchecked / not closed |
| README / deferred FR99 closed | `fr99_deferred_readme_epic44_closed` | permanent non-goal / Path B completion bar |
| fr99/fr38/fr90 revoke narrative | `fr99_docs_revoke_path_b_completion_narrative` | pending 44.4 / RA substitutes FR99 |
| Sprint epic-44 done; 45+ backlog | `fr99_sprint_epic44_done_epic45_plus_backlog` | start Epic 45+ |
| Brand Bitloom / prelude-only | `fr99_closeout_brand_and_prelude_only` | wrong brand/deps |
| Prior mvp/full guards allow done | updated `fr99_*_scope_guards` | false red after close |
| FR90 honesty after FR99 | updated `fr90_host_ide_rust_analyzer` | stale "LSP not delivered" |
