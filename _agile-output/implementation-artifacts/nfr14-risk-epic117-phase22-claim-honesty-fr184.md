# NFR14 风险记录 — Epic 117 Phase 22 宣称诚实门（FR184）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**；Phase 22 **NFR83–NFR87**；宣称面 **FR184**；宣称须引 **FR178–184**。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic110-phase21-claim-honesty-fr177.md`。  
> **前置：** Epic 111 / FR178 **closed**；Epic 112–116 / FR179–183 **closed**（软序建议满足）；Phase 12–21 关闭面仍有效（**NFR83**）。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **117.2–117.3** 标 `ready`。**不得**用 Phase 21 alone 冒充本批五条；**不得**暗示「NFR81 账本已空」。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR184 / Epic 117；NFR14、NFR83、NFR84、NFR86、NFR87；对照 FR178–183 / FR172–177 / FR142 |
| 记录日期 | 2026-09-14 |
| 状态 | **closed — Story 117.3**；Epic 117 关闭；FR184 宣称门可宣称；Phase 22 规划+实现故事齐（Epic 111–117）；**NFR86** 未选加深仍须新合同；**不得**宣称 NFR81 账本已空 |
| **选定** | 在保留 Phase 12–21 与 Epic 111–116 关闭面的前提下，授权 **Phase 22 宣称诚实门（FR184）**：公开 README / deferred / 状态页按**已关** FR178–184 可审计宣称；**禁止** Phase 21 alone 冒充 FR179–183；**禁止**未关 FR 勾选「已交付」；**禁止**暗示超出各 epic NFR14 钉死子集的加深已清或「NFR81 账本已空」；**禁止**把 `git push` 当 FR |

### 已关 / 未关 FR178–184 宣称矩阵（本 epic 钉死）

| FR | Epic | 加深面 | 本记录时状态 | 宣称须引 |
| --- | --- | --- | --- | --- |
| **FR178** | 111 | Phase 22 合同闸门 | **已关闭** | **FR178** |
| **FR179** | 112 | 浮动 CIRCT git HEAD | **已关闭** | **FR179**（≠ FR174 alone） |
| **FR180** | 113 | Handshake dialect 加深 | **已关闭** | **FR180**（≠ FR129 alone） |
| **FR181** | 114 | Style/linter 加深 | **已关闭** | **FR181**（≠ FR176 alone） |
| **FR182** | 115 | unpaired firtool 产品钉 | **已关闭** | **FR182**（≠ FR173/174/179 alone） |
| **FR183** | 116 | 显式扩大 FR142 表面 | **已关闭** | **FR183**（≠ FR142 alone / 静默扩大） |
| **FR184** | 117 | **宣称诚实门**（本 epic） | **已关闭** | **FR178–FR184** via **FR184** |

**Story 117.2 时：** 若 FR184 尚未关闭，状态页仍标本门「未关闭 / 实现中」；关闭后须有 `docs/fr184-*` 指针。  
**Story 117.3 时：** 勾选 Epic 117 / FR184；声明 Phase 22 规划+实现故事齐（Epic 111–117）；仍 **不得**宣称「NFR81 账本已空」。

### Phase 12–21 / Epic 111–116 vs Epic 117 边界（NFR83 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **Phase 12–21** | FR94–177 | **仍有效；不得改写为失败**（**NFR83**） |
| **Epic 111–116 / FR178–183** | Phase 22 加深五条 + 闸门 | **已关闭**；宣称须引对应 FR |
| **Epic 117 / FR184** | **宣称诚实门**（本 epic） | **本实现 epic**（文档/状态页；非新加深） |

**选定：关闭「宣称无 FR 指针 / Phase 21 冒充 Phase 22」缺口；≠ 重做 FR179–183；≠ 清空 NFR86 未选加深 / 「NFR81 账本已空」。**

### FR184 范围（本 epic）

| 交付 | 故事 |
| --- | --- |
| 本 NFR14 + ATDD；门禁 117.2–117.3；宣称矩阵 | **117.1** |
| README「状态与 deferred」、`deferred-work.md`、`docs/fr184-*`、epics/sprint 指针：每条已关 FR178–184 有宣称指针；浮动 HEAD→FR179；Handshake→FR180；Style→FR181；unpaired 钉→FR182；FR142 扩展→FR183；品牌 Bitloom | **117.2** |
| 勾选 Epic 117 / FR184；声明 Phase 22 规划故事齐（Epic 111–117）；宣称须引 FR178–184；NFR86 仍须新合同 | **117.3** |

### 验收谓词（117.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | 已关 FR178–183（及本门关闭后的 **FR184**）各有 README/deferred（或 `docs/fr*`）可审计指针；五条加深引对应 FR |
| **负向 / 禁止** | Phase 21 alone 冒充 FR179–183；未关 FR 写「已交付」；暗示超出 NFR14 子集已清 / 「NFR81 账本已空」；`git push` 当 FR |
| **诚实** | 超出本批钉死子集的加深仍 **NFR86** |

### (a) 上游约束

- **Epic 111 已关闭；Epic 112–116 已关闭**（软序建议满足）。
- **NFR83：** 不得改写 FR94–183「已关闭」。
- **NFR87：** 对外「浮动 HEAD / Handshake / Style / unpaired 钉 / FR142 扩展 / Phase 22 完成」类宣称须经本 FR184 诚实面（及对应已关 FR）。
- **NFR86：** 超出各 epic NFR14 钉死子集仍须新合同；**不得**宣称「NFR81 账本已空」。
- **品牌：** **Bitloom**；设计 crate → **`bitloom-prelude`**（AD-6）。
- **软序：** 117.2 → 117.3。

### (b) 粗工期带

- **预计：** Epic 117 整体约 **0.5–1.5 人周**（117.1 ≤0.25；117.2 文档/指针 0.25–1；117.3 收口 ≤0.5）。
- **置信度 / 假设：** 高（文档宣称；加深已关）。假设不重开 FR179–183 实现。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **117.2–117.3** 标 `ready` 或开工。
- **不得用 Phase 21 alone 宣称 FR179–183 已交付**（须引对应 FR）。
- **不得在未关 FR 上勾选「已交付」。**
- **不得暗示超出 FR179–183 各 epic NFR14 钉死子集的加深已清**，或宣称「**NFR81 账本已空**」。
- **不得改写 Phase 12–21 / Epic 111–116 关闭证据为失败**（NFR83）。
- **不得把 `git push` 当成产品 FR。**
- 不得把本记录冒充 **NFR14-crates**（FCFS）完成定义。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR83** / **NFR86** / **NFR87**
- 升级：缩回「不做宣称门 / 仅口头」须 Correct Course / 产品批准。

### 引用

- AD-28；FR184；对照 FR178–183 / FR177；Phase 22 Correct Course approved 2026-09-12
- **NFR14-crates** ≠ 本门禁

---

### Epic 117 关闭条件（Story 117.3 勾选）

- [x] **FR184 诚实面落地** — Story 117.2（README / deferred / `docs/fr184-*`）
- [x] **文档 / deferred / README / spine 收口** — Story 117.3
- [x] **NFR83/86/87：** Phase 22 宣称须引 FR178–184；NFR86 未选仍须新合同；不得宣称 NFR81 账本已空
- [x] **品牌 / AD-6：** Bitloom；设计 crate → `bitloom-prelude`
- [x] **Phase 22 规划故事齐：** Epic 111–117；≠ 宣称 NFR81/NFR86 账本已空

**Status:** **closed — Story 117.3**（可宣称 Epic 117 / FR184 关闭；Phase 22 规划+实现故事已齐）。
