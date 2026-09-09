# ATDD Checklist — Story 41.2

**Story:** 41-2-树内-hls-调度-mvp-fr95  
**Phase:** red → green (FR95 in-tree HLS schedule MVP)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| 树内 loop-unroll 产出可检查 schedule IR（无 Bambu） | `fr95_in_tree_loop_unroll_emits_schedule_ir_without_bambu` | 仅外挂 / 无 IR |
| schedule IR 含 FR95 / loop-unroll / stages | same | 空壳产物 |
| 可选下游 RTL stub 诚实标注 in-tree-mvp | `fr95_in_tree_emits_rtl_stub_marked_mvp` | 假 RTL 质量话术 |
| 文档：树内 = FR95 完成面；外挂不得单独满足 | `fr95_docs_distinguish_in_tree_vs_external` | 文档混用 / 假交付 |
| 品牌 Bitloom | same | 错误品牌 |
| 非法 trip_count 可读失败 | `fr95_in_tree_rejects_zero_unroll` | silent 0 |
| 外挂路径 API 仍存在（不回退删除） | `fr95_external_path_api_still_present` | 误删 FR35 |
