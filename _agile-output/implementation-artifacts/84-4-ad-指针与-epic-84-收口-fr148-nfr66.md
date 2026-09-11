---
title: '84.4 AD 指针与 Epic 84 收口（FR148 / NFR66）'
type: 'chore'
created: '2026-09-11'
status: 'done'
route: 'oneshot'
baseline_commit: '82a8ea9 Story 84.3: Distinguish Phase 17 library 1.0 from Phase 18 CLI honesty.'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
  - '{project-root}/AGENTS.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic84-phase18-cli-crates-io-publish.md'
  - '{project-root}/_agile-output/implementation-artifacts/79-4-ad-指针与-epic-79-收口-fr141-fr147-nfr62.md'
  - '{project-root}/crates/bitloom/tests/fr141_ad_pointer_epic79_close.rs'
  - '{project-root}/_agile-output/implementation-artifacts/84-3-同步-readme-deferred-路线图指针-fr148.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** README/deferred 诚实面已落地，但 NFR14 / sprint / AGENTS 未勾选 Epic 84 关闭 — 85–86 仍被闸门挡住。

**Approach:** 脊柱 Deferred 声明 Epic 84 已关闭；AGENTS Phase 18 指针；AD-2 Phase 18 注记；NFR14 全勾；`epic-84: done`；85.1 → ready-for-dev；85.2+ / 86.* 保持 backlog（直至各 epic 自有 NFR14）。

## Boundaries & Constraints

**Always:** Spine/AGENTS Phase 18 指针；AD-2 `bitloom-*`；NFR14 关闭勾选；Phase 18 闸门已开声明；Epic 85–86 仍须各自 NFR14；NFR64 Phase 12–17 关闭仍有效；品牌 Bitloom；84.1–84.3 已 done；实发属 Epic 85。

**Ask First:** 若撤回 Phase 18 闸门关闭 — 须改 Correct Course / PRD 与本故事。

**Never:** 将 85.2+ / Epic 86 标 ready（本故事内）；改写 Phase 12–17 关闭为失败；宣称 FR149–153 已关闭；执行 rename/`cargo publish`。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 收口齐全 | spine+AGENTS+NFR14+sprint epic-84 done | ATDD 绿；闸门已开 | N/A |
| 缺 Epic 84 关闭 / 勾选 | 任一缺失 | ATDD 红 | 补齐指针与勾选 |
| 85.2+ 被标 ready | sprint 含 ready-for-dev | ATDD 红 | 保持 backlog |
| 84.3 未 done | 诚实面未齐 | 本故事不得标 done | 先完成 84.3 |

</frozen-after-approval>

## Code Map

- `ARCHITECTURE-SPINE.md` — Deferred Phase 18：闸门 Epic 84 / FR148 **已关闭**；AD-2 Phase 18 指针
- `AGENTS.md` — Phase 18 / FR148–153 指针
- `nfr14-risk-epic84-phase18-cli-crates-io-publish.md` — 关闭条件全勾 + 闸门已开声明
- `epics.md` — `phase18Epic84Status: complete`
- `deferred-work.md` / `README.md` — 闸门已关闭措辞
- `sprint-status.yaml` — `epic-84: done`；`84-4: done`；`85-1: ready-for-dev`；85.2+ backlog
- `crates/bitloom/tests/fr148_ad_pointer_epic84_close.rs` — 本故事 ATDD

## Story

As a 架构维护者,
I want 脊柱/AGENTS 指针与 Epic 84 关闭勾选就绪,
So that 后续实现 epic 有合法 AD-2 / 发布名引用面。

## Acceptance Criteria

1. 更新 ARCHITECTURE-SPINE / AGENTS（若需）Phase 18 指针
2. 引用 AD-2（对外 `bitloom-*`；禁 `rhdl`/`rhdl-bits`）；NFR64/NFR67 诚实面保留
3. 勾选 Epic 84 / FR148 关闭条件（NFR14）
4. NFR14 声明 Phase 18 闸门已开；Epic 85–86 仍须各自 NFR14；实发属 Epic 85
5. sprint：`epic-84: done`；`84-4: done`；85.1 ready-for-dev；85.2+ / 86 backlog
6. ATDD 镜像 `fr141_ad_pointer_epic79_close` → `fr148_ad_pointer_epic84_close`

## Tasks / Subtasks

- [x] T1: Spine / AGENTS / deferred / README 闸门关闭声明
- [x] T2: NFR14 关闭勾选；epics `phase18Epic84Status: complete`
- [x] T3: sprint epic-84 + 84-4 done；85-1 ready-for-dev；85.2+ backlog
- [x] T4: ATDD `fr148_ad_pointer_epic84_close.rs`；code-review Approve；automation-summary

## Dev Notes

- **上下文：** 84.1–84.3 done；Correct Course Phase 18 approved 2026-09-11；本故事 = **闸门收口**。
- 镜像 Story 79.4 / `fr141_ad_pointer_epic79_close.rs`。
- **不**实现 FR149–153；**不**将 85.2+ 标 ready。
- NFR66：本故事落 AD-2 指针；rename/publish 实质仍归 Epic 85。

## Testing

- `cargo test -p bitloom --test fr148_ad_pointer_epic84_close`
- 回归：`cargo clean && cargo fmt --all && just test`

## Dev Agent Record

### Completion Notes List

- Epic 84 / FR148 闸门关闭；Phase 18 闸门已开；85.1 ready-for-dev；85.2+ backlog
- NFR64 Phase 12–17 隔离保留；NFR14 全勾；ATDD 绿；code-review Approve

### File List

- `AGENTS.md`
- `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic84-phase18-cli-crates-io-publish.md`
- `_agile-output/planning-artifacts/epics.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `README.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `crates/bitloom/tests/fr148_ad_pointer_epic84_close.rs`
- `crates/bitloom/tests/nfr14_risk_epic84_phase18_cli_crates_io_publish.rs`
- `_agile-output/implementation-artifacts/84-4-ad-指针与-epic-84-收口-fr148-nfr66.md`
- `_agile-output/implementation-artifacts/84-4-code-review.md`
- `_agile-output/implementation-artifacts/84-4-automation-summary.md`
