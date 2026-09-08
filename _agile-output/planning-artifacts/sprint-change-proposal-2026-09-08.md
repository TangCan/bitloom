---
title: Sprint Change Proposal — Wave 3 非闭包缺口并进计划
date: 2026-09-08
status: approved
trigger: 调研 Wave 3 非闭包缺口并进计划
mode: Batch
change_scope: Moderate
---

# Sprint Change Proposal — Wave 3 非闭包缺口并进计划

## 1. Issue Summary

**触发：** Correct Course（2026-09-08）。用户要求将 deep-recon  
`technical-requirements-implementation-gap-generic-2026-09-08` 中的 **Wave 3 — 非闭包高优先缺口** 正式并进项目计划。

**背景：** Phase 9（Epic 26–30）仅承包受控泛型闭包（FR72–78）。`epics.md` 明确将 Wave 3 列为  
`excludedFromPhase9Rewrite`。同时 Epic 1–25 在规划面多为 `done`，但 crates 盘点显示多项 PRD 合同能力仍为 **Partial**：

| Wave 3 项 | 证据（D3 inventory） | 规划面状态 |
|-----------|----------------------|------------|
| CDC / DoubleFlop 真 RTL | ZST + `mark_cdc_bridge`；无双 FF RTL | Epic 7 / FR23 标 done |
| Bundle 嵌套 / derive | flatten；nested OUT OF SCOPE | Epic 19 / FR51 |
| Chisel Mem / FR28 | `emit_chisel` E0901 on Mem | Epic 20/25 / FR28 |
| HLS 真调度 vs 外挂 | 无树内调度（AD-25） | Epic 24 / FR35·50 外挂路径 |
| IP stubs → 可综合 IP | `ip.rs` smoke stubs | Epic 22 / FR37·48 |
| 形式化 / viz / C ABI 等 | SVA toy；LSP deferred；C ABI Counter-only；SoftF16 host-only | 各对应 epic done |

**问题陈述：** 「Epic complete」被误读为「requirements/PRD 深度已交付」。Wave 3 必须成为可跟踪 backlog（新 Phase），与 Phase 9 闭包并行，否则产品诚实度与 FR23/51/28/48 等合同不对齐。

---

## 2. Impact Analysis

### Checklist 记录（Step 2）

#### §1 Trigger & Context
- [x] **1.1** 触发非单 story，而是调研 Wave 3 + Phase 9 显式排除
- [x] **1.2** 类型：**规划诚实度缺口 / 已签约 FR 实现深度不足**（非战略 pivot）
- [x] **1.3** 证据：research.md Wave 3 表；`crates-implementation-inventory.md` Partial/Absent；`epics.md` excludedFromPhase9

#### §2 Epic Impact
- [x] **2.1** Phase 9 Epic 26–30 **仍可按原计划完成**；不因 Wave 3 阻塞闭包合同
- [x] **2.2** **新增 Phase 10**（Epic 31+），不修改 Phase 9 闭包范围；不回滚 Epic 7/19/20/22/24
- [x] **2.3** Epic 29（IP 闭包）**依赖**「无闭包可综合 IP 基线」→ Phase 10 IP 加深宜在 Epic 29 实现前或并行优先
- [x] **2.4** 不使任何 Phase 9 epic 作废；需要新 epic 填深度
- [x] **2.5** 优先级：Phase 9 Wave0–1 与 Phase 10 CDC/Bundle/Mem **可并行**；IP 基线优先于 Epic 29；HLS 树内调度 **不**新建 epic

#### §3 Artifact Conflicts
- [x] **3.1 PRD：** 不改核心目标；需 **加深 success 条**（或 addendum「深度验收」）针对 FR23/51/28/37·48/33/36/39；MVP 仍可达，但须承认「规划 done ≠ 深度 done」
- [x] **3.2 Architecture：** 可选短注：DoubleFlop/SyncFIFO 不得仅以 ZST 叙事交差 FR23；AD-25 维持无树内 HLS 调度 → Wave 3 HLS 项 **裁决为保持外挂**
- [x] **3.3 UX：** N/A
- [x] **3.4 其他：** `sprint-status.yaml` 缺 Epic 26–30 **且** 将缺 31+；文档/ATDD 需深度夹具；CI 可选增加 Mem→Chisel 夹具

#### §4 Path Forward
- [x] **4.1 Direct Adjustment：** **Viable** — 追加 Phase 10 epic/stories；Effort **High**；Risk **Medium**
- [x] **4.2 Rollback：** **Not viable** — 回滚 Epic 7/19/22 成本高且无益；应 **叠加深度 epic** 而非撤销历史
- [x] **4.3 MVP Review：** **Not required** — 不缩减闭包 MVP；可选将 LSP / SoftF16-to-HIR 标为显式 post-depth defer
- [x] **4.4 Selected：** **Hybrid = Option 1 为主**（新 Phase 10 + PRD/AD 深度澄清）；HLS 调度项走「确认外挂、不建 epic」

---

## 3. Recommended Approach

**选定路径：** 在保留 Phase 9 闭包计划的前提下，新增 **Phase 10 — 非闭包 Partial 深度收口**，将 Wave 3 六项拆为可验收 epic；与 Phase 9 **并行**（IP 基线优先服务 Epic 29）。

| Wave 3 项 | 处置 |
|-----------|------|
| CDC / DoubleFlop 真 RTL | **Epic 31** — FR23 深度 |
| Bundle 嵌套 / derive | **Epic 32** — FR51 深度 |
| Chisel Mem / E0901 | **Epic 33** — FR28 深度 |
| HLS 真调度 vs 外挂 | **不建 epic** — 在 Epic 26.2 决策表钉死「维持外挂 / AD-25」 |
| IP stubs → 可综合 IP | **Epic 34** — FR37/48 深度（无闭包）；**先于或并行于 Epic 29** |
| 形式化 / viz / C ABI 等 | **Epic 35** — 残余 Partial 打包（FR33/36/39；LSP 继续显式 deferred） |

**工作量：** High（5 新 epic，约 15–20 stories 量级，具体由后续 CE 细化）  
**风险：** Medium（与「已 done」历史叙事冲突 → 用「深度增补」措辞，避免否定已交付的最小合同）  
**时间线：** 不阻塞 Story 26.1 开工；建议 Sprint Planning 同时登记 Phase 9 + Phase 10 backlog

---

## 4. Detailed Change Proposals（Batch）

### 4.1 Epics — `epics.md`

**P-E1 — 修订 Phase 9 排除说明**

OLD（概念）：`excludedFromPhase9Rewrite` 含「Wave 3 … tracked in research; not Phase 9 FR set」且暗示仅 research 跟踪。

NEW：保留「不在 Phase 9 实现」，并交叉链接 **Phase 10**；删除「仅 research 跟踪、无 backlog」语义。

Rationale：Correct Course 后 Wave 3 进入正式计划。

---

**P-E2 — 追加 Phase 10 Requirements Inventory + Epic List（草案 ID）**

新建（接 FR78/NFR36 之后）：

```
## Phase 10 Requirements Inventory（追加 · 2026-09-08 · Correct Course）

范围：Wave 3 非闭包 Partial 深度；不重写 Epic 1–30；不替代 Phase 9 闭包。

### Functional（加深既有 FR + 可选新 ID）

FR23-depth: DoubleFlop/SyncFIFO（或文档等价）须 emit 真实同步器 RTL，并可按域 tick 黄金；不得仅以 ZST + mark_cdc_bridge 交差。
FR51-depth: 文档化嵌套 Bundle 与/或 derive 路径；位宽/方向仍 emit 前失败。
FR28-depth: 收敛 Mem→Chisel E0901（支持文档化 Mem 子集或明确永久非目标+替代验收）。
FR37/48-depth: UART/SPI/I2C/FIFO/AXI 至少达到可 elaborate/emit/tick 的非 stub 基线（无闭包）；供 Epic 29 叠加。
FR33-depth / FR36-depth / FR39-depth: C ABI 超出 Counter demo；SoftF16 可综合路径或显式 defer；SVA/formal 超出 toy check。
FR-HLS-policy: （非实现）确认树内调度非目标；外挂 Bambu/Vitis 为唯一产品路径（AD-25）。

NFR14: Phase 10 各 epic ready 前风险记录。
NFR12: Chisel/firtool 钉死对不变（Epic 33）。

### Epic List（建议编号）

### Epic 31: CDC 同步器真 RTL
**FRs:** FR23-depth（继承 FR23/FR52）
**Depends on:** 无（可与 Phase 9 并行）

### Epic 32: Bundle 嵌套与 derive
**FRs:** FR51-depth
**Depends on:** 无（可与 31 并行）

### Epic 33: Chisel Mem / FR28 深度
**FRs:** FR28-depth（继承 FR28/FR46 正向腿）
**Depends on:** Epic 25 CI 门禁仍绿

### Epic 34: 一级 IP 可综合基线（无闭包）
**FRs:** FR37/48-depth
**Depends on:** 建议先于 Epic 29

### Epic 35: 残余 Partial 收口（C ABI / Float / Formal；LSP deferred）
**FRs:** FR33-depth, FR36-depth, FR39-depth；LSP 显式非目标本 epic
**Depends on:** 可并行；优先级低于 31–34
```

正式 FR 编号建议在执行 `bmad-create-epics-and-stories` Phase 10 时定为 **FR79+**（避开碰撞），上表 `*-depth` 为语义占位。

Rationale：Wave 3 → 可跟踪用户价值 epic，避免改写历史 done。

---

### 4.2 PRD — `prd-rhdl-2026-08-19`

**P-P1 — addendum（或 prd 修订块）「实现深度与规划 done」**

NEW 段落要点：
- Epic/sprint `done` 表示当时 AC 最小合同；若 crates 仍为 Partial，须经 **depth epic** 收口。
- 列出 FR23 / FR51 / FR28 / FR37·48 / FR33 / FR36 / FR39 的深度验收指针 → Phase 10。
- HLS：重申无树内调度器；外挂为产品路径（与 AD-25 一致）。

Rationale：防止合同被「已完成 epic」误关。

**P-P2 —（可选，与 Epic 26.4 合并）** 若 26.4 已开 PRD 增补，可将 Phase 10 指针写入同一 addendum，减少 PRD 抖动。

---

### 4.3 Architecture — `ARCHITECTURE-SPINE.md`

**P-A1 — AD-18/AD-22 或新短 AD 注记（建议 AD-29 草案）**

NEW：语言级 CDC 原语的 **可综合真 RTL** 为 FR23 深度验收的一部分；纯 ZST 不足以关闭深度 epic。

**P-A2 — AD-25 交叉引用**

NEW 一句：Correct Course 2026-09-08 确认 Wave 3「HLS 真调度」**不**立项；产品路径保持外挂。

Rationale：架构与 backlog 一致。

---

### 4.4 UX

**N/A** — 无 UI 合同变更。

---

### 4.5 Secondary

**P-S1 — `sprint-status.yaml`**（批准本提案 **并** 跑 Sprint Planning 后）：
- 登记 Epic 26–30（Phase 9，现均缺）
- 登记 Epic 31–35（Phase 10，`backlog`）

**P-S2 — Research 交叉链接**  
在 research.md Recommendations 注明 Wave 3 → Phase 10 Correct Course 已采纳（可选）。

---

## 5. Implementation Handoff

**变更范围分类：Moderate**

| 角色 | 职责 |
|------|------|
| **PM / PO** | 批准本提案；决定 FR79+ 正式编号策略；PRD addendum（P-P1） |
| **Architect** | P-A1/A2 短修订 |
| **PO + `bmad-create-epics-and-stories`** | Phase 10 正式 inventory + Epic 31–35 完整 stories（本提案仅草案） |
| **`bmad-sprint-planning`** | 刷新 `sprint-status.yaml`（Phase 9 + 10） |
| **Developer / `bmad-build`** | 可继续 Phase 9 Story 26.1；Phase 10 在 CE+SP 后按 31→34 优先 |

**成功标准：**
1. `epics.md` 含 Phase 10 inventory + Epic 31–35（含可测 AC 的 stories）
2. PRD addendum 声明深度 vs 规划 done
3. AD 注记 CDC 真 RTL + HLS 外挂确认
4. `sprint-status.yaml` 含 epic-26…35
5. Wave 3 六项均有明确处置（含 HLS「不建 epic」）

**不在本提案直接改代码。**

---

## 6. Checklist §5–6 状态

- [x] 5.1–5.5 提案组件已写入本文
- [x] 6.1–6.3 用户 2026-09-08 批准 (yes)
- [x] 6.4 sprint-status 已登记 Epic 26–35（Phase 10 stories 待 CE 细化后补全）
- [x] 6.5 交接：Moderate → PO/CE + Architect + Sprint Planning + Dev 可继续 26.1

---

**下一步请选择：** **Continue**（进入批准问答） / **Edit**（指出要改的提案条目）


## Approval Record

- **Approved by:** Richard
- **Date:** 2026-09-08
- **Decision:** yes — implement planning-artifact changes per this proposal (Moderate handoff)
