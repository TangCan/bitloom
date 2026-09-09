# ATDD checklist — 46.2 SystemC TLM-2.0 产品面（FR101）

**Phase:** red → green (SystemC TLM-2.0 LT product path)  
**Primary:** `cargo test -p bitloom --test fr101_systemc_tlm_product`

| Requirement | Test | Failure mode |
|-------------|------|--------------|
| Docs LT-only + AD-5 + ≠ FR47 + Bitloom | `fr101_docs_lt_only_ad5_not_rust_fl` | slogan / wrong abstraction |
| Emit TLM LT sources (D1/D2) | `fr101_emit_d1_d2_sources_are_systemc_tlm_not_rust_fl` | Rust FL labeled as TLM |
| SystemC resolve / readable miss | `fr101_resolve_systemc_or_readable_error` | silent fake green |
| LT smoke runnable | `fr101_lt_smoke_build_and_run_when_systemc_present` | no fixture |
| Sprint gates | `fr101_sprint_46_2_done_46_3_backlog_epic_in_progress` | premature epic close |
| FR47 contrast | `fr101_fr47_doc_still_says_not_systemc` | FR47 fake TLM |
| NFR14 still present | `fr101_nfr14_gate_still_present_and_46_2_checkable` | gate deleted |
