---
title: Sprint Change Proposal — Phase 22 NFR81 leftovers 升格
date: 2026-09-12
status: approved
approved: 2026-09-12
step5_user: yes（Q1 五条全做；Q5 FR142=独立 FR183；Batch）
trigger: Phase 21 关闭后 NFR81 leftovers 仍须另开合同；Correct Course 升格为 Phase 22（用户确认触发 + Batch；Q1 五条全做；Q5 FR142 独立 FR）
mode: Batch
change_scope: Major
related_prior: sprint-change-proposal-2026-09-12-phase21-nfr76-leftovers.md (Phase 21 · approved)
epics_ref: epics.md Phase 22 Inventory · Epic 111–117 · FR178–FR184 / NFR83–NFR87（待 create-epics）
approval_defaults: |
  Q1 NFR81 五条全部升格为 FR179–FR183（全做）；
  Q2 浮动 HEAD / Handshake / Style / unpaired firtool 再升钉 / FR142 扩展各 epic 独立 NFR14 钉死子集；禁止静默超子集（→ 新 NFR86）；
  Q3 不以改写 Phase 12–21「已关闭」为代价；NFR83 隔离；
  Q4 宣称须经 FR184；不得用 Phase 21 alone 冒充本批五条；
  Q5 「扩 FR142」为独立 FR183（显式公开 API 表面扩展 + SemVer/docs）；不是静默扩大；
  Q6 保持现行 MSRV，除非某 NFR14 显式另开；超出本批钉死子集仍须新合同（NFR86）；
  Q7 FR182「无上游配对 firtool 再升钉」须修订 AD-9 明示 *unpaired product-pin* 例外纪律（≠ FR173 配对升钉；≠ FR174 optional HEAD alone）。
---

# Sprint Change Proposal — Phase 22 NFR81 leftovers 升格

## 1. Issue Summary

**触发：** Phase 21（FR172–177 / Epic 105–110）**合同与实现已关闭**：NFR76 leftovers 升格 + FR177 宣称诚实。结项诚实面与 `docs/fr177-phase21-claim-honesty.md` 明确写出 **NFR81 leftovers** 仍须**另开合同**。用户确认继续实现下列五条（**Q1：全做**）：

1. **浮动 CIRCT git HEAD**（超文档钉死 unpaired / **FR174** 钉死 **firtool-1.156.0**）  
2. **Handshake dialect** 超 **FR129** / **FR175** NFR14 子集  
3. **更深 Style Guide / linter** 超 **FR176** NFR14 子集  
4. **无上游配对的 firtool 再升钉**（超 **FR173**「须正式配对」纪律；≠ FR174 optional HEAD alone）  
5. **显式扩大 FR142 公开 API 表面**（**Q5：独立 FR**；合法表面扩展，不是静默扩大）

**背景：**

1. README / deferred / FR177：Phase 21 关闭 ≠ 「NFR76 账本已空」；超出 NFR14 仍 **NFR81**。  
2. `docs/fr174-*`：钉死 unpaired **1.156.0**；浮动 git HEAD 超钉死子集仍 NFR81。  
3. `docs/fr175-*` / FR129：Handshake / SV 加深面仍有效；超子集仍 NFR81。  
4. `docs/fr176-*`：组合生态包关闭；更深 linter/ecosystem 仍 NFR81。  
5. `docs/fr173-*` / AD-9：产品钉 **firtool-1.158.0 ↔ Chisel 7.15.0**；**禁止 unpaired product bump**（NFR80）。本批 FR182 若交付「无配对再升钉」，**必须**先修订 AD-9 写明例外。  
6. `docs/public-api-1-0-surface.md` / FR142：1.0 表面锁定；静默扩大曾被 Phase 17–21 反复禁止 — 本批以 **FR183** 显式升格。  
7. `epics.md` 已 stamp `correctCoursePhase22Approved` / `phase22Status: contract-approved`；**Phase 22 Inventory 故事正文**仍待 create-epics；`sprint-status` 已种子 Epic 111–117 `backlog`。

**问题陈述：** 若不另开 Phase 22 合同，则无法合法宣称上述 NFR81 加深；若在无闸门下直接浮动 HEAD / 扩 Handshake / 再加深 Style / unpaired 升钉 / 改 FR142 表面，则违反 NFR81 / NFR80 / FR142 纪律。

**本提案不宣称：** Phase 12–21 AC 未达标；不回滚 FR94–177；不把 `git push` 当成产品 FR；不在批准瞬间强制 live HEAD 浮动、unpaired 产品升钉或破坏性 API 发布（实钉/实发属对应实现 epic + NFR14 + SemVer 政策）。

---

## 2. Impact Analysis

### Checklist 记录（Step 2 · Batch）

#### §1 Trigger & Context
- [x] **1.1** 触发：Phase 21 结项诚实面（FR177 / NFR81）+ 用户确认五条 → Phase 22（Batch）  
- [x] **1.2** 类型：**战略/合同升格** — NFR81 五条升格为 FR178–184；非失败回滚  
- [x] **1.3** 证据：`docs/fr177-*` NFR81 leftovers；fr174/fr175/fr176/fr173；`public-api-1-0-surface.md`；README Phase 21；deferred-work Phase 21；用户 Q1=全做、Q5=FR142 独立 FR  

#### §2 Epic Impact
- [x] **2.1** Epic 1–110 **不回滚、不改已关闭 AC**  
- [x] **2.2** **须新增** Epic 111–117（建议编号；create-epics 锁定）；本提案批准后 stamp PRD + `correctCoursePhase22Approved`；sprint 种子 backlog（finalize）  
- [x] **2.3** 硬依赖：Epic 111（闸门）关闭前 112–117 不得 ready；触及 AD-9 的 112/115 建议串行或明确钉死依赖；FR183（表面扩展）须 SemVer/docs 诚实  
- [x] **2.4** 无作废 epic；不新增超出建议 111–117 的必做 epic（除非批准时改 Q1）  
- [x] **2.5** 软序：**111 →（112 ‖ 113 ‖ 114 ‖ 115 ‖ 116）→ 117**；112/115（HEAD / unpaired pin）建议串行  

#### §3 Artifact Conflicts
- [!] **3.1 PRD** — 须追加 Phase 22 段；frontmatter amendment；权威边界增 FR178–184 / NFR83–87  
- [!] **3.2 Architecture** — Phase 22 指针；**AD-9** 当 FR179/FR182 关闭时按 NFR14 修订；**AD-25** 当 FR180 触及 Handshake；**AD-27** 当 FR181 触及 Style；**FR142 / SemVer 表面文档** 当 FR183 关闭；**不**改 AD-6（除非某 FR 显式另开）  
- [N/A] **3.3 UX**（除非 FR183 表面含公开工具 UX 文档指针 — 默认 N/A）  
- [!] **3.4 其他** — README / deferred-work；AGENTS 卫生；CI 门禁 / FORCE_MISSING；`docs/public-api-1-0-surface.md` 修订或后继；**sprint-status 种子**；create-epics 写 Inventory  

#### §4 Path Forward
- **4.1 Direct Adjustment：** **Viable（选定）** — 批准 Phase 22 合同 → create-epics → 实现 Epic 111–117。Effort **High**；Risk **High**（浮动 HEAD、Handshake 方言、Style 加深、**unpaired product firtool**、公开 API SemVer）  
- **4.2 Rollback：** **Not viable** — 不回滚 Phase 17–21 已上架 / 已关加深  
- **4.3 缩 MVP / 只做子集：** **Not chosen** — 用户 Q1 已确认五条全做  
- [x] **4.4 选定：Option 1 Direct Adjustment**  

---

## 3. Recommended Approach

**批准 Phase 22「NFR81 leftovers 升格」合同**，不回滚 Phase 12–21：

1. PRD/addendum 追加 **Phase 22**；**FR178–FR184** / **NFR83–NFR87**；Phase 12–21 关闭证据 **仍有效**（NFR83）。  
2. **Q1：** NFR81 **五条全部**升格为 **FR179–FR183**（各 epic NFR14 钉死验收子集；禁止静默超子集 — **NFR86**）。  
3. **Q2 / 纪律：**  
   - **FR179** — 浮动 CIRCT git HEAD（超 FR174 文档钉死 1.156.0）；须可读版本/可复现失败语义。  
   - **FR180** — Handshake dialect 加深（超 FR129 C1–C4 / FR175 SV 子集）。  
   - **FR181** — Style Guide / linter 加深（超 FR176 组合包）。  
   - **FR182** — **无上游 Chisel 正式配对**的 firtool **产品钉**再升钉路径；**必须**修订 **AD-9** 明示 *unpaired product-pin* 例外（≠ FR173 配对升钉；≠ FR174 optional HEAD alone）。  
   - **FR183** — **显式**扩大 **FR142** 公开 API 表面（更新表面清单 + SemVer/发版诚实；禁止静默扩大叙事）。  
4. **Q3：** 不得改写 FR94–177「已关闭」（NFR83）。  
5. **Q4：** 宣称须引已关 FR（**FR184** / **NFR87**）；不得用 Phase 21 alone 冒充本批五条。  
6. **Q5：** 「扩 FR142」= **独立 FR183**（用户确认）。  
7. **Q6：** 保持现行 MSRV，除非某 epic NFR14 显式另开；超出本批钉死子集仍须新合同（**NFR86**）。  
8. 实现闸门：**Epic 111** 关闭前，Epic 112–117 不得 ready。  
9. **`git push` / 远程同步不在本合同内。**  

**建议映射（create-epics 可微调编号，但不重编号已关 FR）：**

| FR | Epic（建议） | 用户结果 |
|----|------|----------|
| FR178 | 111 | 合同闸门 + 诚实边界 |
| FR179 | 112 | 浮动 CIRCT git HEAD（超 FR174） |
| FR180 | 113 | Handshake dialect 加深（超 FR129/FR175） |
| FR181 | 114 | 更深 Style Guide / linter（超 FR176） |
| FR182 | 115 | 无上游配对 firtool 产品钉再升钉（AD-9 例外修订） |
| FR183 | 116 | 显式扩大 FR142 公开 API 表面 |
| FR184 | 117 | Phase 22 宣称诚实门 |

**建议 NFR：**

| NFR | 含义 |
|-----|------|
| NFR83 | Phase 12–21 关闭面仍有效；不得改写为失败 |
| NFR84 | Phase 22 规划故事齐（Epic 111–117）后方可宣称规划 complete |
| NFR85 | 触 AD-9 / AD-25 / AD-27 / FR142 表面须先修订再 story ready |
| NFR86 | 超出本批各 epic NFR14 钉死子集仍须新合同 |
| NFR87 | 对外宣称须经 FR184；不得 Phase 21 alone 冒充 |

---

## 4. Detailed Change Proposals（Batch）

### 4.1 PRD `prd.md`

**Artifact:** `prds/prd-rhdl-2026-08-19/prd.md`

**NEW（批准后）：**
- `updated: 2026-09-12`（或实际批准日）
- `amendment` 追加：`phase22-nfr81-leftovers-2026-09-12`
- §0 权威边界追加：**Phase 22：** FR178–184 / NFR83–87 — 不回滚 FR172–177；NFR81 leftovers 升格；≠ `git push`。

### 4.2 PRD `addendum.md`

**NEW — 追加节「Phase 22 NFR81 leftovers 升格（Correct Course）」：**

- **用户决议：** 批准本提案；正式 Phase 22（Epic 111–117 · FR178–FR184 / NFR83–NFR87）。  
- **公开品牌（不变）：** Bitloom / `bitloom` / `bitloom-*`；设计 crate 仍只依赖 `bitloom-prelude`（AD-6）。禁止 `rhdl` / `rhdl-bits`。  
- **批准默认：** 见 frontmatter `approval_defaults`（Q1 五条全做；Q5 FR142=独立 FR183）。  
- **与 Phase 12–21 关系：** 关闭仍有效（NFR83）；本批是新合同加深，**不是**「Phase 21 失败」补救叙事。  
- **实现闸门：** Epic 111 关闭前 112–117 不得 ready；软序 **111 →（112 ‖ 113 ‖ 114 ‖ 115 ‖ 116）→ 117**。  
- **不**在本合同批准瞬间强制浮动 HEAD live / unpaired 产品升钉 / crates.io 破坏性发版 — 属对应实现 epic（+ NFR14 / SemVer）。

### 4.3 `epics.md`

**现状：** Phase 21 Inventory **planning-complete**；Epic 105–110 **complete**；**无** Phase 22。  

**批准后：**
- 跑 **create-epics-and-stories** 追加 Phase 22 Inventory + Epic 111–117 故事  
- 写入 `correctCoursePhase22Approved: <批准日>`  
- 可将 `phase21Status` 对齐为 `complete`（卫生；不回滚关闭证据）  
- Story 111.2 ATDD 须能验证本提案 `status: approved`  

### 4.4 ARCHITECTURE-SPINE / AGENTS / README / deferred / FR142 表面

**NEW（Story 111.3–111.4 / 112–117.x）：**
- 脊柱/AGENTS：Phase 22 指针；AD-9 / AD-25 / AD-27 / FR142 表面按触及 FR 修订（NFR85）  
- README / `deferred-work.md`：区分 Phase 21 完成面 vs Phase 22；各 FR 关闭前不得宣称对应条已交付  
- FR177 文档加指针：NFR81 leftovers → Phase 22 FR179–183（不改写 Phase 21「已关闭」）  
- FR183：修订或后继 `docs/public-api-1-0-surface.md`（或显式 v1.x 表面补丁文档）+ SemVer 诚实  

### 4.5 Sprint status

**现状：** Epic 111–117 **未**出现在 `sprint-status.yaml`。  

**批准后 / 并行：** 经 **Sprint Planning**（或 Correct Course finalize）种子 Epic 111–117 及故事为 `backlog`。**不得**在 Epic 111 关闭前将 112–117 标 `ready-for-dev`。

### 4.6 Stories

**规划故事尚未写入** — 批准后由 create-epics 起草；实现按 `epics.md` Epic 111–117；各 epic 首故事 NFR14；一 story 一 commit。

---

## 5. Implementation Handoff

**Change scope: Major**（五域加深 + unpaired 产品 firtool + 公开 API 表面 + AD 修订）

| 角色 | 职责 |
|------|------|
| PM / 用户 | 批准本提案（Q1–Q7）；确认 stamp |
| PO / Dev | PRD/addendum + epics frontmatter 戳；create-epics Phase 22；Sprint Planning 种子 |
| Developer | Epic 111 →（112 ‖ 113 ‖ 114 ‖ 115 ‖ 116）→ 117（`bmad-build`）；一 story 一 commit；各 epic 先 NFR14 |
| Architect | 111.4 指针；112/115 AD-9；113 AD-25；114 AD-27；116 FR142/SemVer 审阅 |

**Success criteria：**
1. 本提案 `status: approved` + PRD/addendum Phase 22 段可验证  
2. Epic 111 关闭后 112–117 可 ready  
3. FR179–183：各条按 NFR14 子集可勾选；FR184 诚实宣称  
4. Phase 12–21 关闭证据仍有效；未写入本批 NFR14 的更深项仍须新合同（NFR86）  
5. sprint-status 含 Epic 111–117 backlog  

**Next after approval：** stamp 本提案 + PRD/addendum + `correctCoursePhase22Approved` + sprint Epic 111–117 backlog **已落** → **create-epics Phase 22**（写 Inventory + 故事键）→ Story **111.1** NFR14 → **111.2**（验证本提案 approved）。

---

## Checklist §5–6（提案组件）

- [x] **5.1–5.5** Issue / Impact / Approach / MVP / Handoff 已写入本文  
- [x] **6.x** 用户批准（yes · 2026-09-12）；`status: approved`；PRD/addendum/epics stamp + sprint 种子已落  

---

**Workflow complete:** Major handoff → **create-epics Phase 22**（Epic 111–117 · FR178–184）→ Story **111.1** NFR14。
