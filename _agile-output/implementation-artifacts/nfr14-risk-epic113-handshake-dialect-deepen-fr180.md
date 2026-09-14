# NFR14 风险记录 — Epic 113 Handshake dialect 加深（FR180）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28** / **AD-25**；Phase 22 **NFR83–NFR87**；实现面 **FR180**。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic69-circt-handshake.md`；`nfr14-risk-epic112-floating-circt-git-head-fr179.md`。  
> **前置：** Epic 111 / FR178 **closed**；Epic 69 / **FR129** CIRCT Handshake C1–C4 **closed**（仍有效 · NFR83）；Epic 108 / **FR175** `--ir-sv`/`--ir-verilog` **closed**（仍有效 · NFR83）；Epic 62 / FR121 ready/valid **closed**。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **113.2–113.3** 标 `ready`。**不得**以 FR129 alone / FR175 alone / FR121 alone 勾选本 FR。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR180 / Epic 113；NFR14、NFR83、NFR85、NFR86；对照 FR129 C1–C4 / FR175 / FR121 / AD-25 |
| 记录日期 | 2026-09-14 |
| 状态 | **open — Story 113.1**（in-progress）；Epic 113 未关闭 |
| **选定** | 在保留 FR129 C1–C4（`handshake.func`/`handshake.buffer` + 多时钟弹性）关闭面的前提下，授权 **Handshake dialect 加深**：至少增加 **`handshake.fork` + `handshake.join`** 控制扇出/扇入标记 + 产品 API/CLI/ATDD/CI；触及 **AD-25** 须按 **NFR85** 先修订；禁止仅重跑 FR129/FR175 冒充加深 |

### Phase 15/21 关闭面 vs Epic 113（NFR83）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR121 / Epic 62** | Handshake ready/valid 默认可综合 | **仍有效**；alone ≠ FR180 |
| **FR129 / Epic 69** | CIRCT Handshake C1–C4（func/buffer + multi-clock elastic） | **仍有效**；**alone ≠ FR180** |
| **FR175 / Epic 108** | 更广 `--ir-sv` / `--ir-verilog` | **仍有效**；**alone ≠ FR180**（≠ Handshake dialect 加深） |
| **Epic 113 / FR180** | Handshake dialect 加深（超 FR129/FR175） | **本实现 epic** |

### 本批钉死（113.2 必须）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **方言 / 谓词** | 相对 FR129：schedule IR 须含 **`handshake.fork`** 与 **`handshake.join`**，且 `"fr180": true`；保留可回归 FR129 C1–C4 | 仅复述 `handshake.func`/`handshake.buffer`；仅加 docs |
| **API / CLI** | 产品 API（如 `schedule_circt_handshake_deepen` / `meets_fr180_*` 或文档等价）+ CLI 开关；缺工具/错形状 → **非零可读** | 静默回退到 FR129 alone |
| **CI / just** | `just` 与/或 required CI / ATDD 同路径；不得 `continue-on-error` | FR175 SV job alone 冒充 |
| **AD 修订（NFR85）** | Story **113.2 ready 前**修订适用 **AD-25**（Handshake deepen vs FR129 C1–C4 边界） | 无 AD 修订静默宣称方言加深 |
| **与 FR129 / FR175 边界** | FR129 = func/buffer + multi-clock；FR175 = firtool SV/ir-verilog；FR180 = **超 C1–C4 的 Handshake 控制 ops 加深** | FR129 alone / FR175 alone 勾选 |

### 验收谓词 / 失败语义（113.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | deepen 通道验收通过；`docs/fr180-*`；AD-25 已修订（NFR85） |
| **负向** | 缺方言标记 / 错形状 / force-missing（若适用）→ 可读非零 |
| **禁止勾选** | FR129 alone；FR175 alone；FR121 alone；仅 docs；未修订 AD-25 |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **113.1** | 本 NFR14 |
| **113.2** | Handshake deepen 实现 + ATDD/CI；AD-25 先入脊柱（NFR85）；`docs/fr180-*` |
| **113.3** | README/deferred/spine 收口；勾选 Epic 113；未写入子集仍 **NFR86** |

### (a) 上游约束

- **Epic 111 / FR178 已关闭。**
- **FR129 / FR175 / FR121：** **仍有效**；alone ≠ FR180。
- **NFR85：** Handshake dialect 加深须先修订 AD-25。
- **NFR83 / NFR86：** 不得改写已关闭；不得静默超子集（完整 CIRCT Handshake 方言 lower 全家桶仍 **NFR86**）。
- **品牌：** Bitloom；设计依赖不变（AD-6）。

### (b) 粗工期带

- **预计：** Epic 113 **Medium**；本 NFR14 ≤0.25 人周；113.2 约 1–2 人周（方言标记 + API/CLI + AD-25）。
- **置信度：** 中–高（树内 schedule IR 已有 FR129 形状可加深）。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **113.2–113.3** 标 ready。
- **不得**以 FR129 / FR175 / FR121 alone 勾选 FR180。
- **不得**仅重跑 FR129 ATDD / FR175 SV 门禁冒充本 FR。
- **不得**无 AD-25 修订静默宣称（NFR85）。
- **不得**静默扩大 FR142；**不得**把 `git push` 当 FR。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR83** / **NFR85** / **NFR86** / **AD-25**
- 升级：缩回「仅 FR129 C1–C4」须 Correct Course / 产品批准。

### 引用

- AD-28；AD-25；FR180；对照 FR129 / FR175 / FR121；Phase 22 Correct Course approved 2026-09-12
- **NFR14-crates** ≠ 本门禁

---

### Epic 113 关闭条件（Story 113.3 勾选）

- [x] **Handshake deepen 通道 + 验收谓词：** Story 113.2
- [x] **AD-25 修订（NFR85）：** Story 113.2 ready 前落地
- [x] **docs/fr180-* + README/deferred：** Story 113.3
- [x] **NFR83：** FR129/FR175/FR121 关闭面未改写
- [x] **禁止事项未触发：** 未用 FR129/FR175 alone 勾选；未 silent-Ok
- [x] **品牌 / 依赖：** Bitloom / `bitloom-prelude`
- [x] **超子集：** 未写入本 epic NFR14 的更深 dialect/lower 仍 **NFR86**

**Status:** **closed — Story 113.3**（可宣称 FR180；更深 dialect/lower 仍 **NFR86**）。
