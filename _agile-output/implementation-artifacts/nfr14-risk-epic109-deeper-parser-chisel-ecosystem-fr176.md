# NFR14 风险记录 — Epic 109 更深 Parser / Chisel 生态（FR176）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28** / **AD-27** / **AD-9**；Phase 21 **NFR78–NFR82**；实现面 **FR176**。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic97-deeper-chisel-parser-ecosystem-fr165.md`；`nfr14-risk-epic103-chisel-head-parser-fr170.md`。  
> **前置：** Epic 105 / FR172 **closed**；Epic 106–108 **closed**（软序）；Epic 103 / **FR170**、Epic 97 / **FR165**、Epic 77 / **FR138**、Epic 70 / **FR130** **closed**。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **109.2–109.3** 标 `ready`。**不得**以 FR170 alone / FR165 alone / FR138 alone / FR130 alone 勾选本 FR。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR176 / Epic 109；NFR14、NFR78、NFR79、NFR80、NFR81；对照 FR170 / FR165 / FR138 / FR130 / AD-27 |
| 记录日期 | 2026-09-12 |
| 状态 | closed — Story 109.3；Epic 109 / FR176 可宣称关闭；未写入子集仍 **NFR81** |
| **选定** | 在保留 FR170 update-mainline / FR165 linter 关闭面的前提下，授权 **组合生态加深包**：FR165 style-lint **且** FR170 update-mainline parse（现行 AD-9 pin）**且** 新增 FR176 ecosystem 标记层（`emit`/`check` + `just` / CI）；触及则修订 **AD-27** |

### Phase 12–20 关闭面 vs Epic 109（NFR78）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR130 / Epic 70** | Style Guide S1–S4 | **仍有效**；alone ≠ FR176 |
| **FR138 / Epic 77** | `BitloomFirrtlParser.parse` | **仍有效**；alone ≠ FR176 |
| **FR165 / Epic 97** | Style Guide / linter L1–L5 | **仍有效**；alone ≠ FR176 |
| **FR170 / Epic 103** | update-mainline Parser | **仍有效**；**alone ≠ FR176** |
| **Epic 109 / FR176** | 组合生态加深 | **本实现 epic** |

### 本批钉死（109.2 必须）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **组合谓词** | (1) FR165 style-lint gate 通过；(2) FR170 `parser-head-migration-check` @ AD-9 **1.158.0 / 7.15.0** 通过；(3) 新增 FR176 ecosystem 标记（至少：`FR176` 头 + `chisel-ecosystem-pack` + per-module `--- FR176 ecosystem ---` + `emit_chisel_ecosystem_fr176` / `check_chisel_ecosystem_fr176`） | 仅再跑 FR165 或仅再跑 FR170 |
| **产品门禁** | `just chisel-ecosystem-deepen-check` + CI；`BITLOOM_ECOSYSTEM_FORCE_MISSING` → 非零 | silent-Ok；docs-only |
| **AD（NFR80）** | Story **109.2 ready 前**修订 **AD-27** 注明 FR176 组合生态包；默认不改 AD-9 版本 | 无 AD-27 修订静默宣称 |
| **与边界** | FR170 = Parser 方言回迁 alone；FR165 = linter alone；FR176 = **二者串联 + 新标记层** | FR170/165/138/130 alone 勾选 |

### 验收谓词 / 失败语义（109.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | `docs/fr176-*`；组合 just/CI；ATDD 绿 |
| **负向** | force-missing / 缺标记 / 子门禁失败 → 可读非零 |
| **禁止勾选** | FR170 alone；FR165 alone；FR138 alone；FR130 alone；仅 docs |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **109.1** | 本 NFR14 |
| **109.2** | 组合生态实现 + AD-27 修订 + ATDD/CI；`docs/fr176-*` |
| **109.3** | README/deferred 收口；勾选 Epic 109；未写入子集仍 **NFR81** |

### (a) 上游约束

- **Epic 105 已关闭；Epic 106–108 已关闭（软序）。**
- **FR170 / FR165 / FR138 / FR130：** **仍有效**；alone ≠ FR176。
- **NFR80：** 触及 AD-27 须先修订。
- **Scala/Parser 运行时不得进入设计 crate。**
- **品牌：** Bitloom。

### (b) 粗工期带

- **预计：** Epic 109 **Medium**；本 NFR14 ≤0.25 人周；109.2 约 0.5–1.5 人周。
- **置信度：** 中–高（复用既有 FR165/FR170 门禁）。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **109.2–109.3** 标 ready。
- **不得**以 FR170 / FR165 / FR138 / FR130 alone 勾选 FR176。
- **不得**无 AD-27 修订静默宣称（若实现触及）。
- **不得**静默扩大 FR142；**不得**把 `git push` 当 FR。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR78** / **NFR80** / **NFR81** / **AD-27**
- 升级：缩回「仅再跑 FR165 或 FR170」须 Correct Course / 产品批准。

### 引用

- AD-28；AD-27；FR176；对照 FR170 / FR165 / FR138 / FR130；Phase 21 Correct Course approved 2026-09-12
- **NFR14-crates** ≠ 本门禁

---

### Epic 109 关闭条件（Story 109.3 勾选）

- [x] **FR176 钉死子集实现 + 验收** — Story 109.2
- [x] **文档 / deferred / README / spine 收口** — Story 109.3
- [x] **NFR78/81：** 边界与诚实义务保持
- [x] **品牌 / AD-6：** Bitloom；运行时不得进入设计 crate
- [x] **其余 FR177：** 未关前不得宣称 Phase 21 全清
