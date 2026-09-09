# ATDD Checklist — Story 42.3

**Story:** 42-3-fr97-收口与回归  
**Phase:** red → green (FR97 Epic 42 closeout)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| NFR14 Epic 42 关闭条件全勾 + closed | `fr97_nfr14_epic42_close_conditions_checked` | 假关 / 未勾选 |
| README/deferred：FIRRTL→idiomatic 非永久非目标；Epic 42 / FR97 已关 | `fr97_deferred_readme_epic42_closed` | 仍作永久非目标 / 未标关闭 |
| fr28 保留机械 ≠ idiomatic；交叉 FR97 | `fr97_fr28_mechanical_honesty_retained` | 机械诚实回退 / 无交叉 |
| fr97 文档收口（无「42.3 不在本故事」） | `fr97_doc_closeout_no_pending_42_3` | 范围仍挂待办 |
| sprint `epic-42: done`；未开工 epic-43 | `fr97_sprint_epic42_done_epic43_untouched` | 假关 / 越界 |
| 品牌 Bitloom | `fr97_closeout_brand_bitloom` | 错误品牌 |
