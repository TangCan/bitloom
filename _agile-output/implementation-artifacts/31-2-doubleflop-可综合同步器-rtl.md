---
title: '31.2 DoubleFlop 可综合同步器 RTL'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: '3ae14f1'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic31-cdc-true-rtl.md'
  - '{project-root}/_agile-output/implementation-artifacts/31-1-epic-31-nfr14-风险记录.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
  - '{project-root}/_agile-output/specs/spec-rhdl/language-surface.md'
  - '{project-root}/examples/clockdomain_skel/src/lib.rs'
  - '{project-root}/crates/bitloom-prelude/src/lib.rs'
  - '{project-root}/crates/bitloom-builder/src/lib.rs'
warnings: []
deferred:
  - 'SyncFIFO 真 RTL → Story 31.3'
  - 'CDC 深度 ATDD 总收口 + NFR14 Epic 31 关闭勾选 → Story 31.4'
  - '独立 per-domain tick 引擎（仍用全局 Sim::tick MVP 等价）'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** `DoubleFlop` 今日为 prelude ZST + `mark_cdc_bridge` 叙事；emit 无两级同步寄存器网表，不足以交差 FR79 / AD-29 / NFR37。

**Approach:** 将 `DoubleFlop` 升级为可 elaborate 的真 RTL（prelude API → HIR → `.v` 含可识别两级 sync FF）；session 辅助声明/连接；夹具/ATDD 按文档延迟语义对源/目的域 tick（`Sim::tick` MVP）；保留非法未同步跨域 E0220。不实现 SyncFIFO 真 RTL（→ 31.3）。

## Boundaries & Constraints

**Always:** FR79 / AD-29；默认 **2** 级；文档钉死目的域 tick 延迟；emit 含真实寄存器链路（非空 ZST）；非法跨域仍失败（FR23）；设计 crate 只依赖 `bitloom-prelude`；品牌 Bitloom；依赖 Story 31.1 NFR14。

**Ask First:** 无。

**Never:** 仅改文档声称真 RTL；仅 ZST 无网表交差；削弱 E0220；实现 SyncFIFO 真 RTL 或勾选 Epic 31 关闭（→ 31.3/31.4）；把物理亚稳态/MTBF 写进默认完成定义。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| DoubleFlop elaborate/emit | `DoubleFlop::elaborate()` | `.v` 含 `sync_*_ff0`/`ff1`（或文档名）两级 reg + always | finish Err → 诊断 |
| 延迟黄金 | rst 后 din=1，dst tick×2 | dout 在第 2 个目的域 tick 后为 1 | 文档钉死级数=2 |
| 非法跨域 | 无 bridge 的 assign_net 跨域 | `rhdl::E0220` | 不得 emit |
| 合法 DoubleFlop 路径 | 经 declare/elaborate 同步链 | finish OK；网表非空 | N/A |
| 仅 ZST 回归 | 空结构体无 RegDecl | ATDD 红（NFR37） | 必须有 sync FF |

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/lib.rs` — 升级 `DoubleFlop`（Elaboratable + 文档：级数/延迟/非 MTBF）
- `crates/bitloom-builder/src/lib.rs` — `declare_double_flop_stages` / CDC 检查延伸至 `assign_reg_d_from`；保留 `mark_cdc_bridge`
- `crates/bitloom-sim/src/lib.rs` — RegD **NBA**（先算全 next 再提交），保证双 FF 延迟不为同拍折叠
- `crates/bitloom-vlog` — 既有 RegDecl emit（抽检两级 reg）
- `examples/doubleflop_skel/` — FR79 夹具（prelude-only design dep）
- `crates/bitloom/tests/fr79_doubleflop_rtl.rs` — ATDD
- `docs/fr79-doubleflop-cdc.md` + `language-surface.md` — 真 RTL vs Epic 7 最小合同
- `_agile-output/implementation-artifacts/nfr14-risk-epic31-cdc-true-rtl.md` — 只读引用延迟边界（关闭勾选留给 31.4）

## Story

As a 多时钟设计者,
I want `DoubleFlop`（或文档等价）emit 真实双触发器同步链路,
So that CDC 位同步可进入综合与 tick。

## Acceptance Criteria

1. Given Story 31.1；既有 phantom 域 / FR23 非法跨域失败，when 实现 DoubleFlop：prelude API → HIR → emit `.v` 含可识别的两级（或文档钉死级数）同步寄存器（FR79 / AD-29），then 夹具可 elaborate、emit，并对源/目的域按文档方式 tick，黄金值满足同步延迟语义
2. And 非法未同步跨域仍 freeze/诊断失败（继承 FR23）
3. And 不得仅保留空 ZST 无网表（NFR37）

## Tasks / Subtasks

- [x] T1: builder `declare_double_flop_stages` + `assign_reg_d` CDC 检查（AC: 1–2）
- [x] T2: prelude `DoubleFlop` Elaboratable 真 RTL + 级数/延迟文档（AC: 1, 3）
- [x] T3: sim RegD NBA（双 FF 延迟正确）（AC: 1）
- [x] T4: example `doubleflop_skel` + language-surface / `docs/fr79-doubleflop-cdc.md`（AC: 1–3）
- [x] T5: ATDD `fr79_doubleflop_rtl.rs`（AC: 1–3）
- [x] T6: code-review Approve；sprint `31-2: done`（epic-31 保持 in-progress）

## Dev Notes

- **真 RTL 定义（NFR14）：** emit `.v` 含可识别两级 sync 寄存器 + 按域 tick 黄金；非注释形/空 ZST。
- **延迟合同：** `STAGES = 2`；`dout` 相对稳定 `din` 延迟 **2** 个目的域 tick；`Sim::tick` = 按域 tick MVP（与 FR52 一致）。
- **合同外：** 不保证硅片亚稳态消除 / MTBF。
- **vs Epic 7 / FR52：** `clockdomain_skel` 的 `mark_cdc_bridge` 最小合同仍保留；本故事新增真 RTL 路径，不得用历史 done 冒充 FR79。
- **SyncFIFO：** 仍可为 ZST 叙事至 31.3。
- 品牌 Bitloom；公开表面经 `bitloom-prelude`。

### Project Structure Notes

- 夹具 `examples/doubleflop_skel` 对齐 `clockdomain_skel` / `fifo_skel`（design dep = prelude only）
- ATDD 落在 `crates/bitloom/tests/`

### References

- [Source: `epics.md` — Story 31.2 / FR79]
- [Source: `nfr14-risk-epic31-cdc-true-rtl.md` — 双 FF 延迟/亚稳态边界]
- [Source: ARCHITECTURE-SPINE AD-29 / AD-22]
- [Source: `language-surface.md` — ClockDomain / CDC]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

- `cargo test -p bitloom --test fr79_doubleflop_rtl` — 5 passed
- `cargo test -p doubleflop_skel` — 4 passed
- `cargo test -p bitloom-builder -- double_flop…` — 4 passed
- 回归：`clockdomain_skel`、`bitloom-sim`、`fr82_fifo_uart_baseline`、`rv32_pipeline_feasibility` 绿
- bmad-build `render_skill.py` HALT（ambiguous implementation_artifacts）；按既有管线继续

### Completion Notes List

- `DoubleFlop::elaborate` / `elaborate_width` → `sync_ff0`/`sync_ff1` 真 RTL
- builder：`declare_double_flop_stages` + `connect_double_flop`；`assign_reg_d_from` 亦查 E0220
- sim RegD NBA 保证 2-tick 延迟
- 夹具 `examples/doubleflop_skel`；文档 `docs/fr79-doubleflop-cdc.md` + language-surface
- code-review **Approve**；sprint `31-2: done`，`epic-31: in-progress`
- testarch-automate：ATDD 已覆盖本故事 AC；总收口留给 31.4

### File List

- `crates/bitloom-prelude/src/lib.rs`
- `crates/bitloom-builder/src/lib.rs`
- `crates/bitloom-sim/src/lib.rs`
- `crates/bitloom/tests/fr79_doubleflop_rtl.rs`
- `examples/doubleflop_skel/Cargo.toml`
- `examples/doubleflop_skel/src/lib.rs`
- `docs/fr79-doubleflop-cdc.md`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `Cargo.toml`
- `_agile-output/implementation-artifacts/31-2-doubleflop-可综合同步器-rtl.md`
- `_agile-output/implementation-artifacts/31-2-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: Story 31.2 DoubleFlop 可综合同步器 RTL + ATDD + Approve

## Suggested Review Order

**prelude DoubleFlop RTL** → **builder/CDC** → **sim NBA** → **emit/tick 黄金** → **负例 E0220** → **docs**
