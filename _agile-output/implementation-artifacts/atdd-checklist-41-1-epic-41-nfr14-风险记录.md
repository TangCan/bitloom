# ATDD Checklist — Story 41.1

**Story:** 41-1-epic-41-nfr14-风险记录  
**Phase:** red → green (NFR14 in-tree HLS gate ATDD)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| NFR14 fields (a)–(d) | `nfr14_risk_epic41_in_tree_hls_has_required_fields` | missing sections |
| 树内↔外挂共存策略 | same | FR95 conflated with FR35/FR86 |
| scheduling/allocation crate 范围边界 | same | unbounded crate scope |
| FR96 ↔ AD-18 溶解规则 | same | capture closures / tick pollution |
| Forbid 仅文档声称树内 HLS 已交付 | same | doc-only fake close |
| Forbid stub/`BITLOOM_HLS_USE_REAL` 冒充 FR95 done | same | external path steal |
| Forbid silent 动态数据流默认语义 | same | Handshake default sneak |
| Gate 41.2–41.4 | same | premature ready |
| Owner NFR14 / NFR41；引用 AD-25 | same | no accountable owner / wrong AD |
