# NFR14 风险记录 — Epic 85 从 crates.io 安装 Bitloom CLI（FR149–FR152）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 18 **NFR64–NFR67**；实现面 **FR149–FR152**；宣称须引对应 FR。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic84-phase18-cli-crates-io-publish.md`。  
> **前置：** Epic 84 / FR148 **closed**；库 crate **1.0.0** 已在 crates.io；Correct Course Q 默认（rename→`bitloom-firrtl`/`bitloom-viz`；FR152(b)）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 85 后续故事 **85.2–85.6** 标为 `ready`，亦不得开工实现。**不得**在本 epic 关闭前宣称 FR149–152 已关闭。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR149–FR152 / Epic 85；NFR14、NFR64、NFR65、NFR66、NFR67；对照 FR148 / FR153 |
| 记录日期 | 2026-09-11 |
| 状态 | closed — Story 85.6 勾选完成；Epic 85 关闭；FR149–152 实现面可宣称；**FR153** 仍属 Epic 86（86.1 可 ready-for-dev） |
| **选定** | 在保留 Phase 17 关闭面与 Epic 84 合同的前提下，授权 rename/publish `bitloom-firrtl`/`bitloom-viz`、FR152(b) lsp 策略、以及 `bitloom` CLI 1.0.0 上架 |

### Phase 17 / 84 关闭面 vs Epic 85 实现边界（NFR64 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **Phase 17 库 1.0** | FR141–147；库 crate 1.0.0 已上 crates.io | **仍有效；不得改写「已关闭」**（NFR64） |
| **Epic 84 / FR148** | Phase 18 CLI 可发布合同闸门 | **已关闭；不得回写为未批准** |
| **Epic 85 / FR149–152** | firrtl/viz 可发布；lsp (b)；CLI 上架 | **本实现 epic** |
| **Epic 86 / FR153** | 发版后诚实 / SemVer 跟进 | **本 epic 不做**；关闭后才 ready |

**选定：完成 crates.io 安装路径所需 rename/publish/策略，不扩大 FR142，不清空 NFR59（NFR67）。**

### FR149–152 范围（本 epic）

| FR | 交付 | 故事 |
| --- | --- | --- |
| **FR149** | `rhdl-firrtl` → 可发布 **`bitloom-firrtl`**（`publish=true`；禁 `rhdl-firrtl` 发布名；目录可暂留） | 85.2 |
| **FR150** | `rhdl-viz` → 可发布 **`bitloom-viz`**（同理） | 85.3 |
| **FR152** | lsp 策略 **(b)**：`publish=false` 可保留，**必须不挡** `cargo publish -p bitloom`（消除 path-only） | 85.4 |
| **FR151** | `bitloom` 1.0.0 `cargo publish` 成功；`cargo install bitloom` 路径可文档验证 | 85.5 |
| 收口 | 文档/deferred/NFR14 勾选；明确 FR153→Epic 86 | 85.6 |

**软序：** firrtl → viz → lsp 策略 → CLI publish → closeout。

### (a) 上游约束

- **Epic 84 已关闭：** FR148 Correct Course + PRD 批准；README/deferred 诚实面；AD-2 Phase 18 指针。
- **库依赖：** `bitloom-hir`（及既有库 crate）**1.0.0** 已在 crates.io，可供 firrtl/viz/CLI version 需求引用。
- **Rename 迁移风险：** workspace 依赖键、Rust crate 路径 `rhdl_firrtl`/`rhdl_viz` → `bitloom_firrtl`/`bitloom_viz`、文档/CI/夹具路径（目录可暂留 `crates/rhdl-*`）、examples、Justfile。
- **NFR66 / AD-2：** 对外包名 **`bitloom-*`**；**禁止** publish **`rhdl` / `rhdl-bits`**；亦禁止以 **`rhdl-firrtl` / `rhdl-viz`** 为 crates.io 发布名。
- **AD-6：** 设计 crate 只依赖 **`bitloom-prelude`**；不得改写该边界。
- **NFR64：** 不得改写 Phase 17 FR141–147「已关闭」；不得把 CLI 上架叙述偷换成 Phase 17 失败补救。
- **NFR67：** 不得静默吞并 **NFR59**；不得静默扩大 **FR142** 表面；不得把 LSP 产品加深冒充本批必做（默认 (b)）。
- **Publish 诚实：** dry-run 必须绿；实发尽量执行；若 credentials 失败须文档化且 version deps 固定。
- **公开品牌：** **Bitloom**（crates.io / CLI：`bitloom`）。

### (b) 粗工期带

- **预计：** Epic 85 整体约 **1–3 人周**（85.1 ≤0.25；85.2–85.3 rename/publish 各 0.25–0.75；85.4 lsp 依赖解耦 0.25–0.75；85.5 CLI publish 0.25–0.5；85.6 收口 0.25）。
- **置信度 / 假设：** 中（rename 图大；crates.io 实发依赖凭证与 index 延迟）。假设库 1.0.0 仍在 registry；假设不提前开 Epic 86 实现。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **85.2–85.6** 标 `ready` 或开工实现。
- **不得 publish `rhdl` / `rhdl-bits`**（亦不得以 `rhdl-firrtl`/`rhdl-viz` 为 crates.io 名）。
- **不得改写设计 crate → `bitloom-prelude` 边界**（AD-6）。
- **不得静默扩大 FR142** 表面清单。
- **不得留下 path-only 依赖挡 `cargo publish -p bitloom`**（含 lsp）。
- **不得静默吞并 NFR59**（NFR67）。
- **不得把 LSP 加深 / FR152(a) 冒充本批必做。**
- 不得在 FR151 未关闭时宣称 `cargo install bitloom` 已可用（诚实面属关闭证据）。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。
- **不得开工 Epic 86** 直至 Epic 85 关闭（Story 85.6）；关闭后可将 **86.1** 标 `ready-for-dev`。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR64 / NFR65 / NFR66 / NFR67** 共同责任人
- 备份 / 升级路径：撤回 rename 默认 / 改选 FR152(a) 须升级至产品 / Correct Course 批准人；AD-2 发布名争议升级至架构（AD-28）维护者。

---

### Epic 85 故事分工

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **85.1** | 本 NFR14 + ATDD | **本故事** |
| **85.2** | FR149 bitloom-firrtl | Gate：须本记录后才可 ready |
| **85.3** | FR150 bitloom-viz | Gate：须本记录后才可 ready |
| **85.4** | FR152(b) lsp 策略 | Gate：须本记录后才可 ready |
| **85.5** | FR151 bitloom CLI publish | Gate：须 85.2–85.4 |
| **85.6** | 收口；勾选 Epic 85；86.1 ready | 关闭本记录 |

### 引用

- AD-28 — 风险门禁（NFR14）；AD-2 — 对外 `bitloom-*`；AD-6 — prelude
- PRD NFR14；FR149–152；NFR64–NFR67
- Correct Course：`sprint-change-proposal-2026-09-11-phase18-cli-crates-io-publish.md`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 85 关闭条件（Story 85.6 勾选）

- [x] **FR149：** `bitloom-firrtl` dry-run（+ 实发可勾选）— Story 85.2
- [x] **FR150：** `bitloom-viz` dry-run（+ 实发可勾选）— Story 85.3
- [x] **FR152(b)：** lsp 不挡 `bitloom` publish — Story 85.4
- [x] **FR151：** `bitloom` 1.0.0 publish（或 dry-run+凭证失败文档）— Story 85.5
- [x] **NFR14 / NFR64–67：** 本记录勾选关闭；NFR59 仍 deferred
- [x] **FR153：** 明确仍属 Epic 86；可将 86.1 标 ready-for-dev

**Epic 85 已关闭（Story 85.6）：** FR149–152 实现面可宣称；`cargo install bitloom` 可用（FR151）；**FR153** 诚实/SemVer 跟进仍属 Epic 86；**不得**静默吞并 NFR59（NFR67）；公开品牌 **Bitloom**。
