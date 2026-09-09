---
title: '32.3 Bundle derive（或等价）'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: 'b758832'
review_loop_iteration: 1
followup_review_recommended: false
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic32-nested-bundle.md'
  - '{project-root}/_agile-output/implementation-artifacts/32-2-嵌套-bundle-可综合路径.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-32-context.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
  - '{project-root}/_agile-output/specs/spec-rhdl/language-surface.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Designers must hand-write `Bundle::leaves` / `nested_bundles` boilerplate; `#[derive(Bundle)]` was documented defer (Story 32.2 / FR51 MVP).

**Approach:** Provide `#[derive(Bundle)]` in `bitloom-macro`, re-exported from `bitloom-prelude` (AD-6). Document supported/limited set. Positive derived structs enter synthesizable flatten → emit; unsupported nest/field shapes get stable compile-time diagnostics. Do not expand public HIR Bundle nodes; do not unlock `HwVec<Bundle,_>` or ≥2-level recursion.

## Boundaries & Constraints

**Always:** derive or documented equivalent via prelude-only path; document support/limits; positive examples elaborate/emit; unsupported shapes stable diagnostics; design crates depend only on `bitloom-prelude`; Bitloom brand; AD-20 / FR80 / NFR14.

**Ask First:** If derive must live outside prelude-reexport path (would violate AD-6).

**Never:** Require design crates to depend on CLI/`bitloom`; only delete OUT OF SCOPE comments without derive; claim arbitrary nest depth; unlock `HwVec<Bundle,_>`; weaken width/dir emit-before-fail.

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Happy ground Bundle | `#[derive(Bundle)]` struct with Bool/UInt/… fields | `Bundle::leaves` generated; flatten/emit works | N/A |
| Happy one-level nest | Nested Bundle field (non-ground path) | `nested_bundles` → child `leaves`; emit/tick | N/A |
| Unsupported shape | enum / tuple struct / HwVec field / Input-Output field | Stable `rhdl::E0180` / trybuild | No silent accept |
| AD-6 | Design crate deps | Only `bitloom-prelude` | No CLI dep |
| Still OOS | `HwVec<Bundle,_>` / ≥2-level contract | Unchanged OOS / deferred docs | Honest docs |

</frozen-after-approval>

## Code Map

- `crates/bitloom-macro/src/lib.rs` — `#[derive(Bundle)]`
- `crates/bitloom-prelude/src/lib.rs` — re-export derive; Bundle docs
- `examples/bundle_vec_skel/` — `DerivedStream` / `DerivedPacket`; emit/tick; UI negatives
- `_agile-output/specs/spec-rhdl/language-surface.md` — FR80 derive support/limits
- `crates/bitloom/tests/fr80_nested_bundle.rs` — derive + AD-6 guardrails

## Story

As a 设计者,
I want `#[derive(Bundle)]`（或文档等价）生成 Bundle 实现,
So that 少写样板 flatten.

## Acceptance Criteria

1. Given Story 32.2, when 提供 derive 或文档等价 API 并文档化支持/限制集合（FR80）, then 正例结构体可进入可综合路径并 emit
2. Given 不支持的嵌套/字段形态, when derive, then 稳定诊断（compile-fail / trybuild）
3. Given 设计 crate, when 使用 derive, then 仍只依赖 `bitloom-prelude`（AD-6）

## Tasks / Subtasks

- [x] T1: `bitloom-macro` `#[derive(Bundle)]` + prelude re-export (AD-6)
- [x] T2: Document supported/limited set (language-surface + prelude)
- [x] T3: Positive derived fixtures → elaborate/emit/tick; unsupported-shape UI
- [x] T4: ATDD/guardrails green; code-review Approve; sprint → done

## Spec Change Log

- 2026-09-09: FR80 `#[derive(Bundle)]` via prelude; support/limits; E0180 UI negatives; derived nested emit/tick.

## Design Notes

Derive expands to hand-written-equivalent `leaves` / `nested_bundles`. Known grounds → leaves; other simple paths → one-level nested. Reject enums/tuples/`HwVec`/`Input`/`Output`/non-paths with `rhdl::E0180`.

## Verification

**Commands:**
- `cargo test -p bundle_vec_skel` — **PASS** (19 tests)
- `cargo test -p bitloom --test fr80_nested_bundle` — **PASS**
- `cargo test -p bitloom-prelude --lib` — **PASS**

## Review Triage Log

### 2026-09-09 — Formal review（Approve）
- intent_gap: 0
- bad_spec: 0
- patch: 0
- defer: 用户文档限制表收口 → 32.4；≥2 层仍非目标

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- `#[derive(Bundle)]` in `bitloom-macro`; `pub use` from `bitloom-prelude` (AD-6)
- Fixtures `DerivedStream` / `DerivedPacket` / `DerivedNestedBundleSkel` emit+tick
- trybuild: enum / tuple / HwVec field → `rhdl::E0180`
- language-surface + prelude docs; FR80 ATDD extended
- sprint: `epic-32: in-progress`, `32-3: done`
- Leave `_bmad/scripts/render_skill.py.bak` untracked; no full `just test` / no commit

### File List

- `crates/bitloom-macro/src/lib.rs`
- `crates/bitloom-prelude/src/lib.rs`
- `examples/bundle_vec_skel/src/lib.rs`
- `examples/bundle_vec_skel/tests/ui/derive_bundle_enum.rs`
- `examples/bundle_vec_skel/tests/ui/derive_bundle_enum.stderr`
- `examples/bundle_vec_skel/tests/ui/derive_bundle_tuple.rs`
- `examples/bundle_vec_skel/tests/ui/derive_bundle_tuple.stderr`
- `examples/bundle_vec_skel/tests/ui/derive_bundle_hwvec_field.rs`
- `examples/bundle_vec_skel/tests/ui/derive_bundle_hwvec_field.stderr`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `crates/bitloom/tests/fr80_nested_bundle.rs`
- `_agile-output/implementation-artifacts/32-3-bundle-derive-或等价.md`
- `_agile-output/implementation-artifacts/32-3-atdd-checklist.md`
- `_agile-output/implementation-artifacts/32-3-code-review.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic32-nested-bundle.md`
- `_agile-output/implementation-artifacts/epic-32-context.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
