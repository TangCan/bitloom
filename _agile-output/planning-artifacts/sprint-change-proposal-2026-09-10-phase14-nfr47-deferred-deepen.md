---
title: Sprint Change Proposal — Phase 14 NFR47 未选加深升格
date: 2026-09-10
status: approved
approved: 2026-09-10
trigger: Phase 14 FR116 / NFR47 deferred deepen 合同闸门
mode: Batch
change_scope: Moderate
related_prior: sprint-change-proposal-2026-09-10-phase13-mvp-commercial-deepen.md (Phase 13 · approved)
epics_ref: epics.md Phase 14 Inventory · Epic 57–63 · FR116–FR123 / NFR48–NFR51
---

# Sprint Change Proposal — Phase 14 NFR47 未选加深升格

## 1. Issue Summary

**触发：** Correct Course（2026-09-10，Batch）。用户确认：在 Phase 13「MVP→商业加深」**已关闭**（FR106–115 / Epic 48–56）之后，批准把仍标 **NFR47 deferred** 的未选加深子集升格为正式 **Phase 14**（Epic 57–63 · **FR116–FR123** / **NFR48–NFR51**），使下列主题获得合同授权：

1. Tywaves 级 typed IDE 波形（原 FR114 子集 A）  
2. 无 metadata 全树 `#[bitloom::top]` syn-scan（原 FR113 deferred）  
3. SymbiYosys/SMT（原 FR112 分支 A）  
4. 商业 VIP GPIO 全家桶（超出 FR108 近 VIP）  
5. Handshake 默认可综合（须修订 AD-25）  
6. 官方风格 Chisel 全家桶（超出 FR111 D1+D3）

**背景：**

1. Phase 1–13（Epic 1–56）sprint 故事/回顾/action items **全部 `done`**；`phase13Status: complete`；Phase 13 retros + action-items sweep 已接受。  
2. Phase 13 诚实边界将 Tywaves / syn-scan / SBY / VIP GPIO 全家桶 / Handshake 默认 / 官方风格全家桶等记为 **须新合同（NFR47）**；不得用既有关闭面冒充。  
3. Create Epics（Phase 14）已在 `epics.md` 追加 Inventory + Epic 57–63（**22** 则故事）；`phase14Status: planning-complete`；**Story 57.2 / FR116** 要求本 Correct Course + PRD/addendum 授权后方可实现加深 epic。  
4. 现行 `addendum.md` **仅批准到 Phase 13**；脊柱 Deferred「现行加深面」仍写 Phase 13；`sprint-status.yaml` **尚无** Epic 57–63。

**问题陈述：** 若不正式修正 PRD/addendum 与诚实文档指针，实现侧仍受「NFR47 / 须新合同」约束，无法合法将 Epic 58–63 标 `ready`；对外也无法把「Phase 13 商业加深已关」与「Phase 14 NFR47 加深进行中/已关」区分清楚。**本提案不宣称 Phase 12/13 AC 未达标，也不回滚任何既有关闭证据。**

**证据：** `epics.md` Phase 14；addendum 止于 Phase 13；`deferred-work.md` Phase-13 sweep（item-145/149/153/157/161/165）；`docs/fr112`/`fr113`/`fr114`/`fr110`/`fr111` deferred 条；ARCHITECTURE-SPINE AD-25/27「须新合同」；用户 Explicit 触发确认 + Batch。

---

## 2. Impact Analysis

### Checklist 记录（Step 2）

#### §1 Trigger & Context
- [x] **1.1** 触发故事 **57.2**（FR116）— Correct Course + PRD 批准 Phase 14  
- [x] **1.2** 类型：**战略/合同升格** — 新需求 FR116–123；非实现失败  
- [x] **1.3** 证据：epics Phase 14、Phase 13 关闭面、deferred NFR47、用户确认  

#### §2 Epic Impact
- [x] **2.1** Epic 1–56 **不回滚、不改 AC**  
- [x] **2.2** **已规划** Epic 57–63 获合同批准；本轮不重拆范围  
- [x] **2.3** 58–63 硬依赖 57；sprint 目前止于 56  
- [x] **2.4** 无作废 epic；NFR47 deferred 升格为 FR117–122  
- [x] **2.5** 顺序：**57 关闭前** 58–63 不得 ready；其后可按容量并行  

#### §3 Artifact Conflicts
- [!] **3.1 PRD** — 须追加 Phase 14 段；frontmatter amendment；权威边界增 FR116–123  
- [!] **3.2 Architecture** — Deferred「现行加深面」须区分 Phase 13 vs Phase 14；AD-25（Handshake）/ AD-27（风格全家桶）实质修订由 Epic 62/63 + Story 57.4 指针门禁；formal/SBY 形状由 Epic 60 钉死  
- [N/A] **3.3 UX**  
- [!] **3.4 其他** — README / deferred-work；sprint-status 播种 57–63；可选 doc-19 交叉链（Story 57.3）  

#### §4 Path Forward
- **4.1 Direct Adjustment：** **Viable（选定）** — 增补合同与追踪；不回滚代码。Effort Low–Medium；Risk Low–Medium（Handshake/AD-25、VIP GPIO/`ip.rs`、SBY 工具链）  
- **4.2 Rollback：** **Not viable**  
- **4.3 MVP Review（缩 Phase 12/13）：** **Not chosen** — 既有关闭面保持  
- [x] **4.4 选定：Option 1 Direct Adjustment（Hybrid 轻量）** — 同形于 Phase 13 提案流程，只**追加加深合同**  

#### §5–6 Proposal components
- 见本文 §1–5；sprint-status 更新在 **批准后**执行（checklist 6.4）  

---

## 3. Recommended Approach

**批准 Phase 14「NFR47 未选加深升格」合同**，不回滚 Phase 12/13：

1. PRD/addendum 追加 **Phase 14**；明确 **FR116–123 / NFR48–NFR51**；Phase 12 FR94–105 与 Phase 13 FR106–115 **关闭证据仍有效**（NFR48）。  
2. 对外「Tywaves / syn-scan / SBY / VIP GPIO / Handshake / 官方风格全家桶」类宣称：**仅**可在对应 FR116–122 关闭后，按 **FR123** 勾选；**禁止**用 Phase 13 完成面冒充本批加深。  
3. 实现顺序：Sprint Planning / 本提案 6.4 播种 → 从 **57.1** 起；**Epic 57 关闭前** 58–63 不得 ready。  
4. README / deferred / 脊柱指针按 Story **57.3–57.4** 落地；AD-25/27（及 formal 路径）加深修订在各自实现 epic 首故事引用（NFR50）。  

**不选 Rollback：** Phase 12/13 工程与诚实边界仍是基线。  
**不选「只改 sprint 不开 PRD」：** 违反 FR116 / NFR47「explicit new contract required」。

---

## 4. Detailed Change Proposals（Batch）

### 4.1 PRD `prd.md`

**Artifact:** `prds/prd-rhdl-2026-08-19/prd.md`

**OLD（摘录）:**
```yaml
updated: 2026-09-10
amendment: …; phase13-mvp-commercial-deepen-2026-09-10
```
权威边界止于 Phase 13 FR106–115；无 Phase 14 段落指针。

**NEW:**
- `amendment` 追加：`phase14-nfr47-deferred-deepen-2026-09-10`
- 文首追加一行：*2026-09-10 追加（`phase14-nfr47-deferred-deepen`）：Phase 14 NFR47 未选加深升格 **FR116–FR123** / **NFR48–NFR51** — Phase 12/13 关闭仍有效；加深宣称仅引用 FR116–122（见 addendum「Phase 14」）。*
- §0 权威边界追加：**2026-09-10 Phase 14：** FR116–123 / NFR48–51 — 不回滚 FR94–115。

**Rationale:** FR116 自指的 PRD 批准条件。

### 4.2 PRD `addendum.md`

**Artifact:** `prds/prd-rhdl-2026-08-19/addendum.md`

**OLD:** 最新节为「2026-09-10 Update — Phase 13 MVP→商业加深」；无 Phase 14。

**NEW — 追加节「2026-09-10 Update — Phase 14 NFR47 未选加深升格（Correct Course）」：**

- **用户决议：** 批准本提案；正式 Phase 14（Epic 57–63 · FR116–FR123 / NFR48–NFR51）。  
- **公开品牌（不变）：** Bitloom / `bitloom` / `bitloom-*`；设计 crate 仍只依赖 `bitloom-prelude`。  
- **与 Phase 12/13 关系：**
  - Phase 12 FR94–105 / Phase 13 FR106–115 **关闭证据仍有效**；**不得**改写为失败（NFR48）。  
  - Phase 14 是 **新合同** 下的 NFR47 deferred → 显式 FR，**不是**「Phase 13 AC 未达标后的补救」。  
  - deferred 升格映射：Tywaves→**FR117**；syn-scan→**FR118**；SBY/SMT→**FR119**；VIP GPIO→**FR120**；Handshake 默认可综合→**FR121**；官方风格 Chisel→**FR122**；宣称纪律→**FR123**；闸门→**FR116**。  
- **实现闸门：** Epic 57（含 FR116）关闭前，Epic 58–63 不得标 ready。触及 AD-25/27（及 formal）须在实现 epic 引用修订 AD（NFR50）。  
- **合同指针：** `epics.md` Phase 14；本提案路径。  

**Rationale:** 满足 Story 57.2 AC；授权 57.3+。

### 4.3 Epics

**Artifact:** `epics.md`

**OLD:** `phase14Status: planning-complete`；合同戳「待 Epic 57」；无 `correctCoursePhase14Approved`。

**NEW（批准后元数据）:**
- `correctCoursePhase14Approved: 2026-09-10`
- Inventory「合同戳（待 Epic 57）」→「Correct Course 已批准；实现闸门 = Epic 57 关闭」
- 故事正文 **无需重写**（22 则已齐）

**Rationale:** 规划已齐；Correct Course 只补合同戳。

### 4.4 Architecture

**Artifact:** `ARCHITECTURE-SPINE.md` Deferred / AD-25 / AD-27 指针（由 Story 57.4 + 实现 epic 落地）

**OLD:**
> Phase 13「MVP→商业加深」合同（现行加深面）：FR106–FR115 …

**NEW（权限）:**
- Phase 13 FR106–115 = **商业加深已关闭**（基线）。  
- Phase 14 FR116–123 = **NFR47 未选加深合同**（Correct Course 2026-09-10）；关闭后方可按 FR123 宣称对应面。  
- **AD-25：** Epic 62 **必须**修订以允许 Handshake/动态 DF 为默认可综合语义（FR121）；未修订不得宣称 FR121。  
- **AD-27：** Epic 63 可进一步修订风格全家桶；**默认仍禁止**恢复 Parser（除非风险记录另开）。  
- Formal/SBY：实现形状由 Epic 60 + 风险记录钉死（不在脊柱预钉 crate）。  
- Deferred 补充：Tywaves→FR117；syn-scan→FR118；SBY→FR119；VIP GPIO→FR120；Handshake→FR121；官方风格→FR122。  

**Rationale:** 避免脊柱「现行=仅 Phase 13」挡死加深叙事。

### 4.5 Documentation / deferred / README

**Artifacts:** README；`deferred-work.md`；可选 `docs/requirements/19`

**NEW（权限；Story 57.3 落地）:**
- README：明确 Phase 13 已关；Phase 14 = NFR47 升格合同（Epic 57–63）；对应宣称须引 FR116–122。  
- `deferred-work.md`：将 item-149/153/157/161/165 等标注 **升格为 FR117–122 / Epic 58–63**（关闭前仍 deferred 实现态；合同已批准）。  
- standing honesty **保留**；去掉「尚无合同」措辞。  
- 未列入本批的 deferred（自动 FSM 标签、更多 IP FL、第三方 LCOV GUI、emit MemRead 完整生成）仍须另开合同（NFR51）。  

### 4.6 Sprint status

**Artifact:** `implementation-artifacts/sprint-status.yaml`

**NEW（批准后立即或经 Sprint Planning；checklist 6.4）:**

```yaml
  # Phase 14 — NFR47 deferred deepen (Correct Course 2026-09-10-phase14)
  # Gate: epic-57 must be done before 58–63 may leave backlog/ready
  epic-57: backlog
  57-1-epic-57-nfr14-风险记录: backlog
  57-2-correct-course-prd-批准-phase14-fr116: backlog
  57-3-同步-readme-deferred-路线图-fr116-fr123: backlog
  57-4-ad-指针与-epic-57-收口-fr116: backlog
  epic-57-retrospective: optional

  epic-58: backlog
  58-1-epic-58-nfr14-风险记录: backlog
  58-2-typed-ide-波形路径-fr117: backlog
  58-3-fr117-收口与文档指针: backlog
  epic-58-retrospective: optional

  epic-59: backlog
  59-1-epic-59-nfr14-风险记录: backlog
  59-2-syn-scan-发现路径-fr118: backlog
  59-3-fr118-收口与文档指针: backlog
  epic-59-retrospective: optional

  epic-60: backlog
  60-1-epic-60-nfr14-风险记录: backlog
  60-2-sby-smt-路径-fr119: backlog
  60-3-fr119-收口与文档指针: backlog
  epic-60-retrospective: optional

  epic-61: backlog
  61-1-epic-61-nfr14-风险记录: backlog
  61-2-商业-vip-gpio-实现-fr120: backlog
  61-3-fr120-收口与文档指针: backlog
  epic-61-retrospective: optional

  epic-62: backlog
  62-1-epic-62-nfr14-风险记录: backlog
  62-2-ad25-handshake-默认可综合-fr121: backlog
  62-3-fr121-收口与文档指针: backlog
  epic-62-retrospective: optional

  epic-63: backlog
  63-1-epic-63-nfr14-风险记录: backlog
  63-2-官方风格-chisel-全家桶-fr122: backlog
  63-3-fr122-收口与-phase14-指针: backlog
  epic-63-retrospective: optional
```

**Rationale:** checklist 6.4；Epic 57 硬闸门写在注释。

### 4.7 Stories

**无正文改写。** 既有 Story 57.1–63.3 AC 已覆盖闸门、诚实边界与 AD 修订义务。批准后仅允许按实现细节微调 NFR14 风险记录文件名/路径，不改变 FR 完成面。

---

## 5. Implementation Handoff

**Change scope:** **Moderate**（合同 + backlog 重组；不改动已关闭 epic 的代码契约）

| 角色 | 职责 |
|------|------|
| **PO / Dev** | 批准后：落地 4.1–4.2（PRD/addendum）；更新 `epics.md` 合同戳；播种 `sprint-status.yaml`（4.6） |
| **Dev（Epic 57）** | Story 57.1–57.4：NFR14 → 本提案引用 → README/deferred → AD 指针收口；**关闭前 58–63 不得 ready** |
| **Dev（Epic 58–63）** | 各 epic NFR14 → 产品路径 → 收口；62/63 须修订 AD-25/27；60 钉 SBY/SMT 工具链 |
| **Architect** | 审 AD-25/27 修订 diff（跟 62/63）；Deferred 标签与 Phase 14 指针 |

**Success criteria:**
1. addendum 含 Phase 14 段；`correctCoursePhase14Approved` 写入 epics frontmatter  
2. sprint-status 含 Epic 57–63（backlog）  
3. Epic 57 关闭后方可 ready 58–63  
4. 对外宣称遵守 FR123 / NFR48  

**不在本提案范围：** 自动 FSM 标签提取、更多 IP 手写 FL、第三方 LCOV GUI 一等、emit MemRead 完整生成（NFR51）。

---

## 6. Approval

- [x] User approves this Sprint Change Proposal for implementation（2026-09-10）
- [x] Checklist 6.4 sprint-status seeding executed after approval
- [x] Story 57.2 may cite this document as Correct Course evidence
