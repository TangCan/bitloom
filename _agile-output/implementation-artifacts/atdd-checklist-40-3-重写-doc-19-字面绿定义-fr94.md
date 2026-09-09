# ATDD Checklist — Story 40.3

**Story:** 40-3-重写-doc-19-字面绿定义-fr94  
**Phase:** red → green (FR94 doc-19 literal-green rewrite)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| 文首：Phase 11 合同绿=历史；Phase 12 字面绿=当前 | `fr94_doc19_header_dual_framing` | 读者仍把 FR87 当当前完成口径 |
| P5 字面绿含 FR95/FR96（树内 HLS + 闭包变换） | `fr94_doc19_p5_literal_green` | 仍以外挂 HLS 为 P5 绿 |
| P6 字面绿含 FR98/FR99（+ FR97 交叉） | `fr94_doc19_p6_literal_green` | 仍以宿主 rust-analyzer 为字面绿 |
| P7 字面绿含 FR100–105（等价/TLM/双模型/富波形/覆盖率） | `fr94_doc19_p7_literal_green` | 仍排除等价/TLM 为完成条件 |
| NFR42：对应 FR 关闭后方可勾选 | `fr94_doc19_nfr42_checkbox_discipline` | Path B 冲突的「禁止字面勾选」残留为当前纪律 |
| README + deferred 指向字面绿当前合同 | `fr94_readme_deferred_literal_green_pointer` | 交叉链仍只指向合同绿为当前完成 |

**Regression / companion:**
- Update `fr87_doc19_contract_green` → historical framing (must stay green after rewrite)
- Keep `fr94_prd_path_b_gate` green

**Out of scope (must stay for Story 40.4):**
- ARCHITECTURE-SPINE AD-5/25/27 修订
- README「永久非目标…须新 PRD」整节撤销
- Epic 40 关闭；Epic 41–47 ready
