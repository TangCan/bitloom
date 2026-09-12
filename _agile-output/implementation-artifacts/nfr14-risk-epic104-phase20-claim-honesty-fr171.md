# NFR14 风险记录 — Epic 104 Phase 20 宣称诚实门（FR171）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**；Phase 20 **NFR73–NFR77**；宣称面 **FR171**；宣称须引 **FR166–171**。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic98-phase19-claim-honesty-fr156.md` / `nfr14-risk-epic99-phase20-nfr71-four-leftovers.md`。  
> **前置：** Epic 99 / FR166 **closed**；Epic 100–103 / FR167–170 **closed**（软序满足）；Phase 12–19 关闭面仍有效（**NFR73**）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 104 后续故事 **104.2–104.3** 标为 `ready`，亦不得开工文档宣称更新。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR171 / Epic 104；NFR14、NFR73、NFR74、NFR76、NFR77；对照 FR166–170 / FR154–165 / FR142 |
| 记录日期 | 2026-09-12 |
| 状态 | open / in-progress — Story 104.1；Epic 104 进行中；104.2–104.3 须本记录后才可 ready |
| **选定** | 在保留 Phase 12–19 与 Epic 99–103 关闭面的前提下，授权 **Phase 20 宣称诚实门（FR171）**：公开 README / deferred / 状态页按**已关** FR166–171 可审计宣称；**禁止** Phase 19 alone 冒充 FR167–170；**禁止**未关 FR 勾选「已交付」；**禁止**静默扩大 FR142；**禁止**暗示超出 FR167–170 各 epic NFR14 钉死子集的加深已清或「NFR71 账本已空」；**禁止**把 `git push` 当 FR |

### Phase 12–19 / Epic 99–103 vs Epic 104 边界（NFR73 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **Phase 12–19** | FR94–165 | **仍有效；不得改写为失败**（**NFR73**） |
| **Epic 99 / FR166** | Phase 20 闸门 | **已关闭** |
| **Epic 100 / FR167** | ChiselSim / 多端 IDE 商店 | **已关闭**；宣称须引 **FR167** |
| **Epic 101 / FR168** | SPI·I2C·AXI 手写 FL | **已关闭**；宣称须引 **FR168** |
| **Epic 102 / FR169** | CIRCT multi-lower / allocation | **已关闭**；宣称须引 **FR169**；firtool 升钉仍 **NFR76** |
| **Epic 103 / FR170** | update-mainline Parser | **已关闭**；宣称须引 **FR170**；unpaired HEAD 仍 **NFR76** |
| **Epic 104 / FR171** | **宣称诚实门**（本 epic） | **本实现 epic**（文档/状态页；非新加深） |

**选定：关闭「宣称无 FR 指针 / Phase 19 冒充 Phase 20」缺口；≠ 重做 FR167–170；≠ 清空 NFR76 未选加深 / 「NFR71 账本已空」。**

### FR171 范围（本 epic）

| 交付 | 故事 |
| --- | --- |
| 本 NFR14 + ATDD；门禁 104.2–104.3 | **104.1** |
| README「状态与 deferred」、`deferred-work.md`、epics/sprint 指针：每条已关 FR166–171 有宣称指针；ChiselSim→FR167；SPI·I2C·AXI→FR168；CIRCT→FR169；HEAD Parser→FR170；品牌 Bitloom | **104.2** |
| 勾选 Epic 104 / FR171；声明 Phase 20 规划故事齐（Epic 99–104）；宣称须引 FR166–171；NFR76 仍须新合同 | **104.3** |

### 验收谓词（104.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | 已关 FR166–170（及本门关闭后的 **FR171**）各有 README/deferred（或 `docs/fr*`）可审计指针；四条加深引对应 FR |
| **负向 / 禁止** | Phase 19 alone 冒充 FR167–170；未关 FR 写「已交付」；静默扩大 FR142；暗示超出 NFR14 子集已清 / 「NFR71 账本已空」；`git push` 当 FR |
| **诚实** | 超出本批钉死子集的加深仍 **NFR76**（含 firtool 升钉超 AD-9、unpaired HEAD、更广生态） |

### (a) 上游约束

- **Epic 99 已关闭；Epic 100–103 已关闭**（软序建议满足）。
- **NFR73：** 不得改写 FR94–170「已关闭」。
- **NFR77：** 对外「ChiselSim/多商店 / SPI·I2C·AXI FL / CIRCT allocation / HEAD Parser / Phase 20 完成」类宣称须经本 FR171 诚实面（及对应已关 FR）。
- **NFR76：** 超出各 epic NFR14 钉死子集仍须新合同；**不得**宣称「NFR71 账本已空」。
- **不得静默扩大 FR142**（Phase 17 公开 API 表面）。
- **品牌：** **Bitloom**；设计 crate → **`bitloom-prelude`**（AD-6）。
- **软序：** 104.2 → 104.3。

### (b) 粗工期带

- **预计：** Epic 104 整体约 **0.5–1.5 人周**（104.1 ≤0.25；104.2 文档/指针 0.25–1；104.3 收口 ≤0.5）。
- **置信度 / 假设：** 高（文档宣称；加深已关）。假设不重开 FR167–170 实现。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **104.2–104.3** 标 `ready` 或开工。
- **不得用 Phase 19 alone 宣称 FR167–170 已交付**（须引对应 FR）。
- **不得在未关 FR 上勾选「已交付」。**
- **不得静默扩大 FR142。**
- **不得暗示超出 FR167–170 各 epic NFR14 钉死子集的加深已清**，或宣称「**NFR71 账本已空**」。
- **不得改写 Phase 12–19 / Epic 99–103 关闭证据为失败**（NFR73）。
- **不得把 `git push` 当成产品 FR。**
- 不得把本记录冒充 **NFR14-crates**（FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 与 **NFR73 / NFR74 / NFR76 / NFR77** 共同责任人
- 备份 / 升级路径：缩回「不做宣称门 / 仅口头」须升级至产品 / Correct Course 批准人。

### 引用

- AD-28；FR171；对照 FR166–170 / FR156；Phase 20 Correct Course approved 2026-09-12
- **NFR14-crates** ≠ 本门禁

---

### Epic 104 关闭条件（Story 104.3 勾选）

- [ ] **FR171 诚实面落地** — Story 104.2（README / deferred / 状态指针）
- [ ] **文档 / deferred / README / spine 收口** — Story 104.3
- [ ] **NFR73/76/77：** Phase 20 宣称须引 FR166–171；NFR76 未选仍须新合同；不得宣称 NFR71 账本已空
- [ ] **品牌 / AD-6：** Bitloom；设计 crate → `bitloom-prelude`
- [ ] **Phase 20 规划故事齐：** Epic 99–104；≠ 宣称 NFR71/NFR76 账本已空
