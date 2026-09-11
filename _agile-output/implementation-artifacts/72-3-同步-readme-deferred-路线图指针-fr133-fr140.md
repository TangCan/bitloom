---
title: '72.3 同步 README / deferred / 路线图指针（FR133 / FR140）'
type: 'chore'
created: '2026-09-11'
status: 'done'
route: 'oneshot'
baseline_commit: 'f6a35af'
review_loop_iteration: 0
context:
  - '{project-root}/README.md'
  - '{project-root}/_agile-output/implementation-artifacts/deferred-work.md'
  - '{project-root}/docs/requirements/19. 实施路线图.md'
  - '{project-root}/crates/bitloom/tests/fr124_readme_deferred_honesty.rs'
  - '{project-root}/_agile-output/implementation-artifacts/72-2-correct-course-prd-批准-phase-16-fr133.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** README 仍以 Phase 15 为最新加深叙事，未区分 Phase 16 / FR133–140，易把 FR125–131 alone 误读为 Phase 16 / 终局完成面；NFR59 deferred 未在公开状态页诚实列出。

**Approach:** 更新 README「状态与 deferred」与 Phase 16 加深表、FR140 宣称纪律；doc-19 交叉链；核对 deferred Phase 16 pointer（Correct Course 已 seed）；ATDD 锁住诚实面（镜像 `fr124_readme_deferred_honesty`）。**不**勾选 Epic 72 关闭（→ 72.4）；**不**修订 ARCHITECTURE-SPINE Deferred 收口（→ 72.4）。

## Boundaries & Constraints

**Always:** Phase 15 vs Phase 16 完成面清晰；FR133–140 映射；不得把 FR125–131 alone 写成 Phase 16/终局；诚实列出 NFR59；品牌 Bitloom；72.2 合同已批准。

**Ask First:** 若撤回 Phase 16 公开面 — 须改 Correct Course / PRD 与本故事。

**Never:** 勾选 Epic 72 关闭；修订 ARCHITECTURE-SPINE AD 收口（→ 72.4）；将 epic-73..78 标 ready；改写 Phase 12–15 关闭为失败；宣称终局 = 冲 1.0。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 诚实面齐全 | README+deferred+doc-19 含 Phase 16 / FR133–140 / NFR59 | ATDD 绿 | N/A |
| 缺 Phase 16 / FR140 / NFR59 | 任一缺失 | ATDD 红 | 补齐公开指针 |
| FR125–131 alone 冒充终局 | README 暗示 Phase 16 已由 Phase 15 FR 完成 | ATDD 红 | 加「不得」纪律句 |
| 72.2 未 done | sprint 缺 72-2 done | 本故事不得标 done | 先完成 72.2 |

</frozen-after-approval>

## Code Map

- `README.md` — 「状态与 deferred」；Phase 15 段 → 指向 Phase 16；新增 Phase 16 摘要 + 加深表 + FR140
- `_agile-output/implementation-artifacts/deferred-work.md` — Phase 16 pointer（已 seed；核对 NFR59 诚实面）
- `docs/requirements/19. 实施路线图.md` — Phase 16 交叉链（不重定义 §19.7–19.9 MVP）
- `crates/bitloom/tests/fr124_readme_deferred_honesty.rs` — Phase 15 ATDD 样板（保留）
- `crates/bitloom/tests/fr133_readme_deferred_honesty.rs` — 本故事新建
- `_agile-output/implementation-artifacts/sprint-status.yaml` — 72-3 键

## Story

As a 文档维护者,
I want 公开状态页区分 Phase 15 完成面与 Phase 16 终局加深面,
So that 对外宣称不混淆。

## Acceptance Criteria

1. README「状态与 deferred」明确 Phase 15 vs Phase 16 完成面与 FR133–140 映射
2. 不得把 FR125–131 alone 写成 Phase 16 / 终局完成面
3. 诚实列出 NFR59 仍 deferred 项
4. deferred-work /（若需）doc-19 指针与 README 同源
5. 公开品牌 Bitloom
6. Story 72.2 已 done（软闸门）

## Tasks / Subtasks

- [x] T1: README Phase 16 + FR134–139 表 + FR140（AC: 1–3, 5）
- [x] T2: deferred Phase 16 pointer 核对 / 微调 NFR59（AC: 3–4）
- [x] T3: doc-19 Phase 16 交叉链（AC: 4）
- [x] T4: ATDD `fr133_readme_deferred_honesty.rs`（AC: 1–5）
- [x] T5: sprint `72-3` done；epic-72 保持 in-progress；不标 73–78 ready；code-review Approve

## Dev Notes

- **上下文：** Story 72.2 已批准 Correct Course + PRD Phase 16（`f6a35af`）；`correctCoursePhase16Approved: 2026-09-11`；deferred 已有 Phase 16 pointer（`dbe9b55`）。本故事 = **公开诚实门面**（README / deferred / doc-19 + ATDD），非重写合同。
- **不**做 72.4 AD 指针 / Epic 72 关闭勾选。
- 镜像 Story 64.3 / `fr124_readme_deferred_honesty.rs`。
- Phase 15 加深表与 FR132 **保留**；在 Phase 15 摘要段注明原 NFR55 剩余已另开 Phase 16。
- Phase 16 加深表列 FR134–139（Epic 73–78）；闸门仍 Epic 72 / FR133（未关闭）；宣称→FR140。
- NFR59 至少列：自动 FSM 标签提取、第三方 LCOV GUI 一等、emit MemRead stub→完整生成、非 Cargo 全 monorepo 任意路径扫描、GHA formal-sby 镜像卫生。
- 品牌 Bitloom；设计 crate → `bitloom-prelude` only（本故事不改设计 crate）。

## Testing

- `cargo test -p bitloom --test fr133_readme_deferred_honesty`
- 回归：`cargo clean && cargo fmt --all && just test`

## Dev Agent Record

### Completion Notes List

- README Phase 16 摘要 + FR134–139 表 + FR140 / NFR59 诚实面已落地
- deferred Phase 16 pointer 强化 FR125–131 alone 禁令与 Bitloom；doc-19 交叉链
- ATDD 9/9 绿；code-review Approve；automation-summary 已写
- sprint：72-3 done；epic-72 in-progress；73–78 仍 backlog

### File List

- `README.md`
- `docs/requirements/19. 实施路线图.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `crates/bitloom/tests/fr133_readme_deferred_honesty.rs`
- `_agile-output/implementation-artifacts/72-3-同步-readme-deferred-路线图指针-fr133-fr140.md`
- `_agile-output/implementation-artifacts/72-3-code-review.md`
- `_agile-output/implementation-artifacts/72-3-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
