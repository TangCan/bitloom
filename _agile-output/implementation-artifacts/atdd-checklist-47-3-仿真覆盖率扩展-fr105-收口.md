# ATDD checklist — 47.3 仿真覆盖率扩展-fr105-收口

**Phase:** red → green (FR105 coverage extension + Epic 47 / Phase 12 closeout)  
**Primary:** `cargo test -p bitloom --test fr105_sim_coverage_ext`  
**Closeout:** `cargo test -p bitloom --test fr105_epic47_closeout`

| Requirement | Test | Failure mode |
|-------------|------|--------------|
| Docs C1+C2+C3-crop + R1–R2 + ≠ FR34 alone + Bitloom | `fr105_docs_coverage_ext_contract` | slogan / FR34-only fake green |
| Recorder API: v2 report + branch_hit/miss + toggle retained | `fr105_recorder_mux_branch_and_toggle_fixture` | docs-only / no recorder |
| C3 explicit crop (or FSM delivered) | `fr105_docs_c3_fsm_contract` | silent C3 omit |
| NFR14 Epic 47 close checklist all checked + closed | `fr105_nfr14_epic47_close_conditions_checked` | epic fake open |
| README/deferred/AGENTS/doc-19 close + remaining-gate honesty | `fr105_deferred_readme_epic47_closed` | stale「未关闭」 |
| Sprint `47-3: done` + `epic-47: done`; no epic-48 | `fr105_sprint_epic47_done_no_epic48` | epic not closed / new epic |
| fr104 ATDD no longer requires 47-3 backlog | (updated `fr104_*` guards) | stale 47.2 sprint lock |
