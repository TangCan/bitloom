# NFR14 风险记录 — Epic 115 unpaired firtool 产品钉再升钉（FR182）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28** / **AD-9**；Phase 22 **NFR83–NFR87**；实现面 **FR182**。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic112-floating-circt-git-head-fr179.md`；`nfr14-risk-epic106-*`（FR173 配对升钉）。  
> **前置：** Epic 111 / FR178 **closed**；Epic 112 / **FR179** **closed**（软序 AD-9）；Epic 106 / **FR173** 配对升钉 **closed**（仍有效 · NFR83）；Epic 107 / **FR174** unpaired HEAD **1.156.0** **closed**（仍有效 · NFR83）。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **115.2–115.3** 标 `ready`。**不得**以 FR173 alone / FR174 alone / FR179 alone 勾选本 FR。**不得**未修订 AD-9 *unpaired product-pin* 例外就升产品钉。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR182 / Epic 115；NFR14、NFR83、NFR85、NFR86；对照 FR173 / FR174 / FR179 / AD-9 |
| 记录日期 | 2026-09-14 |
| 状态 | **open — Story 115.1**（in-progress）；Epic 115 未关闭 |
| **选定** | 在保留 FR173 配对面（1.158.0↔7.15.0）关闭证据的前提下，授权将 **AD-9 默认产品钉**再升至 **firtool-1.159.0**，且 **无**上游 Chisel 正式配对；须先修订 **AD-9** 明示 *unpaired product-pin* 例外（NFR85）；禁止把 FR174 optional HEAD / FR179 floating-track 冒充本产品钉升钉 |

### Phase 21/22 关闭面 vs Epic 115（NFR83）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR173 / Epic 106** | 配对升钉 → firtool-**1.158.0** ↔ Chisel **7.15.0** | **仍有效**；**alone ≠ FR182** |
| **FR174 / Epic 107** | optional unpaired HEAD 渠道 **1.156.0**（≠ 产品钉） | **仍有效**；**alone ≠ FR182** |
| **FR179 / Epic 112** | floating-track 渠道 **1.159.0**（≠ 默认产品钉） | **仍有效**；**alone ≠ FR182** |
| **Epic 115 / FR182** | unpaired **产品钉**再升钉 → **1.159.0** | **本实现 epic** |

### 本批钉死（115.2 必须）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **目标产品钉** | AD-9 默认产品 firtool 升至 **1.159.0**；文档诚实声明 **无** Chisel 正式配对；下载/sha256/`firtool ensure` 路径可用 | 仅改 FR174/FR179 可选渠道；假装仍有 Chisel 配对 |
| **验收谓词** | `just` / CI / ATDD：产品钉版本可读为 **1.159.0**；缺工具 / 错版本 / FORCE_MISSING → **非零可读**；不得 silent-Ok | `continue-on-error` |
| **AD 修订（NFR85）** | Story **115.2 ready 前**修订 **AD-9**：明示 *unpaired product-pin* 例外；默认钉 = **1.159.0**；≠ FR173 配对纪律；≠ FR174/FR179 可选渠道 alone | 无 AD-9 例外修订静默升钉 |
| **与 FR173 / FR174 / FR179 边界** | FR173 = 配对升钉；FR174 = optional HEAD **1.156.0**；FR179 = floating-track 可选渠道；FR182 = **默认产品钉** unpaired 再升 | FR173/174/179 alone 勾选 |

### 验收谓词 / 失败语义（115.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | 产品钉 **1.159.0** 验收通过；`docs/fr182-*`；AD-9 已修订（NFR85） |
| **负向** | 缺工具 / 错版本 / force-missing → 可读非零 |
| **禁止勾选** | FR173 alone；FR174 alone；FR179 alone；未修订 AD-9 例外；仅 docs；silent-Ok |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **115.1** | 本 NFR14 |
| **115.2** | 产品钉再升钉实现 + ATDD/CI；AD-9 例外先入脊柱（NFR85）；`docs/fr182-*` |
| **115.3** | README/deferred/spine 收口；勾选 Epic 115；未写入子集仍 **NFR86** |

### (a) 上游约束

- **Epic 111 / FR178 已关闭；Epic 112 / FR179 已关闭（软序）。**
- **FR173 / FR174 / FR179：** **仍有效**；alone ≠ FR182。
- **NFR85：** unpaired 产品钉升钉须先修订 AD-9 例外。
- **NFR83 / NFR86：** 不得改写已关闭；不得静默超子集（更高 unpaired 产品钉 / 恢复仅配对纪律仍 **NFR86**）。
- **品牌：** Bitloom；firtool 运行时不得进入设计 crate。

### (b) 粗工期带

- **预计：** Epic 115 **Medium–High**；本 NFR14 ≤0.25 人周；115.2 约 1–2 人周（产品钉面 / ensure / CI）。
- **置信度：** 中（须诚实无配对；下载资产须存在）。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **115.2–115.3** 标 ready。
- **不得**以 FR173 / FR174 / FR179 alone 勾选 FR182。
- **不得**未修订 AD-9 *unpaired product-pin* 例外就升产品钉。
- **不得**把 FR174/FR179 可选渠道冒充默认产品钉。
- **不得**静默扩大 FR142；**不得**把 `git push` 当 FR。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR83** / **NFR85** / **NFR86** / **AD-9**
- 升级：缩回「仅配对升钉」或「仅 optional HEAD」须 Correct Course / 产品批准。

### 引用

- AD-28；AD-9；FR182；对照 FR173 / FR174 / FR179；Phase 22 Correct Course approved 2026-09-12
- **NFR14-crates** ≠ 本门禁

---

### Epic 115 关闭条件（Story 115.3 勾选）

- [ ] **unpaired 产品钉再升钉 + 验收谓词：** Story 115.2
- [ ] **AD-9 *unpaired product-pin* 例外修订（NFR85）：** Story 115.2 ready 前落地
- [ ] **docs/fr182-* + README/deferred：** Story 115.3
- [ ] **NFR83：** FR173/FR174/FR179 关闭面未改写
- [ ] **禁止事项未触发：** 未用 FR173/174/179 alone 勾选；未 silent-Ok
- [ ] **品牌 / 依赖：** Bitloom / `bitloom-prelude`
- [ ] **超子集：** 未写入本 epic NFR14 的更深升钉仍 **NFR86**
