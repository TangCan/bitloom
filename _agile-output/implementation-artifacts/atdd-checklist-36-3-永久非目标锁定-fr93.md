# ATDD Checklist — 36-3 / FR93 permanent non-goals

**Story:** `36-3-永久非目标锁定-fr93`  
**Stack:** backend (Cargo / `bitloom` integration tests)  
**Phase:** green (docs lock landed)

## Acceptance → tests

| AC | Test | Expected red reason (pre-impl) |
|----|------|--------------------------------|
| 树内 HLS 调度器 | `fr93_lists_in_tree_hls_scheduler` | no locked FR93 HLS scheduler non-goal |
| FIRRTL→idiomatic Scala | `fr93_lists_firrtl_to_idiomatic_scala` | no idiomatic Scala non-goal |
| TLM≡CA 形式证明 | `fr93_lists_tlm_ca_formal_proof` | no TLM≡CA formal-proof non-goal |
| VIP 全协议 IP | `fr93_lists_vip_full_protocol_ip` | no VIP full-protocol IP non-goal |
| 按键全 elaborate LSP | `fr93_lists_keystroke_full_elaboration_netlist_lsp` | no keystroke full-elab netlist LSP |
| 须新 PRD | `fr93_requires_new_prd_to_overturn` | no overturn-via-new-PRD clause |
| addendum 指针 | `fr93_addendum_points_to_permanent_non_goals` | addendum lacks FR93 → README/deferred |
| 品牌 Bitloom | `fr93_brand_remains_bitloom` | (should stay green if README intact) |

## Command

```bash
cargo test -p bitloom --test fr93_permanent_non_goals
```

## Out of scope (do not ATDD here)

- Epic 37–39 implementation
- Changing FR87 contract-green clauses in doc-19
