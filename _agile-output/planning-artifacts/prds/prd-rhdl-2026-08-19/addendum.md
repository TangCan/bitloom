# Addendum — Bitloom 阶段二 PRD

技术 HOW、选项矩阵、调研冲突与推翻记录。不替代 `prd.md` 中的 FR。

## 2026-08-21 Update — ①C 概述字面升格

**用户决议：** Update 目的=完成所有需求项；利害=launch；Fast path；**①C** 概述字面硬 FR；**②只改 PRD**（不改 `docs/requirements/1. 项目概述.md`）。

**推翻：** 原「不以 Chisel Scala 为互转契约」「禁止 HIR→TLM/功能模拟器生成」；拒绝调研
`technical-closing-bitloom-overview-requirement-gap-2026-08-21` 的「重定义 done」主建议。

**已知工程现实（须进风险，不得再写回 non-goal）：**

- Chisel/CIRCT 维护者：自 Chisel 5 起不支持 FIRRTL 文本 → Scala `Circuit` 解析；交换边界为 `.fir` + `firtool`（issue chipsalliance/chisel#4899）。FR46 须自建「生成可编译 Scala / 导入路径」，对抗上游便利性。
- 工业 TLM↔RTL 自动等价极少；FR47/FR30 字面完成成本高。
- 同业 IP 多为 stdlib + 树外；FR48 全线一级 IP 为多年维护面。

**NFR14：** P3 必须留下风险与工期记录，禁止静默降级。**门禁形状（Finalize）：** epic → `ready` 前，每份 FR46/47/48/49（及适用的 FR50）记录须含上游约束、粗工期带、禁止静默降级清单、负责人；存放 `_agile-output/implementation-artifacts/` 或故事文件。并行多项须记维护叠加风险。

## 机制选项（FR23 CDC）— 已由 AD-22 关闭

**决议（2026-08-19）：** Clash 式 phantom 域；合法跨越仅 `DoubleFlop` / `SyncFIFO`。见架构脊柱 AD-22。

| 选项 | 优点 | 缺点 | 状态 |
|------|------|------|------|
| Clash 式 phantom domain | rustc 静态拦住误跨域 | API 噪声；学习成本 | **ADOPTED (AD-22)** |
| Spinal 式 ClockDomain + 编译检查 | 生产差异化强 | 实现复杂 | 未采纳 |
| Chisel 式库级 FIFO + 文档纪律 | 实现快 | 无语言强制 | 未采纳 |

## Mem 语义锚（FR26 / AD-21）

- **语言表面：** 暴露 CHIRRTL 友好名（`Mem` / `SyncReadMem`；文档对应 `cmem`/`smem` 语义）。**已决议 2026-08-19。**
- **降级与互转：** 锚定 FIRRTL 规范 `mem` / `firrtl.mem`；CHIRRTL 方言文本不是 FrozenHir↔`.fir` 合同。
- 双口跨时钟仅经命名 CDC FIFO（与 FR23 / AD-22 衔接）。

## NFR3 实现草图

- 资产：钉死 firtool 版本 + 平台 tar + `.sha256`。
- 缓存目录；路径覆盖 env（文档可保留 `RHDL_FIRTOOL_PATH` 别名）。
- 升级策略见 NFR12。

## HLS（FR35 / FR50）

- 仍允许发射 C/LLVM → 调用 Bambu **或** Vitis；无树内 scheduler。
- ①C 下：默认产品路径 + CI 夹具；不可永久 unsupported。

## FR46 实现草图（选项，非决议）

| 选项 | 说明 | 风险 |
|------|------|------|
| A. 自研 FIRRTL→Scala pretty printer + 测试编译 | 不依赖已删 Parser | 生成代码可维护性争议（见 Open Q5） |
| B. 仅保证 `.fir` 交换 + 薄 Scala wrapper 工程 | 较弱，可能不满足「可维护 Chisel」字面 | 验收争议 |
| C. 绑定特定 Chisel 版本内部 API | 脆弱 | 随 CIRCT bump 破碎 |

*[NOTE FOR PM] 选型属架构；须在 FR46 epic 前写入脊柱 AD。*

## FR47 实现草图（选项）

- 功能模拟器：生成 Rust crate（默认 ASSUMPTION）vs SystemC TLM。
- 周期精确：既有 `tick` / cdylib。
- 一致性：扩展现有 equiv 为生成产物对跑。

## Rejected for this PRD（历史；①C 后部分作废）

- ~~把 later-product 继续作为无 FR ID 唯一真相~~ — 仍拒绝。
- ~~把 HLS/FST 提到 P0~~ — 仍不提到 P0；改在 P2b/P3 升验收条。
- ~~不以 Chisel Scala 为契约 / 禁止 HIR→功能模拟器~~ — **①C 作废；见 prd §0 推翻表。**
- ~~调研建议的「重定义 done」作为合同策略~~ — **用户 ①C 拒绝。**

## Phase-3 identity supersession

公开产品名 **Bitloom**，发布名 **`bitloom`**。FR21 正文已于 2026-08-21 修订；实现与文档须按 Bitloom 执行。
