---
title: Sprint Change Proposal — Phase 16 产品终局结项（NFR55 未选加深升格）
date: 2026-09-11
status: approved
approved: 2026-09-11
trigger: Phase 16 FR133 / NFR55 final-closeout 合同闸门
mode: Batch
change_scope: Moderate
step4_user: Continue
step5_user: yes
related_prior: sprint-change-proposal-2026-09-10-phase15-nfr51-leftover-deepen.md (Phase 15 · approved)
epics_ref: epics.md Phase 16 Inventory · Epic 72–78 · FR133–FR140 / NFR56–NFR59
---

# Sprint Change Proposal — Phase 16 产品终局结项（NFR55 未选加深升格）

## 1. Issue Summary

**触发：** Correct Course（2026-09-11，Batch）。用户确认：在 Phase 15「NFR51 剩余升格」**已关闭**（FR124–132 / Epic 64–71）之后，批准把仍标 **NFR55 deferred**、且挡「更强完成宣称」的未选加深升格为正式 **Phase 16「产品终局结项」**（Epic 72–78 · **FR133–FR140** / **NFR56–NFR59**），使下列主题获得合同授权：

1. 真实上游 Tywaves GUI / IDE 插件深度（超出 FR125 T1–T4）  
2. 更多 IP 手写 FL（超选定 Gpio / FR126）  
3. 多外设 / 全芯片 pad 环（超出 FR128 `GpioSocPad` D1–D4）  
4. 完整外部 CIRCT 编译 / 仿真门禁（超出 FR129 C1–C4）  
5. 恢复废弃 Scala `Parser.parse`（须再修订 AD-27；超出 FR130 Style Guide）  
6. VIP/SocPad 再细拆或跨 crate 搬迁（超出 FR131 `ip/` 协议拆）

**背景：**

1. Phase 1–15（Epic 1–71）sprint 故事/回顾/action items **全部 `done`**；`phase15Status: complete`；Phase 15 retros + action-items sweep 已接受。  
2. Phase 15 诚实边界（NFR55）与结项分析将上述主题记为 **须新合同**；不得用既有关闭面冒充「产品终局」。  
3. Create Epics（Phase 16）已在 `epics.md` 追加 Inventory + Epic 72–78（**22** 则故事）；`phase16Status: planning-complete`；`correctCoursePhase16Approved` **尚无**；**Story 72.2 / FR133** 要求本 Correct Course + PRD/addendum 授权后方可实现加深 epic。  
4. 现行 `addendum.md` **仅批准到 Phase 15**；`sprint-status.yaml` **尚无** Epic 72–78。  
5. 「产品终局结项」仍可为 **0.x**；**不等于**冲 1.0 或 backlog 永久空（结项研究；NFR15）；NFR59 未选（FSM 标签、LCOV GUI、MemRead 完整生成、非 Cargo monorepo 扫描、sby 镜像卫生等）**仍须另开合同**。

**问题陈述：** 若不正式修正 PRD/addendum 与诚实文档指针，实现侧仍受「NFR55 / 须新合同」约束，无法合法将 Epic 73–78 标 `ready`；对外也无法把「Phase 15 加深已关」与「Phase 16 产品终局加深」区分清楚。**本提案不宣称 Phase 12–15 AC 未达标，也不回滚任何既有关闭证据。**

**证据：** `epics.md` Phase 16；addendum 止于 Phase 15；`deferred-work.md` Phase-15 sweep（item-201/205/213/217/221/225）；`action-items-sweep-2026-09-11-phase15.md`；ARCHITECTURE-SPINE Deferred「Phase 15 现行加深面」滞后；结项研究（sprint 空 ≠ 终局）；用户 Explicit 触发确认 + Batch。

---

## 2. Impact Analysis

### Checklist 记录（Step 2）

#### §1 Trigger & Context
- [x] **1.1** 触发故事 **72.2**（FR133）— Correct Course + PRD 批准 Phase 16  
- [x] **1.2** 类型：**战略/合同升格** — 新需求 FR133–140；非实现失败  
- [x] **1.3** 证据：epics Phase 16、Phase 15 关闭面、deferred NFR55、结项分析、用户确认  

#### §2 Epic Impact
- [x] **2.1** Epic 1–71 **不回滚、不改 AC**  
- [x] **2.2** **已规划** Epic 72–78 获合同批准；本轮不重拆范围  
- [x] **2.3** 73–78 硬依赖 72；sprint 目前止于 71  
- [x] **2.4** 无作废 epic；NFR55 deferred 升格为 FR134–139  
- [x] **2.5** 顺序：**72 关闭前** 73–78 不得 ready；软实现序 **78 → 74/75**（同触 `ip/`）；73/76/77 可并行  

#### §3 Artifact Conflicts
- [!] **3.1 PRD** — 须追加 Phase 16 段；frontmatter amendment；权威边界增 FR133–140  
- [!] **3.2 Architecture** — Deferred「现行加深面」须区分 Phase 15 vs Phase 16；AD-27（Parser）实质修订由 Epic 77 + Story 72.4 指针门禁；外部 CIRCT 由 Epic 76 + NFR58；crate 边界由 Epic 78  
- [N/A] **3.3 UX**  
- [!] **3.4 其他** — README / deferred-work；sprint-status 播种 72–78；可选 doc-19 交叉链（Story 72.3）；AGENTS.md 卫生  

#### §4 Path Forward
- **4.1 Direct Adjustment：** **Viable（选定）** — 增补合同与追踪；不回滚代码。Effort Medium–High；Risk High（上游 GUI/IDE、全芯片 pad、外部 CIRCT、Parser/AD-27、跨 crate）  
- **4.2 Rollback：** **Not viable**  
- **4.3 MVP Review（缩 Phase 12–15）：** **Not chosen** — 既有关闭面保持  
- [x] **4.4 选定：Option 1 Direct Adjustment（Hybrid 轻量）** — 同形于 Phase 13–15 提案流程，只**追加终局加深合同**  

#### §5–6 Proposal components
- 见本文 §1–5；sprint-status 更新在 **批准后**执行（checklist 6.4）  

---

## 3. Recommended Approach

**批准 Phase 16「产品终局结项 / NFR55 未选加深升格」合同**，不回滚 Phase 12–15：

1. PRD/addendum 追加 **Phase 16**；明确 **FR133–140 / NFR56–NFR59**；Phase 12–15 关闭证据 **仍有效**（NFR56）。  
2. 对外「终局 / Tywaves GUI·IDE / 更多 IP FL / 全芯片 pad / 外部 CIRCT 门禁 / Parser 恢复 / IP 跨 crate」类宣称：**仅**可在对应 FR133–139 关闭后，按 **FR140** 勾选；**禁止**用 Phase 15 完成面冒充本批加深。  
3. 终局口径 = 本批 FR 关闭 + 诚实列出 NFR59 deferred；**不等于**冲 1.0。  
4. 实现顺序：Sprint Planning / 本提案 6.4 播种 → 从 **72.1** 起；**Epic 72 关闭前** 73–78 不得 ready；软序 **78 → 74/75**。  
5. README / deferred / 脊柱指针按 Story **72.3–72.4** 落地；AD-27（及 CIRCT/crate）加深修订在各自实现 epic 首故事引用（NFR58）。  

**不选 Rollback：** Phase 12–15 工程与诚实边界仍是基线。  
**不选「只改 sprint 不开 PRD」：** 违反 FR133 / NFR55「explicit new contract required」。

---

## 4. Detailed Change Proposals（Batch）

### 4.1 PRD `prd.md`

**Artifact:** `prds/prd-rhdl-2026-08-19/prd.md`

**OLD（摘录）:**
```yaml
updated: 2026-09-10
amendment: …; phase15-nfr51-leftover-deepen-2026-09-10
```
权威边界止于 Phase 15 FR124–132；无 Phase 16 段落指针。

**NEW:**
- `updated: 2026-09-11`
- `amendment` 追加：`phase16-nfr55-final-closeout-2026-09-11`
- 文首追加一行：*2026-09-11 追加（`phase16-nfr55-final-closeout`）：Phase 16 产品终局结项 **FR133–FR140** / **NFR56–NFR59** — Phase 12–15 关闭仍有效；终局加深宣称仅引用 FR133–139（见 addendum「Phase 16」）。*
- §0 权威边界追加：**2026-09-11 Phase 16：** FR133–140 / NFR56–59 — 不回滚 FR94–132。

**Rationale:** FR133 自指的 PRD 批准条件。

### 4.2 PRD `addendum.md`

**Artifact:** `prds/prd-rhdl-2026-08-19/addendum.md`

**OLD:** 最新节为「2026-09-10 Update — Phase 15 NFR51 剩余升格」；无 Phase 16。

**NEW — 追加节「2026-09-11 Update — Phase 16 产品终局结项（Correct Course）」：**

- **用户决议：** 批准本提案；正式 Phase 16（Epic 72–78 · FR133–FR140 / NFR56–NFR59）。  
- **公开品牌（不变）：** Bitloom / `bitloom` / `bitloom-*`；设计 crate 仍只依赖 `bitloom-prelude`（除非 FR139 另开）。  
- **与 Phase 12–15 关系：**
  - Phase 12 FR94–105 / Phase 13 FR106–115 / Phase 14 FR116–123 / Phase 15 FR124–132 **关闭证据仍有效**；**不得**改写为失败（NFR56）。  
  - Phase 16 是 **新合同** 下的 NFR55 deferred → 显式 FR，**不是**「Phase 15 AC 未达标后的补救」。  
  - deferred 升格映射：Tywaves GUI/IDE→**FR134**；更多 IP FL→**FR135**；全芯片 pad→**FR136**；外部 CIRCT 门禁→**FR137**；Parser 恢复→**FR138**；VIP/SocPad 再拆/跨 crate→**FR139**；宣称纪律→**FR140**；闸门→**FR133**。  
- **实现闸门：** Epic 72（含 FR133）关闭前，Epic 73–78 不得标 ready。触及 AD-27（FR138 **必须**再修订）及外部 CIRCT/crate 边界须在实现 epic 引用（NFR58）。软序：Epic 78 先于 Epic 74/75。  
- **终局口径：** 本批关闭 + NFR59 诚实 deferred；≠ 冲 1.0。  
- **合同指针：** `epics.md` Phase 16；本提案路径。  

**Rationale:** 满足 Story 72.2 AC；授权 72.3+。

### 4.3 Epics

**Artifact:** `epics.md`

**OLD:** `phase16Status: planning-complete`；Correct Course **pending**；Inventory 写「尚未批准」。

**NEW（批准后元数据）:**
- `correctCoursePhase16Approved: 2026-09-11`
- Inventory「Correct Course 尚未批准」→「Correct Course 已批准；实现闸门 = Epic 72 关闭」
- 故事正文 **无需重写**（22 则已齐）

**Rationale:** 规划已齐；Correct Course 只补合同戳。

### 4.4 Architecture

**Artifact:** `ARCHITECTURE-SPINE.md` Deferred / AD-27（及可选 CIRCT/crate 指针；由 Story 72.4 + 实现 epic 落地）

**OLD:**
> Phase 15「NFR51 剩余升格」合同（现行加深面；闸门 Epic 64…）

**NEW（权限）:**
- Phase 15 FR124–132 = **NFR51 加深已关闭**（基线）。  
- Phase 16 FR133–140 = **NFR55 终局加深合同**（Correct Course 2026-09-11）；关闭后方可按 FR140 宣称对应面。  
- **AD-27：** Epic 77 **必须**再修订以允许 Parser 作为产品关闭条件（FR138）；未修订不得宣称。  
- **外部 CIRCT：** 实现形状由 Epic 76 + 风险记录钉死（NFR58）。  
- **crate 边界：** Epic 78 若跨 crate 须显式合同相对 AD-6。  
- Deferred 补充：GUI/IDE→FR134；更多 IP FL→FR135；全芯片 pad→FR136；外部 CIRCT→FR137；Parser→FR138；再拆/跨 crate→FR139。  

**Rationale:** 避免脊柱「现行=仅 Phase 15」挡死终局叙事。

### 4.5 Documentation / deferred / README

**Artifacts:** README；`deferred-work.md`；可选 `docs/requirements/19`；`AGENTS.md`

**NEW（权限；Story 72.3 落地）:**
- README：明确 Phase 15 已关；Phase 16 = 产品终局结项合同（Epic 72–78）；对应宣称须引 FR133–139；诚实列出 NFR59。  
- `deferred-work.md`：将 item-201/205/213/217/221/225 等标注 **升格为 FR134–139 / Epic 73–78**（关闭前仍 deferred 实现态；合同已批准）。  
- standing honesty **保留**；去掉对本批升格项的「尚无合同」措辞。  
- 未列入本批的 deferred（自动 FSM 标签、第三方 LCOV GUI 一等、emit MemRead stub→完整生成、非 Cargo monorepo 任意路径扫描、GHA formal-sby 镜像卫生）仍须另开合同（NFR59）。  

### 4.6 Sprint status

**Artifact:** `implementation-artifacts/sprint-status.yaml`

**NEW（批准后立即或经 Sprint Planning；checklist 6.4）:**

```yaml
  # Phase 16 — NFR55 final closeout (Correct Course 2026-09-11-phase16)
  # Gate: epic-72 must be done before 73–78 may leave backlog/ready
  # Soft order: prefer epic-78 before epic-74/75 (shared ip/)
  epic-72: backlog
  72-1-epic-72-nfr14-风险记录: backlog
  72-2-correct-course-prd-批准-phase-16-fr133: backlog
  72-3-同步-readme-deferred-路线图指针-fr133-fr140: backlog
  72-4-ad-指针与-epic-72-收口-fr133-fr140-nfr58: backlog
  epic-72-retrospective: optional

  epic-73: backlog
  73-1-epic-73-nfr14-风险记录: backlog
  73-2-上游-tywaves-gui-ide-深度实现与验收-fr134: backlog
  73-3-fr134-收口与文档指针: backlog
  epic-73-retrospective: optional

  epic-74: backlog
  74-1-epic-74-nfr14-风险记录: backlog
  74-2-更多-ip-手写-fl-实现与验收-fr135: backlog
  74-3-fr135-收口与文档指针: backlog
  epic-74-retrospective: optional

  epic-75: backlog
  75-1-epic-75-nfr14-风险记录: backlog
  75-2-多外设-全芯片-pad-环实现与验收-fr136: backlog
  75-3-fr136-收口与文档指针: backlog
  epic-75-retrospective: optional

  epic-76: backlog
  76-1-epic-76-nfr14-风险记录: backlog
  76-2-外部-circt-编译-仿真门禁实现与验收-fr137: backlog
  76-3-fr137-收口与文档指针: backlog
  epic-76-retrospective: optional

  epic-77: backlog
  77-1-epic-77-nfr14-风险记录: backlog
  77-2-parser-恢复与-ad-27-修订验收-fr138: backlog
  77-3-fr138-收口与文档指针: backlog
  epic-77-retrospective: optional

  epic-78: backlog
  78-1-epic-78-nfr14-风险记录: backlog
  78-2-vip-socpad-再细拆或跨-crate-实现与验收-fr139: backlog
  78-3-fr139-收口与-phase-16-故事清单指针: backlog
  epic-78-retrospective: optional
```

**Rationale:** checklist 6.4；与 Epic 72 闸门一致。**建议**批准后先跑 `bmad-sprint-planning` 再统一播种，或本提案批准时直接写入。

### 4.7 Stories（范围）

**无既有 Phase 1–15 故事正文修改。** 新故事已在 `epics.md`（72.1–78.3）。本提案不重写 AC。

---

## 5. Implementation Handoff

**Change scope:** **Moderate**（新加深合同 + backlog 播种；不回滚；不重写既有 epic AC）

| 角色 | 职责 |
|------|------|
| **PM / 用户** | 批准本提案；确认终局口径 ≠ 1.0 |
| **PO / Sprint Planning** | 批准后 seed Epic 72–78；gate 纪律 |
| **Developer（Epic 72）** | 72.1→72.4：NFR14、落地 addendum/戳、README/deferred、脊柱指针 |
| **Developer（Epic 73–78）** | 仅在 Epic 72 `done` 后按软序实现；各 epic 独立 NFR14 |
| **Architect** | Epic 77 AD-27；Epic 76 CIRCT 运维；Epic 78 crate 边界（NFR58） |

**Success criteria:**

1. 本提案 `status: approved`；addendum Phase 16 落地；`correctCoursePhase16Approved: 2026-09-11`  
2. sprint 含 Epic 72–78（backlog）；73–78 在 72 关闭前不得 ready  
3. README/deferred 区分 Phase 15 vs 16；NFR59 诚实列出  
4. 实现从 **72.1** 开始；一 story 一 commit  

**Next after approval:** 落地 §4.1–4.3 合同戳（可并入 Story 72.2）→ `bmad-sprint-planning` → `bmad-build` Story 72.1。

---

## 6. Checklist §6 状态（批准后）

- [x] **6.1** 分析完整（本批）  
- [x] **6.2** 提案自洽  
- [x] **6.3** 用户显式批准 — **yes**（2026-09-11）  
- [x] **6.4** sprint-status 播种 Epic 72–78（backlog）— **done**  
- [x] **6.5** handoff 确认 — Moderate → PO/Sprint Planning + Dev（Epic 72）  
