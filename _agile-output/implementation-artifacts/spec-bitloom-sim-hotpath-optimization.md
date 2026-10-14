---
title: 'bitloom-sim 热路径槽位化与计划复用'
type: 'feature'
created: '2026-10-14'
status: 'done'
route: 'dispatch'
review_loop_iteration: 0
context: []
baseline_commit: 'cff3424a564437939d80cc50a26200eac3c962a5'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem：** RV32 Fibonacci 基准使用的本地 bitloom-sim fork 比 crates.io 1.2.1 快约 6.9–8.1 倍。源码审查发现收益来自 fork 里尚未进入上游的模拟器执行路径优化；1.2.1 本身与发布标签一致，并非打包遗漏。

**Approach：** 将 fork 中语义等价的内部快路径整理并移植到上游：构造时解析信号/内存/目标槽位和组合表达式，运行时通过连续数组访问；复用组合计划与公开 PortValues 已有键。保留现有 API、覆盖报告默认行为、周期语义和波形行为。

## Boundaries & Constraints

**Always：** Interpreter 与 Compiled 引擎保持当前逐周期端口、寄存器、内存和覆盖报告语义。未解析信号、未知内存和越界内存读取保持零值回退；信号截断宽度、同步内存读延迟、reset/enable 与 read-before-write 顺序不变。公开 `PortValues` 内容与键集合保持一致。

**Never：** 不移植本地 fork 的覆盖率开关、FST/VCD 错误处理、SystemC、formal 等独立行为变更；不改变 crate 版本、公开 API 或覆盖率默认值；不整体覆盖上游源码。

## I/O & Edge-Case Matrix

| 场景 | 输入/状态 | 预期行为 | 错误处理 |
|---|---|---|---|
| 已解析组合表达式 | 多级依赖、窄目标及算术/比较/Mux | 两引擎输出及覆盖命中与当前语义相同 | 无 |
| 未解析引用或目标 | HIR 名称不在槽位映射中 | 保持原有零值/64 位默认截断语义 | 不 panic |
| 内存读取 | 多 bank、有效/越界地址、同步读取 | bank 映射、零值回退和一周期延迟不变 | 不 panic |
| 端口写回 | 已有键、首次零值键、未知输入键 | 保留既有值并按当前规则插入或透传 | 无 |

</frozen-after-approval>

## Code Map

- `crates/bitloom-sim/src/engine.rs`：现有 `CompiledKernel` 保存名称与 `AssignExpr`；组合依赖排序可复用。fork 的 `runtime-resolved-expressions.patch` 和 `runtime-resolved-target-slots.patch` 展示 ID 化表达式及目标宽度方案。
- `crates/bitloom-sim/src/lib.rs`：`Sim::lookup`、`eval`、`tick_combinational`、`tick_compiled`、`tick_sequential` 管理热路径；输入、寄存器提交、组合写回都必须同步槽位。公开 `ports()`、波形和覆盖报告行为不变。
- `crates/bitloom-sim/src/coverage.rs`：当前上游已有索引采样优化；复用现有 signal/Mux slot API，不重复移植 fork 的 coverage-fastpath。
- `_agile-output/implementation-artifacts/sprint-status.yaml`：任务为自由形式性能工作，不挂接新 Epic/Story。
- `/nvme_data2/richard/2026/plugctx_learning/third_party/bitloom-sim-1.2.0/runtime-{plan-reuse,signal-slots,resolved-expressions,resolved-target-slots,portvalues-writeback}.patch`：候选实现及已有回归用例；逐项审查语义后移植，不直接覆盖上游文件。
- `/nvme_data2/richard/2026/plugctx_learning/rv32-cpu-bench/results/2026-10-14-registry-1.2.0-vs-1.2.1.md`：发布源码溯源及 registry/fork 同条件性能证据。

## Tasks & Acceptance

**Execution：**
- [x] `crates/bitloom-sim/src/engine.rs` -- 增加构造期 ID 化组合表达式和目标计划，保留组合拓扑顺序及未解析项回退。
- [x] `crates/bitloom-sim/src/lib.rs` -- 增加连续信号/内存槽位并覆盖所有输入、寄存器提交、内存提交和组合写回路径；复用组合计划；原地更新已有 PortValues 键。
- [x] `crates/bitloom-sim/src/lib.rs` 与相关单测 -- 覆盖两引擎的表达式、依赖链、位宽、Mux coverage、内存、多 bank、reset/enable、未知引用及 PortValues 键语义。
- [x] RV32 基准报告 -- 固定工具链、依赖锁、核心和工作量，报告正确性、样本、中位耗时与指令吞吐。

**Acceptance Criteria：**
- Given 现有支持的 HIR 与相同驱动序列，when 使用 Interpreter 或 Compiled 引擎运行，then 每周期公开端口、最终寄存器/内存可观察值及覆盖报告与原语义一致。
- Given 未解析引用、未知内存或越界地址，when 执行快路径，then 结果与原零值回退一致且不 panic。
- Given 固定的三核 Fibonacci workload，when 对比当前上游和移植版 release 构建，then 三核均至少 2× 吞吐提升，且 x2=55、64 条退休指令和周期数正确。
- Given 现有下游代码，when 编译使用 `bitloom-sim`，then 无新增公开 API 或版本/依赖变更。

## Implementation Notes


## Spec Change Log


## Review Triage Log

| Finding | Verdict and evidence | Route |
|---|---|---|
| Blind hunter：寄存器非零初始值没有同步到信号槽 | false — HIR 的 `RegDecl` 没有初始化值字段，现有 `Sim` 也始终将寄存器初始化为零；不存在可表达的非零寄存器初值差异。 | 拒绝 |
| Blind hunter：未解析表达式映射的零值回退需确认 | false — 原 AST 的 `lookup` 对未知信号返回零、未知内存查找返回零；新求值以 `usize::MAX` 作哨兵并通过安全 `.get()` 回退零，且新增未知信号、内存和目标测试。 | 拒绝 |
| Blind hunter：未覆盖全部表达式变体 | low — 初始差异缺少逐变体对照，存在回归遗漏风险；新增测试逐一比较全部 `AssignExpr` 与快速 evaluator，包含内存有效及越界读取。 | patch，已修复 |
| Blind hunter：Compiled 对非零寄存器初值缺少测试 | false — 与第一项同一事实；HIR 不支持非零寄存器初始值，Sim 两引擎均从零开始。 | 拒绝 |
| Blind hunter：需完整同输入的双引擎逐周期全状态对照 | false — 没有给出具体行为分歧；现有回归分别校验两引擎的组合输出、Mux 覆盖、顺序状态、内存延迟与 PortValues，验收所需可观察行为已有覆盖。 | 拒绝 |
| Blind hunter：reset 与 enable 的优先级未联合覆盖 | low — 分开测试不足以记录 reset 在 enable 低时仍优先清零的交互；新增两引擎联合测试并验证 reset 解除后低 enable 保持状态。 | patch，已修复 |
| Blind hunter：应补齐空白规格小节 | false — 这是编辑本次构建规格的建议，按审查规则拒绝；`Design Notes` 已有内容，且本日志按流程记录审查结论。 | 拒绝 |
| Edge-case hunter | 无发现。 | — |
| Verification-gap reviewer | 无验证缺口。 | — |

## Design Notes

表达式解析应只在 `Sim` 构造期进行。动态运行时仍保留按名称的公开端口和值映射；连续槽位只作为内部执行状态。先让所有写路径维护同一槽位真值，再启用数值化 evaluator，避免双份状态漂移。覆盖率与公开名称报告继续走上游现有实现。

## Verification

**Commands：**
- `cargo test -p bitloom-sim` -- 目标单元测试通过。
- `cargo fmt --check --all` -- 格式检查通过。
- RV32 Fibonacci release 基准 -- 三核正确性断言通过并满足吞吐验收。
