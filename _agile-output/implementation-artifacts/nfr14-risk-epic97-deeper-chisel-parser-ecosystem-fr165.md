# NFR14 风险记录 — Epic 97 更深 Chisel/Parser 生态（FR165）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**；Phase 19 **NFR68–NFR72**；实现面 **FR165**；宣称须引对应 FR。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic77-restore-parser-ad27.md` / `nfr14-risk-epic96-broader-circt-mlir-sim-gate-fr164.md`。  
> **前置：** Epic 87 / FR154 **closed**；Epic 77 / **FR138** Parser 产品路径 **closed**；Epic 70 / **FR130** Style Guide S1–S4 **closed**；Epic 96 / FR164 **closed**。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 97 后续故事 **97.2–97.3** 标为 `ready`，亦不得开工实现。**不得**以 FR138 P1–P4 alone 勾选本 FR。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR165 / Epic 97；NFR14、NFR68、NFR69、NFR70、NFR71、NFR72；对照 FR138 / FR130 / FR122 / AD-27 / AD-9 |
| 记录日期 | 2026-09-12 |
| 状态 | closed — Story 97.3 勾选完成；Epic 97 关闭；FR165 实现面可宣称；**FR156** 仍属 Epic 98；**不得**宣称 NFR59「全清」；未选 Chisel HEAD Parser 仍 **NFR71** |
| **选定** | 在保留 FR138 `BitloomFirrtlParser.parse` / FR130 Style Guide S1–S4 关闭面的前提下，授权 **MVP：Style Guide / linter 加深包** — 超出 FR130 标记面的可检查 lint/规则包（产品 API + `just`/脚本门禁 + ATDD）；**任意 Chisel HEAD Parser 回迁** **不做**（须新合同 / NFR71 + AD-9/AD-27） |

### Phase 12–18 / FR138 关闭面 vs Epic 97 实现边界（NFR68 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR122 / FR111 / FR97** | idiomatic / 官方风格 Chisel | **仍有效**；alone ≠ FR165 |
| **FR130 / Epic 70** | Style Guide S1–S4（Parser 未恢复于该 FR） | **仍有效**；alone ≠ FR165 |
| **FR138 / Epic 77** | `BitloomFirrtlParser.parse` P1–P4 + AD-27 | **仍有效；不得改写为失败**；**alone ≠ FR165** |
| **Epic 87–96** | 闸门 / … / FR164 | **已关闭；本 epic 不做** |
| **Epic 97 / FR165** | **更深 Chisel/Parser 生态**（本记录选定：Style Guide/linter 加深） | **本实现 epic** |
| **Epic 98** | FR156 宣称 | **本 epic 不做** |

**选定：关闭「仅 FR138 Parser / FR130 Style 标记」缺口；≠ 重做 FR138；≠ 本批交付任意 Chisel HEAD Parser 回迁。**

### FR165 候选子集与本批选定（NFR71）

| 候选（Epic / README 明示） | 本 epic |
| --- | --- |
| **Style Guide / linter 加深包**（超出 FR130 S1–S4） | **选定 = FR165 MVP** |
| 任意 Chisel HEAD Parser 回迁 / 升钉超 AD-9 配对 | **不做**（须新合同 / NFR71 + AD-9/AD-27） |

### 选定子集钉死（97.2 必须）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **产品形状** | 文档钉死的 emit/check（或等价）+ `just …` / `scripts/…` 门禁：对代表性 FrozenHir 发出 **超出 FR130** 的 Style Guide/linter 契约（额外规则/标记/检查谓词须在 `docs/fr165-*` 钉死） | 仅再跑 FR130 `emit_chisel_style_guide_fr130` 冒充加深 |
| **验收谓词** | (1) docs 声明 ≠ FR138/FR130 alone；(2) ATDD Pass；(3) 故意缺标记 / 错规则 → Fail；(4) FR138/FR130 回归不破 | docs-only |
| **失败语义** | 检查失败 / force-missing（若适用）→ **非零可读** | silent-Ok |
| **AD-27 / NFR70** | **本批不修订 AD-27**（linter 加深不改变 Parser 产品关闭合同）；若 97.2 改选 HEAD Parser 回迁须先修订 AD-27（及 AD-9 若升钉）再标 ready | 静默改 AD-27 / 升钉 Chisel HEAD |
| **crate 边界** | 设计 crate → **`bitloom-prelude`**；lint/emit 在 firrtl/工具链侧 | Scala 运行时进设计 crate |

### 目标产品形状（97.2）

- **入口：** `emit_chisel_style_guide_fr165` / `check_chisel_style_guide_fr165`（命名可微调，须在 docs 钉死）+ `just chisel-style-lint-check`（或等价）。
- **产物：** 加深 Style Guide/linter 契约；`docs/fr165-*`；ATDD。
- **语义锚：** 「更深 Style Guide/linter 生态」须引 **FR165**，不得只引 FR130/FR138。

### 验收谓词 / 失败语义（97.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | 钉死加深规则/标记可检查 Pass；文档对照 FR130/FR138；ATDD 可复现 |
| **负向 / 失败** | 缺加深契约 → Fail / 可读；**不得** silent-Ok |
| **禁止勾选** | 仅 FR138 P1–P4；仅 FR130 S1–S4；仅 FR122；仅 docs |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **97.1** | 本 NFR14 |
| **97.2** | Style Guide/linter 加深实现 + ATDD；`docs/fr165-*` |
| **97.3** | README/deferred/spine 收口；勾选 Epic 97；写明未选 HEAD Parser 仍须新合同 |

### (a) 上游约束

- **Epic 87 已关闭；Epic 88–96 已关闭。**
- **FR138：** P1–P4 **仍有效**；alone ≠ FR165。
- **FR130 / FR122：** 仍有效；alone ≠ FR165。
- **AD-9：** 保持 Chisel 7.14.0 / firtool-1.155.0；HEAD 升钉须另开。
- **NFR68：** 不得改写 FR94–164 / FR138「已关闭」。
- **NFR70：** 本批 **不**修订 AD-27；若改选 HEAD Parser 回迁须先修订脊柱。
- **NFR71：** 禁止静默扩大到任意 Chisel HEAD Parser 回迁。
- **NFR72：** 未关 FR165 前不得宣称更深生态已交付；**不得以 FR138 alone 冒充 FR165**。
- **品牌：** **Bitloom**；设计 crate → **`bitloom-prelude`**（AD-6）。
- **软序：** 97.2 → 97.3。

### (b) 粗工期带

- **预计：** Epic 97 整体约 **0.75–2.5 人周**（97.1 ≤0.25；97.2 emit/check/门禁 + ATDD 0.5–2；97.3 收口 0.25）。
- **置信度 / 假设：** 中（复用 FR130 Style Guide 体例加深规则集）。假设不升 AD-9 / 不改 AD-27。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **97.2–97.3** 标 `ready` 或开工实现。
- **不得以 FR138 P1–P4 alone 勾选 FR165。**
- **不得以 FR130 / FR122 alone 冒充 FR165。**
- **不得仅改文档关闭 FR165。**
- **不得静默扩大到任意 Chisel HEAD Parser 回迁。**
- **不得未履行 NFR70 即静默改 AD-27 / 升钉 AD-9。**
- **不得改写 Phase 12–18 / FR138 / FR130 关闭证据为失败。**
- 不得把本记录冒充 **NFR14-crates**（FCFS）完成定义。
- 不得在 FR165 未关时宣称 NFR59「全清」。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 与 **NFR68 / NFR69 / NFR70 / NFR71 / NFR72** 共同责任人
- 备份 / 升级路径：改选「任意 Chisel HEAD Parser 回迁」代替 Style Guide/linter 加深，须升级至产品 / Correct Course 批准人并修订本记录（及 AD-27/AD-9）。

---

### Epic 97 关闭条件（Story 97.3 勾选）

- [x] **FR165 钉死子集实现 + 验收** — Story 97.2
- [x] **文档 / deferred / README / spine 收口** — Story 97.3
- [x] **NFR68/70/71/72：** 边界与诚实义务保持；未选 HEAD Parser 仍须新合同
- [x] **品牌 / AD-6：** Bitloom；设计 crate → `bitloom-prelude`
- [x] **FR156 / Epic 98：** 宣称门仍开放；不得以本 epic alone 冒充 Phase 19 全清
