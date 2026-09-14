---
title: Sprint Change Proposal — 工程/合同结项收口（Phase 12–22 已关；保留 NFR86）
date: 2026-09-14
status: approved
approved: 2026-09-14
step5_user: yes（Q1–Q6 默认：不开 Phase 23；不新增实现 epic；保留 NFR86；git push 不在合同内）
trigger: Phase 17–22 合同与 sprint backlog 全部关闭后，用户确认正式「工程/合同结项」收口（Batch）；不开 NFR86→Phase 23 加深
mode: Batch
change_scope: Moderate
related_prior: sprint-change-proposal-2026-09-12-phase22-nfr81-leftovers.md (Phase 22 · approved · implementation complete)
epics_ref: 不新增 deepen epic；仅 stamp 结项口径 + 诚实 NFR86
approval_defaults: |
  Q1 本批 = 工程/合同结项收口文档与状态戳，不升格 NFR86 为 Phase 23 加深 FR；
  Q2 不新增实现 epic / 不改已关 Epic 1–117 AC；
  Q3 Phase 12–22 关闭证据仍有效；不得改写为失败；
  Q4 宣称：「sprint/合同 backlog 已空 / 工程结项」≠「NFR86 账本已空」；完整产品宣称仍须引已关 FR（至 FR184 / FR147 / FR140 等）；
  Q5 git push / 远程同步不在本合同内；
  Q6 若日后要做 NFR86 加深，须另开 Correct Course（建议 Phase 23+），不得从本结项 alone 冒充已授权。
---

# Sprint Change Proposal — 工程/合同结项收口

## 1. Issue Summary

**触发：** Correct Course（2026-09-14，Batch）。用户在「项目是否可以结项」分析后确认触发点，并选择 **Batch**。工作假设（可由用户在批准时改写）：

> 正式做 **工程/合同结项收口**：承认 Phase 12–22 钉死合同面与 sprint backlog **已全部关闭**；公开诚实面保留 **NFR86 leftovers**；**本提案不**把 NFR86 升格为 Phase 23 加深合同。

**背景：**

1. `sprint-status.yaml`：`development_status` **645** 条全部 `done`；`action_items` **385** 条全部 `done`；无 `optional` / 未完成 story。  
2. `epics.md`：`phase17Status`…`phase22Status: complete`；Phase 22 Epic 111–117 / FR178–184 规划+实现+回顾已齐。  
3. 产品完成面早已分层关闭：Phase 16 终局（FR140）、Phase 17 Bitloom 1.0（FR147）、Phase 18 CLI 上架（FR148–153）、Phase 19–22 加深升格各自诚实门。  
4. `docs/fr184-phase22-claim-honesty.md` 明示 **NFR86**（无界 tip、更深 Handshake lower、社区 Style 全家桶、继续 firtool 升钉、继续扩 FR142 等）仍须**新合同**；**不得**宣称「NFR81 账本已空」。  
5. `main` 相对 `origin/main` **ahead ~128**；按既有纪律 **`git push` 不是 FR**。

**问题陈述：** 若不正式 Correct Course 结项收口，对外/对内容易混淆三种口径：

| 口径 | 现状 |
|------|------|
| Sprint / 钉死合同 backlog 空 | **已达成** |
| Phase 16「产品终局」+ 1.0/CLI | **已达成**（历史合同） |
| Deferred / NFR86 永久清空 | **未达成且不得宣称** |

**本提案不宣称：** Phase 12–22 AC 未达标；不回滚任何已关 FR；不自动授权 NFR86 实现；不把远程 push 当成交付条件。

**证据：** sprint-status 全 done；epics Phase 17–22 complete；fr184 NFR86 leftovers；README / deferred Phase 22 pointer；结项分析对话（2026-09-14）；用户「确认触发点 + Batch」。

---

## 2. Impact Analysis

### Checklist 记录（Step 2 · Batch）

#### §1 Trigger & Context
- [x] **1.1** 触发：结项分析后用户确认 Correct Course；无单一未完成 story，属**合同收口**  
- [x] **1.2** 类型：**战略/合同收口** — 非实现失败、非回滚、非默认开 Phase 23  
- [x] **1.3** 证据：sprint 全 done；phase17–22 complete；fr184 NFR86；ahead 128（运维）  

#### §2 Epic Impact
- [x] **2.1** Epic 1–117 **可保持已关闭**；无需为结项重开实现 epic  
- [x] **2.2** **默认不新增** deepen epic；可选仅文档/状态戳故事（若批准后 create-epics 认为需要极薄闸门 epic，须另议；本提案推荐**无新 epic**）  
- [x] **2.3** 无剩余 planned deepen epic；NFR86 **不**进入本批 backlog  
- [x] **2.4** 无作废 epic；**不**因结项作废 NFR86（仍 standing）  
- [x] **2.5** 无重排；未来 Phase 23+ 仅在新 Correct Course 后出现  

#### §3 Artifact Conflicts
- [!] **3.1 PRD** — addendum 追加「2026-09-14 工程/合同结项收口」段；写明 sprint 空 + Phase 12–22 仍有效 + NFR86 仍 deferred；**不**引入 FR185+ 除非用户改选开 Phase 23  
- [!] **3.2 Architecture** — SPINE / Deferred 指针：结项收口 ≠ 改 AD；NFR86 仍「须新合同」；**不**为本批修订 AD-9/25/27（无加深）  
- [N/A] **3.3 UX**  
- [!] **3.4 其他** — README / deferred-work / AGENTS / epics frontmatter stamp（如 `engineeringCloseoutApproved` / `engineeringCloseoutStatus: complete`）；**不**改 sprint 故事键；**不**强制 push  

#### §4 Path Forward
- **4.1 Direct Adjustment：** **Viable（选定）** — 文档与状态戳收口。Effort **Low**；Risk **Low**  
- **4.2 Rollback：** **Not viable** — 不回滚 1.0 / Phase 22  
- **4.3 MVP / 缩 scope：** **N/A** — 合同面已关；本批不是缩 MVP  
- **备选（未选）：** 立刻升格 NFR86 → Phase 23 — 须用户明确改触发；本提案默认 **不做**  
- [x] **4.4 选定：Option 1 Direct Adjustment（结项收口 / 不开加深）**  

---

## 3. Recommended Approach

**批准「工程/合同结项收口」**，不回滚 Phase 12–22，不开 NFR86 加深：

1. PRD/addendum 追加结项段：Phase 12–22 关闭证据仍有效；sprint backlog 空；**NFR86 仍须新合同**。  
2. `epics.md` / README / `deferred-work.md` / `AGENTS.md` 同步结项指针与诚实边界。  
3. 对外可宣称「钉死合同面与 sprint 已结项」，**必须**同时诚实 NFR86；产品宣称仍按已关 FR 矩阵（FR140 / FR147 / FR148–153 / … / FR184）。  
4. **禁止**用本结项 alone 冒充：NFR86 已清、Phase 23 已批准、或任意未关加深已交付。  
5. **`git push` 不在本合同内。**  
6. 若日后要做 NFR86 五类（或子集）加深 → **另开 Correct Course（建议 Phase 23）**。

**Effort：** Low（文档/戳）。**Risk：** Low（误宣称风险靠诚实面控制）。

---

## 4. Detailed Change Proposals（Batch）

### 4.1 PRD addendum

**文件：** `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md`

**NEW 段（建议标题）：** `## 2026-09-14 Update — 工程/合同结项收口（Correct Course)`

要点：
- 批准结项收口；Phase 12–22 **仍有效**；sprint 故事/回顾/action items **done**。  
- 结项口径 = 钉死合同 backlog 空 + 诚实 **NFR86**；≠ backlog 永久无加深可能。  
- 不引入本批新 FR；NFR86 → 未来 Phase 23+ 另案。  
- `git push` 不是 FR。

### 4.2 epics.md

**Stamp（示例键名，实现时可微调）：**
- `engineeringCloseoutApproved: 2026-09-14`
- `engineeringCloseoutStatus: complete`
- 短 note：Phase 12–22 complete；NFR86 standing；无 Phase 23 除非新 Correct Course。

**不**追加 Phase 23 Inventory（除非用户改选）。

### 4.3 README / deferred-work / AGENTS

- README「状态与 deferred」增加结项收口一句 + 链到本提案 / fr184 NFR86。  
- deferred-work 增加「Engineering closeout」pointer（对齐 Phase 16–22 pointer 风格）。  
- AGENTS `bmad:context` 刷新：结项已批；NFR86 仍须新合同。

### 4.4 Architecture

- SPINE Deferred / Phase 指针：结项收口；**无** AD 实质修订。

### 4.5 sprint-status.yaml

- **默认无新 epic/story 键**（checklist 6.4 → N/A 或仅 bump `last_updated`）。  
- 若实现方坚持「可勾选闸门故事」，可另开极薄 Epic（本提案**不推荐**，避免空 backlog 再灌水）。

### 4.6 Stories

- **无** OLD→NEW 实现故事修改。  
- 可选后续实现故事（仅文档戳，非本提案强制）：`closeout-1` 同步 README/deferred/AGENTS — 可由 Dev 直接改文件，不必走 create-epics。

---

## 5. Implementation Handoff

**Change scope：** **Moderate**（合同/文档；PO/DEV 协调；无架构重开）

| 角色 | 职责 |
|------|------|
| PM / 用户 | 批准或改写 Q1–Q6（尤其：是否改为立刻开 Phase 23） |
| Developer | 批准后：改 addendum / epics stamp / README / deferred / AGENTS；可选 SPINE 指针；**不**实现 NFR86 |
| Architect | 仅审阅「无 AD 修订」是否成立；有异议则标出 |

**Success criteria：**
1. addendum 结项段已写入且与 Phase 22 段不冲突  
2. README/deferred/AGENTS 同步；明确 NFR86 仍 standing  
3. 无新 deepen backlog 被静默插入  
4. 宣称纪律可核对：结项 ≠ NFR86 空  

**Out of scope：** `git push`；NFR86 实现；crates.io 新版本发布；MSRV 变更  

---

## 7. Finalize log（Step 5–6 · 2026-09-14）

- **User approval:** `yes`（Q1–Q6 默认）  
- **Status:** `approved`  
- **Scope:** Moderate → Dev 已直接落地文档戳（无新 sprint epic）  
- **Artifacts stamped:** addendum；prd.md amendment；epics `engineeringCloseout*`；README；deferred-work；AGENTS；ARCHITECTURE-SPINE pointer  
- **sprint-status.yaml:** 无新键（checklist 6.4 N/A）  
- **Handoff residual:** 可选 `git commit` / `git push`（运维；非 FR）；若要做 NFR86 → 另开 Phase 23 Correct Course  

请回复 **yes / no / revise**，并确认或改写：

1. **Q1** 本批仅结项收口，**不开** NFR86→Phase 23？  
2. **Q2** 不新增实现 epic？  
3. **Q3–Q6** 同意上文 `approval_defaults`？

若你其实要 **立刻升格 NFR86**，请回复 **revise** 并说明子集（全做 / 部分），本提案将改写为 Phase 23 加深合同。
