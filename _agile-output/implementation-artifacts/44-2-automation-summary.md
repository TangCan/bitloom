# Automation summary — Story 44.2

**Mode:** Expand after implementation (Bitloom LSP server MVP)  
**Date:** 2026-09-09

## Coverage decision

Story 44.2 ATDD `crates/bitloom/tests/fr99_bitloom_lsp_server_mvp.rs` already guards:

- `bitloom-lsp` crate + `[[bin]]` + workspace membership
- Reproducible stdio `initialize` session (spawn `cargo run -p bitloom-lsp`)
- VS Code / wiring docs (`docs/fr99-bitloom-lsp.md`)
- Brand Bitloom; rust-analyzer alone does not complete the story
- Sprint: `44-2: done`, `44-3`/`44-4` backlog, `epic-44: in-progress`
- fr38 must not keep an unqualified “no language-server binary” claim

Companion regressions updated (not duplicated):

- `fr91_bitloom_lsp_explicit_defer` — allow documented `bitloom-lsp`; keep Epic 39 Path B historical close
- `fr90_host_ide_rust_analyzer` — same; require FR90 contrast when LSP crate exists

**No additional automate tests** — further coverage would duplicate ATDD or belong to 44.3 (elaborate diagnostics) / 44.4 (FR99 closeout). E2E UI extension publish N/A for MVP wiring docs.

## Harm / priority

| Risk | Level | Automation |
| ---- | ----- | ---------- |
| Half-baked / unstartable language-server claimed done | High | Covered by initialize session ATDD |
| rust-analyzer alone labeled as FR99 / 44.2 | High | Covered by docs string gate |
| Scope creep into 44.3 elaborate or Epic 44 close | High | Covered by sprint / NFR14 scope guards |
| Silent Epic 39 Path B regression | Medium | Covered by rewritten FR91/FR90 guards |

## Outcome

Automate step: **accept ATDD + companion regression updates as sufficient**; no new files.
