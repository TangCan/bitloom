---
title: '57.3 同步 README / deferred / 路线图指针（FR116 / FR123）'
type: 'chore'
created: '2026-09-10'
status: 'done'
baseline_commit: 'd8d1ded'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/planning-artifacts/sprint-change-proposal-2026-09-10-phase14-nfr47-deferred-deepen.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md'
  - '{project-root}/_agile-output/implementation-artifacts/57-2-correct-course-prd-批准-phase-14-fr116.md'
  - '{project-root}/_agile-output/implementation-artifacts/48-3-同步-readme-deferred-路线图-fr106-fr115.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic57-phase14-nfr47-deferred-deepen.md'
  - '{project-root}/README.md'
  - '{project-root}/_agile-output/implementation-artifacts/deferred-work.md'
  - '{project-root}/docs/requirements/19. 实施路线图.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR116 合同已批准（Story 57.2），但公开 README / `deferred-work.md` / doc-19 仍主要叙述 Phase 13，易把「商业加深已关」与「NFR47 未选加深」混淆，或漏掉 FR123 宣称纪律。

**Approach:** 按 Story 48.3 模式同步诚实面：README 区分 Phase 13 关闭 vs Phase 14 加深；deferred 将 item-149/153/157/161/165 等标注升格 FR117–122；可选 doc-19 交叉链；ATDD 锁住。保留 Phase 12/13 关闭证据指针。**不**修订 ARCHITECTURE-SPINE AD（→ 57.4）；**不**将 Epic 58–63 标 ready；**不**实现 FR117+。

## Boundaries & Constraints

**Always:** Phase 13 FR106–115 = 商业加深已关闭；Phase 14 = NFR47 未选子集升格合同；列出 FR117–122 主题；保留 Phase 12/13 关闭证据；FR123 宣称纪律；Bitloom。

**Ask First:** 若撤回 Phase 14 合同 — 须新 Correct Course。

**Never:** 删除 Phase 12/13 关闭证据；ARCHITECTURE-SPINE Deferred/AD 收口（→ 57.4）；勾选 Epic 57 关闭；将 Epic 58–63 标 ready；实现 FR117+ 代码；宣称 Tywaves/syn-scan/SBY/VIP GPIO/Handshake/官方风格已完成。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 诚实面齐全 | README + deferred + doc-19 含 Phase 14 / FR117–122 / FR123 | ATDD 绿 | N/A |
| 缺 Phase 14 或漏 FR123 | 仅写 Phase 13 | ATDD 红 | 补齐后再绿 |
| 删 Phase 12/13 关闭指针 | 去掉「已关闭」/NFR44/48 | ATDD 红 + 审查拒绝 | 恢复指针 |
| 越界改 AD / ready 58–63 | 触及 57.4 / 实现 epic | 拒绝 | 审查拦截 |

</frozen-after-approval>

## Code Map

- `README.md` — 「状态与 deferred」；新增 Phase 14 节 + FR117–122 表
- `_agile-output/implementation-artifacts/deferred-work.md` — Phase 14 pointer；升格 NFR47 items
- `docs/requirements/19. 实施路线图.md` — 可选交叉链（不重定义 §19.7–19.9）
- `crates/bitloom/tests/fr106_readme_deferred_honesty.rs` — 样板（保留）
- `crates/bitloom/tests/fr116_readme_deferred_honesty.rs` — 本故事 ATDD

## Story

As a 文档维护者,
I want 公开状态页区分 Phase 13 完成面与 Phase 14 加深面,
So that 对外宣称不混淆。

## Acceptance Criteria

1. Given Story 57.2, when 更新 README「状态与 deferred」、`deferred-work.md`、（若需）`docs/requirements/19` 交叉链, then 明确：Phase 13 FR106–115 = 商业加深已关闭；Phase 14 = NFR47 未选子集升格合同；列出 FR117–122 主题
2. And 不得删除 Phase 12/13 关闭证据指针
3. And 写明「Tywaves / syn-scan / SBY / VIP GPIO / Handshake / 官方风格全家桶」宣称仅在对应 FR 关闭后可勾选（FR123）

## Tasks / Subtasks

- [x] T1: README Phase 14 节 + FR117–122 加深表 + FR123 宣称纪律；保留 Phase 12/13 关闭证据（AC: 1–3）
- [x] T2: deferred Phase 14 pointer；item-149/153/157/161/165 等升格标注；去掉「尚无合同」措辞（AC: 1–2）
- [x] T3: doc-19 交叉链 Phase 14（不重定义字面绿 MVP）（AC: 1–2）
- [x] T4: ATDD `fr116_readme_deferred_honesty.rs`（AC: 1–3）
- [x] T5: sprint → `57-3` done；**不**开工 57.4 / 58–63
- [x] T6: code-review 记录 Approve

## Dev Notes

- **样板：** Story 48.3（`fr106_readme_deferred_honesty` + README/deferred/doc-19）。
- **升格映射（Correct Course 4.5）：** Tywaves→FR117；syn-scan→FR118；SBY→FR119；VIP GPIO→FR120；Handshake→FR121；官方风格→FR122。
- **仍须另开合同（NFR51）：** 自动 FSM 标签、更多 IP FL、第三方 LCOV GUI、emit MemRead 完整生成等未列入本批项。
- **仍待 57.4：** AD 指针 + Epic 57 关闭勾选。
- **品牌：** Bitloom / `bitloom-*`。

### Project Structure Notes

- 测试落在 `crates/bitloom/tests/`，与 `fr106_readme_deferred_honesty.rs` 同形
- 不改 ARCHITECTURE-SPINE；不改 crates 产品代码

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Story 57.3]
- [Source: `_agile-output/planning-artifacts/sprint-change-proposal-2026-09-10-phase14-nfr47-deferred-deepen.md` §4.5]
- [Source: `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md` — Phase 14]
- [Source: `_agile-output/implementation-artifacts/48-3-同步-readme-deferred-路线图-fr106-fr115.md`]
- [Source: `_agile-output/implementation-artifacts/process-one-story-one-commit.md`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- README / deferred / doc-19 诚实面：Phase 13 关闭 vs Phase 14 加深；FR117–122 表；FR123 宣称纪律
- ATDD `fr116_readme_deferred_honesty` 7 用例绿
- code-review Approve；sprint `57-3: done`；`57-4` 仍 backlog；Epic 58–63 仍 backlog
- NFR14 勾选 README/deferred 项；未触及 AD 收口（→ 57.4）

### File List

- `README.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `docs/requirements/19. 实施路线图.md`
- `crates/bitloom/tests/fr116_readme_deferred_honesty.rs`
- `_agile-output/implementation-artifacts/57-3-同步-readme-deferred-路线图指针-fr116-fr123.md`
- `_agile-output/implementation-artifacts/57-3-code-review.md`
- `_agile-output/implementation-artifacts/57-3-automation-summary.md`
- `_agile-output/implementation-artifacts/atdd-checklist-57-3-同步-readme-deferred-路线图指针-fr116-fr123.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic57-phase14-nfr47-deferred-deepen.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-10: Story 57.3 honesty surface for Phase 14 (FR116/FR123)
