# NFR14 风险记录 — Epic 79 Phase 17 稳定门合同闸门（公开 API 稳定门 / Bitloom 1.0 / FR141）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 17 **NFR60–NFR63**；闸门 **FR141**；宣称 **FR147**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic72-phase16-nfr55-final-closeout.md`。  
> **前置：** Phase 12–16 FR94–140 已关闭；Correct Course `sprint-change-proposal-2026-09-11-phase17-api-stability-1-0.md` **approved**（2026-09-11）；Epic 79–83 已 seed（commit `03eb42d`）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 79 后续故事 **79.2–79.4** 标为 `ready`，亦不得开工实现。**Epic 79 未关闭 / FR141 未验收前，Epic 80–83 不得标 `ready`。**

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR141 / Epic 79；NFR14、NFR60、NFR61、NFR62、NFR63；对照 FR94–140 / NFR40–59；FR147 宣称门面 |
| 记录日期 | 2026-09-11 |
| 状态 | closed — Story 79.4 勾选完成；Epic 79 关闭；Phase 17 闸门已开；Epic 80–83 仍 backlog（各 epic 自有 NFR14 后方可 ready）；发版属 Epic 83 |

### Phase 12–16 关闭面 vs Phase 17 稳定门边界（NFR60 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **Phase 12–15** | FR94–132 / Epic 40–71 关闭证据 | **仍有效；不得改写为失败** |
| **Phase 16 产品终局结项** | FR133–140 / Epic 72–78 关闭证据 | **仍有效；不得改写为失败** |
| **Phase 17 公开 API 稳定门 / Bitloom 1.0** | FR141–147 / Epic 79–83；NFR15 0.x → 表面 1.0 | **本合同**（Correct Course 已批准 2026-09-11） |

**选定：在保留 Phase 12–16 关闭面的前提下，授权公开 API 稳定门合同以合法发布 SemVer 1.0.0。**  
本记录禁止把 1.0 叙述偷换成「Phase 16 AC 未达标后的补救」或「Phase 16 终局 alone ⇒ 1.0」。  
**1.0 口径** = 对 FR142 钉死表面承诺 major 稳定 + FR141–146 关闭 + 诚实 NFR59 deferred（**NFR63**）；**不等于**清空 NFR59 或 backlog 永久空。

### 批准默认（Q1–Q5 · Correct Course）

| # | 决议 |
| --- | --- |
| **Q1** | `bitloom-sim` **纳入** 1.0 表面 |
| **Q2** | `bitloom-hir` / `bitloom-builder` / `bitloom-vlog` 可继续 publish，**不**进 1.0 稳定承诺 |
| **Q3** | Epic 82（FR145）可按 Epic 80 清单 **skip**（无阻塞 breaking） |
| **Q4** | **不以**先消化 NFR59 为 1.0 前提 |
| **Q5** | **MSRV** 保持现行；上调另开并写入 1.0 政策 |

### (a) 上游约束

- **Correct Course 批准（2026-09-11）：** `sprint-change-proposal-2026-09-11-phase17-api-stability-1-0.md` 批准 Phase 17 = Epic 79–83 · FR141–FR147 / NFR60–NFR63；不回滚 Phase 12–16。
- **PRD addendum：** 「Phase 17 公开 API 稳定门 / Bitloom 1.0」合同戳待 **Story 79.2** 验收；「1.0 / 公开 API 稳定」类宣称仅可引用 FR141–146 对应关闭证据（**FR147**）；**禁止**用 Phase 16 终局 alone 冒充 1.0。
- **合同落地：** README / deferred 诚实面 — **Story 79.3**；ARCHITECTURE-SPINE / AGENTS 指针 — **Story 79.4** / **NFR62**。Correct Course 已授权合同（**Story 79.2** 验收戳）。
- **FR142–146 范围摘要：**
  1. **FR142** — 公开 API 表面清单钉死（建议 `docs/public-api-1-0-surface.md`）；默认 in：`bitloom` CLI、`bitloom-prelude`、`bitloom-macro`、`bitloom-sim`；默认 out-of-promise：hir/builder/vlog、LSP；Epic 80
  2. **FR143** — SemVer 1.0 政策（升格结项期 0.x 对表面的约束）；Epic 81
  3. **FR144** — 破坏性变更 CI（`cargo-semver-checks` 或等价）；Epic 81
  4. **FR145** — 预 1.0 表面卫生（可选；可 skip）；Epic 82
  5. **FR146** — 发布 Bitloom **1.0.0** + tag + CHANGELOG；Epic 83；**不得**在本合同批准瞬间 publish
- **NFR60：** Phase 17 关闭不得改写 Phase 12–16 FR94–140「已关闭」。
- **NFR61：** Phase 17 各实现 epic 开工前独立 NFR14（本记录为 Epic 79 门）。
- **NFR62：** 触及 publish 表面、SemVer 政策、CI 门禁时须先修订文档/脊柱/CI 合同。
- **NFR63：** 1.0 **不得**静默吞并 **NFR59**；更深产品加深仍须新合同。
- **NFR15 / 0.x：** 结项期曾停 0.x；本批 **FR143** 授权对钉死表面升 1.0；关闭 sprint ≠ 自动 major。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。
- **与 Phase 12–16：** FR94–140 关闭证据保留；**不得**作为 1.0 / 公开 API 稳定完成口径。
- **软序（非硬依赖）：** `79 → 80 → 81 ‖ 82 → 83`（82 可与 81 软并行；无阻塞时可 skip 82）。

### (b) 粗工期带

- **预计：** Epic 79 整体约 **0.75–2 人周**（79.1 本风险记录 ≤0.25 人周；79.2 Correct Course/PRD 验收对齐 0.25–0.5 人周；79.3 README/deferred 同步 0.25–0.75 人周；79.4 AD 指针收口 0.25–0.5 人周）。**Phase 17 全盘（Epic 80–83）为表面钉死 + 政策/CI + 可选卫生 + 发版带**，不计入本 epic 人周，但必须在计划中显式承认。
- **置信度 / 假设：** 中（对本 epic 文档/合同故事为中–高；对 semver CI 误报与 crates.io 发版为中）。假设不回滚 FR94–140；假设 79.3–79.4 只改合同指针与诚实文档、不提前钉死表面或 publish。若提前开闸 80–83 或静默 publish，维护叠加显著上修。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **79.2–79.4** 标 `ready` 或开工实现。
- **FR141 未验收 / Epic 79 未关闭前，不得将 Epic 80–83 标 `ready` 或开工实现。**
- **不得改写 Phase 12–16 FR94–140「已关闭」为失败**（NFR60）。
- **不得用 Phase 16 终局 alone 冒充 1.0 / 公开 API 稳定**（FR147）。
- **不得因「1.0」口号静默吞并 NFR59**（NFR63）。
- **不得静默扩大超出本记录 / Epic 80 清单钉死的表面承诺**（NFR63 / FR142）。
- 不得在未成文政策 / CI 合同时声称 FR143/FR144 已合法关闭（NFR62；指针 → 79.4；实质 → 81）。
- **不得在 Epic 83 / FR146 之前执行 `cargo publish` 升 1.0.0**。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR60 / NFR61 / NFR62 / NFR63** 共同责任人
- 备份 / 升级路径：撤回 Phase 17 / 缩回仅 0.x 口径须升级至产品 / Correct Course 批准人；政策/CI/发版争议升级至架构（AD-28）维护者。

---

### FR142–146 → Epic 对照

| FR | Epic | 相对 Phase 16 |
| --- | --- | --- |
| FR142 | 80 | 钉死 1.0 公开表面（≠ 终局加深） |
| FR143 | 81 | SemVer 1.0 政策（升格 NFR15 0.x） |
| FR144 | 81 | 破坏性变更 CI 门禁 |
| FR145 | 82 | 预 1.0 卫生（可选 / skip） |
| FR146 | 83 | 发版 1.0.0 |

### 须同步的政策 / CI / publish（NFR62 · 本记录登记）

| 项 | 现状 | Phase 17 目标 |
| --- | --- | --- |
| **表面清单** | 未钉死；多 crate `publish = true` | **FR142** 成文 in/out（Epic 80） |
| **SemVer 政策** | `docs/semver-0x-policy.md`（结项停 0.x） | **FR143** 1.0 政策（Epic 81） |
| **semver CI** | 无表面破坏性变更 required job | **FR144**（Epic 81） |
| **发版** | 0.x | **FR146** 1.0.0 + tag（Epic 83） |
| Deferred 指针 | 现行完成标签 = Phase 12–16 | 区分 Phase 16 vs Phase 17（→ **79.3 / 79.4**） |

### Epic 79 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **79.1** | 本 NFR14 风险记录 + ATDD；钉死 NFR60 边界 / FR142–146 / Q1–Q5 / 禁止项 | **本故事** |
| **79.2** | Correct Course + PRD/addendum Phase 17 验收（合同戳） | Gate：须本记录后才可 ready |
| **79.3** | 同步 README / deferred / 路线图指针（FR141 / FR147） | Gate：须本记录后才可 ready |
| **79.4** | AD 指针与 Epic 79 收口；勾选关闭条件 | Gate：须本记录后才可 ready；关闭后才可开 80–83 |

### 并行 / 维护叠加（Chipyard 式 · NFR61）

- Epic 80–83 **可并行规划**但硬依赖 Epic 79 关闭；软序 `80 → 81 ‖ 82 → 83`。
- 文档面叠加：addendum、README、`deferred-work.md`、ARCHITECTURE-SPINE、semver 政策须同一「Phase 16 终局 vs Phase 17 1.0」叙事；禁止混用。
- 发版风险：Epic 83 触及 crates.io / tag；须 NFR14 + dry-run 策略。

### 引用

- AD-28 — 风险门禁（NFR14）
- PRD NFR14 / FR141；NFR60–NFR63；对照 FR94–140 / NFR40–59
- Correct Course：`sprint-change-proposal-2026-09-11-phase17-api-stability-1-0.md`（approved 2026-09-11）
- Addendum：`prd-rhdl-2026-08-19/addendum.md` — Phase 17（→ 79.2）
- 体例：`nfr14-risk-epic72-phase16-nfr55-final-closeout.md`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 79 关闭条件（Story 79.4 勾选）

- [x] **FR141 / Correct Course + PRD：** Phase 17 批准文案验收 — Story 79.2
- [x] **README / deferred：** Phase 16 vs Phase 17 稳定门区分 — Story 79.3
- [x] **AD 指针：** ARCHITECTURE-SPINE / AGENTS Phase 17 指针 — Story 79.4
- [x] **NFR60–63：** 边界与诚实义务写入本记录并保持
- [x] **禁止事项未触发：** 80–83 在 Epic 79 关闭前未标 ready
- [x] **品牌 / 依赖：** Bitloom / `bitloom-prelude`
- [x] **Epic 80–83：** 仍须各自 NFR14；未实现前不得宣称对应 FR 关闭；发版属 Epic 83

**Phase 17 闸门已开（Story 79.4）：** Epic 80–83 可在各自 NFR14 后标 ready；软序 `80 → 81 ‖ 82 → 83`；**不得**宣称 FR142–146 已关闭直至对应 epic 关闭；**不得**静默吞并 NFR59（NFR63）。