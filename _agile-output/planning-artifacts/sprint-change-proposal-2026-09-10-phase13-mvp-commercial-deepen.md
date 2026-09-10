---
title: Sprint Change Proposal — Phase 13 MVP→商业加深
date: 2026-09-10
status: approved
approved: 2026-09-10
trigger: Phase 13 FR106 / MVP→商业加深合同闸门
mode: Batch
change_scope: Moderate
related_prior: sprint-change-proposal-2026-09-09-phase12-path-b.md (Phase 12 字面绿 · approved)
epics_ref: epics.md Phase 13 Inventory · Epic 48–56 · FR106–FR115 / NFR44–NFR47
---

# Sprint Change Proposal — Phase 13 MVP→商业加深

## 1. Issue Summary

**触发：** Correct Course（2026-09-10，Batch）。用户确认：在 Phase 12 Path B **字面绿 MVP（FR94–105）已结项**之后，批准 **「MVP→商业加深」** 为正式 **Phase 13**（Epic 48–56 · **FR106–FR115** / **NFR44–NFR47**），使 `deferred-work.md` 中 Phase 12 **optional product** 加深面获得合同授权。

**背景：**

1. Phase 1–12（Epic 1–47）sprint 故事/回顾/action items **全部 `done`**；`phase12Status: complete`；Phase 12 retros 已接受。
2. Phase 12 Correct Course 将「七阶段字面全绿」定义为 **MVP 深度可关闭**（NFR42）；诚实边界将 AT、GPIO、C3 FSM、HLS/Chisel/formal 商业深度、LSP 根发现、Tywaves/LCOV 等记为 **optional / 须新合同**。
3. Create Epics（Phase 13）已在 `epics.md` 追加 Inventory + Epic 48–56（28 则故事）；`phase13Status: complete`（规划）；**Story 48.2 / FR106** 要求本 Correct Course + PRD/addendum 授权后方可实现加深 epic。
4. 现行 `addendum.md` **仅批准到 Phase 12**；脊柱 Deferred「现行完成标签」仍写 Phase 12；`sprint-status.yaml` **尚无** Epic 48–56。

**问题陈述：** 若不正式修正 PRD/addendum 与诚实文档指针，实现侧仍受「optional / 须新合同」约束，无法合法将 Epic 49–56 标 `ready`；对外也无法把「Phase 12 MVP 已关」与「Phase 13 商业加深进行中/已关」区分清楚。**本提案不宣称 Phase 12 AC 未达标，也不回滚任何 Phase 12 关闭证据。**

**证据：** `epics.md` Phase 13；addendum「Phase 12 字面绿」止于 FR94–105；`deferred-work.md` optional（items 117/121/125/129 等）；ARCHITECTURE-SPINE Deferred 现行标签；用户 Explicit 触发确认 + Batch + §1–4 分析同意。

---

## 2. Impact Analysis

### Checklist 记录（Step 2）

#### §1 Trigger & Context
- [x] **1.1** 触发故事 **48.2**（FR106）— Correct Course + PRD 批准 Phase 13
- [x] **1.2** 类型：**战略/合同升格** — 新需求 FR106–115；非实现失败
- [x] **1.3** 证据：epics Phase 13、Phase 12 关闭面、deferred optional、用户确认

#### §2 Epic Impact
- [x] **2.1** Epic 1–47 **不回滚、不改 AC**
- [x] **2.2** **已规划** Epic 48–56 获合同批准；本轮不重拆范围
- [x] **2.3** 49–56 硬依赖 48；sprint 目前止于 47
- [x] **2.4** 无作废 epic；optional 升格为 FR107–114
- [x] **2.5** 顺序：**48 关闭前** 49–56 不得 ready；其后可按容量并行

#### §3 Artifact Conflicts
- [!] **3.1 PRD** — 须追加 Phase 13 段；frontmatter amendment；权威边界增 FR106–115
- [!] **3.2 Architecture** — Deferred「现行完成标签」须区分 Phase 12 MVP vs Phase 13 加深；AD-5/25/27 实质修订由实现 epic（49/52/53）+ Story 48.4 指针门禁
- [N/A] **3.3 UX**
- [!] **3.4 其他** — README / deferred-work；sprint-status 播种 48–56；可选 doc-19 交叉链（Story 48.3）

#### §4 Path Forward
- **4.1 Direct Adjustment：** **Viable（选定）** — 增补合同与追踪；不回滚代码。Effort Low–Medium；Risk Low
- **4.2 Rollback：** **Not viable**
- **4.3 MVP Review（缩 Phase 12）：** **Not chosen** — Phase 12 MVP 保持关闭
- [x] **4.4 选定：Option 1 Direct Adjustment（Hybrid 轻量）** — 同形于 Phase 12 提案流程，但**不推翻**既有完成口径，只**追加加深合同**

#### §5–6 Proposal components
- 见本文 §1–5；sprint-status 更新在 **批准后**执行（checklist 6.4）

---

## 3. Recommended Approach

**批准 Phase 13「MVP→商业加深」合同**，不回滚 Phase 12：

1. PRD/addendum 追加 **Phase 13 MVP→商业加深**；明确 **FR106–115 / NFR44–NFR47**；Phase 12 FR94–105 **关闭证据仍有效**（NFR44）。
2. 对外「商业加深 / 非 MVP」类宣称：**仅**可在对应 FR106–114 关闭后，按 **FR115** 勾选；**禁止**用 Phase 12 MVP 冒充商业完整面。
3. 实现顺序：Sprint Planning / 本提案 6.4 播种 → 从 **48.1** 起；**Epic 48 关闭前** 49–56 不得 ready。
4. README / deferred / 脊柱指针按 Story **48.3–48.4** 落地（本提案批准其合同权限）；AD-5/25/27 加深修订在各自实现 epic 首故事引用（NFR46）。

**不选 Rollback：** Phase 12 MVP 工程与诚实边界仍是基线。  
**不选「只改 sprint 不开 PRD」：** 违反 FR106 / deferred「explicit new contract required」。

---

## 4. Detailed Change Proposals（Batch）

### 4.1 PRD `prd.md`

**Artifact:** `prds/prd-rhdl-2026-08-19/prd.md`

**OLD（摘录）:**
```yaml
updated: 2026-09-09
amendment: …; phase12-literal-green-path-b-2026-09-09
```
权威边界止于 Phase 12 FR94–105；无 Phase 13 段落指针。

**NEW:**
- `updated: 2026-09-10`
- `amendment` 追加：`phase13-mvp-commercial-deepen-2026-09-10`
- 文首追加一行：*2026-09-10 追加（`phase13-mvp-commercial-deepen`）：Phase 13 MVP→商业加深 **FR106–FR115** / **NFR44–NFR47** — Phase 12 字面绿 MVP 关闭仍有效；加深宣称仅引用 FR106–114（见 addendum「Phase 13」）。*
- §0 权威边界追加一条：**2026-09-10 Phase 13：** FR106–115 / NFR44–47 — 不回滚 FR94–105。

**Rationale:** FR106 自指的 PRD 批准条件。

### 4.2 PRD `addendum.md`

**Artifact:** `prds/prd-rhdl-2026-08-19/addendum.md`

**OLD:** 最新节为「2026-09-09 Update — Phase 12 字面绿（Path B）」；无 Phase 13。

**NEW — 追加节「2026-09-10 Update — Phase 13 MVP→商业加深（Correct Course）」：**

- **用户决议：** 批准本提案；正式 Phase 13（Epic 48–56 · FR106–FR115 / NFR44–NFR47）。
- **公开品牌（不变）：** Bitloom / `bitloom` / `bitloom-*`；设计 crate 仍只依赖 `bitloom-prelude`。
- **与 Phase 12 关系：**
  - Phase 12 FR94–105 / NFR40–43 **关闭证据仍有效**；**不得**改写为失败（NFR44）。
  - Phase 13 加深 **不得**冒充「Phase 12 AC 未达标后的补救」；是 **新合同** 下的可选→必选升格。
  - deferred optional 升格映射：AT→**FR107**；GPIO→**FR108**；C3 FSM→**FR109**；HLS 深度→**FR110**；Chisel 深度→**FR111**；formal/dual-model 深度→**FR112**；LSP 根发现→**FR113**；Tywaves/LCOV GUI→**FR114**；宣称纪律→**FR115**；闸门→**FR106**。
- **实现闸门：** Epic 48（含 FR106）关闭前，Epic 49–56 不得标 ready。触及 AD-5/25/27 的加深须在实现 epic 引用修订 AD（NFR46）。
- **合同指针：** `epics.md` Phase 13；本提案路径。

**Rationale:** 满足 Story 48.2 AC；授权 48.3+。

### 4.3 Epics

**Artifact:** `epics.md`

**OLD:** `phase13Status: complete`（规划）；`phase13Epic48–56Status: backlog`；注明 FR106 闸门未落地。

**NEW（批准后元数据）:**
- `correctCoursePhase13Approved: 2026-09-10`
- `phase13Contract: mvp-to-commercial-deepen`（已有则可保留）
- 故事正文 **无需重写**
- Inventory 中「FR106 闸门未落地」改为「Correct Course 已批准；实现闸门 = Epic 48 关闭」

**Rationale:** 规划已齐；Correct Course 只补合同戳。

### 4.4 Architecture

**Artifact:** `ARCHITECTURE-SPINE.md` Deferred 段（及其他指针，由 Story 48.4 落地）

**OLD:**
> 产品「全绿 / 七阶段字面完成」标签（现行）：Phase 12 FR94–FR105 …

**NEW（权限；Story 48.4 可直接改或写「实现 epic 内修订」门禁）:**
- Phase 12 FR94–105 = **字面绿 MVP 已关闭**（历史+现行基线）。
- Phase 13 FR106–115 = **商业/可选加深合同**（Correct Course 2026-09-10）；关闭后方可按 FR115 宣称对应加深面。
- AD-5 / AD-25 / AD-27：**允许**在 Epic 49 / 52 / 53 内进一步修订以覆盖 AT、HLS 商业深度、idiomatic 加严；**禁止**未引用修订 AD 即宣称 FR107/110/111 关闭（NFR46）。
- Deferred 中「SystemC TLM 实现形状仅 Epic 46」等句：补充「AT / nb_transport → FR107 / Epic 49」。

**Rationale:** 避免脊柱「现行=仅 Phase 12」挡死加深叙事；实质 AD  diff 可跟实现 epic。

### 4.5 Documentation / deferred / README

**Artifacts:** README「状态与 deferred」；`deferred-work.md`；可选 `docs/requirements/19`

**OLD:** optional product「explicit new contract required」；README 以 Phase 12 为当前完成口径，未区分 Phase 13。

**NEW（权限；Story 48.3 落地）:**
- README：明确 Phase 12 MVP 已关；Phase 13 = 加深合同（Epic 48–56）；「商业加深」须引 FR106–114。
- `deferred-work.md`：将 item-117/121/125/129 等 optional 条标注 **升格为 FR107–114 / Epic 49–56**（关闭前仍 deferred 实现态；合同已批准）。
- standing honesty（MVP ≠ 商业完整）**保留**；仅去掉「尚无合同」措辞。
- doc-19：可选交叉链 Phase 13（不重定义 Phase 12 字面绿勾选）。

### 4.6 Sprint status

**Artifact:** `implementation-artifacts/sprint-status.yaml`

**NEW（批准后立即或经 Sprint Planning；checklist 6.4）:**

```yaml
  # Phase 13 — MVP→commercial deepen (Correct Course 2026-09-10-phase13)
  # Gate: epic-48 must be done before 49–56 may leave backlog/ready
  epic-48: backlog
  48-1-epic-48-nfr14-风险记录: backlog
  48-2-correct-course-prd-批准-phase13-fr106: backlog
  48-3-同步-readme-deferred-路线图-fr106-fr115: backlog
  48-4-ad-指针与-epic-48-收口-fr106: backlog
  epic-48-retrospective: optional

  epic-49: backlog
  49-1-epic-49-nfr14-风险记录: backlog
  49-2-at-style-nb-transport-产品路径-fr107: backlog
  49-3-fr107-收口与文档边界: backlog
  epic-49-retrospective: optional

  epic-50: backlog
  50-1-epic-50-nfr14-风险记录: backlog
  50-2-gpio-近-vip-实现-fr108: backlog
  50-3-fr108-收口与文档边界: backlog
  epic-50-retrospective: optional

  epic-51: backlog
  51-1-epic-51-nfr14-风险记录: backlog
  51-2-fsm-state-visit-覆盖率-fr109: backlog
  51-3-fr109-收口与文档边界: backlog
  epic-51-retrospective: optional

  epic-52: backlog
  52-1-epic-52-nfr14-风险记录: backlog
  52-2-树内-hls-商业深度-fr110: backlog
  52-3-fr110-收口与文档边界: backlog
  epic-52-retrospective: optional

  epic-53: backlog
  53-1-epic-53-nfr14-风险记录: backlog
  53-2-idiomatic-chisel-可维护深度-fr111: backlog
  53-3-fr111-收口与文档边界: backlog
  epic-53-retrospective: optional

  epic-54: backlog
  54-1-epic-54-nfr14-风险记录: backlog
  54-2-形式等价-双模型深度-fr112: backlog
  54-3-fr112-收口与文档边界: backlog
  epic-54-retrospective: optional

  epic-55: backlog
  55-1-epic-55-nfr14-风险记录: backlog
  55-2-lsp-设计根发现加深-fr113: backlog
  55-3-fr113-收口与文档边界: backlog
  epic-55-retrospective: optional

  epic-56: backlog
  56-1-epic-56-nfr14-风险记录: backlog
  56-2-富波形-覆盖率-gui-加深-fr114: backlog
  56-3-fr114-收口与文档边界: backlog
  epic-56-retrospective: optional
```

（键名可按仓库 slug 惯例微调；状态一律 `backlog`，直至 Epic 48 关闭后 49–56 方可 ready。）

**Rationale:** checklist 6.4；否则 Build 无追踪面。

### 4.7 UX
**N/A**

### 4.8 Code / CI
**本提案不直接改产品代码。** 实现从 48.1 起；CI 扩展随各 FR 故事。  
**例外（批准后可立即执行、属合同落地）：** PRD/addendum 戳记、epics 元数据、sprint-status 种子 — 可由 PO/Dev 在批准后一次性合入，或并入 Story 48.2 交付（二选一须在批准时裁定；**默认：批准后立即合入 PRD/addendum + sprint 种子；README/deferred/AD 跟 48.3–48.4**）。

---

## 5. Implementation Handoff

**Change scope: Moderate**（新阶段合同 + backlog 播种；不回滚已交付；多年加深面由后续 epic 承担，非本提案一次交付）

| 角色 | 职责 |
|------|------|
| **PM / 用户（Richard）** | 批准本提案 |
| **PO / Dev** | 批准后：合入 PRD/addendum + epics 戳记；更新 sprint-status 48–56；执行 Story 48.1–48.4 |
| **Architect** | Story 48.4 脊柱指针；Epic 49/52/53 内修订 AD-5/25/27 |
| **Developer agents** | Epic 48 关闭后实现 49–56 |

**Success criteria:**
1. 本提案 `status: approved`
2. addendum Phase 13 段落合入；NFR44 边界可检查
3. sprint-status 含 Epic 48–56 backlog
4. Story 48.1–48.4 可实施；其后 49–56 闸门仅依赖 Epic 48 done

**Non-goals of this proposal:** 不在本文件内实现 AT / GPIO / HLS 深度等代码。

---

## 6. Checklist §6 预留

- [x] **6.1–6.2** 分析与提案已形成
- [x] **6.3** 用户显式批准（yes · 2026-09-10）
- [x] **6.4** 批准后更新 sprint-status（Epic 48–56 backlog）
- [x] **6.5** 交接确认（Moderate → PO/Dev + Architect；实现自 48.1）

---

**Workflow complete.** 实现从 Story **48.1** 起；README/deferred/AD 跟 48.3–48.4。
