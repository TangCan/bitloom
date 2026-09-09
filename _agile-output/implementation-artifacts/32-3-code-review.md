# Code Review: Story 32.3 Bundle derive（或等价）

**Verdict:** Approve

**Scope:** `#[derive(Bundle)]` in `bitloom-macro` + prelude re-export (AD-6); support/limits docs; derived positive emit/tick; unsupported-shape trybuild (`rhdl::E0180`); FR80 guardrails

## Findings

1. **无阻塞缺陷。** Derive 生成与手写等价的 `leaves` / `nested_bundles`；正例 `DerivedPacket` elaborate → emit → tick；enum/tuple/`HwVec` 字段稳定 `rhdl::E0180`；设计 crate 仍仅依赖 `bitloom-prelude`。
2. **AD-6 / AD-20 对齐：** 宏在 `bitloom-macro`，经 prelude `pub use`；未强迫设计 crate 依赖 CLI；未扩展公开 HIR Bundle 节点；一层嵌套合同不变；`HwVec<Bundle,_>` 仍 OOS。
3. **轻量建议（不挡合入）：** 用户文档限制表与更深 ATDD 收口留给 Story 32.4；可选后续支持 `Bits` 别名路径更多写法 / 属性 `#[bundle(nested)]` 显式标注（当前按 path 分类已够用）。

## AC Trace

| AC | Result |
| ---- | ------ |
| derive 或文档等价 + 支持/限制集合；正例可综合 emit | pass |
| 不支持嵌套/字段形态有稳定诊断 | pass |
| 设计 crate 仍只依赖 `bitloom-prelude`（AD-6） | pass |

## Decision

**Approve** — 可标 done；sprint 保持 `epic-32: in-progress`，`32-3: done`。
