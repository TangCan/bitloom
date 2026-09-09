# ATDD Checklist — Story 41.4

**Story:** 41-4-fr95-fr96-收口与回归  
**Phase:** red → green (FR95/FR96 Epic 41 closeout)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| NFR14 Epic 41 关闭条件全勾 + closed | `fr95_fr96_nfr14_epic41_close_conditions_checked` | 假关 / 未勾选 |
| README 不含「不实现树内调度器」；含 FR95 树内完成面 | `fr95_fr96_readme_in_tree_not_forbidden` | 公开面仍禁树内 |
| 外挂不得单独满足 FR95（README/fr35） | `fr95_fr96_external_not_sole_fr95_definition` | 外挂冒充唯一完成面 |
| deferred/README：树内非永久非目标；Epic 41 已关 | `fr95_fr96_deferred_readme_epic41_closed` | 仍作永久非目标 / 未标关闭 |
| sprint `epic-41: done`；未开工 epic-42 | `fr95_fr96_sprint_epic41_done_epic42_untouched` | 假关 / 越界 |
| 品牌 Bitloom | `fr95_fr96_closeout_brand_bitloom` | 错误品牌 |
