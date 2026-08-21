# Epic 24 Context: HLS 产品路径

<!-- Compiled from planning artifacts. Edit freely. Regenerate with compile-epic-context if planning docs change. -->

## Goal

把 `#[hls]`（或等价）从「可选孤儿开关 / 未启用则 unsupported」升为产品路径：钉死单一外挂后端，文档默认可复现产出可综合 RTL，CI/烟测覆盖，文档将 HLS 列为支持功能；禁止树内自研调度器。

## Stories

- Story 24.1: NFR14 风险记录（HLS）
- Story 24.2: 钉死后端的默认 HLS 路径（FR35）
- Story 24.3: CI / 发布烟测夹具（FR35 / FR50）
- Story 24.4: 文档将 HLS 列为支持功能（FR50）

## Requirements & Constraints

- 算法级标记经外挂后端得到可综合 RTL；默认文档路径可用；不可永久 unsupported。
- 钉死 **Bambu 或 Vitis** 二选一；后端缺失须失败可读，不得 silent 成功。
- 至少一个端到端算法夹具进 CI 或发布烟测；可选 runner 允许（工具链过重），但不得零覆盖；失败时 job 不得 ignore。
- 文档将 HLS 列为支持功能并给出跟练步骤；品牌 Bitloom。
- 开工前须有本 epic 的 NFR14 风险记录（字段 a–d）；缺记录不得将 24.2–24.4 标 ready。
- 允许外挂调度；禁止 bitloom/rhdl crate 实现 scheduling/allocation。

## Technical Decisions

- AD-25：仅发射宿主可接受的 IR/C 并调用钉死后端；产品合同要求默认文档路径 + CI/烟测。
- 本 epic 钉死 **PandA Bambu 2024.10**（AppImage / `BITLOOM_BAMBU_PATH`）；不选 Vitis/XLS。
- 既有 Story 9.2 可选钩子升级为产品路径；CLI 动词 `cargo bitloom hls`。
- AD-28 / NFR14：风险门禁在 ready 之前。

## Cross-Story Dependencies

- 依赖 Epic 19（NFR14 模板）；可与 Epic 20–23 并行。
- 24.1 风险记录 → 门禁后方可 ready 24.2–24.4。
- 24.2 默认路径 → 24.3 烟测夹具 → 24.4 文档支持声明。
