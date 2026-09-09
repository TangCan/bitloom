# ATDD Checklist — Story 40.2

**Story:** 40-2-correct-course-prd-推翻-fr93-fr94  
**Phase:** green (FR94 Correct Course + PRD Path B gate)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| Correct Course `approved` + Path B / FR93 | `fr94_correct_course_approved` | implement without contract stamp |
| Addendum Phase 12 字面绿 / FR94–105 | `fr94_addendum_phase12_literal_green` | no Path B PRD section |
| FR93 五条推翻 → FR95–99(/100/101) | `fr94_addendum_overturns_fr93_five_items` | hollow overturn |
| NFR42 + FR87 历史非唯一完成口径 | `fr94_nfr42_claim_discipline_and_fr87_historical` | FR87≡字面绿 steal |
| Bitloom / `bitloom-*` 品牌不变 | `fr94_bitloom_brand_unchanged` | brand drift |
| prd amendment `phase12-literal-green-path-b` | `fr94_prd_amendment_stamp` | missing amendment trail |

**Out of scope (must stay red-path elsewhere / later stories):**
- doc-19 §19.7–19.9 rewrite → Story 40.3
- ARCHITECTURE-SPINE AD / README FR93 lock revocation → Story 40.4
