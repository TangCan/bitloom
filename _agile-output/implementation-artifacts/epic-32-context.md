# Epic 32 Context: Bundle 嵌套与 derive

<!-- Compiled from planning artifacts. Edit freely. Regenerate with compile-epic-context if planning docs change. -->

## Goal

设计者可用文档化嵌套 Bundle 与/或 derive，复合层次不必手工摊平；位宽/方向错误仍在 emit 前失败。相对 FR51 扁平 ground-leaf 最小合同加深为 FR80；不得仅用 OUT OF SCOPE / flatten-only 关闭深度（NFR37）。开工前须有有效 NFR14 风险记录（32.1）。

## Stories

- Story 32.1: Epic 32 NFR14 风险记录
- Story 32.2: 嵌套 Bundle 可综合路径
- Story 32.3: Bundle derive（或等价）
- Story 32.4: 嵌套 Bundle ATDD + 文档

## Requirements & Constraints

- 至少一层文档化嵌套 Bundle → elaborate → emit `.v` → tick（或文档等价）。
- 嵌套字段宽/向不匹配须 emit 前失败。
- nested 不得仅以 OUT OF SCOPE 交差；不得仅删注释而无实现。
- 本 epic 默认验收以一层为主；更深（≥2）须文档钉死或书面非目标。
- 设计 crate 只依赖 `bitloom-prelude`。
- derive：Story 32.3 经 `bitloom-prelude` 提供 `#[derive(Bundle)]`；完整嵌套 ATDD/限制表收口在 32.4。

## Technical Decisions

- 沿用 AD-20 / FR51：复合类型 flatten 为标量 HIR 端口；可不扩展公开 HIR Bundle 节点。
- 嵌套实现为更深命名叶子（`{field}_{member}_{leaf}`），非第二种公开 HIR。
- `HwVec<Bundle,_>` 与任意深度可在本 epic 保持非目标，须诚实文档。
- 捕获闭包不得进入 `tick`（AD-18）。

## Cross-Story Dependencies

- 32.2–32.4 门禁：32.1 NFR14 有效后才可 ready/开工。
- 32.3 依赖 32.2；32.4 依赖 32.2–32.3。
- 与 Epic 19 / FR51 最小合同区分：规划 done ≠ FR80 深度 done。
