# NFR14 风险记录 — Epic 99 Phase 20 合同闸门（NFR71 四条升格 / FR166）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 20 **NFR73–NFR77**；闸门 **FR166**；宣称须引 **FR166–171**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic87-phase19-nfr59-fr152a.md`。  
> **前置：** Phase 12–19 FR94–165 / Epic 40–98 **closed**；Correct Course `sprint-change-proposal-2026-09-12-phase20-nfr71-four-leftovers.md` **approved**（2026-09-12；`correctCoursePhase20Approved: 2026-09-12`；commit `689834f`）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 99 后续故事 **99.2–99.4** 标为 `ready`，亦不得开工实现。**Epic 99 未关闭 / FR166 未验收前，Epic 100–104 不得标 `ready`。**

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR166 / Epic 99；NFR14、NFR73、NFR74、NFR76、NFR77；对照 FR154–165 / NFR68–72；FR167–171 实现面 |
| 记录日期 | 2026-09-12 |
| 状态 | open / in-progress — Story 99.1；Epic 99 闸门进行中；99.2–99.4 须本记录后才可 ready；Epic 100–104 在 Epic 99 关闭前不得 ready |
| **选定** | 在保留 Phase 12–19 关闭面的前提下，授权 Phase 20「NFR71 四条升格」合同闸门（FR166）；实现属 Epic 100–104 |

### Phase 12–19 关闭面 vs Phase 20 边界（NFR73 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **Phase 12–18** | FR94–153 / Epic 40–86 关闭证据 | **仍有效；不得改写为失败** |
| **Phase 19 NFR59 + FR152(a)** | FR154–165 / Epic 87–98 | **仍有效；不得改写「已关闭」**（NFR73） |
| **Phase 20 NFR71 四条** | FR166–171 / Epic 99–104；NFR73–NFR77 | **本合同**（Correct Course 已批准 2026-09-12） |

**选定：在保留 Phase 12–19 关闭面的前提下，授权 NFR71 四条升格。**  
本记录禁止把本批叙述偷换成「Phase 19 失败后的补救」或「NFR59 alone ⇒ NFR71 leftover 已授权」。  
**Phase 20 口径** = FR166–171 对应关闭 + 诚实列出超出各 NFR14 钉死子集的仍须新合同项（**NFR76**）；**不等于**静默扩大 FR142；**`git push` 不是 FR**。

### 批准默认（Q1–Q5 · Correct Course）

| # | 决议 |
| --- | --- |
| **Q1** | NFR71 **四条全部**升格为 **FR167–FR170**（各 epic NFR14 钉死子集；禁止静默超子集 — NFR76） |
| **Q2** | **FR168** — SPI + I2C + AXI **三者皆**手写 FL ≡ tick（禁止「至少一项」交差） |
| **Q3** | **不得**改写 Phase 12–19「已关闭」（**NFR73**） |
| **Q4** | **不得**静默扩大 FR142；宣称须引已关 FR（**FR171** / **NFR77**） |
| **Q5** | **MSRV** 保持现行；firtool 升钉须 Chisel 正式配对后修订 **AD-9**（NFR12 / NFR75）；HEAD Parser 须修订 **AD-27**（±AD-9）；**FR166** 须 Correct Course + PRD 戳后方可开实现 epic |

### (a) 上游约束

- **Correct Course 批准（2026-09-12）：** `sprint-change-proposal-2026-09-12-phase20-nfr71-four-leftovers.md` 批准 Phase 20 = Epic 99–104 · FR166–FR171 / NFR73–NFR77；不回滚 Phase 12–19。
- **PRD addendum：** 「Phase 20」合同戳待 **Story 99.2** 验收；「四条已交付」类宣称仅可引用对应 FR 关闭证据；**禁止**用 Phase 19 alone 冒充。
- **合同落地：** README / deferred 诚实面 — **Story 99.3**；ARCHITECTURE-SPINE / AGENTS 指针 — **Story 99.4**。
- **FR167–171 范围摘要：**
  1. **FR167** — 完整 ChiselSim / 额外 IDE 商店多端；Epic 100
  2. **FR168** — SPI + I2C + AXI 手写 FL；Epic 101
  3. **FR169** — 更广 CIRCT/MLIR allocation / firtool 升钉；Epic 102
  4. **FR170** — Chisel HEAD Parser 回迁；Epic 103
  5. **FR171** — Phase 20 宣称诚实门；Epic 104
- **NFR73：** Phase 20 关闭不得改写 Phase 12–19 FR94–165「已关闭」。
- **NFR74：** Phase 20 各实现 epic 开工前独立 NFR14（本记录为 Epic 99 门）。
- **NFR75：** 触及 AD-9 / AD-25 / AD-27 / IDE 商店时须先修订脊柱/文档/CI 再 story ready。
- **NFR76：** 禁止静默扩大超出各 epic NFR14 钉死子集；未写入 FR167–170 的新加深须另开合同。
- **NFR77：** 宣称仅经 FR171；不得用 FR162/163/164/165 或 Phase 19 alone 冒充本批。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。
- **软序（非硬依赖）：** `99 →（100 ‖ 101 ‖ 102 ‖ 103）→ 104`；Epic 101（`ip/`）建议串行；Epic 102 升钉配对建议先于 Epic 103。

### (b) 粗工期带

- **预计：** Epic 99 整体约 **0.75–2 人周**（99.1 本风险记录 ≤0.25 人周；99.2 Correct Course/PRD 验收 0.25–0.5；99.3 README/deferred 0.25–0.75；99.4 AD 指针收口 0.25–0.5）。**Phase 20 全盘（Epic 100–104）为 High effort**（ChiselSim/多商店、三协议 FL、CIRCT/firtool、HEAD Parser），不计入本 epic 人周，但必须在计划中显式承认。
- **置信度 / 假设：** 中（对本 epic 文档/合同故事为中–高；对 FR167–170 外部工具/商店/Chisel 为中–低）。假设不回滚 FR94–165；假设 99.3–99.4 只改合同指针与诚实文档、不提前实现加深。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **99.2–99.4** 标 `ready` 或开工实现。
- **FR166 未验收 / Epic 99 未关闭前，不得将 Epic 100–104 标 `ready` 或开工实现。**
- **不得改写 Phase 12–19 FR94–165「已关闭」为失败**（NFR73）。
- **不得用 Phase 19 alone 冒充 NFR71 四条合同已授权。**
- **不得静默扩大 FR142** 表面承诺。
- **不得把未写入 FR167–170 的新加深冒充已交付**（NFR76）。
- **不得把 `git push` / 远程同步当成产品 FR。**
- 不得静默 publish `rhdl` / `rhdl-bits`（AD-2 / 继承）。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR73 / NFR74 / NFR76 / NFR77** 共同责任人
- 备份 / 升级路径：缩回「只做子集 / 不做四条全做」须升级至产品 / Correct Course 批准人；AD-9/AD-25/AD-27 争议升级至架构（AD-28）维护者。

---

### FR167–171 → Epic 对照

| FR | Epic | 相对 Phase 19 |
| --- | --- | --- |
| FR167 | 100 | 超 FR162 / FR134（ChiselSim + 多商店） |
| FR168 | 101 | 超 FR163 `UartRx`（SPI+I2C+AXI 三者） |
| FR169 | 102 | 超 FR164（allocation / firtool 升钉） |
| FR170 | 103 | 超 FR165 / FR138（HEAD Parser） |
| FR171 | 104 | 宣称诚实门 |

### Epic 99 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **99.1** | 本 NFR14 风险记录 + ATDD；钉死 NFR73 边界 / FR167–171 / Q1–Q5 / 禁止项 | **本故事** |
| **99.2** | Correct Course + PRD/addendum Phase 20 验收（合同戳） | Gate：须本记录后才可 ready |
| **99.3** | 同步 README / deferred / 路线图指针（FR166） | Gate：须本记录后才可 ready |
| **99.4** | AD 指针与 Epic 99 收口；勾选关闭条件 | Gate：须本记录后才可 ready；关闭后才可开 100–104 |

### 并行 / 维护叠加（Chipyard 式 · NFR74）

- Epic 100–103 **可并行规划**但硬依赖 Epic 99 关闭；软序 `99 →（100 ‖ 101 ‖ 102 ‖ 103）→ 104`。
- 文档面叠加：addendum、README、`deferred-work.md`、ARCHITECTURE-SPINE 须同一「Phase 19 vs Phase 20」叙事；禁止混用。
- 加深风险：Epic 100 商店凭证；Epic 101 `ip/`；Epic 102/103 AD-9/AD-27；须各自 NFR14。

### 引用

- AD-28 — 风险门禁（NFR14）；AD-2 — 对外 `bitloom-*`；AD-6 — `bitloom-prelude`
- PRD NFR14 / FR166；NFR73–NFR77；对照 FR154–165 / NFR68–72
- Correct Course：`sprint-change-proposal-2026-09-12-phase20-nfr71-four-leftovers.md`（approved 2026-09-12）
- Addendum：`prd-rhdl-2026-08-19/addendum.md` — Phase 20（→ 99.2）
- 体例：`nfr14-risk-epic87-phase19-nfr59-fr152a.md`
- **NFR14-crates** ≠ 本门禁
