---
title: '39.4 多视图同刺激与 adapter 模板（FR92）'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: 'f934f03'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic39-ide-multiview.md'
  - '{project-root}/_agile-output/implementation-artifacts/39-1-epic-39-nfr14-风险记录.md'
  - '{project-root}/_agile-output/implementation-artifacts/39-2-宿主-ide-rust-analyzer-工作流-fr90.md'
  - '{project-root}/_agile-output/implementation-artifacts/39-3-浅层-bitloom-lsp-或显式-defer-fr91.md'
  - '{project-root}/docs/fr47-dual-sim-generation.md'
  - '{project-root}/docs/fr78-bridge-adapter-closures.md'
  - '{project-root}/docs/fr29-bridge-abstraction-both.md'
  - '{project-root}/docs/requirements/19. 实施路线图.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
  - '{project-root}/_agile-output/implementation-artifacts/process-one-story-one-commit.md'
warnings: []
deferred:
  - 'Automatic formal FL≡RTL product (explicit non-goal / AD-5)'
  - 'SystemC TLM-2.0 product path (AD-5 permanent non-contract)'
  - 'Historical cargo build-sim --kind full verb matrix (doc-19 historical vision, not P7 green)'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Wave D / P7 合同绿要求「功能路径与周期精确路径**共享刺激/记分板** + 桥接 adapter 模板」，且明确不承诺自动 FL≡RTL / SystemC TLM-2.0。既有 FR47 生成对照与 FR78 `start_wait_complete` 已具备能力，但缺少 **FR92 命名合同页**、显式同刺激记分板骨架、以及 Epic 39 NFR14 **整表关闭勾选**；易被误读为「未交付」或偷换成形式等价/TLM 产品。

**Approach:** （1）新增 `docs/fr92-shared-stimulus-adapter.md`：钉死同刺激合同、复用 FR47/FR78、AD-5 非承诺；（2）在 `bitloom-sim` 提供薄 **SharedStimulusScoreboard** 骨架 + ATDD 夹具证明同一刺激向量驱动功能 sim 与 `tick`；（3）adapter 模板指向既有 FR78（文档+交叉），不另起第二套仿真语义；（4）勾选 NFR14 Epic 39 全部关闭条件；sprint：`39-4` → done，`epic-39` → done。**不**发明新故事；**不**交付 SystemC / 自动形式等价。

## Boundaries & Constraints

**Always:** 复用 FR47 `generate_*` / `check_functional_equiv_generated` 与 FR78 adapter；同一 `Vec<PortValues>`（或记分板）驱动两侧；文档写明不承诺自动 FL≡RTL / SystemC TLM-2.0（AD-5）；勾选 NFR14 关闭清单（含 FR90/FR91/FR92/HTML≠LSP/禁止事项/品牌）；设计 crate 只依赖 `bitloom-prelude`；品牌 Bitloom；one-story-one-commit。

**Ask First:** 无（除非产品坚持把 FR92 改写成形式等价产品——须改 PRD/NFR14）。

**Never:** 引入第二套无对照仿真语义冒充完成；声称自动形式 FL≡RTL 或 SystemC TLM-2.0 产品；重开 FR47 生成器 MVP；交付 Bitloom LSP；发明 epic-39 之外新故事；留下 NFR14 FR92 / 整 epic 未勾。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 同刺激联验 | 共享 stimuli → functional + tick | Pass（PortValues 一致） | EquivStatus Fail |
| 故意破坏 | Wrong AbstractionView + 同 stimuli | Fail | ATDD |
| Adapter 模板 | FR78 `start_wait_complete` + FR92 文档交叉 | 可发现、可复用 | ATDD 文档守卫 |
| AD-5 诚实 | fr92 文档 | 禁自动 FL≡RTL / SystemC TLM | ATDD |
| Epic 关闭 | NFR14 关闭清单 | 全部 `[x]`；`epic-39: done` | ATDD |
| 范围 | 无第二仿真语义 / 无 TLM API | 仓库无 `emit_tlm` 产品路径冒充 | ATDD / 审查 |

</frozen-after-approval>

## Code Map

- `docs/fr92-shared-stimulus-adapter.md` — **NEW** FR92 合同页（同刺激 + adapter 模板 + AD-5 非承诺）
- `crates/bitloom-sim/src/shared_stimulus.rs` — **NEW** `SharedStimulusScoreboard` 薄骨架
- `crates/bitloom-sim/src/lib.rs` / `equiv.rs` — **UPDATE** 导出 / 交叉
- `crates/bitloom/tests/fr92_shared_stimulus_adapter.rs` — **NEW** ATDD
- `crates/bitloom/tests/fr90_host_ide_rust_analyzer.rs` — **UPDATE** 解除 39.4 backlog / FR92 开勾守卫
- `crates/bitloom/tests/fr91_bitloom_lsp_explicit_defer.rs` — **UPDATE** 同上（保留 FR91 `[x]`）
- `_agile-output/implementation-artifacts/nfr14-risk-epic39-ide-multiview.md` — **UPDATE** 勾选全部关闭条件
- `docs/fr47-dual-sim-generation.md` / `docs/fr78-bridge-adapter-closures.md` / `README.md` — **UPDATE** 交叉至 FR92
- `_agile-output/implementation-artifacts/sprint-status.yaml` — `39-4` → done；`epic-39` → done

## Story

As a 验证工程师 / 设计者,
I want 功能路径与周期精确路径共享刺激夹具，并有桥接 adapter 模板,
So that 多视图一致性可落地且不承诺 TLM≡CA。

## Acceptance Criteria

1. Given Story 39.1–39.3；既有 FR47 生成/手写功能模型路径与 `tick`，when 交付至少一夹具证明功能模型（或生成功能 sim）与周期精确 `tick`（及/或 `build-sim`）**共享同一刺激/期望向量**，then 自动化测试稳定通过
2. And 桥接 adapter **模板**（文档 + 代码骨架；复用/交叉 FR78 `start_wait_complete` 即可）可发现
3. And 文档写明**不**承诺自动形式 FL≡RTL / SystemC TLM-2.0（AD-5）
4. And 不得引入第二套无对照的仿真语义作为「完成」
5. And NFR14 记录勾选 Epic 39 **全部**关闭条件；sprint：`39-4: done`，`epic-39: done`

## Tasks / Subtasks

- [x] T1: FR92 合同文档 `docs/fr92-shared-stimulus-adapter.md` + README/fr47/fr78 交叉（AC: 2–4）
- [x] T2: `SharedStimulusScoreboard` 薄骨架 + ATDD 同刺激 pass/fail（AC: 1, 4）
- [x] T3: NFR14 勾选关闭清单；更新 fr90/fr91 范围守卫；sprint epic-39 done（AC: 5）
- [x] T4: code-review / automate / `just test` / 单 commit

## Dev Notes

### Decision

**Reuse, don't fork.** FR92 = Wave D **contract name** over FR47 dual-path + FR78 adapter template. Shared stimulus = one `Vec<PortValues>` (scoreboard) feeding both views. No second sim semantics. No SystemC. No automatic formal FL≡RTL product claim.

### Architecture

- AD-5: dual-model; SystemC TLM-2.0 not contracted; consistency via PortValues contrast tests.
- Design crates: `bitloom-prelude` only; scoreboard/sim helpers live in `bitloom-sim` (toolchain / test dep).
- `build-sim` historical CLI remains deferred wrapper (fr40); FR92 accepts `tick` / FR47 generate path as cycle-accurate side.

### Previous story intelligence

- 39.3 Path B defer closed (`f934f03`); fr90/fr91 ATDD still assert `39-4: backlog` and unticked FR92 — **must update** in this story.
- 30.3 `fr78_fr47_dual_view_coverify` is the technical precedent; FR92 names the product contract and scoreboard surface.

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 39 / Story 39.4 / FR92]
- [Source: `nfr14-risk-epic39-ide-multiview.md` — close checklist]
- [Source: ARCHITECTURE-SPINE AD-5]
- [Source: `docs/requirements/19. 实施路线图.md` §19.9 P7 绿]
- [Source: `process-one-story-one-commit.md`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

- ATDD red→green: `cargo test -p bitloom --test fr92_shared_stimulus_adapter` — 7 passed
- Sibling: fr90 (6), fr91 (6), nfr14_risk_epic39 (1) — passed after guard updates
- code-review: Approve (`39-4-code-review.md`)
- automate: ATDD sufficient (`39-4-automation-summary.md`)

### Completion Notes List

- FR92 合同页 + `SharedStimulusScoreboard`：同刺激驱动功能路径 vs `tick`
- Adapter 模板交叉 FR78；不承诺自动 FL≡RTL / SystemC TLM-2.0
- NFR14 Epic 39 关闭清单全勾；`epic-39: done`；`39-4: done`
- 未发明额外故事；Phase 11 stories done（retros optional）

### File List

- `docs/fr92-shared-stimulus-adapter.md`
- `crates/bitloom-sim/src/shared_stimulus.rs`
- `crates/bitloom-sim/src/lib.rs`
- `crates/bitloom/tests/fr92_shared_stimulus_adapter.rs`
- `crates/bitloom/tests/fr90_host_ide_rust_analyzer.rs`
- `crates/bitloom/tests/fr91_bitloom_lsp_explicit_defer.rs`
- `docs/fr47-dual-sim-generation.md`
- `docs/fr78-bridge-adapter-closures.md`
- `README.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic39-ide-multiview.md`
- `_agile-output/implementation-artifacts/39-4-多视图同刺激与-adapter-模板-fr92.md`
- `_agile-output/implementation-artifacts/atdd-checklist-39-4-多视图同刺激与-adapter-模板-fr92.md`
- `_agile-output/implementation-artifacts/39-4-code-review.md`
- `_agile-output/implementation-artifacts/39-4-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: Ultimate context engine analysis completed — ready-for-dev
- 2026-09-09: FR92 shared stimulus + adapter template；Epic 39 closed；review Approve；done

## Suggested Review Order

**fr92 文档（AD-5 非承诺）** → **SharedStimulusScoreboard + ATDD** → **NFR14 全勾 + epic-39 done** → **fr90/fr91 守卫更新** → **交叉链**
