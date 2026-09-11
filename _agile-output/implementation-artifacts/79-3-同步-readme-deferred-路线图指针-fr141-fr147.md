---
title: '79.3 同步 README / deferred / 路线图指针（FR141 / FR147）'
type: 'chore'
created: '2026-09-11'
status: 'done'
route: 'oneshot'
baseline_commit: '939e514 Story 79.2: Verify Correct Course + PRD Phase 17 FR141 gate stamps.'
review_loop_iteration: 0
context:
  - '{project-root}/README.md'
  - '{project-root}/_agile-output/implementation-artifacts/deferred-work.md'
  - '{project-root}/docs/requirements/19. 实施路线图.md'
  - '{project-root}/crates/bitloom/tests/fr133_readme_deferred_honesty.rs'
  - '{project-root}/_agile-output/implementation-artifacts/79-2-correct-course-prd-批准-phase-17-fr141.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** README 仍以 Phase 16 终局为最新叙事，未区分 Phase 17 / FR141–147，易把 Phase 16 alone 误读为 1.0 / 公开 API 稳定；NFR59 deferred 须在稳定门公开面继续诚实列出。

**Approach:** 更新 README「状态与 deferred」与 Phase 17 稳定门表、FR147 宣称纪律；doc-19 交叉链；deferred Phase 17 pointer；ATDD 锁住诚实面（镜像 `fr133_readme_deferred_honesty`）。**不**勾选 Epic 79 关闭（→ 79.4）；**不**修订 ARCHITECTURE-SPINE Deferred 收口（→ 79.4）。

## Boundaries & Constraints

**Always:** Phase 16 vs Phase 17 完成面清晰；FR141–147 映射；不得把 Phase 16 终局 alone 写成 1.0；诚实列出 NFR59；品牌 Bitloom；79.2 合同已批准。

**Ask First:** 若撤回 Phase 17 公开面 — 须改 Correct Course / PRD 与本故事。

**Never:** 勾选 Epic 79 关闭；修订 ARCHITECTURE-SPINE AD 收口（→ 79.4）；将 epic-80..83 标 ready；改写 Phase 12–16 关闭为失败；宣称 Phase 16 alone = 1.0；执行 `cargo publish` / 升 1.0.0（→ Epic 83）。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 诚实面齐全 | README+deferred+doc-19 含 Phase 17 / FR141–147 / NFR59 | ATDD 绿 | N/A |
| 缺 Phase 17 / FR147 / NFR59 | 任一缺失 | ATDD 红 | 补齐公开指针 |
| Phase 16 alone 冒充 1.0 | README 暗示 1.0 已由 Phase 16 完成 | ATDD 红 | 加「不得」纪律句 |
| 79.2 未 done | sprint 缺 79-2 done | 本故事不得标 done | 先完成 79.2 |

</frozen-after-approval>

## Code Map

- `README.md` — 「状态与 deferred」；Phase 16 段 → 指向 Phase 17；新增 Phase 17 摘要 + 稳定门表 + FR147
- `_agile-output/implementation-artifacts/deferred-work.md` — Phase 17 pointer
- `docs/requirements/19. 实施路线图.md` — Phase 17 交叉链（不重定义 §19.7–19.9 MVP）
- `crates/bitloom/tests/fr133_readme_deferred_honesty.rs` — Phase 16 ATDD 样板（保留）
- `crates/bitloom/tests/fr141_readme_deferred_honesty.rs` — 本故事新建
- `_agile-output/implementation-artifacts/sprint-status.yaml` — 79-3 键

## Story

As a 文档维护者,
I want 公开状态页区分 Phase 16 终局面与 Phase 17 1.0 稳定门,
So that 对外宣称不混淆。

## Acceptance Criteria

1. README「状态与 deferred」明确 Phase 16 vs Phase 17 完成面与 FR141–147 映射
2. 不得把 Phase 16 终局 alone 写成 1.0 / 公开 API 稳定
3. 诚实列出 NFR59 仍 deferred 项
4. deferred-work /（若需）doc-19 指针与 README 同源
5. 公开品牌 Bitloom
6. Story 79.2 已 done（软闸门）

## Tasks / Subtasks

- [x] T1: README Phase 17 + FR142–146 表 + FR147（AC: 1–3, 5）
- [x] T2: deferred Phase 17 pointer（AC: 3–4）
- [x] T3: doc-19 Phase 17 交叉链（AC: 4）
- [x] T4: ATDD `fr141_readme_deferred_honesty.rs`（AC: 1–5）
- [x] T5: sprint `79-3` done；epic-79 保持 in-progress；不标 80–83 ready；code-review Approve

## Dev Notes

- **上下文：** Story 79.2 已批准 Correct Course + PRD Phase 17（`939e514`）；`correctCoursePhase17Approved: 2026-09-11`。本故事 = **公开诚实门面**（README / deferred / doc-19 + ATDD），非重写合同。
- **不**做 79.4 AD 指针 / Epic 79 关闭勾选。
- 镜像 Story 72.3 / `fr133_readme_deferred_honesty.rs`。
- Phase 16 加深表与 FR140 **保留**；在 Phase 16 摘要段注明 1.0 稳定门另开 Phase 17。
- Phase 17 表列 FR142–146（Epic 80–83）；闸门仍 Epic 79 / FR141（未关闭）；宣称→FR147。
- NFR59 至少继续列：自动 FSM 标签、第三方 LCOV GUI 一等、emit MemRead stub→完整生成、非 Cargo 全 monorepo 任意路径扫描、GHA formal-sby 镜像卫生。
- 品牌 Bitloom；设计 crate → `bitloom-prelude` only（本故事不改设计 crate）。
- 当前仍为 **0.x** 直至 FR146 / Epic 83。

## Testing

- `cargo test -p bitloom --test fr141_readme_deferred_honesty`
- 回归：`cargo clean && cargo fmt --all && just test`

## Dev Agent Record

### Completion Notes List

- README Phase 17 摘要 + FR142–146 表 + FR147 / NFR59·NFR63 诚实面已落地
- deferred Phase 17 pointer；doc-19 交叉链；禁 Phase 16 alone→1.0
- ATDD 9/9 绿；code-review Approve；automation-summary 已写
- sprint：79-3 done；epic-79 in-progress；80–83 仍 backlog

### File List

- `README.md`
- `docs/requirements/19. 实施路线图.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `crates/bitloom/tests/fr141_readme_deferred_honesty.rs`
- `_agile-output/implementation-artifacts/79-3-同步-readme-deferred-路线图指针-fr141-fr147.md`
- `_agile-output/implementation-artifacts/79-3-code-review.md`
- `_agile-output/implementation-artifacts/79-3-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
