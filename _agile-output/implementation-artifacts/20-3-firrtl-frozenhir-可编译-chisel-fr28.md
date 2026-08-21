---
title: '20.3 FIRRTL/FrozenHir → 可编译 Chisel（FR28）'
type: 'feature'
created: '2026-08-21'
status: 'done'
baseline_commit: '787c84388bf829ca741f9dc720678599c5e82de0'
review_loop_iteration: 1
context:
  - '{project-root}/_agile-output/implementation-artifacts/epic-20-context.md'
  - '{project-root}/_agile-output/implementation-artifacts/20-2-架构-ad-firrtl-可编译-chisel-fr28-条.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-chisel-bidirectional.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Story 9.1 的 `emit_chisel` 仍以「尽力」拒绝实例（E0902），不满足 AD-27/FR28：钉死 Chisel+firtool 下可编译 + 端口/层次往返谓词。

**Approach:** 升级 `rhdl-firrtl` 生成器为机械风格可编译 Chisel Scala（含实例层次）；夹具断言端口名/宽/向与实例图；CI 以语法+谓词为主并文档钉死 pin；若本机有合格 JVM+Chisel 则可选真编译。

## Boundaries & Constraints

**Always:** 验收=可编译合同（非尽力失败）；钉死 Chisel **7.14.0** ↔ firtool **1.155.0**；公开端口名/宽/向与实例层次往返谓词；机械风格可；库 API（CLI 留给 20.5）；品牌 Bitloom；AD-3 FIRRTL 文本契约不变。

**Ask First:** 若必须绑定 Chisel 内部 API 或改选 NFR14 选项 B/C。

**Never:** 以 E0902「拒绝实例」交差；恢复 Scala `Parser.parse`；用 NFR10 调试再生冒充本故事；实现 FR46 反向导入 / `import` CLI（20.4–20.5）。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Flat counter | FrozenHir 无实例 | 可解析 Scala：`class` + IO + assigns；钉死注释 | N/A |
| Hierarchy | `Top` 实例化 `Child` | `Module(new Child)` + 按方向连线；谓词绿 | N/A |
| `.fir` 路径 | FIRRTL 文本 | `import`→`emit_chisel` 同谓词 | import 失败则诊断 |
| Unsupported mem | `MemDecl` | 结构化失败（子集外） | E0901；不得冒充 FR28 完成 |
| Pin drift | 生成头/常量 | 声明 7.14.0 / firtool-1.155.0 | 测试锁 pin |

</frozen-after-approval>

## Code Map

- `crates/rhdl-firrtl/src/chisel.rs` — `emit_chisel`；层次 `Module(new)`；E0901 mem；E0903 未知模块
- `crates/rhdl-firrtl/src/lib.rs` — `chisel_fr28_*` 谓词与夹具
- FIRRTL `inst` 发射 `lib.rs` — 连接方向参考
- `docs/fr28-chisel-compilable.md` + `README.md` — FR28 可编译合同 + CI pin
- `scripts/chisel-fr28-compile.sh` — 可选真编译（Java≥17+sbt）
- Sprint：`20-3-firrtl-frozenhir-可编译-chisel-fr28`

## Tasks & Acceptance

**Execution:**
- [x] `crates/rhdl-firrtl/src/chisel.rs` -- 发射层次 `Module(new …)` + 端口连线；去掉实例尽力失败；保留 mem 子集外错误 -- FR28
- [x] `crates/rhdl-firrtl/src/lib.rs` -- 反转/新增 ATDD：层次成功、端口/层次谓词、`.fir`→Scala、pin -- 验收
- [x] `docs/fr28-chisel-compilable.md`（或继任名）+ `README.md` -- 可编译合同 + CI 无 Scala 时谓词路径 + pin -- 文档
- [x] 可选 `scripts/` 或测试旁路 -- 若 Java≥17+coursier/sbt 可用则真编译；否则文档跳过 -- NFR14
- [x] `sprint-status.yaml` -- backlog→in-progress→review→done -- 追踪
- [x] `20-3-code-review.md` -- 对抗性审查 -- 流水线

**Acceptance Criteria:**
- Given 20.2 AD 已合入，when 对文档 fixture `emit_chisel`，then 产出机械风格 Scala 且测试证明端口名/宽/向与实例层次谓词成立
- Given 层次 fixture，when `emit_chisel`，then **不**返回 E0902；含 `Module(new …)` 与按方向连线
- Given mem 子集外，when 含 `MemDecl`，then 仍结构化失败（不得声称 mem 已 FR28）
- Given CI，when 无合格 Chisel JVM，then 语法+谓词测试仍绿，且文档钉死 Chisel 7.14.0 / firtool 1.155.0 与可选编译路径
- Given 「结构化尽力失败」，when 评审本故事，then **不算**完成

## Spec Change Log

- 2026-08-21: 实现 FR28 可编译 Chisel 发射（层次 + 谓词）；文档改写；可选编译脚本
- 2026-08-21: 审查补丁 — wire parent_net / dangling 测试；E0903；deferred InOut/文档路径

## Design Notes

Chisel `Module` 隐式 `clock`/`reset`：跳过 IO 中的 `clk`/`rst`，实例连接亦跳过这两项。子模块 Input：`child.io.p := parent`；Output：`parent := child.io.p`。需 circuit 内模块表查端口方向。

```scala
val u0 = Module(new Child)
u0.io.x := io.x
io.y := u0.io.y
```

## Verification

**Commands:**
- `cargo test -p rhdl-firrtl` -- expected: pass（含 FR28 谓词） — **PASS**
- `cargo fmt --all && just test` -- expected: pass

## Review Triage Log

### 2026-08-21 — Formal review pass（blind / edge / verification-gap）
- intent_gap: 0
- bad_spec: 0
- patch: 4（wire 谓词、dangling 谓词、E0903、last_updated）
- defer: 3（文档路径、InOut Analog、非 clk/rst Clock 名）
- reject: FrozenHir 已门禁项、封闭枚举
- addressed_findings: 见 [20-3-code-review.md](20-3-code-review.md)

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- `emit_chisel`：层次 `Module(new …)` + 按方向连线；移除 E0902；E0901 mem；E0903 未知模块
- ATDD：flat / hierarchy / wire-parent / dangling / `.fir`→Scala / pin / mem
- 文档可编译合同；`scripts/chisel-fr28-compile.sh`（Java 11 干净跳过）
- 对抗性审查 + automate 谓词套件

### File List

- `crates/rhdl-firrtl/src/chisel.rs`
- `crates/rhdl-firrtl/src/lib.rs`
- `docs/fr28-chisel-compilable.md`
- `README.md`
- `scripts/chisel-fr28-compile.sh`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `_agile-output/implementation-artifacts/20-3-firrtl-frozenhir-可编译-chisel-fr28.md`
- `_agile-output/implementation-artifacts/20-3-code-review.md`
- `_agile-output/implementation-artifacts/deferred-work.md`

## Change Log

- 2026-08-21: FR28 可编译 Chisel 生成器 + 谓词测试 + 文档/可选脚本（Story 20.3）

## Suggested Review Order

**生成器入口**

- FR28 `emit_chisel`：mem/未知模块预检 + 钉死头
  [`chisel.rs:36`](../../crates/rhdl-firrtl/src/chisel.rs#L36)

- 按方向实例连线（跳过 clk/rst/dangling）
  [`chisel.rs:137`](../../crates/rhdl-firrtl/src/chisel.rs#L137)

- `Module(new …)` 层次发射
  [`chisel.rs:229`](../../crates/rhdl-firrtl/src/chisel.rs#L229)

**验收测试**

- 层次 + 端口/连接谓词
  [`lib.rs:555`](../../crates/rhdl-firrtl/src/lib.rs#L555)

- wire 作 parent_net
  [`lib.rs:569`](../../crates/rhdl-firrtl/src/lib.rs#L569)

- dangling 省略
  [`lib.rs:612`](../../crates/rhdl-firrtl/src/lib.rs#L612)

**文档 / 可选编译**

- 可编译合同与 CI pin
  [`fr28-chisel-compilable.md:1`](../../docs/fr28-chisel-compilable.md#L1)

- 可选 Chisel 7.14.0 编译脚本
  [`chisel-fr28-compile.sh:1`](../../scripts/chisel-fr28-compile.sh#L1)
