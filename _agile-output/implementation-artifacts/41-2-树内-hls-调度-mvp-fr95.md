---
title: '41.2 树内 #[hls] 调度 MVP（FR95）'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: 'e97ee0f'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic41-in-tree-hls.md'
  - '{project-root}/_agile-output/implementation-artifacts/41-1-epic-41-nfr14-风险记录.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
  - '{project-root}/docs/fr35-hls.md'
  - '{project-root}/crates/bitloom/src/hls.rs'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR95 要求树内 `#[hls]` 调度/流水/循环展开产品路径；当前 `bitloom::hls` / CLI 仍只走外挂 Bambu（emit C → stub/`BITLOOM_HLS_USE_REAL`），模块注释与 `docs/fr35-hls.md` 仍写「永不树内调度」。仅靠外挂绿灯**不得**关闭 FR95（修订 AD-25；NFR14 Epic 41）。

**Approach:** 交付**文档化子集**的树内调度 MVP：至少一类可演示构造（优先 **loop-unroll**；可选 pipeline II）。在 elaborate/emit-prep 期对已溶解的 `HlsDataflowOp`（或等价描述符）做树内 scheduling/allocation，写出**可检查**的 schedule IR 产物（及可选下游 RTL stub），**全程不 spawn Bambu**。更新 `docs/fr35-hls.md`：树内 = FR95 完成面；外挂 = 可选/对照/FR35，不得单独满足 FR95。品牌仍为 Bitloom。

## Boundaries & Constraints

**Always:** 引用修订后 AD-25；树内路径可验收且不调用外挂；文档区分 FR95 vs FR35；ATDD 证明调度结果/IR；设计依赖 `bitloom-prelude`；公开品牌 Bitloom；NFR14 Epic 41 记录已存在（41.1）。

**Ask First:** 若要把 Handshake/动态数据流默认语义塞进树内路径 — 禁止（AD-25 Prevents）；若要撤回树内主路径 — Correct Course。

**Never:** 仅改文档声称 FR95 已交付；把 stub/`BITLOOM_HLS_USE_REAL` 标成 FR95 done；实现 FR96 闭包内联（→ 41.3）；勾选 Epic 41 关闭（→ 41.4）；开工 41.3+ / Epic 42–47；商业级完整 HLS 编译器（MVP = 文档化可演示子集 + 可检查产物）。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 树内 loop-unroll MVP | `#[hls]` / API：`schedule_in_tree(…, LoopUnroll{n})`，无 Bambu | 写出 schedule IR（含 stage/unroll 元数据）+ 可选 `.v` stub；ATDD 绿 | 非法 n→可读 Err |
| 外挂路径仍可用 | 既有 `run_hls*` / CLI 默认 | 行为不回退；**不算** FR95 | 缺后端仍可读失败 |
| 仅文档无产物 | 改 fr35 话术但无 API/ATDD | **不合格** | 审查拒收 |
| 冒充 FR95 | ATDD 只跑 stub bambu | **不合格** | ATDD 须断言无 bambu spawn / 走 in-tree API |

</frozen-after-approval>

## Code Map

- `crates/bitloom/src/hls.rs` — **UPDATE**：外挂路径保留；新增树内 `schedule_in_tree` / emit schedule IR（或子模块 `hls/in_tree.rs`）
- `crates/bitloom/src/main.rs` — **UPDATE**：CLI `hls --in-tree`（或 `--schedule loop-unroll=N`）走树内，不调 Bambu
- `crates/bitloom-macro/src/lib.rs` — **UPDATE**：`#[hls]` 注释改为允许树内调度标记（宏仍可不做调度，调度在库 API）
- `crates/rhdl-hls/src/lib.rs` — **可选镜像**薄包装或注释对齐（publish=false legacy）
- `docs/fr35-hls.md` — **UPDATE**：FR95 树内完成面 vs FR35 外挂可选
- `crates/bitloom/tests/fr95_in_tree_hls_schedule.rs` — **NEW** ATDD
- `_agile-output/implementation-artifacts/nfr14-risk-epic41-in-tree-hls.md` — 只读门禁；本故事不勾选关闭条件
- **勿改破坏历史门禁：** `nfr14-risk-hls.md`（Epic 24 仍记「当时禁止树内」）；Wave 0 `closure-decision-table` D1 历史裁决（Epic 41.4/治理再翻）；本 MVP 用新 ATDD + 新文档节证明 FR95

## Story

As a HLS 设计者,
I want 树内 `#[hls]`（或等价）调度/流水/循环展开 MVP,
So that 产品路径不再仅依赖外挂调度器交差。

## Acceptance Criteria

1. Given Story 41.1；Epic 40 已修订 AD-25, when 实现树内调度最小可验收路径（文档化子集：至少一类可演示的 **pipeline 或 loop-unroll**）并接通 elaborate/emit 或文档化 IR 产物, then 至少一个黄金/ATDD：带 `#[hls]` 语义的设计在**不调用**外挂 Bambu 的情况下产出可检查的调度结果或下游 RTL/IR
2. And 文档明确树内路径为 FR95 完成面；外挂路径可保留为可选，但不得单独满足 FR95
3. And 公开品牌仍为 Bitloom

## Tasks / Subtasks

- [x] T1: 树内 schedule API + loop-unroll（或 pipeline）MVP；写出可检查 schedule IR / 可选 RTL（AC: 1）
- [x] T2: CLI 接通树内路径（不 spawn bambu）（AC: 1）
- [x] T3: 更新 `docs/fr35-hls.md`（及必要模块注释）：FR95 树内 vs FR35 外挂（AC: 2–3）
- [x] T4: ATDD `fr95_in_tree_hls_schedule`（无 Bambu；断言 IR/stage）（AC: 1–2）
- [x] T5: 回归既有外挂 HLS 测试不崩；sprint → in-progress/done 由 pipeline 推进；**不**开工 41.3+

## Dev Notes

### MVP 形状（务实，须诚实满足 AC）

推荐 **loop-unroll** 演示子集：

1. 输入：函数名 + `HlsDataflowOp`（复用 FR76 溶解描述符）+ `trip_count: u32`（≥1）。
2. 输出：`InTreeScheduleArtifact`：
   - `kind = LoopUnroll { trip_count }`
   - `stages`: 长度为 `trip_count` 的 stage 列表（含 op / index）
   - `schedule_ir`: 可读文本或 JSON（须含 `fr95`、`loop-unroll`、`trip_count`、各 stage）
   - 可选 `rtl_stub`: 简单 `.v` 展示 unroll 后组合体（**非**商业 HLS 质量；诚实标注 `in-tree-mvp`）
3. `emit_in_tree_schedule(artifact, out_dir)` → 写 `{fn}.schedule.json`（或 `.txt`）+ 可选 `{fn}.v`
4. **禁止**在树内路径调用 `resolve_bambu` / `Command::new(bambu)`。

Pipeline 可作为第二模式（II + stages），但 AC 只需一类。

### 架构合规

- **AD-25（修订）：** 允许 crate 内 scheduling/allocation；外挂不得单独关 FR95；禁 Handshake/动态数据流默认。
- **AD-18：** 本故事**不**做 FR96 闭包内联；可继续消费已溶解的 `HlsDataflowOp`。
- **AD-6：** 设计 crate 只依赖 `bitloom-prelude`；调度实现落在 `bitloom`（工具链包），不强迫设计 crate 依赖 CLI。
- **NFR40：** 不得用「改一句文档 + stub 绿」假交 FR95。
- **NFR41：** 实现/文档引用修订后 AD-25。

### 既有代码状态（须保留外挂）

- `bitloom::hls::run_hls*` / CLI 默认仍为 Bambu 外挂（FR35/FR76/FR86）。
- C stub 头注释可保留「external path: no in-tree schedule in this artifact」；树内产物另写，避免破坏「外挂 C 不含 schedule」单测语义——或把单测改为只断言外挂 emit 头。
- `#[bitloom::hls]` 宏：保留 `HlsMark`；更新 rustdoc「可走树内 FR95 或外挂 FR35」。

### 测试要求

- 新 ATDD：`cargo test -p bitloom --test fr95_in_tree_hls_schedule`
  - 调用树内 API（或 CLI `--in-tree`）
  - 断言 schedule 文件存在且含 unroll/stage 元数据
  - 断言路径不需要 `BITLOOM_BAMBU_PATH`；最好用「未设置 bambu 仍成功」证明
  - 文档抽检：`docs/fr35-hls.md` 含 FR95 / 树内完成面 / 外挂不得单独满足
- 勿破坏：`hls_smoke`、`fr76_*`、`fr88_*`、`nfr14_risk_hls`（历史记录原文）、`nfr14_risk_epic41_*`

### Project Structure Notes

- 实现优先放 `crates/bitloom/src/hls.rs` 或 `hls/mod.rs` + `hls/in_tree.rs`（若拆分须 `pub use`）
- 文档权威章仍是 `docs/fr35-hls.md`（可加 FR95 节，避免另起易漂移的平行章）
- 品牌：Bitloom / `bitloom`；不提发布 `rhdl`

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 41 / Story 41.2]
- [Source: `ARCHITECTURE-SPINE.md` — AD-25 revised 2026-09-09]
- [Source: `nfr14-risk-epic41-in-tree-hls.md`]
- [Source: `crates/bitloom/src/hls.rs` — FR35/FR76 baseline]
- [Source: `process-one-story-one-commit.md`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

### Completion Notes List

- 实现 `schedule_in_tree` / `emit_in_tree_schedule` / `run_hls_in_tree`（loop-unroll 主路径 + Pipeline 枚举）；产物 `{fn}.schedule.json` + `{fn}.v`（`in-tree-mvp`）；不 spawn Bambu
- CLI：`cargo bitloom hls --in-tree --unroll N`
- 更新 `docs/fr35-hls.md`：FR95 树内完成面 vs FR35 外挂不得单独满足
- ATDD `fr95_in_tree_hls_schedule` 5 测绿；审查 Approve；automate 认定 ATDD 已足够
- sprint：`41-2: done`；`41-3`/`41-4` 仍 backlog

### File List

- `crates/bitloom/src/hls.rs`
- `crates/bitloom/src/main.rs`
- `crates/bitloom-macro/src/lib.rs`
- `crates/rhdl-hls/src/lib.rs`
- `crates/bitloom/tests/fr95_in_tree_hls_schedule.rs`
- `docs/fr35-hls.md`
- `_agile-output/implementation-artifacts/41-2-树内-hls-调度-mvp-fr95.md`
- `_agile-output/implementation-artifacts/atdd-checklist-41-2-树内-hls-调度-mvp-fr95.md`
- `_agile-output/implementation-artifacts/41-2-code-review.md`
- `_agile-output/implementation-artifacts/41-2-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: Story context created (ready-for-dev)
- 2026-09-09: FR95 in-tree loop-unroll MVP + ATDD + docs（Story 41.2）

## Suggested Review Order

**树内 schedule API / IR 产物** → **ATDD（无 Bambu）** → **docs FR95 vs FR35** → **外挂路径未回退** → **未越界 41.3+**
