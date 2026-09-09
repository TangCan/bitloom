---
title: Sprint Change Proposal — Phase 12 字面七阶段全绿（Path B）
date: 2026-09-09
status: approved
approved: 2026-09-09
trigger: Phase 12 Path B / 推翻 FR93
mode: Batch
change_scope: Major
related_prior: sprint-change-proposal-2026-09-09.md (Phase 11 合同绿 · approved)
epics_ref: epics.md Phase 12 Inventory · Epic 40–47 · FR94–FR105 / NFR40–NFR43
---

# Sprint Change Proposal — Phase 12 字面七阶段全绿（Path B）

## 1. Issue Summary

**触发：** Correct Course（2026-09-09，Batch）。用户确认：在 Phase 11 **合同绿（A）已结项**之后，继续追求 **「字面七阶段全绿（B / B1）」** — 推翻 Phase 11 **FR93** 永久非目标，批准 Phase 12 **FR94–FR105 / NFR40–NFR43** 与 **Epic 40–47**（Create Epics 工作流已完成规划草稿）。

**背景：**

1. Phase 1–11（Epic 1–39）sprint 故事/回顾/action items **全部 `done`**；`phase11Status: complete`。
2. Phase 11 Correct Course（同日较早提案）将「产品做完 / 七阶段全绿」定义为 **合同绿（FR87 / NFR38）**，并锁定 **FR93** 五条永久非目标（须新 PRD 才能推翻）。
3. Research `technical-doc19-seven-stage-full-green-product-pla-2026-09-09` **明确建议勿走字面全绿**；用户现 **强制选择 B1**，接受多年/高维护。
4. Create Epics（Phase 12）已在 `epics.md` 追加 Inventory + Epic 40–47（30 则故事）；**Story 40.2** 要求本 Correct Course + PRD/addendum 授权后方可实现。

**问题陈述：** 若不正式修正 PRD/addendum 与 AD，实现侧仍受 FR93 / AD-5 / AD-25 / AD-27 / NFR38 约束，无法合法开闸 Epic 41–47；对外也无法把「字面全绿」与「合同绿历史里程碑」区分清楚。

**证据：** `epics.md` Phase 12；addendum「Phase 11 合同绿」+ FR93 指针；`deferred-work.md` 永久非目标；ARCHITECTURE-SPINE AD-5/25/27；research 反建议字面全绿；用户 Explicit Path B + Batch 确认。

---

## 2. Impact Analysis

### Checklist 记录（Step 2）

#### §1 Trigger & Context
- [x] **1.1** 触发非单 story；对齐 Story **40.2**（Correct Course + PRD 推翻 FR93）与 Phase 12 CE 完成
- [x] **1.2** 类型：**战略 pivot** — 从合同绿完成口径转向字面绿；新需求 FR94–105
- [x] **1.3** 证据：epics Phase 12、Phase 11 批准记录、research、用户确认

#### §2 Epic Impact
- [x] **2.1** Epic 1–39 **不回滚**；合同绿交付仍为历史事实
- [x] **2.2** **新增** Epic 40–47（已设计）；不修改 36–39 已关闭故事正文（仅交叉引用口径）
- [x] **2.3** 无「剩余 planned epic」受阻；sprint 目前止于 39
- [x] **2.4** 不使 36–39 作废；使 FR93「永久锁定」状态作废（经 FR94）
- [x] **2.5** 顺序：必须 **40 →（41–47 可并行，均硬依赖 40）**

#### §3 Artifact Conflicts
- [!] **3.1 PRD** — addendum Phase 11 禁止字面宣称 + FR93「须新 PRD」→ **必须**追加 Phase 12 段落并推翻/收缩 FR93
- [!] **3.2 Architecture** — AD-5（不承诺 TLM）、AD-25（HLS 仅外挂）、AD-27（机械≠idiomatic）、Deferred/永久非目标指针 → **必须**修订（Story 40.4 / NFR41）
- [N/A] **3.3 UX** — 无 UI 合同
- [!] **3.4 其他** — `docs/requirements/19` §19.7–19.9；README / `deferred-work.md`；sprint-status 需播种 Epic 40–47；CI 日后随实现扩展（非本提案一次改完）

#### §4 Path Forward
- **4.1 Direct Adjustment：** 仅在现 sprint 内改故事 — **不可行**（无 40+ 条目；且合同禁止）
- **4.2 Rollback：** 回滚 Phase 11 — **不可行且不需要**（合同绿保留为历史里程碑）
- **4.3 MVP Review：** 重新定义「产品做完」为字面绿 — **可行且必要**（扩大范围，非缩小）
- [x] **4.4 选定：Hybrid = Option 3（重新定义完成口径）+ 批准已起草的 Phase 12 epic/story 清单**  
  - Effort：**High**（多年）  
  - Risk：**High**（同业未打包交付；维护成本）  
  - 不假装为 Medium sprint

#### §5–6 Proposal components
- 见本文 §1–5；sprint-status 更新在 **批准后**执行（checklist 6.4）

---

## 3. Recommended Approach

**批准 Phase 12 Path B 合同翻转**，不回滚 Phase 11：

1. PRD/addendum 追加 **Phase 12 字面绿**；明确 **推翻 FR93**（五条改为可交付 FR95–105）。
2. 保留 FR87 合同绿为 **历史已交付标签**；「产品做完 / 字面全绿」仅可引用 **FR94–105**（NFR42）。
3. 实现顺序：Sprint Planning 播种 → 从 **40.1** 起；**Epic 40 关闭前** 41–47 不得 ready。
4. Architecture / doc-19 / README / deferred 按 Story 40.3–40.4 落地（可在 40.x 实现故事中执行，本提案批准其合同权限）。

**不选 Rollback：** Phase 11 工程与诚实边界仍有价值。  
**不选「只改文档不改 PRD」：** 违反 FR93「须新 PRD」自指条款。

---

## 4. Detailed Change Proposals（Batch）

### 4.1 PRD / addendum

**Artifact:** `prds/prd-rhdl-2026-08-19/prd.md` + `addendum.md`

**OLD（addendum Phase 11 要旨）:**
- 对外字面全绿禁止；落地后仅可按 FR87/NFR38 宣称
- FR93 永久非目标须新 PRD 才能推翻

**NEW:**
- 追加 **「2026-09-09 Update — Phase 12 字面绿（Path B）」** 节：
  - 批准本提案；Phase 12 = Epic 40–47 · FR94–FR105 / NFR40–NFR43
  - **推翻 FR93** 五条锁定；对应交付见 FR95–105
  - Phase 11 合同绿（FR87）保留为历史里程碑；**不再**作为「产品做完」唯一口径
  - 字面宣称纪律 = NFR42
  - 指针：`epics.md` Phase 12；research（注明产品否决「勿走字面」建议）
- `prd.md` frontmatter `amendment` 追加 `phase12-literal-green-path-b-2026-09-09`

**Rationale:** 满足 FR93 自指的「新 PRD」条件；授权 Story 40.2+。

### 4.2 Epics

**Artifact:** `epics.md` Phase 12

**OLD:** `phase12Status: complete`（规划完成）；Epic 40–47 已写；sprint 未登记

**NEW（本提案批准后元数据）:**
- `correctCoursePhase12Approved: 2026-09-09`
- `phase12Contract: literal-green-path-b`
- 故事正文 **无需重写**（已通过 CE 校验）
- Epic 41–47 frontmatter 保持「依赖 40」闸门说明

**Rationale:** 规划已齐；Correct Course 只补合同戳。

### 4.3 Architecture

**Artifact:** `ARCHITECTURE-SPINE.md`

**OLD:** AD-5 不承诺 SystemC TLM；AD-25 HLS 仅外挂；AD-27 机械可编译即可；Deferred 指向 FR93 永久非目标

**NEW（权限；具体 diff 由 Story 40.4 落地）:**
- AD-5：允许 **FR101** SystemC TLM-2.0 产品路径（修订「不承诺」）
- AD-25：允许 **FR95** 树内调度为产品主路径之一（外挂可保留可选）
- AD-27：增加 **FR97** idiomatic 验收面（机械路径可并存）
- Deferred / 永久非目标指针：改为指向 Phase 12 交付 FR，不再写「须新 PRD 才能推翻」

**Rationale:** NFR41；否则实现会被旧 AD 挡死。

### 4.4 Documentation / deferred / roadmap

**Artifacts:** `docs/requirements/19. 实施路线图.md`；README；`deferred-work.md`

**NEW（权限；由 Story 40.3–40.4 落地）:**
- §19.7–19.9 改为字面绿勾选条件（对齐 FR95–105）
- 撤销 FR93「永久非目标」锁定节的冻结措辞
- 文首区分：合同绿 = 历史；字面绿 = 当前合同

### 4.5 Sprint status

**Artifact:** `implementation-artifacts/sprint-status.yaml`

**NEW（批准后立即或经 Sprint Planning）:**
```yaml
  epic-40: backlog
  40-1-epic-40-nfr14-风险记录: backlog
  40-2-correct-course-prd-推翻-fr93-fr94: backlog
  40-3-重写-doc-19-字面绿定义-fr94: backlog
  40-4-修订-ad-撤销-fr93-锁定-fr94: backlog
  epic-40-retrospective: optional
  epic-41: backlog
  # … 41.1–41.4, 42.x, 43.x, 44.x, 45.x, 46.x, 47.x …
  epic-47-retrospective: optional
```
（键名可按仓库既有 slug 惯例微调；状态一律 `backlog`，直至 40 关闭。）

**Rationale:** checklist 6.4；否则 Build 无追踪面。

### 4.6 UX
**N/A**

### 4.7 Code / CI
**本提案不直接改代码。** 实现从 40.1 起；CI 扩展随各 FR 故事。

---

## 5. Implementation Handoff

**Change scope: Major**（战略完成口径翻转 + 多年交付面）

| 角色 | 职责 |
|------|------|
| **PM / 用户（Richard）** | 批准本提案；确认接受 research 否决的风险 |
| **PO / Dev** | 批准后：改 PRD/addendum 戳记；Sprint Planning 播种 40–47；执行 Story 40.1–40.4 |
| **Architect** | Story 40.4 修订 AD-5/25/27 |
| **Developer agents** | Epic 40 关闭后按序/并行实现 41–47 |

**Success criteria:**
1. 本提案 `status: approved`
2. addendum Phase 12 段落合入；FR93 锁定解除有据
3. sprint-status 含 Epic 40–47 backlog
4. Story 40.1–40.4 可标 ready 并实施；其后 41–47 闸门仅依赖 Epic 40 done

**Non-goals of this proposal:** 不在本文件内实现树内 HLS / LSP / TLM 等代码。

---

## 6. Checklist §6 预留

- [x] **6.1–6.2** 分析与提案已形成（待用户审）
- [x] **6.3** 用户显式批准（yes · 2026-09-09）
- [x] **6.4** 批准后更新 sprint-status（Epic 40–47 backlog）
- [x] **6.5** 交接确认（Major → PM/Architect + PO/Dev；实现自 40.1）

---

**HALT — 请选择：**
- **Continue** — 进入正式批准询问（Step 5）
- **Edit** — 先修订本提案
