# NFR14 风险记录 — Epic 124 Phase 23 宣称诚实门（FR191）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**；Phase 23 **NFR88–NFR92**；宣称面 **FR191**；宣称须引 **FR185–191**。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic117-phase22-claim-honesty-fr184.md`。  
> **前置：** Epic 118 / FR185 **closed**；Epic 119–121 / FR186–188 **closed**；Epic 123 / FR190 **closed**；Epic 122 / FR189 **blocked-upstream**（诚实列出未关）；Phase 12–22 / 结项关闭面仍有效（**NFR88**）。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **124.2–124.3** 标 `ready`。**不得**用 Phase 22 / 结项 alone 冒充本批五条；**不得**暗示「NFR86 账本已空」；**不得**把未关 **FR189** 写成已交付。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR191 / Epic 124；NFR14、NFR88、NFR89、NFR91、NFR92；对照 FR185–190 / FR178–184 / 结项 |
| 记录日期 | 2026-09-14 |
| 状态 | **open / in-progress**（Story 124.1；Epic 124 未关闭；关闭勾选 → Story 124.3） |
| **选定** | 在保留 Phase 12–22 / 结项与已关 Phase 23 deepen 的前提下，授权 **Phase 23 宣称诚实门（FR191）**：公开 README / deferred / 状态页按**已关** FR185–191 可审计宣称，并**诚实列出未关 FR189**；**禁止** Phase 22 / 结项 alone 冒充 FR186–190；**禁止**未关 FR 勾选「已交付」；**禁止**暗示超出各 epic NFR14 钉死子集的加深已清或「NFR86 账本已空」；**禁止**把 `git push` 当 FR |

### 已关 / 未关 FR185–191 宣称矩阵（本 epic 钉死）

| FR | Epic | 加深面 | 本记录时状态 | 宣称须引 |
| --- | --- | --- | --- | --- |
| **FR185** | 118 | Phase 23 合同闸门 | **已关闭** | **FR185** |
| **FR186** | 119 | 无界 CIRCT tip | **已关闭** | **FR186**（≠ FR179 alone） |
| **FR187** | 120 | Handshake lower deepen | **已关闭** | **FR187**（≠ FR180 alone） |
| **FR188** | 121 | 社区 Style Guide 全家桶 | **已关闭** | **FR188**（≠ FR181 alone） |
| **FR189** | 122 | 继续 firtool 产品钉升钉 | **未关闭 / blocked-upstream**（无已发布 firtool >1.159.0） | **不得宣称已交付**；须引 blocked 诚实面 |
| **FR190** | 123 | 继续显式扩 FR142 | **已关闭** | **FR190**（≠ FR183 alone） |
| **FR191** | 124 | **宣称诚实门**（本 epic） | **未关闭**（本 epic） | **FR185–FR191** via **FR191**（含未关项诚实） |

**Story 124.2 时：** 落地 `docs/fr191-*`；状态页标本门与 **FR189 blocked**；已关 FR186–188/190 有指针。  
**Story 124.3 时：** 勾选 Epic 124 / FR191；可声明 Phase 23 **规划**故事齐（Epic 118–124）；**实现**诚实：FR189 仍未关；仍 **不得**宣称「NFR86 账本已空」。

### Phase 12–22 / 结项 / Epic 118–123 vs Epic 124 边界（NFR88 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **Phase 12–22 + 结项** | FR94–184 / engineering closeout | **仍有效；不得改写为失败**（**NFR88**） |
| **Epic 118–121 / 123** | 闸门 + 已关 deepen | **已关闭**；宣称须引对应 FR |
| **Epic 122 / FR189** | firtool 再升钉 | **未关 / blocked**；**alone ≠ 已交付** |
| **Epic 124 / FR191** | **宣称诚实门**（本 epic） | **本实现 epic**（文档/状态页；非新加深） |

**选定：关闭「宣称无 FR 指针 / Phase 22·结项冒充 Phase 23 / 把未关 FR189 写成已交付」缺口；≠ 重做 FR186–190；≠ 清空 NFR91 / 「NFR86 账本已空」。**

### FR191 范围（本 epic）

| 交付 | 故事 |
| --- | --- |
| 本 NFR14 + ATDD；门禁 124.2–124.3；宣称矩阵（含 FR189 未关） | **124.1** |
| README「状态与 deferred」、`deferred-work.md`、`docs/fr191-*`、epics/sprint 指针：已关 FR185–188/190 有宣称指针；FR189 = blocked-upstream；品牌 Bitloom | **124.2** |
| 勾选 Epic 124 / FR191；声明 Phase 23 **规划**故事齐（Epic 118–124）；宣称须引 FR185–191；诚实 FR189 未关；NFR91 仍须新合同 | **124.3** |

### 验收谓词（124.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | 已关 FR185–188/190（及本门关闭后的 **FR191**）各有 README/deferred（或 `docs/fr*`）可审计指针；**FR189** 显式 **blocked / 未关** |
| **负向 / 禁止** | Phase 22 / 结项 alone 冒充 FR186–190；未关 FR189 写「已交付」；暗示超出 NFR14 子集已清 / 「NFR86 账本已空」；`git push` 当 FR |
| **诚实** | 超出本批钉死子集的加深仍 **NFR91**；Epic 122 Correct Course 未决 ≠ 已交付 |

### (a) 上游约束

- **Epic 118 已关闭；Epic 119–121 / 123 已关闭；Epic 122 诚实未关。**
- **NFR88：** 不得改写 FR94–190「已关闭」；不得改写结项。
- **NFR92：** 对外 Phase 23 类宣称须经本 FR191 诚实面（及对应已关 FR）；未关项须列出。
- **NFR91：** 超出各 epic NFR14 钉死子集仍须新合同；**不得**宣称「NFR86 账本已空」。
- **品牌：** **Bitloom**；设计 crate → **`bitloom-prelude`**（AD-6）。
- **软序：** 124.2 → 124.3；Epic 119–123「已关或诚实列出未关」已满足。

### (b) 粗工期带

- **预计：** Epic 124 整体约 **0.5–1.5 人周**（124.1 ≤0.25；124.2 文档/指针 0.25–1；124.3 收口 ≤0.5）。
- **置信度 / 假设：** 高（文档宣称；加深大多已关；FR189 诚实 blocked）。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **124.2–124.3** 标 `ready` 或开工。
- **不得用 Phase 22 / 结项 alone 宣称 FR186–190 已交付**（须引对应 FR）。
- **不得在未关 FR189 上勾选「已交付」。**
- **不得暗示超出 FR186–190 各 epic NFR14 钉死子集的加深已清**，或宣称「**NFR86 账本已空**」。
- **不得改写 Phase 12–22 / 结项 / 已关 Epic 关闭证据为失败**（NFR88）。
- **不得把 `git push` 当成产品 FR。**
- 不得把本记录冒充 **NFR14-crates**（FCFS）完成定义。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR88** / **NFR89** / **NFR91** / **NFR92**
- 升级：缩回「不做宣称门 / 仅口头 / 假装 FR189 已关」须 Correct Course / 产品批准。

### 引用

- AD-28；FR191；对照 FR185–190 / FR184；Phase 23 Correct Course approved 2026-09-14
- `sprint-change-proposal-2026-09-14-fr189-upstream-block.md` — FR189 blocked
- **NFR14-crates** ≠ 本门禁

---

### Epic 124 关闭条件（Story 124.3 勾选）

- [ ] **FR191 诚实面落地** — Story 124.2（README / deferred / `docs/fr191-*`；含 FR189 blocked）
- [ ] **文档 / deferred / README / spine 收口** — Story 124.3
- [ ] **Phase 23 规划故事齐声明** — Epic 118–124（NFR89）；实现诚实列出 FR189 未关
- [ ] **NFR88：** Phase 12–22 / 结项 / 已关 deepen 未改写
- [ ] **禁止事项未触发：** 未用 Phase 22/结项 alone；未宣称 NFR86 账本已空；未把 FR189 写成已交付
- [ ] **品牌 / 依赖：** Bitloom / `bitloom-prelude`
- [ ] **超子集：** 超出本批 NFR14 仍 **NFR91**
