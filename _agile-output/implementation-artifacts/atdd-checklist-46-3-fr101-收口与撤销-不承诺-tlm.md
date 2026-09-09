# ATDD checklist — 46.3 FR101 收口与撤销「不承诺 TLM」

| Guard | Test | Failure mode |
|-------|------|--------------|
| NFR14 Epic 46 close checkboxes all `[x]` + closed | `fr101_nfr14_epic46_close_conditions_checked` | unchecked / not closed |
| README/deferred revoke「不承诺 TLM」exclusion; mark FR101/Epic 46 closed | `fr101_deferred_readme_epic46_closed` | still「关闭前不得宣称」without closed note |
| docs/fr101 marks Epic 46 closed; LT-only honesty; ≠ FR47 | `fr101_docs_epic46_closed_lt_only` | claims AT delivered / FR47=TLM / not closed |
| sprint `46-3: done` + `epic-46: done`; epic-47 gated (backlog or in-progress after 47.1) | `fr101_sprint_epic46_done_epic47_gated` | epic-46 not done / epic-47 started without 47.1 |
| Prior product ATDD still green after sprint guard relax | `fr101_systemc_tlm_product` suite | product regression |

**Recipe:**
```text
cargo test -p bitloom --test fr101_epic46_closeout
cargo test -p bitloom --test fr101_systemc_tlm_product
```
