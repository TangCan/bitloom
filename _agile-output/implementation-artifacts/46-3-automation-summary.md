# Automation summary — Story 46.3 FR101 closeout

| Guard | Test |
|-------|------|
| NFR14 Epic 46 close conditions | `fr101_nfr14_epic46_close_conditions_checked` |
| README/deferred/AGENTS Epic 46 closed | `fr101_deferred_readme_epic46_closed` |
| fr101 docs closed + LT-only | `fr101_docs_epic46_closed_lt_only` |
| Sprint epic-46 done / 47 backlog | `fr101_sprint_epic46_done_epic47_backlog` |
| Product face regression | `fr101_systemc_tlm_product` (sprint guard relaxed) |

**Note:** Epic 47 remains backlog; no automate expansion there in this story.

**Recipe:**
```text
cargo test -p bitloom --test fr101_epic46_closeout
cargo test -p bitloom --test fr101_systemc_tlm_product
cargo clean && cargo fmt --all && just test
```
