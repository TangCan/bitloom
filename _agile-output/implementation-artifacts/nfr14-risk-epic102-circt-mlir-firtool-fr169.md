# NFR14 风险记录 — Epic 102 更广 CIRCT/MLIR / firtool 升钉（FR169）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28** / **AD-9**；Phase 20 **NFR73–NFR77**；实现面 **FR169**；宣称须引对应 FR。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic96-broader-circt-mlir-sim-gate-fr164.md`。  
> **前置：** Epic 99 / FR166 **closed**；Epic 96 / **FR164** 外部仿真门禁 **closed**；Epic 76 / **FR137** compile **closed**；Epic 69 / **FR129** **closed**。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 102 后续故事 **102.2–102.3** 标为 `ready`，亦不得开工实现。**不得**以 FR164 alone / FR137 alone / FR129 alone 勾选本 FR。升钉须 **AD-9** / Stack + 上游 Chisel **正式配对**（NFR12 / NFR75）。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR169 / Epic 102；NFR14、NFR73、NFR74、NFR75、NFR76、NFR77；对照 FR164 / FR137 / FR129 / AD-9 / AD-25 |
| 记录日期 | 2026-09-12 |
| 状态 | closed — Story 102.3 勾选完成；Epic 102 关闭；FR169 实现面可宣称（**(A)** multi-lower @ firtool-1.155.0）；**(B)** firtool 升钉仍 **NFR76**；**FR170–171** 仍属 Epic 103–104 |
| **选定** | 在保留 FR164 仿真门禁 / FR137 compile 关闭面的前提下，授权 **(A) 更广 CIRCT/MLIR dialect allocation（或多 lower）产品加深** **和/或 (B) 经 Chisel 正式配对后的 firtool 升钉（修订 AD-9）** — **至少交付 (A) 或 (B) 之一，且须超出 FR164**；禁止 PATH 随机 firtool / 未配对 CIRCT HEAD 冒充已升钉 |

### Phase 12–19 / FR164 关闭面 vs Epic 102 实现边界（NFR73 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR129 / Epic 69** | Handshake / 弹性 | **仍有效**；alone ≠ FR169 |
| **FR137 / Epic 76** | 外部 firtool **编译**门禁 | **仍有效**；alone ≠ FR169 |
| **FR164 / Epic 96** | 外部 **仿真**门禁 | **仍有效；不得改写为失败**；**alone ≠ FR169** |
| **Epic 99 / FR166** | Phase 20 闸门 | **已关闭** |
| **Epic 102 / FR169** | 更广 CIRCT/MLIR allocation / firtool 升钉 | **本实现 epic** |
| **Epic 100–101 / 103–104** | 其它 Phase 20 | **本 epic 不做**（Parser → Epic 103） |

**选定：关闭 FR164「更广 MLIR / 升钉 deferred」缺口；≠ 重做 FR164 sim；≠ 仅 docs。**

### 本批钉死（102.2 必须 · 至少 A 或 B）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **(A) CIRCT/MLIR allocation** | 超出 FR164 sim 的 dialect **allocation** 或文档等价 **多 lower** 产品路径 + ATDD/CI；工具版本钉死 | 仅再跑 FR164 sim 冒充加深 |
| **(B) firtool 升钉** | 上游 Chisel **正式配对**证据 + 修订 **AD-9** / Stack（NFR75）后再升；夹具/CI 跟钉 | PATH 随机；未配对 CIRCT HEAD；无 AD-9 修订静默升 |
| **失败语义** | 缺工具 / 版本不符 / force-missing → **非零可读**；不得 silent-Ok | `continue-on-error` |
| **与 FR164 边界** | FR164 = sim gate @ 现行 pin；FR169 = allocation **和/或** 升钉超 FR164 | FR164 alone 勾选 |

### 验收谓词 / 失败语义（102.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | (A) 与/或 (B) 验收谓词通过；`docs/fr169-*`；对照 FR164/FR137；ATDD/CI 可复现 |
| **负向** | 缺工具 / 错版本 / force-missing → 可读非零 |
| **禁止勾选** | FR164 alone；FR137 alone；FR129 alone；PATH 随机 firtool；未配对 HEAD；仅 docs |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **102.1** | 本 NFR14 |
| **102.2** | (A) 与/或 (B) 实现 + ATDD/CI；若 (B) 则 AD-9 先入脊柱（NFR75）；`docs/fr169-*` |
| **102.3** | README/deferred/spine 收口；勾选 Epic 102；未写入子集仍 **NFR76** |

### (a) 上游约束

- **Epic 99 已关闭。**
- **FR164 / FR137 / FR129：** **仍有效**；alone ≠ FR169。
- **AD-9 / NFR12：** 升钉必须等 Chisel 正式配对 + 修订脊柱；禁止未配对 HEAD。
- **NFR73：** 不得改写 FR94–165「已关闭」。
- **NFR75：** 触及 firtool/AD-9 前须修订文档/脊柱指针。
- **NFR76：** 禁止静默扩大超出本记录钉死子集。
- **软序：** 若本批含升钉且 Epic 103 依赖新钉死对 → **先 102 配对再 103 Parser**。
- **品牌：** Bitloom；CIRCT 运行时 **不得**进入设计 crate。

### (b) 粗工期带

- **预计：** Epic 102 整体 **High**（allocation 与/或升钉配对）；本 NFR14 ≤0.25 人周；102.2 约 1–4 人周视是否含升钉。
- **置信度：** 中–低（上游配对节奏 / CIRCT 形态）。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **102.2–102.3** 标 ready。
- **不得**以 FR164 alone / FR137 alone / FR129 alone 勾选 FR169。
- **不得** PATH 随机 firtool / 未配对 CIRCT HEAD 冒充已升钉。
- **不得**无 AD-9 修订静默升钉（若选 B）。
- **不得**静默扩大 FR142；**不得**把 `git push` 当 FR。
- 不得把本记录冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR73** / **NFR75** / **NFR76** / **AD-9**
- 升级：缩回「仅再跑 FR164」须 Correct Course / 产品批准。

### 引用

- AD-28；AD-9；FR169；对照 FR164 / FR137 / FR129；Phase 20 Correct Course approved 2026-09-12
- **NFR14-crates** ≠ 本门禁

---

### Epic 102 关闭条件（Story 102.3 勾选）

- [x] **FR169 钉死子集实现 + 验收** — Story 102.2（**(A)** multi-lower / HW dialect allocation @ AD-9 pin；**(B)** 未选）
- [x] **文档 / deferred / README / spine 收口** — Story 102.3
- [x] **NFR73/76：** 边界与诚实义务保持；firtool 升钉超 AD-9 与未写入子集须新合同
- [x] **品牌 / AD-6：** Bitloom；CIRCT 运行时不得进入设计 crate
- [x] **其余 FR170–171：** 未关前不得宣称 Phase 20 全清 / NFR71 账本已空
