# ATDD Checklist — Story 40.4

**Story:** 40-4-修订-ad-撤销-fr93-锁定-fr94  
**Phase:** red → green (AD revise + FR93 unlock / FR94 / NFR41)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| AD-5 允许 SystemC TLM-2.0 产品路径（FR101） | `fr94_ad5_allows_tlm_product_path` | 实现 Epic 46 仍被旧 AD 挡死 |
| AD-25 允许树内 HLS 调度（FR95）；外挂可选 | `fr94_ad25_allows_in_tree_hls` | 实现 Epic 41 仍被「仅外挂」挡死 |
| AD-27 增加 idiomatic 验收面（FR97） | `fr94_ad27_adds_idiomatic_acceptance` | 实现 Epic 42 仍只能交差机械面 |
| Deferred 指向 Phase 12 FR94–105；非 FR93 永久锁 | `fr94_spine_deferred_path_b_pointer` | 脊柱仍把字面全绿钉在 FR87/FR93 |
| README/deferred：五条→Phase 12 FR；无当前「须新 PRD」锁 | `fr94_readme_deferred_fr93_unlocked` | 公开面仍挡死实现 / 与 FR94 冲突 |
| NFR41：实现 epic 须引用已修订 AD | `fr94_nfr41_ad_gate_stated` | 后续 epic 无 AD 门禁句 |
| 品牌 Bitloom 保持 | `fr94_ad_unlock_brand_bitloom` | 品牌漂移 |

**Rewrite / companion:**
- Rewrite `fr93_permanent_non_goals` → historical overturn + Phase 12 FR mapping（不再要求当前「须新 PRD」锁）
- Keep `fr94_prd_path_b_gate` + `fr94_doc19_literal_green` green

**Out of scope (must NOT happen):**
- Epic 41–47 标 ready 或开工实现
- 冒充字面 HLS/LSP/Chisel/TLM 实现已交付
