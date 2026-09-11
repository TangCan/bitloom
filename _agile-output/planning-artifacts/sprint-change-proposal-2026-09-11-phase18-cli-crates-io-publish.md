---
title: Sprint Change Proposal — Phase 18 CLI / 依赖 crate crates.io 可发布收口
date: 2026-09-11
status: approved
approved: 2026-09-11
step5_user: yes（Q1–Q5 草案默认）
trigger: FR146 库 crate 1.0.0 上架后 bitloom CLI 仍无法 cargo publish；Phase 18 规划已齐待 Correct Course 批准
mode: Batch
change_scope: Moderate
related_prior: sprint-change-proposal-2026-09-11-phase17-api-stability-1-0.md (Phase 17 · approved)
epics_ref: epics.md Phase 18 Inventory · Epic 84–86 · FR148–FR153 / NFR64–NFR67
approval_defaults: |
  Q1 rename rhdl-firrtl → bitloom-firrtl (AD-2); Q2 rename rhdl-viz → bitloom-viz;
  Q3 FR152(b) bitloom-lsp may stay publish=false if it does not block bitloom publish;
  Q4 no NFR59 prerequisite; Q5 keep current MSRV / workspace 1.0.0 lockstep for new packages.
---

# Sprint Change Proposal — Phase 18 CLI / 依赖 crate crates.io 可发布收口

## 1. Issue Summary

**触发：** Phase 17（FR141–147 / Epic 79–83）**合同与实现已关闭**：workspace **1.0.0**、tag **`v1.0.0`**、GitHub Release；库 crate（`bitloom-macro` / `hir` / `builder` / `vlog` / `sim` / `prelude`）已上 crates.io **1.0.0**。用户继续运维 follow-up 时，`cargo publish -p bitloom` **失败**。`epics.md` 已完成 Phase 18 规划（`phase18Status: planning-complete`），**尚缺** Correct Course + PRD 批准戳。

**背景：**

1. **FR146** 接受路径允许 dry-run + 清单；CLI 明示为 **manual follow-up**（`docs/fr146-bitloom-1-0-0-release.md`）。  
2. **阻塞证据（2026-09-11）：**  
   - `rhdl-firrtl` / `rhdl-viz`：`publish = false`，workspace **path-only**（无 crates.io version）  
   - `cargo publish -p bitloom`：`all dependencies must have a version requirement`（`rhdl-firrtl`）  
   - `bitloom-lsp`：`publish = false` + path **dev-dep**（须钉死不挡打包策略）  
3. **AD-2：** 对外包名须为 **`bitloom-*`**；禁止 publish `rhdl` / `rhdl-bits`；目录可暂留 `rhdl-*`。  
4. **FR142：** CLI 子命令属 1.0 **in-surface**；`rhdl-*` **不得**作为 1.0 表面名。  
5. **NFR59** 仍 deferred（NFR63）；本批 **不得**静默吞并。

**问题陈述：** 若不另开「CLI / 依赖 crate 可发布」合同，则无法合法关闭 FR146 遗留的 CLI 上架缺口，也无法诚实宣称 `cargo install bitloom`；若在无闸门下直接 rename/publish，则缺少 PRD/AD 同步与宣称纪律。

**本提案不宣称：** Phase 17 AC 未达标；不回滚 FR141–147；不要求先清空 NFR59；不把 LSP 产品加深升格为本批必做。

---

## 2. Impact Analysis

### Checklist 记录（Step 2 · Batch）

#### §1 Trigger & Context
- [x] **1.1** 触发：FR146 CLI follow-up / live `cargo publish -p bitloom` 失败；用户确认 Phase 18 Correct Course  
- [x] **1.2** 类型：**技术限制 + 合同升格** — 发布图缺口升格为 FR148–153；非 Phase 17 回滚  
- [x] **1.3** 证据：publish 错误日志、`fr146` 清单、Cargo.toml `publish=false`、AD-2、`public-api-1-0-surface.md`、已上架库 crate 1.0.0  

#### §2 Epic Impact
- [x] **2.1** Epic 1–83 **不回滚、不改已关闭 AC**  
- [x] **2.2** **已规划** Epic 84–86（`epics.md`）；本提案批准后 stamp PRD + sprint seed  
- [x] **2.3** 硬依赖：Epic 84（闸门）关闭前 85–86 不得 ready  
- [x] **2.4** 无作废 epic；不新增超出 84–86 的必做 epic  
- [x] **2.5** 顺序：**84 → 85 → 86**；85 内 soft order：firrtl → viz → lsp 策略 → CLI publish  

#### §3 Artifact Conflicts
- [!] **3.1 PRD** — 须追加 Phase 18 段；frontmatter amendment；权威边界增 FR148–153 / NFR64–67  
- [!] **3.2 Architecture** — Phase 18 指针；强调 AD-2 rename/`bitloom-*`；**不**改 AD-6（设计 crate → prelude）  
- [N/A] **3.3 UX**  
- [!] **3.4 其他** — README / deferred-work / fr146 清单 / Release 说明；AGENTS 卫生；sprint-status 播种 84–86；CI/semver assume-published 属 Epic 86  

#### §4 Path Forward
- **4.1 Direct Adjustment：** **Viable（选定）** — 批准 Phase 18 合同 → 实现 Epic 84–86。Effort Medium；Risk Medium（rename 迁移面、依赖次序、lsp 策略误伤测试）  
- **4.2 Rollback：** **Not viable** — 不回滚已上架库 1.0.0  
- **4.3 缩 MVP / 放弃 CLI 上架：** **Not chosen** — CLI 属 FR142 in-surface；放弃会留下永久诚实缺口  
- [x] **4.4 选定：Option 1 Direct Adjustment**  

---

## 3. Recommended Approach

**批准 Phase 18「CLI / 依赖 crate crates.io 可发布收口」合同**，不回滚 Phase 17：

1. PRD/addendum 追加 **Phase 18**；**FR148–FR153** / **NFR64–NFR67**；Phase 12–17 关闭证据 **仍有效**（NFR64）。  
2. **Q1–Q2：** `rhdl-firrtl` → **`bitloom-firrtl`**、`rhdl-viz` → **`bitloom-viz`**（`publish=true`；禁止以 `rhdl-*` 为 crates.io 名）。  
3. **Q3：** FR152 **(b)** — `bitloom-lsp` 默认可保持 `publish=false`，**必须**不挡 `bitloom` 打包；(a) 另开合同。  
4. **Q4：** 不以消化 NFR59 为 CLI 上架前提（NFR67）。  
5. **Q5：** 保持现行 MSRV；新包与工作区 **1.0.0** 对齐（除非故事另决）。  
6. 实现闸门：**Epic 84** 关闭前，Epic 85–86 不得 ready。  
7. 对外「CLI 已可从 crates.io 安装」类宣称：仅可在对应 **FR148–153** 关闭后勾选。  

**已规划映射（批准后不重编号）：**

| FR | Epic | 用户结果 |
|----|------|----------|
| FR148 | 84 | 合同闸门 + 诚实边界 |
| FR149–152 | 85 | firrtl/viz 可发布 + lsp 策略 + `bitloom` 上架 |
| FR153 | 86 | SemVer assume-published + 文档/Release 诚实 |

---

## 4. Detailed Change Proposals（Batch）

### 4.1 PRD `prd.md`

**Artifact:** `prds/prd-rhdl-2026-08-19/prd.md`

**NEW（批准后）：**
- `updated: 2026-09-11`（或实际批准日）
- `amendment` 追加：`phase18-cli-crates-io-publish-2026-09-11`
- §0 权威边界追加：**Phase 18：** FR148–153 / NFR64–67 — 不回滚 FR141–147；CLI 上架 ≠ 清空 NFR59。

### 4.2 PRD `addendum.md`

**NEW — 追加节「Phase 18 CLI crates.io 可发布（Correct Course）」：**

- **用户决议：** 批准本提案；正式 Phase 18（Epic 84–86 · FR148–FR153 / NFR64–NFR67）。  
- **公开品牌（不变）：** Bitloom / `bitloom` / `bitloom-*`；设计 crate 仍只依赖 `bitloom-prelude`（AD-6）。禁止 `rhdl` / `rhdl-bits`。  
- **批准默认 Q1–Q5：** 见 frontmatter `approval_defaults`。  
- **与 Phase 17 关系：** Phase 17 关闭仍有效（NFR64）；本批收口 FR146 CLI follow-up，**不是**「1.0 失败」补救叙事。  
- **实现闸门：** Epic 84 关闭前 85–86 不得 ready；顺序 **84 → 85 → 86**。  
- **不**在本合同批准瞬间强制 `cargo publish`——实发属 Epic 85 / FR149–151。

### 4.3 `epics.md`

**现状：** Phase 18 Inventory + Epic 84–86 故事 **已写入**（`phase18Status: planning-complete`）。  

**批准后：**
- 写入 `correctCoursePhase18Approved: 2026-09-11`（或实际日）  
- 可保持故事正文；Story 84.2 ATDD 须能验证本提案 `status: approved`  

### 4.4 ARCHITECTURE-SPINE / AGENTS / README / deferred

**NEW（Story 84.3–84.4 / 86.x）：**
- 脊柱/AGENTS：Phase 18 指针 + AD-2（`bitloom-firrtl` / `bitloom-viz`）  
- README / `deferred-work.md`：区分「库 1.0 已上架」vs「CLI 待 FR151」；NFR59 仍 deferred  
- `docs/fr146-*` / GitHub Release：Epic 85–86 更新 CLI 状态与 SemVer assume-published  

### 4.5 Sprint status

**批准后（建议 `bmad-sprint-planning`）：** 播种 `epic-84`…`epic-86` 及故事键为 `backlog`（或项目惯例初始态）；**不得**在 Epic 84 关闭前将 85–86 标 `ready-for-dev`。

### 4.6 Stories

**无正文 diff：** 规划故事已齐。实现按 `epics.md` Epic 84–86；若批准时改 Q，再改对应 AC。

---

## 5. Implementation Handoff

**Change scope: Moderate**

| 角色 | 职责 |
|------|------|
| PM / 用户 | 批准本提案（Q1–Q5）；确认 stamp |
| PO / Dev | PRD/addendum + epics frontmatter 戳；sprint seed |
| Developer | Epic 84 → 85 → 86（`bmad-build`）；一 story 一 commit |
| Architect | 84.4 AD-2 指针审阅（轻量） |

**Success criteria：**
1. 本提案 `status: approved` + PRD/addendum Phase 18 段可验证  
2. Epic 84 关闭后 85–86 可 ready  
3. `bitloom-firrtl` / `bitloom-viz` / `bitloom` **1.0.0** 可在 crates.io 勾选（lsp 按 (b)）  
4. FR153：文档/SemVer 诚实；NFR59 仍 deferred  

**Next after approval：** stamp PRD → sprint seed → Story **84.1** NFR14。

---

## Checklist §5–6（提案组件）

- [x] **5.1–5.5** Issue / Impact / Approach / MVP / Handoff 已写入上文  
- [x] **6.1–6.2** 分析完整；提案可执行  
- [x] **6.3** 用户显式批准（yes · 2026-09-11）  
- [x] **6.4** sprint-status 播种（批准后 · 本工作流）  
- [x] **6.5** handoff 确认（批准后 · Moderate → PO/Dev + Build）
