# ATDD Checklist — 37-3 / FR88 HLS Path B (explicit stub)

**Story:** `37-3-可选夜间真机-bambu-或显式保持-stub-fr88`  
**Stack:** backend (Cargo / `bitloom` integration tests)  
**Phase:** green (Path B docs + deferred + NFR14 closed)  
**Disposition:** **Path B** (no nightly real-Bambu job)

## Acceptance → tests

| AC | Test | Expected red reason (pre-impl) |
|----|------|--------------------------------|
| Path B 文档 | `fr88_fr35_documents_path_b_stub_default` | fr35 lacks Epic 37「选 B」句 |
| 树内调度非目标 | `fr88_fr35_tree_in_hls_scheduler_remains_non_goal` | (likely green already) |
| deferred 收口 | `fr88_deferred_closes_optional_nightly_as_path_b` | item-54 still `deferred — 可选夜间` |
| NFR14 关闭勾选 | `fr88_nfr14_epic37_close_conditions_checked_path_b` | close checklist unchecked / status open |
| README 交叉 | `fr88_readme_cross_links_path_b_honesty` | README lacks FR88 Path B framing |
| CI 无假绿 | `fr88_ci_hls_smoke_has_no_continue_on_error` | (should stay green) |

## Command

```bash
cargo test -p bitloom --test fr88_hls_stub_path_b_honesty
```

## Out of scope (do not ATDD here)

- Path A nightly real Bambu CI job (not chosen)
- In-tree HLS scheduler
- Epic 38
