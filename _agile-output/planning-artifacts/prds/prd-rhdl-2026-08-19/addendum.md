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
- **NFR12：** Chisel **7.15.0** + firtool **1.159.0**（FR182 *unpaired product-pin* / AD-9 revised 2026-09-14；不得私自升版交差）。FR173 配对关闭证据 **7.15.0 ↔ 1.158.0** 仍有效（NFR83）；更早 **7.14.0 ↔ 1.155.0** 仍有效（NFR78）。
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

## 2026-09-10 Update — Phase 13 MVP→商业加深（Correct Course）

**用户决议：** 批准 `sprint-change-proposal-2026-09-10-phase13-mvp-commercial-deepen.md` —
在 Phase 12 字面绿 **MVP 已关闭**之后，正式 **Phase 13**「MVP→商业加深」
（Epic 48–56 · **FR106–FR115** / **NFR44–NFR47**）。

**公开品牌（不变）：** **Bitloom**；crates.io / CLI **`bitloom`** / `bitloom-*`（设计 crate 仍只依赖
`bitloom-prelude`）。禁止发布 `rhdl` / `rhdl-bits`。

**与 Phase 12 的关系（必须同时读）：**
- Phase 12 **FR94–FR105 / NFR40–NFR43** 关闭证据 **仍有效**；**不得**改写为失败（**NFR44**）。
- Phase 13 是 **新合同**下的加深（optional product → 显式 FR），**不是**「Phase 12 AC 未达标」的补救叙事。
- deferred optional 升格映射：
  1. SystemC TLM AT / `nb_transport` → **FR107**
  2. GPIO 近 VIP → **FR108**
  3. FSM / state-visit 覆盖率（C3）→ **FR109**
  4. 树内 HLS 商业深度 → **FR110**
  5. Idiomatic Chisel 可维护深度 → **FR111**
  6. 形式等价 / 双模型深度 → **FR112**
  7. LSP 设计根发现加深 → **FR113**
  8. Tywaves / LCOV GUI → **FR114**
  （闸门 **FR106**；宣称纪律 **FR115**。）
- 对外「商业加深 / 非 MVP」类表述：**仅**可在对应 FR106–114 关闭后，按 **FR115** 宣称；
  **禁止**用 Phase 12 MVP 冒充商业完整面。

**实现闸门：** Epic 48（Story 48.1–48.4 · **FR106**）关闭前，Epic 49–56 不得标 ready。  
触及 **AD-5 / AD-25 / AD-27** 的加深须在实现 epic 引用修订 AD（**NFR46**）；README / deferred /
脊柱指针由 Story **48.3–48.4** 落地。

**合同指针：** 详见 `epics.md` Phase 13 Inventory（FR106–FR115 / NFR44–NFR47）。
- 变更提案：`_agile-output/planning-artifacts/sprint-change-proposal-2026-09-10-phase13-mvp-commercial-deepen.md`

## 2026-09-10 Update — Phase 14 NFR47 未选加深升格（Correct Course）

**用户决议：** 批准 `sprint-change-proposal-2026-09-10-phase14-nfr47-deferred-deepen.md` —
在 Phase 13 MVP→商业加深 **已关闭**之后，正式 **Phase 14**「NFR47 未选加深升格」
（Epic 57–63 · **FR116–FR123** / **NFR48–NFR51**）。

**公开品牌（不变）：** **Bitloom**；crates.io / CLI **`bitloom`** / `bitloom-*`（设计 crate 仍只依赖
`bitloom-prelude`）。禁止发布 `rhdl` / `rhdl-bits`。

**与 Phase 12/13 的关系（必须同时读）：**
- Phase 12 **FR94–FR105** 与 Phase 13 **FR106–FR115** 关闭证据 **仍有效**；**不得**改写为失败（**NFR48**）。
- Phase 14 是 **新合同**下的加深（NFR47 deferred → 显式 FR），**不是**「Phase 13 AC 未达标」的补救叙事。
- deferred 升格映射：
  1. Tywaves 级 typed IDE 波形 → **FR117**
  2. 无 metadata 全树 `#[bitloom::top]` syn-scan → **FR118**
  3. SymbiYosys/SMT（原 FR112 分支 A）→ **FR119**
  4. 商业 VIP GPIO 全家桶 → **FR120**
  5. Handshake 默认可综合 → **FR121**
  6. 官方风格 Chisel 全家桶 → **FR122**
  （闸门 **FR116**；宣称纪律 **FR123**。）
- 对外「Tywaves / syn-scan / SBY / VIP GPIO / Handshake / 官方风格全家桶」类表述：**仅**可在对应 FR116–122 关闭后，按 **FR123** 宣称；
  **禁止**用 Phase 13 完成面冒充本批加深。

**实现闸门：** Epic 57（Story 57.1–57.4 · **FR116**）关闭前，Epic 58–63 不得标 ready。  
触及 **AD-25 / AD-27**（及 formal/SBY 路径）的加深须在实现 epic 引用修订 AD（**NFR50**）；README / deferred /
脊柱指针由 Story **57.3–57.4** 落地。

**合同指针：** 详见 `epics.md` Phase 14 Inventory（FR116–FR123 / NFR48–NFR51）。
- 变更提案：`_agile-output/planning-artifacts/sprint-change-proposal-2026-09-10-phase14-nfr47-deferred-deepen.md`

## 2026-09-10 Update — Phase 15 NFR51 剩余升格（Correct Course）

**用户决议：** 批准 `sprint-change-proposal-2026-09-10-phase15-nfr51-leftover-deepen.md` —
在 Phase 14 NFR47 未选加深升格 **已关闭**之后，正式 **Phase 15**「NFR51 剩余升格」
（Epic 64–71 · **FR124–FR132** / **NFR52–NFR55**）。

**公开品牌（不变）：** **Bitloom**；crates.io / CLI **`bitloom`** / `bitloom-*`（设计 crate 仍只依赖
`bitloom-prelude`）。禁止发布 `rhdl` / `rhdl-bits`。

**与 Phase 12–14 的关系（必须同时读）：**
- Phase 12 **FR94–FR105**、Phase 13 **FR106–FR115** 与 Phase 14 **FR116–FR123** 关闭证据 **仍有效**；**不得**改写为失败（**NFR52**）。
- Phase 15 是 **新合同**下的加深（NFR51 deferred → 显式 FR），**不是**「Phase 14 AC 未达标」的补救叙事。
- deferred 升格映射：
  1. 上游 Tywaves 一等集成 → **FR125**
  2. 更多 IP 手写 FL（formal 分支 C）→ **FR126**
  3. 默认 CI 强制真 sby → **FR127**
  4. 全 SoC pad / 商业对拍深度 → **FR128**
  5. CIRCT Handshake / 多时钟弹性缓冲全家桶 → **FR129**
  6. 完整 Style Guide ± Parser 恢复 → **FR130**
  7. `ip.rs` 按协议拆分卫生 → **FR131**
  （闸门 **FR124**；宣称纪律 **FR132**。）
- 对外「Tywaves 一等 / 更多 IP FL / 强制 sby CI / 全 SoC pad / CIRCT Handshake / Style Guide·Parser / ip.rs 拆分」类表述：**仅**可在对应 FR124–131 关闭后，按 **FR132** 宣称；
  **禁止**用 Phase 14 完成面冒充本批加深。

**实现闸门：** Epic 64（Story 64.1–64.4 · **FR124**）关闭前，Epic 65–71 不得标 ready。  
触及 **AD-25 / AD-27**（及 CI formal/sby）的加深须在实现 epic 引用修订 AD（**NFR54**）；README / deferred /
脊柱指针由 Story **64.3–64.4** 落地。软实现序：建议 **Epic 71 先于 Epic 68**（同触 `ip.rs`）。

**合同指针：** 详见 `epics.md` Phase 15 Inventory（FR124–FR132 / NFR52–NFR55）。
- 变更提案：`_agile-output/planning-artifacts/sprint-change-proposal-2026-09-10-phase15-nfr51-leftover-deepen.md`

## 2026-09-11 Update — Phase 16 产品终局结项（Correct Course）

**用户决议：** 批准 `sprint-change-proposal-2026-09-11-phase16-nfr55-final-closeout.md` —
在 Phase 15 NFR51 剩余升格 **已关闭**之后，正式 **Phase 16**「产品终局结项 / NFR55 未选加深升格」
（Epic 72–78 · **FR133–FR140** / **NFR56–NFR59**）。

**公开品牌（不变）：** **Bitloom**；crates.io / CLI **`bitloom`** / `bitloom-*`（设计 crate 仍只依赖
`bitloom-prelude`，除非 FR139 另开）。禁止发布 `rhdl` / `rhdl-bits`。

**与 Phase 12–15 的关系（必须同时读）：**
- Phase 12 **FR94–FR105**、Phase 13 **FR106–FR115**、Phase 14 **FR116–FR123** 与 Phase 15 **FR124–FR132** 关闭证据 **仍有效**；**不得**改写为失败（**NFR56**）。
- Phase 16 是 **新合同**下的加深（NFR55 deferred → 显式 FR），**不是**「Phase 15 AC 未达标」的补救叙事。
- deferred 升格映射：
  1. 真实上游 Tywaves GUI / IDE 插件深度 → **FR134**
  2. 更多 IP 手写 FL（超 Gpio）→ **FR135**
  3. 多外设 / 全芯片 pad 环 → **FR136**
  4. 完整外部 CIRCT 编译 / 仿真门禁 → **FR137**
  5. 恢复废弃 Scala `Parser.parse`（须再修订 AD-27）→ **FR138**
  6. VIP/SocPad 再细拆或跨 crate → **FR139**
  （闸门 **FR133**；宣称纪律 **FR140**。）
- 对外「终局 / Tywaves GUI·IDE / 更多 IP FL / 全芯片 pad / 外部 CIRCT 门禁 / Parser 恢复 / IP 跨 crate」类表述：**仅**可在对应 FR133–139 关闭后，按 **FR140** 宣称；
  **禁止**用 Phase 15 完成面冒充本批加深。
- **终局口径：** 本批 FR 关闭 + 诚实列出 NFR59 deferred；**不等于**冲 1.0 或 backlog 永久空（NFR15）。

**实现闸门：** Epic 72（Story 72.1–72.4 · **FR133**）关闭前，Epic 73–78 不得标 ready。  
触及 **AD-27**（FR138 **必须**再修订）及外部 CIRCT/crate 边界须在实现 epic 引用（**NFR58**）；README / deferred /
脊柱指针由 Story **72.3–72.4** 落地。软实现序：建议 **Epic 78 先于 Epic 74/75**（同触 `ip/`）。

**合同指针：** 详见 `epics.md` Phase 16 Inventory（FR133–FR140 / NFR56–NFR59）。
- 变更提案：`_agile-output/planning-artifacts/sprint-change-proposal-2026-09-11-phase16-nfr55-final-closeout.md`

## 2026-09-11 Update — Phase 17 公开 API 稳定门 / Bitloom 1.0（Correct Course）

**用户决议：** 批准 `sprint-change-proposal-2026-09-11-phase17-api-stability-1-0.md` —
在 Phase 16 产品终局结项 **已关闭**之后，正式 **Phase 17**「公开 API 稳定门」（Epic 79–83 · **FR141–FR147** / **NFR60–NFR63**），
使维护者可对 **钉死表面** 合法发布 SemVer **1.0.0**。

**批准默认（Q1–Q5）：**
1. **`bitloom-sim` 纳入** 1.0 表面（`tick` / VCD / 公开双模型 API）。
2. **`bitloom-hir` / `bitloom-builder` / `bitloom-vlog`：** 可继续 crates.io publish，**不**进入 1.0 稳定承诺。
3. **Epic 82（FR145）** 可按 Epic 80 清单 **skip**（无阻塞 breaking 时）。
4. **不以**先消化 **NFR59** 为 1.0 前提。
5. **MSRV** 默认保持现行；上调另开并写入 1.0 政策。

**公开品牌（不变）：** **Bitloom**；crates.io / CLI **`bitloom`** / `bitloom-*`；设计 crate 仍只依赖
`bitloom-prelude`（AD-6）。禁止发布 `rhdl` / `rhdl-bits`。

**与 Phase 12–16 的关系（必须同时读）：**
- Phase 12–16（FR94–140）关闭证据 **仍有效**；**不得**改写为失败（**NFR60**）。
- Phase 17 是 **新合同**下的稳定门，**不是**「Phase 16 AC 未达标」的补救叙事。
- 映射：闸门 **FR141**；表面清单 **FR142**；SemVer 1.0 政策 **FR143**；破坏性变更 CI **FR144**；
  预 1.0 卫生 **FR145**（可选）；发版 **FR146**；宣称纪律 **FR147**。
- 对外「1.0 / 公开 API 稳定」类表述：**仅**可在对应 FR141–146 关闭后，按 **FR147** 宣称；
  **禁止**用 Phase 16 终局 alone 冒充 1.0。
- **1.0 口径：** 对 FR142 钉死表面承诺 major 稳定；**不等于**清空 NFR59 / 永不变更内部实现（**NFR63**）。

**实现闸门：** Epic 79（Story 79.1–79.4 · **FR141**）关闭前，Epic 80–83 不得标 ready。  
顺序：**79 → 80 → 81 →（82 按需）→ 83**。README / deferred / 脊柱指针由 Story **79.3–79.4** 落地。
**不**在本合同批准瞬间执行 `cargo publish`——发版属 **FR146 / Epic 83**。

**合同指针：** 详见 `epics.md` Phase 17 Inventory（FR141–FR147 / NFR60–NFR63）。
- 变更提案：`_agile-output/planning-artifacts/sprint-change-proposal-2026-09-11-phase17-api-stability-1-0.md`

## 2026-09-11 Update — Phase 18 CLI / 依赖 crate crates.io 可发布收口（Correct Course）

**用户决议：** 批准 `sprint-change-proposal-2026-09-11-phase18-cli-crates-io-publish.md` —
在 Phase 17 公开 API 稳定门 **已关闭**且库 crate **1.0.0** 已上 crates.io 之后，正式 **Phase 18**
「CLI / 依赖 crate crates.io 可发布」（Epic 84–86 · **FR148–FR153** / **NFR64–NFR67**），
收口 FR146 明示的 **`bitloom` CLI** 手工 follow-up。

**批准默认（Q1–Q5）：**
1. **`rhdl-firrtl` → `bitloom-firrtl`**（AD-2；`publish=true`；禁止以 `rhdl-firrtl` 为 crates.io 名）。
2. **`rhdl-viz` → `bitloom-viz`**（同上）。
3. **FR152 (b)：** `bitloom-lsp` 默认可保持 `publish=false`，**必须**不挡 `bitloom` 打包；(a) 另开合同。
4. **不以**先消化 **NFR59** 为 CLI 上架前提。
5. **MSRV** 默认保持现行；新包与工作区 **1.0.0** 对齐（除非故事另决）。

**公开品牌（不变）：** **Bitloom**；crates.io / CLI **`bitloom`** / `bitloom-*`；设计 crate 仍只依赖
`bitloom-prelude`（AD-6）。禁止发布 `rhdl` / `rhdl-bits`。

**与 Phase 12–17 的关系（必须同时读）：**
- Phase 12–17（FR94–147）关闭证据 **仍有效**；**不得**改写为失败（**NFR64**）。
- Phase 18 是 **新合同**下的 CLI 发布收口，**不是**「Phase 17 / 1.0 失败」的补救叙事。
- 映射：闸门 **FR148**；`bitloom-firrtl` **FR149**；`bitloom-viz` **FR150**；CLI 上架 **FR151**；
  lsp 策略 **FR152**；发版后诚实 / SemVer 跟进 **FR153**。
- 对外「CLI 已可从 crates.io 安装」类表述：**仅**可在对应 FR148–153 关闭后宣称；
  **禁止**在 FR151 关闭前暗示 `cargo install bitloom` 已可用。
- **口径：** CLI 上架 ≠ 清空 NFR59 / 扩大 FR142 表面（**NFR67**）。

**实现闸门：** Epic 84（Story 84.1–84.4 · **FR148**）关闭前，Epic 85–86 不得标 ready。  
顺序：**84 → 85 → 86**。README / deferred / 脊柱指针由 Story **84.3–84.4** 落地；实发属 Epic **85**。  
**不**在本合同批准瞬间强制 `cargo publish`。

**合同指针：** 详见 `epics.md` Phase 18 Inventory（FR148–FR153 / NFR64–NFR67）。
- 变更提案：`_agile-output/planning-artifacts/sprint-change-proposal-2026-09-11-phase18-cli-crates-io-publish.md`

## 2026-09-12 Update — Phase 19 NFR59 全子集升格 + FR152(a) bitloom-lsp 上架（Correct Course）

**用户决议：** 批准 `sprint-change-proposal-2026-09-12-phase19-nfr59-fr152a.md` —
在 Phase 18 CLI crates.io 可发布 **已关闭**之后，正式 **Phase 19**
「NFR59 全子集升格 + FR152(a)」（Epic 87–98 · **FR154–FR165** / **NFR68–NFR72**），
打开曾被 **NFR63 / NFR67** 禁止静默吞并的 NFR59 账本，并升格 Phase 18 预留的 **FR152(a)**。

**批准默认（Q1–Q5）：**
1. **NFR59 README 九条全部**升格为 **FR157–FR165**（各 epic NFR14 钉死验收子集；禁止静默超子集 — NFR71）。
2. **FR152 (a)：** `bitloom-lsp`：`publish = true` + version；重写 `docs/fr152-*`；(a)；**live** `cargo publish -p bitloom-lsp`（Epic 88 / FR155）；不得破坏已关闭 FR151。
3. **不得**改写 Phase 12–18「已关闭」（**NFR68**）。
4. **不得**静默扩大 FR142；宣称须引已关 FR（**FR156** / **NFR72**）。
5. **MSRV** 默认保持现行；lsp 版本建议与工作区 **1.0.0** 对齐（除非故事另决）。

**公开品牌（不变）：** **Bitloom**；crates.io / CLI **`bitloom`** / `bitloom-*`；设计 crate 仍只依赖
`bitloom-prelude`（AD-6）。禁止发布 `rhdl` / `rhdl-bits`。

**与 Phase 12–18 的关系（必须同时读）：**
- Phase 12–18（FR94–153）关闭证据 **仍有效**；**不得**改写为失败（**NFR68**）。
- Phase 19 是 **新合同**下的加深与 lsp 上架，**不是**「Phase 18 / CLI 失败」的补救叙事。
- 映射：闸门 **FR154**；lsp (a) **FR155**；宣称 **FR156**；NFR59 九条 **FR157–FR165**。
- 对外「lsp 已上架 / NFR59 某子集已交付」类表述：**仅**可在对应 FR 关闭后宣称；
  **禁止**用 Phase 18 alone 冒充。
- **口径：** 本批关闭 ≠ 清空超出各 NFR14 钉死子集的更深项（**NFR71**）；`git push` **不是** FR。

**实现闸门：** Epic 87（Story 87.1–87.4 · **FR154**）关闭前，Epic 88–98 不得标 ready。  
软序：**87 →（88 ‖ 89…97）→ 98**；Epic 95（`ip/`）与其它触及面建议串行。  
README / deferred / 脊柱指针由 Story **87.3–87.4** 落地；lsp live 属 Epic **88**。  
**不**在本合同批准瞬间强制 `cargo publish`。

**合同指针：** 详见 `epics.md` Phase 19 Inventory（FR154–FR165 / NFR68–NFR72）。
- 变更提案：`_agile-output/planning-artifacts/sprint-change-proposal-2026-09-12-phase19-nfr59-fr152a.md`

## 2026-09-12 Update — Phase 20 NFR71 四条升格（Correct Course）

**用户决议：** 批准 `sprint-change-proposal-2026-09-12-phase20-nfr71-four-leftovers.md` —
在 Phase 19 NFR59 升格 + FR152(a) **已关闭**之后，正式 **Phase 20**
「NFR71 四条升格」（Epic 99–104 · **FR166–FR171** / **NFR73–NFR77**），
打开曾被 **NFR71** 明确要求另开合同的四条 leftover。

**批准默认（Q1–Q5）：**
1. **NFR71 四条全部**升格为 **FR167–FR170**（各 epic NFR14 钉死验收子集；禁止静默超子集 — NFR76）。
2. **FR168：** SPI + I2C + AXI **三者皆**手写 FL ≡ tick（禁止「至少一项」交差）。
3. **不得**改写 Phase 12–19「已关闭」（**NFR73**）。
4. **不得**静默扩大 FR142；宣称须引已关 FR（**FR171** / **NFR77**）。
5. **MSRV** 默认保持现行；firtool 升钉须上游 Chisel **正式配对**后修订 **AD-9**（NFR12 / NFR75）；HEAD Parser 须修订 **AD-27**（±AD-9）。

**公开品牌（不变）：** **Bitloom**；crates.io / CLI **`bitloom`** / `bitloom-*`；设计 crate 仍只依赖
`bitloom-prelude`（AD-6）。禁止发布 `rhdl` / `rhdl-bits`。

**与 Phase 12–19 的关系（必须同时读）：**
- Phase 12–19（FR94–165）关闭证据 **仍有效**；**不得**改写为失败（**NFR73**）。
- Phase 20 是 **新合同**下的加深，**不是**「Phase 19 失败」的补救叙事。
- 映射：闸门 **FR166**；ChiselSim/多商店 **FR167**；SPI·I2C·AXI FL **FR168**；CIRCT/firtool **FR169**；HEAD Parser **FR170**；宣称 **FR171**。
- 对外「四条已交付」类表述：**仅**可在对应 FR 关闭后宣称；
  **禁止**用 Phase 19 alone 冒充。
- **口径：** 本批关闭 ≠ 清空超出各 NFR14 钉死子集的更深项（**NFR76**）；`git push` **不是** FR。

**实现闸门：** Epic 99（Story 99.1–99.4 · **FR166**）关闭前，Epic 100–104 不得标 ready。  
软序：**99 →（100 ‖ 101 ‖ 102 ‖ 103）→ 104**；Epic 101（`ip/`）与其它触及面建议串行；Epic 102 升钉配对建议先于 Epic 103。  
README / deferred / 脊柱指针由 Story **99.3–99.4** 落地；实现加深属 Epic **100–103**。  
**不**在本合同批准瞬间强制商店 live publish / firtool 升钉。

**合同指针：** 详见 `epics.md` Phase 20 Inventory（FR166–FR171 / NFR73–NFR77）。
- 变更提案：`_agile-output/planning-artifacts/sprint-change-proposal-2026-09-12-phase20-nfr71-four-leftovers.md`

## 2026-09-12 Update — Phase 21 NFR76 leftovers 升格（Correct Course）

**用户决议：** 批准 `sprint-change-proposal-2026-09-12-phase21-nfr76-leftovers.md` —
在 Phase 20 NFR71 四条升格 **已关闭**之后，正式 **Phase 21**
「NFR76 leftovers 升格」（Epic 105–110 · **FR172–FR177** / **NFR78–NFR82**），
打开曾被 **NFR76** 明确要求另开合同的四条 leftover。

**批准默认（Q1–Q5）：**
1. **NFR76 四条全部**升格为 **FR173–FR176**（各 epic NFR14 钉死验收子集；禁止静默超子集 — NFR81）。
2. **FR173：** firtool 升钉 **必须**上游 Chisel **正式配对**后修订 **AD-9 / Stack**（禁止 unpaired bump）。
3. **不得**改写 Phase 12–20「已关闭」（**NFR78**）。
4. **不得**静默扩大 FR142；宣称须引已关 FR（**FR177** / **NFR82**）。
5. **MSRV** 默认保持现行，除非某 epic NFR14 显式另开；超出本批钉死子集仍须新合同（**NFR81**）。

**公开品牌（不变）：** **Bitloom**；crates.io / CLI **`bitloom`** / `bitloom-*`；设计 crate 仍只依赖
`bitloom-prelude`（AD-6）。禁止发布 `rhdl` / `rhdl-bits`。

**与 Phase 12–20 的关系（必须同时读）：**
- Phase 12–20（FR94–171）关闭证据 **仍有效**；**不得**改写为失败（**NFR78**）。
- Phase 21 是 **新合同**下的加深，**不是**「Phase 20 失败」的补救叙事。
- 映射：闸门 **FR172**；firtool 升钉 **FR173**；unpaired HEAD **FR174**；更广 CIRCT/MLIR/sim **FR175**；更深 Parser/Chisel 生态 **FR176**；宣称 **FR177**。
- 对外「NFR76 四条已交付」类表述：**仅**可在对应 FR 关闭后宣称；
  **禁止**用 Phase 20 alone 冒充。
- **口径：** 本批关闭 ≠ 清空超出各 NFR14 钉死子集的更深项（**NFR81**）；`git push` **不是** FR。

**实现闸门：** Epic 105（Story 105.1–105.4 · **FR172**）关闭前，Epic 106–110 不得标 ready。  
软序：**105 →（106 ‖ 107 ‖ 108 ‖ 109）→ 110**；触及 AD-9 的 Epic 106/107 建议串行或明确配对依赖。  
README / deferred / 脊柱指针由 Story **105.3–105.4** 落地；实现加深属 Epic **106–109**。  
**不**在本合同批准瞬间强制 firtool live 升钉 / HEAD 二进制。

**合同指针：** 详见 `epics.md` Phase 21 Inventory（**planning-complete** · Epic 105–110 · FR172–FR177 / NFR78–NFR82）。
- 变更提案：`_agile-output/planning-artifacts/sprint-change-proposal-2026-09-12-phase21-nfr76-leftovers.md`

## 2026-09-12 Update — Phase 22 NFR81 leftovers 升格（Correct Course）

**用户决议：** 批准 `sprint-change-proposal-2026-09-12-phase22-nfr81-leftovers.md` —
在 Phase 21 NFR76 leftovers 升格 **已关闭**之后，正式 **Phase 22**
「NFR81 leftovers 升格」（Epic 111–117 · **FR178–FR184** / **NFR83–NFR87**），
打开曾被 **NFR81** 明确要求另开合同的五条 leftover。

**批准默认（Q1–Q7）：**
1. **NFR81 五条全部**升格为 **FR179–FR183**（各 epic NFR14 钉死验收子集；禁止静默超子集 — **NFR86**）。
2. 各加深条独立 NFR14；浮动 HEAD / Handshake / Style / unpaired firtool / FR142 扩展分 epic。
3. **不得**改写 Phase 12–21「已关闭」（**NFR83**）。
4. 宣称须引已关 FR（**FR184** / **NFR87**）；**禁止**用 Phase 21 alone 冒充本批五条。
5. **「扩 FR142」= 独立 FR183**（显式公开 API 表面扩展 + SemVer/docs；不是静默扩大）。
6. **MSRV** 默认保持现行，除非某 epic NFR14 显式另开；超出本批钉死子集仍须新合同（**NFR86**）。
7. **FR182：** 无上游 Chisel 正式配对的 firtool **产品钉**再升钉须修订 **AD-9** 明示 *unpaired product-pin* 例外（≠ FR173 配对升钉；≠ FR174 optional HEAD alone）。

**公开品牌（不变）：** **Bitloom**；crates.io / CLI **`bitloom`** / `bitloom-*`；设计 crate 仍只依赖
`bitloom-prelude`（AD-6）。禁止发布 `rhdl` / `rhdl-bits`。

**与 Phase 12–21 的关系（必须同时读）：**
- Phase 12–21（FR94–177）关闭证据 **仍有效**；**不得**改写为失败（**NFR83**）。
- Phase 22 是 **新合同**下的加深，**不是**「Phase 21 失败」的补救叙事。
- 映射：闸门 **FR178**；浮动 HEAD **FR179**；Handshake dialect **FR180**；更深 Style/linter **FR181**；unpaired firtool 产品钉 **FR182**；显式 FR142 扩展 **FR183**；宣称 **FR184**。
- 对外「NFR81 五条已交付」类表述：**仅**可在对应 FR 关闭后宣称；
  **禁止**用 Phase 21 alone 冒充。
- **口径：** 本批关闭 ≠ 清空超出各 NFR14 钉死子集的更深项（**NFR86**）；`git push` **不是** FR。

**实现闸门：** Epic 111（Story 111.1–111.4 · **FR178**）关闭前，Epic 112–117 不得标 ready。  
软序：**111 →（112 ‖ 113 ‖ 114 ‖ 115 ‖ 116）→ 117**；触及 AD-9 的 Epic 112/115 建议串行。  
README / deferred / 脊柱指针由 Story **111.3–111.4** 落地；实现加深属 Epic **112–116**。  
**不**在本合同批准瞬间强制浮动 HEAD live / unpaired 产品升钉 / crates.io 破坏性发版。

**合同指针：** 详见 `epics.md` Phase 22 Inventory（create-epics 后 · Epic 111–117 · FR178–FR184 / NFR83–NFR87）。
- 变更提案：`_agile-output/planning-artifacts/sprint-change-proposal-2026-09-12-phase22-nfr81-leftovers.md`

## 2026-09-14 Update — 工程/合同结项收口（Correct Course）

**用户决议：** 批准 `sprint-change-proposal-2026-09-14-engineering-closeout.md` —
在 Phase 12–22 钉死合同面与 sprint backlog **全部关闭**之后，正式做 **工程/合同结项收口**。
**本批不开** NFR86 → Phase 23 加深合同；不新增实现 epic。

**批准默认（Q1–Q6）：**
1. 本批 = 结项收口文档与状态戳；**不**升格 NFR86 为 Phase 23 加深 FR。
2. **不**新增实现 epic；**不**改已关 Epic 1–117 AC。
3. Phase 12–22 关闭证据 **仍有效**；**不得**改写为失败。
4. 宣称：「sprint/合同 backlog 已空 / 工程结项」**≠**「NFR86 账本已空」；产品宣称仍须引已关 FR（至 **FR184** / **FR147** / **FR140** 等）。
5. **`git push` / 远程同步不在本合同内。**
6. 若日后要做 NFR86 加深，须 **另开** Correct Course（建议 Phase 23+）；**不得**从本结项 alone 冒充已授权。

**公开品牌（不变）：** **Bitloom**；crates.io / CLI **`bitloom`** / `bitloom-*`；设计 crate 仍只依赖
`bitloom-prelude`（AD-6）。禁止发布 `rhdl` / `rhdl-bits`。

**结项目径（必须同时读）：**
- Sprint 故事 / 回顾 / action items **全部 done**；`phase17Status`…`phase22Status: complete`。
- Phase 16 产品终局（FR140）、Phase 17 Bitloom 1.0（FR147）、Phase 18 CLI 上架（FR148–153）、Phase 19–22 加深升格 **仍有效**。
- **NFR86**（无界 CIRCT tip、更深 Handshake lower、社区 Style 全家桶、继续 firtool 升钉、继续扩 FR142 等）**仍 standing** — 见 `docs/fr184-phase22-claim-honesty.md`。
- **禁止**用本结项 alone 冒充：NFR86 已清、Phase 23 已批准、或未关加深已交付。

**合同指针：**
- 变更提案：`_agile-output/planning-artifacts/sprint-change-proposal-2026-09-14-engineering-closeout.md`
- `epics.md`：`engineeringCloseoutApproved` / `engineeringCloseoutStatus: complete`

## 2026-09-14 Update — Phase 23 NFR86 leftovers 升格（Correct Course）

**用户决议：** 批准 `sprint-change-proposal-2026-09-14-phase23-nfr86-leftovers.md` —
在工程/合同结项收口之后，正式 **Phase 23**「NFR86 leftovers 升格」
（Epic 118–124 · **FR185–FR191** / **NFR88–NFR92**），打开曾被 **NFR86** 明确要求另开合同的五条 leftover。

**批准默认（Q1–Q8）：**
1. **NFR86 五条全部**升格为 **FR186–FR190**（各 epic NFR14 钉死验收子集；禁止静默超子集 — **NFR91**）。
2. 各加深条独立 NFR14；无界 tip / Handshake lower / Style 全家桶 / firtool 再升钉 / FR142 再扩展分 epic。
3. **不得**改写 Phase 12–22「已关闭」或工程结项（**NFR88**）。
4. 宣称须引已关 FR（**FR191** / **NFR92**）；**禁止**用 Phase 22 alone 或工程结项 alone 冒充本批五条。
5. **「继续扩 FR142」= 独立 FR190**（显式公开 API 表面扩展 + SemVer/docs；不是静默扩大）。
6. **MSRV** 默认保持现行，除非某 epic NFR14 显式另开；超出本批钉死子集仍须新合同（**NFR91**）。
7. 触 **AD-9 / AD-25 / AD-27 / FR142** 须按 **NFR90** 先修订再 story ready。
8. **`git push` / 远程同步不在本合同内。**

**公开品牌（不变）：** **Bitloom**；crates.io / CLI **`bitloom`** / `bitloom-*`；设计 crate 仍只依赖
`bitloom-prelude`（AD-6）。禁止发布 `rhdl` / `rhdl-bits`。

**与 Phase 12–22 / 结项的关系（必须同时读）：**
- Phase 12–22（FR94–184）与工程结项关闭证据 **仍有效**；**不得**改写为失败（**NFR88**）。
- Phase 23 是 **新合同**下的加深，**不是**「Phase 22 / 结项失败」的补救叙事。
- 映射：闸门 **FR185**；无界 CIRCT tip **FR186**；Handshake lower **FR187**；Style 全家桶 **FR188**；继续 firtool 升钉 **FR189**；继续扩 FR142 **FR190**；宣称 **FR191**。
- **口径：** 本批关闭 ≠ 清空超出各 NFR14 钉死子集的更深项（**NFR91**）；`git push` **不是** FR。

**实现闸门：** Epic 118（Story 118.1–118.4 · **FR185**）关闭前，Epic 119–124 不得标 ready。  
软序：**118 →（119 ‖ 120 ‖ 121 ‖ 122 ‖ 123）→ 124**；触及 AD-9 的 Epic 119/122 建议串行。  
README / deferred / 脊柱指针由 Story **118.3–118.4** 落地；实现加深属 Epic **119–123**。

**合同指针：** 详见 `epics.md` Phase 23 Inventory（Epic 118–124 · FR185–FR191 / NFR88–NFR92）。
- 变更提案：`_agile-output/planning-artifacts/sprint-change-proposal-2026-09-14-phase23-nfr86-leftovers.md`

## 2026-09-14 Update — FR189 / Epic 122 upstream park（Correct Course）

**用户决议：** 批准 `sprint-change-proposal-2026-09-14-fr189-upstream-block.md` — 方案 **A（Park Epic 122）**。

1. FR189 验收仍要求已发布 firtool **>1.159.0** + **AD-9** 修订（**NFR90**）。
2. **不得**以 **1.159.0** / **FR182 alone** / **FR186** live tip 勾选 FR189。
3. Phase 23 **规划**故事（Epic 118–124）与 **FR191** 诚实门关闭面仍有效；实现债：**FR189 未关 / parked**。
4. 上游发布 `firtool-*` >1.159.0 后直接恢复 Story **122.2**；超子集仍 **NFR91**。
5. `git push` **不是** FR。公开品牌 **Bitloom**。

**合同指针：** `_agile-output/planning-artifacts/sprint-change-proposal-2026-09-14-fr189-upstream-block.md`

## 2026-09-14 Update — FR189 / Epic 122 延期关账（非已交付）（Correct Course）

**用户决议：** 批准 `sprint-change-proposal-2026-09-14-fr189-defer-close.md` — **延期关账（非已交付）**（在方案 A park 之后）。

1. Sprint：Epic 122 / 122.2 / 122.3 = **`deferred`**（**不是** `done`）。
2. **FR189 未交付**；不得以 1.159.0 / FR182 alone / FR186 live tip 勾选。
3. 后续「超 1.159.0 产品钉再升」归 **NFR91** — 须新合同。
4. Phase 23 规划故事（Epic 118–124）与 FR191 诚实门关闭面仍有效；宣称矩阵标 FR189 **deferred / 未交付**。
5. Story 122.1 NFR14 保持 done（形状参考）。`git push` **不是** FR。公开品牌 **Bitloom**。

**合同指针：** `_agile-output/planning-artifacts/sprint-change-proposal-2026-09-14-fr189-defer-close.md`

## 2026-09-14 Update — Phase 23 诚实结项收口（Correct Course）

**用户决议：** 批准 `sprint-change-proposal-2026-09-14-phase23-honest-closeout.md` — 用户确认 live tip **firtool-1.159.0** 已是最新，要求结项。

1. **Phase 23 诚实结项**：规划故事齐；retro / action items 齐；已关 FR185–188 / FR190–191；可停当前 sprint。
2. **FR189 仍未交付**：合同要求产品钉严格 **> 1.159.0**；tip==产品钉 ⇒ 上游暂无可升版本；**不得**因「已是最新」勾选 FR189 已交付；Epic 122 保持 **`deferred`**。
3. 后续超 1.159.0 产品钉 → **NFR91** 新合同。
4. Phase 12–22 工程结项仍有效；**≠** 「NFR86 / NFR91 账本已空」。`git push` **不是** FR。公开品牌 **Bitloom**。

**合同指针：** `_agile-output/planning-artifacts/sprint-change-proposal-2026-09-14-phase23-honest-closeout.md`

## 2026-09-20 Update — Phase 24 可组合 IP 与可验证生态

用户于 2026-09-20 批准完整规划登记及 M0 / Epic 125 执行。正式范围：Epic 125–130，FR192–201，NFR93–99。交付目标为可复用模块定义、流基础、CSR/AXI-Lite 连接、UART/GPIO/Timer/IRQ 系统，以及单独验收的一个外部小核。原 FR48、历史关闭及 Epic 122 / FR189 deferred 保留；NFR91 未清空。

Epic 125 关闭前，126–130 不得 ready/in-progress；关闭后每 Epic 实现仍须对应 .1 NFR14 done，并按依赖安排。此次授权仅执行 M0，后续保持 backlog。核心与外部独立关闭，外部未通过不得称整个 Phase 24 完成。产品实现、规划完成和发布分别记录。新增 API 必须显式更新 FR142 表面清单与 SemVer 文档，不自动进入稳定承诺；不调整工具钉。

正式接口：`docs/ip/phase24-contract.md`；故事和依赖：`epics.md` Phase 24 Inventory；M0 风险：`_agile-output/implementation-artifacts/epic-125-nfr14.md`。

| FR | 能力 | 验收摘要 | Epic |
|---|---|---|---|
| FR192 | 合同与 M0 闸门 | 接口/复位/地址/兼容边界固化，现有 AXI 缺陷修复后才开放后续实现。 | 125 |
| FR193 | 既有 AXI 接收与响应正确性 | 分拍 AW/W、读写并发、背压及 reset 的 native 与实际生成 RTL 回归通过；手写 FL 同步修正。 | 125 |
| FR194 | 同一 session 内模块组合 | 共用模块定义 helper，顶层一次 freeze；参数标识与实例图诊断，旧 elaborate 入口兼容。 | 126 |
| FR195 | 流基础组件 | 两槽注册切片和 WIDTH 1..64 / DEPTH 1..16 的寄存器 FIFO，独立参考与属性验证。 | 126 |
| FR196 | CSR 及总线连接 | 静态描述、地址文档/C 头文件、AXI-Lite 桥、单主静态译码，提交点及错误规则可验证。 | 127 |
| FR197 | 可组合外设 | Timer、IRQ、GPIO CSR、带 FIFO UART 全部交付；明确异步输入与板级边界。 | 128 |
| FR198 | 完整外设系统 | 单时钟 UART/GPIO/Timer/IRQ 系统通过真实层级 RTL 执行；干净 checkout 一条命令复现。 | 129 |
| FR199 | 外部 IP 可复现来源 | 固定完整源闭包、版本、校验、许可证材料、工具及维护人；禁网重放通过。 | 130 |
| FR200 | 一个外部小核真实适配 | 准确 module binding、参数/端口映射、关联 RTL 源和独立行为测试；空壳不达标。 | 130 |
| FR201 | 支持矩阵与生态维护 | 核心与外部状态分别记录，新增 API 表面显式登记、兼容门禁、贡献模板、维护人和证据链接。 | 129,130 |

- **NFR93：** 既有关闭证据与 FR189 deferred 保持；新发现缺陷必须公开跟踪，历史关闭不得代替当前正确性证据。
- **NFR94：** 每 Epic 实现前完成 NFR14：上游限制、工具、风险、估算、负责人、禁止静默降级；每故事一个提交。
- **NFR95：** emit 成功不等于执行成功；实际 RTL、Rust、综合、形式证明分别记录，工具缺失不得静默 skip 为通过。
- **NFR96：** 设计仅依赖 bitloom-prelude；旧 API/端口保留；新增稳定表面先显式登记；MSRV/工具钉不随本阶段变更。
- **NFR97：** 首版单时钟、32-bit 数据/16-bit 地址、小寄存器 FIFO；不包含原生层级仿真、通用 FIRRTL 内存、复杂外部核或未列协议。
- **NFR98：** 外部依赖固定完整 commit/文件闭包与校验、许可证原文和维护人；构建层显式获取，prelude 不拉取源码；无 native 模型明确 unsupported。
- **NFR99：** 收益由组合复用、上手步骤、CI 与维护成本实测；不承诺未经测量的加速/PPA。超本阶段子集需另立范围。

**2026-09-20 M0 实施关闭：** Story125.1–125.3 / Epic125 / FR192–193 已完成。证据见 `docs/ip/phase24-axi-green-evidence.md`；FR194–201 / Epic126–130 backlog，整个Phase24未交付，FR189 deferred/NFR91不变。

2026-09-20 Story126.2 / FR194 已完成：同一session模块定义复用、参数专门化与实例图/连接诊断；首批Gpio共用定义体，实际RTL验证通过。实现与证据见 docs/ip/module-composition.md 和 docs/ip/module-composition-evidence.md。126.1/126.2 done；126.3/126.4及Epic127–130保持backlog，Epic126/M1未关闭，FR195–201未交付；FR189 deferred/NFR91保持。

## M1 / Epic126 实施关闭（2026-09-20）

2026-09-20 Story126.4七步完成，随本故事单独提交关闭Epic126/M1：126.1–126.4均done，FR194模块组合与FR195两槽注册切片/参数FIFO交付。clean + fmt + just test退出0，460个实际结果块汇总1763 passed、0 failed、14 ignored；8个专用形式测试另有真实运行证据，6个既有doc-test保持ignored。最终证据 `_agile-output/test-artifacts/126-4-final-verification.md`，关闭映射 `_agile-output/implementation-artifacts/epic-126-closeout.md`。Epic127–130 / FR196–201尚待各自七步与NFR14，整个Phase24未完成；FR189/Epic122 deferred及NFR91保持，不push、不publish。

本节为当前状态，前文M0/126.2/126.3时点的范围记录保留为历史。新增API已逐符号登记FR142，属SemVer minor，未更改包版本或发布。

## M2 / Epic127 实施关闭（2026-09-21）

Story127.1–127.4随各自单故事提交完成，FR196静态CSR描述/软件地址产物、AXI-Lite桥和固定四窗CsrDecoder交付。127.4实际clean/fmt/justtest为470结果块、1823 passed / 0 failed / 25 ignored，专用decoder/CSR/bridge形式与综合本轮另跑通过，不把ignored计PASS。关闭映射 `_agile-output/implementation-artifacts/epic-127-closeout.md`，完整验收 `_agile-output/test-artifacts/127-4-final-verification.md`。新增API逐符号登记FR142/minor，未改工具钉、包版本或发布。

真实七模块夹具验证总线/CSR语义，不代表Epic128外设算法或Epic129系统；native/generated层级仍unsupported。Epic128–130/FR197–201及整个Phase24未交付；下一故事128.1 NFR14，全部未完成故事七步授权继续有效。FR189/Epic122 deferred和NFR91保持，不push、不publish。此前各时点交付状态保留为历史。

## M3 / Epic128 实施关闭（2026-09-21）

128.1–128.5七步完成，随Story128.5单独提交关闭Epic128/FR197/M3：Timer32、五源IRQ、GPIO32 CSR及双FIFO UART CSR交付。UART最终实际clean/fmt/just test：482个结果块、1870 passed / 0 failed / 47 ignored；7项新专用backend/formal入口另有实际PASS，未把ignored计PASS。当前39命令完整定向证据、独立原始归档与关闭映射见`_agile-output/test-artifacts/128-5-final-verification.md`及`_agile-output/implementation-artifacts/epic-128-closeout.md`。

新增符号逐项登记FR142/minor，旧UART/GPIO/VIP/手写FL保持。只关闭M3，Epic129/FR198与核心FR201、Epic130/FR199–200及整个Phase24仍开放，下一129.1；native/generated层级与有限形式/物理边界保持。用户全部未完故事七步授权继续有效，FR189 deferred/NFR91保留；未改工具钉/包版本、不push/publish。此前各时点记录保留为历史。
