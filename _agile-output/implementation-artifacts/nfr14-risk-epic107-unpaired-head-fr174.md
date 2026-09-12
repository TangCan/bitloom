# NFR14 风险记录 — Epic 107 unpaired CIRCT / Chisel HEAD 产品路径（FR174）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28** / **AD-9** / **AD-27**；Phase 21 **NFR78–NFR82**；实现面 **FR174**。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic106-firtool-bump-ad9-fr173.md`；`nfr14-risk-epic103-chisel-head-parser-fr170.md`。  
> **前置：** Epic 105 / FR172 **closed**；Epic 106 / FR173 **closed**（AD-9 现钉 **firtool-1.158.0 ↔ Chisel 7.15.0**）；Epic 103 / **FR170** update-mainline **closed**（@ 当时 1.155.0 关闭面仍有效 · NFR78）。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **107.2–107.3** 标 `ready`。**不得**以 FR170 alone / FR138 alone / FR165 alone / FR173 alone 勾选本 FR。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR174 / Epic 107；NFR14、NFR78、NFR79、NFR80、NFR81；对照 FR170 / FR138 / FR165 / AD-9 / AD-27 |
| 记录日期 | 2026-09-12 |
| 状态 | **closed — Story 107.3**；Epic 107 / FR174 可宣称；浮动 git HEAD 超 1.156.0 仍 **NFR81** |
| **选定** | 在保留 FR170 document-pinned update-mainline 关闭面的前提下，授权相对现行 AD-9 钉死对的 **unpaired CIRCT 与/或 Chisel HEAD（或文档钉死未配对主线二进制）产品路径**；须修订适用 AD（至少 **AD-9** 与/或 **AD-27**）；禁止 PATH-random 冒充 HEAD |

### Phase 12–20 / FR170 关闭面 vs Epic 107（NFR78）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR138 / Epic 77** | `BitloomFirrtlParser.parse` | **仍有效**；alone ≠ FR174 |
| **FR165 / Epic 97** | Style Guide / linter | **仍有效**；alone ≠ FR174 |
| **FR170 / Epic 103** | update-mainline @ 当时钉死对 | **仍有效**；**alone ≠ FR174** |
| **FR173 / Epic 106** | 配对升钉 → 1.158.0 / 7.15.0 | **仍有效**；alone ≠ unpaired HEAD |
| **Epic 107 / FR174** | unpaired HEAD 产品路径 | **本实现 epic** |

### 本批钉死（107.2 必须）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **目标二进制** | 文档钉死 **至少一条** unpaired 路径：(A) CIRCT/firtool **HEAD**（或文档钉死的未配对 firtool tag ≠ AD-9 产品钉）经 `RHDL_FIRTOOL_PATH` / 专用 env；**或** (B) Chisel **HEAD**/SNAPSHOT（或文档钉死未配对主线）经专用 just/CI 夹具 | PATH 默认信任；把 FR170 update-mainline 冒充 HEAD |
| **验收谓词** | `just` / CI / ATDD：正向可复现；缺工具 / 未设置 HEAD 路径 / force-missing → **非零可读**；不得 silent-Ok | `continue-on-error` |
| **AD 修订（NFR80）** | Story **107.2 ready 前**修订适用 AD：**AD-9**（若产品文档化 HEAD firtool 渠道）与/或 **AD-27**（若 Chisel HEAD Parser/生态路径）；写明 ≠ AD-9 默认产品钉 | 无 AD 修订静默宣称 HEAD 产品路径 |
| **与 FR170 边界** | FR170 = document-pinned update-mainline 方言/API；FR174 = **未配对 HEAD 二进制**工作流 | FR170 alone 勾选 |

### 验收谓词 / 失败语义（107.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | 文档化 HEAD（或未配对主线）路径验收通过；`docs/fr174-*`；适用 AD 已修订 |
| **负向** | 缺 HEAD 工具 / 错版本 / force-missing → 可读非零 |
| **禁止勾选** | FR170 alone；FR138 alone；FR165 alone；FR173 alone；PATH 随机；仅 docs |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **107.1** | 本 NFR14 |
| **107.2** | HEAD 路径实现 + ATDD/CI；适用 AD 先入脊柱（NFR80）；`docs/fr174-*` |
| **107.3** | README/deferred/spine 收口；勾选 Epic 107；未写入子集仍 **NFR81** |

### (a) 上游约束

- **Epic 105 / 106 已关闭。**
- **FR170 / FR138 / FR165：** **仍有效**；alone ≠ FR174。
- **NFR80：** HEAD 渠道须先修订 AD-9 与/或 AD-27。
- **NFR78 / NFR81：** 不得改写已关闭；不得静默超子集。
- **软序：** 建议在 Epic 106 配对升钉之后（已满足）。
- **品牌：** Bitloom；CIRCT/Chisel HEAD 运行时不得进入设计 crate。

### (b) 粗工期带

- **预计：** Epic 107 **Medium–High**；本 NFR14 ≤0.25 人周；107.2 约 1–3 人周（外部 HEAD 可用性）。
- **置信度：** 中（HEAD 漂移；须 FORCE_MISSING 纪律）。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **107.2–107.3** 标 ready。
- **不得**以 FR170 / FR138 / FR165 / FR173 alone 勾选 FR174。
- **不得** PATH 随机冒充 HEAD 产品路径。
- **不得**无适用 AD 修订静默宣称（NFR80）。
- **不得**静默扩大 FR142；**不得**把 `git push` 当 FR。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR78** / **NFR80** / **NFR81** / **AD-9** / **AD-27**
- 升级：缩回「仅 FR170 docs」须 Correct Course / 产品批准。

### 引用

- AD-28；AD-9；AD-27；FR174；对照 FR170 / FR138 / FR165 / FR173；Phase 21 Correct Course approved 2026-09-12
- **NFR14-crates** ≠ 本门禁

---

### Epic 107 关闭条件（Story 107.3 勾选）

- [x] **FR174 钉死子集实现 + 验收** — Story 107.2
- [x] **文档 / deferred / README / spine 收口** — Story 107.3
- [x] **NFR78/81：** 边界与诚实义务保持
- [x] **品牌 / AD-6：** Bitloom；运行时不得进入设计 crate
- [x] **其余 FR175–177：** 未关前不得宣称 Phase 21 全清
