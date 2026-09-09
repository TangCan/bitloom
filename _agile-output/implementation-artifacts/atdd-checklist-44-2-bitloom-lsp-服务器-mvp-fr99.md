# ATDD Checklist — Story 44.2

**Story:** 44-2-bitloom-lsp-服务器-mvp-fr99  
**Phase:** red → green (Bitloom LSP server MVP + editor wiring)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| `bitloom-lsp` crate + binary | `fr99_bitloom_lsp_crate_and_binary_exist` | no product LSP surface |
| Minimal initialize / capabilities | `fr99_bitloom_lsp_initialize_session` | half-baked / unstartable server |
| VS Code or equiv wiring docs | `fr99_bitloom_lsp_editor_wiring_docs` | no editor path |
| Brand Bitloom; ≠ rust-analyzer alone | `fr99_bitloom_lsp_brand_and_not_rust_analyzer_alone` | RA fake green |
| Scope: no 44.3 elaborate / no Epic 44 close | `fr99_bitloom_lsp_mvp_scope_guards` | scope creep / premature FR99 close |
| Sprint 44-2 done; 44-3/44-4 backlog; epic-44 in-progress | same | wrong sprint keys |

**Companion regression (green with implementation):** rewrite FR91/FR90 “forbid `bitloom-lsp` crate” guards to allow documented Epic 44 MVP while keeping Epic 39 Path B historically closed.
