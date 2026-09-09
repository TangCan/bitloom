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
- ~~调研建议的「重定义 done」作为合同策略~~ — **用户 ①C 拒绝。**（注：2026-09-09 批准的是 **路线图绿标签合同绿**，非回滚 ①C 概述 FR；见「Phase 11 合同绿」节。）

## Phase-3 identity supersession

公开产品名 **Bitloom**，发布名 **`bitloom`**。FR21 正文已于 2026-08-21 修订；实现与文档须按 Bitloom 执行。

## 2026-08-21 Update — FR71 / NFR34（默认 CI Chisel JVM 门禁）

**用户决议：** Phase 8 追加 epic（Epic 25）；先修 PRD 再 sprint tracking。

**合同：**
- **FR71：** 默认 CI required job 对黄金 `.scala` 做真实 Chisel JVM 编译；禁 skip=0；Rust 谓词不可单独交差。
- **NFR34：** Temurin Java≥17、`cache: sbt`、setup-sbt、无 `continue-on-error`；本机 `just test` 可 Rust-only + `just chisel-fr28-jvm`。
- **FR28 success** 显式指向 FR71/NFR34 为默认 CI 证据面。

**调研：** `_agile-output/planning-artifacts/research/technical-forcing-jvm-chisel-compile-in-default-ci-2026-08-21/research.md`（Pattern A）。

**ID 避撞：** FR71 / NFR34（不用 FR53/FR60/NFR15）。Epic 映射：`epics.md` Epic 25。


## 2026-09-08 Update — Phase 10 / Wave 3 深度诚实度（Correct Course）

**用户决议：** 批准 `sprint-change-proposal-2026-09-08.md` — 将调研 Wave 3 非闭包 Partial 缺口并进 **Phase 10**（Epic 31–35），与 Phase 9 闭包并行。

**合同澄清：**
- Epic/sprint `done` 表示当时 AC 的**最小合同**；若 crates 盘点仍为 Partial，须经 **depth epic**（Phase 10）收口，不得用历史 `done` 关闭深度缺口。
- 深度指针（正式 FR79+ 编号由后续 CE/PRD 增补钉死）：FR23 CDC 真 RTL、FR51 嵌套 Bundle、FR28 Mem→Chisel、FR37/48 非 stub IP、FR33/36/39 残余 Partial。
- **HLS：** 维持 **AD-25** 外挂-only；**不**立项树内调度器（Wave 3 HLS 项无新实现 epic）。
- Phase 9（FR72–78）范围不变。
- **Phase 9 Wave 0 决策表（FR72 / Story 26.2）：** `architecture/architecture-rhdl-2026-08-18/closure-decision-table-2026-09-08.md`（HLS 自由 vs 可综合分裂；FR75→Epic 28；const fn / 生成器双轨；Cap-R-58）。

**调研：** `_agile-output/planning-artifacts/research/technical-requirements-implementation-gap-generic-2026-09-08/research.md`（Wave 3）。

## 2026-09-08 Update — Phase 9 闭包合同（FR72–78 / NFR35–36）

**用户决议：** Story 26.4 — 将 `epics.md` Phase 9 inventory 正式写入 PRD（§5.9 + NFR 表），使实现与验收有合同 ID。

**合同：**
- **FR16（继承澄清）：** 周期精确路径仍拒**捕获**闭包；允许 elaborate-time 非捕获且不得以闭包对象进 `tick`。
- **FR72–FR78：** 合同解锁 → 生成器 MVP → SynthesizableClosure / comb·seq → HLS/IP → 桥接模板。
- **NFR35 / NFR36：** 两类闭包可测区分；冻前消解 / 后端无闭包 IR（Cap-R-58）。
- **门禁：** Epic 27–30 ready 依赖 NFR14 记录 + 本增补落地。

**链接：**
- 决策表（FR72 / Story 26.2）：`architecture/architecture-rhdl-2026-08-18/closure-decision-table-2026-09-08.md`
- NFR14 风险记录：`_agile-output/implementation-artifacts/nfr14-risk-phase9-closures.md`
- PRD 正文：`prd.md` §5.9 / §6 NFR35–36 / SM-8

**ID 避撞：** FR72–78 / NFR35–36（接 FR71/NFR34）；**不得**复用 FR47 或与 Phase 7「闭环」混写。Epic 映射：`epics.md` Epic 26–30。

**公开成功标准 Waves：** 生成器 → 可综合 → HLS/IP → 桥接（SM-8）。

## 2026-09-09 Update — FR81 Mem→Chisel 合同（Epic 33 / Story 33.2）

**用户决议 / 管道偏好：** 选定 **Path A — 支持文档化 Mem 子集**（非 Path B 永久非目标）；与 AD-27 可编译 Chisel Scala 产品路径一致。

**合同：**
- **唯一路径：** Path A；`emit_chisel` 对文档化子集产出可编译 Scala；子集外保留 **E0901**。
- **子集（最小）：** 单时钟 AD-21 `Mem` / `SyncReadMem`（HIR `MemDecl`，含可选常量 `init`）。
- **NFR12：** Chisel **7.14.0** ↔ firtool **1.155.0**（不得私自升版交差）。
- **NFR37：** FR28「done」+ 历史全量 Mem→E0901 ≠ FR81 深度关闭。
- **FR71：** 不得削弱 `fr28-chisel-jvm` / `just chisel-fr28-jvm`。

**链接：**
- 决策页（FR81 / Story 33.2）：`architecture/architecture-rhdl-2026-08-18/fr81-mem-chisel-contract-decision-2026-09-09.md`
- NFR14 风险记录：`_agile-output/implementation-artifacts/nfr14-risk-epic33-chisel-mem.md`
- Epic 映射：`epics.md` Epic 33

**实现指针：** Story 33.3 按 Path A 降级；33.4 ATDD + FR71 回归。

## 2026-09-09 Update — FR84 SoftF16 显式 defer（Epic 35 / Story 35.3）

**用户 / 管道决议：** 选定 **Option B — 显式 defer**（非 Option A SoftF16→HIR→emit 可综合夹具）。不得用 host-only SoftF16 或 `Bits<16>` 位向量 emit 冒充可综合浮点算子。

**合同：**
- **唯一路径：** FR84 **Option B**；SoftF16 保持 **host-only** 黄金模型（FR36 历史最小合同）。
- **未交付：** SoftF16 浮点算子 → HIR → emit 可综合路径（Option A）— **explicitly deferred**。
- **禁止话术（NFR37）：** **不得**声称 SoftF16 可综合浮点 / synthesizable SoftF16 已交付；**不得**把 `Bits<16>` emit 表面营销为可综合浮点算子库。
- **NFR14：** Epic 35 风险记录勾选 FR84 已选 B；关闭条件可经 `fr84_softf16_explicit_defer` ATDD 检查。

**链接：**
- 用户文档：`docs/fr36-rhdl-float.md`
- NFR14：`_agile-output/implementation-artifacts/nfr14-risk-epic35-residual-partials.md`
- Epic 映射：`epics.md` Epic 35 Story 35.3

## 2026-09-09 Update — FR85 Formal 非玩具夹具（Epic 35 / Story 35.4）

**用户 / 管道决议：** 选定 **Option A — 实现**（非 Option B 显式 defer）：真实设计导出 SVA + 文档钉死的外部 checker 调用。

**合同：**
- **唯一路径：** FR85 **Option A**；Counter HIR → `rhdl_formal::emit_sva` → `crates/rhdl-formal/fixtures/fr85_counter_sva.sv`；入口 `just formal-sva-check` / `scripts/formal-sva-check.sh`（默认 `verilator --lint-only --assert`；可选 `sby`）。
- **禁止话术（NFR14 / NFR37）：** **不得**用 `check_sva_text` / toy 字符串启发式关闭 FR85 或宣称 FR39 深度完成；缺 checker 时脚本须 **非零退出**（禁止 silent success）。
- **LSP：** hover/goto **继续 deferred**；**不是** Epic 35 完成条件（见 `docs/fr38-viz-lsp.md`）。
- **NFR14：** Epic 35 风险记录勾选 FR85 已选 A 与全部关闭条件；可经 `fr85_formal_fixture_beyond_toy` ATDD 检查。

**链接：**
- 用户文档：`docs/fr39-formal-sva.md`；LSP：`docs/fr38-viz-lsp.md`
- NFR14：`_agile-output/implementation-artifacts/nfr14-risk-epic35-residual-partials.md`
- Epic 映射：`epics.md` Epic 35 Story 35.4

## 2026-09-09 Update — Phase 11 合同绿（Correct Course）

**用户决议：** 批准 `sprint-change-proposal-2026-09-09.md` —
将「产品做完 / docs/requirements/19 七阶段全绿」定义为 **合同绿**（research
`technical-doc19-seven-stage-full-green-product-pla-2026-09-09`），正式 **Phase 11**
（Epic 36–39 · **FR87–FR93** / **NFR38–NFR39**）。

**与 2026-08-21 ①C 的边界（必须同时读）：**
- ①C 升格的 FR46–52 及后续深度 FR71–86 **仍然有效**；已交付验收 **不回滚**。
- ①C「拒绝重定义 done」适用于当时「概述愿景是否升格为硬 FR」的决策，
  **不**禁止其后用合同条款定义 **路线图阶段五–七的「绿」标签** 与剩余生态缺口。
- 对外「概述愿景已全部兑现 / 七阶段字面全绿」类表述：在合同绿落地前 **禁止**；
  落地后仅可按 **FR87 / NFR38** 合同条款宣称。

**合同指针：** 详见 `epics.md` Phase 11 Inventory（FR87–FR93 / NFR38–NFR39）。
- 调研：`_agile-output/planning-artifacts/research/technical-doc19-seven-stage-full-green-product-pla-2026-09-09/research.md`
- 变更提案：`_agile-output/planning-artifacts/sprint-change-proposal-2026-09-09.md`

**FR93 永久非目标（公开清单指针）：** 树内/自研 HLS 调度器；FIRRTL→idiomatic Scala；默认 TLM≡CA 形式证明；VIP 级全协议 IP；按键全设计 elaborate 的 netlist LSP。正文见 README「状态与 deferred」§永久非目标（FR93）与 `_agile-output/implementation-artifacts/deferred-work.md` 同名节。**历史锁定语：** 原「须新 PRD 才能推翻」——已由下方 **Phase 12** 满足并推翻。

## 2026-09-09 Update — Phase 12 字面绿（Path B）（Correct Course）

**用户决议：** 批准 `sprint-change-proposal-2026-09-09-phase12-path-b.md` —
在 Phase 11 合同绿结项之后，继续追求 **字面七阶段全绿（Path B / B1）**，正式 **Phase 12**
（Epic 40–47 · **FR94–FR105** / **NFR40–NFR43**）。

**公开品牌（不变）：** **Bitloom**；crates.io / CLI **`bitloom`** / `bitloom-*`（设计 crate 仍只依赖
`bitloom-prelude`）。禁止发布 `rhdl` / `rhdl-bits`。

**与 Phase 11 / FR93 的关系（必须同时读）：**
- Phase 11 **FR87 / NFR38 合同绿**仍为**历史已交付里程碑**；已交付工程 **不回滚**。
  FR87 **不再**作为「产品做完 / 七阶段全绿」的**唯一**完成口径。
- 本更新 **推翻 FR93** 五条永久非目标锁定（闸门 **FR94**）；逐条收缩为可交付：
  1. 树内 / 自研 HLS 调度 → **FR95** / **FR96**
  2. FIRRTL→idiomatic / 可维护 Chisel Scala → **FR97**
  3. 默认 TLM≡CA / 形式等价 → **FR100**（SystemC TLM 产品 → **FR101**）
  4. VIP 级全协议 IP → **FR98**
  5. 按键全设计 elaborate netlist LSP → **FR99**
  （扩展字面绿面另见 **FR102–FR105**。）
- Research `technical-doc19-seven-stage-full-green-product-pla-2026-09-09` 曾建议勿走字面全绿；
  **产品强制选择 B1**，接受多年/高维护（**NFR40**）。
- 对外「七阶段字面全绿 / 产品字面做完」：**仅**可在 FR94–105 对应门关闭后，按 **NFR42** 宣称；
  **禁止**继续用 FR87 合同绿冒充字面 B。

**实现闸门：** Epic 40（Story 40.1–40.4）关闭前，Epic 41–47 不得标 ready。  
须修订 ARCHITECTURE-SPINE **AD-5 / AD-25 / AD-27** 及 doc-19 / README / deferred（Story 40.3–40.4 · **NFR41**）——**Story 40.4 已落地**（脊柱修订 + README/deferred 撤销 FR93 当前锁）。

**合同指针：** 详见 `epics.md` Phase 12 Inventory（FR94–FR105 / NFR40–NFR43）。
- 变更提案：`_agile-output/planning-artifacts/sprint-change-proposal-2026-09-09-phase12-path-b.md`
- 调研（反建议字面；已被本决议否决为排期默认）：`research/technical-doc19-seven-stage-full-green-product-pla-2026-09-09/research.md`
