# ATDD Checklist — Story 41.3

**Story:** 41-3-hls-闭包数据流变换-fr96  
**Phase:** red → green (FR96 HLS closure dataflow transform)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| 闭包 dissolve/inline 后进入 41.2 树内调度 | `fr96_closure_transform_enters_in_tree_schedule` | 仅裸 Op / 外挂 dissolve |
| schedule IR 含 fr96 + fr95 / stages；无 Bambu | same | 空壳 / 假路径 |
| 捕获闭包在调度前可读失败 | `fr96_capturing_closure_rejected_before_schedule` | silent / 进入 schedule |
| 错路径 token 调度前失败 | `fr96_wrong_path_rejected_before_schedule` | 绕过 D1 |
| 文档：FR96 + 交叉链 FR72–78 | `fr96_docs_crosslink_fr72_through_fr78` | 文档假交 / 无交叉链 |
| 品牌 Bitloom | same | 错误品牌 |
| 产物无 closure IR 残留（NFR36） | `fr96_schedule_has_no_closure_residue` | Fn 泄漏进 IR |
