---
title: Sprint Change Proposal — Phase 15 NFR51 剩余升格
date: 2026-09-10
status: approved
approved: 2026-09-10
trigger: Phase 15 FR124 / NFR51 leftover deepen 合同闸门
mode: Batch
change_scope: Moderate
related_prior: sprint-change-proposal-2026-09-10-phase14-nfr47-deferred-deepen.md (Phase 14 · approved)
epics_ref: epics.md Phase 15 Inventory · Epic 64–71 · FR124–FR132 / NFR52–NFR55
---

# Sprint Change Proposal — Phase 15 NFR51 剩余升格

## 1. Issue Summary

**触发：** Correct Course（2026-09-10，Batch）。用户确认：在 Phase 14「NFR47 未选加深升格」**已关闭**（FR116–123 / Epic 57–63）之后，批准把仍标 **NFR51 deferred** 的明示剩余升格为正式 **Phase 15**（Epic 64–71 · **FR124–FR132** / **NFR52–NFR55**），使下列主题获得合同授权：

1. 上游 Tywaves 一等集成（超出 FR117 自研 typed-wave）  
2. 更多 IP 手写 FL（formal 分支 C）  
3. 默认 CI 强制真 sby  
4. 全 SoC pad / 商业对拍深度（超出 FR120 `GpioVip` C1–C4）  
5. CIRCT Handshake 方言 / 多时钟弹性缓冲全家桶（超出 FR121 ready/valid）  
6. 完整 Style Guide 全文 ± Parser 恢复（超出 FR122 O1–O4；恢复 Parser 须修订 AD-27）  
7. `ip.rs` 按协议拆分卫生（现 ~3213 LOC）

**背景：**

1. Phase 1–14（Epic 1–63）sprint 故事/回顾/action items **全部 `done`**；`phase14Status: complete`；Phase 14 retros + action-items sweep 已接受。  
2. Phase 14 诚实边界将上述主题记为 **须新合同（NFR51）**；不得用既有关闭面冒充。  
3. Create Epics（Phase 15）已在 `epics.md` 追加 Inventory + Epic 64–71（**25** 则故事）；`phase15Status: planning-complete`；`correctCoursePhase15Approved: pending`；**Story 64.2 / FR124** 要求本 Correct Course + PRD/addendum 授权后方可实现加深 epic。  
4. 现行 `addendum.md` **仅批准到 Phase 14**；`sprint-status.yaml` **尚无** Epic 64–71。

**问题陈述：** 若不正式修正 PRD/addendum 与诚实文档指针，实现侧仍受「NFR51 / 须新合同」约束，无法合法将 Epic 65–71 标 `ready`；对外也无法把「Phase 14 加深已关」与「Phase 15 NFR51 剩余加深」区分清楚。**本提案不宣称 Phase 12–14 AC 未达标，也不回滚任何既有关闭证据。**

**证据：** `epics.md` Phase 15；addendum 止于 Phase 14；`deferred-work.md` Phase-14 sweep（item-173/177/181/185/189/193 等）；`action-items-sweep-2026-09-10-phase14.md`；ARCHITECTURE-SPINE AD-25/27「须新合同」边界；用户 Explicit 触发确认 + Batch。

---

## 2. Impact Analysis

### Checklist 记录（Step 2）

#### §1 Trigger & Context
- [x] **1.1** 触发故事 **64.2**（FR124）— Correct Course + PRD 批准 Phase 15  
- [x] **1.2** 类型：**战略/合同升格** — 新需求 FR124–132；非实现失败  
- [x] **1.3** 证据：epics Phase 15、Phase 14 关闭面、deferred NFR51、用户确认  

#### §2 Epic Impact
- [x] **2.1** Epic 1–63 **不回滚、不改 AC**  
- [x] **2.2** **已规划** Epic 64–71 获合同批准；本轮不重拆范围  
- [x] **2.3** 65–71 硬依赖 64；sprint 目前止于 63  
- [x] **2.4** 无作废 epic；NFR51 deferred 升格为 FR125–131  
- [x] **2.5** 顺序：**64 关闭前** 65–71 不得 ready；软实现序 **71 → 68**（同触 `ip.rs`）  

#### §3 Artifact Conflicts
- [!] **3.1 PRD** — 须追加 Phase 15 段；frontmatter amendment；权威边界增 FR124–132  
- [!] **3.2 Architecture** — Deferred「现行加深面」须区分 Phase 14 vs Phase 15；AD-25（CIRCT/多时钟）/ AD-27（Style Guide/Parser）实质修订由 Epic 69/70 + Story 64.4 指针门禁；sby CI 由 Epic 67 钉死  
- [N/A] **3.3 UX**  
- [!] **3.4 其他** — README / deferred-work；sprint-status 播种 64–71；可选 doc-19 交叉链（Story 64.3）  

#### §4 Path Forward
- **4.1 Direct Adjustment：** **Viable（选定）** — 增补合同与追踪；不回滚代码。Effort Low–Medium；Risk Medium（上游 Tywaves、真 sby CI、CIRCT、Parser/AD-27、`ip.rs` 拆分）  
- **4.2 Rollback：** **Not viable**  
- **4.3 MVP Review（缩 Phase 12–14）：** **Not chosen** — 既有关闭面保持  
- [x] **4.4 选定：Option 1 Direct Adjustment（Hybrid 轻量）** — 同形于 Phase 13/14 提案流程，只**追加加深合同**  

#### §5–6 Proposal components
- 见本文 §1–5；sprint-status 更新在 **批准后**执行（checklist 6.4）  

---

## 3. Recommended Approach

**批准 Phase 15「NFR51 剩余升格」合同**，不回滚 Phase 12–14：

1. PRD/addendum 追加 **Phase 15**；明确 **FR124–132 / NFR52–NFR55**；Phase 12–14 关闭证据 **仍有效**（NFR52）。  
2. 对外「Tywaves 一等 / 更多 IP FL / 强制 sby CI / 全 SoC pad / CIRCT Handshake / Style Guide·Parser / ip.rs 拆分」类宣称：**仅**可在对应 FR124–131 关闭后，按 **FR132** 勾选；**禁止**用 Phase 14 完成面冒充本批加深。  
3. 实现顺序：Sprint Planning / 本提案 6.4 播种 → 从 **64.1** 起；**Epic 64 关闭前** 65–71 不得 ready；软序 **71 → 68**。  
4. README / deferred / 脊柱指针按 Story **64.3–64.4** 落地；AD-25/27（及 CI formal）加深修订在各自实现 epic 首故事引用（NFR54）。  

**不选 Rollback：** Phase 12–14 工程与诚实边界仍是基线。  
**不选「只改 sprint 不开 PRD」：** 违反 FR124 / NFR51「explicit new contract required」。

---

## 4. Detailed Change Proposals（Batch）

### 4.1 PRD `prd.md`

**Artifact:** `prds/prd-rhdl-2026-08-19/prd.md`

**OLD（摘录）:**
```yaml
updated: 2026-09-10
amendment: …; phase14-nfr47-deferred-deepen-2026-09-10
```
权威边界止于 Phase 14 FR116–123；无 Phase 15 段落指针。

**NEW:**
- `amendment` 追加：`phase15-nfr51-leftover-deepen-2026-09-10`
- 文首追加一行：*2026-09-10 追加（`phase15-nfr51-leftover-deepen`）：Phase 15 NFR51 剩余升格 **FR124–FR132** / **NFR52–NFR55** — Phase 12–14 关闭仍有效；加深宣称仅引用 FR124–131（见 addendum「Phase 15」）。*
- §0 权威边界追加：**2026-09-10 Phase 15：** FR124–132 / NFR52–55 — 不回滚 FR94–123。

**Rationale:** FR124 自指的 PRD 批准条件。

### 4.2 PRD `addendum.md`

**Artifact:** `prds/prd-rhdl-2026-08-19/addendum.md`

**OLD:** 最新节为「2026-09-10 Update — Phase 14 NFR47 未选加深升格」；无 Phase 15。

**NEW — 追加节「2026-09-10 Update — Phase 15 NFR51 剩余升格（Correct Course）」：**

- **用户决议：** 批准本提案；正式 Phase 15（Epic 64–71 · FR124–FR132 / NFR52–NFR55）。  
- **公开品牌（不变）：** Bitloom / `bitloom` / `bitloom-*`；设计 crate 仍只依赖 `bitloom-prelude`。  
- **与 Phase 12–14 关系：**
  - Phase 12 FR94–105 / Phase 13 FR106–115 / Phase 14 FR116–123 **关闭证据仍有效**；**不得**改写为失败（NFR52）。  
  - Phase 15 是 **新合同** 下的 NFR51 deferred → 显式 FR，**不是**「Phase 14 AC 未达标后的补救」。  
  - deferred 升格映射：上游 Tywaves→**FR125**；更多 IP FL→**FR126**；强制 sby CI→**FR127**；全 SoC pad→**FR128**；CIRCT Handshake→**FR129**；Style Guide/Parser→**FR130**；ip.rs 拆分→**FR131**；宣称纪律→**FR132**；闸门→**FR124**。  
- **实现闸门：** Epic 64（含 FR124）关闭前，Epic 65–71 不得标 ready。触及 AD-25/27（及 CI formal）须在实现 epic 引用修订 AD（NFR54）。软序：Epic 71 先于 Epic 68。  
- **合同指针：** `epics.md` Phase 15；本提案路径。  

**Rationale:** 满足 Story 64.2 AC；授权 64.3+。

### 4.3 Epics

**Artifact:** `epics.md`

**OLD:** `phase15Status: planning-complete`；`correctCoursePhase15Approved: pending`。

**NEW（批准后元数据）:**
- `correctCoursePhase15Approved: 2026-09-10`
- Inventory「Correct Course 尚未批准」→「Correct Course 已批准；实现闸门 = Epic 64 关闭」
- 故事正文 **无需重写**（25 则已齐）

**Rationale:** 规划已齐；Correct Course 只补合同戳。

### 4.4 Architecture

**Artifact:** `ARCHITECTURE-SPINE.md` Deferred / AD-25 / AD-27 指针（由 Story 64.4 + 实现 epic 落地）

**OLD:**
> Phase 14「NFR47 未选加深升格」合同（现行加深面）：FR116–FR123 …

**NEW（权限）:**
- Phase 14 FR116–123 = **NFR47 加深已关闭**（基线）。  
- Phase 15 FR124–132 = **NFR51 剩余加深合同**（Correct Course 2026-09-10）；关闭后方可按 FR132 宣称对应面。  
- **AD-25：** Epic 69 **可能**再修订以覆盖 CIRCT Handshake / 多时钟弹性缓冲（FR129）；未评估/未修订不得宣称全家桶。  
- **AD-27：** Epic 70 可进一步修订完整 Style Guide；**若恢复 Parser 必须**显式修订本 AD；默认仍禁止静默恢复。  
- sby CI：实现形状由 Epic 67 + 风险记录钉死。  
- Deferred 补充：Tywaves 一等→FR125；更多 IP FL→FR126；强制 sby CI→FR127；全 SoC pad→FR128；CIRCT→FR129；Style Guide/Parser→FR130；ip.rs 拆分→FR131。  

**Rationale:** 避免脊柱「现行=仅 Phase 14」挡死加深叙事。

### 4.5 Documentation / deferred / README

**Artifacts:** README；`deferred-work.md`；可选 `docs/requirements/19`

**NEW（权限；Story 64.3 落地）:**
- README：明确 Phase 14 已关；Phase 15 = NFR51 升格合同（Epic 64–71）；对应宣称须引 FR124–131。  
- `deferred-work.md`：将 item-173/181/185/189/193 等标注 **升格为 FR125–131 / Epic 65–71**（关闭前仍 deferred 实现态；合同已批准）。  
- standing honesty **保留**；去掉「尚无合同」措辞（对本批升格项）。  
- 未列入本批的 deferred（自动 FSM 标签、第三方 LCOV GUI 一等、emit MemRead 完整生成、非 Cargo monorepo 任意路径扫描）仍须另开合同（NFR55）。  

### 4.6 Sprint status

**Artifact:** `implementation-artifacts/sprint-status.yaml`

**NEW（批准后立即或经 Sprint Planning；checklist 6.4）:**

```yaml
  # Phase 15 — NFR51 leftover deepen (Correct Course 2026-09-10-phase15)
  # Gate: epic-64 must be done before 65–71 may leave backlog/ready
  # Soft order: prefer epic-71 before epic-68 (shared ip.rs)
  epic-64: backlog
  64-1-epic-64-nfr14-风险记录: backlog
  64-2-correct-course-prd-批准-phase-15-fr124: backlog
  64-3-同步-readme-deferred-路线图指针-fr124-fr132: backlog
  64-4-ad-指针与-epic-64-收口-fr124-nfr54: backlog
  epic-64-retrospective: optional

  epic-65: backlog
  65-1-epic-65-nfr14-风险记录: backlog
  65-2-上游-tywaves-一等路径实现与验收-fr125: backlog
  65-3-fr125-收口与文档指针: backlog
  epic-65-retrospective: optional

  epic-66: backlog
  66-1-epic-66-nfr14-风险记录: backlog
  66-2-更多-ip-手写-fl-路径实现与验收-fr126: backlog
  66-3-fr126-收口与文档指针: backlog
  epic-66-retrospective: optional

  epic-67: backlog
  67-1-epic-67-nfr14-风险记录: backlog
  67-2-默认-ci-真-sby-门禁实现与验收-fr127: backlog
  67-3-fr127-收口与文档指针: backlog
  epic-67-retrospective: optional

  epic-68: backlog
  68-1-epic-68-nfr14-风险记录: backlog
  68-2-全-soc-pad-路径实现与验收-fr128: backlog
  68-3-fr128-收口与文档指针: backlog
  epic-68-retrospective: optional

  epic-69: backlog
  69-1-epic-69-nfr14-风险记录: backlog
  69-2-ad-25-修订-若需-circt-handshake-路径-fr129: backlog
  69-3-fr129-收口与文档指针: backlog
  epic-69-retrospective: optional

  epic-70: backlog
  70-1-epic-70-nfr14-风险记录: backlog
  70-2-ad-27-修订-若需-style-guide-parser-路径-fr130: backlog
  70-3-fr130-收口与文档指针: backlog
  epic-70-retrospective: optional

  epic-71: backlog
  71-1-epic-71-nfr14-风险记录: backlog
  71-2-ip-rs-按协议拆分实现与验收-fr131: backlog
  71-3-fr131-收口与-phase-15-故事清单指针: backlog
  epic-71-retrospective: optional
```

**Rationale:** checklist 6.4；Epic 64 硬闸门写在注释。

### 4.7 Stories

**无正文改写。** 既有 Story 64.1–71.3 AC 已覆盖闸门、诚实边界与 AD/CI 修订义务。批准后仅允许按实现细节微调 NFR14 风险记录文件名/路径，不改变 FR 完成面。

---

## 5. Implementation Handoff

**Change scope:** **Moderate**（合同 + backlog 重组；不改动已关闭 epic 的代码契约）

| 角色 | 职责 |
|------|------|
| **PO / Dev** | 批准后：落地 4.1–4.2（PRD/addendum）；更新 `epics.md` 合同戳；播种 `sprint-status.yaml`（4.6） |
| **Dev（Epic 64）** | Story 64.1–64.4：NFR14 → 本提案引用 → README/deferred → AD 指针收口；**关闭前 65–71 不得 ready** |
| **Dev（Epic 65–71）** | 各 epic NFR14 → 产品路径 → 收口；69/70 须评估/修订 AD-25/27；67 钉 CI sby；建议 **71 先于 68** |
| **Architect** | 审 AD-25/27 修订 diff（跟 69/70）；Deferred 标签与 Phase 15 指针 |

**Success criteria:**
1. addendum 含 Phase 15 段；`correctCoursePhase15Approved` 写入 epics frontmatter  
2. sprint-status 含 Epic 64–71（backlog）  
3. Epic 64 关闭后方可 ready 65–71  
4. 对外宣称遵守 FR132 / NFR52  

**不在本提案范围：** 自动 FSM 标签提取、第三方 LCOV GUI 一等、emit MemRead 完整生成、非 Cargo 全 monorepo 任意路径扫描（NFR55）。

---

## 6. Approval

- [x] User approves this Sprint Change Proposal for implementation（2026-09-10）
- [x] Checklist 6.4 sprint-status seeding executed after approval
- [x] Story 64.2 may cite this document as Correct Course evidence
