# NFR14 风险记录 — Epic 122 继续 firtool 产品钉升钉（FR189）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28** / **AD-9**；Phase 23 **NFR88–NFR92**；实现面 **FR189**。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic115-unpaired-firtool-product-pin-fr182.md`；`nfr14-risk-epic119-unbounded-circt-tip-fr186.md`。  
> **前置：** Epic 118 / FR185 **closed**；Epic 115 / **FR182** unpaired 产品钉 **firtool-1.159.0** **closed**（仍有效 · NFR88）；Epic 119 / **FR186** live tip **closed**（仍有效 · 非产品钉）；Epic 112 / **FR179** floating-track **closed**；Epic 106 / **FR173** 配对 **1.158.0↔7.15.0** **closed**。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **122.2–122.3** 标 `ready`。**不得**以 FR182 alone / FR186 alone / FR179 alone / FR173 alone 勾选本 FR。**不得**未修订 AD-9 就升产品钉。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR189 / Epic 122；NFR14、NFR88、NFR90、NFR91；对照 FR182 / FR186 / FR179 / FR173 / AD-9 |
| 记录日期 | 2026-09-14 |
| 状态 | **open / in-progress / parked**（Story 122.1 done；Epic 122 **blocked-upstream** — Correct Course **A** approved 2026-09-14；关闭勾选 → Story 122.3 after tip >1.159.0） |
| **选定** | 在保留 FR182 **firtool-1.159.0** unpaired 产品钉关闭面的前提下，授权将 **AD-9 默认产品钉**再升至 **严格大于 1.159.0** 的已发布 `firtool-*` tag（semver）；Story **122.2 开工时**须从 GitHub `llvm/circt` releases 解析并**钉死具体版本号**写入 docs/本记录；Chisel 默认仍 **7.15.0**，除非该 firtool 有上游正式配对（若有则按配对升钉；若无则延续 *unpaired product-pin* 例外）；触及 **AD-9** 须按 **NFR90** 先修订；禁止把 FR186 live tip / FR179 floating-track 冒充默认产品钉 |

### Phase 22 / FR182 关闭面 vs Epic 122（NFR88）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR173 / Epic 106** | 配对升钉 **1.158.0 ↔ 7.15.0** | **仍有效**；alone ≠ FR189 |
| **FR179 / Epic 112** | floating-track 可选渠道 **1.159.0** | **仍有效**；alone ≠ FR189 |
| **FR182 / Epic 115** | unpaired **产品钉** **1.159.0** | **仍有效**；**alone ≠ FR189** |
| **FR186 / Epic 119** | live tip 渠道（非产品钉） | **仍有效**；alone ≠ FR189 |
| **Epic 122 / FR189** | 继续产品钉升钉（超 1.159.0） | **本实现 epic** |

### 本批钉死（122.2 必须）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **目标产品钉** | AD-9 默认产品 firtool 升至 **>1.159.0** 的已发布 tag（122.2 解析后写入具体 `firtool-X.Y.Z`）；文档诚实声明配对或 *unpaired product-pin*；`firtool ensure` / sha256 路径可用 | 停留在 1.159.0；仅改 FR186/FR179 可选渠道；假装升钉 |
| **无新版本时** | 若 122.2 开工时 GitHub **仍无** `firtool-*` >1.159.0：**不得** silent-Ok 宣称 FR189；须非零可读失败或 Correct Course 改目标 | 用 live tip 瞬时 HEAD 冒充产品钉 |
| **验收谓词** | `just` / CI / ATDD：产品钉版本可读为钉死的 **X.Y.Z > 1.159.0**；缺工具 / 错版本 / FORCE_MISSING → **非零可读** | `continue-on-error` |
| **AD 修订（NFR90）** | Story **122.2 ready 前**修订 **AD-9**：默认钉 = 新版本；配对或延续 unpaired 例外；≠ FR182 alone；≠ FR186/FR179 渠道 alone | 无 AD-9 修订静默升钉 |
| **与 FR182 / FR186 边界** | FR182 = 产品钉 **1.159.0**；FR186 = live tip **非产品钉**；FR189 = **默认产品钉再升** | FR182/186/179 alone 勾选 |

### 验收谓词 / 失败语义（122.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | 产品钉 **>1.159.0** 验收通过；`docs/fr189-*`；AD-9 已修订（NFR90） |
| **负向** | 无可用新版本 / 缺工具 / 错版本 / FORCE_MISSING → 可读非零 |
| **禁止勾选** | FR182 alone；FR186 alone；FR179 alone；FR173 alone；未修订 AD-9；仅 docs；silent-Ok |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **122.1** | 本 NFR14（形状：>1.159.0；具体 X.Y.Z → 122.2） |
| **122.2** | 解析并钉死具体版本 + 实现 + ATDD/CI；AD-9 先入脊柱（NFR90）；`docs/fr189-*` |
| **122.3** | README/deferred/spine 收口；勾选 Epic 122；未写入子集仍 **NFR91** |

### (a) 上游约束

- **Epic 118 / FR185 已关闭。**
- **FR182 / FR186 / FR179 / FR173：** **仍有效**；alone ≠ FR189。
- **NFR90：** 产品钉再升须先修订 AD-9。
- **NFR88 / NFR91：** 不得改写已关闭；不得静默超子集（更高升钉 / 恢复仅配对纪律超本批仍 **NFR91**）。
- **品牌：** Bitloom；firtool 运行时不得进入设计 crate（AD-6 / `bitloom-prelude`）。
- **软序：** 与 Epic 119（AD-9）建议串行 — Epic 119 **已关闭**，本 epic 可合法开工。

### (b) 粗工期带

- **预计：** Epic 122 **Medium–High**；本 NFR14 ≤0.25 人周；122.2 约 1–2 人周（依赖上游是否已发 >1.159.0）。
- **置信度：** 中（2026-09-14 观测：公开 releases 最新仍为 **firtool-1.159.0**；122.2 可能阻塞于上游发布）。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **122.2–122.3** 标 ready。
- **不得**以 FR182 / FR186 / FR179 / FR173 alone 勾选 FR189。
- **不得**未修订 AD-9 就升产品钉（NFR90）。
- **不得**把 FR186 live tip / FR179 floating-track 冒充默认产品钉。
- **不得**在无 >1.159.0 发布时 silent 宣称已升钉。
- **不得**静默扩大 FR142；**不得**把 `git push` 当 FR。
- 不得把本记录冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR88** / **NFR90** / **NFR91** / **AD-9**
- 升级：缩回「仅 1.159.0」或改配对策略须 Correct Course / 产品批准。

### 引用

- AD-28；AD-9；FR189；对照 FR182 / FR186 / FR179 / FR173；Phase 23 Correct Course approved 2026-09-14
- `docs/fr182-unpaired-firtool-product-pin.md` — 更高产品钉曾归 NFR86 → 本 FR
- **NFR14-crates** ≠ 本门禁

---

### Epic 122 关闭条件（Story 122.3 勾选）

- [ ] **产品钉 >1.159.0 通道 + 验收谓词：** Story 122.2
- [ ] **AD-9 修订（NFR90）：** Story 122.2 ready 前落地
- [ ] **docs/fr189-* + README/deferred：** Story 122.3
- [ ] **NFR88：** FR182/FR186/FR179/FR173 关闭面未改写
- [ ] **禁止事项未触发：** 未用 FR182/FR186 alone 勾选；未 silent-Ok
- [ ] **品牌 / 依赖：** Bitloom / `bitloom-prelude`
- [ ] **超子集：** 未写入本 epic NFR14 的更高升钉仍 **NFR91**
