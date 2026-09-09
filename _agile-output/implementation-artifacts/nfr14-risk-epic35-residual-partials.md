# NFR14 风险记录 — Epic 35 残余 Partial 收口（FR83 / FR84 / FR85）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 10 **NFR37**（规划 done ≠ 深度 done）。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic34-ip-baseline.md`；`nfr14-risk-epic33-chisel-mem.md`。  
> **前置：** Epic 34（FR82 IP 基线）已 **done**；本 epic 可并行，排程优先级低于 31–34。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 35 后续故事 **35.2–35.4** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR83 / FR84 / FR85（加深 FR33 / FR36 / FR39）/ Epic 35；NFR14、NFR37 |
| 记录日期 | 2026-09-09 |
| 状态 | draft — Story 35.1；关闭条件待 35.4 勾选 |

### (a) 上游约束

- **crates Partial 现状（Wave 3 证据）：**
  - **C ABI / FR33：** `rhdl-cabi` cdylib 硬编码 **Counter-only** DUT（`counter_hir()`）；无文档化第二 DUT / 通用 generate·link 路径 ⇒ **FR83** 深度未关闭。
  - **SoftF16 / FR36：** `rhdl-float::SoftF16` 为 **host-only** 黄金模型；emit 表面按 `Bits<16>` 位向量对待，**无可综合浮点算子 → HIR → emit** 夹具 ⇒ **FR84** 须二选一（实现或显式 defer）。
  - **Formal / FR39：** `rhdl-formal::check_sva_text` 为 **toy check**（字符串启发式，非真实 checker / 工具链调用）⇒ **FR85** 深度未关闭。
  - **LSP / FR38 切片：** hover/goto **继续 deferred**；本 epic **不**新建 FR、**不**交付 language-server 二进制（仅 35.4 文档声明）。
- **继承 FR 与深度 FR 对照（NFR37）：** Epic 8 / 9 / 10 历史 `done` = 当时最小合同；**≠** FR83/84/85 深度关闭。不得用 FR33/36/39 历史 done 冒充本 epic 完成。
- **FR86 排除：** HLS 外挂政策由 Epic **26.2** + **AD-25** 覆盖；本 epic **不**实现 FR86。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。
- **NFR37：** 公开/内部文档不得仅以 Partial 现状或历史 epic `done` 关闭 FR33/36/39 深度；深度缺口以 **FR83/84/85** 验收或显式合同化 defer。

### (b) 粗工期带

- **预计：** Epic 35 整体约 **1.5–3 人周**（35.1 本风险记录 ≤0.25 人周；35.2 C ABI 超出 Counter 0.5–1 人周；35.3 SoftF16 可综合 **或** 显式 defer 0.25–1.5 人周（defer 偏短）；35.4 formal 真夹具 + LSP deferred 文档 0.5–1 人周）。
- **置信度 / 假设：** 中；假设 cdylib 可扩展为第二 DUT 或通用路径而不重写仿真内核；formal「非玩具」定义为至少一真实设计导出 + 文档化外部工具链调用（不必自研完整 model checker）。若要求完整 IEEE SoftF16 可综合算子库或商用 formal 集成，工期显著上修。每条 FR 选定 **实现** 或 **显式 defer** 后须写入决策/文档（NFR37），不得静默声称深度完成。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **35.2–35.4** 标 `ready` 或开工实现。
- **不得用 `check_sva_text` / toy check 关闭 FR85**（或宣称 FR39 深度完成）。
- **若 FR84 选显式 defer：不得声称 SoftF16 可综合浮点已交付**（亦不得用 host-only / `Bits<16>` 位向量路径冒充可综合浮点算子）。
- 不得把 Counter-only cdylib demo 冒充 FR83 / FR33 深度关闭。
- 不得把 Epic 8/9/10 历史 `done` 或 Partial 现状冒充本 epic 深度关闭（**NFR37：规划 done ≠ 深度 done**）。
- 不得把 **LSP** hover/goto 标为本 epic 完成条件，或交付伪 language-server 交差。
- 不得在未改 PRD / 本记录的前提下把 FR83/84/85 任一条静默砍掉或改回「仅文档 unsupported」永久交差（显式 defer 须书面合同化）。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR37** 深度诚实度共同责任人
- 备份 / 升级路径：FR83/84/85「实现 vs defer」选型争议升级至产品 / Phase 10 排程负责人；formal 工具链选型或 SoftF16 可综合范围争议升级至 AD-28 / 架构维护者。

---

### FR83 / FR84 / FR85 — 实现 vs 显式 defer（选项表）

> Story 35.1 **只记录选项与禁令**；选定落地在 35.2–35.4。每行须在关闭前勾选其一，并满足对应禁止事项。

| FR | 加深对象 | 现状（Partial） | 选项 A — 实现 | 选项 B — 显式 defer | 关闭故事 |
| --- | --- | --- | --- | --- | --- |
| **FR83** | FR33 C ABI | Counter-only cdylib | 文档化第二 DUT **或** 通用 generate/link 路径 + 非 Counter-only 夹具 | PRD/用户文档显式 deferred：声明 cdylib 仍 Counter-only，**不得**声称通用 C ABI 深度已交付 | 35.2 **已选 A**（`Adder` + `rhdl_sim_new_dut`） |
| **FR84** | FR36 SoftF16 | host-only 模型 | SoftF16 → HIR → emit 可综合夹具（至少一黄金数值/位宽） | PRD/用户文档显式 deferred：**不得声称 SoftF16 可综合**已交付 | 35.3 **已选 B**（PRD addendum + `docs/fr36-rhdl-float.md`） |
| **FR85** | FR39 Formal/SVA | `check_sva_text` toy | 至少一真实设计导出 SVA（或文档钉死的 formal 工具链调用）并执行**非玩具**检查 | PRD/用户文档显式 deferred：声明 formal 仍玩具级，**不得**用 toy check 关闭深度 | 35.4 |

**LSP（非本 epic）：** 继续 **deferred**；35.4 用户文档须写明 hover/goto **不是**本 epic 完成条件。无独立「实现 vs defer」选型行——本阶段固定 defer。

### 历史最小合同 vs Epic 35 深度（对照）

| 维度 | 历史（Epic 8/9/10） | Epic 35 |
| --- | --- | --- |
| C ABI | Counter demo + C harness 对齐（FR33） | FR83：超出 Counter-only **或** 显式 defer |
| SoftF16 | host RTE 黄金（FR36） | FR84：可综合路径 **或** 显式 defer（禁假可综合话术） |
| Formal | emit + toy `check_sva_text`（FR39） | FR85：真夹具/工具链 **或** 显式 defer（禁 toy 关单） |
| LSP | 已声明 deferred（FR38） | **仍 deferred**；仅文档收口 |

### 并行 / 维护叠加（Chipyard 式）

- 与 Epic 31–34 并行时：本 epic 优先级更低；不得挤占 CDC / Bundle / Mem / IP 深度回归面的维护带宽而不记风险。
- 与 Phase 9 闭包 epic 并行时：C ABI / SoftF16 / formal 夹具不得引入捕获闭包进入 `tick`（AD-18）；elaborate-time 非捕获 `Fn` 若出现须冻前消解。
- SoftF16 若选实现路径：须与 `docs/fr36-rhdl-float.md` / prelude 表面同步，禁止「代码可综合 / 文档仍 host-only」分裂话术（或反之）。

### 引用

- AD-28 — 风险门禁（NFR14）
- PRD NFR14 / FR33 / FR36 / FR39 / FR83 / FR84 / FR85；NFR37（规划 done ≠ 深度 done）
- FR86 / AD-25 — **不在**本 epic
- 现状 crate：`rhdl-cabi`、`rhdl-float`、`rhdl-formal`
- 体例：`nfr14-risk-epic34-ip-baseline.md`；`nfr14-risk-epic33-chisel-mem.md`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 35 关闭条件（Story 35.4 勾选）

- [x] **FR83：** 已落地选项 A（非 Counter-only 夹具）**或** 选项 B（显式 defer 合同段落）
- [x] **FR84：** 已落地选项 A（可综合夹具）**或** 选项 B（显式 defer；无「可综合 SoftF16 已交付」话术）— **已选 B**
- [ ] **FR85：** 已落地选项 A（非 toy formal 夹具）**或** 选项 B（显式 defer）；**未**用 `check_sva_text` 关单
- [ ] **LSP：** 用户文档声明仍 deferred / 非本 epic 完成条件
- [ ] **NFR37：** 相对 Epic 8/9/10 历史最小合同已文档化；不得用 Partial / 历史 `done` 冒充深度关闭
- [ ] **禁止事项未触发：** 无 toy 关 FR85；无 defer 却称 SoftF16 可综合；无提前标 35.2–35.4 ready（对本记录而言）

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 35 故事 35.2–35.4 标 `ready`。**  
**Epic 35 关闭条件（上节）待 Story 35.4 勾选。**
