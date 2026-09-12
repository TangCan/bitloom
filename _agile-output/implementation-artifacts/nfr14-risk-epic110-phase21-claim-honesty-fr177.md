# NFR14 风险记录 — Epic 110 Phase 21 宣称诚实门（FR177）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**；Phase 21 **NFR78–NFR82**；宣称面 **FR177**；宣称须引 **FR172–177**。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic104-phase20-claim-honesty-fr171.md`。  
> **前置：** Epic 105 / FR172 **closed**；Epic 106–109 / FR173–176 **closed**（软序满足）；Phase 12–20 关闭面仍有效（**NFR78**）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 110 后续故事 **110.2–110.3** 标为 `ready`，亦不得开工文档宣称更新。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR177 / Epic 110；NFR14、NFR78、NFR79、NFR81、NFR82；对照 FR172–176 / FR166–171 / FR142 |
| 记录日期 | 2026-09-12 |
| 状态 | open — Story 110.1；Epic 110 进行中；110.2–110.3 待本记录后 ready |
| **选定** | 在保留 Phase 12–20 与 Epic 105–109 关闭面的前提下，授权 **Phase 21 宣称诚实门（FR177）**：公开 README / deferred / 状态页按**已关** FR172–177 可审计宣称；**禁止** Phase 20 alone 冒充 FR173–176；**禁止**未关 FR 勾选「已交付」；**禁止**静默扩大 FR142；**禁止**暗示超出 FR173–176 各 epic NFR14 钉死子集的加深已清或「NFR76 账本已空」；**禁止**把 `git push` 当 FR |

### Phase 12–20 / Epic 105–109 vs Epic 110 边界（NFR78 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **Phase 12–20** | FR94–171 | **仍有效；不得改写为失败**（**NFR78**） |
| **Epic 105 / FR172** | Phase 21 闸门 | **已关闭** |
| **Epic 106 / FR173** | firtool 升钉 AD-9 | **已关闭**；宣称须引 **FR173** |
| **Epic 107 / FR174** | unpaired HEAD | **已关闭**；宣称须引 **FR174** |
| **Epic 108 / FR175** | 更广 CIRCT SV | **已关闭**；宣称须引 **FR175** |
| **Epic 109 / FR176** | Parser/Chisel 生态 | **已关闭**；宣称须引 **FR176** |
| **Epic 110 / FR177** | **宣称诚实门**（本 epic） | **本实现 epic**（文档/状态页；非新加深） |

**选定：关闭「宣称无 FR 指针 / Phase 20 冒充 Phase 21」缺口；≠ 重做 FR173–176；≠ 清空 NFR81 未选加深 / 「NFR76 账本已空」。**

### FR177 范围（本 epic）

| 交付 | 故事 |
| --- | --- |
| 本 NFR14 + ATDD；门禁 110.2–110.3 | **110.1** |
| README「状态与 deferred」、`deferred-work.md`、`docs/fr177-*`、epics/sprint 指针：每条已关 FR172–177 有宣称指针；firtool→FR173；unpaired→FR174；CIRCT SV→FR175；Parser 生态→FR176；品牌 Bitloom | **110.2** |
| 勾选 Epic 110 / FR177；声明 Phase 21 规划故事齐（Epic 105–110）；宣称须引 FR172–177；NFR81 仍须新合同 | **110.3** |

### 验收谓词（110.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | 已关 FR172–176（及本门关闭后的 **FR177**）各有 README/deferred（或 `docs/fr*`）可审计指针；四条加深引对应 FR |
| **负向 / 禁止** | Phase 20 alone 冒充 FR173–176；未关 FR 写「已交付」；静默扩大 FR142；暗示超出 NFR14 子集已清 / 「NFR76 账本已空」；`git push` 当 FR |
| **诚实** | 超出本批钉死子集的加深仍 **NFR81** |

### (a) 上游约束

- **Epic 105 已关闭；Epic 106–109 已关闭**（软序建议满足）。
- **NFR78：** 不得改写 FR94–176「已关闭」。
- **NFR82：** 对外「firtool 升钉 / unpaired HEAD / 更广 CIRCT·sim / 更深 Parser 生态 / Phase 21 完成」类宣称须经本 FR177 诚实面（及对应已关 FR）。
- **NFR81：** 超出各 epic NFR14 钉死子集仍须新合同；**不得**宣称「NFR76 账本已空」。
- **不得静默扩大 FR142**（Phase 17 公开 API 表面）。
- **品牌：** **Bitloom**；设计 crate → **`bitloom-prelude`**（AD-6）。
- **软序：** 110.2 → 110.3。

### (b) 粗工期带

- **预计：** Epic 110 整体约 **0.5–1.5 人周**（110.1 ≤0.25；110.2 文档/指针 0.25–1；110.3 收口 ≤0.5）。
- **置信度 / 假设：** 高（文档宣称；加深已关）。假设不重开 FR173–176 实现。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **110.2–110.3** 标 `ready` 或开工。
- **不得用 Phase 20 alone 宣称 FR173–176 已交付**（须引对应 FR）。
- **不得在未关 FR 上勾选「已交付」。**
- **不得静默扩大 FR142。**
- **不得暗示超出 FR173–176 各 epic NFR14 钉死子集的加深已清**，或宣称「**NFR76 账本已空**」。
- **不得改写 Phase 12–20 / Epic 105–109 关闭证据为失败**（NFR78）。
- **不得把 `git push` 当成产品 FR。**
- 不得把本记录冒充 **NFR14-crates**（FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 与 **NFR78 / NFR81 / NFR82** 共同责任人
- 备份 / 升级路径：缩回「不做宣称门 / 仅口头」须升级至产品 / Correct Course 批准人。

### 引用

- AD-28；FR177；对照 FR172–176 / FR171；Phase 21 Correct Course approved 2026-09-12
- **NFR14-crates** ≠ 本门禁

---

### Epic 110 关闭条件（Story 110.3 勾选）

- [ ] **FR177 诚实面落地** — Story 110.2（README / deferred / 状态指针）
- [ ] **文档 / deferred / README / spine 收口** — Story 110.3
- [ ] **NFR78/81/82：** Phase 21 宣称须引 FR172–177；NFR81 未选仍须新合同；不得宣称 NFR76 账本已空
- [ ] **品牌 / AD-6：** Bitloom；设计 crate → `bitloom-prelude`
- [ ] **Phase 21 规划故事齐：** Epic 105–110；≠ 宣称 NFR76/NFR81 账本已空
