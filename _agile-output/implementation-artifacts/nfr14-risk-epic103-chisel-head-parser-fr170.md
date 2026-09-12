# NFR14 风险记录 — Epic 103 Chisel HEAD Parser 回迁（FR170）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28** / **AD-27**（±**AD-9**）；Phase 20 **NFR73–NFR77**；实现面 **FR170**；宣称须引对应 FR。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic97-deeper-chisel-parser-ecosystem-fr165.md` / Epic 77 Parser 记录。  
> **前置：** Epic 99 / FR166 **closed**；Epic 77 / **FR138** `BitloomFirrtlParser.parse` **closed**；Epic 97 / **FR165** Style Guide/linter **closed**；Epic 70 / **FR130** **closed**。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 103 后续故事 **103.2–103.3** 标为 `ready`，亦不得开工实现。**不得**以 FR138 alone / FR165 alone / FR130 alone 勾选本 FR。须 **修订 AD-27**（若升钉工具链则联动 **AD-9**；NFR75）。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR170 / Epic 103；NFR14、NFR73、NFR74、NFR75、NFR76、NFR77；对照 FR138 / FR165 / FR130 / AD-27 / AD-9 |
| 记录日期 | 2026-09-12 |
| 状态 | closed — Story 103.3 勾选完成；Epic 103 关闭；FR170 实现面可宣称（update-mainline `parseUpdateMainline` / FIRRTL 6.0.0；AD-27 已修订；AD-9 未改）；unpaired HEAD/firtool 升钉仍 **NFR76**；**FR171** 仍属 Epic 104 |
| **选定** | 在保留 FR138 Parser / FR165 linter / FR130 Style Guide 关闭面的前提下，授权 **相对现行钉死对（Chisel 7.14.0 / firtool-1.155.0 / `BitloomFirrtlParser.parse`）的 Chisel HEAD（或文档钉死更新主线）Parser 产品路径回迁**；**必须 Correct Course 痕迹 + 修订 AD-27**；若依赖新 firtool/Chisel 钉死对则联动 AD-9（建议先完成 Epic 102 配对） |

### Phase 12–19 / FR138 关闭面 vs Epic 103 实现边界（NFR73 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR130 / Epic 70** | Style Guide S1–S4 | **仍有效**；alone ≠ FR170 |
| **FR138 / Epic 77** | `BitloomFirrtlParser.parse` + AD-27 | **仍有效；不得改写为失败**；**alone ≠ FR170** |
| **FR165 / Epic 97** | Style Guide / linter 加深 | **仍有效**；alone ≠ FR170 |
| **Epic 99 / FR166** | Phase 20 闸门 | **已关闭** |
| **Epic 102 / FR169** | CIRCT/firtool 升钉配对 | **软序建议先于本 epic（若本批升钉）** |
| **Epic 103 / FR170** | Chisel HEAD Parser 回迁 | **本实现 epic** |
| **Epic 100–101 / 104** | 其它 Phase 20 | **本 epic 不做** |

**选定：关闭 FR165「HEAD Parser deferred」缺口；≠ 重做 FR138；≠ 仅 linter 加深冒充 Parser 回迁。**

### 本批钉死（103.2 必须）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **钉死对** | 文档钉死目标 Chisel / firtool（HEAD 或更新主线）；相对现行 7.14.0 / 1.155.0 的回迁差 | 未文档钉死的 PATH 随机版本 |
| **Parser API** | 产品路径保持/更新 `BitloomFirrtlParser.parse`（或 docs 等价）对目标钉死对可验收 | 删除 FR138 产品路径 |
| **AD-27 / NFR75** | **103.2 ready 前**修订 AD-27（及若升钉则 AD-9）落入脊柱 | 静默改 AD 或无修订宣称 HEAD |
| **验收谓词** | ATDD Pass；故意错版本/缺 Parser → Fail；FR138/FR165/FR130 回归不破 | docs-only；FR165 alone |

### 验收谓词 / 失败语义（103.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | 目标钉死对下 Parser 产品路径 Pass；`docs/fr170-*`；AD-27（±AD-9）已修订；ATDD 可复现 |
| **负向** | 错版本 / 缺工具 / force-missing → 可读非零 |
| **禁止勾选** | FR138 alone；FR165 Style Guide/linter alone；FR130 alone；仅 docs；无 AD-27 修订 |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **103.1** | 本 NFR14 |
| **103.2** | HEAD/更新主线 Parser 实现 + ATDD；AD-27（±AD-9）修订；`docs/fr170-*` |
| **103.3** | README/deferred/spine 收口；勾选 Epic 103；更深生态仍 **NFR76** |

### (a) 上游约束

- **Epic 99 已关闭。**
- **FR138 / FR165 / FR130：** **仍有效**；alone ≠ FR170。
- **AD-27：** 本批 **必须修订**；若升钉工具链联动 **AD-9**（NFR75）。
- **NFR73：** 不得改写 FR94–165「已关闭」。
- **NFR76：** 禁止静默扩大超出本记录钉死子集。
- **软序：** 若 Epic 102 含 firtool/Chisel 升钉配对 → **建议先 102 再 103**。
- **品牌：** Bitloom；设计 crate → `bitloom-prelude`；Scala/Parser 运行时不得进设计 crate。

### (b) 粗工期带

- **预计：** Epic 103 整体 **High**（上游 HEAD 漂移）；本 NFR14 ≤0.25 人周；103.2 约 1–3 人周。
- **置信度：** 中–低（Chisel HEAD / 配对节奏）。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **103.2–103.3** 标 ready。
- **不得**以 FR138 alone / FR165 alone / FR130 alone 勾选 FR170。
- **不得**无 AD-27 修订宣称 HEAD Parser 已回迁。
- **不得**静默扩大 FR142；**不得**把 `git push` 当 FR。
- 不得把本记录冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR73** / **NFR75** / **NFR76** / **AD-27**
- 升级：缩回「仅 FR165 linter」须 Correct Course / 产品批准。

### 引用

- AD-28；AD-27；AD-9；FR170；对照 FR138 / FR165 / FR130；Phase 20 Correct Course approved 2026-09-12
- **NFR14-crates** ≠ 本门禁

---

### Epic 103 关闭条件（Story 103.3 勾选）

- [x] **FR170 钉死子集实现 + 验收** — Story 103.2（update-mainline Parser + AD-27 修订）
- [x] **文档 / deferred / README / spine 收口** — Story 103.3
- [x] **NFR73/76：** 边界与诚实义务保持；unpaired HEAD/firtool 升钉须新合同
- [x] **品牌 / AD-6：** Bitloom；Scala/Parser 运行时不得进入设计 crate
- [x] **其余 FR171：** 未关前不得宣称 Phase 20 全清 / NFR71 账本已空
