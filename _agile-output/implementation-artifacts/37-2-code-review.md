# Code Review: Story 37.2 firtool/Chisel 钉死运维与机械 Chisel 诚实声明（FR88）

**Verdict:** Approve

**Scope:** `docs/fr28-chisel-compilable.md` · `docs/fr46-chisel-import.md` · README · `nfr11` 指针 · `fr88_firtool_chisel_ops_honesty.rs` · story/sprint

## Findings

1. **无阻塞缺陷。** 钉死对 7.14.0↔1.155.0、运维清单（`firtool ensure` / `RHDL_FIRTOOL_PATH` / `RHDL_FIRTOOL_CACHE` / 禁 PATH 冒充）、公开「可编译 ≠ idiomatic」、README 交叉链、FR46 对称诚实句与 ATDD 均到位。
2. **范围正确：** 未改 `emit_chisel`；未开工 37.3；未勾选 NFR14 Epic 37 全关闭条件。
3. **轻量建议（不挡合入）：** FR28→README 锚点依赖渲染器对中文标题的 slug；若某阅读器断链，可再加显式 HTML id。非本故事阻塞。

## AC Trace

| AC | Result |
| ---- | ------ |
| 钉死版本对 + 缓存/覆盖入口（`RHDL_FIRTOOL_PATH` 等） | pass |
| emit_chisel / 往返 = 可编译 ≠ idiomatic | pass |
| FR28/FR46 无「可维护手写风格」误导 | pass |
| 不强制改 emit；实现可选 | pass（docs-only） |

## Decision

**Approve** — 可标 done；sprint `37-2: done`；`epic-37` 保持 `in-progress`（37.3 仍 backlog）。
