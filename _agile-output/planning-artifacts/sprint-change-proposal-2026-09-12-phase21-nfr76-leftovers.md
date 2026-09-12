---
title: Sprint Change Proposal — Phase 21 NFR76 leftovers 升格
date: 2026-09-12
status: approved
approved: 2026-09-12
step5_user: yes（Q1–Q5 草案默认）
trigger: Phase 20 关闭后 NFR76 leftovers 仍须另开合同；Correct Course 升格为 Phase 21（用户确认触发 + Batch）
mode: Batch
change_scope: Major
related_prior: sprint-change-proposal-2026-09-12-phase20-nfr71-four-leftovers.md (Phase 20 · approved)
epics_ref: epics.md Phase 21 Inventory · Epic 105–110 · FR172–FR177 / NFR78–NFR82（planning-complete）
approval_defaults: |
  Q1 NFR76 四条全部升格为 FR173–FR176（全做）；
  Q2 FR173 firtool 升钉须上游 Chisel 正式配对后修订 AD-9 / Stack（禁止 unpaired bump）；
  Q3 不以改写 Phase 12–20「已关闭」为代价；NFR78 隔离；
  Q4 不得静默扩大 FR142 表面；宣称须经 FR177；
  Q5 保持现行 MSRV，除非某 NFR14 显式另开；超出本批钉死子集仍须新合同（NFR81）。
---

# Sprint Change Proposal — Phase 21 NFR76 leftovers 升格

## 1. Issue Summary

**触发：** Phase 20（FR166–171 / Epic 99–104）**合同与实现已关闭**：NFR71 四条升格 + FR171 宣称诚实。结项诚实面与 `docs/fr171-phase20-claim-honesty.md` 明确写出 **NFR76 leftovers** 仍须**另开合同**：

1. **firtool 升钉超 AD-9**（须 Chisel 正式配对 + 修订 AD-9 / Stack；≠ FR169(A) 同钉 multi-lower）  
2. **unpaired CIRCT / Chisel HEAD 二进制产品路径**（≠ FR170 document-pinned update-mainline @ 1.155.0）  
3. **更广 CIRCT/MLIR/sim 套件**（超 FR169 NFR14 钉死子集）  
4. **更深 Parser / Chisel 生态**（超 FR170 NFR14 钉死子集）  

**背景：**

1. README / deferred / FR171：Phase 20 关闭 ≠ 「NFR71 账本已空」；超出 NFR14 仍 **NFR76**。  
2. `docs/fr169-*`：option **(B)** firtool bump **显式 deferred**。  
3. `docs/fr170-*`：unpaired HEAD / firtool bump **仍 NFR76**；AD-9 **未改**。  
4. ARCHITECTURE-SPINE AD-9 / Stack：仍钉 **firtool-1.155.0 ↔ Chisel 7.14.0**；firtool-1.156.0 等配对后再改（NFR12）。  
5. `epics.md` **尚无** Phase 21 Inventory；`sprint-status` 无 Epic 105+；**尚缺** Correct Course 批准戳。

**问题陈述：** 若不另开 Phase 21 合同，则无法合法宣称上述 NFR76 加深；若在无闸门下直接升钉 / 采用 unpaired HEAD / 扩大 CIRCT·Parser 验收面，则违反 NFR76 / NFR12 / NFR75 纪律。

**本提案不宣称：** Phase 12–20 AC 未达标；不回滚 FR94–171；不把 `git push` 当成产品 FR；不在批准瞬间强制 firtool live 升钉或 HEAD 二进制发布（实钉/实发属对应实现 epic + NFR14）。

---

## 2. Impact Analysis

### Checklist 记录（Step 2 · Batch）

#### §1 Trigger & Context
- [x] **1.1** 触发：Phase 20 结项诚实面 + 用户确认 NFR76 → Phase 21（Batch）  
- [x] **1.2** 类型：**战略/合同升格** — NFR76 四条升格为 FR172–177；非失败回滚  
- [x] **1.3** 证据：`docs/fr171-*` NFR76 leftovers；fr169 option B；fr170 unpaired HEAD；AD-9 / Stack；README Phase 20；deferred-work Phase 20  

#### §2 Epic Impact
- [x] **2.1** Epic 1–104 **不回滚、不改已关闭 AC**  
- [x] **2.2** **须新增** Epic 105–110（建议编号；create-epics 锁定）；本提案批准后 stamp PRD + `correctCoursePhase21Approved`；sprint 种子 backlog（finalize）  
- [x] **2.3** 硬依赖：Epic 105（闸门）关闭前 106–110 不得 ready；FR173（升钉）建议先于或并行约束 FR174（HEAD）  
- [x] **2.4** 无作废 epic；不新增超出建议 105–110 的必做 epic（除非批准时改 Q1）  
- [x] **2.5** 软序：**105 →（106 ‖ 107 ‖ 108 ‖ 109）→ 110**；触及 AD-9 的 106/107 建议串行或明确配对依赖  

#### §3 Artifact Conflicts
- [!] **3.1 PRD** — 须追加 Phase 21 段；frontmatter amendment；权威边界增 FR172–177 / NFR78–82  
- [!] **3.2 Architecture** — Phase 21 指针；**AD-9 / Stack 必修订**当 FR173 关闭；AD-27±AD-9 当 FR174/FR176 触及；**不**改 AD-6（除非某 FR 显式另开）  
- [N/A] **3.3 UX**  
- [!] **3.4 其他** — README / deferred-work；AGENTS 卫生；firtool/Chisel 升钉运维与 CI；CIRCT sim/alloc 加深 ATDD；Parser/HEAD 门禁；**sprint-status 种子**；create-epics 写 Inventory  

#### §4 Path Forward
- **4.1 Direct Adjustment：** **Viable（选定）** — 批准 Phase 21 合同 → create-epics → 实现 Epic 105–110。Effort **High**；Risk **High**（外部 firtool/Chisel 配对、HEAD、更广 CIRCT、更深 Parser 生态；AD-9 修订）  
- **4.2 Rollback：** **Not viable** — 不回滚 Phase 17–20 已上架 / 已关加深  
- **4.3 缩 MVP / 只做子集：** **Not chosen（默认）** — 用户触发已列四条；若批准时改 Q1 再缩  
- [x] **4.4 选定：Option 1 Direct Adjustment**  

---

## 3. Recommended Approach

**批准 Phase 21「NFR76 leftovers 升格」合同**，不回滚 Phase 12–20：

1. PRD/addendum 追加 **Phase 21**；**FR172–FR177** / **NFR78–NFR82**；Phase 12–20 关闭证据 **仍有效**（NFR78）。  
2. **Q1：** NFR76 **四条全部**升格为 **FR173–FR176**（各 epic NFR14 钉死验收子集；禁止静默超子集 — NFR81）。  
3. **Q2：** **FR173** — firtool 升钉 **必须**上游 Chisel **正式配对**后修订 **AD-9 / Stack**（禁止 unpaired bump / PATH-random）。  
4. **Q3：** 不得改写 FR94–171「已关闭」（NFR78）。  
5. **Q4：** 不得静默扩大 FR142；宣称须引已关 FR（FR177 / NFR82）。  
6. **Q5：** 保持现行 MSRV，除非某 epic NFR14 显式另开；超出本批钉死子集仍须新合同（NFR81）。  
7. 实现闸门：**Epic 105** 关闭前，Epic 106–110 不得 ready。  
8. **`git push` / 远程同步不在本合同内。**  

**建议映射（create-epics 可微调编号，但不重编号已关 FR）：**

| FR | Epic（建议） | 用户结果 |
|----|------|----------|
| FR172 | 105 | 合同闸门 + 诚实边界 |
| FR173 | 106 | firtool 升钉超 AD-9（配对 + AD-9 修订） |
| FR174 | 107 | unpaired CIRCT/Chisel HEAD 产品路径 |
| FR175 | 108 | 更广 CIRCT/MLIR/sim（超 FR169 NFR14） |
| FR176 | 109 | 更深 Parser/Chisel 生态（超 FR170 NFR14） |
| FR177 | 110 | Phase 21 宣称诚实门 |

---

## 4. Detailed Change Proposals（Batch）

### 4.1 PRD `prd.md`

**Artifact:** `prds/prd-rhdl-2026-08-19/prd.md`

**NEW（批准后）：**
- `updated: 2026-09-12`（或实际批准日）
- `amendment` 追加：`phase21-nfr76-leftovers-2026-09-12`
- §0 权威边界追加：**Phase 21：** FR172–177 / NFR78–82 — 不回滚 FR166–171；NFR76 leftovers 升格；≠ `git push`。

### 4.2 PRD `addendum.md`

**NEW — 追加节「Phase 21 NFR76 leftovers 升格（Correct Course）」：**

- **用户决议：** 批准本提案；正式 Phase 21（Epic 105–110 · FR172–FR177 / NFR78–NFR82）。  
- **公开品牌（不变）：** Bitloom / `bitloom` / `bitloom-*`；设计 crate 仍只依赖 `bitloom-prelude`（AD-6）。禁止 `rhdl` / `rhdl-bits`。  
- **批准默认 Q1–Q5：** 见 frontmatter `approval_defaults`。  
- **与 Phase 12–20 关系：** 关闭仍有效（NFR78）；本批是新合同加深，**不是**「Phase 20 失败」补救叙事。  
- **实现闸门：** Epic 105 关闭前 106–110 不得 ready；软序 **105 →（106 ‖ 107 ‖ 108 ‖ 109）→ 110**。  
- **不**在本合同批准瞬间强制 firtool live 升钉 / HEAD 二进制 — 属 Epic 106 / 107（+ NFR14）。

### 4.3 `epics.md`

**现状：** Phase 20 Inventory **complete**；**无** Phase 21。  

**批准后：**
- 跑 **create-epics-and-stories** 追加 Phase 21 Inventory + Epic 105–110 故事  
- 写入 `correctCoursePhase21Approved: 2026-09-12`（或实际日）  
- Story 105.2 ATDD 须能验证本提案 `status: approved`  

### 4.4 ARCHITECTURE-SPINE / AGENTS / README / deferred

**NEW（Story 105.3–105.4 / 106–110.x）：**
- 脊柱/AGENTS：Phase 21 指针；**AD-9 / Stack** 在 FR173 关闭时修订；AD-27±AD-9 在 FR174/FR176 触及且 NFR14 要求时修订  
- README / `deferred-work.md`：区分 Phase 20 完成面 vs Phase 21；各 FR 关闭前不得宣称对应条已交付  
- FR171 文档可加指针：NFR76 leftovers → Phase 21 FR173–176（不改写 Phase 20「已关闭」）

### 4.5 Sprint status

**现状：** Epic 105–110 **未**出现在 `sprint-status.yaml`。  

**批准后 / 并行：** 经 **Sprint Planning**（或 Correct Course finalize）种子 Epic 105–110 及故事为 `backlog`。**不得**在 Epic 105 关闭前将 106–110 标 `ready-for-dev`。

### 4.6 Stories

**规划故事尚未写入** — 批准后由 create-epics 起草；实现按 `epics.md` Epic 105–110；若批准时改 Q，再改对应 AC。

---

## 5. Implementation Handoff

**Change scope: Major**（四域加深 + firtool/Chisel 配对升钉 + HEAD + AD-9 修订）

| 角色 | 职责 |
|------|------|
| PM / 用户 | 批准本提案（Q1–Q5）；确认 stamp |
| PO / Dev | PRD/addendum + epics frontmatter 戳；create-epics Phase 21；Sprint Planning 种子 |
| Developer | Epic 105 →（106 ‖ 107 ‖ 108 ‖ 109）→ 110（`bmad-build`）；一 story 一 commit；各 epic 先 NFR14 |
| Architect | 105.4 指针；106/107/109 触及 AD 时审阅 |

**Success criteria：**
1. 本提案 `status: approved` + PRD/addendum Phase 21 段可验证  
2. Epic 105 关闭后 106–110 可 ready  
3. FR173–176：各条按 NFR14 子集可勾选；FR177 诚实宣称  
4. Phase 12–20 关闭证据仍有效；未写入本批 NFR14 的更深项仍须新合同（NFR81）  
5. sprint-status 含 Epic 105–110 backlog  

**Next after approval：** stamp 本提案 + PRD/addendum + `correctCoursePhase21Approved` + sprint Epic 105–110 backlog **已落** → **create-epics Phase 21**（写 Inventory + 故事键）→ Story **105.1** NFR14 → **105.2**（验证本提案 approved）。

---

## Checklist §5–6（提案组件 · 待用户 Continue/批准）

- [x] **5.1–5.5** Issue / Impact / Approach / MVP / Handoff 已写入上文  
- [x] **6.1–6.2** 分析完整；提案可执行  
- [x] **6.3** 用户显式批准（yes · 2026-09-12 · Q1–Q5 默认）  
- [x] **6.4** sprint-status 已种子 Epic 105–110 `backlog`（故事键待 create-epics 后补全）  
- [x] **6.5** handoff 确认（Major → create-epics Phase 21 → Build 105.x；README/deferred 属 Story 105.3）
