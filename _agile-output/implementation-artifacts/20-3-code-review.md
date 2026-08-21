# Code Review: Story 20.3 FIRRTL/FrozenHir → 可编译 Chisel（FR28）

**Reviewer:** adversarial pass（blind / edge / verification-gap）  
**Date:** 2026-08-21  
**Verdict:** accept after patches

## Findings triage

| Severity | Category | Finding | Disposition |
| --- | --- | --- | --- |
| medium | patch | 层次连接仅测端口↔端口；wire 作 parent_net 无回归 | 增 `chisel_fr28_hierarchy_via_wire_parent_net` |
| medium | patch | dangling 连接跳过无断言 | 增 `chisel_fr28_dangling_connect_omitted` |
| medium | patch | 未知子模块 `panic!` | 改为预检 `rhdl::E0903` |
| low | patch | sprint `last_updated` 回拨 | 已校正 |
| low | defer | 文档路径仍名 `fr28-chisel-compilable.md` | deferred-work |
| low | defer | InOut → `Analog(Analog())` 双包；预存 | deferred-work |
| low | defer | 非 `clk`/`rst` 名的 Clock/Reset 端口仍进 IO | deferred-work |
| — | reject | 未连接子输入 / 未知 parent net | FrozenHir seal 已门禁 |
| — | reject | PortDirection 穷举外分支 | 枚举封闭 |

## AC checklist

| AC | Status |
| --- | --- |
| fixture→Scala；端口名/宽/向 + 实例层次谓词 | pass |
| 层次不返回 E0902；`Module(new …)` + 方向连线 | pass |
| MemDecl → E0901（子集外） | pass |
| CI 无 JVM 时谓词绿；文档钉死 7.14.0 / 1.155.0 | pass |
| 「结构化尽力失败」不算完成 | pass（合同文档 + 测试） |

## testarch-automate

- `crates/rhdl-firrtl`：`chisel_fr28_*`（flat / hierarchy / wire-parent / dangling / fir→import→emit / pin / mem）
- 可选：`scripts/chisel-fr28-compile.sh`（Java≥17+sbt；本机 Java 11 干净跳过）
- 脊柱门禁 `ad27_compilable_chisel` 已由 20.2 覆盖（本故事不重复）

## Disposition

Patches 合入 → `cargo test -p rhdl-firrtl` / `just test` 绿 → mark done。
