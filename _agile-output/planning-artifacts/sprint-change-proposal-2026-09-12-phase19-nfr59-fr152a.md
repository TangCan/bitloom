---
title: Sprint Change Proposal — Phase 19 NFR59 全子集升格 + FR152(a) bitloom-lsp 上架
date: 2026-09-12
status: approved
approved: 2026-09-12
step5_user: yes（Q1–Q5 草案默认）
trigger: Phase 18 关闭后 NFR59 仍 deferred（NFR63/NFR67）且 FR152(a) 另开合同；create-epics Phase 19 规划已齐待 Correct Course 批准
mode: Batch
change_scope: Major
related_prior: sprint-change-proposal-2026-09-11-phase18-cli-crates-io-publish.md (Phase 18 · approved)
epics_ref: epics.md Phase 19 Inventory · Epic 87–98 · FR154–FR165 / NFR68–NFR72
approval_defaults: |
  Q1 NFR59 README 九条全部升格为 FR157–FR165（全做）；
  Q2 FR152(a)：bitloom-lsp publish=true + live cargo publish -p bitloom-lsp；重写 docs/fr152-* (b)→(a)；
  Q3 不以改写 Phase 12–18「已关闭」为代价；NFR68 隔离；
  Q4 不得静默扩大 FR142 表面；宣称须经 FR156；
  Q5 保持现行 MSRV；lsp 版本建议与工作区 1.0.0 对齐（除非故事另决）。
---

# Sprint Change Proposal — Phase 19 NFR59 全子集升格 + FR152(a) bitloom-lsp 上架

## 1. Issue Summary

**触发：** Phase 18（FR148–153 / Epic 84–86）**合同与实现已关闭**：`bitloom` CLI **1.0.0** 可 `cargo install`；FR152 **(b)** 故意保持 `bitloom-lsp` `publish=false` 且不挡 CLI；FR153 SemVer/发版诚实已关。结项诚实面明确写出三件「不算 FR 缺口」事项中的前两项现需**另开合同**交付：

1. **NFR59 仍 deferred（NFR63 / NFR67）** — 1.0 / CLI 上架 ≠ 清空更深 IP/GUI/协议/CIRCT/Chisel 等。  
2. **`bitloom-lsp` 仍 `publish=false`（FR152(b)）** — Phase 18 明示 **(a) 另开合同**。  
3. **`main` 未 push** — **运维**，**不是**本提案 FR（不纳入 Phase 19 合同）。

**背景：**

1. README「NFR59 仍 deferred」账本九条（自动 FSM 标签、第三方 LCOV GUI、MemRead stub→完整生成、非 Cargo monorepo 扫描、formal-sby 镜像卫生、更深 GUI/IDE、未列协议 FL、更广 CIRCT/MLIR、更深 Chisel/Parser）。  
2. `docs/fr152-bitloom-lsp-publish-policy.md` 仍钉死 **(b)**；**(a)** 标 Deferred。  
3. `epics.md` 已完成 Phase 19 规划（`phase19Status: planning-complete`；Epic **87–98** / **38** 故事）；`sprint-status.yaml` **已种子** backlog。  
4. **尚缺** Correct Course + PRD/addendum 批准戳（Story 87.2）。

**问题陈述：** 若不另开 Phase 19 合同，则无法合法宣称 NFR59 任一加深或 `bitloom-lsp` crates.io 上架；若在无闸门下直接实现/publish，则违反 NFR63/NFR67 与 FR152(b)「另开合同」纪律。

**本提案不宣称：** Phase 12–18 AC 未达标；不回滚 FR94–153；不把 `git push` 当成产品 FR；不在批准瞬间强制 live publish（实发属 Epic 88 / FR155）。

---

## 2. Impact Analysis

### Checklist 记录（Step 2 · Batch）

#### §1 Trigger & Context
- [x] **1.1** 触发：Phase 18 结项诚实面 + create-epics Phase 19（用户确认；Batch）  
- [x] **1.2** 类型：**战略/合同升格** — NFR59 全子集 + FR152(a) 升格为 FR154–165；非失败回滚  
- [x] **1.3** 证据：README NFR59 账本；`docs/fr152-*` (b)；addendum Phase 18 Q3「(a) 另开合同」；NFR63/NFR67；`epics.md` Phase 19 Inventory；sprint Epic 87–98 backlog  

#### §2 Epic Impact
- [x] **2.1** Epic 1–86 **不回滚、不改已关闭 AC**  
- [x] **2.2** **已规划** Epic 87–98；本提案批准后 stamp PRD + `correctCoursePhase19Approved`（sprint **已种子**）  
- [x] **2.3** 硬依赖：Epic 87（闸门）关闭前 88–98 不得 ready  
- [x] **2.4** 无作废 epic；不新增超出 87–98 的必做 epic  
- [x] **2.5** 软序：**87 →（88 ‖ 89…97）→ 98**；Epic 95（`ip/`）与其它触及面建议串行  

#### §3 Artifact Conflicts
- [!] **3.1 PRD** — 须追加 Phase 19 段；frontmatter amendment；权威边界增 FR154–165 / NFR68–72  
- [!] **3.2 Architecture** — Phase 19 指针；触及 AD-25/AD-27 时由 Epic 96/97 NFR14+NFR70 修订；**不**改 AD-6（除非某 FR 显式另开）  
- [N/A] **3.3 UX**  
- [!] **3.4 其他** — README / deferred-work / fr152 政策重写；AGENTS 卫生；CI/`formal-sby`（FR161）；crates.io lsp live（FR155）；各加深域 ATDD  

#### §4 Path Forward
- **4.1 Direct Adjustment：** **Viable（选定）** — 批准 Phase 19 合同 → 实现 Epic 87–98。Effort **High**；Risk **High**（九条加深域 + live publish；工具链/IDE/CIRCT/Chisel 外部依赖）  
- **4.2 Rollback：** **Not viable** — 不回滚 Phase 17/18 已上架 1.0.0 / CLI  
- **4.3 缩 MVP / 只做 lsp 或子集：** **Not chosen** — 用户 Step 1 已锁 **全做 FR157–165** + FR155 live publish  
- [x] **4.4 选定：Option 1 Direct Adjustment**  

---

## 3. Recommended Approach

**批准 Phase 19「NFR59 全子集升格 + FR152(a)」合同**，不回滚 Phase 12–18：

1. PRD/addendum 追加 **Phase 19**；**FR154–FR165** / **NFR68–NFR72**；Phase 12–18 关闭证据 **仍有效**（NFR68）。  
2. **Q1：** NFR59 README **九条全部**升格为 **FR157–FR165**（各 epic NFR14 钉死验收子集；禁止静默超子集 — NFR71）。  
3. **Q2：** **FR152(a)** — `bitloom-lsp`：`publish=true` + version；重写 `docs/fr152-*`；(a)；**live** `cargo publish -p bitloom-lsp`（Epic 88）；不得破坏已关闭 FR151。  
4. **Q3：** 不得改写 FR94–153「已关闭」（NFR68）。  
5. **Q4：** 不得静默扩大 FR142；宣称须引已关 FR（FR156 / NFR72）。  
6. **Q5：** 保持现行 MSRV；lsp 版本建议与工作区 **1.0.0** 对齐（除非故事另决）。  
7. 实现闸门：**Epic 87** 关闭前，Epic 88–98 不得 ready。  
8. **`git push` / 远程同步不在本合同内。**  

**已规划映射（批准后不重编号）：**

| FR | Epic | 用户结果 |
|----|------|----------|
| FR154 | 87 | 合同闸门 + 诚实边界 |
| FR155 | 88 | bitloom-lsp FR152(a) live crates.io |
| FR157 | 89 | 自动 FSM 标签提取 |
| FR158 | 90 | 第三方 LCOV GUI 一等集成 |
| FR159 | 91 | MemRead 完整生成 |
| FR160 | 92 | 非 Cargo monorepo 路径扫描 |
| FR161 | 93 | formal-sby 镜像卫生 |
| FR162 | 94 | 更深 GUI/IDE |
| FR163 | 95 | 未列协议手写 FL |
| FR164 | 96 | 更广 CIRCT/MLIR / 仿真门禁 |
| FR165 | 97 | 更深 Chisel/Parser 生态 |
| FR156 | 98 | Phase 19 宣称诚实门 |

---

## 4. Detailed Change Proposals（Batch）

### 4.1 PRD `prd.md`

**Artifact:** `prds/prd-rhdl-2026-08-19/prd.md`

**NEW（批准后）：**
- `updated: 2026-09-12`（或实际批准日）
- `amendment` 追加：`phase19-nfr59-fr152a-2026-09-12`
- §0 权威边界追加：**Phase 19：** FR154–165 / NFR68–72 — 不回滚 FR148–153；NFR59 九条升格 + FR152(a)；≠ `git push`。

### 4.2 PRD `addendum.md`

**NEW — 追加节「Phase 19 NFR59 + FR152(a)（Correct Course）」：**

- **用户决议：** 批准本提案；正式 Phase 19（Epic 87–98 · FR154–FR165 / NFR68–NFR72）。  
- **公开品牌（不变）：** Bitloom / `bitloom` / `bitloom-*`；设计 crate 仍只依赖 `bitloom-prelude`（AD-6）。禁止 `rhdl` / `rhdl-bits`。  
- **批准默认 Q1–Q5：** 见 frontmatter `approval_defaults`。  
- **与 Phase 12–18 关系：** 关闭仍有效（NFR68）；本批是新合同加深/上架，**不是**「Phase 18 失败」补救叙事。  
- **实现闸门：** Epic 87 关闭前 88–98 不得 ready；软序 **87 →（88 ‖ 89…97）→ 98**。  
- **不**在本合同批准瞬间强制 `cargo publish`——lsp live 属 Epic 88 / FR155。

### 4.3 `epics.md`

**现状：** Phase 19 Inventory + Epic 87–98 故事 **已写入**（`phase19Status: planning-complete`）。  

**批准后：**
- 写入 `correctCoursePhase19Approved: 2026-09-12`（或实际日）  
- 可保持故事正文；Story 87.2 ATDD 须能验证本提案 `status: approved`  

### 4.4 ARCHITECTURE-SPINE / AGENTS / README / deferred / fr152

**NEW（Story 87.3–87.4 / 88.x / 98.x）：**
- 脊柱/AGENTS：Phase 19 指针；AD-25/AD-27 仅在 FR164/FR165 触及且 NFR14 要求时修订  
- README / `deferred-work.md`：区分 Phase 18 完成面 vs Phase 19；各 FR 关闭前不得宣称对应条已交付  
- `docs/fr152-bitloom-lsp-publish-policy.md`：**Epic 88** 重写 (b)→(a)  

### 4.5 Sprint status

**现状：** Epic 87–98 及 38 故事键 **已为 `backlog`**（create-epics Complete 时种子）。  

**批准后：** 无需重编号；**不得**在 Epic 87 关闭前将 88–98 标 `ready-for-dev`。

### 4.6 Stories

**无正文 diff：** 规划故事已齐。实现按 `epics.md` Epic 87–98；若批准时改 Q，再改对应 AC。

**OLD（FR152 政策语义 · 文档）：**
- (b) selected；(a) deferred  

**NEW（批准 + Epic 88）：**
- (a) selected；live crates.io publish 为目标；≠ LSP 功能加深（功能加深仍属 FR157+ 等）

---

## 5. Implementation Handoff

**Change scope: Major**（多域加深 + 外部工具/商店/CIRCT/Chisel + live publish）

| 角色 | 职责 |
|------|------|
| PM / 用户 | 批准本提案（Q1–Q5）；确认 stamp |
| PO / Dev | PRD/addendum + epics frontmatter 戳 |
| Developer | Epic 87 →（88 ‖ 89…97）→ 98（`bmad-build`）；一 story 一 commit；各 epic 先 NFR14 |
| Architect | 87.4 指针；96/97 触及 AD 时审阅 |

**Success criteria：**
1. 本提案 `status: approved` + PRD/addendum Phase 19 段可验证  
2. Epic 87 关闭后 88–98 可 ready  
3. FR155：`bitloom-lsp` live crates.io 可勾选  
4. FR157–165：各条按 NFR14 子集可勾选；FR156 诚实宣称  
5. Phase 12–18 关闭证据仍有效；未写入本批 NFR14 的更深项仍须新合同（NFR71）  

**Next after approval：** stamp PRD/addendum + epics `correctCoursePhase19Approved` → Story **87.1** NFR14 → **87.2**（验证本提案 approved）。

---

## Checklist §5–6（提案组件）

- [x] **5.1–5.5** Issue / Impact / Approach / MVP / Handoff 已写入上文  
- [x] **6.1–6.2** 分析完整；提案可执行  
- [x] **6.3** 用户显式批准（yes · 2026-09-12）  
- [x] **6.4** sprint-status 已种子（批准前已完成；批准后无需改 ID）  
- [x] **6.5** handoff 确认（批准后 · Major → PM stamp + Build 87.x）
