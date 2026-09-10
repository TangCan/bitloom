# ATDD Checklist — Story 59.1 Epic 59 NFR14 风险记录

**Date:** 2026-09-10  
**Mode:** Red-phase scaffold then green with risk record  
**Stack:** backend / docs gate（Rust `#[test]` 文件存在性断言）  
**Generation:** AI（对齐 Epic 55.1 / 58.1）

## Acceptance mapping

| AC | Red test | Green when |
| ---- | -------- | ---------- |
| AC1 钉死扫描范围/识别/失败/共存 | `nfr14_risk_epic59_syn_scan_design_root_discovery` | 记录含 syn-scan 策略四钉 |
| AC2 禁止 DesignFixture / metadata / shallow / docs-only | 同测试 | 禁止项字符串齐全 |
| AC3 无记录则 59.2–59.3 不得 ready；负责人 | 同测试门禁句 + 负责人 | 含 59.2–59.3 ready 门禁与 NFR14/NFR51 |

## Test file

`crates/bitloom/tests/nfr14_risk_epic59_syn_scan_design_root_discovery.rs`

## Red confirmation

Expected before Story 59.1 build: `cargo test -p bitloom --test nfr14_risk_epic59_syn_scan_design_root_discovery` **FAIL**（缺风险记录文件）。

## Notes

- Documentation gate only；无产品 runtime API。
- 选定发现策略：**Workspace `#[bitloom::top]` syn-scan**；FR113 metadata / FR99 DesignFixture 仍回归、alone ≠ FR118。
- E2E / UI / API levels N/A。
