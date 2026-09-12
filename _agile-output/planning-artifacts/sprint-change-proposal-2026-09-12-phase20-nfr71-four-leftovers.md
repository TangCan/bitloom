---
title: Sprint Change Proposal — Phase 20 NFR71 四条升格
date: 2026-09-12
status: approved
approved: 2026-09-12
step5_user: yes（Q1–Q5 草案默认）
trigger: Phase 19 关闭后 NFR71 四条 leftover 仍须另开合同；create-epics Phase 20 规划已齐待 Correct Course 批准
mode: Batch
change_scope: Major
related_prior: sprint-change-proposal-2026-09-12-phase19-nfr59-fr152a.md (Phase 19 · approved)
epics_ref: epics.md Phase 20 Inventory · Epic 99–104 · FR166–FR171 / NFR73–NFR77
approval_defaults: |
  Q1 NFR71 四条全部升格为 FR167–FR170（全做）；
  Q2 FR168：SPI + I2C + AXI 三者皆交付手写 FL ≡ tick（非「至少一项」）；
  Q3 不以改写 Phase 12–19「已关闭」为代价；NFR73 隔离；
  Q4 不得静默扩大 FR142 表面；宣称须经 FR171；
  Q5 保持现行 MSRV；firtool 升钉须 Chisel 正式配对后修订 AD-9（NFR12 / NFR75）。
---

# Sprint Change Proposal — Phase 20 NFR71 四条升格

## 1. Issue Summary

**触发：** Phase 19（FR154–165 / Epic 87–98）**合同与实现已关闭**：NFR59 九条升格 + `bitloom-lsp` live crates.io + FR156 宣称诚实。结项诚实面与 `docs/fr156-phase19-claim-honesty.md` 明确写出 **NFR71 leftovers** 仍须**另开合同**：

1. **完整 ChiselSim / 额外 IDE 商店多端**（超 FR162 / FR134）  
2. **SPI / I2C / AXI 手写 FL**（超 FR163 `UartRx`）  
3. **更广 CIRCT/MLIR allocation / firtool 升钉**（超 FR164；须 AD-9 配对）  
4. **任意 Chisel HEAD Parser 回迁**（超 FR165 / FR138；须 AD-27±AD-9）  

**背景：**

1. README / deferred / FR156 诚实面：Phase 19 关闭 ≠ NFR71 账本已空。  
2. `docs/fr162` / `fr163` / `fr164` / `fr165` Non-goals 与 Epic 94–98 retro action 指向同一四条。  
3. `epics.md` 已完成 Phase 20 规划（`phase20Status: planning-complete`；Epic **99–104** / **19** 故事）。  
4. `sprint-status.yaml` **尚未**种子 Epic 99–104（与 Phase 19 批准时已种子不同）。  
5. **尚缺** Correct Course + PRD/addendum 批准戳（Story 99.2）。

**问题陈述：** 若不另开 Phase 20 合同，则无法合法宣称上述四条加深；若在无闸门下直接实现/升钉/多商店发布，则违反 NFR71 / NFR12 / NFR75 纪律。

**本提案不宣称：** Phase 12–19 AC 未达标；不回滚 FR94–165；不把 `git push` 当成产品 FR；不在批准瞬间强制商店 live publish 或 firtool 升钉（实发/升钉属对应实现 epic + NFR14）。

---

## 2. Impact Analysis

### Checklist 记录（Step 2 · Batch）

#### §1 Trigger & Context
- [x] **1.1** 触发：Phase 19 结项诚实面 + create-epics Phase 20（用户确认；Batch）  
- [x] **1.2** 类型：**战略/合同升格** — NFR71 四条升格为 FR166–171；非失败回滚  
- [x] **1.3** 证据：`docs/fr156-*` NFR71 leftovers；fr162–165 Non-goals；Epic 94–98 retro；README Phase 19；`epics.md` Phase 20 Inventory  

#### §2 Epic Impact
- [x] **2.1** Epic 1–98 **不回滚、不改已关闭 AC**  
- [x] **2.2** **已规划** Epic 99–104；本提案批准后 stamp PRD + `correctCoursePhase20Approved`；sprint **已种子** backlog（Correct Course finalize）  
- [x] **2.3** 硬依赖：Epic 99（闸门）关闭前 100–104 不得 ready  
- [x] **2.4** 无作废 epic；不新增超出 99–104 的必做 epic  
- [x] **2.5** 软序：**99 →（100 ‖ 101 ‖ 102 ‖ 103）→ 104**；Epic 101（`ip/`）串行；Epic 102 升钉配对建议先于 Epic 103  

#### §3 Artifact Conflicts
- [!] **3.1 PRD** — 须追加 Phase 20 段；frontmatter amendment；权威边界增 FR166–171 / NFR73–77  
- [!] **3.2 Architecture** — Phase 20 指针；触及 AD-9/AD-25/AD-27 时由 Epic 102/103（及 100 商店）NFR14+NFR75 修订；**不**改 AD-6（除非某 FR 显式另开）  
- [N/A] **3.3 UX**  
- [!] **3.4 其他** — README / deferred-work；AGENTS 卫生；IDE 商店凭证/CI；firtool/Chisel 升钉运维；`ip/` FL ATDD；各加深域 ATDD；**sprint-status 种子**  

#### §4 Path Forward
- **4.1 Direct Adjustment：** **Viable（选定）** — 批准 Phase 20 合同 → 实现 Epic 99–104。Effort **High**；Risk **High**（ChiselSim/多商店、三协议 FL、CIRCT/firtool 升钉、HEAD Parser；外部依赖 + AD 修订）  
- **4.2 Rollback：** **Not viable** — 不回滚 Phase 17–19 已上架 / 已关加深  
- **4.3 缩 MVP / 只做子集：** **Not chosen** — 用户 create-epics + Step 1 已锁 **四条全做**；FR168 = **三者皆交付**  
- [x] **4.4 选定：Option 1 Direct Adjustment**  

---

## 3. Recommended Approach

**批准 Phase 20「NFR71 四条升格」合同**，不回滚 Phase 12–19：

1. PRD/addendum 追加 **Phase 20**；**FR166–FR171** / **NFR73–NFR77**；Phase 12–19 关闭证据 **仍有效**（NFR73）。  
2. **Q1：** NFR71 **四条全部**升格为 **FR167–FR170**（各 epic NFR14 钉死验收子集；禁止静默超子集 — NFR76）。  
3. **Q2：** **FR168** — SPI + I2C + AXI **三者皆**手写 FL ≡ tick（禁止「至少一项」交差）。  
4. **Q3：** 不得改写 FR94–165「已关闭」（NFR73）。  
5. **Q4：** 不得静默扩大 FR142；宣称须引已关 FR（FR171 / NFR77）。  
6. **Q5：** 保持现行 MSRV；firtool 升钉须上游 Chisel **正式配对**后修订 **AD-9**（NFR12 / NFR75）；HEAD Parser 须修订 **AD-27**（±AD-9）。  
7. 实现闸门：**Epic 99** 关闭前，Epic 100–104 不得 ready。  
8. **`git push` / 远程同步不在本合同内。**  

**已规划映射（批准后不重编号）：**

| FR | Epic | 用户结果 |
|----|------|----------|
| FR166 | 99 | 合同闸门 + 诚实边界 |
| FR167 | 100 | 完整 ChiselSim + 多端 IDE 商店 |
| FR168 | 101 | SPI + I2C + AXI 手写 FL |
| FR169 | 102 | 更广 CIRCT/MLIR allocation / firtool 升钉 |
| FR170 | 103 | Chisel HEAD Parser 回迁 |
| FR171 | 104 | Phase 20 宣称诚实门 |

---

## 4. Detailed Change Proposals（Batch）

### 4.1 PRD `prd.md`

**Artifact:** `prds/prd-rhdl-2026-08-19/prd.md`

**NEW（批准后）：**
- `updated: 2026-09-12`（或实际批准日）
- `amendment` 追加：`phase20-nfr71-four-leftovers-2026-09-12`
- §0 权威边界追加：**Phase 20：** FR166–171 / NFR73–77 — 不回滚 FR154–165；NFR71 四条升格；≠ `git push`。

### 4.2 PRD `addendum.md`

**NEW — 追加节「Phase 20 NFR71 四条升格（Correct Course）」：**

- **用户决议：** 批准本提案；正式 Phase 20（Epic 99–104 · FR166–FR171 / NFR73–NFR77）。  
- **公开品牌（不变）：** Bitloom / `bitloom` / `bitloom-*`；设计 crate 仍只依赖 `bitloom-prelude`（AD-6）。禁止 `rhdl` / `rhdl-bits`。  
- **批准默认 Q1–Q5：** 见 frontmatter `approval_defaults`。  
- **与 Phase 12–19 关系：** 关闭仍有效（NFR73）；本批是新合同加深，**不是**「Phase 19 失败」补救叙事。  
- **实现闸门：** Epic 99 关闭前 100–104 不得 ready；软序 **99 →（100 ‖ 101 ‖ 102 ‖ 103）→ 104**。  
- **不**在本合同批准瞬间强制商店 live publish / firtool 升钉——属 Epic 100 / 102（+ NFR14）。

### 4.3 `epics.md`

**现状：** Phase 20 Inventory + Epic 99–104 故事 **已写入**（`phase20Status: planning-complete`）。  

**批准后：**
- 写入 `correctCoursePhase20Approved: 2026-09-12`（或实际日）  
- 可保持故事正文；Story 99.2 ATDD 须能验证本提案 `status: approved`  

### 4.4 ARCHITECTURE-SPINE / AGENTS / README / deferred

**NEW（Story 99.3–99.4 / 100–104.x）：**
- 脊柱/AGENTS：Phase 20 指针；AD-9 / AD-25 / AD-27 仅在对应 FR 触及且 NFR14 要求时修订（NFR75）  
- README / `deferred-work.md`：区分 Phase 19 完成面 vs Phase 20；各 FR 关闭前不得宣称对应条已交付  
- FR156 文档可加指针：NFR71 四条 → Phase 20 FR167–170（不改写 Phase 19「已关闭」）

### 4.5 Sprint status

**现状：** Epic 99–104 **未**出现在 `sprint-status.yaml`。  

**批准后 / 并行：** 经 **Sprint Planning** 种子 Epic 99–104 及 19 故事为 `backlog`（或闸门 epic `backlog`，实现 epic 在 99 关闭前不得 `ready-for-dev`）。**不得**在 Epic 99 关闭前将 100–104 标 `ready-for-dev`。

### 4.6 Stories

**无正文 diff：** 规划故事已齐。实现按 `epics.md` Epic 99–104；若批准时改 Q，再改对应 AC。

---

## 5. Implementation Handoff

**Change scope: Major**（四域加深 + 外部商店/CIRCT/Chisel/firtool + AD 修订）

| 角色 | 职责 |
|------|------|
| PM / 用户 | 批准本提案（Q1–Q5）；确认 stamp |
| PO / Dev | PRD/addendum + epics frontmatter 戳；Sprint Planning 种子 |
| Developer | Epic 99 →（100 ‖ 101 ‖ 102 ‖ 103）→ 104（`bmad-build`）；一 story 一 commit；各 epic 先 NFR14 |
| Architect | 99.4 指针；102/103 触及 AD 时审阅 |

**Success criteria：**
1. 本提案 `status: approved` + PRD/addendum Phase 20 段可验证  
2. Epic 99 关闭后 100–104 可 ready  
3. FR167–170：各条按 NFR14 子集可勾选；FR171 诚实宣称  
4. Phase 12–19 关闭证据仍有效；未写入本批 NFR14 的更深项仍须新合同（NFR76）  
5. sprint-status 含 Epic 99–104 backlog  

**Next after approval：** stamp PRD/addendum + epics `correctCoursePhase20Approved`（已落）+ sprint backlog 种子（已落）→ Story **99.1** NFR14 → **99.2**（验证本提案 approved）。

---

## Checklist §5–6（提案组件 · 待用户 Continue/批准）

- [x] **5.1–5.5** Issue / Impact / Approach / MVP / Handoff 已写入上文  
- [x] **6.1–6.2** 分析完整；提案可执行  
- [x] **6.3** 用户显式批准（yes · 2026-09-12 · Q1–Q5 默认）  
- [x] **6.4** sprint-status 已种子 Epic 99–104 backlog（Correct Course finalize）  
- [x] **6.5** handoff 确认（Major → PM stamp 已落 + Build 99.x；README/deferred 属 Story 99.3）
