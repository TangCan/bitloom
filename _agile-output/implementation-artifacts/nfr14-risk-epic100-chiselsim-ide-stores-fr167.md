# NFR14 风险记录 — Epic 100 完整 ChiselSim / 多端 IDE 商店（FR167）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**；Phase 20 **NFR73–NFR77**；实现面 **FR167**；宣称须引对应 FR。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic94-deeper-gui-ide-fr162.md`。  
> **前置：** Epic 99 / FR166 **closed**；Epic 94 / **FR162** 默认 Tywaves GUI 主表面 **closed**；Epic 73 / **FR134** G1–G4 **closed**。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 100 后续故事 **100.2–100.3** 标为 `ready`，亦不得开工实现。**不得**以 FR162 alone 或 FR134 G1–G4 alone 勾选本 FR。**(a)+(b) 皆须交付。**

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR167 / Epic 100；NFR14、NFR73、NFR74、NFR75、NFR76、NFR77；对照 FR162 / FR134 / FR125 / FR117 |
| 记录日期 | 2026-09-12 |
| 状态 | closed — Story 100.3 勾选完成；Epic 100 关闭；FR167 实现面可宣称；**FR168–171** 仍属 Epic 101–104；更深 GUI/商店未写入本记录子集仍 **NFR76** |
| **选定** | 在保留 FR162 / FR134 关闭面的前提下，授权 **(a) 完整 ChiselSim 耦合** 且 **(b) 额外 IDE 商店多端**（至少 Open VSX 与/或 JetBrains 等价端，超出 FR134 G1）**二者皆交付** |

### Phase 12–19 / FR162 关闭面 vs Epic 100 实现边界（NFR73 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR134 / Epic 73** | G1–G4 opt-in GUI/IDE | **仍有效；alone ≠ FR167** |
| **FR162 / Epic 94** | 默认 Tywaves GUI 主表面 | **仍有效；alone ≠ FR167** |
| **Epic 99 / FR166** | Phase 20 闸门 | **已关闭** |
| **Epic 100 / FR167** | ChiselSim + 多端 IDE 商店 | **本实现 epic** |
| **Epic 101–104** | 其它 Phase 20 | **本 epic 不做** |

**选定：交付完整 ChiselSim 耦合 + 额外 IDE 商店多端；≠ 重做 FR162 默认面；≠ 仅文档宣称。**

### 本批钉死（100.2 必须 · (a)+(b)）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **(a) ChiselSim** | 文档 + 产品路径：完整 ChiselSim 耦合（相对 FR162 Tywaves GUI 的加深）；验收谓词/夹具或文档化手动清单在 100.2 钉死 | 删除 FR162 默认面；冒充 FR162 alone |
| **(b) IDE 商店多端** | 至少超出 FR134 G1 的 **Open VSX** 与/或 **JetBrains** 等价发布端；产物/版本/凭证在 100.2 钉死 | 仅 VS Code marketplace G1 复用冒充多端 |
| **失败语义** | 缺令牌 / 缺工具 / force-missing → **非零可读**；**不得** silent-Ok 冒充已发布 | `continue-on-error` / skip |
| **NFR75** | 触及商店发布前修订文档/CI 合同指针 | 无修订静默发版 |

### 验收谓词 / 失败语义（100.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | (a) 与 (b) 验收谓词均通过；文档显式对照 FR162 / FR134；ATDD 或文档化手动清单可复现 |
| **负向** | 缺令牌/缺工具 → 可读非零；禁止 silent-Ok |
| **禁止勾选** | FR162 alone；FR134 G1–G4 alone；仅 docs；只做 (a) 或只做 (b) |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **100.1** | 本 NFR14 |
| **100.2** | (a)+(b) 实现 + ATDD/手册；`docs/fr167-*` |
| **100.3** | README/deferred 收口；勾选 Epic 100；写明未写入 NFR14 的更深 GUI/商店仍 **NFR76** |

### (a) 上游约束

- **Epic 99 已关闭。**
- **FR162 / FR134：** **仍有效**；不得改写为失败；alone ≠ FR167。
- **NFR73：** 不得改写 FR94–165「已关闭」。
- **NFR75：** IDE 商店发布须诚实同步文档/CI。
- **NFR76：** 禁止静默扩大超出本记录钉死子集。
- **品牌：** Bitloom；设计 crate 仍 `bitloom-prelude`。

### (b) 粗工期带

- **预计：** Epic 100 整体 **High**（ChiselSim 耦合 + 多商店凭证/CI）；本 NFR14 ≤0.25 人周。
- **置信度：** 中–低（外部商店/上游依赖）。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **100.2–100.3** 标 ready。
- **不得**以 FR162 alone / FR134 alone 勾选 FR167。
- **不得**只交付 (a) 或只交付 (b)。
- **不得**缺令牌 silent-Ok 冒充已发布。
- **不得**静默扩大 FR142；**不得**把 `git push` 当 FR。
- 不得把本记录冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR73** / **NFR75** / **NFR76**
- 升级：缩回「只做一边」须 Correct Course / 产品批准。

### 引用

- AD-28；FR167；对照 FR162 / FR134；Phase 20 Correct Course approved 2026-09-12
- **NFR14-crates** ≠ 本门禁

---

### Epic 100 关闭条件（Story 100.3 勾选）

- [x] **FR167 钉死子集实现 + 验收** — Story 100.2（(a)+(b)）
- [x] **文档 / deferred / README 收口** — Story 100.3
- [x] **NFR73/75/76：** 边界与诚实义务保持；未写入更深 GUI/商店须新合同
- [x] **品牌 / AD-6：** Bitloom；设计 crate → `bitloom-prelude`
- [x] **其余 FR168–171：** 未关前不得宣称 Phase 20 全清 / NFR71 账本已空
