# ATDD Checklist — 36-2 / FR87 doc-19 contract-green

**Story:** `36-2-重写-doc-19-阶段五-七完成定义-fr87`  
**Stack:** backend (Cargo / `bitloom` integration tests)  
**Phase:** red → green via docs rewrite

## Acceptance → tests

| AC | Test | Expected red reason (pre-impl) |
|----|------|--------------------------------|
| P5 合同绿 | `fr87_doc19_p5_contract_green` | doc-19 lacks 外挂 HLS 可复现 / 薄 IP / VCD·层次 as completion |
| P6 合同绿 | `fr87_doc19_p6_contract_green` | lacks 精选 IP 深度 / 文档站 / rust-analyzer |
| P7 合同绿 | `fr87_doc19_p7_contract_green` | lacks 同刺激 + adapter 模板; lacks forbid auto-equiv / SystemC TLM |
| 禁字面全绿 | `fr87_doc19_forbids_literal_full_green_checkbox` | no FR87/NFR38 forbid clause |
| README/deferred | `fr87_readme_and_deferred_point_to_contract_green` | no 合同绿 pointer |

## Command

```bash
cargo test -p bitloom --test fr87_doc19_contract_green
```

## Out of scope (do not ATDD here)

- FR93 permanent non-goals body → Story 36.3
- Epic 37–39 implementation depth
