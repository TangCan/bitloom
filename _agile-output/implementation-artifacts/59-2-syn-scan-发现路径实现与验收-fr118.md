---
title: '59.2 Syn-scan 发现路径实现与验收（FR118）'
type: 'feature'
created: '2026-09-10'
status: 'done'
baseline_commit: 'c0ac6f9'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic59-syn-scan-design-root-discovery.md'
  - '{project-root}/_agile-output/implementation-artifacts/59-1-epic-59-nfr14-风险记录.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-59-context.md'
  - '{project-root}/docs/fr113-lsp-design-root-discovery.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/process-one-story-one-commit.md'
warnings: []
deferred:
  - 'FR118 / Epic 59 closeout docs + NFR14 checkboxes (Story 59.3)'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Story 59.1 已钉死 NFR14 syn-scan 策略，但产品仍只有 FR99 DesignFixture 与 FR113 metadata `design_roots`；无 metadata 包无法发现 `#[bitloom::top]`。

**Approach:** （1）在 `bitloom-lsp` `discover` 层：无 metadata 时扫描 `src/**/*.rs` 识别 `#[bitloom::top]`/`#[rhdl::top]`。（2）夹具 `fr118_syn_ok` / `fr118_syn_fail` + 注册表 elaborate。（3）`docs/fr118-*.md` + fr113 交叉链。（4）ATDD：正例 / 失败可读 / FR99+FR113 回归 / shallow ≠ finish。（5）sprint `59-2: done`；保持 `59-3` backlog。**不**勾选 Epic 59 关闭。

## Boundaries & Constraints

**Always:** syn-scan 产品路径；无 metadata 正例；失败可读；FR99/FR113 回归（NFR48）；shallow 不得 finish；品牌 Bitloom；one-story-one-commit。

**Ask First:** 若要把 FR118 收口文档并入本故事（→59.3）。

**Never:** 仅 DesignFixture / 仅 metadata / shallow finish / docs-only 关闭 FR118；实现 59.3 收口或勾选 Epic 59 关闭；冒充 NFR14-crates。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 无 metadata + top | fr118_syn_ok | discover → Fr118OkCounter；full elaborate Pass | N/A |
| syn-scan fail root | fr118_syn_fail | finish + 可读诊断 | E0142 等 |
| metadata 包 | fr113_meta_ok | 仍走 metadata（共存） | NFR48 |
| DesignFixture | OkCounter | 仍绿 | NFR48 |
| shallow | syn-scan root | called_finish=false | 不得伪装 |
| 无 top | fr113_meta_none | no-design-roots | 可读 |

</frozen-after-approval>

## Code Map

- `crates/bitloom-lsp/src/discover.rs` — **UPDATE** syn-scan path
- `crates/bitloom-lsp/src/lib.rs` — **UPDATE** Fr118* registry + messages
- `crates/bitloom-lsp/fixtures/fr118_syn_{ok,fail}` — **NEW**
- `docs/fr118-syn-scan-design-root-discovery.md` — **NEW**
- `docs/fr113-lsp-design-root-discovery.md` — **UPDATE** FR118 pointer
- `crates/bitloom/tests/fr118_syn_scan_design_root_discovery.rs` — **NEW** ATDD
- `_agile-output/implementation-artifacts/sprint-status.yaml` — `59-2` → done

## Story

As a 硬件设计者,
I want 无 metadata 的 workspace 仍能发现 `#[bitloom::top]` 并进入 elaborate,
So that 设计根发现不依赖手写 `design_roots`。

## Acceptance Criteria

1. Given Story 59.1 已钉死策略, when 交付 syn-scan 产品路径 + ATDD/夹具（无 metadata 正例与失败可读）, then `bitloom-lsp` / elaborate 可用该发现结果
2. And FR99 DesignFixture 与 FR113 metadata 回归不破（NFR48）
3. And shallow 不得伪装 finish 成功；不实现 59.3 收口

## Tasks / Subtasks

- [x] T1: discover syn-scan + Fr118 registry（AC: 1）
- [x] T2: fixtures fr118_syn_{ok,fail}（AC: 1）
- [x] T3: docs/fr118 + fr113 交叉（AC: 1–3）
- [x] T4: ATDD fr118_syn_scan_design_root_discovery（AC: 1–3）
- [x] T5: sprint / code-review / automate / fmt+test / 单 commit

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- syn-scan in `discover_design_roots` when metadata absent
- ATDD green: positive / fail / shallow / FR99 / FR113
- sprint: `59-2: done`；`59-3` backlog；`epic-59` in-progress

### File List

- `crates/bitloom-lsp/src/discover.rs`
- `crates/bitloom-lsp/src/lib.rs`
- `crates/bitloom-lsp/Cargo.toml`
- `crates/bitloom-lsp/fixtures/fr118_syn_ok/**`
- `crates/bitloom-lsp/fixtures/fr118_syn_fail/**`
- `docs/fr118-syn-scan-design-root-discovery.md`
- `docs/fr113-lsp-design-root-discovery.md`
- `crates/bitloom/tests/fr118_syn_scan_design_root_discovery.rs`
- `crates/bitloom/tests/fr113_lsp_design_root_discovery.rs`
- `crates/bitloom/tests/fr113_epic55_closeout.rs`
- `_agile-output/implementation-artifacts/59-2-syn-scan-发现路径实现与验收-fr118.md`
- `_agile-output/implementation-artifacts/59-2-code-review.md`
- `_agile-output/implementation-artifacts/59-2-automation-summary.md`
- `_agile-output/implementation-artifacts/atdd-checklist-59-2-syn-scan-发现路径实现与验收-fr118.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-10: Story 59.2 FR118 syn-scan discovery product path + ATDD
