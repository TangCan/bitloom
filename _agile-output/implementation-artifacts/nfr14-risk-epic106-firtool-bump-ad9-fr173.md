# NFR14 风险记录 — Epic 106 firtool 升钉超 AD-9（FR173）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28** / **AD-9**；Phase 21 **NFR78–NFR82**；实现面 **FR173**；宣称须引对应 FR。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic102-circt-mlir-firtool-fr169.md`。  
> **前置：** Epic 105 / FR172 **closed**（Story 105.4）；Epic 102 / **FR169**（**(A)** multi-lower @ firtool-1.155.0）**closed**；Epic 96 / **FR164**、Epic 76 / **FR137** **closed**。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 106 后续故事 **106.2–106.3** 标为 `ready`，亦不得开工实现。**不得**以 FR169 alone / FR164 alone / FR137 alone 勾选本 FR。升钉须上游 Chisel **正式配对** + 修订 **AD-9** / Stack（**NFR80** / NFR12）。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR173 / Epic 106；NFR14、NFR78、NFR79、NFR80、NFR81；对照 FR169 / FR164 / FR137 / AD-9 |
| 记录日期 | 2026-09-12 |
| 状态 | **closed — Story 106.3**；Epic 106 / FR173 可宣称；未写入更高升钉仍 **NFR81** |
| **选定** | 在保留 FR169(A) @ firtool-1.155.0 / FR164 / FR137 关闭面的前提下，授权经 Chisel **正式配对**后的 **firtool 升钉超现行 AD-9 `firtool-1.155.0`**，并 **必须**同步修订 **AD-9 / Stack**；禁止 unpaired bump / PATH-random firtool |

### Phase 12–20 关闭面 vs Epic 106 实现边界（NFR78 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR137 / Epic 76** | 外部 firtool **编译**门禁 | **仍有效**；alone ≠ FR173 |
| **FR164 / Epic 96** | 外部 **仿真**门禁 | **仍有效**；alone ≠ FR173 |
| **FR169 / Epic 102** | multi-lower / HW dialect @ **firtool-1.155.0**（**(B)** 升钉未选） | **仍有效；不得改写为失败**；**alone ≠ FR173** |
| **Epic 105 / FR172** | Phase 21 闸门 | **已关闭** |
| **Epic 106 / FR173** | firtool 升钉配对 AD-9 | **本实现 epic** |
| **Epic 107–110** | HEAD / 更广 CIRCT / Parser / 宣称 | **本 epic 不做** |

**选定：关闭 NFR76「firtool 升钉超 AD-9」缺口；≠ 重做 FR169(A) multi-lower alone；≠ 仅 docs。**

### 本批钉死（106.2 必须）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **目标配对** | **Chisel 7.15.0 ↔ firtool-1.158.0**（上游正式表：https://www.chisel-lang.org/docs/appendix/versioning ；相对现行 AD-9 Chisel 7.14.0 ↔ firtool-1.155.0） | PATH 随机；未在正式表的配对；跳过 Chisel 升版本只换 firtool |
| **AD-9 / Stack** | Story **106.2 ready 前**落地修订 AD-9 Rule + Stack 表 + 运维清单（资产 `firrtl-bin-linux-x64.tar.gz` @ `firtool-1.158.0` + `.sha256`）（**NFR80**） | 无 AD 修订静默升；仅改 CI 字符串 |
| **验收谓词** | 下载/缓存校验；`just` / CI 目标钉死新 pin；版本不符 / 缺工具 → **非零可读**；不得 silent-Ok | `continue-on-error`；PATH 默认信任 |
| **与 FR169 边界** | FR169 = (A) allocation @ **1.155.0**；FR173 = **产品钉死 firtool 升至 1.158.0 + AD-9** | FR169 alone / FR164 alone / FR137 alone 勾选 |

### 验收谓词 / 失败语义（106.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | 产品路径调用 **firtool-1.158.0**（Chisel 7.15.0 配对）；AD-9 / Stack 已修订；`docs/fr173-*`；ATDD/CI 可复现 |
| **负向** | 缺工具 / 错版本 / force-missing → 可读非零 |
| **禁止勾选** | FR169 alone；FR164 alone；FR137 alone；PATH 随机 firtool；未配对 HEAD；仅 docs；无 AD-9 修订 |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **106.1** | 本 NFR14：钉死目标配对、AD-9 计划、验收谓词、与 FR169 边界 |
| **106.2** | 升钉实现 + ATDD/CI；**AD-9 / Stack 先入脊柱**（NFR80）；`docs/fr173-*` |
| **106.3** | README/deferred/spine 收口；勾选 Epic 106；未写入子集仍 **NFR81** |

### (a) 上游约束

- **Epic 105 已关闭。**
- **FR169 / FR164 / FR137：** **仍有效**；alone ≠ FR173。
- **配对证据来源：** Chisel Project Versioning 表 — **Chisel 7.15.0 → Firtool 1.158.0**（官方 docs appendix/versioning）。
- **AD-9 / NFR80 / NFR12：** 升钉必须等正式配对 + 修订脊柱/Stack/运维清单后再标 106.2 ready；禁止未配对 HEAD / PATH-random。
- **NFR78：** 不得改写 FR94–171「已关闭」。
- **NFR81：** 禁止静默扩大超出本记录钉死子集（例如再升到未写入的更高 firtool）。
- **软序：** Epic 107 unpaired HEAD 建议在本 epic 配对升钉之后。
- **品牌：** Bitloom；CIRCT/firtool 运行时 **不得**进入设计 crate（AD-6）。

### (b) 粗工期带

- **预计：** Epic 106 整体 **Medium–High**（下载资产、CI、回归、AD 修订）；本 NFR14 ≤0.25 人周；106.2 约 0.5–2 人周。
- **置信度：** 中–高（官方配对表已存在；实现主要是钉死与回归）。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **106.2–106.3** 标 ready。
- **不得**以 FR169 alone / FR164 alone / FR137 alone 勾选 FR173。
- **不得** PATH 随机 firtool / 未配对 CIRCT HEAD 冒充已升钉。
- **不得**无 AD-9 / Stack 修订静默升钉（NFR80）。
- **不得**静默扩大 FR142；**不得**把 `git push` 当 FR。
- 不得把本记录冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR78** / **NFR80** / **NFR81** / **AD-9**
- 升级：缩回「停留 1.155.0 / 仅 docs」须 Correct Course / 产品批准。

### 引用

- AD-28；AD-9；FR173；对照 FR169 / FR164 / FR137；Phase 21 Correct Course approved 2026-09-12
- 配对表：https://www.chisel-lang.org/docs/appendix/versioning （Chisel 7.15.0 ↔ firtool 1.158.0）
- **NFR14-crates** ≠ 本门禁

---

### Epic 106 关闭条件（Story 106.3 勾选）

- [x] **FR173 钉死子集实现 + 验收** — Story 106.2（firtool-1.158.0 + Chisel 7.15.0；AD-9 已修订）
- [x] **文档 / deferred / README / spine 收口** — Story 106.3
- [x] **NFR78/81：** 边界与诚实义务保持；未写入更高升钉/配对项须新合同
- [x] **品牌 / AD-6：** Bitloom；CIRCT 运行时不得进入设计 crate
- [x] **其余 FR174–177：** 未关前不得宣称 Phase 21 全清 / NFR76 账本已空
