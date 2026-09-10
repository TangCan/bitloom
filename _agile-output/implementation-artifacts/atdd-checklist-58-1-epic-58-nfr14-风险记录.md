# ATDD Checklist — Story 58.1 Epic 58 NFR14 风险记录

**Date:** 2026-09-10  
**Mode:** Red-phase scaffold then green with risk record  
**Stack:** backend / docs gate（Rust `#[test]` 文件存在性断言）  
**Generation:** AI（对齐 Epic 56.1 / 57.1）

## Acceptance mapping

| AC | Red test | Green when |
| ---- | -------- | ---------- |
| AC1 钉死 A 或 B + 工具/夹具 | `nfr14_risk_epic58_tywaves_typed_ide_waveform` | 记录选定 (B) + 证明义务/夹具/工具 |
| AC2 禁止 I1–I3 / VCD·GTKWave / FR114 LCOV / docs-only | 同测试 | 禁止项字符串齐全 |
| AC3 无记录则 58.2–58.3 不得 ready；负责人 | 同测试门禁句 + 负责人 | 含 58.2–58.3 ready 门禁与 NFR14/NFR51 |

## Test file

`crates/bitloom/tests/nfr14_risk_epic58_tywaves_typed_ide_waveform.rs`

## Red confirmation

Expected before Story 58.1 build: `cargo test -p bitloom --test nfr14_risk_epic58_tywaves_typed_ide_waveform` **FAIL**（缺风险记录文件）。

## Notes

- Documentation gate only；无产品 runtime API。
- 选定加深子集 **(B) 自研等价 typed IDE 波形**；(A) Tywaves 一等集成 deferred。
- E2E / UI / API levels N/A。
