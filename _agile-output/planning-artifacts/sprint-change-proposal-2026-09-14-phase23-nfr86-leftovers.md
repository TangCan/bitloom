---
title: Sprint Change Proposal — Phase 23 NFR86 leftovers 升格
date: 2026-09-14
status: approved
approved: 2026-09-14
step5_user: yes（Q1 五条全做；Q2–Q8 默认；Batch）
trigger: 工程结项后用户要求继续解决 NFR86 五项；Correct Course 升格为 Phase 23（Batch；Q1 五条全做）
mode: Batch
change_scope: Major
related_prior: |
  sprint-change-proposal-2026-09-12-phase22-nfr81-leftovers.md (Phase 22 · approved · complete);
  sprint-change-proposal-2026-09-14-engineering-closeout.md (engineering closeout · approved · NFR86 standing)
epics_ref: epics.md Phase 23 Inventory · Epic 118–124 · FR185–FR191 / NFR88–NFR92
approval_defaults: |
  Q1 NFR86 五条全部升格为 FR186–FR190（全做）；
  Q2 各条独立 epic + NFR14 钉死子集；禁止静默超子集（→ 新 NFR91）；
  Q3 不以改写 Phase 12–22「已关闭」/ 工程结项为代价；NFR88 隔离；
  Q4 宣称须经 FR191；不得用 Phase 22 alone 或工程结项 alone 冒充本批五条；不得宣称「NFR86 账本已空」直至本批关闭且诚实门勾选；
  Q5 「继续扩 FR142」为独立 FR190（显式表面扩展 + SemVer/docs）；不是静默扩大；
  Q6 保持现行 MSRV，除非某 NFR14 显式另开；超出本批钉死子集仍须新合同（NFR91）；
  Q7 触及 AD-9 / AD-25 / AD-27 / FR142 须按 NFR90 先修订再 story ready；
  Q8 git push / 远程同步不在本合同内。
---

# Sprint Change Proposal — Phase 23 NFR86 leftovers 升格

## 1. Issue Summary

**触发：** Correct Course（2026-09-14，Batch）。工程/合同结项收口已批准，但诚实面仍保留 **NFR86**。用户明确要求继续解决下列五项（与 `docs/fr184-phase22-claim-honesty.md` NFR86 leftovers 对齐）：

1. **无界 CIRCT tip**（超 FR179 文档钉死浮动轨 **firtool-1.159.0** / cache 渠道）  
2. **更深 Handshake dialect / lower**（超 FR180 fork+join）  
3. **社区 Style Guide 全家桶**（超 FR181 wartremover + fatal-warnings）  
4. **继续 firtool 产品钉升钉**（超 FR182 **1.159.0** unpaired 产品钉）  
5. **继续显式扩 FR142 公开 API 表面**（超 FR183 `bitloom-firrtl` interop 入面）

**背景：**

1. Phase 22（FR178–184 / Epic 111–117）**已关闭**；工程结项 `engineeringCloseoutStatus: complete`。  
2. 结项与 FR184 **明确禁止**宣称「NFR81/NFR86 账本已空」；超出 Phase 22 NFR14 仍 **NFR86**。  
3. sprint backlog 当前为空；无 Phase 23 Inventory / sprint 种子。  
4. 若不另开合同，不得合法实现或宣称上述五条。

**问题陈述：** 结项 alone ≠ 授权 NFR86 实现。须新 Phase 23 合同闸门 + 五条加深 FR + 宣称诚实门，否则违反 NFR86 / 结项纪律。

**本提案不宣称：** Phase 12–22 AC 未达标；不回滚 FR94–184；不把 `git push` 当 FR；不在批准瞬间强制 live tip / 破坏性 API 发版 / 任意 firtool 升钉（实钉属对应实现 epic + NFR14 + SemVer / AD 修订）。

**证据：** fr184 NFR86 leftovers；deferred Engineering closeout；用户「请继续解决这个」；Phase 22 关闭面。

---

## 2. Impact Analysis

### Checklist（Step 2 · Batch）

#### §1 Trigger & Context
- [x] **1.1** 触发：结项后 NFR86 五项 + 用户继续解决  
- [x] **1.2** 类型：**战略/合同升格** — NFR86 → FR185–191  
- [x] **1.3** 证据：fr184；结项提案；Phase 22 关闭  

#### §2 Epic Impact
- [x] **2.1** Epic 1–117 **不回滚**  
- [x] **2.2** **须新增** Epic 118–124（建议）；批准后 stamp + create-epics + sprint 种子  
- [x] **2.3** 硬依赖：Epic 118（闸门）关闭前 119–124 不得 ready；触 AD-9 的 119/122 建议串行  
- [x] **2.4** 无作废 epic；NFR86 五条升格为本批 FR  
- [x] **2.5** 软序：**118 →（119 ‖ 120 ‖ 121 ‖ 122 ‖ 123）→ 124**  

#### §3 Artifact Conflicts
- [!] **3.1 PRD** — Phase 23 段；amendment；权威边界 FR185–191 / NFR88–92  
- [!] **3.2 Architecture** — Phase 23 指针；AD-9/25/27/FR142 按触及 FR 修订（NFR90）  
- [N/A] **3.3 UX**（默认）  
- [!] **3.4** README / deferred / AGENTS；sprint 种子；create-epics；各 FR 文档  

#### §4 Path Forward
- **4.1 Direct Adjustment：** **Viable（选定）** — Effort **High**；Risk **High**  
- **4.2 Rollback：** **Not viable**  
- **4.3 只做子集：** **默认不选**（Q1 全做；用户可 revise）  
- [x] **4.4 Option 1 Direct Adjustment**  

---

## 3. Recommended Approach

**批准 Phase 23「NFR86 leftovers 升格」**，不回滚 Phase 12–22 / 工程结项：

1. PRD/addendum 追加 **Phase 23**；**FR185–FR191** / **NFR88–NFR92**；Phase 12–22 + 工程结项 **仍有效**（**NFR88**）。  
2. **Q1：** NFR86 **五条全部** → **FR186–FR190**（各 epic NFR14；禁止静默超子集 — **NFR91**）。  
3. **纪律摘要：**  
   - **FR186** — 无界 / live CIRCT tip（超 FR179 文档钉死浮动轨）；须可读 tip 身份 / 可复现失败 / 诚实「非产品默认钉」。  
   - **FR187** — Handshake dialect/lower 加深（超 FR180 fork+join）；须修订 **AD-25**（NFR90）。  
   - **FR188** — 社区 Style Guide 全家桶（超 FR181）；须修订 **AD-27**（NFR90）。  
   - **FR189** — 继续 firtool 产品钉升钉（超 FR182 1.159.0）；配对或 unpaired 例外须修订 **AD-9**（NFR90）。  
   - **FR190** — 继续显式扩 FR142（超 FR183）；更新表面清单 + SemVer；禁止静默扩大。  
4. **宣称：** 须经 **FR191**（**NFR92**）；不得用 Phase 22 / 结项 alone 冒充；本批关闭后仍不得静默宣称「NFR86 账本已空」若超本批 NFR14（→ **NFR91**）。  
5. 闸门：**Epic 118** 关闭前 119–124 不得 ready。  
6. **`git push` 不在本合同内。**

**建议映射（create-epics 可微调编号，不重编号已关 FR）：**

| FR | Epic | 内容 |
|----|------|------|
| FR185 | 118 | Phase 23 合同闸门 |
| FR186 | 119 | 无界 CIRCT tip（超 FR179） |
| FR187 | 120 | Handshake lower/dialect 加深（超 FR180） |
| FR188 | 121 | 社区 Style Guide 全家桶（超 FR181） |
| FR189 | 122 | 继续 firtool 产品钉升钉（超 FR182） |
| FR190 | 123 | 继续显式扩 FR142（超 FR183） |
| FR191 | 124 | Phase 23 宣称诚实门 |

**建议 NFR：**

| NFR | 含义 |
|-----|------|
| NFR88 | Phase 12–22 + 工程结项关闭面仍有效 |
| NFR89 | Phase 23 规划故事齐后方可宣称规划 complete |
| NFR90 | 触 AD-9 / AD-25 / AD-27 / FR142 须先修订再 ready |
| NFR91 | 超出本批各 epic NFR14 仍须新合同 |
| NFR92 | 宣称须经 FR191；不得 Phase 22/结项 alone 冒充 |

---

## 4. Detailed Change Proposals（Batch）

### 4.1–4.2 PRD
- `prd.md`：`amendment` 追加 `phase23-nfr86-leftovers-2026-09-14`；权威边界 Phase 23  
- `addendum.md`：追加「Phase 23 NFR86 leftovers 升格」段（镜像 Phase 22 结构）

### 4.3 epics.md
- 批准后：`correctCoursePhase23Approved`；`phase23Status: contract-approved` → create-epics → planning-complete  
- Inventory Epic 118–124 / FR185–191 / NFR88–92  

### 4.4 Architecture / README / deferred / AGENTS
- Phase 23 指针；各加深 FR 关闭时修订触及的 AD；fr184 加「→ Phase 23」指针（不改写 Phase 22 已关闭）

### 4.5 sprint-status
- 种子 Epic 118–124 + 故事 `backlog`；118 关闭前不得 ready 119–124  

### 4.6 Stories
- 批准后 create-epics；各 epic 首故事 NFR14；一 story 一 commit  

---

## 5. Implementation Handoff

**Change scope: Major**

| 角色 | 职责 |
|------|------|
| PM / 用户 | 批准 Q1–Q8（或 revise 子集） |
| PO / Dev | stamp PRD/epics；create-epics；sprint 种子 |
| Developer | 118 →（119‖…‖123）→ 124；`bmad-build`；一 story 一 commit |
| Architect | AD-9/25/27 / FR142 审阅 |

**Success criteria：**
1. 提案 approved + addendum Phase 23 可验证  
2. Epic 118 关闭后 119–124 可 ready  
3. FR186–190 按 NFR14 可勾选；FR191 诚实宣称  
4. Phase 12–22 / 结项仍有效；超本批仍 NFR91  
5. sprint 含 Epic 118–124  

**Next after approval：** stamp 本提案 + PRD/addendum + `correctCoursePhase23Approved` + sprint Epic 118–124 backlog **已落** → **create-epics Phase 23**（Inventory + 故事已写入）→ Story **118.1** NFR14。

---

## 7. Finalize log（Step 5–6 · 2026-09-14）

- **User approval:** `yes`（Q1 五条全做；Q2–Q8 默认）  
- **Status:** `approved`  
- **Scope:** Major  
- **Artifacts stamped:** addendum；prd.md；epics Phase 23 Inventory + frontmatter；sprint Epic 118–124 `backlog`；AGENTS / deferred / README / SPINE / fr184 指针  
- **Handoff residual:** Build Story **118.1** NFR14 → **118.2** 验证本提案 approved；软序 118→（119‖…‖123）→124  
