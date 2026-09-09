---
title: '31.4 CDC 深度 ATDD + 文档叙事收口'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: '6492222'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic31-cdc-true-rtl.md'
  - '{project-root}/_agile-output/implementation-artifacts/31-2-doubleflop-可综合同步器-rtl.md'
  - '{project-root}/_agile-output/implementation-artifacts/31-3-syncfifo-或等价-可综合跨域-fifo.md'
  - '{project-root}/docs/fr79-doubleflop-cdc.md'
  - '{project-root}/docs/fr79-syncfifo-cdc.md'
  - '{project-root}/examples/clockdomain_skel/src/lib.rs'
  - '{project-root}/_agile-output/specs/spec-rhdl/language-surface.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Epic 31 真 RTL（31.2 DoubleFlop + 31.3 SyncFIFO）已落地，但缺统一 ATDD/文档叙事收口：ClockDomain/CDC 用户面仍易被 FR52/`mark_cdc_bridge` 最小合同话术淹没；NFR14 Epic 31 关闭条件未勾选；NFR37 诚实度未在产品入口钉死。

**Approach:** 以文档 + 轻量 docs/矩阵 ATDD 收口：跟练页串联既有 `fr79_doubleflop_rtl` / `fr79_syncfifo_rtl` 黄金与负例，并对照 FR52「仅 ZST + bridge」夹具；更新 README / language-surface / clockdomain_skel 叙事；勾选 NFR14 Epic 31 关闭条件；sprint `31-4` + `epic-31` → done。不新增 prelude/HIR API。

## Boundaries & Constraints

**Always:** 依赖 31.2–31.3 真 RTL；ATDD 覆盖 DoubleFlop + SyncFIFO 黄金、非法跨域负例、与「仅 ZST」对照（若可测）；文档化 `just test` 或等价配方；ClockDomain/CDC 真 RTL vs 历史最小合同（FR52 叙事 + NFR37）；NFR14 关闭勾选；品牌 Bitloom。

**Ask First:** 无。

**Never:** `cargo clean` / 全量 `just test` 作为本故事强制门禁（可文档化配方 + 跑定向 ATDD）；git commit/push；仅改文档声称真 RTL 而无既有 emit/tick 黄金；削弱 E0220；把 FR52 `mark_cdc_bridge` 冒充 FR79 深度关闭。

## I/O & Edge-Case Matrix

| Scenario | Expected |
|----------|----------|
| DoubleFlop 黄金 | `fr79_doubleflop_rtl` 仍绿（两级 sync + 2-tick 延迟） |
| SyncFIFO 黄金 | `fr79_syncfifo_rtl` 仍绿（FIFO+sync + 跨域写读/满） |
| 负例非法跨域 | E0220 仍失败（夹具内与 docs ATDD 点名） |
| 仅 ZST 对照 | FR52 `mark_cdc_bridge` emit 无 `sync_ff*`；`DoubleFlop`/`SyncFIFO` elaborate 有真网表 |
| 用户文档 | ClockDomain/CDC：真 RTL 要求 vs Epic 7/FR52 最小合同 |
| NFR14 | Epic 31 关闭条件全 `[x]` |
| 配方 | 跟练页给出定向 `cargo test`；并指向 `just test` |

</frozen-after-approval>

## Code Map

- `docs/tutorials/cdc-depth.md` — **NEW** CDC 深度跟练（FR79 收口）
- `docs/fr79-doubleflop-cdc.md` / `docs/fr79-syncfifo-cdc.md` — 跟练入口 + 收口交叉链接
- `README.md` — FR52/FR79 ClockDomain/CDC 索引
- `examples/clockdomain_skel/src/lib.rs` — 文案：FR52 最小 vs FR79 真 RTL
- `_agile-output/specs/spec-rhdl/language-surface.md` — Story 31.4 收口标记
- `_agile-output/implementation-artifacts/nfr14-risk-epic31-cdc-true-rtl.md` — 关闭勾选 + 现状刷新
- `crates/bitloom/tests/fr79_cdc_depth_closeout.rs` — **NEW** docs/矩阵 ATDD

## Story

As a 质量负责人 / 学习者,
I want ATDD 与产品叙事证明 FR79 深度，而非 Epic 7 最小合同,
So that NFR37 可勾选.

## Acceptance Criteria

1. Given Story 31.2–31.3, when 增加 ATDD：DoubleFlop + SyncFIFO 黄金；负例非法跨域；与「仅 ZST」回归对照（若可测）, then `just test`（或文档化配方）稳定通过（FR79）
2. And 用户文档更新 ClockDomain/CDC 章节：真 RTL 要求、与历史最小合同区别（FR52 叙事 + NFR37）
3. And NFR14 记录勾选 Epic 31 关闭条件

## Tasks / Subtasks

- [x] T1: CDC 深度跟练页 + README / language-surface / fr79 docs / clockdomain_skel 叙事（AC: 1–2）
- [x] T2: ATDD `fr79_cdc_depth_closeout` + 确认 `fr79_doubleflop_rtl` / `fr79_syncfifo_rtl` 绿（AC: 1）
- [x] T3: NFR14 Epic 31 关闭条件全勾；刷新 crates 现状段（AC: 3）
- [x] T4: code-review Approve；sprint `31-4` + `epic-31` done

## Dev Notes

- **落点：** 文档 + 轻量 docs/对照 ATDD；复用既有 `fr79_*_rtl` 黄金，不重写同步器。
- **仅 ZST 对照：** FR52 `mark_cdc_bridge` 路径 emit 无 `sync_ff*`；`DoubleFlop`/`SyncFIFO::elaborate` 产 RegDecl/MemDecl — ATDD 断言对照。
- **配方：** 跟练页列定向测试；贡献者全量仍为 `just test`（本故事不跑 `cargo clean && just test`）。
- **品牌：** Bitloom；crates `bitloom` / `bitloom-prelude`。

### References

- [Source: `epics.md` — Story 31.4 / FR79]
- [Source: `nfr14-risk-epic31-cdc-true-rtl.md` — Epic 31 关闭条件]
- [Source: Stories 31.2 / 31.3 completion + code-review 轻量建议]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

- ATDD: `cargo test -p bitloom --test fr79_cdc_depth_closeout` — 4 passed
- Goldens: `fr79_doubleflop_rtl` (5) + `fr79_syncfifo_rtl` (6) + `nfr14_risk_epic31_cdc_true_rtl` (1) — ok
- Skels: `clockdomain_skel` / `doubleflop_skel` / `syncfifo_skel` — ok
- testarch-automate: docs/矩阵 ATDD 已覆盖 AC；无需额外 automate 套件
- User constraint: no `cargo clean` / full `just test` / commit

### Completion Notes List

- UJ「CDC 深度」：`docs/tutorials/cdc-depth.md` → 黄金 + 负例 + ZST 对照 + `just test` 配方
- README / language-surface / clockdomain_skel / fr79 docs：FR52 最小 vs FR79 真 RTL（NFR37）
- NFR14 Epic 31 关闭条件全勾；crates 现状刷新；`epic-31: done`
- code-review **Approve**
- `_bmad/scripts/render_skill.py.bak` 保持 untracked

### File List

- `docs/tutorials/cdc-depth.md`
- `docs/fr79-doubleflop-cdc.md`
- `docs/fr79-syncfifo-cdc.md`
- `README.md`
- `examples/clockdomain_skel/src/lib.rs`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic31-cdc-true-rtl.md`
- `crates/bitloom/tests/fr79_cdc_depth_closeout.rs`
- `_agile-output/implementation-artifacts/31-4-cdc-深度-atdd-文档叙事收口.md`
- `_agile-output/implementation-artifacts/31-4-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: CDC depth ATDD + docs narrative close-out + Epic 31 NFR14 close（Story 31.4）

## Suggested Review Order

**跟练页** → **ZST 对照 ATDD** → **用户文档叙事** → **NFR14 勾选** → **sprint epic-31: done**
