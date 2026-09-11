# NFR14 风险记录 — Epic 72 Phase 16 终局合同闸门（NFR55 升格 / 产品终局结项 / FR133）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 16 **NFR56–NFR59**；闸门 **FR133**；宣称 **FR140**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic64-phase15-nfr51-leftover-deepen.md`。  
> **前置：** Phase 12–15 FR94–132 已关闭；Correct Course `sprint-change-proposal-2026-09-11-phase16-nfr55-final-closeout.md` **approved**（2026-09-11）；Epic 72–78 已 seed（commit `dbe9b55`）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 72 后续故事 **72.2–72.4** 标为 `ready`，亦不得开工实现。**Epic 72 未关闭 / FR133 未验收前，Epic 73–78 不得标 `ready`。**

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR133 / Epic 72；NFR14、NFR56、NFR57、NFR58、NFR59；对照 FR94–132 / NFR40–55；FR140 宣称门面 |
| 记录日期 | 2026-09-11 |
| 状态 | closed — Story 72.4 勾选完成；Epic 72 关闭；Epic 73–78 仍 backlog（各 epic 自有 NFR14 后方可 ready） |

### Phase 12–15 关闭面 vs Phase 16 终局加深边界（NFR56 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **Phase 12 字面绿 MVP** | FR94–105 / Epic 40–47 关闭证据 | **仍有效；不得改写为失败** |
| **Phase 13 MVP→商业加深** | FR106–115 / Epic 48–56 关闭证据 | **仍有效；不得改写为失败** |
| **Phase 14 NFR47 未选加深升格** | FR116–123 / Epic 57–63 关闭证据 | **仍有效；不得改写为失败** |
| **Phase 15 NFR51 剩余升格** | FR124–132 / Epic 64–71 关闭证据 | **仍有效；不得改写为失败** |
| **Phase 16 产品终局结项 / NFR55 升格** | FR133–140 / Epic 72–78；NFR55 deferred→显式 FR | **本合同**（Correct Course 已批准 2026-09-11） |

**选定：在保留 Phase 12–15 关闭面的前提下，授权 NFR55 未选加深升格为产品终局结项。**  
本记录禁止把终局加深叙述偷换成「Phase 15 AC 未达标后的补救」。  
「产品终局结项」= 本批 FR133–140 关闭 + 诚实 NFR59 deferred；**不等于**冲 1.0 或 backlog 永久空。

### (a) 上游约束

- **Correct Course 批准（2026-09-11）：** `sprint-change-proposal-2026-09-11-phase16-nfr55-final-closeout.md` 批准 Phase 16 = Epic 72–78 · FR133–FR140 / NFR56–NFR59；不回滚 Phase 12–15。
- **PRD addendum：** 「Phase 16 产品终局结项 / NFR55 升格」合同戳待 **Story 72.2** 验收；「Tywaves GUI·IDE / 更多 IP FL / 全芯片 pad / 外部 CIRCT 门禁 / Parser 恢复 / IP 跨 crate」类宣称仅可引用 FR133–139 对应关闭证据（**FR140**）；**禁止**用 Phase 15 完成面冒充本批终局加深完成面。
- **合同落地：** README / deferred 诚实面 — **Story 72.3**；ARCHITECTURE-SPINE / AGENTS 指针 — **Story 72.4** / **NFR58**。Correct Course 已授权合同（**Story 72.2** 验收戳）。
- **FR134–139 加深范围摘要：**
  1. **FR134** — 真实上游 Tywaves GUI / IDE 插件深度（超出 FR125 T1–T4）；Epic 73
  2. **FR135** — 更多 IP 手写 FL（超选定 Gpio / FR126）；Epic 74
  3. **FR136** — 多外设 / 全芯片 pad 环（超出 FR128 `GpioSocPad` D1–D4）；Epic 75
  4. **FR137** — 完整外部 CIRCT 编译 / 仿真门禁（超出 FR129 C1–C4）；Epic 76；**须钉死外部 CIRCT 工具链与缺工具失败语义**
  5. **FR138** — 恢复废弃 Scala `Parser.parse`；**必须**再修订 **AD-27**；Epic 77
  6. **FR139** — VIP/SocPad 再细拆或跨 crate 搬迁（超出 FR131 `ip/` 协议拆）；Epic 78；**须钉死 crate 边界 / 迁移说明**
- **NFR56：** Phase 16 关闭不得改写 Phase 12 FR94–105、Phase 13 FR106–115、Phase 14 FR116–123 或 Phase 15 FR124–132「已关闭」。
- **NFR57：** Phase 16 各实现 epic 开工前独立 NFR14（本记录为 Epic 72 门）。
- **NFR58：** 触及 AD-27（FR138 **必须**再修订）、外部 CIRCT/工具链运维（FR137）、设计 crate 边界（FR139）时须先修订脊柱/文档/CI 合同再标实现 story ready。
- **NFR59：** 升格后的 FR 禁止静默扩大超出各 epic 风险记录钉死的子集；**不得因「终局」口号静默吞并**未列入本清单的 deferred（至少：自动 FSM 标签、第三方 LCOV GUI 一等、emit MemRead stub→完整生成、非 Cargo 全 monorepo 任意路径扫描、GHA formal-sby 镜像卫生跟踪）。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6），除非 FR133/FR139 另开合同明确跨 crate；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。
- **与 Phase 12–15：** FR94–132 关闭证据保留；**不得**作为 Phase 16 终局加深完成口径。
- **软序（非硬依赖）：** Epic **78 先于** Epic **74/75**（同触 `bitloom-prelude` `ip/`）；建议 `72 → 78 → {74, 75} ‖ {73, 76, 77}`。

### (b) 粗工期带

- **预计：** Epic 72 整体约 **0.75–2 人周**（72.1 本风险记录 ≤0.25 人周；72.2 Correct Course/PRD 验收对齐 0.25–0.5 人周；72.3 README/deferred 同步 0.25–0.75 人周；72.4 AD 指针收口 0.25–0.5 人周）。**Phase 16 全盘（Epic 73–78）为多年/多 epic 终局加深带**，不计入本 epic 人周，但必须在计划中显式承认。
- **置信度 / 假设：** 中（对本 epic 文档/合同故事为中–高；对加深总工期为低）。假设不回滚 FR94–132；假设 72.3–72.4 只改合同指针与诚实文档、不提前实现 FR134+。若提前开闸 73–78，维护叠加显著上修。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **72.2–72.4** 标 `ready` 或开工实现。
- **FR133 未验收 / Epic 72 未关闭前，不得将 Epic 73–78 标 `ready` 或开工实现。**
- **不得改写 Phase 12–15 FR94–132「已关闭」为失败**（NFR56）。
- **不得用 Phase 15 完成面冒充 NFR55 终局加深 / Phase 16 完成面**（FR140）。
- **不得因「终局」口号静默吞并 NFR59 未选子集**（NFR59）。
- **不得静默扩大超出本记录 / 各实现 epic NFR14 钉死的加深子集**（NFR59）。
- 不得在未引用/修订适用 **AD-27**（及外部 CIRCT 运维、crate 边界说明）时声称 FR138/137/139 已合法关闭（NFR58；指针 → 72.4；实质修订 → 77/76/78）。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR56 / NFR57 / NFR58 / NFR59** 共同责任人
- 备份 / 升级路径：撤回 Phase 16 / 缩回仅 Phase 15 口径须升级至产品 / Correct Course 批准人；AD 修订争议升级至架构（AD-28）维护者。

---

### FR134–139 → Epic 对照

| FR | Epic | 加深相对 Phase 12–15 关闭面 |
| --- | --- | --- |
| FR134 | 73 | 真实上游 Tywaves GUI/IDE vs FR125 T1–T4 |
| FR135 | 74 | 更多 IP 手写 FL vs FR126 选定 Gpio |
| FR136 | 75 | 全芯片 pad 环 vs FR128 GpioSocPad D1–D4 |
| FR137 | 76 | 外部 CIRCT 编译/仿真门禁 vs FR129 C1–C4 |
| FR138 | 77 | 恢复 Parser vs FR130 Style Guide（须 AD-27） |
| FR139 | 78 | VIP/SocPad 再拆或跨 crate vs FR131 `ip/` 协议拆 |

### 须同步的 AD / 工具链（NFR58 · 本记录登记）

| AD / 工具链 | Phase 15 现状 | Phase 16 终局加深目标 |
| --- | --- | --- |
| **AD-27** | Style Guide S1–S4；默认禁止 Parser | Epic 77 **必须**再修订以恢复 Parser（FR138） |
| **外部 CIRCT** | FR129 C1–C4 产品路径；完整外部编译/仿真门禁仍 deferred | **FR137** 钉死工具版本与缺工具失败语义（Epic 76） |
| **crate 边界** | FR131 `ip/` 协议模块拆；设计仍只依赖 `bitloom-prelude` | **FR139** VIP/SocPad 再拆或跨 crate；公开路径稳定或迁移说明（Epic 78） |
| Deferred 指针 | 现行完成标签 = Phase 12–15 | 区分 Phase 15 vs Phase 16（→ **72.3 / 72.4**） |

### Epic 72 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **72.1** | 本 NFR14 风险记录 + ATDD；钉死 NFR56 边界 / FR134–139 摘要 / 禁止项 | **本故事** |
| **72.2** | Correct Course + PRD/addendum Phase 16 验收（合同戳） | Gate：须本记录后才可 ready |
| **72.3** | 同步 README / deferred / 路线图指针（FR133 / FR140） | Gate：须本记录后才可 ready |
| **72.4** | AD 指针与 Epic 72 收口；勾选关闭条件 | Gate：须本记录后才可 ready；关闭后才可开 73–78 |

### 并行 / 维护叠加（Chipyard 式 · NFR57）

- Epic 73–78 **可并行**（硬依赖 Epic 72 关闭）；软序建议 **78 → 74/75**（同触 `ip/`）。
- 文档面叠加：addendum、README、`deferred-work.md`、ARCHITECTURE-SPINE 须同一「Phase 15 vs Phase 16」叙事；禁止混用。
- `ip/` 体积风险：Epic 78 跨 crate / 再拆 + Epic 74 FL + Epic 75 pad — 分 epic 验收。

### 引用

- AD-28 — 风险门禁（NFR14）
- PRD NFR14 / FR133；NFR56–NFR59；对照 FR94–132 / NFR40–55
- Correct Course：`sprint-change-proposal-2026-09-11-phase16-nfr55-final-closeout.md`（approved 2026-09-11）
- Addendum：`prd-rhdl-2026-08-19/addendum.md` — Phase 16（→ 72.2）
- 体例：`nfr14-risk-epic64-phase15-nfr51-leftover-deepen.md`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 72 关闭条件（Story 72.4 勾选）

- [x] **FR133 / Correct Course + PRD：** Phase 16 批准文案验收 — Story 72.2
- [x] **README / deferred：** Phase 15 vs Phase 16 终局加深区分 — Story 72.3
- [x] **AD 指针：** ARCHITECTURE-SPINE / AGENTS Phase 16 指针 — Story 72.4
- [x] **NFR56–59：** 边界与诚实义务写入本记录并保持
- [x] **禁止事项未触发：** 73–78 在 Epic 72 关闭前未标 ready
- [x] **品牌 / 依赖：** Bitloom / `bitloom-prelude`
- [x] **Epic 73–78：** 仍须各自 NFR14；未实现前不得宣称对应 FR 关闭

### Epic 72 关闭声明

**Phase 16 闸门已开（FR133 / FR140）：** Correct Course + PRD/addendum + README/deferred + 脊柱/AGENTS 指针齐备。Epic 73–78 **可以**离开永久冻结，但仍为 `backlog` 直至各 epic 自有 NFR14（NFR57）。Phase 12–15 关闭证据仍有效（NFR56）。未实现前不得宣称 FR134–139 关闭。
