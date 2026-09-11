---
title: '84.3 同步 README / deferred / 路线图指针（FR148）'
type: 'chore'
created: '2026-09-11'
status: 'done'
route: 'oneshot'
baseline_commit: 'ab5c77f Story 84.2: Verify Correct Course + PRD Phase 18 FR148 gate stamps.'
review_loop_iteration: 0
context:
  - '{project-root}/README.md'
  - '{project-root}/_agile-output/implementation-artifacts/deferred-work.md'
  - '{project-root}/docs/requirements/19. 实施路线图.md'
  - '{project-root}/docs/fr146-bitloom-1-0-0-release.md'
  - '{project-root}/crates/bitloom/tests/fr141_readme_deferred_honesty.rs'
  - '{project-root}/_agile-output/implementation-artifacts/84-2-correct-course-prd-批准-phase-18-fr148.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** README 仍以 Phase 17 库 1.0 为最新安装叙事（含 `cargo install bitloom` 快速开始），未区分 Phase 18 / FR148–153，易把「库已 1.0」误读为 CLI 可安装；NFR59 deferred 须在 CLI 合同公开面继续诚实列出。

**Approach:** 更新 README「状态与 deferred」与 Phase 18 CLI 表、快速开始诚实面；doc-19 / deferred / fr146 交叉；ATDD 锁住诚实面（镜像 `fr141_readme_deferred_honesty`）。**不**勾选 Epic 84 关闭（→ 84.4）；**不**修订 ARCHITECTURE-SPINE Deferred 收口（→ 84.4）。

## Boundaries & Constraints

**Always:** Phase 17 vs Phase 18 完成面清晰；FR148–153 映射；不得把库 1.0 alone 写成 CLI 已上架；诚实列出 NFR59；品牌 Bitloom；84.2 合同已批准。

**Ask First:** 若撤回 Phase 18 公开面 — 须改 Correct Course / PRD 与本故事。

**Never:** 勾选 Epic 84 关闭；修订 ARCHITECTURE-SPINE AD 收口（→ 84.4）；将 epic-85..86 标 ready；改写 Phase 12–17 关闭为失败；宣称 CLI 已可 `cargo install`；执行 rename/`cargo publish`（→ Epic 85）。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 诚实面齐全 | README+deferred+doc-19 含 Phase 18 / FR148–153 / NFR59 / FR151 | ATDD 绿 | N/A |
| 缺 Phase 18 / FR151 / NFR59 | 任一缺失 | ATDD 红 | 补齐公开指针 |
| 库 1.0 alone 冒充 CLI | README 暗示 cargo install 已可用 | ATDD 红 | 加「不得」纪律句 |
| 84.2 未 done | sprint 缺 84-2 done | 本故事不得标 done | 先完成 84.2 |

</frozen-after-approval>

## Code Map

- `README.md` — 快速开始诚实面；Phase 17→18；Phase 18 表 + FR151 纪律
- `_agile-output/implementation-artifacts/deferred-work.md` — Phase 18 pointer 加厚
- `docs/requirements/19. 实施路线图.md` — Phase 18 交叉链
- `docs/fr146-bitloom-1-0-0-release.md` — CLI → FR151 诚实
- `crates/bitloom/tests/fr148_readme_deferred_honesty.rs` — 本故事新建
- `_agile-output/implementation-artifacts/sprint-status.yaml` — 84-3 键

## Story

As a 文档维护者,
I want 公开状态页区分「库 1.0 已上架」与「CLI 可发布合同」,
So that 对外不暗示 `cargo install bitloom` 已可用（在 FR151 关闭前）。

## Acceptance Criteria

1. README「状态与 deferred」明确 Phase 17 vs Phase 18 完成面与 FR148–153 映射
2. 诚实写出：库 crate 已 1.0.0；CLI 仍待 FR151；NFR59 仍 deferred
3. deferred-work /（若需）doc-19 / Release 指针与 README 同源
4. 公开品牌 Bitloom
5. Story 84.2 已 done（软闸门）

## Tasks / Subtasks

- [x] T1: README Phase 18 + FR149–153 表 + 快速开始诚实面（AC: 1–2, 4）
- [x] T2: deferred Phase 18 pointer 加厚（AC: 2–3）
- [x] T3: doc-19 / fr146 Phase 18 交叉链（AC: 3）
- [x] T4: ATDD `fr148_readme_deferred_honesty.rs`（AC: 1–4）
- [x] T5: sprint `84-3` done；epic-84 保持 in-progress；不标 85–86 ready；code-review Approve

## Dev Notes

- **上下文：** Story 84.2 已批准 Correct Course + PRD Phase 18（`ab5c77f`）。本故事 = **公开诚实门面**。
- **不**做 84.4 AD 指针 / Epic 84 关闭勾选。
- 镜像 Story 79.3 / `fr141_readme_deferred_honesty.rs`。
- 品牌 Bitloom；设计 crate → `bitloom-prelude` only。

## Testing

- `cargo test -p bitloom --test fr148_readme_deferred_honesty`
- 回归：`cargo clean && cargo fmt --all && just test`

## Dev Agent Record

### Completion Notes List

- README Phase 18 摘要 + FR149–153 表 + 快速开始诚实面已落地
- deferred / doc-19 / fr146 指针；禁 cargo install before FR151
- ATDD 9/9 绿；code-review Approve；automation-summary 已写
- sprint：84-3 done；epic-84 in-progress；85–86 仍 backlog

### File List

- `README.md`
- `docs/requirements/19. 实施路线图.md`
- `docs/fr146-bitloom-1-0-0-release.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `crates/bitloom/tests/fr148_readme_deferred_honesty.rs`
- `crates/bitloom/tests/fr141_readme_deferred_honesty.rs`
- `_agile-output/implementation-artifacts/84-3-同步-readme-deferred-路线图指针-fr148.md`
- `_agile-output/implementation-artifacts/84-3-code-review.md`
- `_agile-output/implementation-artifacts/84-3-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
