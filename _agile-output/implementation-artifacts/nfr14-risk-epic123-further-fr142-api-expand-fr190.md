# NFR14 风险记录 — Epic 123 继续显式扩大 FR142 公开 API 表面（FR190）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28** / **AD-6**；Phase 23 **NFR88–NFR92**；实现面 **FR190**；表面权威 [`docs/public-api-1-0-surface.md`](../../docs/public-api-1-0-surface.md)（FR142 + FR183）。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic116-explicit-fr142-api-expand-fr183.md`；`nfr14-risk-epic122-further-firtool-product-pin-fr189.md`。  
> **前置：** Epic 118 / FR185 **closed**；Epic 116 / **FR183** firrtl interop expand **closed**（仍有效 · NFR88）；Epic 80 / **FR142** **closed**；Epic 81 / **FR143** SemVer **closed**。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **123.2–123.3** 标 `ready`。**不得**以 FR183 alone / FR142 alone / 「代码已有 pub」alone / 未改表面文档 勾选本 FR。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR190 / Epic 123；NFR14、NFR88、NFR90、NFR91；对照 FR183 / FR142 / FR143 / AD-6 |
| 记录日期 | 2026-09-14 |
| 状态 | **open / in-progress**（Story 123.1；Epic 123 未关闭；关闭勾选 → Story 123.3） |
| **选定** | 在保留 FR183 `bitloom-firrtl` emit/Parser 关闭面的前提下，**显式**再升入 in-surface：**FIRRTL 文本 `emit`/`import` + 往返谓词**与 **文档化 `check_*` 验收族**；更新 `docs/public-api-1-0-surface.md` + SemVer 诚实；**AD-6 不变**；禁止静默扩大 |

### Phase 17/22 关闭面 vs Epic 123（NFR88）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR142 / Epic 80** | 1.0 表面锁定 | **仍有效**；alone ≠ FR190 |
| **FR143 / Epic 81** | SemVer 政策 | **仍有效** |
| **FR183 / Epic 116** | firrtl emit/Parser 显式扩展 | **仍有效**；**alone ≠ FR190** |
| **Epic 123 / FR190** | 继续显式扩大表面（超 FR183） | **本实现 epic** |

### 拟增表面条目清单（123.2 必须落地）

| # | 拟升入 in-surface | 当前 | 明确不在本 epic |
| --- | --- | --- | --- |
| **S1** | `bitloom-firrtl` **FIRRTL 文本**：`emit`、`import`；往返谓词 `ports_roundtrip_ok`、`instance_graph_roundtrip_ok`（文档化维护者/工具链 interop） | FR183 未列入（仍属未承诺 `pub`） | 整 crate 任意内部模块；设计 crate 依赖 firrtl |
| **S2** | 文档化 **`check_*` 验收族**（与已 in-surface emit 面对齐）：至少 `check_idiomatic_chisel`、`check_idiomatic_chisel_fr111`/`fr122`、`check_chisel_style_guide_fr130`/`fr165`、`check_chisel_ecosystem_fr176`、`check_chisel_style_linter_fr181`、`check_chisel_style_guide_pack_fr188` | emit 已 in-surface；check 未显式升入 | 未文档化的其它 `check_*`；silent 全量 pub |
| **S3** | 表面文档 **FR190 / v1.x expand** 节 + SemVer：**additive → minor**；**AD-6** 仍 prelude-only | FR183 节存在 | 改 AD-6；升 `bitloom-lsp` / hir / builder / vlog 为 1.0-stable |

### 本批钉死（123.2 必须）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **表面文档（NFR90）** | Story **123.2** 修订 `docs/public-api-1-0-surface.md`（含 S1–S3）；`docs/fr190-*` | 只改代码不改表面文档 |
| **SemVer 诚实** | additive → minor 叙事；引用 FR143 | 静默扩；无文档宣称已扩 |
| **验收谓词** | ATDD 断言表面含 FR190 节与 S1–S2 条目；prelude 不依赖 firrtl | docs-only 无清单 |
| **与 FR183 边界** | FR183 = emit/Parser 表；FR190 = **文本 emit/import + check 族**再扩展 | FR183 alone 勾选 |

### 验收谓词 / 失败语义（123.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | 表面文档已列 S1–S2；`docs/fr190-*`；SemVer 诚实（NFR90） |
| **负向** | 声称扩展但表面无条目 → 失败；设计 crate 依赖 `bitloom-firrtl` → 失败（AD-6） |
| **禁止勾选** | FR183 alone；FR142 alone；「代码已有 pub」alone；未改表面文档；静默扩大 |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **123.1** | 本 NFR14 |
| **123.2** | 修订表面文档 + `docs/fr190-*` + ATDD（NFR90） |
| **123.3** | README/deferred 收口；勾选 Epic 123；未写入子集仍 **NFR91** |

### (a) 上游约束

- **Epic 118 / FR185 已关闭。**
- **FR183 / FR142 / FR143：** **仍有效**；alone ≠ FR190。
- **NFR90：** 触 FR142 表面须先修订表面文档再宣称。
- **NFR88 / NFR91：** 不得改写已关闭；不得静默超子集（lsp/hir/builder/vlog 1.0-stable 仍 **NFR91**）。
- **AD-6：** 设计 crate → **仅** `bitloom-prelude`；本 FR **不**改 AD-6。
- **品牌：** Bitloom；禁止 publish `rhdl` / `rhdl-bits`。
- **软序：** 与 Epic 122 可并行；Epic 122 若 upstream 阻塞 **不**阻断本 epic。

### (b) 粗工期带

- **预计：** Epic 123 **Low–Medium**；本 NFR14 ≤0.25 人周；123.2 约 0.5–1 人周。
- **置信度：** 高（条目已钉死；无 firtool 二进制依赖）。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **123.2–123.3** 标 ready。
- **不得**以 FR183 / FR142 alone 勾选 FR190。
- **不得**未改 `public-api-1-0-surface.md` 宣称已扩（NFR90）。
- **不得**静默扩大超出 S1–S2；**不得**把 `git push` 当 FR。
- 不得把本记录冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR88** / **NFR90** / **NFR91** / **AD-6** / FR142 表面
- 升级：缩回「仅 FR183 表」或升 lsp/hir 须 Correct Course / 产品批准。

### 引用

- AD-28；AD-6；FR190；对照 FR183 / FR142 / FR143；Phase 23 Correct Course approved 2026-09-14
- `docs/fr183-explicit-fr142-api-expand.md` — 继续扩表面曾归 NFR86 → 本 FR
- **NFR14-crates** ≠ 本门禁

---

### Epic 123 关闭条件（Story 123.3 勾选）

- [ ] **表面扩展 S1–S3 + 验收谓词：** Story 123.2
- [ ] **表面文档修订（NFR90）：** Story 123.2
- [ ] **docs/fr190-* + README/deferred：** Story 123.3
- [ ] **NFR88：** FR183/FR142/FR143 关闭面未改写
- [ ] **禁止事项未触发：** 未用 FR183 alone 勾选；未 silent expand
- [ ] **品牌 / 依赖：** Bitloom / `bitloom-prelude`；AD-6
- [ ] **超子集：** 未写入本 epic NFR14 的更深表面仍 **NFR91**
