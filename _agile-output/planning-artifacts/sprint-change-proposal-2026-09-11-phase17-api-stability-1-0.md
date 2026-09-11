---
title: Sprint Change Proposal — Phase 17 公开 API 稳定门（Bitloom 1.0）
date: 2026-09-11
status: approved
approved: 2026-09-11
trigger: Phase 16 合同结项后，用户要求解决「≠ Bitloom 1.0 / 公开 API 永久稳定」缺口
mode: Batch
change_scope: Moderate
step4_user: Continue（起草）
step5_user: 批准（Q1–Q5 草案默认）
related_prior: sprint-change-proposal-2026-09-11-phase16-nfr55-final-closeout.md (Phase 16 · approved)
epics_ref: epics.md Phase 17 Inventory · Epic 79–83 · FR141–FR147 / NFR60–NFR63
approval_defaults: |
  Q1 bitloom-sim IN surface; Q2 hir/builder/vlog publish OK not in 1.0 promise;
  Q3 Epic 82 skip-if-no-blockers; Q4 no NFR59 prerequisite; Q5 keep current MSRV.
---

# Sprint Change Proposal — Phase 17 公开 API 稳定门（Bitloom 1.0）

## 1. Issue Summary

**触发：** Phase 16（FR133–140 / Epic 72–78）**合同结项已齐**（实现 + retros + action-items sweep；`phase16Status: complete`）。用户确认：结项分析中「**不是** Bitloom 1.0 / 公开 API 永久稳定」须另立路径解决；要求继续起草本批 Correct Course / FR。

**背景：**

1. **NFR15** 与 `docs/semver-0x-policy.md` 明文：结项期停 **0.x**；sprint/backlog 关闭 **不是** 升 1.0 的理由；**1.0 = 维护者主动声明的公开 API 稳定门**。  
2. Phase 16 终局口径 = 本批加深关闭 + 诚实 **NFR59** deferred；**明确不等于**冲 1.0。  
3. 当前已 `publish = true` 的 crate 至少包括：`bitloom`、`bitloom-prelude`、`bitloom-sim`、`bitloom-macro`、`bitloom-hir`、`bitloom-builder`、`bitloom-vlog`（workspace ~0.1.2）——表面边界 **尚未** 以稳定门合同钉死。  
4. **NFR59**（更深 GUI/IDE、未列协议 FL、更广 pad、CIRCT 仿真/MLIR lower、更深 Parser 生态、跨 crate C2 等）**仍 deferred**；1.0 **不得**静默吞并 NFR59。

**问题陈述：** 若不另开「公开 API 稳定门」合同，对外无法合法宣称 SemVer **1.0** / 「公开 API 稳定」；若用 Phase 16 关闭证据直接 bump 1.0，则违反 NFR15 与 FR140 诚实纪律。

**本提案不宣称：** Phase 16 AC 未达标；不回滚 FR94–140；不要求先清空 NFR59 才可 1.0。

---

## 2. Impact Analysis

### Checklist 记录（Step 2）

#### §1 Trigger & Context
- [x] **1.1** 触发：Phase 16 结项后用户 Explicit「如何解决 ≠ 1.0」→「请继续」起草  
- [x] **1.2** 类型：**战略/合同升格** — 新需求 FR141–147；非实现失败  
- [x] **1.3** 证据：NFR15、`semver-0x-policy.md`、Phase 16 deferred/README、crates publish 现状  

#### §2 Epic Impact
- [x] **2.1** Epic 1–78 **不回滚、不改已关闭 AC**  
- [x] **2.2** **拟规划** Epic 79–83（本提案批准后写入 `epics.md` + sprint seed）  
- [x] **2.3** 硬依赖：Epic 79（闸门）关闭前 80–83 不得 ready  
- [x] **2.4** 无作废 epic；NFR15「停 0.x」在本批关闭后由 **FR143** 升格为 1.0 政策（仅钉死表面）  
- [x] **2.5** 顺序：**79 → 80 → 81 →（82 按需）→ 83**；82 可与 81 部分并行若卫生项独立  

#### §3 Artifact Conflicts
- [!] **3.1 PRD** — 须追加 Phase 17 段；frontmatter amendment；权威边界增 FR141–147  
- [!] **3.2 Architecture** — Deferred / Stack 增加「1.0 公开表面」指针；**不**借机改 AD-6（设计 crate → prelude）除非表面清单另决  
- [N/A] **3.3 UX**  
- [!] **3.4 其他** — `docs/semver-0x-policy.md` → 1.0 政策文档；README 状态段；CHANGELOG；deferred-work；AGENTS 卫生；sprint-status 播种 79–83  

#### §4 Path Forward
- **4.1 Direct Adjustment：** **Viable（选定）** — 追加稳定门合同 + 实现门禁 + 发 1.0.0。Effort Medium；Risk Medium（表面划界争议、semver 工具误报、过早冻结）  
- **4.2 Rollback：** **Not viable**  
- **4.3 先做完全部 NFR59 再 1.0：** **Not chosen** — 违背「1.0 = 承诺边界」；会无限期拖延  
- [x] **4.4 选定：Option 1 Direct Adjustment** — 划表面 → 政策/CI → 可选卫生 → 发版  

---

## 3. Recommended Approach

**批准 Phase 17「公开 API 稳定门 / Bitloom 1.0」合同**，不回滚 Phase 12–16：

1. PRD/addendum 追加 **Phase 17**；**FR141–FR147** / **NFR60–NFR63**；Phase 12–16 关闭证据 **仍有效**（NFR60）。  
2. **先钉死公开表面**（FR142），再写 1.0 SemVer 政策（FR143）与破坏性变更 CI 门禁（FR144）。  
3. 仅对表面内 API 承诺稳定；表面外可继续 `publish = false`、隐藏、或 0.x 实验路径（须在清单写明）。  
4. **NFR59 仍诚实 deferred**；1.0 宣称 **不得** 暗示 NFR59 已关闭。  
5. 发版：**`bitloom` 1.0.0**（及表面清单承诺同时升 1.0 的库 crate）；annotated tag `v1.0.0` + CHANGELOG。  
6. 对外「1.0 / 公开 API 稳定」类宣称：仅可在对应 FR141–146 关闭后，按 **FR147** 勾选。  

**默认建议表面（FR142 钉死前可修订；批准本提案 ≠ 冻结下列表——冻结在 Epic 80）：**

| 纳入 1.0 表面（建议） | 暂不纳入 / 须显式决定 |
|----------------------|------------------------|
| `bitloom` CLI：已文档化的稳定子命令与 flag（`build` / `new` / `firtool` / 已公开的 wave·hls 等；实验 flag 标 unstable） | 未文档化的内部调试开关 |
| `bitloom-prelude`：设计者主路径（`Bits`/`UInt`/`Clock`/`Reset`/`Input`/`Output`/`Bundle`/`HwVec`、elaborate 入口、`ip::*` **稳定再导出路径**） | prelude 内未文档化的 helper；纯内部 re-export 细节 |
| `bitloom-macro`：文档化属性宏（如 `#[module]` / `Bundle` 等产品路径） | 宏展开内部细节 |
| `bitloom-sim`：**可选**纳入——若纳入则钉死 `tick`/VCD/公开双模型 API；否则保持「可用但非 1.0 稳定承诺」并在清单写明 | HIR 节点布局、builder 内部、vlog 内部 AST |
| — | `bitloom-hir` / `bitloom-builder` / `bitloom-vlog`：建议 **降为非稳定承诺**（可继续 publish 但标「非 1.0 表面」或考虑 `publish = false` / 仅 workspace） |
| — | `bitloom-lsp`、`rhdl-*` 工具 crate：默认不进 1.0 表面 |

---

## 4. Detailed Change Proposals（Batch）

### 4.1 PRD `prd.md`

**Artifact:** `prds/prd-rhdl-2026-08-19/prd.md`

**NEW（批准后）：**
- `updated: 2026-09-11`（或实际批准日）
- `amendment` 追加：`phase17-api-stability-1-0-2026-09-11`
- §0 权威边界追加：**Phase 17：** FR141–147 / NFR60–63 — 不回滚 FR94–140；1.0 ≠ 清空 NFR59。

### 4.2 PRD `addendum.md`

**NEW — 追加节「Phase 17 公开 API 稳定门（Correct Course）」：**

- **用户决议：** 批准本提案；正式 Phase 17（Epic 79–83 · FR141–FR147 / NFR60–NFR63）。  
- **公开品牌（不变）：** Bitloom / `bitloom` / `bitloom-*`；设计 crate 仍只依赖 `bitloom-prelude`（AD-6），除非另开合同。  
- **1.0 定义：** 对 **FR142 钉死的公开表面** 承诺 SemVer major 稳定；破坏性变更须升 major。  
- **非目标：** 不以清空 NFR59 / 永不变更内部实现 / 永久无 deferred 作为 1.0 条件。

### 4.3 `docs/semver-0x-policy.md`

**NEW（Epic 81 / FR143）：** 增补或并行 `docs/semver-1-0-policy.md`：

- 1.0 起：表面内 breaking → major；MSRV 上调政策；弃用窗口；与 Cargo/crates.io 对齐。  
- 明确：关闭 sprint ≠ 自动 major。  
- 保留 0.x 政策文件作历史，或改名为「pre-1.0」。

### 4.4 ARCHITECTURE-SPINE / AGENTS / README / deferred

**批准后由 Epic 79 收口故事落地指针：** Phase 17 现行稳定门；Phase 16 关闭仍有效；NFR59 仍 deferred；1.0 宣称 → FR147。

### 4.5 `epics.md` + `sprint-status.yaml`

**批准后：** 追加 Phase 17 Inventory + Epic 79–83；`phase17Status: planning-complete` → 实现中更新；sprint 播种故事键。

---

## 5. Proposed FR / NFR / Epics（批准后写入 epics.md）

### 5.1 Functional Requirements

| ID | 标题 | 验收要点 |
|----|------|----------|
| **FR141** | Phase 17 稳定门合同闸门 | Correct Course + PRD/addendum 批准；同步 README/deferred/脊柱指针；**未完成则 FR142+ 不得 ready** |
| **FR142** | 公开 API 表面清单 | 成文清单：crate × 模块/类型/CLI 动词；标明 in-surface / out-of-surface；设计 crate → prelude 边界不变；ATDD 锁文件存在与必填节 |
| **FR143** | SemVer 1.0 政策 | 取代「结项期强制 0.x」对表面的约束；breaking/MSRV/弃用规则成文；与 NFR15 关系写清（结项曾停 0.x；本 FR 授权升 1.0） |
| **FR144** | 破坏性变更 CI 门禁 | 对表面 crate 跑 `cargo-semver-checks`（或文档等价）；失败非零；禁止 continue-on-error |
| **FR145** | 预 1.0 表面卫生（可选加深） | 仅处理 FR142 清单标出的、阻塞稳定承诺的 breaking 毛刺；**≠** NFR59 产品加深 |
| **FR146** | 发布 Bitloom **1.0.0** | 表面承诺 crate 版本 1.0.0；`cargo publish`（或 dry-run+手动清单）+ tag `v1.0.0` + CHANGELOG；`cargo doc` 绿 |
| **FR147** | 1.0 宣称诚实门面 | 对外「1.0 / 公开 API 稳定」须引 FR141–146；**禁止**用 Phase 16 终局 alone 冒充 1.0；NFR59 仍须诚实列出 |

### 5.2 Non-Functional Requirements

| ID | 内容 |
|----|------|
| **NFR60** | Phase 12–16 关闭面仍有效；不得改写 FR94–140「已关闭」为失败 |
| **NFR61** | Phase 17 规划故事齐后方可宣称本批规划 complete |
| **NFR62** | 触及 publish 表面 / SemVer 政策 / CI 门禁时须先修订文档/脊柱/CI 合同（诚实同步） |
| **NFR63** | 1.0 **不得**静默吞并 **NFR59**；更深产品加深仍须新合同 |

### 5.3 Epics（建议）

| Epic | FR | 主题 | Gate |
|------|-----|------|------|
| **79** | FR141, FR147 | 稳定门合同与诚实门面 | 79 关闭前 80–83 不得 ready |
| **80** | FR142 | 公开表面清单钉死 | 须 79 |
| **81** | FR143, FR144 | 1.0 政策 + semver CI | 须 80 |
| **82** | FR145 | 预 1.0 卫生（可裁剪为 skip 若清单无阻塞项） | 须 80；可与 81 软并行 |
| **83** | FR146 | 发版 1.0.0 + 收口 | 须 81；（若开 82）须 82 |

**故事形态（每 epic 建议 2–4）：** NFR14（若触及工具链/发布风险）→ 实现/文档 → 收口；一 story 一 commit。

### 5.4 验收谓词（跨 epic）

1. **表面：** `docs/public-api-1-0-surface.md`（或等价）存在；ATDD 断言 in/out 分区与品牌/prelude 边界。  
2. **政策：** `docs/semver-1-0-policy.md`（或升格后的 semver 政策）存在；明确 breaking → major。  
3. **门禁：** CI required job 对表面 crate 跑 semver-checks（或文档钉死路径 `just semver-check`）；缺工具/违规 → 非零。  
4. **发版：** 仓库版本与 tag `v1.0.0` 对齐；CHANGELOG 有 1.0.0；README 声明 1.0 表面并保留 NFR59 列表。  
5. **诚实：** FR147 ATDD：禁止「Phase 16 已结 ⇒ 1.0」话术；须引 FR141–146。

---

## 6. Handoff / 批准后步骤

1. 用户将本文件 `status` → **approved**，填写 `approved:` 日期。  
2. `bmad-create-epics-and-stories`（或等价）把 §5 写入 `epics.md`；`phase17Status: planning-complete`。  
3. PRD/addendum / deferred / README 指针按 Epic 79 故事落地。  
4. `bmad-sprint-planning` 播种 Epic 79–83。  
5. 从 **79.1** 开工；**禁止**未关 79 即标 80–83 ready。  
6. **不**在本提案批准瞬间执行 `cargo publish`——发版属 **FR146 / Epic 83**。

---

## 7. Risks & Open Questions（请用户拍板）

| # | 问题 | 建议默认 |
|---|------|----------|
| Q1 | `bitloom-sim` 是否进入 1.0 表面？ | **纳入**（设计者常用 `tick`/VCD）；若希望表面更小可 **排除** 并文档化 |
| Q2 | `bitloom-hir` / `builder` / `vlog` 是否继续 crates.io publish？ | **publish 可保留，但不进 1.0 稳定承诺**；或逐步 `publish = false` |
| Q3 | Epic 82（卫生）是否强制？ | **按 Epic 80 清单决定**：无阻塞 breaking → 可 skip/标 optional |
| Q4 | 1.0 是否要求先消化部分 NFR59？ | **否**（默认）；NFR59 保持 deferred |
| Q5 | MSRV 在 1.0 时是否同时上调？ | **默认保持现行 MSRV**；上调另开小项并写入政策 |

---

## 8. Approval

**现状：`approved`（2026-09-11）。** 用户回复「批准」；Q1–Q5 采用草案默认（见 frontmatter `approval_defaults`）。

已落地：PRD/addendum Phase 17；`epics.md` Epic 79–83；`sprint-status.yaml` 播种。发版仍属 **FR146 / Epic 83**。
