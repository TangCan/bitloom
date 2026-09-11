# NFR14 风险记录 — Epic 84 Phase 18 合同闸门（CLI / 依赖 crate crates.io 可发布 / FR148）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 18 **NFR64–NFR67**；闸门 **FR148**；宣称须引 **FR148–153**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic79-phase17-api-stability-1-0.md`。  
> **前置：** Phase 17 FR141–147 / Epic 79–83 **closed**；库 crate **1.0.0** 已在 crates.io；FR146 清单仍记 CLI follow-up；Correct Course `sprint-change-proposal-2026-09-11-phase18-cli-crates-io-publish.md` **approved**（2026-09-11；`correctCoursePhase18Approved: 2026-09-11`）；Epic 84–86 已 seed（commit `58a89e3`）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 84 后续故事 **84.2–84.4** 标为 `ready`，亦不得开工实现。**Epic 84 未关闭 / FR148 未验收前，Epic 85–86 不得标 `ready`。**

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR148 / Epic 84；NFR14、NFR64、NFR65、NFR66、NFR67；对照 FR141–147 / NFR60–63；FR149–153 实现面 |
| 记录日期 | 2026-09-11 |
| 状态 | open — Story 84.1；Epic 84 未关闭；84.2–84.4 待办；Epic 85–86 仍 backlog（Epic 84 关闭后方可 ready） |
| **选定** | 在保留 Phase 17 关闭面的前提下，授权 Phase 18「CLI / 依赖 crate crates.io 可发布」合同闸门（FR148）；实现属 Epic 85–86 |

### Phase 17 关闭面 vs Phase 18 CLI 可发布边界（NFR64 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **Phase 12–16** | FR94–140 / Epic 40–78 关闭证据 | **仍有效；不得改写为失败** |
| **Phase 17 公开 API 稳定门 / Bitloom 1.0** | FR141–147 / Epic 79–83；库 crate **1.0.0** 已上 crates.io | **仍有效；不得改写「已关闭」**（NFR64） |
| **Phase 18 CLI / 依赖 crate crates.io 可发布** | FR148–153 / Epic 84–86；NFR64–NFR67 | **本合同**（Correct Course 已批准 2026-09-11） |

**选定：在保留 Phase 17 关闭面的前提下，授权 CLI 可发布合同以合法收口 FR146 明示的 CLI follow-up。**  
本记录禁止把 CLI 上架叙述偷换成「Phase 17 / 1.0 失败后的补救」或「库 1.0 alone ⇒ CLI 可发布已授权」。  
**CLI crates.io 口径** = FR148–153 关闭 + 诚实 NFR59 deferred（**NFR67**）；**不等于**清空 NFR59、改写 FR141–147、或扩大 FR142 表面。

### 批准默认（Q1–Q5 · Correct Course）

| # | 决议 |
| --- | --- |
| **Q1** | rename `rhdl-firrtl` → 可发布 **`bitloom-firrtl`**（AD-2；禁止以 `rhdl-*` 为 crates.io 名） |
| **Q2** | rename `rhdl-viz` → 可发布 **`bitloom-viz`**（同 AD-2） |
| **Q3** | FR152 **(b)** — `bitloom-lsp` 默认可保持 `publish=false`，**必须**不挡 `bitloom` 打包；(a) 另开合同 |
| **Q4** | **不以**先消化 NFR59 为 CLI 上架前提（NFR67） |
| **Q5** | **MSRV** 保持现行；新包与工作区 **1.0.0** 对齐（除非故事另决）；**FR148** 须 Correct Course + PRD 戳后方可开实现 epic |

### (a) 上游约束

- **Correct Course 批准（2026-09-11）：** `sprint-change-proposal-2026-09-11-phase18-cli-crates-io-publish.md` 批准 Phase 18 = Epic 84–86 · FR148–FR153 / NFR64–NFR67；不回滚 Phase 17。
- **PRD addendum：** 「Phase 18 CLI / 依赖 crate crates.io 可发布」合同戳待 **Story 84.2** 验收；「CLI 已上架 / `cargo install bitloom`」类宣称仅可引用 FR148–153 对应关闭证据；**禁止**用库 1.0 alone 冒充 CLI 可发布已授权。
- **合同落地：** README / deferred 诚实面 — **Story 84.3**；ARCHITECTURE-SPINE / AGENTS 指针 — **Story 84.4** / **NFR66**。Correct Course 已授权合同（**Story 84.2** 验收戳）。
- **FR149–153 范围摘要：**
  1. **FR149** — `rhdl-firrtl` → 可发布 **`bitloom-firrtl`**；Epic 85
  2. **FR150** — `rhdl-viz` → 可发布 **`bitloom-viz`**；Epic 85
  3. **FR151** — `bitloom` CLI 具备 version 依赖并 `cargo publish` 成功；Epic 85
  4. **FR152** — `bitloom-lsp` 发布策略 **(b)** 默认（不挡打包）；Epic 85
  5. **FR153** — 发版后诚实与 SemVer 跟进；Epic 86
- **NFR64：** Phase 18 关闭不得改写 Phase 17 FR141–147「已关闭」。
- **NFR65：** Phase 18 各实现 epic 开工前独立 NFR14（本记录为 Epic 84 门）。
- **NFR66：** 对外包名 **`bitloom-*`**；禁止 publish **`rhdl` / `rhdl-bits`**；目录可暂留 `rhdl-*`（AD-2）。
- **NFR67：** CLI 上架 **不得**静默吞并 **NFR59**；亦不得把 LSP 一等深化冒充本批必做（除非显式勾选 FR152(a)）。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。
- **与 Phase 17：** FR141–147 关闭证据保留；库 crate 1.0.0 上架仍有效；**不得**作为 CLI crates.io 完成口径。
- **软序（非硬依赖）：** `84 → 85 → 86`；85 内 soft：firrtl → viz → lsp 策略 → CLI publish。

### (b) 粗工期带

- **预计：** Epic 84 整体约 **0.75–2 人周**（84.1 本风险记录 ≤0.25 人周；84.2 Correct Course/PRD 验收对齐 0.25–0.5 人周；84.3 README/deferred 同步 0.25–0.75 人周；84.4 AD 指针收口 0.25–0.5 人周）。**Phase 18 全盘（Epic 85–86）为 rename/publish + 诚实面**，不计入本 epic 人周，但必须在计划中显式承认。
- **置信度 / 假设：** 中（对本 epic 文档/合同故事为中–高；对 rename 迁移与 crates.io publish 为中）。假设不回滚 FR141–147；假设 84.3–84.4 只改合同指针与诚实文档、不提前 rename/publish。若提前开闸 85–86 或静默 publish，维护叠加显著上修。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **84.2–84.4** 标 `ready` 或开工实现。
- **FR148 未验收 / Epic 84 未关闭前，不得将 Epic 85–86 标 `ready` 或开工实现。**
- **不得改写 Phase 17 FR141–147「已关闭」为失败**（NFR64）。
- **不得用库 crate 1.0 alone 冒充 CLI 可发布合同已授权。**
- **不得 publish `rhdl` / `rhdl-bits`**（NFR66 / AD-2）；对外须 `bitloom-*`。
- **不得因「CLI 上架」口号静默吞并 NFR59**（NFR67）。
- **不得把 LSP 产品加深冒充本批必做**（NFR67；默认 FR152(b)）。
- 不得静默扩大超出 FR142 钉死的表面承诺。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR64 / NFR65 / NFR66 / NFR67** 共同责任人
- 备份 / 升级路径：撤回 Phase 18 / 缩回仅库 1.0 口径须升级至产品 / Correct Course 批准人；AD-2 / 发布名争议升级至架构（AD-28）维护者。

---

### FR149–153 → Epic 对照

| FR | Epic | 相对 Phase 17 |
| --- | --- | --- |
| FR149 | 85 | bitloom-firrtl 可发布（≠ 改写 1.0 库关闭） |
| FR150 | 85 | bitloom-viz 可发布 |
| FR151 | 85 | bitloom CLI 1.0.0 上架 |
| FR152 | 85 | bitloom-lsp 策略 (b) |
| FR153 | 86 | 发版后诚实 / SemVer 跟进 |

### Epic 84 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **84.1** | 本 NFR14 风险记录 + ATDD；钉死 NFR64 边界 / FR149–153 / Q1–Q5 / 禁止项 | **本故事** |
| **84.2** | Correct Course + PRD/addendum Phase 18 验收（合同戳） | Gate：须本记录后才可 ready |
| **84.3** | 同步 README / deferred / 路线图指针（FR148） | Gate：须本记录后才可 ready |
| **84.4** | AD 指针与 Epic 84 收口；勾选关闭条件 | Gate：须本记录后才可 ready；关闭后才可开 85–86 |

### 并行 / 维护叠加（Chipyard 式 · NFR65）

- Epic 85–86 **可并行规划**但硬依赖 Epic 84 关闭；软序 `84 → 85 → 86`。
- 文档面叠加：addendum、README、`deferred-work.md`、ARCHITECTURE-SPINE、fr146 清单须同一「Phase 17 库 1.0 vs Phase 18 CLI」叙事；禁止混用。
- Publish 风险：Epic 85 触及 rename + crates.io；须各自 NFR14 + dry-run 策略。

### 引用

- AD-28 — 风险门禁（NFR14）；AD-2 — 对外 `bitloom-*`
- PRD NFR14 / FR148；NFR64–NFR67；对照 FR141–147 / NFR60–63
- Correct Course：`sprint-change-proposal-2026-09-11-phase18-cli-crates-io-publish.md`（approved 2026-09-11）
- Addendum：`prd-rhdl-2026-08-19/addendum.md` — Phase 18（→ 84.2）
- 体例：`nfr14-risk-epic79-phase17-api-stability-1-0.md`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 84 关闭条件（Story 84.4 勾选）

- [ ] **FR148 / Correct Course + PRD：** Phase 18 批准文案验收 — Story 84.2
- [ ] **README / deferred：** Phase 17 vs Phase 18 CLI 合同区分 — Story 84.3
- [ ] **AD 指针：** ARCHITECTURE-SPINE / AGENTS Phase 18 指针 — Story 84.4
- [ ] **NFR64–67：** 边界与诚实义务写入本记录并保持
- [ ] **禁止事项未触发：** 85–86 在 Epic 84 关闭前未标 ready
- [ ] **品牌 / 依赖：** Bitloom / `bitloom-prelude`；禁 publish `rhdl`/`rhdl-bits`
- [ ] **Epic 85–86：** 仍须各自 NFR14；未实现前不得宣称对应 FR 关闭

**Epic 84 未关闭：** 不得宣称 FR148 已关闭；不得将 Epic 85–86 标 ready；**不得**静默吞并 NFR59（NFR67）。
