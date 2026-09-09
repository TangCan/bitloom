# ATDD Checklist — Story 39.2

**Story:** 39-2-宿主-ide-rust-analyzer-工作流-fr90  
**Phase:** green (FR90 docs + ATDD)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| FR90 doc + Bitloom + rust-analyzer + bitloom-prelude | `fr90_doc_identifies_bitloom_and_rust_analyzer_workflow` | no workflow page / wrong brand |
| Reproducible steps: completion / goto / diagnostics + 1.97.1 + RA tip | `fr90_doc_has_reproducible_steps_for_completion_goto_diagnostics` | vague or incomplete steps |
| Fixture `examples/counter_ports` explained | `fr90_doc_explains_fixture_project` | no fixture narrative |
| Host IDE ≠ hardware/Bitloom LSP / HTML | `fr90_doc_distinguishes_host_ide_from_hardware_lsp_and_html` | contract steal |
| README + fr38 cross-link | `fr90_readme_and_fr38_cross_link_host_path` | orphan docs |
| No LSP binary; 39.3/39.4 backlog; NFR14 FR91/92 unticked | `fr90_scope_guards_no_lsp_binary_and_epic39_siblings_backlog` | scope creep |
