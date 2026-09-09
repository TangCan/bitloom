# Code Review: Story 33.3 实现所选 Mem/Chisel 路径（FR81 Path A）

**Verdict:** Approve

**Scope:** `crates/rhdl-firrtl/src/chisel.rs` Path A Mem emit + unit/ATDD + fr73 Chisel expectation + light docs; sprint key `33-3-…`

## Findings

1. **无阻塞缺陷。** Path A 子集（`Mem` / `SyncReadMem` + 可选常量 init）经 `emit_chisel` 产出机械 Scala；`MemWrite`/`MemRead` 使用 `.write`/`.read`；NFR12 钉死对未改（7.14.0 ↔ 1.155.0）；FR71 无 Mem 计数器路径仍 emit。
2. **E0901 保留（非批发删除）。** 子集外（如 init 长度 ≠ depth、depth/width 0）仍返回 `rhdl::E0901`；ATDD + 单元测覆盖。
3. **轻量建议（不挡合入 / → 33.4）：** 本机有 JDK17+sbt 时对含 Mem 的 Scala 跑一次真编译；FR71 JVM job 仍以无 Mem 黄金为主，完整 Mem JVM 夹具属 Story 33.4。

## AC Trace

| AC | Result |
| ---- | ------ |
| Path A Mem 子集 → 可编译风格 Scala（FR81） | pass（`Mem`/`SyncReadMem`/init/write/read） |
| NFR12 钉死对不漂移 | pass（`CHISEL_TARGET`/`FIRTOOL_TARGET` + 头注释） |
| 子集外明确失败（保留 E0901） | pass |
| 不破坏 FR71 无 Mem 黄金路径 | pass（counter emit + 钉死串；未改 `fr28_golden_counter.scala`） |

## Decision

**Accept** — 标 `33-3-…: done`；`epic-33` 保持 `in-progress`；提交由父代理执行。
