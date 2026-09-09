# ATDD Checklist — Story 45.2

**Story:** 45-2-自动-fl-rtl-形式等价产品路径-fr100  
**Phase:** red → green (FR100 formal-equiv product path)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| FR100 doc names F1=(i) in-tree bounded prover + Bitloom | `fr100_docs_contract_f1_branch_and_completion_surface` | no product contract |
| Docs declare FR100 completion surface beyond FR92; FR92 alone insufficient | `fr100_docs_fr92_supporting_not_sufficient` | FR92 fake green |
| Automatic random/compare path Pass (reproducible seed) | `fr100_random_compare_pass_reproducible` | no F3 random fixture |
| Random/compare deliberate mismatch → readable Fail | `fr100_random_compare_deliberate_mismatch_readable` | silent always-pass |
| Bounded exhaustive formal product entry Pass | `fr100_bounded_exhaustive_pass` | no F1-(i) entry |
| Bounded exhaustive deliberate mismatch → readable Fail | `fr100_bounded_exhaustive_deliberate_mismatch_readable` | hollow formal entry |
| Sprint: 45-2 done; 45-3/45-4 backlog; epic-45 in-progress | `fr100_sprint_45_2_done_epic_open` | wrong epic close |
| API distinct from SharedStimulusScoreboard rename | `fr100_api_beyond_random_scoreboard_alone` | F5 violation |
