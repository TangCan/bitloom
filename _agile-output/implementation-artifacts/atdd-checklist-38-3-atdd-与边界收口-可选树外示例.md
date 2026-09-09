# ATDD Checklist — 38-3 / FR89 Epic 38 boundary closeout

**Story:** `38-3-atdd-与边界收口-可选树外示例`  
**Stack:** backend (Cargo / `bitloom` integration tests)  
**Phase:** green (NFR14 closed + deferred cross-ref + closeout ATDD)  
**Disposition:** close Epic 38 (branch A deepen already in 38.2); optional `ip_box` baud demo

## Acceptance → tests

| AC | Test | Expected red reason (pre-impl) |
|----|------|--------------------------------|
| NFR14 关闭勾选 | `fr89_nfr14_epic38_close_conditions_checked` | close checklist unchecked / status not closed |
| deferred 交叉引用 | `fr89_deferred_full_protocol_cross_ref_closed_by_38_3` | still has open「关闭交叉引用 → Story 38.3」 |
| 加深 ATDD 稳定 | `fr89_deepen_atdd_fixture_still_present` | (should stay green from 38.2) |
| 非 RX | `fr89_no_branch_b_rx_delivery_claims` | (should stay green) |
| docs 边界 | `fr89_docs_ip_keeps_fr89_subset_and_non_goals` | (should stay green) |

## Command

```bash
cargo test -p bitloom --test fr89_epic38_boundary_closeout
```

## Out of scope (do not ATDD here)

- Branch B minimal RX
- VIP / full-protocol UART
- Epic 39
- New out-of-tree crate (optional `ip_box` light touch only)
