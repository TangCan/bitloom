# Epic 20 Context: 与 Chisel 双向互操作

<!-- Compiled from planning artifacts. Edit freely. Regenerate with compile-epic-context if planning docs change. -->

## Goal

让设计者把 Bitloom 模块导出为在钉死 Chisel+firtool 下可编译的 Chisel Scala，再经 `.fir`/导入路径回到 Bitloom/FrozenHir，并提供文档化混合夹具与 `import` CLI，从而兑现概述字面的双向互操作（不再以「尽力失败」交差）。

## Stories

- Story 20.1: NFR14 风险记录（Chisel 双向）
- Story 20.2: 架构 AD — FIRRTL→可编译 Chisel（FR28 条）
- Story 20.3: FIRRTL/FrozenHir → 可编译 Chisel（FR28）
- Story 20.4: 反向导入 Chisel/`.fir` → Bitloom（FR46 腿 2）
- Story 20.5: `import` CLI + 混合夹具（FR40 / FR46 腿 3）

## Requirements & Constraints

- 正向：FrozenHir/`.fir` → 可编译 Chisel Scala；钉死 Chisel+firtool 下编译通过；公开端口名/宽/向与实例层次满足往返谓词；允许机械风格；禁止「结构化尽力失败」交差。
- 反向：`.fir`（及文档化 Chisel 工作流输出）→ FrozenHir 或可编辑表面 → emit/tick；对称往返谓词；至少一夹具覆盖导出再导入或外部 `.fir` 导入再 emit。
- 产品入口：`import` CLI（`--help` + smoke）+ 混合夹具（一侧 Bitloom、一侧 Chisel/`.fir` 进入同一 emit/后端路径）。
- 设计 crate 仍只依赖 `bitloom-prelude`；导入/生成工具可在 CLI/工具链 crate。
- 开工前须有本 epic 的 NFR14 风险记录（字段 a–d）；缺记录不得将后续故事标 ready。
- 「可维护」= 可编译 + 端口/层次谓词（Open Q5 已关闭）；不要求手写 idiomatic。
- 不依赖已删除的 Scala FIRRTL Parser；不要求恢复 Chisel 5 前 Parser API。
- 调试用 HIR→源码再生不得冒充本 epic 双向合同。

## Technical Decisions

- FIRRTL 文本契约仍是 FrozenHir ↔ `FIRRTL version 6.0.0` 文本；Chisel Scala 产品路径是单独 AD（AD-27），不替代文本契约。
- 钉死栈：Chisel 7.14.0 ↔ firtool 1.155.0；升钉跟 NFR12 / Chisel 正式配对。
- CIRCT 时代：交换边界为 `.fir` + firtool；无上游 Scala `Circuit` 解析便利；本工具链自建生成/导入。
- FR46 草图选项：A 自研 pretty printer+测试编译；B 仅 `.fir`+薄 wrapper（可能不满足字面）；C 绑内部 API（脆弱）。脊柱倾向 A 类路径。
- 历史 NFR9「不承诺可维护 Chisel Scala」已被概述字面升格 / FR28+FR46 推翻，须在 AD 中显式记录。

## Cross-Story Dependencies

- 依赖 Epic 19（NFR14 模板与合同/AD 解锁）与既有 FIRRTL；不依赖 Epic 21–24。
- 20.1 风险记录 → 门禁后方可 ready 20.2–20.5。
- 20.2 AD 合入 → 20.3 实现生成器；20.3 → 20.4 反向；20.3–20.4 → 20.5 CLI/混合夹具。
