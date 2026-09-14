# NFR14 风险记录 — Epic 120 Handshake lower 加深（FR187）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28** / **AD-25**；Phase 23 **NFR88–NFR92**；实现面 **FR187**。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic113-handshake-dialect-deepen-fr180.md`；`nfr14-risk-epic119-unbounded-circt-tip-fr186.md`。  
> **前置：** Epic 118 / FR185 **closed**；Epic 113 / **FR180** fork+join deepen **closed**（仍有效 · NFR88）；Epic 69 / **FR129** C1–C4 **closed**；Epic 108 / **FR175** **closed**。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **120.2–120.3** 标 `ready`。**不得**以 FR180 alone / FR129 alone / FR175 alone 勾选本 FR。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR187 / Epic 120；NFR14、NFR88、NFR90、NFR91；对照 FR180 fork+join / FR129 C1–C4 / FR175 / AD-25 |
| 记录日期 | 2026-09-14 |
| 状态 | **closed — Story 120.3**（Epic 120 / FR187 可宣称；≠ FR180 alone） |
| **选定** | 在保留 FR180 `handshake.fork`+`handshake.join` 关闭面的前提下，授权 **Handshake lower/dialect 再加深**：至少增加 **`handshake.branch` + `handshake.merge`**（控制分支/合流）标记 + 产品 API/CLI/ATDD/CI；触及 **AD-25** 须按 **NFR90** 先修订；禁止仅重跑 FR180/FR129 冒充加深 |

### Phase 22 / FR180 关闭面 vs Epic 120（NFR88）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR129 / Epic 69** | CIRCT Handshake C1–C4 | **仍有效**；alone ≠ FR187 |
| **FR175 / Epic 108** | `--ir-sv` / `--ir-verilog` | **仍有效**；alone ≠ FR187 |
| **FR180 / Epic 113** | `handshake.fork` + `handshake.join` deepen | **仍有效**；**alone ≠ FR187** |
| **Epic 120 / FR187** | Handshake lower 加深（超 FR180） | **本实现 epic** |

### 本批钉死（120.2 必须）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **方言 / 谓词** | 相对 FR180：schedule IR 须含 **`handshake.branch`** 与 **`handshake.merge`**，且 `"fr187": true`；保留可回归 FR180 fork+join / FR129 C1–C4 | 仅复述 fork/join；仅加 docs |
| **API / CLI** | 产品 API（如 `schedule_circt_handshake_lower_deepen` / `meets_fr187_*` 或文档等价）+ CLI 开关；缺工具/错形状 → **非零可读** | 静默回退到 FR180 alone |
| **CI / just** | `just` 与/或 required CI / ATDD 同路径；不得 `continue-on-error` | FR180 deepen job alone 冒充 |
| **AD 修订（NFR90）** | Story **120.2 ready 前**修订适用 **AD-25**（lower deepen vs FR180 fork+join 边界） | 无 AD 修订静默宣称 |
| **与 FR180 / FR129 边界** | FR180 = fork+join；FR187 = **超 fork+join 的 branch+merge lower 加深** | FR180 alone / FR129 alone 勾选 |

### 验收谓词 / 失败语义（120.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | lower deepen 通道验收通过；`docs/fr187-*`；AD-25 已修订（NFR90） |
| **负向** | 缺方言标记 / 错形状 / force-missing → 可读非零 |
| **禁止勾选** | FR180 alone；FR129 alone；FR175 alone；仅 docs；未修订 AD-25 |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **120.1** | 本 NFR14 |
| **120.2** | Handshake lower deepen 实现 + ATDD/CI；AD-25 先入脊柱（NFR90）；`docs/fr187-*` |
| **120.3** | README/deferred/spine 收口；勾选 Epic 120；未写入子集仍 **NFR91** |

### (a) 上游约束

- **Epic 118 / FR185 已关闭。**
- **FR180 / FR129 / FR175：** **仍有效**；alone ≠ FR187。
- **NFR90：** Handshake lower 加深须先修订 AD-25。
- **NFR88 / NFR91：** 不得改写已关闭；不得静默超子集（完整 CIRCT Handshake lower 全家桶超本批仍 **NFR91**）。
- **品牌：** Bitloom；设计依赖不变（AD-6 / `bitloom-prelude`）。

### (b) 粗工期带

- **预计：** Epic 120 **Medium**；本 NFR14 ≤0.25 人周；120.2 约 1–2 人周（方言标记 + API/CLI + AD-25）。
- **置信度：** 中–高（树内 schedule IR 已有 FR180 形状可再加深）。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **120.2–120.3** 标 ready。
- **不得**以 FR180 / FR129 / FR175 alone 勾选 FR187。
- **不得**仅重跑 FR180 ATDD / FR129 ATDD 冒充本 FR。
- **不得**无 AD-25 修订静默宣称（NFR90）。
- **不得**静默扩大 FR142；**不得**把 `git push` 当 FR。
- 不得把本记录冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR88** / **NFR90** / **NFR91** / **AD-25**
- 升级：缩回「仅 FR180 fork+join」须 Correct Course / 产品批准。

### 引用

- AD-28；AD-25；FR187；对照 FR180 / FR129 / FR175；Phase 23 Correct Course approved 2026-09-14
- `docs/fr180-handshake-dialect-deepen.md` — 更深 lower 曾归 NFR86 → 本 FR
- **NFR14-crates** ≠ 本门禁

---

### Epic 120 关闭条件（Story 120.3 勾选）

- [x] **Handshake lower deepen 通道 + 验收谓词：** Story 120.2
- [x] **AD-25 修订（NFR90）：** Story 120.2 ready 前落地
- [x] **docs/fr187-* + README/deferred：** Story 120.3
- [x] **NFR88：** FR180/FR129/FR175 关闭面未改写
- [x] **禁止事项未触发：** 未用 FR180/FR129 alone 勾选；未 silent-Ok
- [x] **品牌 / 依赖：** Bitloom / `bitloom-prelude`
- [x] **超子集：** 未写入本 epic NFR14 的更深 dialect/lower 仍 **NFR91**
