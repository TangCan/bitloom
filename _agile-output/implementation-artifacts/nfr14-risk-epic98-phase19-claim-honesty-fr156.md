# NFR14 风险记录 — Epic 98 Phase 19 宣称诚实门（FR156）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**；Phase 19 **NFR68–NFR72**；宣称面 **FR156**；宣称须引 **FR154–165**。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic87-phase19-nfr59-fr152a.md` / `nfr14-risk-epic86-fr153-semver-honesty.md`。  
> **前置：** Epic 87 / FR154 **closed**；Epic 88–97 / FR155 / FR157–165 **closed**（软序满足）；Phase 12–18 关闭面仍有效（**NFR68**）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 98 后续故事 **98.2–98.3** 标为 `ready`，亦不得开工文档宣称更新。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR156 / Epic 98；NFR14、NFR68、NFR69、NFR71、NFR72；对照 FR154–165 / FR148–153 / FR142 |
| 记录日期 | 2026-09-12 |
| 状态 | closed — Story 98.3 勾选完成；Epic 98 关闭；FR156 宣称门可宣称；Phase 19 规划故事齐（Epic 87–98）；**NFR71** 未选加深仍须新合同；**不得**宣称 NFR71 账本已空 |
| **选定** | 在保留 Phase 12–18 与 Epic 87–97 关闭面的前提下，授权 **Phase 19 宣称诚实门（FR156）**：公开 README / deferred / 状态页按**已关** FR154–165 可审计宣称；**禁止** Phase 18 alone 冒充 lsp/NFR59；**禁止**未关 FR 勾选「已交付」；**禁止**静默扩大 FR142；**禁止**暗示超出各 epic NFR14 钉死子集的加深已清（**NFR71**） |

### Phase 12–18 / Epic 87–97 vs Epic 98 边界（NFR68 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **Phase 12–18** | FR94–153 | **仍有效；不得改写为失败** |
| **Epic 87 / FR154** | Phase 19 闸门 | **已关闭** |
| **Epic 88 / FR155** | `bitloom-lsp` live crates.io | **已关闭**；宣称须引 **FR155**（≠ Phase 18 / FR152(b) alone） |
| **Epic 89–97 / FR157–165** | NFR59 九条升格钉死子集 | **已关闭**；各条宣称须引对应 FR |
| **Epic 98 / FR156** | **宣称诚实门**（本 epic） | **本实现 epic**（文档/状态页；非新加深） |

**选定：关闭「宣称无 FR 指针 / Phase 18 冒充 Phase 19」缺口；≠ 重做 FR155–165；≠ 清空 NFR71 未选加深。**

### FR156 范围（本 epic）

| 交付 | 故事 |
| --- | --- |
| 本 NFR14 + ATDD；门禁 98.2–98.3 | **98.1** |
| README「状态与 deferred」、`deferred-work.md`、epics/sprint 指针：每条已关 FR154–165 有宣称指针；`bitloom-lsp`→FR155；NFR59 各条→FR157–165；品牌 Bitloom | **98.2** |
| 勾选 Epic 98 / FR156；声明 Phase 19 规划故事齐（Epic 87–98）；宣称须引 FR154–165；NFR71 仍须新合同 | **98.3** |

### 验收谓词（98.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | 已关 FR154–165（含 **FR154** / **FR155** / **FR157** … **FR165**）各有 README/deferred（或 `docs/fr*`）可审计指针；lsp 上架引 **FR155**；NFR59 条引 **FR157–165** |
| **负向 / 禁止** | Phase 18 alone 冒充 lsp/NFR59；未关 FR 写「已交付」；静默扩大 FR142；暗示超出 NFR14 子集已清；`git push` 当 FR |
| **诚实** | 超出本批钉死子集的加深仍 **NFR71**；不得宣称「NFR59 永久账本全清且无新合同」若仍有 NFR71 项 |

### (a) 上游约束

- **Epic 87 已关闭；Epic 88–97 已关闭**（软序建议满足）。
- **NFR68：** 不得改写 FR94–165「已关闭」。
- **NFR72：** 对外「lsp 已上架 / NFR59 某子集已交付 / Phase 19 完成」类宣称须经本 FR156 诚实面（及对应已关 FR）。
- **NFR71：** Chisel HEAD Parser、更广 MLIR、SPI/I2C/AXI FL、ChiselSim 多商店等未选子集仍须新合同。
- **不得静默扩大 FR142**（Phase 17 公开 API 表面）。
- **品牌：** **Bitloom**；设计 crate → **`bitloom-prelude`**（AD-6）。
- **软序：** 98.2 → 98.3。

### (b) 粗工期带

- **预计：** Epic 98 整体约 **0.5–1.5 人周**（98.1 ≤0.25；98.2 文档/指针 0.25–1；98.3 收口 ≤0.5）。
- **置信度 / 假设：** 高（文档宣称；加深已关）。假设不重开 FR155–165 实现。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **98.2–98.3** 标 `ready` 或开工。
- **不得用 Phase 18 alone 宣称 lsp 已上架 / NFR59 已交付**（须 FR155 / FR157–165）。
- **不得在未关 FR 上勾选「已交付」。**
- **不得静默扩大 FR142。**
- **不得暗示超出 FR157–165 各 epic NFR14 钉死子集的加深已清**（NFR71）。
- **不得改写 Phase 12–18 / Epic 87–97 关闭证据为失败**（NFR68）。
- **不得把 `git push` 当成产品 FR。**
- 不得把本记录冒充 **NFR14-crates**（FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 与 **NFR68 / NFR69 / NFR71 / NFR72** 共同责任人
- 备份 / 升级路径：缩回「不做宣称门 / 仅口头」须升级至产品 / Correct Course 批准人。

---

### Epic 98 关闭条件（Story 98.3 勾选）

- [x] **FR156 诚实面落地** — Story 98.2（README / deferred / 状态指针）
- [x] **文档 / deferred / README / spine 收口** — Story 98.3
- [x] **NFR68/71/72：** Phase 19 宣称须引 FR154–165；NFR71 未选仍须新合同
- [x] **品牌 / AD-6：** Bitloom；设计 crate → `bitloom-prelude`
- [x] **Phase 19 规划故事齐：** Epic 87–98；≠ 宣称 NFR71 账本已空
