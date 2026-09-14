# NFR14 风险记录 — Epic 114 更深 Style Guide / linter（FR181）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28** / **AD-27**；Phase 22 **NFR83–NFR87**；实现面 **FR181**。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic109-deeper-parser-chisel-ecosystem-fr176.md`；`nfr14-risk-epic113-handshake-dialect-deepen-fr180.md`。  
> **前置：** Epic 111 / FR178 **closed**；Epic 109 / **FR176** 组合生态包 **closed**（仍有效 · NFR83）；Epic 97 / **FR165**、Epic 70 / **FR130**、Epic 77 / **FR138** **closed**。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **114.2–114.3** 标 `ready`。**不得**以 FR176 alone / FR165 alone / FR130 alone / FR138 alone 勾选本 FR。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR181 / Epic 114；NFR14、NFR83、NFR85、NFR86；对照 FR176 / FR165 / FR130 / FR138 / AD-27 |
| 记录日期 | 2026-09-14 |
| 状态 | **open — Story 114.1**（in-progress）；Epic 114 未关闭 |
| **选定** | 在保留 FR176 组合生态包关闭面的前提下，授权 **更深 Style Guide / linter** 产品路径：至少新增一层可检 **style-linter deepen** 标记（`emit`/`check` + `just` / CI；超 FR176 E1–E4）；触及 **AD-27** 须按 **NFR85** 先修订；禁止仅重跑 FR176/FR165/FR130 冒充加深 |

### Phase 15/19/21 关闭面 vs Epic 114（NFR83）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR130 / Epic 70** | Style Guide S1–S4 | **仍有效**；alone ≠ FR181 |
| **FR138 / Epic 77** | `BitloomFirrtlParser.parse` | **仍有效**；alone ≠ FR181 |
| **FR165 / Epic 97** | Style Guide / linter L1–L5 | **仍有效**；alone ≠ FR181 |
| **FR176 / Epic 109** | 组合 FR165+FR170+ecosystem 标记 | **仍有效**；**alone ≠ FR181** |
| **Epic 114 / FR181** | 更深 Style Guide / linter（超 FR176） | **本实现 epic** |

### 本批钉死（114.2 必须）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **加深子集** | 相对 FR176：新增至少 **`FR181` 头** + 社区 style-linter 标记（至少 `chisel-wartremover-rules` **或** 文档等价 + `fatal-warnings-lint`）+ per-module `--- FR181 style-linter ---` + `emit_chisel_style_linter_fr181` / `check_chisel_style_linter_fr181`；产品门禁须 **串联** FR176 ecosystem gate（`chisel-ecosystem-deepen-check`）**且** FR181 deepen 标记检查 | 仅再跑 FR176 / FR165 / FR130 |
| **产品门禁** | `just chisel-style-linter-deepen-check`（或文档等价）+ CI / ATDD；`BITLOOM_STYLE_LINTER_DEEPEN_FORCE_MISSING=1` → 非零可读 | `continue-on-error`；silent-Ok |
| **AD 修订（NFR85）** | Story **114.2 ready 前**修订适用 **AD-27**（Style/linter deepen vs FR176 组合包边界） | 无 AD 修订静默宣称 |
| **与 FR176 / FR165 边界** | FR176 = 组合生态包 E1–E4；FR165 = L1–L5 alone；FR181 = **超 FR176 的 style-linter deepen 层** | FR176/165/130/138 alone 勾选 |

### 验收谓词 / 失败语义（114.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | deepen 通道验收通过；`docs/fr181-*`；AD-27 已修订（NFR85） |
| **负向** | 缺标记 / 子门禁失败 / FORCE_MISSING → 可读非零 |
| **禁止勾选** | FR176 alone；FR165 alone；FR130 alone；FR138 alone；仅 docs；未修订 AD-27 |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **114.1** | 本 NFR14 |
| **114.2** | Style/linter deepen 实现 + ATDD/CI；AD-27 先入脊柱（NFR85）；`docs/fr181-*` |
| **114.3** | README/deferred/spine 收口；勾选 Epic 114；未写入子集仍 **NFR86** |

### (a) 上游约束

- **Epic 111 / FR178 已关闭。**
- **FR176 / FR165 / FR130 / FR138：** **仍有效**；alone ≠ FR181。
- **NFR85：** Style/linter 加深须先修订 AD-27。
- **NFR83 / NFR86：** 不得改写已关闭；不得静默超子集（完整社区 Style Guide 全家桶 / 任意 IDE 插件全家桶仍 **NFR86**）。
- **品牌：** Bitloom；Scala/Parser 运行时不得进入设计 crate（AD-6）。

### (b) 粗工期带

- **预计：** Epic 114 **Medium**；本 NFR14 ≤0.25 人周；114.2 约 0.5–1.5 人周（复用 FR176 gate + 新标记层）。
- **置信度：** 中–高（FR176 组合门禁已在树）。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **114.2–114.3** 标 ready。
- **不得**以 FR176 / FR165 / FR130 / FR138 alone 勾选 FR181。
- **不得**仅重跑 FR176 ecosystem / FR165 style-lint 冒充本 FR。
- **不得**无 AD-27 修订静默宣称（NFR85）。
- **不得**静默扩大 FR142；**不得**把 `git push` 当 FR。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR83** / **NFR85** / **NFR86** / **AD-27**
- 升级：缩回「仅 FR176 组合包」须 Correct Course / 产品批准。

### 引用

- AD-28；AD-27；FR181；对照 FR176 / FR165 / FR130 / FR138；Phase 22 Correct Course approved 2026-09-12
- **NFR14-crates** ≠ 本门禁

---

### Epic 114 关闭条件（Story 114.3 勾选）

- [ ] **Style/linter deepen 通道 + 验收谓词：** Story 114.2
- [ ] **AD-27 修订（NFR85）：** Story 114.2 ready 前落地
- [ ] **docs/fr181-* + README/deferred：** Story 114.3
- [ ] **NFR83：** FR176/FR165/FR130/FR138 关闭面未改写
- [ ] **禁止事项未触发：** 未用 FR176/FR165 alone 勾选；未 silent-Ok
- [ ] **品牌 / 依赖：** Bitloom / `bitloom-prelude`
- [ ] **超子集：** 未写入本 epic NFR14 的更深生态/IDE 项仍 **NFR86**
