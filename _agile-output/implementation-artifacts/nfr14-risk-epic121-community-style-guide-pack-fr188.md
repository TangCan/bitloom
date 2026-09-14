# NFR14 风险记录 — Epic 121 社区 Style Guide 全家桶（FR188）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28** / **AD-27**；Phase 23 **NFR88–NFR92**；实现面 **FR188**。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic114-deeper-style-guide-linter-fr181.md`；`nfr14-risk-epic120-handshake-lower-deepen-fr187.md`。  
> **前置：** Epic 118 / FR185 **closed**；Epic 114 / **FR181** wartremover+fatal-warnings deepen **closed**（仍有效 · NFR88）；Epic 109 / **FR176**、Epic 97 / **FR165**、Epic 70 / **FR130** **closed**。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **121.2–121.3** 标 `ready`。**不得**以 FR181 alone / FR176 alone / FR165 alone / FR130 alone 勾选本 FR。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR188 / Epic 121；NFR14、NFR88、NFR90、NFR91；对照 FR181 / FR176 / FR165 / FR130 / AD-27 |
| 记录日期 | 2026-09-14 |
| 状态 | **open / in-progress**（Story 121.1；Epic 121 未关闭；关闭勾选 → Story 121.3） |
| **选定** | 在保留 FR181 `chisel-wartremover-rules`+`fatal-warnings-lint` 关闭面的前提下，授权 **社区 Style Guide 全家桶** 产品路径：至少增加 **`chisel-community-style-guide` + `scalafmt-community`** 标记 + `emit_chisel_style_guide_pack_fr188` / `check_chisel_style_guide_pack_fr188`（或文档等价）+ per-module `--- FR188 style-guide-pack ---`；产品门禁须 **串联** FR181 deepen gate；触及 **AD-27** 须按 **NFR90** 先修订；禁止仅重跑 FR181/FR176 冒充全家桶 |

### Phase 22 / FR181 关闭面 vs Epic 121（NFR88）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR130 / Epic 70** | Style Guide S1–S4 | **仍有效**；alone ≠ FR188 |
| **FR165 / Epic 97** | Style Guide / linter L1–L5 | **仍有效**；alone ≠ FR188 |
| **FR176 / Epic 109** | 组合生态包 E1–E4 | **仍有效**；alone ≠ FR188 |
| **FR181 / Epic 114** | wartremover + fatal-warnings deepen | **仍有效**；**alone ≠ FR188** |
| **Epic 121 / FR188** | 社区 Style Guide 全家桶（超 FR181） | **本实现 epic** |

### 本批钉死（121.2 必须）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **全家桶子集** | 相对 FR181：新增至少 **`FR188` 头** + **`chisel-community-style-guide` + `scalafmt-community`** + per-module `--- FR188 style-guide-pack ---` + `emit_chisel_style_guide_pack_fr188` / `check_chisel_style_guide_pack_fr188`；产品门禁须 **串联** FR181 style-linter deepen gate（`chisel-style-linter-deepen-check`）**且** FR188 pack 标记检查 | 仅再跑 FR181 / FR176 / FR165 / FR130；任意 IDE 插件全家桶 |
| **产品门禁** | `just chisel-style-guide-pack-check`（或文档等价）+ CI / ATDD；`BITLOOM_STYLE_GUIDE_PACK_FORCE_MISSING=1` → 非零可读 | `continue-on-error`；silent-Ok |
| **AD 修订（NFR90）** | Story **121.2 ready 前**修订适用 **AD-27**（社区 Style Guide 全家桶 vs FR181 deepen 边界） | 无 AD 修订静默宣称 |
| **与 FR181 / FR176 边界** | FR181 = wartremover+fatal-warnings deepen；FR188 = **超 FR181 的社区 Style Guide pack 层** | FR181/176/165/130 alone 勾选 |

### 验收谓词 / 失败语义（121.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | 全家桶通道验收通过；`docs/fr188-*`；AD-27 已修订（NFR90） |
| **负向** | 缺标记 / 子门禁失败 / FORCE_MISSING → 可读非零 |
| **禁止勾选** | FR181 alone；FR176 alone；FR165 alone；FR130 alone；仅 docs；未修订 AD-27 |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **121.1** | 本 NFR14 |
| **121.2** | Style Guide 全家桶实现 + ATDD/CI；AD-27 先入脊柱（NFR90）；`docs/fr188-*` |
| **121.3** | README/deferred/spine 收口；勾选 Epic 121；未写入子集仍 **NFR91** |

### (a) 上游约束

- **Epic 118 / FR185 已关闭。**
- **FR181 / FR176 / FR165 / FR130：** **仍有效**；alone ≠ FR188。
- **NFR90：** 社区 Style Guide 全家桶须先修订 AD-27。
- **NFR88 / NFR91：** 不得改写已关闭；不得静默超子集（任意 IDE 插件 / 完整社区 Style Guide 超本批钉死子集仍 **NFR91**）。
- **品牌：** Bitloom；Scala/Parser 运行时不得进入设计 crate（AD-6 / `bitloom-prelude`）。

### (b) 粗工期带

- **预计：** Epic 121 **Medium**；本 NFR14 ≤0.25 人周；121.2 约 0.5–1.5 人周（复用 FR181 gate + 新 pack 层）。
- **置信度：** 中–高（FR181 deepen 门禁已在树）。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **121.2–121.3** 标 ready。
- **不得**以 FR181 / FR176 / FR165 / FR130 alone 勾选 FR188。
- **不得**仅重跑 FR181 style-linter / FR176 ecosystem 冒充本 FR。
- **不得**无 AD-27 修订静默宣称（NFR90）。
- **不得**静默扩大 FR142；**不得**把 `git push` 当 FR。
- 不得把本记录冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR88** / **NFR90** / **NFR91** / **AD-27**
- 升级：缩回「仅 FR181 wartremover deepen」须 Correct Course / 产品批准。

### 引用

- AD-28；AD-27；FR188；对照 FR181 / FR176 / FR165 / FR130；Phase 23 Correct Course approved 2026-09-14
- `docs/fr181-deeper-style-guide-linter.md` — 社区 Style Guide 全家桶曾归 NFR86 → 本 FR
- **NFR14-crates** ≠ 本门禁

---

### Epic 121 关闭条件（Story 121.3 勾选）

- [ ] **Style Guide 全家桶通道 + 验收谓词：** Story 121.2
- [ ] **AD-27 修订（NFR90）：** Story 121.2 ready 前落地
- [ ] **docs/fr188-* + README/deferred：** Story 121.3
- [ ] **NFR88：** FR181/FR176/FR165/FR130 关闭面未改写
- [ ] **禁止事项未触发：** 未用 FR181/FR176 alone 勾选；未 silent-Ok
- [ ] **品牌 / 依赖：** Bitloom / `bitloom-prelude`
- [ ] **超子集：** 未写入本 epic NFR14 的更深 IDE/社区 Style 仍 **NFR91**
