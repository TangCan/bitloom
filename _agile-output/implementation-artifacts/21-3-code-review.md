# Code Review: Story 21.3 生成 Rust 功能模拟器工件（FR47 腿 1）

**Reviewer:** adversarial pass (unattended)  
**Date:** 2026-08-21  
**Verdict:** accept

## Findings

### Low — accepted

1. **功能模型 = FrozenHir 解释器子集：** 与 `Sim::tick` 对齐的最小 codegen，非 HLS；符合「prefer minimal workable generators」。
2. **生成 crate 仅依赖 `bitloom-hir`：** 设计 crate 仍只 `bitloom-prelude`；生成器在 `bitloom-sim` / CLI。

### Out of scope (not defects)

- 周期精确工件 + 桥接对照（21.4）。
- FR30 接入生成路径（21.5）。

## AC checklist

| AC | Status |
| --- | --- |
| CLI/API 生成 Rust 功能模拟器工件 | pass（`generate_functional_sim` + `gen-func`） |
| 黄金夹具 PortValues | pass（reset+3 → data_out==3；生成 crate `cargo test`） |
| 无 SystemC | pass（文档 Non-goals） |
| 设计 crate 只依赖 bitloom-prelude | pass |

## Disposition

Mark story done；进入 commit。

## testarch-automate

- 新增 `crates/bitloom/tests/fr47_functional_sim_gen.rs`（文档门禁 + 黄金 PortValues + 生成 crate 编译测试）。
- `bitloom-sim` `generate` 模块单测：tick 对照 + emit 文件。
