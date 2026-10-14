---
title: '在 HIR freeze 阶段拒绝零宽信号'
type: 'bugfix'
created: '2026-10-14'
status: 'done'
route: 'oneshot'
review_loop_iteration: 0
context: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** `GroundType::UInt { width: 0 }` 与 `SInt { width: 0 }` 当前可以进入 `FrozenHir`。该类型不能表示合法的非空硬件向量；仿真器在有符号比较/移位时可能 panic，Verilog emitter 也会从 `width - 1` 生成错误范围。

**Approach:** 在所有后端共享的 HIR freeze 阶段拒绝零宽 UInt/SInt 声明，并返回带源 span 的结构化 Diagnostic。覆盖端口、wire 和 register；合法的宽度至少为 1 的电路行为不变。

</frozen-after-approval>

## Implementation Notes

- `GroundType` 宽度是 `u32`；Builder 的 `record_width` 目前直接记录零，`finish()` 最终调用 `bitloom_hir::seal_from_builder`。
- `freeze()` 是 Builder 和 FIRRTL import 共用的校验入口；Story 1.3 已在此处校验时钟、复位，Story 1.4 在此处报告多驱动。
- `bitloom-vlog::verilog_ty` 对多位类型按 `[width - 1:0]` 生成范围；Builder 对零宽 memory 与零宽 Slice 已拒绝。
- 本修复集中于 HIR freeze，不修改 `GroundType` API、仿真器算术语义或各后端局部校验。
- 在 `crates/bitloom-hir/src/lib.rs` 的 freeze 路径统一检查 UInt/SInt 端口、wire 和 register；无效声明以 E0271 和原始 span 返回。
- 验证：`cargo fmt -p bitloom-hir --check`、`cargo test -p bitloom-hir`，以及 Builder/FIRRTL/Verilog/Sim 四个消费者的测试均通过。
- FIRRTL import 回归确认零宽端口在共用 freeze 边界返回 E0271；用户可见诊断记录见 `docs/hir-freeze-diagnostics.md`。
- Review Triage：采纳错误码文档与 FIRRTL import 覆盖建议；完成状态已更新为 `done`。
