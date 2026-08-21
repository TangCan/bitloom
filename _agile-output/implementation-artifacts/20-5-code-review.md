# Code Review: Story 20.5 import CLI + 混合夹具（FR40 / FR46 腿 3）

**Reviewer:** adversarial pass（blind / edge / verification-gap）  
**Date:** 2026-08-21  
**Verdict:** accept

## Findings triage

| Severity | Category | Finding | Disposition |
| --- | --- | --- | --- |
| low | defer | `import` 不写 Chisel Scala（仅 `.v` / 可选 `.fir`） | deferred：CLI 对齐 `build` Verilog 后端；Scala 仍走库 API |
| low | defer | 混合夹具 via `include_str` 路径耦合 monorepo layout | deferred：文档夹具；standalone 用户用 `--input` |
| — | reject | 解析 Scala / JVM smoke | 合同为 `.fir` |
| — | reject | visualize/wave | 非本故事 |

## AC checklist

| AC | Status |
| --- | --- |
| `import` `--help` + smoke | pass |
| 混合夹具同一 emit 后端 | pass（`examples/chisel_mixed`） |
| README/文档指向 | pass |
| Epic 20 → done | pass（收尾） |

## testarch-automate

- `crates/bitloom/tests/import_cli.rs`（help / smoke / bad header）
- `examples/chisel_mixed`（Bitloom + `.fir` → `bitloom_vlog::emit`）

## Disposition

`just test` 绿 → mark 20.5 + epic-20 done。
