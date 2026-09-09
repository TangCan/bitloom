---
title: '31.3 SyncFIFO（或等价）可综合跨域 FIFO'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: '611373c'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic31-cdc-true-rtl.md'
  - '{project-root}/_agile-output/implementation-artifacts/31-2-doubleflop-可综合同步器-rtl.md'
  - '{project-root}/docs/fr79-doubleflop-cdc.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
  - '{project-root}/crates/bitloom-prelude/src/lib.rs'
  - '{project-root}/crates/bitloom-prelude/src/ip.rs'
  - '{project-root}/crates/bitloom-builder/src/lib.rs'
warnings: []
deferred:
  - 'CDC 深度 ATDD 总收口 + NFR14 Epic 31 关闭勾选 → Story 31.4'
  - '独立 per-domain / dual physical clock 引擎（仍用全局 Sim::tick MVP）'
  - '可配置 DEPTH/WIDTH 全家桶（本故事钉死默认 4×8 MVP）'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** `SyncFIFO` 仍为 prelude ZST / `mark_cdc_bridge` 叙事；无跨域 FIFO 真 RTL，不足以交差 FR79 SyncFIFO 腿。一级 IP `ip::SyncFifo`（FR82 单时钟）不得被冒充为语言级 CDC SyncFIFO。

**Approach:** 实现语言级 `SyncFIFO`（默认 DEPTH=4、WIDTH=8）：phantom 双域 + 灰码指针经 DoubleFlop 跨域同步 + mem/full/empty；elaborate→emit→tick；文档钉死参数与延迟/满空最小子集；保留 E0220；不破坏 FR82 SyncFifo 基线。

## Boundaries & Constraints

**Always:** FR79 / AD-29；依赖 31.2 域绑定/DoubleFlop/emit 通路；深度/宽度文档化；夹具跨域写/读 + 满/空（或书面最小子集）；非法未标记跨域仍失败；区分 `SyncFIFO` vs `ip::SyncFifo`；品牌 Bitloom；设计 crate 只依赖 `bitloom-prelude`。

**Ask First:** 扩展非 4×8 深度/宽度产品矩阵；真实双物理时钟端口（`wr_clk`/`rd_clk`）。

**Never:** 仅改文档声称真 RTL；仅 ZST 无 FIFO/同步网表；削弱 E0220；把 `ip::SyncFifo` 改成 CDC 或破坏 FR82 ATDD；勾选 Epic 31 NFR14 关闭（→ 31.4）；引入闭包进 tick（AD-18）。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| SyncFIFO elaborate/emit | `SyncFIFO::<4,8>::elaborate()` | `.v` 含 mem/FIFO 结构 + 跨域 sync FF（灰码 DoubleFlop）+ full/empty | finish Err → 诊断 |
| 跨域写→读 | wr 一拍后等文档延迟再 rd | data_out 匹配；empty/full 符合文档 | ATDD fail |
| 满 | 写满 DEPTH 且读侧同步后 | full=1；再写忽略 | assert |
| 非法跨域 | 无 bridge 的 assign 跨域 | `rhdl::E0220` | 不得 emit |
| FR82 回归 | `ip::SyncFifo` 夹具 | 仍绿；模块名 `SyncFifo` ≠ `SyncFIFO` | 不得改坏 |

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/lib.rs` — 升级 `SyncFIFO` 为 Elaboratable 真 RTL（参数/延迟/与 IP 消歧文档）
- `crates/bitloom-builder/src/lib.rs` — 复用 `declare_double_flop_stages` / `mark_cdc_bridge` / mem API（必要时薄辅助）
- `crates/bitloom/tests/fr79_syncfifo_rtl.rs` — ATDD
- `examples/syncfifo_skel/` — FR79 夹具（prelude-only design dep）
- `docs/fr79-syncfifo-cdc.md` + `docs/fr79-doubleflop-cdc.md` + `language-surface.md` — 真 RTL vs IP SyncFifo
- `crates/bitloom-prelude/src/ip.rs` — 仅文档交叉引用，不改 FR82 行为
- NFR14 记录只读（关闭勾选 → 31.4）

## Story

As a 多时钟设计者,
I want 语言级跨域 FIFO 原语发出可综合 RTL,
So that 多位/流式 CDC 有合同路径。

## Acceptance Criteria

1. Given Story 31.2 的域绑定与 emit 通路，when 实现 SyncFIFO（或文档等价）：可 elaborate/emit/tick，深度/宽度参数文档化（FR79），then 夹具证明跨域写入/读取在文档延迟与满/空语义下正确（或文档化最小子集）
2. And 未标记 CDC 的非法跨域仍失败
3. And Verilog（及可选 FIRRTL）抽检非空同步/FIFO 结构
4. And 不破坏 `ip::SyncFifo` FR82 基线（命名/行为消歧）

## Tasks / Subtasks

- [x] T1: prelude `SyncFIFO<4,8>` Elaboratable 真 RTL + 参数/延迟/vs IP 文档（AC: 1, 3, 4）
- [x] T2: 域绑定 + 灰码指针 DoubleFlop 同步 + mem full/empty（AC: 1, 3）
- [x] T3: example `syncfifo_skel` + `docs/fr79-syncfifo-cdc.md` + language-surface（AC: 1–4）
- [x] T4: ATDD `fr79_syncfifo_rtl.rs`；确认 FR82 夹具仍绿（AC: 1–4）
- [x] T5: code-review Approve；sprint `31-3: done`（epic-31 保持 in-progress）

## Dev Notes

- **真 RTL 定义：** emit `.v` 含可识别 FIFO（mem + ptr/full/empty）**且**跨域同步结构（DoubleFlop 灰码指针）；非空 ZST。
- **文档化最小子集：** 默认 `DEPTH=4`、`WIDTH=8`；单物理 `clk`/`rst` + phantom 双域（与 DoubleFlop MVP 一致）；`Sim::tick` = 按域 tick MVP。
- **延迟合同（钉死）：** 写后 empty 经灰码 DoubleFlop 同步（`LATENCY_PTR_SYNC_TICKS = 2`）才在读侧清除；注册读 `data_out` 再 +1 tick。见 `docs/fr79-syncfifo-cdc.md`。
- **vs `ip::SyncFifo`：** FR82 单时钟一级 IP；本故事语言级 CDC `SyncFIFO`。模块/类型名大小写与路径均不同。
- **合同外：** 非 MTBF/硅片亚稳态签核；非完整可配置异步 FIFO 产品矩阵。
- 品牌 Bitloom；公开表面经 `bitloom-prelude`。

### Project Structure Notes

- 夹具对齐 `doubleflop_skel`（design dep = prelude only）
- ATDD 落在 `crates/bitloom/tests/`

### References

- [Source: `epics.md` — Story 31.3 / FR79]
- [Source: `nfr14-risk-epic31-cdc-true-rtl.md` — SyncFIFO vs IP；禁止仅文档]
- [Source: ARCHITECTURE-SPINE AD-29 / AD-22]
- [Source: Story 31.2 DoubleFlop 模式]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

- `cargo test -p bitloom --test fr79_syncfifo_rtl` — 6 passed
- `cargo test -p syncfifo_skel` — 4 passed
- `cargo test -p bitloom --test fr79_doubleflop_rtl --test fr82_fifo_uart_baseline --test nfr14_risk_epic31_cdc_true_rtl` — 绿
- `cargo test -p bitloom-prelude --lib sync_fifo` — 2 passed
- 未跑全量 `cargo clean && just test`（按故事管线）

### Completion Notes List

- `SyncFIFO::<4,8>::elaborate` → mem + 灰码 wr/rd ptr + `w2r`/`r2w` DoubleFlop + Cummings full/empty
- 夹具 `examples/syncfifo_skel`；文档 `docs/fr79-syncfifo-cdc.md` + language-surface + IP README 消歧
- ATDD 覆盖参数、emit 结构、跨域写读延迟、满语义、E0220、≠ SyncFifo
- code-review **Approve**；sprint `31-3: done`，`epic-31: in-progress`
- testarch-automate：本故事 ATDD 已覆盖 AC；Epic 总收口留给 31.4
- `_bmad/scripts/render_skill.py.bak` 保持 untracked

### File List

- `crates/bitloom-prelude/src/lib.rs`
- `crates/bitloom-prelude/src/ip.rs`
- `crates/bitloom/tests/fr79_syncfifo_rtl.rs`
- `examples/syncfifo_skel/Cargo.toml`
- `examples/syncfifo_skel/src/lib.rs`
- `docs/fr79-syncfifo-cdc.md`
- `docs/fr79-doubleflop-cdc.md`
- `docs/ip/README.md`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `Cargo.toml`
- `_agile-output/implementation-artifacts/31-3-syncfifo-或等价-可综合跨域-fifo.md`
- `_agile-output/implementation-artifacts/31-3-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: Story 31.3 SyncFIFO 可综合跨域 FIFO + ATDD + Approve

## Suggested Review Order

**prelude SyncFIFO RTL** → **CDC/灰码/满空** → **emit/tick 黄金** → **E0220 负例** → **FR82 未破** → **docs 消歧**
