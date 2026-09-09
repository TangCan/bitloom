---
title: '32.2 嵌套 Bundle 可综合路径'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: '92817906b0cbb1653b84a475f42194d8d5ac2202'
review_loop_iteration: 1
followup_review_recommended: false
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic32-nested-bundle.md'
  - '{project-root}/_agile-output/implementation-artifacts/32-1-epic-32-nfr14-风险记录.md'
  - '{project-root}/_agile-output/implementation-artifacts/19-4-实现-bundle-vec-可综合路径-fr51.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-32-context.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
  - '{project-root}/_agile-output/specs/spec-rhdl/language-surface.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR51 仅支持 ground-leaf `Bundle` flatten；prelude / language-surface 将嵌套 Bundle 标为 OUT OF SCOPE，设计者须手工摊平层次（NFR37）。

**Approach:** 在既有 flatten→标量 HIR 路径上支持**至少一层**文档化嵌套 Bundle（子 Bundle 作父成员）→ elaborate → emit `.v` → tick；嵌套叶宽/向不匹配仍 emit 前失败。不扩展公开 HIR Bundle 节点；不实现 `#[derive(Bundle)]`（32.3）；不放开 `HwVec<Bundle,_>`。

## Boundaries & Constraints

**Always:** 至少一层嵌套可综合；夹具 elaborate/emit/tick；嵌套叶宽/向 emit 前失败；nested 不再仅以 OUT OF SCOPE 交差；设计 crate 仅 `bitloom-prelude`；品牌 Bitloom；AD-20 / FR80 / NFR14 记录假设。

**Ask First:** 若须公开 HIR Bundle 节点（而非更深命名叶子）才能验收。

**Never:** 仅删 OUT OF SCOPE 注释而无实现；静默声称任意深度；实现 derive / `HwVec<Bundle,_>`；弱化宽/向门禁；设计 crate 依赖 CLI/`bitloom`。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Happy 一层嵌套 | 父 Bundle 含子 Bundle 成员 | flatten `{field}_{nested}_{leaf}` → emit `.v` → tick | N/A |
| Width mismatch | 嵌套叶连接同位宽失败 | emit 前 E0131 | 不得 emit |
| Dir mismatch | 向嵌套输入叶赋值 | emit 前 E0112 | 不得 emit |
| FR51 回归 | 纯 ground Bundle / HwVec | 行为不变 | N/A |
| 仍 OUT OF SCOPE | `HwVec<Bundle,_>` / derive | 编译期失败（既有 trybuild） | 文档诚实 |

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/lib.rs` — `Bundle::nested_bundles`；`bundle_leaves` 一层嵌套命名
- `examples/bundle_vec_skel/src/lib.rs` — `Packet` 嵌套 `Stream`；`NestedBundleSkel` / 宏端口；宽/向负例
- `_agile-output/specs/spec-rhdl/language-surface.md` — FR80 一层合同；HwVec/derive 仍 OOS
- `crates/bitloom/tests/fr80_nested_bundle.rs` — 文档/API 门禁 ATDD
- `crates/bitloom-builder` — 复用 E0131/E0112（未改）

## Story

As a 设计者,
I want 在端口/内部使用嵌套 `Bundle`,
So that 复合层次不必手工摊平。

## Acceptance Criteria

1. Given Story 32.1，when 支持至少一层文档化嵌套 Bundle → HIR → emit `.v`（FR80），then 夹具可 elaborate、emit、tick（或文档化等价验收）
2. Given 嵌套字段宽/向不匹配，when elaborate/连接，then emit 前失败（FR8/FR51 精神）
3. Given prelude / language-surface，when 文档嵌套，then nested 不再仅以 OUT OF SCOPE 交差（NFR37）

## Tasks / Subtasks

- [x] T1: prelude `Bundle` 一层嵌套 API + flatten 命名 `{field}_{member}_{leaf}`
- [x] T2: 夹具 nested Bundle → elaborate → emit → tick；宽/向负例
- [x] T3: language-surface / prelude 文档：一层 IN SCOPE；≥2 / HwVec&lt;Bundle&gt; / derive 仍 defer
- [x] T4: ATDD 绿；sprint → done；code-review Approve

## Spec Change Log

- 2026-09-09: FR80 一层 `nested_bundles`；夹具 `Packet`/`NestedBundleSkel`；language-surface 更新；FR80 ATDD。

## Design Notes

嵌套 = 更深命名叶子（`{field}_{nested}_{leaf}`），非公开 HIR Bundle 节点。父 `nested_bundles` 指向子 `leaves`；Epic 32 默认不递归更深。`HwVec<Bundle,_>` / derive 仍 OOS。

## Verification

**Commands:**
- `cargo test -p bundle_vec_skel` — **PASS**（15 tests，含嵌套 emit/tick 与宽/向负例）
- `cargo test -p bitloom --test fr80_nested_bundle` — **PASS**
- `cargo test -p bitloom-prelude -p bitloom-macro -p bitloom-builder --lib` — **PASS**

## Review Triage Log

### 2026-09-09 — Formal review（Approve）
- intent_gap: 0
- bad_spec: 0
- patch: 0
- defer: ≥2 层递归 / 用户文档限制表 → 32.4；derive → 32.3

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- `Bundle::nested_bundles` + flatten `{field}_{nested}_{leaf}`
- 夹具 `Packet` / `NestedBundleSkel` / `NestedBundlePorts`：elaborate → emit → tick；嵌套 E0131/E0112
- language-surface + prelude 文档：一层 FR80；HwVec&lt;Bundle&gt;/derive 仍 OOS
- ATDD `fr80_nested_bundle`；审查 Approve；testarch-automate：夹具+门禁已覆盖，无需额外层
- sprint：`epic-32: in-progress`，`32-2: done`
- 附带：`render_skill.py` 接受 bmm/gds 相同 `implementation_artifacts`（与 custom config 注释一致）；`.bak` 保持未跟踪

### File List

- `crates/bitloom-prelude/src/lib.rs`
- `examples/bundle_vec_skel/src/lib.rs`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `crates/bitloom/tests/fr80_nested_bundle.rs`
- `_agile-output/implementation-artifacts/32-2-嵌套-bundle-可综合路径.md`
- `_agile-output/implementation-artifacts/32-2-code-review.md`
- `_agile-output/implementation-artifacts/epic-32-context.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic32-nested-bundle.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `_bmad/scripts/render_skill.py`

## Suggested Review Order

- `Bundle::nested_bundles` / `bundle_leaves` — `crates/bitloom-prelude/src/lib.rs`
- `Packet` / `NestedBundleSkel` — `examples/bundle_vec_skel/src/lib.rs`
- language-surface Composite / FR80 — `_agile-output/specs/spec-rhdl/language-surface.md`
- FR80 ATDD — `crates/bitloom/tests/fr80_nested_bundle.rs`
