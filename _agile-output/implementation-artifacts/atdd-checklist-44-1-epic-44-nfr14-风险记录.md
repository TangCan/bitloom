# ATDD Checklist — Story 44.1

**Story:** 44-1-epic-44-nfr14-风险记录  
**Phase:** red → green (NFR14 full-elaborate Bitloom LSP gate ATDD)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| NFR14 fields (a)–(d) | `nfr14_risk_epic44_full_elaborate_lsp_has_required_fields` | missing sections |
| 按键全 elaborate 性能/范围边界 | same | FR99 bounds undefined |
| 与宿主 rust-analyzer 分工 | same | FR90/FR99 conflated |
| Forbid 半成品 language-server 交差 | same | half-baked binary fake green |
| Forbid HTML 可视化计入 LSP | same | HTML ≠ LSP violated |
| Forbid 仅浅层诊断关闭 FR99 | same | shallow-only close |
| Gate 44.2–44.4 | same | premature ready |
| Owner NFR14 / NFR40 | same | no accountable owner |
