---
title: '20.4 反向导入 Chisel/.fir → Bitloom（FR46 腿 2）'
type: 'feature'
created: '2026-08-21'
status: 'done'
baseline_commit: '17ef30697256f86c6e59b9f67931cdf040c1b06e'
review_loop_iteration: 1
context:
  - '{project-root}/_agile-output/implementation-artifacts/epic-20-context.md'
  - '{project-root}/_agile-output/implementation-artifacts/20-3-firrtl-frozenhir-可编译-chisel-fr28.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-chisel-bidirectional.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR46 反向腿未产品化：虽有 FIRRTL `import`，缺与正向对称的端口/实例图往返谓词、外部 `.fir`（含文档化 Chisel→firtool 产出）→ FrozenHir → emit/tick 夹具。

**Approach:** 加固 `rhdl_firrtl::import`（接受 firtool 风格双向 connect），导出公开谓词 API，夹具覆盖导出再导入与外部 `.fir` 再 emit/tick；文档钉死 CIRCT 交换边界为 `.fir`（不解析 Scala）。

## Boundaries & Constraints

**Always:** 对称往返谓词（公开端口名/宽/向 + 实例图）；`.fir`→FrozenHir→emit/tick；设计 crate 仅 `bitloom-prelude`；品牌 Bitloom；AD-3 子集不变。

**Ask First:** 若必须解析 Chisel Scala 源码（非 `.fir`）才能交差。

**Never:** 恢复 Scala `Parser.parse`；用 NFR10 调试再生冒充 FR46；实现 `import` CLI / 混合夹具产品入口（留给 20.5）；静默降级 FR46。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Export→import | FrozenHir → emit `.fir` → import | 端口/实例图谓词相对原 HIR 成立 | N/A |
| External `.fir` | 文档夹具（含 `parent <= inst.port`） | import → emit FIRRTL/Verilog；层次谓词绿 | 坏头/子集外 → E0401/E0402 |
| Chisel workflow | 文档：Chisel→firtool→`.fir`→import | 不解析 `.scala`；路径写清 | N/A |
| Import→tick | 导入 counter | `bitloom-sim` tick 行为合理 | N/A |

</frozen-after-approval>

## Code Map

- `crates/rhdl-firrtl/src/lib.rs` — `import`；加固 output connect；`ports_roundtrip_ok` / `instance_graph_roundtrip_ok`
- `crates/rhdl-firrtl/fixtures/` — 外部层次 `.fir` 夹具
- `docs/fr46-chisel-import.md` — FR46 反向腿 + Chisel 工作流
- `README.md` — 链到 FR46 文档
- Sprint：`20-4-反向导入-chisel-fir-bitloom-fr46-腿-2`

## Tasks & Acceptance

**Execution:**
- [x] `crates/rhdl-firrtl/src/lib.rs` -- 接受 `parent <= inst.port`；公开对称往返谓词；ATDD（导出再导入 / 外部 fir→emit / tick） -- FR46
- [x] `crates/rhdl-firrtl/fixtures/external_hierarchy.fir` -- 外部层次夹具（firtool 风格 connect） -- 夹具
- [x] `docs/fr46-chisel-import.md` + `README.md` -- 反向腿与 Chisel→firtool→`.fir` 路径 -- 文档
- [x] `sprint-status.yaml` -- backlog→in-progress→review→done -- 追踪
- [x] `20-4-code-review.md` -- 对抗性审查 -- 流水线

**Acceptance Criteria:**
- Given 20.3 正向可用，when `.fir`（及文档化 Chisel 工作流产出）→ FrozenHir → emit/tick，then 公开端口与实例图满足与正向对称的往返谓词（FR46）
- Given 至少一夹具，when 覆盖「导出再导入」或「外部 `.fir` 导入再 emit」，then 测试绿
- Given 设计 crate，when 依赖图，then 仍仅 `bitloom-prelude`（导入工具在 firrtl/CLI crates）

## Spec Change Log

- 2026-08-21: 审查补丁 — 未知 dotted lhs 不发明 net；deferred unknown rhs-dot / dangling

## Design Notes

CIRCT 时代：不解析 Chisel Scala；交换边界为 firtool 写出的 `FIRRTL version 6.0.0` 文本。导入须同时接受本工具链 emit 的 `inst.port <= parent` 与 firtool 常见的 `parent <= inst.port`（输出）。

## Verification

**Commands:**
- `cargo test -p rhdl-firrtl` -- expected: pass（含 FR46 谓词） — **PASS**
- `cargo fmt --all && just test` -- expected: pass — **PASS**

## Review Triage Log

### 2026-08-21 — Formal review pass
- patch: 1（dotted lhs 跳过）
- defer: 2（unknown rhs-dot；dangling 往返）
- addressed: 见 [20-4-code-review.md](20-4-code-review.md)

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- `import` 接受 firtool 风格 `parent <= inst.port`
- 公开 `ports_roundtrip_ok` / `instance_graph_roundtrip_ok`
- 外部夹具 + 导出再导入 + tick ATDD
- 文档 `docs/fr46-chisel-import.md`

### File List

- `crates/rhdl-firrtl/src/lib.rs`
- `crates/rhdl-firrtl/Cargo.toml`
- `crates/rhdl-firrtl/fixtures/external_hierarchy.fir`
- `docs/fr46-chisel-import.md`
- `README.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `_agile-output/implementation-artifacts/20-4-反向导入-chisel-fir-bitloom-fr46-腿-2.md`
- `_agile-output/implementation-artifacts/20-4-code-review.md`
- `_agile-output/implementation-artifacts/deferred-work.md`

## Change Log

- 2026-08-21: FR46 反向导入谓词 + firtool connect + 夹具/文档（Story 20.4）
