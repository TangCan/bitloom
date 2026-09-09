# ATDD Checklist — Story 45.3 多视图属性全矩阵（FR102）

| Acceptance | Test | Failure mode |
|------------|------|--------------|
| Full matrix docs (functional_model/abstraction/functional_state/bridge/both) + completion surface | `fr102_docs_full_matrix_and_completion_surface` | matrix row missing |
| Illegal combination gates documented | same + `fr102_negative_illegal_leak_guard_in_docs_and_risk` | illegal section missing |
| Adapter templates alone ≠ FR102 | `fr102_docs_adapters_supporting_not_sufficient` | template fake green |
| `functional_state` never HIR/freeze (docs) | `fr102_docs_functional_state_never_hir` | leak allowed in docs |
| Positive: soft fields skipped from FrozenHir ports | `fr102_positive_functional_state_skipped_from_frozen_ports` | soft name in ports |
| HostView kinds compile (matrix rows) | `fr102_hostview_matrix_kinds_compile` | attribute expand fail |
| Cross-docs / README honesty | `fr102_cross_docs_and_readme_honest` | missing FR102 link |
| Sprint: 45-3 done; 45-4 backlog; epic-45 in-progress | `fr102_sprint_45_3_done_epic_open` | wrong epic close |

```text
cargo test -p bitloom --test fr102_multiview_attribute_matrix
```
