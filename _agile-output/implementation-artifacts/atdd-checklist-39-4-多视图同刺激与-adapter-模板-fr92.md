# ATDD Checklist — Story 39.4

**Story:** 39-4-多视图同刺激与-adapter-模板-fr92  
**Phase:** green (FR92 shared stimulus + adapter template)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| FR92 doc names shared stimulus + adapter template | `fr92_docs_contract_shared_stimulus_and_adapter` | hollow Wave D / P7 |
| Docs forbid auto FL≡RTL / SystemC TLM-2.0 (AD-5) | `fr92_docs_forbid_auto_equiv_and_systemc_tlm` | marketing steal |
| SharedStimulusScoreboard + same stimuli → functional≡tick pass | `fr92_shared_scoreboard_functional_matches_tick` | no shared fixture |
| Deliberate mismatch with same stimuli fails | `fr92_shared_scoreboard_deliberate_mismatch_fails` | silent always-pass |
| Adapter template cross-link (FR78) | `fr92_adapter_template_cross_links_fr78` | orphan template claim |
| NFR14 Epic 39 close checklist all ticked; epic-39 done | `fr92_nfr14_and_sprint_epic39_closed` | premature / incomplete close |
| No second uncorrelated sim semantics / no emit_tlm product | `fr92_no_second_sim_semantics_or_tlm_product` | fork semantics |
