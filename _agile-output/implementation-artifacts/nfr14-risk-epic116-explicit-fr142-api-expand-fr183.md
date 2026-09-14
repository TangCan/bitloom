# NFR14 风险记录 — Epic 116 显式扩大 FR142 公开 API 表面（FR183）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28** / **AD-6**；Phase 22 **NFR83–NFR87**；实现面 **FR183**；表面权威 [`docs/public-api-1-0-surface.md`](../../docs/public-api-1-0-surface.md)（FR142）。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic115-unpaired-firtool-product-pin-fr182.md`；Epic 80 / FR142 表面清单。  
> **前置：** Epic 111 / FR178 **closed**；Epic 80 / **FR142** 表面清单 **closed**（仍有效 · NFR83）；Phase 17–21「禁止静默扩大 FR142」纪律 **仍有效**（NFR83）。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **116.2–116.3** 标 `ready`。**不得**以「代码里已有 `pub`」alone / FR142 alone / 未改表面文档 勾选本 FR。**不得**静默扩大叙事。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR183 / Epic 116；NFR14、NFR83、NFR85、NFR86；对照 FR142 / FR143 / SemVer |
| 记录日期 | 2026-09-14 |
| 状态 | **open — Story 116.1**（in-progress）；Epic 116 未关闭 |
| **选定** | **显式**将下列条目从 FR142 **out-of-surface / out-of-promise** 升入 **in-surface**（更新 `docs/public-api-1-0-surface.md` + SemVer/发版诚实）；**禁止**未改文档即宣称已扩 |

### Phase 17–22 关闭面 vs Epic 116（NFR83）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR142 / Epic 80** | 1.0 公开 API 表面清单锁定 | **仍有效**；**alone ≠ FR183**（本 FR = **显式扩展**该清单） |
| **FR143 / Epic 81** | SemVer 1.0 政策（in-surface breaking → major；additive 可 minor） | **仍有效**；本批扩展须遵守 |
| **Phase 17–21 纪律** | 禁止静默扩大 FR142 | **仍有效**；本 FR 是合法升格路径 |
| **Epic 116 / FR183** | 显式扩大表面 + 文档/SemVer 诚实 | **本实现 epic** |

### 拟增表面条目清单（116.2 必须落地）

| # | 拟升入 in-surface | 当前（FR142） | 明确不在本 epic |
| --- | --- | --- | --- |
| **S1** | `bitloom-firrtl` **文档化** interop 表面子集：`emit_chisel`（及文档化 idiomatic/check 变体若已产品化）、`CHISEL_TARGET` / `FIRTOOL_TARGET`、`BitloomFirrtlParser` 产品路径（含 update-mainline 文档化入口） | 现为 **out-of-surface**（`bitloom-firrtl` 可发布但非 1.0 SemVer 承诺） | 未文档化的 `pub`；整 crate 任意内部模块全量承诺 |
| **S2** | 表面文档增加 **FR183 / v1.x expand** 节：列出 S1；注明设计 crate **仍仅**依赖 `bitloom-prelude`（**AD-6 不变**）；`bitloom-firrtl` 为 **维护者/工具链 interop** in-surface，**不是**设计依赖 | FR142 无 FR183 节 | 把 `bitloom-firrtl` 塞进设计 crate 依赖图 |
| **S3** | SemVer 诚实：S1 为 **additive** → 受影响已发布 crate 按 FR143 走 **minor**（或文档化「表面文档修订 + changelog」谓词）；**不得**在未改 `public-api-1-0-surface.md` 时宣称已扩 | — | 静默扩；major 无 breaking 证据 |

**明确不在本 epic（仍 NFR86 / 另开合同）：** 将 `bitloom-hir` / `bitloom-builder` / `bitloom-vlog` 升为 1.0-stable；将 `bitloom-lsp` 升入 in-surface；任意未列入上表的 `pub` 晋升；改 AD-6。

### 本批钉死（116.2 必须）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **表面文档（NFR85）** | Story **116.2** 修订 [`docs/public-api-1-0-surface.md`](../../docs/public-api-1-0-surface.md)（或显式后继补丁文档并互链）；含 S1–S3 | 只改代码不改表面文档；另立未互链私文档冒充 |
| **SemVer / 发版诚实** | ATDD/文档可验证：additive 扩展叙事；changelog / 版本谓词可读；引用 FR143 | 「未发版也算已扩表面」；暗示 breaking 却标 minor 无证据 |
| **验收谓词** | `docs/fr183-*`；表面清单含 FR183 节；ATDD 断言 S1–S3 与禁止静默扩大 | docs-only 无表面文件修订；silent promotion |
| **与 FR142 边界** | FR142 = 原 1.0 锁定面；FR183 = **显式**增量；不得改写 FR142「已关闭」为失败 | FR142 alone 勾选 FR183 |

### 验收谓词 / 失败语义（116.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | 表面文档已列 S1；FR183 产品文档；SemVer 诚实可读（NFR85） |
| **负向** | 声称扩展但表面文档无条目 → 验收失败；设计 crate 依赖 `bitloom-firrtl` → 失败（AD-6） |
| **禁止勾选** | FR142 alone；「代码已有 pub」alone；未改表面文档；静默扩大叙事；docs-only 无清单 |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **116.1** | 本 NFR14（拟增清单 + SemVer/docs 计划） |
| **116.2** | 修订表面文档 + `docs/fr183-*` + ATDD；SemVer 诚实（NFR85） |
| **116.3** | README/deferred 收口；勾选 Epic 116；未写入子集仍 **NFR86** |

### (a) 上游约束

- **Epic 111 / FR178 已关闭。**
- **FR142 / FR143：** **仍有效**；alone ≠ FR183；扩展须遵守 SemVer 政策。
- **NFR85：** 触 FR142 表面须先修订表面文档再宣称。
- **NFR83 / NFR86：** 不得改写已关闭；不得静默超子集。
- **AD-6：** 设计 crate → **仅** `bitloom-prelude`；本 FR **不**改 AD-6。
- **品牌：** Bitloom / `bitloom-*`；禁止 publish `rhdl` / `rhdl-bits`。

### (b) 粗工期带

- **预计：** Epic 116 **Low–Medium**；本 NFR14 ≤0.25 人周；116.2 约 0.5–1 人周（表面文档 + ATDD + 诚实谓词）。
- **置信度：** 高（扩展条目已钉死；无工具链二进制升钉）。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **116.2–116.3** 标 ready。
- **不得**以 FR142 alone / 「已有 `pub`」alone 勾选 FR183。
- **不得**未修订 `docs/public-api-1-0-surface.md`（或互链后继）就宣称表面已扩。
- **不得**把 `bitloom-firrtl` 升为设计 crate 依赖（破坏 AD-6）。
- **不得**静默扩大超出 S1–S3 的条目；**不得**把 `git push` 当 FR。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR83** / **NFR85** / **NFR86** / FR142 表面
- 升级：扩大超出 S1–S3（如 lsp / hir 1.0-stable）须 Correct Course / 产品批准 → **NFR86**。

### 引用

- AD-28；AD-6；FR183；对照 FR142 / FR143；`docs/public-api-1-0-surface.md`；`docs/semver-1-0-policy.md`；Phase 22 Correct Course approved 2026-09-12
- **NFR14-crates** ≠ 本门禁

---

### Epic 116 关闭条件（Story 116.3 勾选）

- [ ] **显式表面扩展 + 验收谓词：** Story 116.2
- [ ] **表面文档修订（NFR85）：** Story 116.2
- [ ] **docs/fr183-* + README/deferred：** Story 116.3
- [ ] **NFR83：** FR142/FR143 / Phase 17–21 关闭面未改写
- [ ] **禁止事项未触发：** 未静默扩大；未破坏 AD-6
- [ ] **品牌 / 依赖：** Bitloom / `bitloom-prelude` 设计边界
- [ ] **超子集：** 未写入本 epic NFR14 的更深 API 扩展仍 **NFR86**
