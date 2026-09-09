# Automation Summary — Story 39.3

**Mode:** Expand after implementation (FR91 Path B doc closeout)  
**Date:** 2026-09-09  
**Decision:** ATDD sufficient — no additional automation layer

## Covered

- `docs/fr38-viz-lsp.md` FR91 Path B / explicit defer + forbid delivered claim
- README FR91 Path B + 不得声称已交付
- HTML ≠ LSP
- NFR14 FR91 `[x]`；FR92 still open
- No `crates/language-server` / `bitloom-lsp` / `bitloom_lsp`
- Sprint `39-4` backlog

## Why no further tests

Further tests would duplicate docs/NFR14 string gates without new risk surfaces (no runtime LSP binary, no editor integration in Path B).

## Residual risks

| Risk | Severity | Mitigation |
| ---- | ---- | ---- |
| Future half-built LSP crate added silently | High | Path-existence ATDD |
| Premature 39.4 / epic close | High | FR92 open + 39.4 backlog ATDD |
| Marketing host RA / HTML as Bitloom LSP | High | fr38/README forbid-delivered + HTML≠LSP |
