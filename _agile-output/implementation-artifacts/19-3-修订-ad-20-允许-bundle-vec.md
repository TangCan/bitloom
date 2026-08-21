---
title: '19.3 修订 AD-20（允许 Bundle/Vec）'
type: 'chore'
created: '2026-08-21'
status: 'done'
baseline_revision: '45f7e5734375336be35bf05677d3a21eece3e1b8'
review_loop_iteration: 0
followup_review_recommended: false
context:
  - '{project-root}/_agile-output/implementation-artifacts/epic-19-context.md'
  - '{project-root}/_agile-output/implementation-artifacts/19-2-修订-ad-5-允许生成-rust-功能模拟器.md'
warnings: []
deferred:
  - summary: >-
      AD-12 阶段一 ground 仍写「无 Bundle/Vector」，未交叉指向 AD-20 可选扩展。
    evidence: |-
      Blind hunter；阶段一历史条文，现行可综合合同以 AD-20/FR51 为准。
    location: >-
      ARCHITECTURE-SPINE.md AD-12
    severity: medium
  - summary: >-
      epics.md Story 19.3 Given 仍用「不得进入」现在时；Epic 17 仍写「禁止未立项 Bundle/Vec」。
    evidence: |-
      Blind hunter；规划归档措辞，不影响脊柱合同。
    location: >-
      _agile-output/planning-artifacts/epics.md
    severity: low
  - summary: >-
      Capability map「阶段二表面加厚 + Bundle/Vec」同列 FR22/FR51，叙事粗。
    evidence: |-
      Blind hunter；Prevents/Rule 已分界，map 行未拆。
    location: >-
      ARCHITECTURE-SPINE.md Capability map
    severity: low
---

<intent-contract>

## Intent

**Problem:** 历史 AD-20「Bundle/Vec 禁止可综合」与 PRD FR51 冲突；脊柱已有 2026-08-21 修订段，但仍须核对 FR51 引用、FR22 边界与项目上下文，并用 ATDD 锁住合同。

**Approach:** 核对并补齐 AD-20（允许文档化 Bundle/Vec 进入可综合路径；位宽/方向 emit 前失败；引用 PRD FR51；明确 FR22 非目标、复合类型由 FR51）；同步 AGENTS / language-surface 缺口；落地文档门禁测试。不实现完整语言（留给 19.4）。

## Boundaries & Constraints

**Always:** Bundle/Vec（或文档等价）允许可综合路径；宽/向错误 emit 前失败；修订可追溯 PRD FR51；FR22 边界清晰；品牌 Bitloom。

**Block If:** 需要在本故事实现 prelude/builder/emit 的 Bundle/Vec 语言本体。

**Never:** 完成 FR51 完整语言实现；把「禁止 Bundle/Vec」重新写成阻断 FR51 的依据；另立公开 HIR 或改运行时行为。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| AD-20 已允许且缺口已补 | 脊柱 AD-20 段 | ATDD 绿；故事只补缺口 | 无 |
| Revised 缺 FR51 / FR22 边界 | 仅写「取消禁止」 | 补全后绿 | ATDD 红直至补齐 |
| AGENTS 仍写旧禁 | 含「禁止 Bundle」可综合 | 更正或指向脊柱 | ATDD 断言无旧禁 |
| 误把 FR22 当复合类型交付 | Rule 把 Bundle 算进 FR22 验收 | 明确复合类型走 FR51 | ATDD 失败 |

</intent-contract>

## Code Map

- `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md` L183–188 — AD-20 Rule/Revised（允许 Bundle/Vec + FR51；emit 前宽/向；FR22 边界）
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md` — FR22「本 FR 非目标」→ FR51；FR51 本体
- `_agile-output/specs/spec-rhdl/language-surface.md` — Composite types + FR22 边界；Comb/seq 指向 FR51
- `AGENTS.md` — Brand lock AD-20 / FR51 正向指针
- `crates/bitloom/tests/ad20_bundle_vec_allowed.rs` — 文档门禁 ATDD
- `crates/bitloom/tests/ad5_rust_functional_sim_allowed.rs` — 样板
- `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/team-walkthrough.html` — 已写 Bundle/Vec 允许（只读核对）
- `_agile-output/implementation-artifacts/sprint-status.yaml` — key `19-3-修订-ad-20-允许-bundle-vec`

## Tasks & Acceptance

**Execution:**
- [x] `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md` -- 核对 AD-20；Rule/Revised 补齐 **PRD FR51**、emit 前宽/向失败、**FR22 边界** -- AC 对齐
- [x] `AGENTS.md` -- Brand lock 增加 AD-20 / FR51 正向指针 -- 防上下文漂移
- [x] `_agile-output/specs/spec-rhdl/language-surface.md` -- 补 FR22↔FR51 边界 -- 表面目录一致
- [x] `crates/bitloom/tests/ad20_bundle_vec_allowed.rs` -- 新增 ATDD 门禁 -- 门禁
- [x] `_agile-output/implementation-artifacts/sprint-status.yaml` -- `19-3-…`：in-progress → review → done -- 冲刺追踪
- [x] `_agile-output/implementation-artifacts/19-3-code-review.md` -- 对抗性审查笔记 -- 流水线要求

**Acceptance Criteria:**
- Given 脊柱含 AD-20，when 阅读 Rule/Revised，then 允许文档化 Bundle/Vec（或等价）进入可综合路径；位宽/方向不匹配须在 emit 前失败
- Given 修订说明，when 对照 PRD，then 引用 **PRD FR51**；明确与 FR22「本 FR 非目标」的边界（复合类型由 FR51 交付）
- Given `AGENTS.md` / language-surface，when 扫描旧禁令与边界，then 已更正或指向脊柱；表面目录不把 Bundle/Vec 静默算进 FR22
- Given 本故事范围，when 完成，then **未**实现完整 Bundle/Vec 语言（留给 19.4）
- Given ATDD 文件存在，when `cargo test -p bitloom --test ad20_bundle_vec_allowed`，then 通过

## Spec Change Log

- 2026-08-21: AD-20 补 FR22 边界 + PRD FR51 Revised；AGENTS / language-surface；ATDD；审查见 [19-3-code-review.md](19-3-code-review.md)

## Review Triage Log

### 2026-08-21 — Review pass
- intent_gap: 0
- bad_spec: 0
- patch: 2: (high 0, medium 0, low 2)
- defer: 3: (high 0, medium 1, low 2)
- reject: 4
- addressed_findings:
  - `[low]` `[patch]` ATDD 收紧 silently / PRD FR51 / 本 FR 非目标 / 不再作为阻断
  - `[low]` `[patch]` language-surface Comb/seq 指向 Composite / FR51

## Design Notes

AD-20 主体已在 2026-08-21 修订；本故事以**核对 + FR22 边界/引用补齐 + ATDD 锁合同**为主。完整语言实现属 Story 19.4。

## Verification

**Commands:**
- `cargo test -p bitloom --test ad20_bundle_vec_allowed` -- expected: PASS — **PASS (2026-08-21)**
- `cargo fmt --all && just test` -- expected: 全绿 — **PASS (2026-08-21)**

## Auto Run Result

- **Summary:** AD-20 补齐 PRD FR51 引用与 FR22「本 FR 非目标」边界；允许 Bundle/Vec 可综合且 emit 前宽/向失败；AGENTS Brand lock + language-surface 同步；ATDD 锁合同。未实现 FR51 语言。
- **Files:** ARCHITECTURE-SPINE AD-20；AGENTS.md；language-surface.md；`ad20_bundle_vec_allowed.rs`；sprint-status；story + code-review。
- **Review:** patches 2 low；deferred 3；reject 4；intent_gap/bad_spec 0。
- **Follow-up review:** false（patched: high 0, medium 0, low 2；score 2 < 5）。
- **Verification:** targeted ATDD PASS；`cargo fmt --all && just test` PASS。
- **Risks:** AD-12 / epics 史态措辞仍可能误导；合同以脊柱 AD-20 为准。

## Dev Agent Record

### Completion Notes

- AD-20 Rule/Revised：FR22 边界 + PRD FR51；历史禁令不再阻断 FR51
- AGENTS Brand lock 正向指针；language-surface Composite + Comb/seq 指针
- ATDD 锁住允许、宽/向、FR51、FR22 边界、silently、AGENTS、language-surface
- **未**实现 Bundle/Vec 语言本体

### File List

- `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md`
- `AGENTS.md`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `crates/bitloom/tests/ad20_bundle_vec_allowed.rs`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `_agile-output/implementation-artifacts/19-3-code-review.md`
- `_agile-output/implementation-artifacts/19-3-修订-ad-20-允许-bundle-vec.md`
