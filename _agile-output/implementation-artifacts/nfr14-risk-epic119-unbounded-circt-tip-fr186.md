# NFR14 风险记录 — Epic 119 无界 CIRCT tip（FR186）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28** / **AD-9**；Phase 23 **NFR88–NFR92**；实现面 **FR186**。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic112-floating-circt-git-head-fr179.md`；`nfr14-risk-epic118-phase23-nfr86-leftovers.md`。  
> **前置：** Epic 118 / FR185 **closed**；Epic 112 / **FR179** floating-track firtool-**1.159.0** **closed**（仍有效 · NFR88）；Epic 115 / FR182 产品钉 **1.159.0** **closed**。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **119.2–119.3** 标 `ready`。**不得**以 FR179 alone / FR174 alone / FR182 alone 勾选本 FR。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR186 / Epic 119；NFR14、NFR88、NFR90、NFR91；对照 FR179 / FR174 / FR182 / AD-9 |
| 记录日期 | 2026-09-14 |
| 状态 | **closed — Story 119.3**；Epic 119 / FR186 可宣称关闭；超子集仍 **NFR91** |
| **选定** | 在保留 FR179 文档钉死 **浮动轨 1.159.0** 关闭面的前提下，授权相对该钉死浮动轨之外的 **可复现无界 / live CIRCT tip** 产品通道；触及 **AD-9** 须按 **NFR90** 先修订；诚实「非产品默认钉」；禁止 PATH-random / silent-Ok |

### Phase 22 / FR179 关闭面 vs Epic 119（NFR88）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR179 / Epic 112** | 文档钉死浮动轨 **1.159.0**（channel ≠ 产品 cache） | **仍有效**；**alone ≠ FR186** |
| **FR182 / Epic 115** | unpaired **产品钉** **1.159.0** | **仍有效**；alone ≠ 无界 tip |
| **FR174 / Epic 107** | unpaired **1.156.0** | **仍有效**；alone ≠ FR186 |
| **Epic 119 / FR186** | 无界 / live CIRCT tip | **本实现 epic** |

### 本批钉死（119.2 必须）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **目标** | 文档钉死 **至少一条** 相对 FR179 浮动轨钉死之外的 **无界 / live tip**（或等价可复现 tip 通道）：可读 tip 身份 + 可复现获取 | 仅复述 FR179 `1.159.0` 浮动轨；把 FR182 产品钉冒充 tip |
| **验收谓词** | `just` / CI / ATDD：正向可复现 tip 身份；缺工具 / 版本不符 / force-missing → **非零可读**；不得 silent-Ok | `continue-on-error` |
| **AD 修订（NFR90）** | Story **119.2 ready 前**修订适用 **AD-9**（tip 渠道 vs 浮动轨 / 产品钉边界） | 无 AD 修订静默宣称 tip 产品路径 |
| **与 FR179 / FR182 边界** | FR179 = 文档钉死浮动轨；FR186 = **超该钉死**的无界 tip；FR182 = 产品钉 | FR179 alone / FR182 alone 勾选 |
| **诚实** | 明确「**非**产品默认钉 / **非** FR179 浮动轨 alone」 | 暗示 tip = 默认产品 firtool |

### 验收谓词 / 失败语义（119.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | tip 通道验收通过；`docs/fr186-*`；AD-9 已修订（NFR90） |
| **负向** | 缺工具 / 错身份 / force-missing → 可读非零 |
| **禁止勾选** | FR179 alone；FR174 alone；FR182 alone；PATH 随机；仅 docs |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **119.1** | 本 NFR14 |
| **119.2** | tip 实现 + ATDD/CI；AD-9 先入脊柱（NFR90）；`docs/fr186-*` |
| **119.3** | README/deferred/spine 收口；勾选 Epic 119；未写入子集仍 **NFR91** |

### (a) 上游约束

- **Epic 118 / FR185 已关闭。**
- **FR179 / FR182 / FR174：** **仍有效**；alone ≠ FR186。
- **NFR90：** tip 渠道须先修订 AD-9。
- **NFR88 / NFR91：** 不得改写已关闭；不得静默超子集。
- **软序：** 与 Epic 122（AD-9 / firtool 再升钉）建议串行。
- **品牌：** Bitloom；CIRCT/firtool 运行时不得进入设计 crate（`bitloom-prelude`）。

### (b) 粗工期带

- **预计：** Epic 119 **Medium–High**；本 NFR14 ≤0.25 人周；119.2 约 1–3 人周（tip 漂移 / 缓存 / CI）。
- **置信度：** 中–低（live tip 漂移；须 FORCE_MISSING 纪律）。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **119.2–119.3** 标 ready。
- **不得**以 FR179 / FR174 / FR182 alone 勾选 FR186。
- **不得** PATH 随机冒充 tip 产品路径；**不得** silent-Ok。
- **不得**无 AD-9 修订静默宣称（NFR90）。
- **不得**把 tip 冒充产品默认钉或 FR179 浮动轨 alone。
- **不得**静默扩大 FR142；**不得**把 `git push` 当 FR。
- 不得把本记录冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR88** / **NFR90** / **NFR91** / **AD-9**
- 升级：缩回「仅 FR179 浮动轨」须 Correct Course / 产品批准。

### 引用

- AD-28；AD-9；FR186；对照 FR179 / FR174 / FR182；Phase 23 Correct Course approved 2026-09-14
- `docs/fr179-floating-circt-git-head.md` — 无界 tip 曾归 NFR86 → 本 FR
- **NFR14-crates** ≠ 本门禁

---

### Epic 119 关闭条件（Story 119.3 勾选）

- [x] **无界 tip 通道 + 验收谓词：** Story 119.2
- [x] **AD-9 修订（NFR90）：** tip 渠道边界写入脊柱
- [x] **docs/README/deferred：** ≠ FR179 alone；诚实非产品默认钉
- [x] **NFR88 / NFR91：** 边界保持；超子集仍须新合同
- [x] **禁止事项未触发：** 未用 FR179 alone 勾选
- [x] **品牌 / 依赖：** Bitloom / `bitloom-prelude`
