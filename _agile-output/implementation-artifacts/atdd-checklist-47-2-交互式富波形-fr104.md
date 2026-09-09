# ATDD checklist — 47.2 交互式富波形（FR104）

**Phase:** red → green (interactive waveform product path)  
**Primary:** `cargo test -p bitloom --test fr104_interactive_wave`

| Requirement | Test | Failure mode |
|-------------|------|--------------|
| Docs I1–I3 + Bitloom + ≠ static timing + VCD retained | `fr104_docs_interactive_contract` | slogan / static-only fake green |
| wave emits interactive.html with I1–I3 markers | `fr104_wave_emits_interactive_html_i1_i3` | static timing alone |
| VCD + timing.html still emitted (AD-5/24 / FR38) | `fr104_vcd_and_timing_still_emitted` | VCD path removed |
| Manual/repro steps documented | `fr104_docs_repro_steps` | no checkable acceptance |
| Sprint gates | `fr104_sprint_47_2_done_47_3_backlog_epic_in_progress` | premature epic close |
| NFR14 FR104 checkable; no FR105 close | `fr104_nfr14_47_2_checkable_not_epic_closed` | epic fake close |
| fr38 cross-link / defer lifted for interactive | `fr104_fr38_cross_links_interactive` | stale defer-only narrative |
