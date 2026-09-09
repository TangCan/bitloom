---
title: '45.2 自动 FL≡RTL / 形式等价产品路径（FR100）'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: 'e90c337'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic45-formal-equiv-dual-model.md'
  - '{project-root}/_agile-output/implementation-artifacts/45-1-epic-45-nfr14-风险记录.md'
  - '{project-root}/docs/fr92-shared-stimulus-adapter.md'
  - '{project-root}/docs/fr30-dual-view-equiv.md'
  - '{project-root}/docs/fr47-dual-sim-generation.md'
  - '{project-root}/docs/fr39-formal-sva.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
  - '{project-root}/_agile-output/implementation-artifacts/process-one-story-one-commit.md'
warnings: []
deferred:
  - 'FR102 multi-view attribute full matrix (Story 45.3)'
  - 'FR103 first-class IP dual-model completeness + Epic 45 closeout (Story 45.4)'
  - 'SystemC TLM-2.0 product path (FR101 / Epic 46)'
  - 'Full-chip unbounded SMT / commercial SymbiYosys proof suite (F4 out of MVP)'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Path B / FR100 要求产品化的「自动随机/对照 + 形式等价（或绑定形式工具）」路径，使字面绿不再把 FL≡RTL 证明产品永久排除。既有 FR92 `SharedStimulusScoreboard` 与 FR30 bounded checker 仅是同刺激 PortValues 对照——**不得单独关闭 FR100**（NFR14 F5）。缺少：钉死 F1 工具支的合同文档、超出「仅随机记分板」的形式/有界证明产品入口、以及两条 ATDD 可复现夹具（随机/对照 + 形式入口）与可读失败。

**Approach:** （1）新增 `docs/fr100-formal-equiv.md`：钉死 **F1 支 (i)**——树内形式/有界证明检查器 API（超出仅 PortValues 随机记分板）；声明本页为 FR100 完成面；明确 FR92/FR30 为配套非充分；遵守 F2–F5。（2）在 `bitloom-sim` 交付薄产品 API：**(a)** 自动随机/对照路径（可复用 FR47 生成功能 vs `tick`，随机刺激）；**(b)** 有界穷举形式等价产品入口（文档钉死的小输入字母表 × 深度 K，穷举 FL≡tick，非随机采样）。（3）ATDD：可复现 Pass + 故意不一致 Fail（可读诊断）+ 文档诚实守卫（不得用 FR92 alone 冒充 FR100）。（4）sprint：`45-2` → done；**保持** `45-3`/`45-4` backlog；`epic-45` 仍 in-progress。**不**实现 FR102/FR103；**不**勾选 Epic 45 关闭。

## Boundaries & Constraints

**Always:** 遵守 NFR14 F1–F5；文档钉死 F1=(i)；≥1 自动随机/对照夹具 + ≥1 形式/有界证明产品入口夹具；ATDD 可复现关闭 + Fail 可读；文档声明本路径为 FR100 完成面（超出 FR92）；设计 crate 只依赖 `bitloom-prelude`；品牌 Bitloom；one-story-one-commit；AD-17 PortValues 观测面。

**Ask First:** 若产品改选 F1=(ii) 外部 SymbiYosys/SMT 为唯一完成支（须改 NFR14 F1 叙述与 docs）；若要把全芯片无界证明纳入 MVP（违反 F4）。

**Never:** 仅用 FR92/FR30 随机记分板勾选 FR100（F5）；实现 FR102 属性全矩阵（→45.3）；实现 FR103 IP 双模型或勾选 Epic 45 关闭（→45.4）；把 SystemC TLM（Epic 46）并入本故事；无入口宣称 FR100；把 FR85 SVA lint 冒充 FL≡RTL 形式等价完成面。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 随机/对照 Pass | 固定 seed + 生成功能 vs tick | `EquivStatus::Pass` 可复现 | N/A |
| 随机/对照 Fail | 故意错误 AbstractionView / 破坏观测 | `Fail { cycle, mismatches }` 可读 | ATDD |
| 有界穷举 Pass | 文档字母表 × 深度 K；FL≡tick | Pass；穷尽边界内组合 | N/A |
| 有界穷举 Fail | 同边界内注入不一致 | Fail + 可读诊断 | ATDD |
| 文档诚实 | fr100 合同页 | 声明 F1=(i)；FR92 配套非充分；本页=FR100 完成面 | ATDD 字符串守卫 |
| 范围（F4） | 超出钉死模块/深度 | 文档写明非目标；API/文档不静默宣称全芯片 | 审查 |
| 非本故事 | FR102/103 / epic-45 done | sprint 仍 backlog / in-progress | 不得误改 |

</frozen-after-approval>

## Code Map

- `docs/fr100-formal-equiv.md` — **NEW** FR100 合同页（F1=(i)；双路径；FR92 非充分；完成面声明）
- `crates/bitloom-sim/src/formal_equiv.rs` — **NEW** 随机/对照 + 有界穷举形式等价产品 API
- `crates/bitloom-sim/src/lib.rs` — **UPDATE** 导出
- `crates/bitloom-sim/src/shared_stimulus.rs` / `equiv.rs` — **UPDATE** 交叉注释（FR92/FR30 ≠ FR100 完成）
- `crates/bitloom/tests/fr100_formal_equiv_product.rs` — **NEW** ATDD
- `docs/fr92-shared-stimulus-adapter.md` / `docs/fr30-dual-view-equiv.md` — **UPDATE** 交叉至 FR100（诚实：FR92/FR30 仍非 FR100 alone）
- `README.md`（若已有 FR92 索引）— **UPDATE** 链至 fr100
- `_agile-output/implementation-artifacts/nfr14-risk-epic45-formal-equiv-dual-model.md` — **UPDATE** 可选勾选 45.2 关闭项（若记录有逐故事勾选；**不**勾选 Epic 45 整表关闭）
- `_agile-output/implementation-artifacts/sprint-status.yaml` — `45-2` → done；`45-3`/`45-4` backlog；`epic-45` in-progress

## Story

As a 验证工程师,
I want 产品化的自动比对 + 形式等价检查路径,
So that 字面绿不再排除 TLM≡CA/FL≡RTL 证明产品。

## Acceptance Criteria

1. Given Story 45.1（NFR14 F1–F5 已钉死）, when 交付文档化流程 + 可运行夹具：至少一条自动随机/对照路径，以及一条形式等价（树内有界证明）产品入口, then ATDD 证明该路径可复现关闭；失败可读
2. And 文档明确本路径为 FR100 完成面（超出 FR92 同刺激记分板）；不得声称 FR92 alone 即 FR100
3. And 遵守 F1–F5；钉死 F1=(i)；不实现 FR102/FR103；不关闭 Epic 45

## Tasks / Subtasks

- [x] T1: `docs/fr100-formal-equiv.md` + fr92/fr30/README 交叉（AC: 1–2）
- [x] T2: `bitloom-sim` 随机/对照 API + 有界穷举形式入口 API（AC: 1, F1–F3）
- [x] T3: ATDD `fr100_formal_equiv_product` — Pass/Fail 可读 + 文档诚实守卫（AC: 1–2）
- [x] T4: sprint `45-2: done`；保持 `45-3`/`45-4` backlog；`epic-45` in-progress（AC: 3）
- [x] T5: code-review / automate / `cargo clean && cargo fmt --all && just test` / 单 commit

## Dev Notes

### Decision — F1 branch (i)

**Select in-tree bounded formal/prover API (F1-i), not external SymbiYosys-only (F1-ii).**

Rationale: `just test` / CI must reproduce FR100 close without requiring `sby` installed. FR85 already documents external SVA checker binding; FR100 object is **FL ≡ cycle-accurate tick**, not SVA lint. Exhaustive enumeration over a **doc-pinned small input alphabet × depth K** is clearly beyond “PortValues random scoreboard alone” (F5 / F1-i).

### Architecture

- Host APIs in **`bitloom-sim`** (toolchain). Design crates stay on **`bitloom-prelude` only** (AD-6).
- Observation surface remains **`PortValues`** (AD-17); reuse `EquivStatus` / `PortMismatch` / FR47 `check_functional_equiv_generated`.
- Random path may wrap/reuse FR92 stimuli machinery but docs must say **配套非充分**.
- Formal product entry must **not** be a thin rename of `SharedStimulusScoreboard::check_generated` — it must expose bounded-exhaustive (or equivalent prover) semantics distinct from random sampling.
- AD-5 revised: default TLM≡CA formal proof maps to FR100; SystemC TLM **product** remains Epic 46 — do not emit TLM.

### Previous story intelligence (45.1 / `e90c337`)

- NFR14 record: `nfr14-risk-epic45-formal-equiv-dual-model.md` with F1–F5, forbid random scoreboard alone, gate 45.2–45.4.
- ATDD: `nfr14_risk_epic45_formal_equiv_dual_model.rs` — keep green; do not weaken F5 wording.
- Sprint: `epic-45: in-progress`, `45-1: done`, `45-2` was backlog → this story moves it to done at end.
- Pattern: 39.4 FR92 = docs + thin sim API + `crates/bitloom/tests/fr92_*.rs` + honesty guards.

### Git intelligence

- Recent: `e90c337` Epic 45 NFR14; `bd1b42f` Epic 44 close; FR99 LSP path.
- Follow one-story-one-commit; Chinese/English commit style matching recent (“Deliver …” / “Epic …”).

### Suggested API sketch (non-binding; implementer may refine names)

```rust
// crates/bitloom-sim/src/formal_equiv.rs
pub struct FormalEquivProduct { /* seed / alphabet / depth */ }
impl FormalEquivProduct {
    /// F3 companion: automatic random PortValues compare (FL vs tick).
    pub fn check_random_compare(&self, hir: FrozenHir) -> EquivStatus { … }
    /// F1-(i) product entry: exhaustive bounded FL≡tick over pinned alphabet×depth.
    pub fn check_bounded_exhaustive(&self, hir: FrozenHir) -> EquivStatus { … }
}
```

Deliberate mismatch fixtures must surface `Fail { cycle, mismatches }` with human-readable port diffs (existing `PortMismatch`).

### Testing

- Primary: `cargo test -p bitloom --test fr100_formal_equiv_product`
- Full gate: `cargo clean && cargo fmt --all && just test`
- Do not require `sby`/SymbiYosys for green.

### Project Structure Notes

- Docs: `docs/fr100-formal-equiv.md` (root fr* pattern)
- Sim: `crates/bitloom-sim/src/formal_equiv.rs`
- ATDD: `crates/bitloom/tests/fr100_formal_equiv_product.rs`
- No changes under `bitloom-prelude` unless unavoidable (prefer none)

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 45 / Story 45.2 / FR100]
- [Source: `nfr14-risk-epic45-formal-equiv-dual-model.md` — F1–F5]
- [Source: `docs/fr92-shared-stimulus-adapter.md` — supporting ≠ sufficient]
- [Source: `docs/fr30-dual-view-equiv.md` — bounded checker precedent]
- [Source: ARCHITECTURE-SPINE AD-5 / AD-17 / AD-6]
- [Source: `process-one-story-one-commit.md`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

### Completion Notes List

- 钉死 F1=(i)：`FormalEquivProduct` 有界穷举 FL≡tick + 随机/对照配套路径
- `docs/fr100-formal-equiv.md` 声明 FR100 完成面；FR92 配套非充分
- ATDD 8 项绿；NFR14 勾选 45.2/FR100；sprint 45-2 done，epic-45 in-progress

### File List

- `docs/fr100-formal-equiv.md`
- `crates/bitloom-sim/src/formal_equiv.rs`
- `crates/bitloom-sim/src/lib.rs`
- `crates/bitloom-sim/src/shared_stimulus.rs`
- `crates/bitloom/tests/fr100_formal_equiv_product.rs`
- `docs/fr92-shared-stimulus-adapter.md`
- `docs/fr30-dual-view-equiv.md`
- `README.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic45-formal-equiv-dual-model.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `_agile-output/implementation-artifacts/45-2-自动-fl-rtl-形式等价产品路径-fr100.md`
- `_agile-output/implementation-artifacts/45-2-code-review.md`
- `_agile-output/implementation-artifacts/45-2-automation-summary.md`
- `_agile-output/implementation-artifacts/atdd-checklist-45-2-自动-fl-rtl-形式等价产品路径-fr100.md`

## Change Log

- 2026-09-09: Story context created (ready-for-dev) — FR100 formal-equiv product path
- 2026-09-09: Implemented FormalEquivProduct + docs + ATDD; review Approve; automate sufficient; done

## Suggested Review Order

**fr100 合同（F1=(i) / FR92 非充分）** → **formal_equiv API（随机 + 有界穷举）** → **ATDD Pass/Fail** → **sprint（仅 45-2 done）**
