# ATDD Checklist — Story 42.1

**Story:** 42-1-epic-42-nfr14-风险记录  
**Phase:** red → green (NFR14 idiomatic Chisel gate ATDD)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| NFR14 fields (a)–(d) | `nfr14_risk_epic42_idiomatic_chisel_has_required_fields` | missing sections |
| Idiomatic 验收条（命名/结构/可读性或官方风格子集） | same | FR97 criteria undefined |
| FIRRTL→Scala Circuit 官方不支持关系 | same | wait-on-upstream fake plan |
| Forbid 仅文案把机械 emit 标成 idiomatic | same | doc-only re-label |
| Forbid 无替代合同要求恢复 Parser | same | abandoned-Parser dependency |
| Gate 42.2–42.3 | same | premature ready |
| Owner NFR14 / NFR41；引用 AD-27 | same | no accountable owner / wrong AD |
