---
title: 'Sim 输入事件触发异步复位'
type: 'bugfix'
created: '2026-10-14'
status: 'done'
route: 'oneshot'
review_loop_iteration: 0
context:
  - AGENTS.md
  - _agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md
---

<frozen-after-approval reason="用户要求继续处理已记录的异步复位延期；范围遵循 AD-23 与现有 Sim 输入事件 API">

## Intent

**Problem:** HIR 和 Verilog emitter 保留异步复位标记，Verilog 在 `posedge rst` 立即清零，但 native `Sim` 目前只在 `tick()` 采样复位，导致仿真结果和硬件不一致。

**Approach:** 在现有 `Sim::set_inputs` 输入事件入口识别高有效 reset 的上升沿，立即清除关联的异步复位寄存器并传播组合输出；同步复位仍只在 `tick()` 生效。对 Interpreter 与 Compiled 两种引擎使用同一规则。

</frozen-after-approval>

## Implementation Notes

- 代码入口：`crates/bitloom-sim/src/lib.rs` 的 `Sim::set_inputs`、`tick_combinational` 和 `Stmt::RegDecl` 中的 `async_reset` / `reset` 字段。
- 现有测试原先只在时钟步进边界附近改变 reset，未验证无 tick 的异步断言；新增回归覆盖异步立即断言、释放后下一 tick 恢复，以及同步寄存器仍等待 tick，两种 TickEngine 均执行。异步与同步用例都在 `en=0` 时断言 reset。
- 复用 `set_inputs` 作为离散输入事件，不增加时间刻度或公开 API。cycle-only `GeneratedFunctional` 与生成 crate 继续只表达每周期采样；本修复不宣称它们能观察两次 cycle 之间的 reset 脉冲。
- 修改 `crates/bitloom-sim/src/lib.rs` 和 `docs/backend-boundary-evidence-2026-09-20.md`；无不可逆操作。`cargo fmt -p bitloom-sim --check` 与 `cargo test -p bitloom-sim` 均通过（50 tests）。

## Review Triage Log

| Finding | Verdict | Evidence |
|---|---|---|
| Async reset 与 `has_enable: true` 的组合未覆盖 | medium — patched | 新回归以 `en=0` 断言 async reset，确认寄存器立即清零；同步 reset 对照确认仍在 tick 清零。 |
| spec 在实施记录已有结果时仍为 in-progress | false | 审查发生时 review 与最终验收尚未结束，in-progress 正确；验收完成后现已标记 done。 |
