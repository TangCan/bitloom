# NFR14 风险记录 — Epic 112 浮动 CIRCT git HEAD（FR179）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28** / **AD-9**；Phase 22 **NFR83–NFR87**；实现面 **FR179**。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic107-unpaired-head-fr174.md`；`nfr14-risk-epic111-phase22-nfr81-leftovers.md`。  
> **前置：** Epic 111 / FR178 **closed**；Epic 107 / **FR174** document-pinned unpaired **firtool-1.156.0** **closed**（仍有效 · NFR83）；Epic 106 / FR173 配对升钉 **closed**。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **112.2–112.3** 标 `ready`。**不得**以 FR174 alone / FR173 alone / FR182 alone 勾选本 FR。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR179 / Epic 112；NFR14、NFR83、NFR85、NFR86；对照 FR174 / FR173 / FR182 / AD-9 |
| 记录日期 | 2026-09-14 |
| 状态 | **open — Story 112.1**（in-progress）；Epic 112 未关闭 |
| **选定** | 在保留 FR174 文档钉死 unpaired **1.156.0** 关闭面的前提下，授权相对该钉死子集之外的 **可复现浮动 CIRCT git HEAD**（或等价可复现 HEAD 通道）产品路径；触及 **AD-9** 须按 **NFR85** 先修订；禁止 PATH-random / silent-Ok |

### Phase 21 / FR174 关闭面 vs Epic 112（NFR83）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR173 / Epic 106** | 配对升钉 → 1.158.0 / 7.15.0 | **仍有效**；alone ≠ 浮动 HEAD |
| **FR174 / Epic 107** | 文档钉死 unpaired **1.156.0** | **仍有效**；**alone ≠ FR179** |
| **FR182 / Epic 115** | unpaired **产品钉**再升钉 | **并行软序**；≠ 浮动 HEAD 通道 alone |
| **Epic 112 / FR179** | 浮动 CIRCT git HEAD | **本实现 epic** |

### 本批钉死（112.2 必须）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **目标** | 文档钉死 **至少一条** 相对 FR174 **1.156.0** 之外的 **浮动 CIRCT git HEAD**（或等价可复现 HEAD 通道）：可读版本语义 + 可复现获取 | 仅复述 FR174 `1.156.0` 钉死；把 FR173 配对升钉冒充浮动 HEAD |
| **验收谓词** | `just` / CI / ATDD：正向可复现；缺工具 / 版本不符 / force-missing → **非零可读**；不得 silent-Ok | `continue-on-error` |
| **AD 修订（NFR85）** | Story **112.2 ready 前**修订适用 **AD-9**（浮动 HEAD 渠道 vs 产品钉 / FR174 钉死子集边界） | 无 AD 修订静默宣称浮动 HEAD 产品路径 |
| **与 FR174 / FR182 边界** | FR174 = 文档钉死 unpaired **1.156.0**；FR179 = **超该钉死**的浮动 git HEAD；FR182 = unpaired **产品钉**再升钉 | FR174 alone / FR182 alone 勾选 |

### 验收谓词 / 失败语义（112.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | 浮动 HEAD 通道验收通过；`docs/fr179-*`；AD-9 已修订（NFR85） |
| **负向** | 缺工具 / 错版本 / force-missing → 可读非零 |
| **禁止勾选** | FR174 alone；FR173 alone；FR182 alone；PATH 随机；仅 docs |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **112.1** | 本 NFR14 |
| **112.2** | 浮动 HEAD 实现 + ATDD/CI；AD-9 先入脊柱（NFR85）；`docs/fr179-*` |
| **112.3** | README/deferred/spine 收口；勾选 Epic 112；未写入子集仍 **NFR86** |

### (a) 上游约束

- **Epic 111 / FR178 已关闭。**
- **FR174 / FR173：** **仍有效**；alone ≠ FR179。
- **NFR85：** 浮动 HEAD 渠道须先修订 AD-9。
- **NFR83 / NFR86：** 不得改写已关闭；不得静默超子集。
- **软序：** 与 Epic 115（AD-9 / unpaired 产品钉）建议串行。
- **品牌：** Bitloom；CIRCT/firtool 运行时不得进入设计 crate。

### (b) 粗工期带

- **预计：** Epic 112 **Medium–High**；本 NFR14 ≤0.25 人周；112.2 约 1–3 人周（HEAD 漂移 / 缓存）。
- **置信度：** 中（HEAD 漂移；须 FORCE_MISSING 纪律）。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **112.2–112.3** 标 ready。
- **不得**以 FR174 / FR173 / FR182 alone 勾选 FR179。
- **不得** PATH 随机冒充浮动 HEAD 产品路径；**不得** silent-Ok。
- **不得**无 AD-9 修订静默宣称（NFR85）。
- **不得**静默扩大 FR142；**不得**把 `git push` 当 FR。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR83** / **NFR85** / **NFR86** / **AD-9**
- 升级：缩回「仅 FR174 1.156.0」须 Correct Course / 产品批准。

### 引用

- AD-28；AD-9；FR179；对照 FR174 / FR173 / FR182；Phase 22 Correct Course approved 2026-09-12
- **NFR14-crates** ≠ 本门禁

---

### Epic 112 关闭条件（Story 112.3 勾选）

- [ ] **浮动 HEAD 通道 + 验收谓词：** Story 112.2
- [ ] **AD-9 修订（NFR85）：** Story 112.2 ready 前落地
- [ ] **docs/fr179-* + README/deferred：** Story 112.3
- [ ] **NFR83：** FR174/FR173 关闭面未改写
- [ ] **禁止事项未触发：** 未用 FR174 alone 勾选；未 silent-Ok
- [ ] **品牌 / 依赖：** Bitloom / `bitloom-prelude`
- [ ] **超子集：** 未写入本 epic NFR14 的更深 HEAD/方言仍 **NFR86**
