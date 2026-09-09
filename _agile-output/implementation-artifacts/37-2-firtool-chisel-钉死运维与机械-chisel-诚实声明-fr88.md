---
title: '37.2 firtool/Chisel 钉死运维与机械 Chisel 诚实声明（FR88）'
type: 'docs'
created: '2026-09-09'
status: 'done'
baseline_commit: 'dd7e698'
review_loop_iteration: 0
context:
  - '{project-root}/docs/fr28-chisel-compilable.md'
  - '{project-root}/docs/fr46-chisel-import.md'
  - '{project-root}/README.md'
  - '{project-root}/docs/nfr11-firtool-platforms.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic37-interop-hls.md'
  - '{project-root}/_agile-output/implementation-artifacts/37-1-epic-37-nfr14-风险记录.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
  - '{project-root}/_agile-output/implementation-artifacts/process-one-story-one-commit.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR88（firtool/Chisel 条）要求用户/维护者文档钉死 Chisel↔firtool 运维（版本对、缓存、覆盖入口）并公开「可编译 ≠ idiomatic」；现有 `docs/fr28-chisel-compilable.md` 已写版本对与机械风格，但缺完整运维清单与足够醒目的诚实声明；README firtool 节与 FR28 交叉链不完整；FR28/FR46 文档可能仍被读成「可维护手写风格」。

**Approach:** 以文档合同为主（不强制改 `emit_chisel`）：在 `docs/fr28-chisel-compilable.md`（及必要时 `fr46`）落盘钉死运维清单 + 「可编译 ≠ idiomatic」公开声明；README 交叉链到该清单；抽检并修正误导表述（NFR39）。用 docs ATDD 锁住版本对、覆盖 env、诚实声明与交叉链。本故事**不**落地 37.3 HLS 夜间/stub 选型。

## Boundaries & Constraints

**Always:** Gate 37.1 NFR14 已存在；钉死对 Chisel **7.14.0** ↔ firtool **1.155.0**；文档化缓存/覆盖（`RHDL_FIRTOOL_PATH` 或文档等价，可提 `RHDL_FIRTOOL_CACHE`）；公开「可编译 ≠ idiomatic」；抽检 FR28/FR46 无「手写可维护 idiomatic」误导；品牌 Bitloom；NFR12 / NFR39；one-story-one-commit。

**Ask First:** 若发现实现与合同冲突（emit 行为与「可编译≠idiomatic」矛盾）— 记入 NFR14 风险记录，不静默改合同。

**Never:** 开工 Story 37.3；私自升 firtool/Chisel 钉死对；把机械 Chisel 写成 idiomatic / 可维护手写风格；把 stub CI 写成 HLS 质量已验；宣称 FR93 idiomatic 非目标已交付；改树内 HLS。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 钉死运维清单 | `docs/fr28-chisel-compilable.md`（或等价） | 含 7.14.0 ↔ 1.155.0；缓存/覆盖入口（`RHDL_FIRTOOL_PATH` 等）；禁默认裸 PATH 冒充钉死 | ATDD 红直至落盘 |
| 诚实声明 | 同上 + README 交叉链 | 明确 emit_chisel / 往返 = 可编译 + 端口/层次谓词，**不**要求 idiomatic Scala | ATDD 红 |
| 误导抽检 | FR28 / FR46 产品文档 | 无「可维护手写风格 / idiomatic」冒充完成话术 | 修正文档；ATDD 可锁关键诚实句 |
| 实现可选 | `emit_chisel` 源码 | 本故事可不改；若冲突记 NFR14 | Ask First |
| 范围越界 | 试图做 37.3 Bambu 夜间 | 拒收 | 审查拒收 |

</frozen-after-approval>

## Code Map

- `docs/fr28-chisel-compilable.md` — **UPDATE** 增加/钉死：(1) firtool/Chisel 运维清单（版本对、`firtool ensure`/缓存、`RHDL_FIRTOOL_PATH`/`RHDL_FIRTOOL_CACHE`、禁止 PATH 冒充）；(2) 醒目「可编译 ≠ idiomatic」公开声明（FR88 / AD-27）
- `docs/fr46-chisel-import.md` — **UPDATE**（若缺）交叉声明反向腿亦不要求 idiomatic；钉死对与 FR28 一致
- `README.md` — **UPDATE** firtool 节与/或合同表交叉链到 FR28 运维清单 + 诚实声明；表项可注明 FR88
- `docs/nfr11-firtool-platforms.md` — **可选** 一句指针到 FR28 运维清单（不扩大范围）
- `crates/bitloom/tests/fr88_firtool_chisel_ops_honesty.rs` — **NEW** docs ATDD
- `_agile-output/implementation-artifacts/nfr14-risk-epic37-interop-hls.md` — **不**勾选 Epic 37 关闭条件（留给 37.3）；本故事可在关闭条件 firtool 条旁备注进度，但正式勾选归 37.3
- `_agile-output/implementation-artifacts/sprint-status.yaml` — `37-2-…` → done（实现收口时）

## Story

As a 工具链维护者 / 用户,
I want 钉死版本运维清单与「可编译 ≠ idiomatic」公开声明,
So that 互操作边界诚实可复现。

## Acceptance Criteria

1. Given Story 37.1；AD-9 / NFR12；AD-27 / FR28，when 更新用户/维护者文档（至少：`docs/fr28-chisel-compilable.md` 或等价 + README 交叉链），then 写明当前钉死 Chisel↔firtool 版本对与缓存/覆盖入口（`RHDL_FIRTOOL_PATH` 或文档等价）
2. And 明确声明 emit_chisel / 往返验收 = **可编译 + 端口/层次谓词**，**不**要求 idiomatic Scala（FR88）
3. And 抽检既有 FR28/FR46 文档无「可维护手写风格」误导表述（NFR39）
4. And 本故事不强制改 emit 实现（文档合同即可；若发现实现与合同冲突须记入风险记录）

## Tasks / Subtasks

- [x] T1: 更新 `docs/fr28-chisel-compilable.md` — 运维清单 + 诚实声明（AC: 1–2）
- [x] T2: 抽检/修正 `docs/fr46-chisel-import.md`；README 交叉链（AC: 1–3）
- [x] T3: ATDD `fr88_firtool_chisel_ops_honesty.rs`（AC: 1–3）
- [x] T4: sprint `37-2` → done；code-review / automate 收口；**不**开工 37.3；**不**勾选 NFR14 Epic 37 全关闭

## Dev Notes

### 钉死对（不得私自升）

| 组件 | 版本 |
|------|------|
| Chisel | **7.14.0** |
| firtool | **1.155.0** |

升钉须上游正式配对并更新 ARCHITECTURE-SPINE Stack / AD-9 / addendum / 本运维清单（NFR12）。源码常量：`rhdl_firrtl::{CHISEL_TARGET,FIRTOOL_TARGET}`。

### 运维入口（须文档化）

- CLI：`cargo run -p bitloom -- firtool info|ensure`（下载/校验/缓存）
- 覆盖：`RHDL_FIRTOOL_PATH`（含 `firtool` 的目录）；缓存根：`RHDL_FIRTOOL_CACHE`
- 默认**不信任** PATH 上的 firtool（NFR3）
- Chisel JVM 编译：`just chisel-fr28-jvm` / GHA `fr28-chisel-jvm`（FR71）；本故事只文档交叉，不改 CI

### 诚实声明（须公开落盘）

> `emit_chisel` / FR28↔FR46 往返验收 = **钉死栈下可编译** + **公开端口名/宽/向与实例层次谓词**。允许机械/生成风格。**不**要求 idiomatic / 手写可维护 Scala。FIRRTL→idiomatic Scala 为 **FR93 永久非目标**（须新 PRD）。

### 现有文件现状（实现前）

- `docs/fr28-chisel-compilable.md`：已有版本对与「允许机械风格」；缺独立运维节与醒目「≠ idiomatic」句；无 `RHDL_FIRTOOL_*` 覆盖说明。
- `README.md` §firtool：已有 1.155.0 + `RHDL_FIRTOOL_PATH`/`CACHE`；合同表链到 FR28，但未强调 FR88 诚实声明。
- `docs/fr46-chisel-import.md`：有钉死对；无 idiomatic 否定句。
- 抽检：产品 `docs/fr28*` / `fr46*` 当前无「可维护手写」误导；实现时再扫一遍并锁住诚实句。

### Previous story intelligence（37.1）

- NFR14 记录已开门禁；37.2 可 ready。
- 禁止私自升 firtool；禁止机械冒充 idiomatic；HLS 条留给 37.3。
- ATDD 体例：`crates/bitloom/tests/nfr14_risk_epic37_interop_hls.rs` / `fr93_permanent_non_goals.rs`（读文件 assert 关键词）。

### Project Structure Notes

- 文档优先：`docs/` + README；测试：`crates/bitloom/tests/`
- 设计 crate 仍只依赖 `bitloom-prelude`；本故事预期零 emit 代码改动

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 37 / Story 37.2 / FR88]
- [Source: `_agile-output/implementation-artifacts/nfr14-risk-epic37-interop-hls.md`]
- [Source: `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md` — NFR12]
- [Source: ARCHITECTURE-SPINE AD-9 / AD-27]
- [Source: `_agile-output/implementation-artifacts/process-one-story-one-commit.md`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

- ATDD red→green: `cargo test -p bitloom --test fr88_firtool_chisel_ops_honesty` — 7 passed（红相 2 失败：缺 `RHDL_FIRTOOL_PATH` / ≠ idiomatic）

### Completion Notes List

- `docs/fr28-chisel-compilable.md`：运维清单（7.14.0↔1.155.0、`firtool ensure`、`RHDL_FIRTOOL_PATH`/`CACHE`、禁 PATH 冒充）+ 公开「可编译 ≠ idiomatic」
- `docs/fr46-chisel-import.md`：对称诚实边界 + 运维交叉链
- README 合同表 / firtool 节交叉链 FR88；`nfr11` 指针
- ATDD `fr88_firtool_chisel_ops_honesty`；未改 emit；未开工 37.3；未勾选 NFR14 Epic 37 全关闭

### File List

- `docs/fr28-chisel-compilable.md`
- `docs/fr46-chisel-import.md`
- `docs/nfr11-firtool-platforms.md`
- `README.md`
- `crates/bitloom/tests/fr88_firtool_chisel_ops_honesty.rs`
- `_agile-output/implementation-artifacts/37-2-firtool-chisel-钉死运维与机械-chisel-诚实声明-fr88.md`
- `_agile-output/implementation-artifacts/atdd-checklist-37-2-firtool-chisel-钉死运维与机械-chisel-诚实声明-fr88.md`
- `_agile-output/implementation-artifacts/37-2-code-review.md`
- `_agile-output/implementation-artifacts/37-2-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: FR88 firtool/Chisel 钉死运维清单 + 可编译≠idiomatic 诚实声明 + ATDD（Story 37.2）

## Suggested Review Order

**FR28 运维清单 + 诚实声明** → **README 交叉链** → **FR46 抽检** → **ATDD** → **sprint 37-2（未开工 37.3）**
