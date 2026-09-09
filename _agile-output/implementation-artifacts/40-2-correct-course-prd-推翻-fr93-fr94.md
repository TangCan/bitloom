---
title: '40.2 Correct Course + PRD 推翻 FR93（FR94）'
type: 'chore'
created: '2026-09-09'
status: 'done'
baseline_commit: '7b5ff7a'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/sprint-change-proposal-2026-09-09-phase12-path-b.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/40-1-epic-40-nfr14-风险记录.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic40-literal-path-b.md'
  - '{project-root}/_agile-output/implementation-artifacts/deferred-work.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR94 要求 Correct Course + PRD/addendum 明确批准 Path B 并推翻/收缩 FR93；`d6b2986` 已落地正文，但缺少 **Story 40.2 验收闸门**（ATDD + 缺口补齐），易把「提案已合入」误判为 FR94 PRD 门关闭，或漏掉 NFR42 / Bitloom 品牌 / FR87 历史口径断言。

**Approach:** 以 Story 40.1 完成与已批准 Correct Course 为前提，**验证并补齐** PRD/addendum Phase 12 合同戳：Phase 12 = 字面七阶段全绿；FR93 五条推翻并映射 FR95–105；FR87 为历史里程碑、字面宣称仅 FR94–105（NFR42）；品牌 Bitloom/`bitloom-*`。用 ATDD 锁住提案 `status: approved`、addendum Phase 12 节、prd amendment、NFR42 与品牌。本故事**不**重写 doc-19（→ 40.3）、**不**修订 ARCHITECTURE-SPINE / README 永久非目标锁定正文（→ 40.4）、**不**开闸 Epic 41–47。

## Boundaries & Constraints

**Always:** Correct Course approved；addendum「Phase 12 字面绿」；推翻 FR93 五条 → FR95–105；NFR42；FR87 历史非唯一「产品做完」；Bitloom / `bitloom-*`；40.1 NFR14 门已满足；品牌与 AD-6（设计 crate → `bitloom-prelude`）。

**Ask First:** 若产品撤回 Path B / 恢复 FR93 永久锁定 — 须新 Correct Course，不得静默改 ATDD。

**Never:** 重写 `docs/requirements/19` §19.7–19.9（→ 40.3）；修订 ARCHITECTURE-SPINE AD-5/25/27 或撤销 README「须新 PRD」锁（→ 40.4）；勾选 Epic 40 关闭条件；将 Epic 41–47 标 ready；把本故事冒充字面 HLS/LSP/Chisel 实现。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 合同戳齐全 | Correct Course `approved` + addendum Phase 12 + prd amendment + NFR42 + Bitloom | ATDD 绿；故事可 done | N/A |
| 缺 Phase 12 节 / 未推翻 FR93 | addendum 无 Path B / 无「推翻 FR93」 | ATDD 红 | 补齐 addendum 后再绿 |
| 缺 NFR42 / 仍用 FR87 作唯一完成口径 | 无 NFR42 或未写明 FR87 历史 | ATDD 红 | 补宣称纪律句 |
| 缺 Bitloom 品牌 | Phase 12 节无 Bitloom/`bitloom` | ATDD 红 | 补品牌句（不改发布名） |
| 越界改 doc-19 / AD | 本故事触及 40.3/40.4 范围 | 拒绝；留给后续故事 | 审查拦截 |

</frozen-after-approval>

## Code Map

- `_agile-output/planning-artifacts/sprint-change-proposal-2026-09-09-phase12-path-b.md` — Correct Course **approved**（合同源）
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md` — Phase 11 + **Phase 12 字面绿**；FR93 推翻
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md` — `amendment: …phase12-literal-green-path-b-2026-09-09`；Bitloom 身份
- `_agile-output/planning-artifacts/epics.md` — Story 40.2 AC；Epic 40 Gate；FR94
- `_agile-output/implementation-artifacts/nfr14-risk-epic40-literal-path-b.md` — 40.1 门禁（前置）
- `_agile-output/implementation-artifacts/deferred-work.md` — Phase 12 已推翻指针（README 锁 → 40.4）
- `crates/bitloom/tests/fr93_permanent_non_goals.rs` — 历史 FR93 锁 ATDD（40.4 前仍绿；本故事另加 FR94 门）
- `crates/bitloom/tests/prd_phase9_closures.rs` / `fr84_softf16_explicit_defer.rs` — addendum 字符串门样板

## Story

As a 产品负责人,
I want PRD/addendum 经 Correct Course 明确批准 Path B 并推翻/收缩 FR93,
So that 字面绿具备合同授权。

## Acceptance Criteria

1. Given Story 40.1；现 addendum「Phase 11 合同绿」禁止字面宣称，when 批准并落地 sprint-change-proposal（或等价）+ PRD/addendum Phase 12 段落，then 写明：Phase 12 = 字面七阶段全绿；FR93 五条被推翻或逐条收缩为可交付 FR95–105
2. And 写明：合同绿（FR87）仍可作为历史已交付标签，但不再作为「产品做完」唯一口径；字面宣称仅引用 FR94–105（NFR42）
3. And 公开品牌仍为 Bitloom / `bitloom-*`

## Tasks / Subtasks

- [x] T1: 核对 Correct Course `approved` + addendum/prd Phase 12 合同戳；补齐缺口（五条映射 / NFR42 / Bitloom）（AC: 1–3）
- [x] T2: ATDD `fr94_prd_path_b_gate.rs`（AC: 1–3）
- [x] T3: sprint → `40-2` review（审查后 done）；**不**开工 40.3+ / 41–47
- [x] T4: code-review 记录 Approve

## Dev Notes

- **已落地（d6b2986）：** Correct Course 全文；addendum Phase 12；prd amendment；epics Phase 12；deferred 历史锁定+推翻指针；sprint Epic 40–47 播种。本故事是 **验收/补洞/ATDD**，不是重写提案。
- **本故事补齐：** addendum Phase 12 增加 Bitloom 品牌句、FR87「不再唯一完成口径」、FR93 五条→FR95–101 逐条映射。
- **仍待 40.3：** `docs/requirements/19` §19.7–19.9 字面绿勾选。
- **仍待 40.4：** ARCHITECTURE-SPINE AD-5/25/27；README「永久非目标（FR93）」须新 PRD 锁撤销；Epic 40 关闭勾选。
- **与 fr93 ATDD 共存：** `fr93_permanent_non_goals` 在 40.4 前仍断言公开锁含「须新 PRD」（README 仍写）；本故事 FR94 ATDD 断言 **addendum/提案已推翻**，二者不互相删除。
- 设计 crate 只依赖 `bitloom-prelude`；本故事仅文档+测试。

### Project Structure Notes

- ATDD 落在 `crates/bitloom/tests/`，与 `fr93_*` / `prd_phase9_*` 同模式
- 故事/审查/ATDD checklist 落在 `_agile-output/implementation-artifacts/`

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 40 / Story 40.2]
- [Source: `_agile-output/planning-artifacts/sprint-change-proposal-2026-09-09-phase12-path-b.md`]
- [Source: `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md` — Phase 12]
- [Source: `_agile-output/implementation-artifacts/40-1-epic-40-nfr14-风险记录.md`]
- [Source: `_agile-output/implementation-artifacts/process-one-story-one-commit.md`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- 验证 Correct Course `approved` + prd amendment `phase12-literal-green-path-b`
- 补齐 addendum Phase 12：Bitloom 品牌；FR87 非唯一完成口径；FR93 五条→FR95–101 映射
- ATDD `fr94_prd_path_b_gate`（6 tests）绿
- NFR14 Epic 40 记录勾选 40.2 FR94 合同戳项；未勾选 Epic 40 关闭 / 未开 40.3+

### File List

- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md`
- `_agile-output/implementation-artifacts/40-2-correct-course-prd-推翻-fr93-fr94.md`
- `_agile-output/implementation-artifacts/atdd-checklist-40-2-correct-course-prd-推翻-fr93-fr94.md`
- `_agile-output/implementation-artifacts/40-2-code-review.md`
- `_agile-output/implementation-artifacts/40-2-automation-summary.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic40-literal-path-b.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `crates/bitloom/tests/fr94_prd_path_b_gate.rs`

## Change Log

- 2026-09-09: Story context created (ready-for-dev) — FR94 PRD/Correct Course gate
- 2026-09-09: Verified/filled Phase 12 addendum + FR94 ATDD；ready for review

## Suggested Review Order

**Correct Course approved** → **addendum Phase 12（FR93 推翻 / NFR42 / Bitloom）** → **ATDD** → **sprint（40.3+ 仍 backlog）**
