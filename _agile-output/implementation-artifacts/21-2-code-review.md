# Code Review: Story 21.2 手写 bridge / abstraction / both 回归（FR29）

**Reviewer:** adversarial pass (unattended; nested review subagents skipped per parent constraint)  
**Date:** 2026-08-21  
**Verdict:** accept

## Findings

### Low — accepted

1. **属性仍用 `#[rhdl::…]` 宏路径：** 与现有 macro 一致；文档未强行改宏命名空间（避免无 scope 的破坏性 rename）。
2. **ATDD 不直接 `cargo test -p mixed_both`：** 以源文件存在 + 全量 `just test` 收口；与 Epic 20 文档门禁故事一致。

### Out of scope (not defects)

- 未实现 FR47 生成器（明确留给 21.3+）。
- 未将 FR30 equiv 接到生成产物（21.5）。

## AC checklist

| AC | Status |
| --- | --- |
| 混合夹具 PortValues 对照 pass / 故意不一致 fail | pass（mixed_both + bitloom-sim） |
| 文档说明手写 ↔ 生成关系；生成不取代手写 | pass |
| 相关 cargo test 通过 | pass |
| 未实现 FR47 生成器 | pass（ATDD 禁 generate_functional_sim） |

## Disposition

Mark story done；进入 commit。

## testarch-automate

- 新增 `crates/bitloom/tests/fr29_handwritten_bridge_regression.rs`（文档+夹具门禁；非 N/A）。
- 既有 `examples/mixed_both` / `bitloom-sim` mixed_both 测试保持为运行时回归。
