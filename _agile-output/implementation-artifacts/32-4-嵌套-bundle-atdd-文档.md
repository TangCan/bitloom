---
title: '32.4 嵌套 Bundle ATDD + 文档'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: '28d650f'
review_loop_iteration: 1
followup_review_recommended: false
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic32-nested-bundle.md'
  - '{project-root}/_agile-output/implementation-artifacts/32-2-嵌套-bundle-可综合路径.md'
  - '{project-root}/_agile-output/implementation-artifacts/32-3-bundle-derive-或等价.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-32-context.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/specs/spec-rhdl/language-surface.md'
  - '{project-root}/examples/bundle_vec_skel/src/lib.rs'
  - '{project-root}/crates/bitloom/tests/fr80_nested_bundle.rs'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Stories 32.2–32.3 delivered one-level nested Bundle + `#[derive(Bundle)]`, but Epic 32 lacks UJ-level ATDD/docs close-out: user-facing nested/derive examples + limits table, docs-matrix ATDD locking the recipe, and NFR14 Epic 32 close checkboxes still unticked (NFR37 honesty).

**Approach:** Document + light docs/matrix ATDD close-out (mirror Story 31.4). Add follow-along with minimal nested/derive examples and a limits table; extend `fr80_nested_bundle` to assert docs + NFR14 ticks + skel goldens exist; point README/language-surface at the follow-along; tick NFR14 Epic 32 close conditions; sprint `32-4` + `epic-32` → done. Do **not** add new prelude/HIR APIs or unlock `HwVec<Bundle,_>` / ≥2-level recursion.

## Boundaries & Constraints

**Always:** Depend on 32.2–32.3; ATDD covers nested positive emit/tick + width/dir negatives fail before emit; document `just test` or directed recipe; user docs nested/derive minimal examples + limits table; tick NFR14 Epic 32 close; Bitloom brand; AD-6 / AD-20 / FR80 / NFR37.

**Ask First:** None.

**Never:** `cargo clean` / full `just test` as forced gate; git commit/push; claim arbitrary nest depth; unlock `HwVec<Bundle,_>`; only delete OUT OF SCOPE comments; weaken width/dir emit-before-fail; invent new public HIR Bundle nodes.

## I/O & Edge-Case Matrix

| Scenario | Expected |
|----------|----------|
| Nested positive (hand + derive) | `bundle_vec_skel` emit/tick goldens still green |
| Width/dir negatives | Nested leaf mismatch fails before emit (skel tests) |
| Derive unsupported | trybuild `rhdl::E0180` still green |
| User docs | Tutorial: nested/derive examples + limits table + recipe |
| NFR14 | Epic 32 close checklist all `[x]` |
| Recipe | Tutorial names directed `cargo test`; points at `just test` |
| Still OOS | `HwVec<Bundle,_>` / ≥2-level documented non-goal |

</frozen-after-approval>

## Code Map

- `docs/tutorials/nested-bundle.md` — **NEW** FR80 follow-along (examples + limits + recipe)
- `docs/fr80-nested-bundle.md` — **NEW** product note
- `README.md` — index FR80 / `bundle_vec_skel` / tutorial
- `_agile-output/specs/spec-rhdl/language-surface.md` — Story 32.4 close-out mark
- `examples/bundle_vec_skel/src/lib.rs` — crate docs cross-link
- `crates/bitloom/tests/fr80_nested_bundle.rs` — docs + NFR14 + goldens close-out ATDD
- `_agile-output/implementation-artifacts/nfr14-risk-epic32-nested-bundle.md` — close checklist ticks

## Story

As a 质量负责人,
I want 嵌套 Bundle 正/负例自动化与文档示例,
So that FR80/NFR37 可回归.

## Acceptance Criteria

1. Given Story 32.2–32.3, when 增加 ATDD：嵌套正例 emit/tick；宽/向负例 emit 前失败, then `just test`（或文档化配方）稳定通过（FR80）
2. And 用户文档含嵌套/derive 最小示例与限制表
3. And NFR14 记录勾选 Epic 32 关闭条件

## Tasks / Subtasks

- [x] T1: Follow-along + README / language-surface / skel narrative (AC: 1–2)
- [x] T2: ATDD close-out locking docs + NFR14 + skel goldens/negatives (AC: 1)
- [x] T3: Tick NFR14 Epic 32 close conditions; refresh status (AC: 3)
- [x] T4: code-review Approve; sprint `32-4` + `epic-32` → done

## Spec Change Log

- 2026-09-09: FR80 UJ close-out — nested-bundle tutorial + limits table; fr80 product doc; docs ATDD; NFR14 Epic 32 close.

## Design Notes

Docs + matrix ATDD only; reuse `bundle_vec_skel` goldens. No new prelude/HIR surface.

## Verification

**Commands:**
- `cargo test -p bitloom --test fr80_nested_bundle` — **PASS** (7 tests)
- `cargo test -p bundle_vec_skel` — **PASS** (19 tests)

## Review Triage Log

### 2026-09-09 — Formal review（Approve）
- intent_gap: 0
- bad_spec: 0
- patch: 0
- defer: none

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

- ATDD: `cargo test -p bitloom --test fr80_nested_bundle` — 7 passed
- Skel: `cargo test -p bundle_vec_skel` — 19 passed
- testarch-automate: docs/矩阵 ATDD 已覆盖 AC；无需额外 automate 套件
- User constraint: no `cargo clean` / full `just test` / commit

### Completion Notes List

- UJ「嵌套 Bundle」：`docs/tutorials/nested-bundle.md` → 最小示例 + 限制表 + `just test` 配方
- `docs/fr80-nested-bundle.md` + README / language-surface / skel 交叉链接
- NFR14 Epic 32 关闭条件全勾；`epic-32: done`
- code-review **Approve**
- `_bmad/scripts/render_skill.py.bak` 保持 untracked

### File List

- `docs/tutorials/nested-bundle.md`
- `docs/fr80-nested-bundle.md`
- `README.md`
- `examples/bundle_vec_skel/src/lib.rs`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic32-nested-bundle.md`
- `crates/bitloom/tests/fr80_nested_bundle.rs`
- `_agile-output/implementation-artifacts/32-4-嵌套-bundle-atdd-文档.md`
- `_agile-output/implementation-artifacts/32-4-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: Nested Bundle ATDD + docs close-out + Epic 32 NFR14 close（Story 32.4）

## Suggested Review Order

**跟练页 + 限制表** → **ATDD close-out** → **NFR14 勾选** → **sprint epic-32: done**
