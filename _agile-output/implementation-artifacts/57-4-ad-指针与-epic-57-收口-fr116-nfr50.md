---
title: '57.4 AD 指针与 Epic 57 收口（FR116 / NFR50）'
type: 'chore'
created: '2026-09-10'
status: 'done'
baseline_commit: 'b91819e'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic57-phase14-nfr47-deferred-deepen.md'
  - '{project-root}/_agile-output/implementation-artifacts/48-4-ad-指针与-epic-48-收口-fr106.md'
  - '{project-root}/_agile-output/implementation-artifacts/57-3-同步-readme-deferred-路线图指针-fr116-fr123.md'
  - '{project-root}/crates/bitloom/tests/fr106_ad_pointer_epic48_close.rs'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** README/deferred 诚实面已落地（57.3），但 ARCHITECTURE-SPINE Deferred 仍以 Phase 13 为「现行加深面」，且 NFR14 / sprint 未勾选 Epic 57 关闭 — 58–63 仍被闸门挡住，且 NFR50 AD 修订门禁不可检查。

**Approach:** 按 Story 48.4 模式收口：脊柱 Deferred 声明 Phase 14 合同 +「实现 epic 内修订 AD-25/27 / formal」指针（NFR50）；勾选 NFR14 Epic 57 关闭条件；`epic-57: done`；Epic 58–63 **保持 backlog**（关闭后可离开永久冻结，但仍 backlog 直至各自 NFR14 / create-story）。**不**实质修订 AD-25/27 正文（→ 62/63）；**不**实现 FR117+。

## Boundaries & Constraints

**Always:** Phase 14 Deferred 指针；NFR50 门禁可执行；NFR14 关闭勾选；`epic-57: done`；58–63 仍 backlog（非 ready-for-dev）；Bitloom。

**Ask First:** 若要在本故事实质改写 AD-25/27 Rule 正文 — 属 Epic 62/63，须另开。

**Never:** 将 Epic 58–63 标 ready；实现 FR117+；改写 FR94–115 为失败；静默扩大 NFR47 子集；恢复 Scala Parser 为产品依赖。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 收口齐全 | Spine Phase 14 + NFR50；NFR14 全勾；epic-57 done | ATDD 绿 | N/A |
| 缺 Phase 14 / NFR50 指针 | 仅 Phase 13 Deferred | ATDD 红 | 补脊柱 |
| 提前 ready 58–63 | sprint 标 ready | 审查拒绝；闸门语义违反 | 保持 backlog |
| 实质改 AD-25 Rule | 本故事改 Handshake 默认可综合 | 越界 | 推回 Epic 62 |

</frozen-after-approval>

## Code Map

- `ARCHITECTURE-SPINE.md` — Deferred：Phase 14 合同；AD-25/27/formal 加深指针（NFR50）
- `nfr14-risk-epic57-phase14-nfr47-deferred-deepen.md` — 状态 closed；关闭条件全勾
- `epics.md` — `phase14Epic57Status: complete`
- `sprint-status.yaml` — `57-4` done；`epic-57: done`；58–63 backlog
- `crates/bitloom/tests/fr116_ad_pointer_epic57_close.rs` — ATDD（样板 `fr106_ad_pointer_epic48_close`）

## Story

As a 架构 / 维护者,
I want 脊柱/AD 指针说明 Phase 14 加深须引用修订 AD，并勾选 Epic 57 关闭,
So that 58–63 不被「仅 Phase 13」叙述挡死，且闸门可检查关闭。

## Acceptance Criteria

1. Given Story 57.3, when 视需要修订 ARCHITECTURE-SPINE 指针（或文档化「实现 epic 内修订 AD-25/27 / formal 路径」门禁）并更新 NFR14, then NFR14 勾选 Epic 57 关闭；**Epic 58–63 此前不得标 ready**
2. And 明确：各实现 epic 仍须在首故事引用适用已修订 AD（NFR50）

## Tasks / Subtasks

- [x] T1: ARCHITECTURE-SPINE Deferred — Phase 14 合同指针 + AD-25/27/formal 实现 epic 修订门禁（NFR50）（AC: 1–2）
- [x] T2: NFR14 状态 closed；关闭条件全勾选（AC: 1）
- [x] T3: epics.md `phase14Epic57Status: complete`；sprint `57-4` done + `epic-57: done`；58–63 仍 backlog（AC: 1）
- [x] T4: ATDD `fr116_ad_pointer_epic57_close.rs`（AC: 1–2）
- [x] T5: code-review Approve；automation summary

## Dev Notes

- **样板：** Story 48.4（`fr106_ad_pointer_epic48_close` + Spine Deferred + NFR14 勾选 + epic done）。
- **Closeout ATDD 勿过冻：** Epic 57 关闭后 58–63 **可**离开永久冻结，但仍 backlog 直至各自 NFR14 — 断言「仍 seeded / 存在 epic-58…」即可，**不要**把 58–63 永久钉死为不得离开 backlog（见 Epic 48 retro finding #8 / `e85bc69`）。
- **本故事只写指针：** 不改 AD-25/27 Rule 正文；Handshake / 官方风格 / SBY 实质修订分别属 Epic 62 / 63 / 60。
- **品牌：** Bitloom / `bitloom-*`。
- **一 story 一 commit；NO push。**

### Project Structure Notes

- 测试落在 `crates/bitloom/tests/`，与 `fr106_ad_pointer_epic48_close.rs` 同形
- 不改 crates 产品代码

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Story 57.4]
- [Source: `_agile-output/implementation-artifacts/48-4-ad-指针与-epic-48-收口-fr106.md`]
- [Source: `_agile-output/implementation-artifacts/nfr14-risk-epic57-phase14-nfr47-deferred-deepen.md`]
- [Source: `_agile-output/implementation-artifacts/process-one-story-one-commit.md`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- Spine Deferred：Phase 13→已关闭；Phase 14 现行加深面 + NFR50 AD-25/27/formal 指针
- NFR14 closed；关闭条件全勾；`phase14Epic57Status: complete`
- sprint `epic-57: done`；`57-4: done`；Epic 58–63 仍 backlog
- ATDD `fr116_ad_pointer_epic57_close` 3 用例绿；code-review Approve

### File List

- `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic57-phase14-nfr47-deferred-deepen.md`
- `_agile-output/planning-artifacts/epics.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `crates/bitloom/tests/fr116_ad_pointer_epic57_close.rs`
- `_agile-output/implementation-artifacts/57-4-ad-指针与-epic-57-收口-fr116-nfr50.md`
- `_agile-output/implementation-artifacts/57-4-code-review.md`
- `_agile-output/implementation-artifacts/57-4-automation-summary.md`
- `_agile-output/implementation-artifacts/atdd-checklist-57-4-ad-指针与-epic-57-收口-fr116-nfr50.md`
- `crates/bitloom/tests/fr116_readme_deferred_honesty.rs` — relax 57.3 over-freeze (57.4 may close; 58–63 seeded)

## Change Log

- 2026-09-10: Story 57.4 AD pointer + close Epic 57 Phase 14 gate (FR116/NFR50)
