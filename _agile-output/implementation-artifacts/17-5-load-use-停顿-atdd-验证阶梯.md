---
title: '17.5 Load-use 停顿 ATDD + 验证阶梯'
type: 'feature'
created: '2026-08-20'
status: 'done'
baseline_commit: 'f7a0b2344fe9c923f8a574b13e8de2f79f1a3c23'
review_loop_iteration: 1
context:
  - '{project-root}/_agile-output/implementation-artifacts/epic-17-context.md'
  - '{project-root}/examples/rv32_pipe/PIPE.md'
  - '{project-root}/examples/rv32_core/COMPLIANCE.md'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** 17.4 已有五级 + ALU 转发 + 分支 flush，但缺 load-use 停顿；无停顿时紧邻 LW 的消费者会静默用陈旧 RF；COMPLIANCE 也未写清 Episode II 验证阶梯。

**Approach:** 在 `EpisodeIIPipe` 加 Harris 式 load-use：检测 ID/EX 为 LW 且 IF/ID 源寄存器匹配 → 冻结 PC/IF-ID、向 ID/EX 插 bubble，随后靠既有 MEM/WB→EX 转发；独立命名 ATDD tick；更新 COMPLIANCE/PIPE 验证阶梯（定向 → 可选 rv32ui）。

## Boundaries & Constraints

**Always:**
- 品牌 Bitloom；设计依赖仅 `bitloom-prelude`
- 取指仍为 harness `instr`（17.2）；stall 用按级 mux hold（非模块级 `en`）
- 保留 17.4 转发（EX/MEM 非 LW 优先，其次 MEM/WB）与 predict-not-taken flush
- 独立 ATDD：无停顿会失败、有正确停顿则通过
- 验证阶梯文档：定向 →（可选）`rv32ui`；不宣称 arch-test / 完整 DV
- `cargo test -p rv32_pipe`、`-p rv32_core`；`cargo bitloom build --package rv32_pipe`

**Ask First:**
- 若必须改 `bitloom-builder`/`bitloom-sim` 才能表达 hold/bubble → 停并上报

**Never:**
- CSR/trap；把 arch-test 绿当作流水正确；为「省 stall」而加 MEM 级 `load_q`→EX 旁路（会使 ATDD 在无 stall 时仍绿）；改 `rv32_core` 成流水

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Load-use | `LW rd` 后紧跟用 `rd` 的 ADDI/ADD | 1 拍 stall 后 MEM/WB→EX 转发，结果正确 | 无 stall → 黄金失败 |
| ALU RAW（回归） | 紧邻 ADDI→ADDI/ADD | 仍仅靠转发，无额外 stall | 错则既有黄金红 |
| Taken BEQ（回归） | 错误路径写 RF | 仍 flush，不因 stall 逻辑破坏 | 未 flush → RF 脏 |

</frozen-after-approval>

## Code Map

- `examples/rv32_pipe/src/lib.rs:561-615` — `do_stall` / `use_rs2` 门控 / PC·IF-ID hold / ID/EX bubble
- `examples/rv32_pipe/src/lib.rs:467-505` — 转发：`em_not_lw`；MEM/WB→EX（stall 后路径）
- `examples/rv32_pipeline_feasibility/src/lib.rs:32-45` — stall hold mux 模式
- `examples/rv32_core/src/lib.rs:436-448` — `enc_lw` / `enc_sw` 对照
- `examples/rv32_core/COMPLIANCE.md` — 验证阶梯
- `examples/rv32_pipe/PIPE.md` — load-use 合同

## Tasks & Acceptance

**Execution:**
- [x] `examples/rv32_pipe/src/lib.rs` — load-use 检测 + PC/IF-ID hold + ID/EX bubble；编码 `enc_lw`/`enc_sw`；ATDD tick + 既有黄金仍绿
- [x] `examples/rv32_pipe/PIPE.md` — 记录 load-use；验证命令与 ATDD 名
- [x] `examples/rv32_core/COMPLIANCE.md` — 验证阶梯 directed → 可选 rv32ui；未启用项；禁止 arch-test 宣称；rv32ui 标延期
- [x] 验证 — `cargo test -p rv32_pipe`、`-p rv32_core`；`cargo bitloom build --package rv32_pipe`
- [x] 收口 — 本故事 done；sprint-status（17.5 + epic-17）；`17-5-code-review.md`；**不** git commit

**Acceptance Criteria:**
- Given 17.4 五级+转发可 tick，when 实现 load-use（冻 PC/IF-ID、bubble ID/EX、再 MEM/WB→EX），then 独立命名 ATDD 在无停顿会失败、正确实现下通过
- Given COMPLIANCE/PIPE，when 读验证阶梯，then 定向→可选 rv32ui；写明未启用；不宣称完整 DV/arch-test
- Given rv32ui 未接入，when 标可选/延期，then 不阻塞 Done
- Given 无 CSR，when 跑测试与 bitloom build，then `rv32_pipe` 与 `rv32_core` 绿

## Spec Change Log

- review_loop 1 / patch：`rs2` stall 仅对 ADD/BEQ/SW 门控，避免 I-type `imm[4:0]` 与 LW rd 伪冲突导致假停顿。KEEP：mux hold、`em_not_lw`、无 `load_q`→EX 旁路、ATDD 名 `tick_load_use_stall_atdd_golden`。

## Design Notes

- **检测：** `do_stall := id_ex_is_lw ∧ rd≠0 ∧ (rd=rs1 ∨ (rd=rs2 ∧ use_rs2))`；`use_rs2 := ADD∨BEQ∨SW`。
- **优先：** IF/ID：`flush→bubble`，否则 `stall→hold`，否则前进；ID/EX：`flush∨stall→bubble`。
- **PC：** stall 时 `next_pc=pc`。
- **勿** `load_q`→EX 旁路：async `declare_mem` + seq→comb 下同拍可读到 `load_q`，旁路会让无 stall 也绿。
- **ATDD：** SW→`LW x3`→紧邻 `ADDI x4,x3,1`；期望 x4=43；强制 `do_stall=0` 时 x4=1。

## Verification

**Commands:**
- `cargo test -p rv32_pipe` — 全部 pass（含 load-use ATDD）
- `cargo test -p rv32_core` — 全部 pass
- `cargo bitloom build --package rv32_pipe` — success

## Dev Agent Record

### Completion Notes

- `EpisodeIIPipe`：load-use stall（PC/IF-ID hold + ID/EX bubble）+ 既有 MEM/WB→EX；`rs2` 经 `use_rs2` 门控
- ATDD：`tick_load_use_stall_atdd_golden`（人工验证无 stall 时失败 x4=1）
- `COMPLIANCE.md` / `PIPE.md`：验证阶梯 directed → 可选 rv32ui（延期）；无 arch-test 宣称；无 CSR
- sprint：`17-5` 与 `epic-17` → done；**未 git commit**

### File List

- `examples/rv32_pipe/src/lib.rs`
- `examples/rv32_pipe/PIPE.md`
- `examples/rv32_core/COMPLIANCE.md`
- `examples/rv32_core/SUBSET.md`
- `_agile-output/implementation-artifacts/17-5-load-use-停顿-atdd-验证阶梯.md`
- `_agile-output/implementation-artifacts/17-5-code-review.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Suggested Review Order

**Hazard 单元**

- Load-use 检测与 `use_rs2` 门控
  [`lib.rs:561`](../../examples/rv32_pipe/src/lib.rs#L561)

- PC / IF-ID hold 与 ID/EX bubble
  [`lib.rs:592`](../../examples/rv32_pipe/src/lib.rs#L592)

**文档阶梯**

- 定向 → 可选 rv32ui；禁止 arch-test 宣称
  [`COMPLIANCE.md:1`](../../examples/rv32_core/COMPLIANCE.md#L1)

- PIPE 合同与 ATDD 名
  [`PIPE.md:1`](../../examples/rv32_pipe/PIPE.md#L1)

**ATDD**

- `tick_load_use_stall_atdd_golden`
  [`lib.rs:972`](../../examples/rv32_pipe/src/lib.rs#L972)
